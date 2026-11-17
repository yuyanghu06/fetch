# Create shared compact errors and steady clipboard controls

- Local timestamp / acting role / milestone: 2026-11-17T16:31:44-05:00 / Coder / Subagent feedback
- Requested scope: Provide the requested error/info detail and prevent the clipboard press paths from flashing.
- Starting repository state: Clipboard icons used the general changing press decoration. Bulk and editor errors were large text.
- Work performed / changed files: components.rs adds a native hover tooltip for the compact error, and uses a stable original raised face around the bulk icons. surface.rs provides that shared raised control wrapper.
- Decisions: Keep the full returned text and the existing popup clipboard controls. Use Iced opaque to stop a disabled hitbox from starting a drag.
- Checks and evidence: git diff --check passed. Widget captures are pending.
- Resulting state / acceptance: Controls source ready.
- Remaining work and handoff: Check normal, hovered, pressed and busy-rendered surfaces.
