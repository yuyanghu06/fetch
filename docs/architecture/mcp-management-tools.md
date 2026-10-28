# Fetch-owned MCP management tools

October 27, 2026. **Presented contract approved in D49, with D48 ongoing descriptive API coverage.** D47 separately approves the presented connection policy. Reuse the existing application APIs; no application implementation is authorized.

## Standing coverage requirement — D48

The user requires descriptive MCP tools for current/future Fetch product API operations. The eight tools below are the approved presented management mapping, not a permanently closed inventory. Describe each Fetch-owned operation's reads/changes, inputs, results/pending states, permissions and user handoffs; use the same backend API rules and caller authority. Preserve the existing Settings-only manual credential boundary, native OAuth/protocol roles and raw imported metadata. D48 accepts the general requirement; D49 accepts the presented names/input/result mapping below. Exact implementation mechanisms remain later review. Future selected memory APIs follow this rule without deciding their routes or release scope now.

## Routing and arguments

POST /mcp receives protocol messages. For tools/call, the handler reads params.name and params.arguments. An imported tool alias resolves to its exact server and original tool name; the argument object becomes the proposed tool-call API body unchanged, then the upstream tools/call arguments unchanged. There is no new input wrapper, field renaming, default insertion or conversion of the model's values. JSON whitespace/key order need not match.

Fetch-owned tools have separately defined input schemas. Their routing selectors become encoded URL/query values and the applicable remaining fields become the API body. Shared backend APIs retain business validation, authorization and effects; the adapter does not implement a second installation/auth/config/restart workflow.

## Approved presented inventory and input mappings — D49

The following are the eight tools approved at the presented scope. A question mark marks an optional field, not literal JSON syntax. Inputs are objects; IDs and paths are strings, args and ID filters are arrays of strings, force_restart is boolean, and environment is an array of name/reference objects.

| MCP name | Input object fields | Existing API mapping |
| --- | --- | --- |
| fetch__discover_servers | config_path? | GET /api/v1/setup/discovery; supplied config_path becomes its query value. |
| fetch__install_servers | servers: [{config_path, server_name}] | POST /api/v1/setup/installations; same servers body. |
| fetch__get_installation | installation_id | GET /api/v1/setup/installations/{installation_id}; no request body. |
| fetch__get_servers | id?: string[], restart_id? | GET /api/v1/servers; repeated id query values and optional restart_id. |
| fetch__list_tools | server_id?: string[], view? | GET /api/v1/tools; repeated server_id query values and optional view. D65 defaults an omitted tool view to the actual MCP caller revision; explicit raw is supported. |
| fetch__authenticate_server | server_id | POST /api/v1/servers/{server_id}/authenticate; body {}. |
| fetch__update_server_config | server_id, args?, working_directory?, environment?, connection?, execution? | PATCH /api/v1/servers/{server_id}/config; D63 permits remote connection.headers references, with backend server-kind validation. D68 adds execution.max_concurrent_calls/default_timeout_ms/tool_timeouts_ms under the same backend config authority. |
| fetch__restart_server | server_id, force_restart?: boolean | POST /api/v1/servers/{server_id}/restart; body contains the supplied flag, or {} when absent. |

Underlying API rules/defaults stay intact. Discovery's optional path has its existing default candidate behavior. Installation requires a nonempty servers list and each source pair. Server GET with restart_id requires exactly one id. Absent server/tool filters retain their existing permitted-all behavior. Config arrays replace their field, omitted fields are unchanged, working_directory accepts a string or null, and environment contains only {name, credential_ref} bindings. Empty config body remains a no-op. Absent force_restart uses the API's false default; the adapter does not insert its own defaults or coerce values.

Tool descriptions must state what is read or changed, applicable permissions, pending outcomes, human handoffs and the matching approved workflow. fetch__list_tools reads the canonical installed-server catalog; native protocol tools/list advertises permitted Fetch-owned tools plus individually aliased imported tools. These are distinct operations, not two names for the same list.

D68's approved execution amendment must be present in this tool's schema and description: optional `execution` has `max_concurrent_calls`, `default_timeout_ms` and `tool_timeouts_ms`. Forward supplied values unchanged to the same config PATCH; omission preserves settings and null removes only the named per-tool timeout override, never means unlimited. Preserve per-tool visibility/target permissions. Execution-only changes apply hot without restarting the child or invalidating provider-auth evidence; mixed patches retain the API's atomic validation/save and launch-change rules. This restores an omitted already-approved mapping, not a new tool or permission.

No Settings-only credential writer is exposed to agents. Configuration accepts credential references rather than manual values. No generic fetch__call_tool, shell, GitHub CLI, uninstall/update tool or new backend operation is added. GitHub CLI scope remains a separate proposal.

See [API contracts](api-contract.md), [restart contract](restart-contract.md) and [name policy](mcp-tool-names.md) for the existing behavior rather than deriving new rules from this table.

## Caller permissions and validation

