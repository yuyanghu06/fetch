# Load models on first editor open and cap the menu

- Local timestamp / acting role / milestone: 2026-11-17T17:36:22-05:00 / Coder / Workspace agent actions
- Requested scope: Start the existing model read when Add or Edit opens and show up to four visible rows.
- Starting repository state: Refresh and post-conversion could load the list. Other editor entry points did not start a read.
- Work performed / changed files: workspace.rs reuses the native catalog task from the editor entry points, keeps already loaded results and uses Iced menu_height for the cap while keeping the menu scrollable.
- Decisions: Keep the optional model field, the existing app resolution and manual Refresh.
- Checks and evidence: Read the pinned Iced row height and overlay API. git diff --check passed.
- Resulting state / acceptance: Source ready for widget tests.
- Remaining work and handoff: Check the initial request and scrolling through more than four rows.
