# Plan compact clipboard and Save feedback

- Local timestamp / acting role / milestone: 2026-11-17T16:31:44-05:00 / Planner / Subagent feedback
- Requested scope: Move bulk feedback to the left of the controls, add the exact success text, use a gray hover info icon for errors, remove the review checkbox and fix the click flash.
- Starting repository state: Bulk errors filled the list, and a checkbox gated converted drafts.
- Work performed / changed files: Read the native Iced button, tooltip and layout paths. The face switches from white to blue on press, and a disabled press can fall through to the native window-drag action.
- Decisions: Use a stable toolbar face, capture press within each icon, and keep full details behind the compact marker by the relevant control.
- Checks and evidence: Source findings established the two press paths. Native macOS flash itself has not been reproduced.
- Resulting state / acceptance: Bounded plan ready.
- Remaining work and handoff: Implement and verify the small window and press frames.
