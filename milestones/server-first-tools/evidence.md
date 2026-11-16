# Server-first tools — evidence and handoff

November12,2026. Source, focused fixtures and signed development build accepted by the independent Evaluator. Actual Claude/Codex and native Mac approval remain manual. No live client success is claimed.

## What changed

- Initial MCP discovery returns fixed Fetch list-servers/list-selected-tools/call-tool helpers plus already-permitted management tools. It no longer collects every upstream tool.
- Both supported startup paths include every permitted installed server name/ID in instructions; the list-server tool description repeats the complete inventory for model-visible discovery. Local metadata includes stopped/Off servers without starting/contacting them.
- fetch__list_tools requires explicit nonempty server_id on MCP. Selected catalogs run concurrently, up to8 at a time, preserving order/raw values/errors/permissions. Direct HTTP GET tools retains its prior optional-filter contract.
- fetch__call_tool takes server_id, original tool_name and inner arguments. It uses existing raw runner execution/results, queues/deadlines/progress/cancellation/interactions and continuation bindings. Generic client approval display was explicitly accepted; no upstream outputSchema is advertised on the generic caller.
- Tool-name mappings stage and save one changed batch; unchanged mappings do not rewrite the aliases file and failed saves do not commit live changes.
- Agent descriptions explain using startup IDs, selecting only needed servers, reading original schemas, calling exact names/arguments and not guessing/bulk-loading/authenticating automatically or replaying unknown outcomes.

## Files

| File | Responsibility |
| --- | --- |
| src/mcp/management.rs | Fixed helper schemas, permissions, model guidance and inventory text |
| src/mcp/mod.rs | Startup inventory, fixed native catalog and raw generic dispatch |
| src/api/runtime.rs | Safe catalog-only server inventory, explicit MCP selection validation and concurrent selected live reads |
| src/runner/catalog.rs and src/runner/mod.rs | Stable transactional alias batches and existing call execution reuse |
| src/connector/mod.rs | Accurate null-ID backend error diagnostics, strict non-null ownership retained |
| tests/server_first_tools.rs |4actual gateway/API/performance/continuation/durability fixture cases |
| tests/connector_support/checks.rs |16stdio/HTTPS/profile/concurrency/interaction/diagnostic checks |
| tests/local_mvp_acceptance.rs | Focused existing expectation/mock updates |

No new HTTP route, generic call through the management text wrapper, per-connection catalog expansion, auth profile migration, personal client/trust edit, provider/client experiment or release was introduced. Existing Workspace/server-Off/UI work is preserved.

## Actual verification

| Command | Result |
| --- | --- |
| cargo test --locked --test server_first_tools --test local_mvp_acceptance --test raw_json |18passed:4newserverfirst,13existingMVP,1rawJSON |
| cargo test --locked --lib connector:: |16passed, including modern stdio/pinned HTTPS startup/selection/call, concurrent connections, progress/interactions, immediate cancellation, reconnect/restart/no replay and approval-state fixtures |
| ./scripts/run-development.sh --build-only |Pass,15.08seconds; separate signed com.fetch.local.development executable |
| codesign --verify --strict --verbose=1 target/debug/fetch-development |Valid on disk, satisfies designated requirement |
| git diff --check |Pass |

34focused passing checks total. Source tests use owned loopback peers, disposable state and memory credentials; no production Keychain/native approvals or real coding clients run. Existing warnings remain. Independent Evaluator ACCEPT confirms source and evidence.

The first4-case run exposed a test-parser assumption that HTTP403must contain an MCP event; fixture corrected while preserving403/no-dispatch assertions. One subsequent compile hit No space left on device while building the library archive. The same checks passed after space became available. Root's bounded old-cache selection removed0directories; no personal files, binaries, state or services were deleted. These issues did not weaken production checks.

## Reported Claude error

The screenshot's exact other-connection/ended-request diagnostic has a reproducible path: unsupported idless notification gets a backend400JSON-RPC error with id:null; old bridge misclassified null as a foreign request ID. The new checks preserve HTTP400, original method and -32601/message while keeping unknown non-null response IDs strictly rejected, including token redaction.

This corrects error reporting. It does not add support for that notification, prove which notification the real Claude session sent, or establish that the live SDK failure is solved. Catalog latency attribution also remains unmeasured. Source/fixture acceptance is separate from real-client acceptance.

## Manual next action

Explicitly Quit Fetch, reopen the signed development executable, then use Reconnect in Claude/Codex. Existing stdio registration and saved approved profile can be reused. Confirm fixed startup helpers/full IDs, request one server's tools, make a deliberately approved call, and record the actual outcome/error. No reinstall/config/trust change is required to pick up this build.

There is one connector process per client MCP connection, sharing Fetch's backend. Separate coding sessions usually launch separate connectors; subagents inside one session may share the host's MCP connection. A saved claude-code profile can be shared across those connections, separately from Codex and other Fetch installations.
