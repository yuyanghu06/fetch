# Keep Paste beside Copy and bring conflicts into view

- Local timestamp / acting role / milestone: 2026-11-17T16:07:07-05:00 / Orchestrator / Add subagent
- Requested scope: Address the two user-visible findings from the widget fixtures before delivery.
- Starting repository state: The shared Copy confirmation reserved 72 px to the left of Copy, between the two icons. The edit body could stay scrolled past its error after a duplicate Save.
- Work performed / changed files: copy_feedback.rs and components.rs move the reserved confirmation space before the Paste/Copy pair. Only Copy triggers it. workspace.rs scrolls an open draft to the top on a Save error to reveal the message and filename.
- Decisions: Keep the existing popup shell, copy feedback and scroll structure. Apply the scroll reset to new drafts.
- Checks and evidence: git diff --check passed. The disposable API test passed on unchanged backend source. Final widget retest is pending.
- Resulting state / acceptance: Both source corrections are in place.
- Remaining work and handoff: Confirm the two controls are adjacent, and repeat the collision and rename actions from the smallest popup.
