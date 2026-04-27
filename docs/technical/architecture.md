# Architecture — ai-code-reviewer

A post-commit git hook daemon that extracts the diff of the latest commit,
sends it to the Claude API for code review, and delivers the result to a
Telegram chat.

---

## System overview

`ai-code-reviewer` is a compiled Rust binary with two subcommands:

- `install` — copies the hook script from `hooks/post-commit` into
  `.git/hooks/post-commit` of the target repository.
- `review` — runs the full pipeline: reads config, gets the diff of the latest
  commit, calls the Claude API, and sends the review to Telegram.

The binary is invoked automatically after every `git commit` via the installed
hook. It is a short-lived process — it runs, delivers the review, and exits.
There is no persistent daemon, queue, or database.

---

## Data flow

```
git commit
    │
    ▼
.git/hooks/post-commit          (shell script — installed by `install` subcommand)
    │  checks binary exists at absolute path; calls `ai-code-reviewer review`
    ▼
main.rs :: run_review()
    │
    ├─► config.rs :: Config::load()
    │       reads config.toml from the same directory as the binary
    │       fields: anthropic_api_key, telegram_bot_token, telegram_chat_id,
    │               max_diff_chars (default 8000)
    │
    ├─► git.rs :: get_diff()
    │       runs `git diff HEAD~1 HEAD`
    │       returns raw unified diff as a UTF-8 string
    │       diff is truncated to max_diff_chars before being sent to Claude
    │
    ├─► claude.rs :: review_diff()
    │       POST https://api.anthropic.com/v1/messages
    │       model: claude-haiku-4-5-20251001
    │       max_tokens: 1024
    │       system prompt: expert code reviewer focusing on bugs, security,
    │                      performance, and maintainability
    │       returns the text content block from the API response
    │
    └─► telegram.rs :: send_message()
            POST https://api.telegram.org/bot{token}/sendMessage
            message prefixed with "*AI Code Review*\n\n"
            truncated to 4096 chars (Telegram API hard limit)
```

---

## Module responsibilities

### `src/main.rs`

Entry point and CLI definition. Uses `clap` derive macros to parse subcommands.
Owns the `run_review()` and `run_install()` functions, which are the only places
where modules are composed together. All cross-cutting concerns (truncation
logging, early exit on empty diff) live here rather than inside modules.

### `src/config.rs`

Loads and deserializes `config.toml`. The file is resolved relative to the
binary's own directory (`current_exe().parent()`) so the config travels with
the binary regardless of the working directory when the hook fires.

Exposes `Config::from_toml_str` as `pub(crate)` to allow unit tests to parse
config from string literals without touching the filesystem.

### `src/git.rs`

Runs git subprocesses via `std::process::Command`. The internal `run_git()`
function is `pub(crate)` so tests can call arbitrary git subcommands to verify
error propagation and stdout capture. `get_diff()` is the only public entry
point and always diffs `HEAD~1` against `HEAD`.

### `src/claude.rs`

HTTP client for the Anthropic Messages API. Serializes the request body with
`serde_json`, sends it with `reqwest::blocking::Client`, and deserializes the
response into typed structs. The text extraction logic (`extract_text_from_response`)
is `pub(crate)` so it can be unit-tested against fixture JSON without making
network calls. Constants (`MODEL`, `API_URL`, `ANTHROPIC_VERSION`) are defined
at the module top level.

### `src/telegram.rs`

HTTP client for the Telegram Bot API. `prepare_text()` is `pub(crate)` and
handles the 4096-character truncation with a visible `… (truncated)` suffix.
The public `send_message()` function calls `prepare_text` before sending,
ensuring the limit is always enforced.

---

## Key design decisions

### Blocking HTTP, no async — 2026-04-26

**Context:** The tool is invoked as a post-commit hook (a short-lived child
process). Async runtimes add startup overhead and complexity with no benefit
when there is only one sequential HTTP call per invocation.

**Decision:** Use `reqwest` with the `blocking` feature. No `tokio` runtime.

**Alternatives rejected:** `tokio` + `reqwest` async — adds ~200 µs runtime
startup and requires `async fn` propagation throughout the call chain for no
throughput gain.

**Consequences:** Cannot parallelize the Claude and Telegram calls in the
future without switching to async. Acceptable given the tool's sequential
nature.

---

### Config co-located with binary — 2026-04-26

**Context:** The hook fires with the working directory set to the repository
root of the project being reviewed, not the ai-code-reviewer project. A
relative path like `./config.toml` would look in the wrong place.

