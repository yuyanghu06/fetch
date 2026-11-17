# Replace the selected definition with row Paste

- Local timestamp / acting role / milestone: 2026-11-17T17:36:22-05:00 / Coder / Workspace agent actions
- Requested scope: The user selected immediate row Paste. Use the first copied definition for the selected file and create the rest as new files.
- Starting repository state: Toolbar Paste all handled a multi-agent clipboard bundle with create-only saves.
- Work performed / changed files: workspace/bulk.rs accepts single and bulk Fetch clipboard bundles on a selected row, uses ordinary Save for the first selected path and create-only saves for the rest, and preserves per-file outcomes. components.rs provides a shared 40px Copy, Paste and X row.
- Decisions: Keep other filenames protected. Apply only conversions without pending issues.
- Checks and evidence: Source diff reviewed. Focused tests pending.
- Resulting state / acceptance: Helpers and controls ready.
- Remaining work and handoff: Complete action dispatch, the Remove confirmation and disposable real-use checks.
