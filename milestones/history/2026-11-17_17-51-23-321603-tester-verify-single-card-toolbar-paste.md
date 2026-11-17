# Verify toolbar Paste from one copied card

- Local timestamp / acting role / milestone: 2026-11-17T17:51:23-05:00 / Tester / Workspace agent actions
- Requested scope: Check that the toolbar creates the single copied definition immediately.
- Starting repository state: The toolbar accepts both clipboard formats, and Copy records the selected card path.
- Work performed / changed files: Extended src/ui/workspace_tests.rs to copy a registered card that links to a differently named file, click toolbar Paste and run the native Save path in a temporary workspace.
- Decisions: Use the production clipboard encoder and simulate delivering the system clipboard result.
- Checks and evidence: Two focused widget tests passed in 3.09 seconds. The selected card produced exactly one new file called review.v2.toml with the original content. The tests also cover existing filename conflicts, Copy all, row Paste and popup Copy.
- Resulting state / acceptance: PASS. The save is queued without opening a draft.
- Remaining work and handoff: Live macOS clipboard observation remains after reopening Fetch.
