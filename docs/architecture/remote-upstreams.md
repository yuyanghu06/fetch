# HTTP and SSE upstream transport adapters

October27, 2026. **D61 selects HTTP/SSE upstream support as separate transport functions routed by the shared runner.** This supersedes D35 local-only exclusion at the scope level. The recorded Claude inventory contains four nonloopback HTTP/SSE definitions. Exact discovery/import/config/state/lifecycle amendments, upstream revisions and compatibility policies remain open; the prior proposal below is not blanket-approved. No application implementation is authorized.

## Selected adapter structure — D61

The shared runner reads the managed server connection type and uses either local-process/stdio handling or an HTTP/SSE transport adapter. Native MCP methods reach the same backend operations and authority; arguments/results remain raw. The connection type is saved routing information, not a new model-supplied tool argument. Distinct Streamable HTTP and legacy HTTP+SSE mechanics stay inside the relevant adapters. No new client-facing route or second gateway is required merely to select a transport.

The user explicitly reaffirmed that the shared runner includes handling for all these types. It owns the call path for stdio, Streamable HTTP and legacy HTTP+SSE, including dispatch and applicable message/result/error/cancellation handling through their adapter functions. Shared caller permissions, scheduling and deadlines apply around every path. This requirement is not met by recording a transport label while only implementing stdio. Only selected protocol features are relayed; exact revision/lifecycle policies remain open below.

## Per-tool routing context — D62

Each tool has Fetch-owned routing configuration identifying its managed server/original tool and connection type. At invocation the runner receives this information as internal execution context, alongside the raw argument object, and selects the transport function. All tools belonging to the same server connection use consistent transport information. The transport adapter handles native protocol framing; each upstream invocation remains the appropriate MCP method with unchanged arguments.

This is not an extra field injected into model arguments, a rewrite of the upstream Tool JSON, a credential-bearing tool description or a caller-supplied routing override. Exact config keys/enums, storage duplication versus references and consistency/update mechanisms are still unselected; do not silently substitute a particular physical layout for the user's per-tool configuration requirement. Shared authority, queue/deadline and raw result handling remain above the adapter.

## Prior scope proposal and remaining amendments

| Area | Recommendation |
| --- | --- |
| Connections | Support selected Streamable HTTP and deprecated HTTP+SSE upstream definitions alongside local child servers. These are upstream client adapters, not additional Fetch listeners or a second client-facing gateway. |
| Client access | Keep Fetch's own API and /mcp listener localhost-only; existing client-facing transport/version targets remain D47. Upstream protocol versions are a separate compatibility choice. |
| Setup | Discover supported literal URL-based definitions from the selected configuration sources. For a selected URL entry, prepare a durable connection definition instead of downloading a package. Source capture/reuse/selection and permission rules stay shared; preparation alone does not connect, authenticate or prove readiness. |
| Data and authority | Use the same backend permissions, stable tool identity, queues and unchanged model arguments/terminal results. Remote transport is not an exemption from caller permissions or a new generic tool-call wrapper. |
| Authentication | Obtain fresh upstream authorization through a supported integration; do not migrate source tokens/header values or forward the caller's Fetch bearer token. Preserve encrypted per-server credential storage, Settings-only manual values, refresh ownership and the stored-credential service-test success gate. Exact provider integration is still required. |
| Connection lifecycle | Fetch can open/close/reconnect its client connection. It does not launch, kill or restart the provider's hosted process. Do not apply local child-exit proof or D20 process restart stages to remote work. |
| Failure | Reconnection never automatically replays a possibly executed tool call. Loss of a connection or cancellation does not prove remote work/effects stopped. Preserve truthful unknown outcomes and define capacity/recovery explicitly. |

No WebSocket/custom remote executor support, arbitrary shell/header helpers, source credential migration, provider-process controls, personal client edits or implementation is added by this proposal.

## One subsequent complete API amendment — still open

D61 selects transport coverage but does not finalize the affected existing API schemas. Review the affected discovery metadata/eligibility, installation preparation/progress representation, ordinary server/config fields, auth applicability and reconnect-versus-restart behavior as one coherent amendment. Preserve the accepted method/path inventory where suitable; new routes, fields, status values, error codes or MCP management tools are not selected by this scope document. Existing install inputs remain source path/name references until a reviewed amendment changes them. Direct URL submission/edit rules are not assumed.

