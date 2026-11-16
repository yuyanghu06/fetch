# Server-first MCP tools

## Selected behavior — November12,2026

User requests server-first default, selected-server optimizations, model instructions and the full permitted server ID list at startup. User accepts a fixed Fetch forwarding tool after explanation that Claude/Codex approve that generic tool rather than each upstream tool. No further permission prompt is required for this selected implementation.

The earlier recommendation to expand each connection's callable catalog after selection is superseded. Current MCP2026-07-28 tools specification forbids catalogs varying per connection or as a side effect of another request. The legacy2025-11-25 text lacks that prohibition; Fetch needs one coherent supported default. See [current tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools) and [legacy tools](https://modelcontextprotocol.io/specification/2025-11-25/server/tools). The original recommendation missed this restriction; it is corrected before that implementation.

## Existing endpoint, fixed tools

All client operations use existing POST /mcp. No new HTTP route, connection header, selection session or authority is introduced. Native tools/list returns only permitted Fetch-owned helpers and existing management tools for callers already holding management permission; it does not collect/export upstream tool definitions.

| MCP tool | Inputs | Reads/calls and output | Permission |
| --- | --- | --- | --- |
| fetch__list_servers | Empty object | Reads installed records and current local runtime snapshots. Returns status:ok, servers:[{server_id,name,status}], next_steps:[]; includes Off/disconnected records truthfully. No upstream catalog/start/login. | fetch:catalog, filtered per server |
| fetch__list_tools | Nonempty server_id string array; existing optional view | Reuses existing filtered catalog read/result envelope. Requests only selected permitted installed servers, at most8 concurrently, retaining requested order and existing partial/unavailable/raw-definition behavior. Omitted/empty selection is invalid on MCP. | fetch:catalog, filtered per server/tool |
| fetch__call_tool | server_id, tool_name, arguments object | Routes original tool name and untouched inner arguments through existing Runner::invoke. Returns native upstream ToolResult through existing revision rules, without the management JSON-to-text wrapper. | fetch:tools:call for original server/tool |

Direct HTTP GET /api/v1/tools retains its existing optional server_id filter and omitted-filter behavior; its selected acquisitions gain the same parallelism. Existing management routes and privileges remain. Missing/hidden/invalid targets retain existing failure rules; discovery never grants permission, automatically starts/authenticates, retries uncertain calls or hides source errors.

Generic calling retains outer requestState/inputResponses/_meta on every round while replacing only routing name and arguments for upstream. Existing continuation checks bind owner/server/generation/method/original upstream arguments/deadline. Progress, cancellation and supported interactions use the same InvocationContext. A static generic caller cannot publish each upstream outputSchema; raw definitions remain available through selected listing and results remain native.

## Startup and model instructions

Include every permitted installed server ID and name in Fetch-owned startup instructions and the server-list tool description. Use valid MCP fields/channels, never stdout diagnostics or appended text on upstream tools/results. No truncation or hidden server metadata. Agent guidance: inspect inventory; choose servers needed for the user's task; read only those definitions; use exact original names/input schemas when calling; never guess IDs, load all servers by default, or invent/start login flows; surface unavailable/server errors and respect permissions. Existing explicit user requests to inspect multiple servers can select them together.

Catalog-only inventory excludes launch configuration, tokens, credentials and source paths. Existing fetch__get_servers remains management-only. Full names/IDs come from local installed metadata, so startup does not wait on upstream tool collection.

## Performance and boundaries

Parallelize selected live catalog acquisition with existing futures_util; retain order/permission/error/raw behavior and existing collection deadlines. Do not add caching/stale fallback or change call timeouts. Group alias assignments under one lock, clone the current map, persist changed candidate once, then commit. Repeated unchanged mappings do not rewrite aliases.json; failed persistence does not commit new in-memory mappings. Native fixed discovery itself no longer needs per-upstream aliases.

## Verification and current limits

Implementation,34focused checks, signed development build and independent source/fixture review ACCEPT. Actual clients remain user-manual. Focus disposable tests on startup inventory/no upstream requests, selected-server parallelism, unchanged raw call/results, permissions, cancellation/progress/interactions, modern continuation routing, and transactional durable aliases. Preserve existing live Claude tools-fetch failure as unresolved until actual success is observed. No personal settings/credentials/trust changes or new live client/account tests; user owns manual actual-client acceptance. No release/packaging/notarization/project scope/authentication changes.

Final evidence: [34focused checks, signed build and manual handoff](../../milestones/server-first-tools/evidence.md). The reported SDK failure is not claimed solved; null-ID rejection diagnostics now retain its real backend cause instead of claiming cross-connection delivery.
