# CLAUDE.md — Project Workflow Entry Point

This is the single entry point for Claude Code in this project.
Read this file before doing anything else. Then follow the references below.

---

## Project Identity

**Project name:** ai-code-reviewer
**Description:** A post-commit git hook daemon that extracts the diff of the latest commit, sends it to the Claude API for code review, and delivers the result to a Telegram chat.
**Primary language(s):** Rust
**Repository URL:** https://github.com/lexosi/ai-code-reviewer
**Main branch:** main
**Team / owner:** lexosi

---

## Quick orientation

Before planning or executing any task, Claude must:

1. Read `.claude/SESSION.md` — load context from the previous session.
2. Read `.claude/AGENTS.md` — understand which agents exist and when to use them.
3. Read `.claude/SKILLS.md` — understand available skills and tools.
4. If the task involves multiple steps or agents, invoke the **orchestrator** agent first.

---

## Repository layout

```
src/          # application source (main.rs, config.rs, git.rs, claude.rs, telegram.rs)
scripts/      # install.ps1 and install.sh (git hook installers)
hooks/        # the post-commit hook script
docs/         # documentation
```

---

## Development environment

**Package manager:** cargo
**How to install dependencies:** `cargo build --release`
**How to run tests:** `cargo test`
**How to start the dev server / app:** `cargo run`
**Linter / formatter:** cargo clippy

---

## Git flow

Branch strategy enforced in this project:

```
feature/<name>  →  test/<name>  →  main
```

- Never commit directly to `main`.
- Every feature branch must pass tests before merging.
- PRs use the template at `.claude/resources/pr-template.md`.
- See `.claude/agents/git-flow/agent.md` for the full git workflow agent.

---

## Coding standards

- All code and comments in English
- Use `Result<T, E>` for error handling, no `unwrap()` in production paths
- `config.toml` lives next to the binary, never committed to git

---

## Key files and references

| File | Purpose |
|------|---------|
| `.claude/AGENTS.md` | Index of all agents and when to invoke them |
| `.claude/SKILLS.md` | Available skills and external tools |
| `.claude/SESSION.md` | Session memory — read at start, write at end |
| `.claude/agents/orchestrator/agent.md` | Multi-step task planning |
| `.claude/agents/git-flow/agent.md` | Branching, commits, and PRs |
| `.claude/agents/debugger/agent.md` | Error diagnosis and root-cause analysis |
| `.claude/agents/session-memory/agent.md` | Session continuity across conversations |
| `.claude/resources/sdd-template.md` | Software design document template |
| `.claude/resources/pr-template.md` | Pull request description template |
| `docs/technical/architecture.md` | System architecture overview |

---

## Context for Claude

- Strictness level: **FLEXIBLE** — the workflow guides, it does not block.
- Trivial task definition: single-file edits explicitly described by the user, pure Q&A, or reading files. When in doubt, invoke the orchestrator.
- When uncertain about scope, ask rather than assume.
- Prefer minimal, targeted changes over large rewrites.
- After completing a session, invoke the **session-memory** agent to write `.claude/SESSION.md`.
- Design decisions worth preserving should go in `docs/technical/architecture.md`.

---

## Project-specific notes

- **Windows build:** requires CMD (not PowerShell) — run `vcvars64.bat`, set `CMAKE_GENERATOR=Visual Studio 17 2022`, then `cargo build --release`.
- `config.toml` and `state.json` must be listed in `.gitignore` and never committed.
- Use `reqwest` with the `blocking` feature, following the same pattern as `screenpipe-macro`.
