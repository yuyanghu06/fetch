# Default window size and shared Markdown styling

- Local timestamp / acting role / milestone: 2026-11-16T13:44:33.742576-05:00 / Planner, integrated by Orchestrator / workspace-session-error
- Requested scope: use user's pictured current Fetch window size as startup default; remove black fenced/inline Markdown code styling and match inline text size; tool Copy explicitly includes name and result together.
- Starting repository state: prior Workspace presentation follow-up accepted; extensive other changes preserved.
- Work performed / changed files: read-only Planner inspects window_settings, cached Markdown and Iced Viewer/Style. Orchestrator runs a disposable CoreGraphics metadata reader and immediately cleans its source/binary.
- Decisions: measured current Fetch bounds1058x789logical; change only opening size, preserve minimum. Reuse cached Markdown with supported custom Viewer for shared light inset fenced blocks and light inline highlight at normal14px font. Tool Copy exact format name + blank line + original raw result; user explicitly selects both, superseding prior output-only behavior. No personal/source/history/API changes.
- Checks and evidence: on-screen owner fetch-development, width1058 height789. Read-only metadata only, no window/app mutation. Local Iced default uses dark code surface and different inline font. Some existing pixel fixtures use874stride despite default-size views; make those calculations follow actual fixture width as part of verification.
- Resulting state / acceptance: bounded plan accepted; implementation next.
- Remaining work and handoff: shared rendering/default-size changes, focused tests/captures, signed build and independent review. No native relaunch or personal-file edits.
