# Plan the server header display change

- Local timestamp / acting role / milestone: 2026-11-07T18:25:29.915209-05:00 / planner / server-header-auth-status
- Requested scope: Hide authenticated/not_required authentication labels in code and Figma.
- Starting repository state: Earlier neumorphic Figma baseline and icon pass complete; existing unrelated changes preserved.
- Work performed / changed files: Read-only Planner scoped the condition in src/ui/mod.rs; no backend or control changes.
- Decisions: Keep this pass bounded to the user-selected presentation changes; no API or authentication behavior change.
- Checks and evidence: Root inspected final source diff against the bounded plan.
- Resulting state / acceptance: Plan implemented.
- Remaining work and handoff: Retain other authentication statuses.
