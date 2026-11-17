# Update the shared Finder button and Workspace popup

- Local timestamp / acting role / milestone: 2026-11-17T15:46:52-05:00 / Coder / Startup and Workspace polish
- Requested scope: Invert Finder colors, keep the popup while fixing preview scrolling, remove the Copied agent line, and center standalone empty Workspace messages.
- Starting repository state: Plan recorded. The server animation and test files were assigned separately.
- Work performed / changed files: components.rs adds a white filled Finder face with blue cutouts on the existing blue button style, a selectable full-height file preview, and a shared centered empty message. workspace.rs pins the path in the preview body, uses one scroller below it, removes the copy status and applies the shared empty state.
- Decisions: Preserve the popup shell, Edit controls, Finder action and clipboard data. Keep loading and mixed error states in their usual layout.
- Checks and evidence: Reviewed source and ran git diff --check on the Workspace changes. Focused widget checks are pending.
- Resulting state / acceptance: Source implemented. Appearance and scrolling still require widget verification.
- Remaining work and handoff: Tester checks long preview scrolling, rendered Finder pixels, Copy action and centered messages.
