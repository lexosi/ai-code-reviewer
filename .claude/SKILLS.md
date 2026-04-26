# SKILLS.md — Available Skills and Tools

This file documents the Claude Code skills and external tools available in this project.
The orchestrator reads this before deciding how to approach a task.

---

## Built-in Claude Code tools

These are always available:

| Tool | When to use |
|------|-------------|
| `Read` | Read any file before editing it |
| `Edit` | Make targeted changes to existing files |
| `Write` | Create new files from scratch |
| `Bash` | Run shell commands, tests, build steps |
| `Grep` | Search for patterns across the codebase |
| `Glob` | Find files by name pattern |
| `WebSearch` | Look up documentation, error messages, packages |
| `WebFetch` | Fetch a specific URL |

---

## Claude Code skills (slash commands)

Invoke these by name when the task matches.

| Skill | Purpose |
|-------|---------|
| `init` | Generate a `CLAUDE.md` for an existing codebase |
| `review` | Review a pull request |
| `security-review` | Security audit of pending changes |
| `simplify` | Refactor changed code for clarity and efficiency |
| `update-config` | Modify `.claude/settings.json`, add hooks or permissions |
| `fewer-permission-prompts` | Reduce repetitive permission prompts by adding allowlists |

---

## Project-specific tools

[PLACEHOLDER: list any project-specific CLI tools, scripts, or MCP servers that Claude should know about, e.g.]

| Tool | Command | Purpose |
|------|---------|---------|
| [PLACEHOLDER] | [PLACEHOLDER: e.g. `npm run lint`] | [PLACEHOLDER: e.g. Run linter] |
| [PLACEHOLDER] | [PLACEHOLDER: e.g. `./scripts/seed-db.sh`] | [PLACEHOLDER: e.g. Seed local database] |

---

## MCP servers

[PLACEHOLDER: list any MCP servers configured for this project, e.g.]

| Server | Purpose |
|--------|---------|
| [PLACEHOLDER: e.g. `github`] | [PLACEHOLDER: e.g. Read/write GitHub issues and PRs] |
| [PLACEHOLDER: e.g. `postgres`] | [PLACEHOLDER: e.g. Query the local dev database] |

If no MCP servers are configured, delete this section.

---

## Tool usage guidelines

- Always use `Read` before `Edit` — never edit a file you haven't read.
- Use `Bash` for commands that have no dedicated tool. Prefer dedicated tools otherwise.
- Use `WebSearch` to look up unfamiliar APIs or error messages before guessing.
- Do not run destructive `Bash` commands (drop table, rm -rf, force push) without confirming with the user.
