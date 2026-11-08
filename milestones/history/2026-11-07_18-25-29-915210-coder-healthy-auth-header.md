# Hide healthy authentication labels

- Local timestamp / acting role / milestone: 2026-11-07T18:25:29.915210-05:00 / coder / server-header-auth-status
- Requested scope: Same bounded server-header request.
- Starting repository state: Earlier neumorphic Figma baseline and icon pass complete; existing unrelated changes preserved.
- Work performed / changed files: Coder changed only src/ui/mod.rs: use name/runtime alone for authenticated or not_required, otherwise retain the exact authentication suffix.
- Decisions: Keep this pass bounded to the user-selected presentation changes; no API or authentication behavior change.
- Checks and evidence: Root inspected the two-line conditional and unchanged callbacks/availability.
- Resulting state / acceptance: Implementation complete.
- Remaining work and handoff: Independent render/build and review recorded separately.
