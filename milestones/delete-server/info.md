# Delete a Fetch-managed MCP server

Goal: provide DELETE /api/v1/servers/{server_id} through the existing management
authority and a shared backend, plus matching fetch__delete_server MCP tool.

Scope selected using the user's explicit best-judgment delegation: reject body/
query input, require fetch:manage for that server, return404 for missing targets
and409 for active installation/auth/restart/calls or unresolved remote outcomes.
No force option. Close the idle connection before deleting Fetch-owned config,
credentials/key/private state and unshared private installation packages. Preserve
source configs/scripts, shared runtimes, application identity and other servers.
Return200 JSON with server_id/status deleted/next_steps empty only on success;
report real stop/cleanup errors, retaining the record until cleanup can finish.

Acceptance: consistent HTTP/MCP permissions and errors, no stale reconnection,
safe contained cleanup and retry, disappeared catalogs/discovery installed state,
preserved unrelated files/accounts. Use URL-shaped API modules for the new route;
do not reorganize existing route families in this fix. Status: source implemented,
Tester/Evaluator ACCEPT after owned-worker and fenced-Quit corrections; Git
delivery pending. Compilation/native/runtime remain unverified for this change.

Next queued task: implement the user's shared asynchronous MCP authentication
request AFTER deletion delivery. Read current AGENTS.md, November3 upstream-auth
reuse notes and authentication-engine.md before that work. No auth code changes
belong in this delete milestone. No packaging/release or personal-state changes.
