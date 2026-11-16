# Persistent server on/off — selected November 12

User explicitly selects Off surviving Fetch Quit/relaunch until turned on. The operation extends the existing start/restart POST; no new route or config-PATCH field is selected. This contract is approved for implementation; source is implemented, 30 focused checks and the final signed build pass, and independent Evaluator ACCEPTs the bounded change. Live app/provider/client acceptance remains manual.

## Request and permission

`POST /api/v1/servers/{server_id}/restart` adds optional `action`, accepting `restart` or `stop`; omission means current restart/start behavior. The UI and `fetch__restart_server` management tool use the same backend. Existing `fetch:manage`, target visibility, HTTP authentication and JSON validation rules remain.

Stop body: `{"action":"stop"}`. Stop rejects `interactive:true` or `force_restart:true` with `400 INVALID_REQUEST`. There is no force-stop feature. Dispatch Stop before remote login renewal/provider access.

## What Off does

Under the per-server operation lock, reject active startup/installation follow-up, login, restart or active/queued/uncertain/metadata work with `409 SERVER_BUSY`. Fence new tool work atomically with checking idleness. Save `config.enabled:false` before closing; old records without this optional boolean mean enabled. Server reads expose the normalized preference separately from actual runtime.

Close the Fetch-owned local child, or Fetch's connection to a hosted service, through existing close handling with the existing five-second budget. Preserve packages/configuration/logins/logs/terminal state. Remote Off does not stop the provider itself. Keep retryable child ownership until closure is confirmed. Invalidate old catalog/continuations and send the existing tools/resources/prompts list-change notifications. An accepted stop worker continues if its HTTP/MCP caller disconnects.

Confirmed stop returns `200` with `server_id`, `status:"stopped"`, `enabled:false`, `changed` and `next_steps:[]`. Already disabled and confirmed stopped returns `changed:false`. Return existing `404 SERVER_NOT_FOUND` and permission failures. Failure saving the preference returns `500 CONFIG_WRITE_FAILED` without closing. Failure confirming close returns `503 SERVER_STOP_FAILED`, retains the disabled preference and actual failed runtime; never report stopped success. Repeating Stop retries any unconfirmed live context.

## Turning on and staying off

Existing explicit start/restart POSTs set enabled true and use the existing accepted result (`202` and restart ID while pending), readiness/failure and GET monitoring. Interactive explicit restart can enable then authenticate; direct deprecated authentication honors Off. Internal connection attempts cannot reenable a disabled server. Startup, install follow-up, background renewal, preflight and delayed starts check current saved state under the same server gate. Preserve existing call admission and raw MCP inputs/results.

## Native toggle

Use a shared toggle before the server card's existing actions. It reads config.enabled and calls the common POST; it does not manage processes or persistence. Reconcile actual returned/read state before treating an action as complete. Preserve stop/on errors in a per-server Console path, opening that Console on failure. Do not label a stop failure Restart failed or restore removed global banners.

## Verification and boundaries

Use a few existing confined/disposable server fixtures: stop/on through the product route, durable off after a fresh Manager/startup, busy work preserved, disabled background paths silent, HTTP/MCP forwarding/permission and honest close failure; actual native toggle clicks/appearance. No personal servers, accounts, credentials, client settings or services. Planner assigns production files to one Coder and test files to Tester; independent Evaluator reviews after verification. Parent serializes journals. See ../../milestones/server-on-off/info.md.

## Current acceptance

30 focused checks pass: 3 confined HTTP/MCP product scenarios (0.90s), 2 private no-network/background/conflict checks (0.07s), and 25 actual widget checks (9.88s). Inspected minimum-window toggle and Off/On/busy Console PNGs. Final signed-development build-only passes (24.59s) and strict signature verification passes. Independent Evaluator ACCEPT; no source fixes requested. Existing compiler/deprecation warnings remain.

Fetch was not relaunched. Actual personal/provider accounts, client connections and macOS approvals were not tested; those remain manual acceptance. Failed-close and caller-disconnect behavior were reviewed in source through the reused retained-child close path and detached worker, without fault injection. No personal config/credentials/trust, running services, Figma, commit/push or release changes. Explicit Quit/reopen is needed to see the rebuilt app.
