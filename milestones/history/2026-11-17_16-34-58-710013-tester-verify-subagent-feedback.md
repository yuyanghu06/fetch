# Verify toolbar and Save feedback at minimum window size

- Local timestamp / acting role / milestone: 2026-11-17T16:34:58-05:00 / Tester / Subagent feedback
- Requested scope: Check exact success text, five-error hover details, Save placement, checkbox removal and press behavior.
- Starting repository state: Bold Add, stable clipboard controls, tooltip and Workspace feedback source present.
- Work performed / changed files: Updated the existing Workspace bulk and draft fixtures. They drive real widget presses, test disabled capture, and use disposable Workspace file operations. A first comparison detected unrelated rotating Refresh pixels and a second retry needed the fixture's normal relayout. Each fixture issue was corrected.
- Decisions: Compare the tooltip area and real press outcomes rather than requiring every window pixel to be fixed.
- Checks and evidence: Four final focused checks passed in 5.03 seconds. Confirmed Copied all!, Pasted all!, the full five-error hover at 760 by 520, and error beside Save without the checkbox. Fresh rediscovery shows successful pasted cards. No DragWindow from a busy icon press.
- Resulting state / acceptance: Widget and disposable API checks pass. See [captures](../subagent-feedback/info.md).
- Remaining work and handoff: The native macOS compositor was outside the fixture, so the user's click after restart remains the live flash check.
