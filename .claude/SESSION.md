# SESSION.md — Session Memory

This file is written by the **session-memory** agent at the end of each working session
and read at the start of the next one. It gives Claude continuity across conversations.

---

## Format

Each session entry follows this structure:

```
## Session — YYYY-MM-DD

**What was worked on:** <one or two sentences>
**Key decisions made:** <bullet list, or "none">
**Work in progress:** <what is unfinished and where it left off>
**Blockers or open questions:** <anything unresolved>
**Files changed:** <list of files that were meaningfully modified>
**Next steps:** <what should happen at the start of the next session>
```

Append new entries at the top (most recent first).
Do not delete old entries — they form a useful history.
Keep each entry concise: the goal is orientation, not a transcript.

---

<!-- Session entries go below this line -->
