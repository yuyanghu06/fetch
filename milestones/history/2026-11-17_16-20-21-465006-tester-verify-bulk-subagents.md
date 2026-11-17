# Verify bulk clipboard use with disposable Workspace files

- Local timestamp / acting role / milestone: 2026-11-17T16:20:21-05:00 / Tester / Bulk subagents
- Requested scope: Verify the toolbar, selected App and folder, immediate Paste all, no overwrite, and conversion results.
- Starting repository state: Bulk source and approved immediate paste were in place.
- Work performed / changed files: Added one widget scenario to workspace_tests.rs that uses a temporary HTTPS instance of the real Workspace router. Adapted the existing two-logo widget probes to distinguish the clipboard icons. Exercised the toolbar and read/write helpers against disposable app directories.
- Decisions: Use fixture clipboard messages rather than the system clipboard.
- Checks and evidence: Three focused tests passed. The real file APIs excluded other app and scope, preserved exact same-app bytes and nested directories, refused an occupied name, created eligible files, and left a conversion needing review untouched. Final repeat passed in 3.38 seconds. See [results](../bulk-subagents/info.md).
- Resulting state / acceptance: The workflow and minimum-window placement pass.
- Remaining work and handoff: Finish signed build and live user clipboard observation.
