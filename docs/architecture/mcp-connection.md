# Localhost MCP connection contract

October 27, 2026. **Connection policy approved in D47 at the presented scope.** After the complete contract and protocol/API/tool-routing clarifications, the user said "okok cool you're good." This contract covers the single client-facing MCP resource and transport/session behavior; exact OAuth issuer/TLS/scopes, SDK/framework, configured port, supported client releases and ordinary execution mechanisms remain later review. No application implementation or live tests are authorized.

## Approved endpoint and transport — D47/D70

```text
https://127.0.0.1:<Fetch-port>/mcp
```

Use one `/mcp` endpoint in the same Fetch process and configured loopback API listener. The numeric port is not selected here; clients use the exact address published by Fetch. Bind-conflict/startup handling (including any no-silent-address-switch refinement) remains an unshown step5 policy to review before implementation. D70 selects HTTPS for this shared local listener. The displayed numeric loopback host illustrates the address; the exact published hostname/port, matching certificate names and remaining key/client trust mechanics are still unselected.

Use the Streamable HTTP family only. Do not add a stdio bridge, a second gateway process, separate deprecated HTTP+SSE endpoints, a LAN/Tailscale/public listener or a compatibility URL alias. Streamable HTTP may itself use SSE response bodies; this is not the deprecated separate-endpoint transport.

## Approved protocol compatibility target — D47

Explicitly target both researched revisions, `2025-11-25` and `2026-07-28`, rather than assuming every client uses the newest revision. D47 approves these compatibility targets, not a claim that a chosen SDK/current client release has already passed. Exact client releases, native adapters and SDK implementation must be verified in D46 isolated tests before implementation acceptance.

| Operation | Legacy 2025-11-25 mode | Modern 2026-07-28 mode |
| --- | --- | --- |
| Setup/discovery | Standard initialize handshake and initialized notification. | Standard server/discover and per-request protocol/capability declarations; no initialization session. |
| POST /mcp | Native JSON-RPC messages, JSON or SSE responses as required by that revision; also accepts the appropriate native client responses/notifications. | Native single JSON-RPC requests/notifications with required version/method/name metadata consistency; JSON or SSE responses as required, including a subscriptions/listen stream for the selected catalog-change channel. |
| GET /mcp | Authenticated SSE channel for supported protocol/catalog notifications, using that legacy session. | 405 Method Not Allowed; no standalone GET stream. |
| DELETE /mcp | End the authenticated legacy MCP session, proposed empty204 on success. Does not stop/restart managed servers. | 405 Method Not Allowed; no MCP session deletion. |
| Notification acknowledgement | Empty202 when the revision requires acknowledgement. | Empty202 for accepted notifications. |

Notification202 is a protocol acknowledgement, not an asynchronous Fetch tool job. Do not add application next_steps or status wrappers to native MCP envelopes. Required content types, Accept/version/method headers, invalid-message/unsupported-version/header-mismatch/unknown-method codes and framing follow the selected revision; exact adapter code mappings are review work, not invented reserved codes.

The connection supports the standard discovery/tool-catalog/tool-call surface and permitted catalog-change delivery for approved tool visibility. This does not select actual builtin tool names or additional progress/tasks/elicitation/sampling/roots/resource/prompt capabilities. Only advertise capabilities actually selected and implemented. Catalog notifications reflect known backend events; this proposal adds no background upstream scan/poll or credential test. Exact catalog-change mechanics and compatibility with real clients must be tested; transport support in documentation is not a protocol-version guarantee.

## Approved authentication and local request boundary — D47

Require valid Fetch bearer authorization on every protected `/mcp` HTTP request, including protocol initialization, streams and legacy DELETE. Validate expiry/intended audience and current caller permissions; loopback access is not anonymous authority. Caller-to-Fetch auth remains distinct from each server's private provider credentials. Authentication never returns those provider values or substitutes for the required D25/D31 service credential test.

