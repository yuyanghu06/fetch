# Polish install checkboxes, Instructions Paste and key glow

- Local timestamp / acting role / milestone: 2026-11-17T18:40:33-05:00 / Orchestrator / Native UI follow-ups
- Requested scope: Show white selectable install boxes, gray out unsupported rows, put failure details behind the status info icon, add Paste to instruction editors and tint auth hover gray.
- Starting repository state: Selectable unchecked boxes blended into the modal. Unsupported rows used a different disabled palette. Failed items showed a paragraph. Instruction headers had Copy alone. The shared key used the blue Refresh glow.
- Work performed / changed files: Updated src/ui/mod.rs, components.rs and theme.rs for install styles and failure details. src/ui/workspace.rs adds clipboard reading for the still-selected instruction draft. src/ui/surface.rs supplies the key glow.
- Decisions: Reuse the shared hover tooltip and clipboard controls. The Paste event checks the selected app, file and view before editing. An installed gray checkbox keeps its checkmark.
- Checks and evidence: Inspected the working tree and ran git diff --check.
- Resulting state / acceptance: Implemented without changing install or file API contracts.
- Remaining work and handoff: Verify focused widget behavior and rebuild the signed executable.