Advertise and invoke tools only within the authenticated caller's underlying API permissions. Preserve caller identity; no elevated adapter credential or reserved namespace can bypass backend authorization, including server-specific restrictions. Exact OAuth scope names and Settings authority mechanisms remain step4 decisions.

Reject malformed MCP messages and unknown tool names through native handling. Validate Fetch-owned selector shapes for routing, encode path/query selectors safely and preserve supplied applicable values. Source reading, supported launch forms, references, active-operation conflicts and other business rules remain in the APIs. Imported-tool definitions and argument objects retain their separate raw-preservation contract.

## Approved presented results and errors — D49

For these Fetch-owned tools, represent a valid application API JSON response as one MCP ToolResult with exactly one text block containing that complete serialized JSON, including its existing next_steps. Do not duplicate it in structuredContent or a second text block; omit outputSchema for these text-returning Fetch-owned tools. D49 selects this mapping for these Fetch-owned tools; upstream schemas/results and native protocol fields stay unchanged.

| API outcome | MCP presentation |
| --- | --- |
| Application 2xx response | isError false; preserve all returned state and guidance in the single JSON text block. |
| 202 installation/auth/restart | Accepted/pending, not completed; return its IDs/state/guidance without automatically polling. |
| GET200 retrieving a failed job, incomplete catalog, required setup or unknown auth | Successful read with isError false; preserve the actual domain status rather than inventing successful completion. |
| Ordinary non-caller-auth 4xx/5xx application error | isError true; one text block with the complete safe application error JSON and next_steps. |
| Fetch caller auth/permission rejection, malformed MCP, unknown method/name | Native safe authentication/protocol handling under D43/D47; no fabricated application ToolResult. |
| No valid API JSON response | Existing D43/D44 Fetch-owned explanatory failure text, without raw transport diagnostics, fabricated successful state or mutation replay. |

Classify errors by ownership, not an upstream numeric status alone. A provider credential rejection retains CREDENTIAL_VERIFICATION_FAILED and the operation's application outcome; it must not trigger a Fetch caller OAuth challenge. Concrete verifier HTTP statuses remain open under D31. For uncertain mutations, preserve known not_started versus unknown outcomes; a lost response does not prove the operation failed or its effects stopped.

Imported tools still return the raw upstream ToolResult, including upstream isError, unchanged under D43. This Fetch-owned JSON text mapping never applies to those results. D44 Fetch explanations use the existing next_steps actor/action/instructions structure; no response_context or extra upstream notice returns.

## Lifecycle, defaults and boundaries

Each tool invokes its one existing operation. The underlying operation may start the explicitly requested installation, login or restart as already reviewed. The adapter does not automatically chain another operation, poll, retry, log in, restart or edit original harness configurations. Agents and native UI follow the same reviewed prerequisites/states/actions and human handoffs under D32/D33; the UI reads structured API state rather than guidance text.

Disconnect/reconnect does not replay mutations or imply cancellation/rollback. Preserve the operation's existing same-run result lookup, overlap and Quit rules. No new timer, task system, historical recovery, scope, SDK, numeric port or execution policy is selected.

## Acceptance evidence after authorization

Within the selected isolated test environment required by D46, verify all eight route/query/body mappings, safe selector encoding, omitted optional fields and unchanged values. Check permission filtering and no elevation, credential-writer absence, reference-only config inputs, single JSON text result, 202 versus completion and failed-job read behavior. Check native caller-auth versus provider/application errors, invalid/missing API responses, no automatic mutation replay/chaining, and unchanged imported definitions/arguments/results.

Documentation review is not runtime, SDK, OAuth, host/model or connection evidence. No source, dependency, environment, test credential or live MCP connection is created by this proposal.

## Remote API amendment — presented behavior approved in D63

The [D63 remote amendment](remote-api-amendment.md) adds the shown optional connection:{headers?:[{name,credential_ref}]} to fetch__update_server_config and forwarding it unchanged to the existing config PATCH, with identical server-kind/authority/validation. URL/transport stay read-only, local servers reject the remote field, and no header values are exposed. Update fetch__restart_server's description for Connect/Reconnect/force semantics; existing tool names/other inputs and D49 result mapping stay unchanged. D63 accepts the presented input/description behavior; exact schema-generation/adapter mechanisms remain open.

D64 requires valid remaining tools to stay available after known protocol rejection, with explicit incomplete status/rejection explanation through the existing listing API/tool. Exact per-origin/rejection schema and native catalog metadata delivery remain open; D49 JSON text-block output supplies the explanation when the Fetch listing tool is called, without claiming universal proactive model awareness.

## Catalog-view input — approved in D65

D65 approves [catalog status and protocol views](tool-catalog-status.md): optional view=raw|2025-11-25|2026-07-28 on fetch__list_tools, forwarded to existing GET /api/v1/tools. Omitted management-tool view follows the actual MCP caller protocol; direct HTTP omission is raw, and explicit raw remains selectable. This contextual default/description is an accepted D49 mapping amendment. Return the same API JSON/next_steps in its existing text block; no automatic warning lookup or imported Tool rewrite.
