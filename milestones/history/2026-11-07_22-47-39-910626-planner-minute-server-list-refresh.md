# Plan one-minute server list refresh

- Local timestamp / acting role / milestone: 2026-11-07T22:47:39.910626-05:00 / independent Planner, recorded by root / periodic-server-refresh
- Requested scope: User asked runner refresh GET every minute; clarified app server list through GET /api/v1/servers.
- Starting repository state: Prior healthy-auth header source change and Figma/docs work remain uncommitted; preserve them.
- Work performed / changed files: Read-only Planner/root traced Desktop::refresh→NativeClient::servers→existing GET. Existing1sTick reads expanded consoles and pending auth/deletion; installation/restart have their own1sreads. Native client/session are removed on window close. Lower Runner has no management GET client/authority and is not selected. Root creates scope/journal files.
- Decisions: Add independent60second UI timer using existing refresh/pending-read ownership. Preserve all faster reads and manual Refresh; no new API or lower-runner permission.
- Checks and evidence: src/ui/mod.rs, client.rs, auth_requests.rs, application.rs and background authority tests inspected without running app/services/tests. User's explicit clarification selects app behavior.
- Resulting state / acceptance: Bounded plan assigned to Coder owning src/ui/mod.rs.
- Remaining work and handoff: Inspect implementation; focused in-memory UI checks and compile; independent final review. No native launch/account/MCP/personal-state operations.
