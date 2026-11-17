# Show Loading... in the waiting panel

- Local timestamp / acting role / milestone: 2026-11-17T17:59:58-05:00 / Orchestrator / Workspace loading status
- Requested scope: Show the exact Loading... status during the blank dimmed Workspace panel state.
- Starting repository state: The shared dimmed layer covered waiting Sessions, conversations and subagent popups with no text.
- Work performed / changed files: Updated src/ui/components.rs and the Sessions widget check in src/ui/workspace_tests.rs. Updated the shared component and UI verification guides.
- Decisions: Center Loading... with the existing muted panel text layout. The label appears only for the existing pending overlay.
- Checks and evidence: The focused test passed. It checks centering on an empty initial load, visibility over retained rows during refresh and removal when the list arrives. Reviewed the offscreen render. Signed build completed in 9.23 seconds and strict signature verification passed.
- Resulting state / acceptance: Ready after reopening Fetch.
- Remaining work and handoff: Live app behavior was outside the offscreen fixture.
