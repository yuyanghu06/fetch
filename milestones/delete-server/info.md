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
Tester/Evaluator ACCEPT after owned-worker and fenced-Quit corrections. Later
compilation/UI-check evidence is in the server-delete-control milestone. Actual
backend install/delete diagnostics now pass; native window/Keychain acceptance
remains separate.

The earlier queued shared-authentication task was completed as a separate source
milestone; that handoff is superseded by shared-mcp-authentication/info.md.

November3 deletion-latency investigation and the subsequently approved speed fix
are complete. Baselines: Google75.225s, Everything20.400s. The wrapper now flushes
once after each selected subtree, including missing-target retry; recursive
safety checks and credential/config durability remain unchanged.

Independent storage tests6/6 PASS, both actual confined repeats PASS, native
binary builds and independent source/evidence evaluation ACCEPT. Same packages
and entry counts: Google now0.889s (74.336s saved), Everything0.262s (20.138s saved,
with real connection/echo). See [optimized evidence](optimized-evidence.md) and
[baseline evidence](latency-evidence.md). All disposable state/processes removed;
personal clients/accounts/services preserved. Existing Fetch needs Quit/reopen
to use the new binary. Native window/HTTP/real Keychain/Google-account checks
remain separate. No release or packaging action occurred.
