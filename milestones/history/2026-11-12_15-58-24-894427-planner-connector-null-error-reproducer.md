# connector null error reproducer

- Local timestamp / acting role / milestone: 2026-11-12T15:58:24.894427-05:00 / planner / server-first-tools
- Requested scope: Selected fixed tool flow, focused verification and diagnosis of reported connector failure.
- Starting repository state: Implemented fixed discovery/call and performance source; prior real client failure unresolved; unrelated work retained.
- Work performed / changed files: User provided exact otherconnection/endedrequest error. Planner source reproducer: idless roots/list_changed notification gets backend400JSONRPC id:null -32601 unsupported; connector mistakes null for a foreign active ID. This is a reproducible explanation of the misleading message, not proof the live Claude request was that notification. Minimal correction retains actualcode/message/details/context and strict nonnull ownership, without inventingIDs or silently adding notification support.
- Decisions: No real clients/personalstate; no extra auth/routes/connectionstate. Preserve errors and raw payloads.
- Checks and evidence: Logs /tmp/fetch-server-first-tests.log, /tmp/fetch-server-first-api-tests.log and /tmp/fetch-server-first-connector-tests.log. Compiling a normal library archive once hit disk space; retry passed when space became available. Root's bounded old-cache cleanup selected/removed0directories.
- Resulting state / acceptance: API/source evidence positive; final connector checks/signed build/evaluation pending.
- Remaining work and handoff: Finish observed-error diagnostic regression, final focused suite and signed build-only; actual client success remains manual.
