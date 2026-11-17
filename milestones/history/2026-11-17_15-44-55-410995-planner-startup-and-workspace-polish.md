# Plan startup and Workspace UI polish

- Local timestamp / acting role / milestone: 2026-11-17T15:44:55-05:00 / Planner / Startup and Workspace polish
- Requested scope: Make server cards pop in one by one at startup, invert the shared Finder button colors, fix subagent preview scrolling while keeping the popup, remove the Copied agent message and center empty-panel text.
- Starting repository state: Clean working tree at 6543cf3. Existing pop-in draws all cards at 92% width or larger even before their stagger. The subagent preview has nested scroll areas and lets its path scroll out of view.
- Work performed / changed files: Read native rendering, refresh handling, Workspace preview, and existing offscreen tests. The orchestrator recorded the scope and assigned separate source and test files.
- Decisions: Use the first startup list, hidden delayed rows and a small overshoot. Keep the existing manual refresh motion. Use the blue shared button style with a filled white Finder mark. Keep popup controls and path fixed above one scrolling preview. Remove the Copy message while preserving the clipboard action. Center standalone empty messages inside their panels.
- Checks and evidence: Read pinned Iced text editor and redraw code. No tests ran in this pass.
- Resulting state / acceptance: Implementation and offscreen verification are in progress.
- Remaining work and handoff: Verify the first three redraw stages, settled and refresh states, Finder appearance and action, long preview scrolling, Copy and empty-panel layout. Build the signed development binary without launching.
