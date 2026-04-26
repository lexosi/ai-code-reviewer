# AGENTS.md — Agent Index

This file is the authoritative index of all agents available in this project.
The **orchestrator** reads this before planning any multi-step task.

---

## Agent roster

| Agent | File | One-line purpose |
|-------|------|-----------------|
| Orchestrator | `agents/orchestrator/agent.md` | Decomposes complex tasks, delegates to other agents |
| Git Flow | `agents/git-flow/agent.md` | Manages branches, commits, PRs, and merges |
| Debugger | `agents/debugger/agent.md` | Diagnoses errors, traces root causes, proposes minimal fixes |
| Session Memory | `agents/session-memory/agent.md` | Writes and reads `.claude/SESSION.md` for cross-session continuity |

---

## When to invoke each agent

### Orchestrator
Invoke when:
- The task has more than two distinct steps.
- Multiple agents may need to collaborate.
- The user's request is ambiguous and needs to be broken into a plan before execution.
- You are unsure which agent(s) should handle a request.

### Git Flow
Invoke when:
- Creating, switching, or merging branches.
- Writing commit messages.
- Preparing or reviewing a pull request.
- Checking branch state before starting new work.

### Debugger
Invoke when:
- A test suite is failing.
- An exception or panic has been reported.
- Unexpected behavior needs root-cause analysis.
- A log file needs to be read and interpreted.

### Session Memory
Invoke when:
- Starting a new session (read `SESSION.md` for prior context).
- Ending a session (write a summary to `SESSION.md`).
- The user says "remember this for next time" or similar.

---

## Adding a new agent

1. Create `agents/<name>/agent.md` using the four-section format:
   `Role`, `Responsibilities`, `When to invoke`, `What to never do`.
2. Add a row to the roster table above.
3. Add an entry to the "When to invoke" section above.