Missing/invalid/expired credentials use native401 and the appropriate bearer/discovery challenge; insufficient authority uses403. Exact scopes/registration/renewal/endpoints are step4. Recheck permissions at discovery/listing and invocation rather than granting power from a session ID or tool prefix.

Bind only loopback. Validate the actual listener Host/authority against the explicitly configured local authority; do not trust arbitrary proxy headers or accept wildcard hosts. If Origin is supplied, allow only explicitly trusted configured Fetch origins and reject others with403. Authenticated native clients may omit Origin; omission does not waive bearer validation. The concrete initial allowlist is the advertised Fetch origin, with no wildcard browser access; additional trusted origins need explicit configuration/review.

### Actual OAuth HTTPS dependency

Current MCP OAuth security requires authorization-server endpoints to use HTTPS; the documented localhost exception concerns redirect URIs. The candidate HTTP loopback MCP resource does **not** select a same-port plaintext OAuth issuer. Step4 must resolve the issuer/authorization/token endpoint/TLS/trust design before working OAuth interoperability or implementation can be accepted. D47 alone selected no external identity service, certificate setup or extra listener; D70 subsequently approves the presented unique local certificate/trust direction for the same listener, with no actual installation or extra listener authorized. See [the source research](research.md#localhost-mcp-connection-and-client-compatibility--october-27).

## Approved session/disconnect policy — D47

Legacy MCP sessions are ephemeral within the current Fetch process and bound to their original authenticated client/grant context. A session ID is not authentication. Renewed valid credentials from the same authorized context may continue according to the selected auth contract; tokens/permissions are still validated per request. Exact identity binding and session storage/expiry mechanisms remain step4/5, not a saved worker/session-sharing policy.

Modern mode has no initialization/session-ID state. Its per-request metadata and any permitted stream/subscription state follow the selected revision. Neither mode promises durable transport sessions or event replay/result recovery across Quit/crash/relaunch. Unknown/expired legacy sessions use native handling and require fresh setup rather than attaching to another caller's session.

Closing a stream/session or reconnecting affects that client transport. It does not start/login/restart managed servers, change alias identities, replay tools/call, cancel unrelated clients or prove external effects stopped/rolled back. Revision-specific cancellation signals still apply to the affected request; upstream cancellation, finite call timers, shared application state/workers and D20 active-work/drain accounting remain step3 prerequisites. Deleting a legacy session does not grant an implicit provider-stop operation.

Backgrounding/closing Fetch's window retains the selected running process behavior. Explicit Quit still stops managed servers and closes MCP connections. Reopening/reconnecting cannot silently replay an earlier possibly executed call or claim historical completion; no call-status route/job persistence is introduced.

## Tools and result ownership

Tool operations use the same backend APIs and authority. Resolve D45 aliases to exact server/original names, pass unchanged model argument objects, and preserve raw upstream terminal ToolResults under D43. Fetch-owned operational failures render existing next_steps explanations as text under D44, with known not_started versus unknown outcomes; no response_context field or added notice on upstream results.

Native authentication, malformed/unsupported-version/unknown-method/alias/session and upstream protocol failures stay native safe protocol handling. Do not convert generic exceptions into a different error class accidentally or expose raw credential-bearing diagnostics. Exact native numeric codes are governed by the supported revision/adapter review; ordinary call HTTP error fields remain their separately reviewed application contract.

## Acceptance evidence required before implementation acceptance

All actual connection tests must run in an approved isolated VM/virtual computer/suitable environment under D46, with disposable fixtures/dedicated credentials and no access to personal files, configs, apps, OS trust or existing services.

- Exercise real supported Codex/Claude Code/OpenCode releases and both selected protocol modes: initialization/discovery, headers, JSON/SSE, notifications, GET/DELETE and native errors. No compatibility is established by docs alone.
- Validate per-request bearer/audience/permission checks, native401/403/discovery, Host/Origin rejection and cross-client legacy-session isolation, using the separately approved HTTPS issuer/TLS setup.
- Verify catalog visibility/aliases, backend invocation, raw results and Fetch-only failure guidance without new model argument fields or protocol wrappers.
- Verify disconnect/reconnect/session loss/DELETE/Quit and revision cancellation semantics without call replay, process restart, false rollback or premature drain accounting.
- Confirm unsupported capabilities/versions/errors and actual loopback-only binding. No numeric port, execution/session timeout or SDK is supplied by this document.

D47 selects the presented dual-revision target rather than either single-era alternative; actual SDK/client compatibility is still unverified. No framework, SDK dependency, environment provision, OAuth/TLS setup, source code or live MCP/client test has been created or run. D47 does not select unshown exact response codes (such as the illustrated legacy DELETE204), numeric port/SDK/client releases or ordinary execution mechanisms, and does not authorize application implementation.

## Shared handlers and runner — D59

The user accepts individual shared functions for each selected functionality, invoked by the runner. POST /mcp dispatches by native method: tools/list and tools/call reach tool handlers; selected resources/list, resources/read, prompts/list/get and tasks methods would reach their own handlers at that same URL. These method examples explain the architecture; they do not approve their release scope or detailed contracts.

Dispatch uses protocol message types, methods, defined result discriminators and the owning request context, never executable instructions inferred from returned text or unknown metadata.

The runner also handles messages while operations run: progress goes to the owning caller; requests for input/model/workspace context go to that capable caller; its answer goes back to the originating server. Legacy callbacks and modern input_required continuations need their revision adapters. The final upstream ToolResult stays raw. No automatic model execution, UI consent, credential entry, replay or capability impersonation follows from dispatching a handler.

This is one endpoint path, not a promise that every exchange completes in one POST or one JSON response. D47 still governs JSON/SSE and revision-specific GET/DELETE/listen channels. Preserve exact caller/server/request association, permissions, protocol metadata and cancellation/deadline handling; raw model arguments contain no Fetch routing or continuation controls. Separate client-facing HTTP/UI interaction contracts remain open where needed. Shared handlers keep product operations API-first and reuse existing backend authority rather than adding a second behavior path.

Primary checks: [legacy Streamable HTTP](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports) carries requests/responses/notifications through POST at one path and allows streaming; [modern MRTR](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr) requests additional input through native continuation results and subsequent requests. D59 selects the internal organization, not all capability/remote transport contracts or implementation. No runtime code or connection test was performed.

## Approved automatic streaming defaults — D60

October27 follow-up to D59: after asking whether streaming can be the default, the user said "You’ve got it" to the presented automatic policy (D60). Fetch chooses the native response type automatically per request, with no user-facing streaming toggle or extra model argument. Tool calls stream by default, relaying available progress then the unchanged final result. Quick discovery/list reads use a single JSON response. Additional selected operations use their appropriate native interaction flow; their exact capability contracts remain open. Protocol notifications/client responses retain their revision-required empty acknowledgements; authentication and pre-stream validation errors retain native HTTP handling. Do not stream every HTTP response indiscriminately or change the existing /api/v1 JSON contracts.

SSE here carries complete native MCP messages: available request-related progress/notifications, then the operation's final JSON-RPC response. Preserve the terminal upstream ToolResult, even when it is the only event. This does not generate partial text/image/audio/result chunks, invent progress for quiet tools or guarantee client UI display. Once response headers have been sent, subsequent failures use the applicable native stream/protocol handling rather than attempting to change HTTP status. Only relay actually selected/supported events to the owning caller.

Retain D47's version rules: legacy streams may carry related server requests; modern streams use input_required/MRTR for input and separate subscriptions/listen for change notifications. Do not keep an operation stream open after its final response merely because streaming is the default. Modern response-stream closure cancels that request; legacy SSE disconnection alone should not imply cancellation. Neither proves upstream side effects stopped. Progress/keep-alives do not silently reset the selected Fetch execution deadline, and transport choice does not implement pending capability contracts.

Both [legacy transport](https://modelcontextprotocol.io/specification/2025-11-25/basic/transports) and [current Streamable HTTP](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http) require clients to accept JSON and SSE and let the server choose for each request. They permit this policy; they do not prescribe Fetch's chosen default. D60 approves the defaults shown to the user, not additional capability contracts or implementation. Exact unshown SDK/wire mechanisms remain review work. Compatibility still requires D46 isolated evidence.

The [protocol adapter guide](mcp-protocol-adapters.md) now records D64 marker approval and explicitly incomplete valid-tool availability, replacing whole-catalog failure for one rejected definition. Exact partial-list native metadata/client visibility, upstream targets/setup and modern private/zero-TTL defaults remain open; dual-version implementation/host compatibility is not proved.

The [catalog-status proposal](tool-catalog-status.md) now specifies native _meta["fetch/catalog"] summaries and actual-client view selection, without Tool/result duplication or unsupported top-level next_steps. Metadata visibility remains client-dependent and untested. Exact fields/upstream/cache defaults remain proposed.

D65 now approves native _meta["fetch/catalog"] summaries and actual-client view selection, with initial upstream2025/2026 targets and applicable modern0/private catalog hints. Tool definitions/results are not duplicated; no custom next_steps root is added. SDK/raw-fidelity/host metadata visibility still require isolated tests.

The [native relay proposal](mcp-interaction-relay.md) now reviews progress and roots/sampling/elicitation/MRTR on the existing /mcp path. D47 transport/revision rules remain; shared legacy context cannot announce arbitrary per-call capabilities and receive-side SDK correlation must be proved. No new listener/REST interaction/UI/model or implementation is authorized.

D66 approves the presented native relay behavior: endpoint dispatch into the shared runner, selected transport function and owner-bound progress/workspace/model/form handlers. Native control replies/continuations return through /mcp; existing backend services and D47 protocol framing remain. Actual SDK receive correlation/client support stays unverified.

D67 approves the presented [native resources/prompts surface](mcp-resources-prompts.md) on existing /mcp handlers/runner, qualified selectors/preserved payloads, shown budgets/framing/catalog events and explicit live-content/Tasks deferrals. Exact unshown diagnostics, codec/client evidence remain open; no new listener/REST/UI/model is selected.

October28 unresolved trust clarification: the displayed HTTP loopback resource is a candidate, not proof that OAuth and protected-resource metadata meet HTTPS/certificate requirements. The [research finding](research.md#authentication-deduplication-and-remaining-trust-gap--october28) records the specific missing listener/issuer/browser/client trust decision. Standard OAuth and existing localhost-only policy remain selected; D70 subsequently selects the presented HTTPS direction; no actual trust-store change is authorized by that design approval.

D70 approves the presented [local OAuth HTTPS/certificate direction](local-oauth-trust.md): one HTTPS loopback listener and a unique per-install certificate with explicit trust/normal verification. Actual key/client mechanics and isolated browser/client proof remain open; the next proposed contract is app authorization/token behavior.

D69 confirms that the browser connects an app to local Fetch to obtain that app's access token, without a Fetch account, signup or password. The exact approval/authority controls remain for review. Do not read browser login shorthand as an approved account system or the pending HTTPS/trust proposal as adopted.

D70 now selects the presented local HTTPS/shared listener, per-install certificate/key and explicit trust/normal-verification direction. Prior paragraphs describing the entire scheme as unselected are historical at D47/D69 scope. Next is the [complete account-free app authorization/token proposal](client-authorization.md), whose new routes/scopes/Mac approval/defaults remain proposed; no testing or implementation is authorized.

D71 approves the presented account-free app-connection routes, Mac confirmation, scopes and token/lifecycle/admission policies. Older paragraphs labeling the entire client authorization contract proposed are superseded at that presented scope. Exact unshown mechanisms/client evidence remain open; next is Settings authority/key protection.
