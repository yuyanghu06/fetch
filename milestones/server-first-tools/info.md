# Server-first MCP tools

Goal: connect without collecting every upstream tool. Publish a fixed small Fetch catalog, include the complete permitted server ID/name list at startup, and load raw tool definitions only for explicitly requested servers. Calls use a fixed Fetch forwarding tool.

User approved implementation on November12 and, after the approval-screen difference was explained, accepted the fixed three-tool approach. The earlier connection-specific catalog expansion is superseded because current MCP forbids it.

Scope: fixed MCP discovery/calling helpers and model instructions; parallel selected-server catalog reads; transactional grouped alias persistence that skips unchanged saves. Existing management tools, raw upstream definitions/arguments/results, permissions, queues, cancellation and interactions remain backend-owned. No new HTTP routes, auth changes, project scoping, personal settings or live client tests.

Completion: focused disposable gateway/stdio/performance tests, signed build-only and independent review. Actual Claude/Codex/Mac acceptance remains manual and must be reported separately. Status: source/fixtures/signed build accepted; actual client/native acceptance remains manual.

See [contract](../../docs/architecture/server-first-tools.md). Root serializes journal writes; specialists preserve unrelated Workspace/server-Off changes.

Performance source pass integrated: selected catalog reads run concurrently (cap8); ordered transactional alias batches skip unchanged saves. Source reviewed; independent fixture checks pending.

Fixed-flow source integrated. Both startup revisions carry complete permitted IDs/names and model guidance; native tools/list no longer contacts upstreams. Raw generic dispatch reuses existing runner continuation/events/permission path. Independent tests/review/build pending.

18API/MVP/raw/new checks pass. Prior migrated13connector checks pass; additionalmodern and observednull-error checks awaitfinalrun. Independent preliminary review finds no source blocker; build/finalACCEPT pending. One archive compile hit disk space; retry passed. No personal files/services changed or real clients launched.

Final scoped verification:34passing focused checks; signed build-only15.08seconds/strict verification; independent Evaluator ACCEPT. [Evidence/manual next steps](evidence.md). Earlier in-progress status paragraphs above are historical. No app/client relaunch was performed.
