# Pull Request Template

Copy this template when creating a PR. Fill in all sections.
Delete sections that genuinely do not apply (e.g. "Screenshots" for a backend-only change).

---

## Summary

<!-- One paragraph: what does this PR do and why? -->

## Changes

<!-- Bullet list of the meaningful changes. Focus on the "what", not the "how". -->

- 
- 

## Motivation

<!-- Why was this change needed? Reference a ticket, bug report, or design doc if available. -->

Closes #[ISSUE_NUMBER]

## Test plan

<!-- How was this tested? Check all that apply and describe what you ran. -->

- [ ] Existing tests pass (`[PLACEHOLDER: test command]`)
- [ ] New tests added for the changed behavior
- [ ] Manually tested — describe what you did:
  > 
- [ ] No tests needed — explain why:
  > 

## Checklist

- [ ] Branch follows naming convention (`feature/<name>` → `test/<name>` → `main`)
- [ ] No secrets, credentials, or `.env` files committed
- [ ] No unrelated changes included
- [ ] `CLAUDE.md` updated if project structure or workflow changed
- [ ] `docs/technical/architecture.md` updated if an architectural decision was made

## Screenshots / recordings

<!-- For UI changes: before and after. Delete this section for non-visual changes. -->

| Before | After |
|--------|-------|
|        |       |

## Notes for reviewer

<!-- Anything the reviewer should know: tricky areas, known limitations, follow-up work. -->
