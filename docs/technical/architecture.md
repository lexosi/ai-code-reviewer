# Architecture

This document records significant architectural decisions and system design for
**[PLACEHOLDER: project-name]**.

It is a living document. When a meaningful architectural decision is made during
a Claude Code session, the session-memory or orchestrator agent should update
or append to this file.

---

## System overview

[PLACEHOLDER: describe the system at a high level — what it does, who uses it, what it connects to]

---

## Component map

[PLACEHOLDER: diagram or description of the major components and how they relate]

```
[PLACEHOLDER: e.g.
  Browser ──► API Server ──► PostgreSQL
                   │
                   └──► Redis (cache)
                   └──► S3 (file storage)
]
```

---

## Key design decisions

Use this section to record decisions that are not obvious from the code —
especially those where an alternative was rejected.

### Decision record format

```
### [SHORT TITLE] — YYYY-MM-DD

**Context:** why this decision needed to be made
**Decision:** what was decided
**Alternatives rejected:** what else was considered and why it was not chosen
**Consequences:** what this decision makes easier or harder going forward
```

---

<!-- Add decision records below this line, most recent first -->

### [PLACEHOLDER: first decision title] — YYYY-MM-DD

**Context:** [PLACEHOLDER]
**Decision:** [PLACEHOLDER]
**Alternatives rejected:** [PLACEHOLDER]
**Consequences:** [PLACEHOLDER]

---

## Data model

[PLACEHOLDER: describe the core entities and their relationships.
Link to migration files or schema definitions if they exist.]

---

## External dependencies

[PLACEHOLDER: list significant external services, APIs, or libraries and why they were chosen]

| Dependency | Purpose | Notes |
|------------|---------|-------|
| [PLACEHOLDER] | [PLACEHOLDER] | |

---

## Security model

[PLACEHOLDER: describe authentication, authorization, and any relevant threat model.
Even a short paragraph is better than nothing.]

---

## Performance considerations

[PLACEHOLDER: known bottlenecks, scaling limits, caching strategy, or SLOs if defined]

---

## Known limitations and technical debt

[PLACEHOLDER: list known shortcuts or areas that need future improvement]

- [PLACEHOLDER]
