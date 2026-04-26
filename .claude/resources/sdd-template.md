# Software Design Document (SDD) Template

Use this template when designing a non-trivial feature before implementation.
A completed SDD lives in `docs/technical/` and is linked from `architecture.md`.

---

## Document metadata

| Field | Value |
|-------|-------|
| **Title** | |
| **Author** | |
| **Date** | YYYY-MM-DD |
| **Status** | Draft / In Review / Approved / Superseded |
| **Related tickets** | #N, #N |

---

## Problem statement

<!-- What problem are we solving? Why does it need solving now?
     One paragraph. Be concrete — describe the user or system impact. -->

---

## Goals

<!-- What must this design achieve? Use "must" for requirements, "should" for preferences. -->

- Must: 
- Must: 
- Should: 

## Non-goals

<!-- Explicitly state what this design will NOT do. Prevents scope creep. -->

- 
- 

---

## Background and context

<!-- What does a reader need to know to understand this design?
     Reference existing code, docs, or prior decisions as needed. -->

---

## Proposed design

### Overview

<!-- One paragraph or diagram describing the approach at a high level. -->

### Detailed design

<!-- Walk through the significant components, data flows, or algorithms.
     Use subsections, diagrams, or pseudocode as needed. -->

#### Component / module: [name]

<!-- What does it do? How does it fit into the system? -->

#### Data model changes

<!-- Tables, schemas, types, or structs being added or modified. -->

#### API changes

<!-- New endpoints, changed signatures, removed interfaces.
     Include before/after for any breaking changes. -->

#### Error handling

<!-- How are failure cases handled? What does the caller see? -->

---

## Alternatives considered

<!-- What other approaches did you evaluate? Why did you reject them?
     Even one sentence per alternative is enough. -->

| Alternative | Why rejected |
|-------------|-------------|
| | |

---

## Risks and open questions

<!-- What could go wrong? What is still unknown? -->

| Risk / question | Severity | Mitigation / answer |
|-----------------|----------|---------------------|
| | | |

---

## Implementation plan

<!-- High-level steps in order. Reference branch names or ticket numbers if known. -->

1. 
2. 
3. 

**Estimated effort:** [PLACEHOLDER: e.g. 2 days, 1 sprint]

---

## Testing approach

<!-- How will correctness be verified? Unit, integration, E2E, load tests? -->

---

## Rollout and rollback

<!-- How will this be deployed? How do we roll back if something goes wrong? -->
