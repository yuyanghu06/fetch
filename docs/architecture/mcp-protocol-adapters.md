# MCP protocol adaptations — selected behavior and remaining proposals

October27, 2026. **D64 approves the native legacy-result marker exception and returning valid tools with explicitly incomplete catalog status after known protocol rejection.** It rejects the prior whole-origin/native-catalog failure proposal. D65 subsequently approves the presented partial-list view/metadata mapping, initial upstream targets/setup policy and modern catalog cache defaults; actual implementation/host visibility remains unverified. No permission to sanitize unknown fields or implement the application follows; source research is complete but no SDK or runtime compatibility is selected.

## Version and adapter policy — presented behavior approved in D65

Target upstream revisions2025-11-25 and2026-07-28, independently of D47's client-facing targets. A transport label is not a protocol version, and legacy HTTP+SSE transport support does not prove support for every earlier revision. Only use a revision supported by both Fetch's implemented adapter and that server. Additional older/newer upstream targets need explicit compatibility evidence; do not claim all inventory entries work from transport metadata alone.

Learn compatibility through the appropriate read-only modern discovery or legacy initialize/version/capability setup, never by invoking a tool. Accept only a supported negotiated legacy response; modern requests carry the required per-request version/metadata. A recognized compatibility/version signal during setup may select another supported revision; arbitrary authentication/TLS/network/HTTP errors are not downgrade signals. Never retry/replay a tool invocation to negotiate a version or treat transport reconnection as a new authorization grant. Exact SDK negotiation/framing mechanisms must follow and be tested against the selected revisions.

The shared runner uses internal source/transport/version/caller context. Canonical raw upstream Tool definitions, model argument objects and received terminal ToolResult values remain available before SDK normalization. Fetch-owned native MCP envelopes/transport headers are constructed for the client-facing revision. Protocol adapters do not create a new wrapper or model argument format.

## Required HTTP header mappings without changing arguments

Modern HTTP tools may contain x-mcp-header annotations assigning an argument property to a Mcp-Param-* HTTP header. Preserve the original definition/argument object; mirror applicable supplied values into native transport headers as the negotiated revision requires. This is not insertion of Fetch configuration, server selectors, stored provider credentials or deadline values into arguments. Stored credentials remain separately assigned/injected by the trusted auth/transport integration.

Do not treat arbitrary unknown metadata as instructions or implement a field whitelist. The only new Tool validation proposed here is the modern HTTP protocol's specific header-annotation requirement. Valid mapping extraction/encoding/reserved framing must follow that standard. Appropriate known mapping evidence must exist before dispatch; no hidden list/auth/start/retry is added to D50's call route just to recover missing metadata. Exact metadata/profile integration remains implementation work.

## Mandatory client rejection and explicitly incomplete publication — D64

The standard requires modern HTTP clients to reject/exclude definitions with invalid x-mcp-header declarations. Official SDKs may do this before returning tools/list. Preserve/observe the pre-filter raw response and detect rejection explicitly; SDK logs alone or a filtered empty array do not prove a complete raw catalog. A library path that silently filters/normalizes without observable raw capture cannot satisfy Fetch's contract.

D64 selects this behavior after a complete upstream collection:

- Return the valid tools unchanged and keep them usable. In the example,19 valid definitions are returned rather than failing the whole20-tool server.
- Mark Fetch catalog completeness false and provide the rejected tool's identity and safe reason separately from raw Tool objects. Other valid origins remain usable. Unknown fields alone are never a rejection reason; no generic projection/schema/content cleaner is added.
- Apply the corresponding selected behavior at the modern client-facing export boundary, including tools originating from stdio/legacy servers whose annotations an HTTP host must reject. Do not fail the combined native catalog merely because one definition is rejected.
- Preserve duplicate-name and incomplete/failed traversal handling under D37/D38. This exception does not allow pages collected before a timeout to masquerade as a complete catalog, guess unseen counts or use cached stale fallback. GET still does not start/login/invoke/repair a server.
- A known rejected definition is not published as callable for the affected mode; valid remaining definitions retain ordinary authority/routing/queue/deadline rules. No auto-retry of invocation, definition rewrite or fake authentication promotion follows.

The prior proposal to mark the whole affected origin unavailable or fail the combined modern catalog solely for one rejected definition is **superseded by D64**. The standard mandates invalid-tool exclusion; explicit partial availability/completeness disclosure is Fetch's selected product behavior.

### Partial-list delivery/schema — D65

D65 selects the concrete [catalog-status/view contract](tool-catalog-status.md): HTTP available/partial/unavailable with rejected_tools, root view/complete, optional raw/2025/2026 views, native _meta["fetch/catalog"] summaries and the matching Fetch listing-tool default. Source mandatory rejections carry across views; modern export-only rejection does not globally poison raw/legacy views.

Metadata visibility remains client-dependent. The listing tool emits complete API JSON/next_steps in text when called; do not claim automatic model awareness. Native resultType:"complete" remains protocol completion, distinct from Fetch catalog complete:false. No fake cursor, unsupported next_steps root or raw Tool/result rewrite carries the warning.