The amendment must state what connection settings can be edited, caller authority, safe endpoint/header-reference representation, saved versus applied changes, lifetime/overlap/retries, timeout/uncertainty accounting, errors and matching deterministic UI/agent guidance. No process ID, remote restart success or authentication-success inference may be fabricated for URL entries.

## Research constraints before compatibility can be claimed

- [Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http) and [deprecated HTTP+SSE](https://modelcontextprotocol.io/specification/2024-11-05/basic/transports) are different upstream adapters. The old SSE endpoint announces a POST destination; current Fetch client-facing /mcp does not become that old transport. Exact revisions/fallback conditions remain explicit review, and authentication/recognized modern errors are not reasons for a silent downgrade.
- Legacy2025 stream loss should not imply cancellation; modern2026 response-stream closure signals cancellation. Neither connection closure nor Quit proves remote execution stopped. See [legacy transport](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports) and [current cancellation](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/cancellation). Retiring a remote connection generation is not a substitute for child-exit evidence; capacity recovery needs its own policy.
- [Modern HTTP tool-header annotations](https://modelcontextprotocol.io/specification/2026-07-28/server/tools#custom-headers-from-tool-parameters) require parameter-to-header handling and exclusion of invalid annotated definitions. This raises an explicit D37 compatibility conflict, not permission to silently filter/clean raw tools. Failing the entire catalog instead is not expressly established as an allowed alternative by the checked clause. Select version/annotation handling before claiming modern upstream compliance; a legacy revision is an option only if that upstream actually supports it.
- [Authorization security](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations) preserves separate caller/Fetch/provider credentials. Endpoint URLs, announced POST destinations, redirects and OAuth-discovery destinations require explicit destination/credential rules; never forward authorization blindly across origins. Preserve existing loopback/private gateway entries rather than inventing a blanket public-internet-only rule. Exact URL/TLS/redirect rules remain prerequisites, not selected policies.

## Implementation evidence and limits

Read-only research reused the prior active Apache-2.0 gateway maintenance/license/dependency/issue comparisons. The [official Rust SDK](https://github.com/modelcontextprotocol/rust-sdk) supplies Streamable HTTP, and [agentgateway's upstream SSE adapter](https://github.com/agentgateway/agentgateway/blob/3ef10407860d6f5c0b10dcd13779fc274a7f58f1/crates/agentgateway/src/mcp/upstream/sse.rs) illustrates separate legacy transport support. No library/version is selected; SDK license-transition/version checks remain needed. Existing session-isolation/cleanup findings remain relevant in research.md.

D61 accepts the separate-adapter organization and HTTP/SSE support, not every clause in the earlier proposal. This is source-backed design feasibility, not proof that the user's four services or client releases work. No provider capability discovery, personal service/tool call, login, package install, client/server startup, environment provisioning or personal configuration change occurred. D46 isolated testing and the application implementation pause remain in force.

D63 approves the presented [remote API amendment](remote-api-amendment.md), including fixed endpoint identity, editable header references, safe remote state and existing-route reconnect with an explicit force/unknown-work tradeoff. Separate protocol/security/implementation prerequisites remain open.

D63 subsequently accepts the presented [remote API amendment](remote-api-amendment.md), including fixed endpoint identity, header-reference configuration and explicit forced-reconnect/unknown-work behavior. Remaining separately listed compatibility/auth/security/implementation prerequisites stay open.

A concrete [protocol-adapter proposal](mcp-protocol-adapters.md) now addresses the D37 header-annotation conflict and a newly confirmed D43 legacy-to-modern result marker conflict. SDKs can filter before returning data; preserve/observe raw data first and request explicit publication/presentation exceptions. None is approved by D63 alone.

D64 resolves publication direction for known protocol-rejected tools: retain valid tools with explicit incomplete status/rejection details, rather than whole-source failure. Exact HTTP-origin/native metadata and visibility remain open; no generic raw Tool sanitization is authorized.
