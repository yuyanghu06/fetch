# Explain the current installation route

- Local timestamp / acting role / milestone: 2026-11-03T17:12:59.367921-05:00 / Orchestrator / shared-mcp-authentication behavior walkthrough.
- Requested scope: Explain how the install route works now.
- Starting repository state: Shared auth router delivered; Google pending; distinct imported entries accepted for multiple instances, no duplicate control requested. Existing documentation edits preserved.
- Work performed / changed files: Read HTTP/MCP adapters, Installer submit/prepare/progress and package/script preparation, discovery capture and Manager connection hook. Updated snapshot/to-do and installation guide with source-inspection reference. No application changes.
- Decisions: None added. Current POST /api/v1/setup/installations requires management authorization and config_path/server_name selections, returns202 with installation_id. One active group prepares rows sequentially. Existing exact source path/name reuses saved instance. New rows reread/classify source, prepare private npm/Python packages or retain scripts/remote URLs, save names/references without imported env/header values, then attempt normal Connect. Preparation persists despite login/connection failures; ready reused servers are not interrupted.
- Checks and evidence: src/api/mod.rs69-70; src/api/runtime.rs26-35 and145; src/installation/mod.rs45-118,120-208; src/discovery/mod.rs98-146; src/manager/mod.rs52-88. Source-only inspection; no install/build/test/provider/helper/client operation.
- Resulting state / acceptance: Current request/progress, execution, credentials and error boundaries ready to explain. Preparation errors currently use generic INSTALLATION_FAILED; runtime/auth state is read separately through GET servers.
- Remaining work and handoff: Continue user behavior walkthrough without silently changing installation/authentication contracts; preserve Google deferral and no-extra-instance-control decision.
