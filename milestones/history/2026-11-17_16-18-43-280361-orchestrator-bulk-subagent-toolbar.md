# Wire Copy all and Paste all into Workspace

- Local timestamp / acting role / milestone: 2026-11-17T16:18:43-05:00 / Orchestrator / Bulk subagents
- Requested scope: Put the shared pair immediately before Add subagent and run the selected immediate workflow.
- Starting repository state: Bulk helper and shared single-agent controls were available.
- Work performed / changed files: workspace.rs shares the visible app filter with Copy all and captures the selected folder and app. Paste reads the clipboard then calls the existing APIs. It refreshes the shown files after additions and reports paths and returned errors for items it could not add. components.rs shares the glyphs with the popup.
- Decisions: Successful Copy may finish after the user changes the app selector. An already-started paste uses the captured destination.
- Checks and evidence: git diff --check passed. Focused checks and independent review are running.
- Resulting state / acceptance: Source ready for verification.
- Remaining work and handoff: Verify minimum-window layout and disposable same-app, cross-app and collision paths. Rebuild and sign.