**Decision:** Resolve `config.toml` via `std::env::current_exe().parent()`.
The config file always lives next to the binary.

**Alternatives rejected:** `XDG_CONFIG_HOME` / platform config dirs — adds
a dependency and makes the install story more complex. Environment variables
for each credential — usable but verbose and error-prone for users managing
multiple repositories.

**Consequences:** One binary can only serve one set of credentials. A user
reviewing multiple repositories must either copy the binary or symlink the
config. Acceptable for the v0.1 use case.

---

### Client-side diff truncation — 2026-04-26

**Context:** Large commits (generated code, lock file updates) can produce
diffs of hundreds of kilobytes, which exceed practical prompt sizes and inflate
API costs.

**Decision:** Truncate the raw diff to `max_diff_chars` (default 8000)
**before** sending to Claude. The truncation is a hard byte-count slice, not
a semantic split.

**Alternatives rejected:** Sending the full diff and relying on Claude's
context window — would make review cost unpredictable. Splitting the diff into
chunks — meaningful chunking of a unified diff requires parsing the format;
out of scope for v0.1.

**Consequences:** Reviews of large commits will be incomplete. The truncation
boundary may fall mid-hunk. Acceptable trade-off; `max_diff_chars` is
configurable so users with higher budgets can increase it.

---

### Absolute binary path in hook script — 2026-04-26

**Context:** On Windows with Git Bash, `command -v` and PATH-based lookups for
the `.exe` binary were unreliable. The binary was not found even when on PATH.

**Decision:** Hard-code the absolute path to the binary in `hooks/post-commit`
using `[ ! -x "$REVIEWER" ]` to check existence and executable permission
before invoking.

**Alternatives rejected:** PATH lookup via `command -v` — failed in practice
on Windows/Git Bash. Relative path from hook — unreliable because the hook's
working directory varies.

**Consequences:** The hook script is not portable; it must be re-installed
(`ai-code-reviewer install`) if the binary moves. Documented as a known
limitation.

---

### Testability via pure function extraction — 2026-04-26

**Context:** The Claude and Telegram modules make outbound HTTP calls, making
them impossible to unit-test without mocking or a live server. Mocks were
rejected because they can diverge from real API behavior.

**Decision:** Extract the pure logic (JSON parsing, text truncation) into
`pub(crate)` functions (`extract_text_from_response`, `prepare_text`,
`run_git`) that can be tested in isolation. Network-dependent paths are tested
only via end-to-end validation.

**Alternatives rejected:** HTTP mocking with `wiremock` or `httpmock` — adds
a dev-dependency and couples tests to wire format details that may change.
Integration test suite hitting live APIs — non-deterministic and requires
credentials in CI.

**Consequences:** Unit test coverage is limited to pure logic. The HTTP paths
rely on end-to-end smoke testing for validation.

---

## External dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `anyhow` | 1 | Ergonomic error propagation with context chains |
| `clap` | 4 (derive) | CLI argument parsing with subcommands |
| `reqwest` | 0.12 (blocking, json) | HTTP client for Claude and Telegram APIs |
| `serde` | 1 (derive) | Serialization/deserialization trait derivation |
| `serde_json` | 1 | JSON encode/decode for API payloads |
| `toml` | 0.8 | TOML config file parsing |

---

## Security model

Credentials (`anthropic_api_key`, `telegram_bot_token`) are stored in
`config.toml` on the local filesystem next to the binary. The file is never
committed to git (enforced via `.gitignore`). There is no credential rotation,
encryption at rest, or secrets manager integration in v0.1.

The binary communicates only with `api.anthropic.com` and `api.telegram.org`
over HTTPS (enforced by `reqwest` / `native-tls`). No data is written to disk
beyond the config file.

The hook script guards against a missing binary rather than silently failing,
so a misconfigured install cannot block commits.

---

## Known limitations and technical debt

- Hook script contains a hard-coded absolute binary path — must be re-installed
  if the binary moves. `scripts/install.ps1` also references a relative path
  and may need updating for out-of-tree installs.
- Diff truncation is a byte-count slice with no awareness of hunk boundaries;
  the last hunk in a truncated diff may be malformed.
- A single `config.toml` co-located with the binary means one binary = one set
  of credentials. Multi-repo or multi-account setups require multiple installs.
- No `--dry-run` flag to skip Telegram delivery during local testing.
- No retry logic on transient HTTP failures from either API.
- `max_tokens: 1024` is fixed in code; should be a config field for users who
  need longer reviews.
