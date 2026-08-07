# ai-code-reviewer

[![CI](https://github.com/lexosi/ai-code-reviewer/actions/workflows/ci.yml/badge.svg)](https://github.com/lexosi/ai-code-reviewer/actions/workflows/ci.yml)

A post-commit git hook that sends the latest commit's diff to the Claude API for
automated code review and delivers the result to Telegram. Written in Rust.

## What it does

- Runs as a **short-lived binary** invoked by a git `post-commit` hook — no
  daemon, queue, or database.
- Pipeline: **diff → Claude → Telegram**. It reads the diff of the latest commit
  (`git diff HEAD~1 HEAD`), sends it to the Anthropic Messages API, and posts the
  review to a Telegram chat.
- **Configurable diff truncation**: large diffs are truncated to a character
  limit before being sent, keeping API cost bounded.
- **Dry-run mode**: prints the review to stdout instead of sending to Telegram,
  for local testing.

## Requirements

- Rust toolchain (**edition 2024**).
- An **Anthropic API key**.
- A **Telegram bot token** and **chat id**.
- **git**.

## Build

```sh
cargo build --release
```

The binary is produced at `target/release/ai-code-reviewer`.

## Configuration

Copy `config.toml.example` to `config.toml` and place it **next to the binary**.
The config is resolved via `current_exe().parent()`, so it travels with the
binary regardless of the working directory when the hook fires.

```toml
anthropic_api_key  = "sk-ant-..."
telegram_bot_token = "123456:ABC-..."
telegram_chat_id   = "-1001234567890"

# Optional: maximum diff characters sent to Claude (default: 8000)
# max_diff_chars = 8000
```

Fields:

| Field | Required | Default | Description |
|-------|----------|---------|-------------|
| `anthropic_api_key` | yes | — | Anthropic API key |
| `telegram_bot_token` | yes | — | Telegram bot token |
| `telegram_chat_id` | yes | — | Target Telegram chat id |
| `max_diff_chars` | no | `8000` | Max diff characters sent to Claude |

`config.toml` is gitignored — **never commit real secrets**.

## Install the hook

From the **root of the target repository** you want reviewed, run:

```sh
ai-code-reviewer install
```

This copies `hooks/post-commit` into that repository's `.git/hooks/`.

On Windows / Git Bash, PATH lookup of the `.exe` can be unreliable, so the hook
resolves the binary via the `AI_CODE_REVIEWER_BIN` environment variable first,
then falls back to `ai-code-reviewer` on PATH. Point it at the absolute binary
path:

```sh
export AI_CODE_REVIEWER_BIN=/path/to/ai-code-reviewer/target/release/ai-code-reviewer.exe
```

Alternatively, add the binary directory to your `PATH`. If neither resolves, the
hook prints a notice and skips the review — it never blocks the commit.

## Usage

Once installed, every `git commit` in the target repository triggers a review
automatically.

Manual runs:

```sh
# Run the full pipeline and send the review to Telegram
ai-code-reviewer review

# Print the review to stdout, skip Telegram
ai-code-reviewer review --dry-run
```

## Architecture

The binary is a single sequential process split into five modules — `main`
(CLI + pipeline composition), `config` (loads `config.toml`), `git` (runs
`git diff` subprocesses), `claude` (Anthropic Messages API client), and
`telegram` (Telegram Bot API client). HTTP calls use **blocking `reqwest`** (no
async runtime, since there is one sequential call per invocation) with a **30s
timeout**. The review model is `claude-haiku-4-5-20251001`.

Full design, data flow, and decision records: [`docs/technical/architecture.md`](docs/technical/architecture.md).

## Testing

```sh
cargo test
```

There are **30 unit tests** covering pure logic: JSON response parsing,
char-boundary-safe truncation, CLI argument parsing, and git subprocess
handling. Network-dependent paths (Claude and Telegram HTTP calls) are validated
via end-to-end smoke testing rather than mocks, to avoid tests coupling to wire
formats that can diverge from real API behavior.

## Known limitations

See the [Known limitations](docs/technical/architecture.md#known-limitations-and-technical-debt)
section of the architecture doc. In brief:

- Diff truncation is a character-count slice with no awareness of hunk
  boundaries — a truncated diff may end mid-hunk.
- One binary = one set of credentials (single co-located `config.toml`).
  Multi-repo / multi-account setups require multiple installs.
- No retry logic on transient HTTP failures from either API.

## License

This project is licensed under the MIT License — see [LICENSE](LICENSE).
