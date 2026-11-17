# Verify Add, Paste and the duplicate-save flow

- Local timestamp / acting role / milestone: 2026-11-17T16:08:36-05:00 / Tester / Add subagent
- Requested scope: Exercise Add, copy-adjacent Paste, explicit Save and recovery from an occupied filename.
- Starting repository state: The create-only backend had passed its initial test. The popup spacing and scroll corrections were present.
- Work performed / changed files: Exercised the actual Subagents button, blank native editor and fixture clipboard. The test reaches Save at the minimum window, forwards the rejection scroll action and retries with a new filename. The actual HTTP route was tested with temporary files for duplicate, new and edit saves.
- Decisions: Keep final screenshots and a compact result log in the milestone. Use fixture clipboard rather than the system clipboard.
- Checks and evidence: Five Workspace widget checks passed in 4.16 seconds. One Workspace HTTP check passed in 0.03 seconds. The orchestrator ran the existing Copy confirmation case, which passed in 1.60 seconds. See [test and render evidence](../add-subagent/info.md).
- Resulting state / acceptance: Confirmed blank draft, adjacent buttons, focused plain paste, visible conflict and preserved definition, valid create and regular edit.
- Remaining work and handoff: Independent review and signed build. macOS live app and clipboard remain for user observation.