## Required result marker: approved narrow D43 presentation exception — D64

Let R be a received terminal upstream ToolResult. Canonical application HTTP result remains R unchanged. For native MCP presentation only:

- If the originating legacy revision and response semantics establish a terminal result and R lacks resultType, the modern2026 native copy may add only resultType:"complete", which its protocol requires.
- Preserve every existing field, value, content block/order, structuredContent, isError and unknown/nested value. No Fetch notice/guidance, duplicate result, media classification/copy, content concatenation or derived outputSchema is added.
- Preserve an existing compatible marker. Do not overwrite an invalid/conflicting marker or turn input_required/task/unknown data into completion. The adapter must know the source revision/terminal semantics; absence in a malformed modern response is not a legacy adaptation license.
- If presentation is incompatible, use safe native protocol/adapter failure instead of rewriting the received value or automatically repeating the operation. Retain truthful known-completed versus unknown execution internally; a presentation failure does not prove the provider action failed or was undone. This creates no new result lookup/job retention API.

D64 approves this explicit D43 exception for required native protocol presentation, not an assertion that the outgoing native ToolResult is byte/value-identical to legacy R when the missing marker is added. Canonical HTTP raw R remains unchanged. Other cross-version interactions/tasks/resources/prompts still require their selected capability/adapter contracts; this marker rule does not implement them.

## Modern native catalog metadata default — approved in D65

Use ttlMs:0 and cacheScope:"private" in the applicable modern native discovery/catalog envelopes. These declare no catalog reuse and no sharing across callers, matching fresh permission-filtered reads. Do not insert them into raw Tool objects, terminal R, legacy-only envelopes or canonical HTTP data, and do not enable a shared cache/result cache/stale fallback. Upstream SDK catalog caching must not replace D38's live complete read. D65 selects these exact presented defaults; actual client/SDK cache handling must still be verified.

## Primary research evidence and limits

- [Modern header-annotation specification](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/basic/transports/streamable-http.mdx#L402) requires invalid-definition exclusion. It does not separately prescribe Fetch's partial-availability disclosure semantics.
- [Rust rmcp-v3.8.0 implementation](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.8.0/crates/rmcp/src/transport/streamable_http_client.rs#L94) filters with retain before passing the response upward. [Inspected unit tests](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.8.0/crates/rmcp/src/transport/streamable_http_client.rs#L2572) cover invalid-tool removal, valid retention and legacy preservation. The release file matched pinned main cd88056. Rust requires capture below that filtering transport; no public preserve-raw-plus-rejections option was established.
- [TypeScript v2.4.0 listing implementation](https://github.com/modelcontextprotocol/typescript-sdk/blob/v2.4.0/packages/client/src/client/client.ts#L2556) filters individual/aggregated pages before caching/return. A lower-level request boundary is a possible capture point, not a verified lossless catalog API. Release content matched pinned39ae8e6.
- [Current Result definition](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts#L223) requires resultType; legacy absence handling by a receiving client does not remove Fetch's modern-server obligation.
- [Legacy version negotiation](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle#version-negotiation), [modern versioning source](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/basic/versioning.mdx) and [modern backwards compatibility](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http#backward-compatibility) establish different setup/framing, not automatic tool-call retry authority.

Both official SDKs are maintained and document Apache-2.0 transition/retained MIT contributions; Rust uses Tokio/Serde/Reqwest. Prior maintained licensed gateway comparisons remain relevant for adapter design. No SDK/version/license/dependency is selected or installed, no code copied, and cited unit tests were inspected rather than run. Unknown-field/numeric fidelity, raw interception/rejection evidence and host interoperability need actual D46 isolated verification after implementation authorization.

## Future acceptance evidence

Verify both supported revision paths/unsupported rejection; supported setup negotiation without auth downgrade or call replay; pre-filter observability; raw unknown/nested/numeric preservation; complete:false with valid remaining tools after explicit rejection; no unmarked shortened/false-empty catalog; existing duplicate/collection failures unchanged; modern partial export for legacy-origin invalid annotations; valid header mirroring outside unchanged arguments; legacy marker-only addition with canonical R unchanged; invalid/current-missing-marker and nonterminal failures; no extra result/content/schema copies; caller-specific private/zero-TTL metadata and live-read behavior. No runtime/host/provider test, environment provisioning, personal configuration or credential change has occurred.

The exact [catalog status/view proposal](tool-catalog-status.md) now presents the remaining partial-list schema and native delivery choice as one reviewable amendment. View-specific rejection preserves legacy/raw usability while native/Fetch listing defaults match the caller. Upstream targets/setup and modern cache defaults remain proposed with that bundle.

The next [native interaction relay proposal](mcp-interaction-relay.md) addresses intermediate control frames separately from preserved terminal ToolResults, with strict owner/capability/continuation/deadline binding. It does not expand D64 terminal marker exceptions into arbitrary data rewrites or approve Tasks/resources/prompts.

D66 selects the presented native progress/interaction control adapters and owner-bound continuation behavior. This does not expand the terminal raw-result exception beyond D64 or approve extra protocol capabilities; exact SDK/handle/resource/recovery mechanics remain open.
