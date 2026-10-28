# Connecting an app to local Fetch

D73 clarification: each agent keeps its own Fetch credential/token for one `/mcp` connection covering permitted installed tools. Provider logins are separate user setup and reused; ordinary calls neither open login nor repeat provider credential verification. Preserve D71 renewal/permissions; no static-token alternative is introduced.

October28, 2026. **Presented app-connection contract approved in D71.** The user replied “Yes” to the complete presentation, including the one-time Mac confirmation and separate management permission. D71 records the shown routes, flow, scopes, lifetimes, renewal/revocation/restart behavior and admission limits. D10/D47/D69/D70 remain intact. The detailed review draft below includes unshown standard-wire/OS/storage implementation clauses; D71 does not silently approve those. No implementation or live test is authorized.

## Approval scope and remaining mechanisms

Use [D71](decisions.md#d71--approve-the-presented-account-free-app-connection-contract) for the precise accepted scope. The next review is human Settings authority and key protection/unlock/recovery. Keep exact Mac-prompt timer/foreground behavior, unshown wire/registration refinements, key/storage/crypto, connection-administration UI/API and isolated client/rotation/certificate proof explicit without reopening selected route/permission/default behavior. Proposal wording below is retained where it describes an unshown detail or the original draft, not an instruction to re-review the whole contract.

## What the person experiences

The app opens Fetch's local HTTPS page. The page shows the requesting app's label, its callback address and requested access. You choose Allow or Deny. On Allow, Fetch asks macOS to confirm the local person using an available Mac unlock method, such as Touch ID or the existing Mac password. Fetch never collects that password in its web page. This approval method is now selected by D71; it creates no Fetch account and was not a requirement already selected by D69/D70.

After approval the browser returns a one-use code to the app. The app exchanges it privately for its access/renewal tokens. Tokens do not appear in the browser address bar. Ordinary later calls and renewal do not open the browser or ask for Mac authentication again. An expired/revoked connection needs a new approval.

The app label is supplied by the requesting client, not proof that its executable is genuine. Localhost, a familiar label, registration, an existing agent token or a browser page load never approves a new connection automatically.

## Address, registration and callback

Let `F` mean Fetch's stable published local HTTPS origin, including its chosen port. Exact numeric port/hostname remains the existing listener/onboarding decision. All endpoints below run in the same Fetch process/listener. The canonical protected-resource identifier is `F/mcp`.

Initial client registration uses standard dynamic registration (DCR). This is supported in the inspected Codex, Claude Code and OpenCode sources. The initial proposal does not advertise Client ID Metadata Documents (CIMD), which would require fetching client-owned public websites. This explicitly defers a modern MCP recommendation for the initial local slice; it is not a claim that DCR is the newest preferred mechanism. No fixed branded client IDs, client secrets, remote metadata/logo fetch or new client-registration management API are implied. Actual clients must prove this scope sufficient under D58; a required CIMD-only client is a gap to review.

The callback belongs to the coding app. Register HTTP loopback callback URIs using the exact literal `localhost`, loopback IPv4 address or loopback IPv6 address, with no userinfo or fragment. No wildcard hosts, remote callbacks or custom URI schemes in the initial proposal. Native loopback ports may vary under the standard native-client exception; the scheme, literal host, path and query must otherwise match registration. Do not treat `localhost` and a numeric address as interchangeable. Code exchange must repeat the exact redirect URI used by that authorization attempt.

No Fetch-owned `/oauth/callback` is added for this flow. Provider-to-Fetch callbacks remain separate provider-auth work.

## Proposed routes, inputs, results and authority

OAuth endpoints use their standard metadata/registration/form/redirect/error formats. They do not gain application JSON wrappers or `next_steps`. Existing application API JSON and management-tool text retain D22/D49.

| Method and path | Input / reads or changes | Result / authority |
| --- | --- | --- |
| `GET /.well-known/oauth-protected-resource/mcp` | No body; reads public Fetch resource metadata. | `200` standard metadata: exact `F/mcp`, issuer `F`, supported scopes, header bearer method. No server inventory/provider credentials. Advertise this URL in the native `/mcp`401 challenge. |
| `GET /.well-known/oauth-authorization-server` | No body; reads public issuer metadata. | `200` issuer/endpoints, code+refresh capabilities, public-client `none` auth, PKCE `S256`, scopes and issuer-response support. OAuth metadata is sufficient; no extra OIDC discovery/identity-token route. |
| `POST /oauth/register` | Standard JSON native-client metadata, including redirect URIs and supported token auth/grants. Records a public client, not an access grant. | `201` standard effective metadata and generated `client_id`; no `client_secret`. Reject confidential-client methods. Public clients must explicitly select `token_endpoint_auth_method: none`; do not reinterpret RFC7591's omitted `client_secret_basic` default. Omitted `grant_types` retains the standard code-only default; clients needing refresh must register it. Ignore unsupported extension metadata as RFC7591 permits rather than inventing strict unknown-field rejection. |
| `GET /oauth/authorize` | `response_type=code`, registered `client_id`/`redirect_uri`, `code_challenge`, `code_challenge_method=S256`, exact `resource=F/mcp`, optional `scope` and `state`. Reads registration and creates a temporary browser-bound request. | HTML request/approval page after validation. It creates no grant/token on GET. Establish a Secure HttpOnly SameSite=Lax browser cookie and request-specific CSRF proof. No bearer token is required to request approval. |
| `POST /oauth/authorize` | Form `request_id`, `decision=allow|deny`, CSRF proof and selected scope subset. Requires matching secure browser state and same-origin proof; no OAuth credential can substitute. | Deny returns OAuth `access_denied`. Allow starts one fresh request-bound macOS authentication; only success may produce a one-use code redirect. Redirect contains `code` or a standard error, unchanged `state` when supplied and issuer `iss=F`. Never redirect to an unvalidated URI. |
| `POST /oauth/token` | Form code grant: `grant_type=authorization_code`, code, public `client_id`, exact redirect URI, PKCE verifier and resource. Refresh: `grant_type=refresh_token`, refresh token, client ID, same resource and optional scope subset. | `200` standard bearer access token, `expires_in`, effective scope and a refresh token when registered/authorized. Code/refresh is consumed atomically before successful publication; no resource/scope/client expansion. No browser/OS prompt during refresh. |
| `POST /oauth/revoke` | Standard form token, public client ID and optional type hint; possession plus matching client binding is required. | Empty `200` after durable revocation of that connection's whole access/refresh family. An unknown/already-invalid token also returns `200` without disclosure. It cannot revoke a different connection just by naming its client. Storage failure cannot report success. |

Issuer metadata explicitly includes `authorization_response_iss_parameter_supported: true`; authorization success/error responses identify `F`. It advertises `response_types_supported: [code]`, `grant_types_supported: [authorization_code, refresh_token]`, `token_endpoint_auth_methods_supported: [none]` and `code_challenge_methods_supported: [S256]`. No password, implicit or client-credentials grant is added.

Codes bind the approved grant, issuer, client, exact request redirect/resource, selected scopes, target restrictions and PKCE challenge. A code is usable once. Preserve supplied state exactly. Existing Fetch-owned native errors and raw upstream results remain unchanged.

## Proposed permissions and defaults

| Scope | Access it actually grants |
| --- | --- |
| `fetch:catalog` | Caller-visible installed tool/resource/prompt catalogs and permitted list-change delivery. Listing alone grants no execution, content read or server management. |
| `fetch:tools:call` | Imported tool invocation through `/mcp` or its existing application call API, subject to exact server/original-tool restrictions. |
| `fetch:content:read` | Authorized native resource reads, prompt retrieval and supported completion. Catalog access remains separate. No automatic prompt application/model execution. |
| `fetch:manage` | Existing discovery/install/progress/server GET/authenticate/config/restart and their matching Fetch management tools, preserving target restrictions and all existing prerequisites. It does not grant imported-tool invocation or manual secret writes. |

The default **Use installed MCPs** requests the first three. If `scope` is omitted use that documented default; no separate `offline_access` scope is required to renew an otherwise eligible connection. If scope is explicit, grant only a human-approved subset of what was requested. Reject unknown scopes rather than downgrade them silently. Management is requested separately and unchecked by default; the person must explicitly approve it. If no requested permission is approved, deny the connection.

Make the breadth clear on the approval page: the initial grant covers **all current and future installed servers, including tools added later**, for its selected operations, subject to backend server/tool/item restrictions. This is a proposed visible permission choice; it is not inferred from a token's presence. Never expand an existing grant's scopes on refresh or merely because a client asks for more later; increased access needs fresh human approval.

Keep D23: ordinary server GET remains management-only. Native `tools/list` may expose eligible Fetch-owned management definitions according to their underlying API permission; imported catalogs still require catalog scope. A management-tool call follows its API permission, not imported-call scope. Tool names, session IDs, transport capabilities and aliases never confer authority.

Define Fetch's application APIs and `/mcp` as surfaces of the same protected Fetch service identified by `F/mcp`. The same token may call the corresponding existing `/api/v1` operations only under the same grant/operation/target checks. It is never an upstream credential and is never forwarded as one. Do not accept tokens issued for other origins/resources. Future memory APIs/scopes remain a separate selected-scope review under D48.

No ordinary OAuth grant permits manual credential/environment-value writes, certificate trust changes, client approvals or general human Settings authority. This approval mechanism does not silently settle the separate Settings/key review.

## Human approval without a Fetch account

On Allow, validate browser cookie, origin/CSRF and pending request. Freeze client, redirect, resource, permissions and PKCE binding before starting a fresh macOS `LAContext` using device-owner authentication. Keep reuse disabled/default zero; never reuse a prior successful authentication context for another approval. Fetch receives an OS success/failure, not the Mac password. Use available Mac unlock methods; do not promise every device has Touch ID or Apple Watch support.

Only the successful OS result bound to that same unchanged, unexpired request may issue a code. Browser-supplied success flags, an agent bearer token, a previous Mac unlock or a callback for an ended request cannot substitute. Invalidate the context on completion, denial, expiry, Quit or changed request. Ignore late callbacks.

Allow **at most120 seconds** for a Mac prompt, capped by the request's remaining lifetime. Serialize prompts; never automatically retry or open repeated prompts. Denial/cancellation/failed verification issues no code. Noninteractive/system-cancel failures may show Bring Fetch forward / Retry while the original request remains valid, requiring another deliberate action; otherwise end with safe OAuth denial/unavailable handling. The exact foreground behavior needs isolated macOS evidence.

## Proposed token lifetime, renewal, restart and revocation

| Item | Proposed lifetime / behavior |
| --- | --- |
| Pending browser approval | Ten minutes from creation, including any OS prompt. Reloading the same bound request does not reset it or auto-approve. |
| Authorization code | Sixty seconds, one use. |
| Access token | One hour. Use a Fetch-generated opaque random credential; no identity/profile data or provider secrets embedded. |
| Refresh token | Expires after ninety days without a successful refresh. No fixed absolute expiry while the connection is actively renewed. |
| Ordinary refresh | Rotate to a new refresh token atomically. Earlier access tokens in that family may remain valid until their original expiry, provided the grant stays active. |
| Refresh race/reuse | First successful refresh wins. Reuse of its spent token revokes the whole family and requires reconnecting. No hidden grace/replay window. Concurrent refreshes or a lost successful response can therefore require approval again; actual clients must prove acceptable behavior. |
| Quit/reopen | Drop pending approvals/codes. Preserve successfully saved client registrations, granted connections and revocations; valid unexpired connections resume without a Fetch account/browser prompt. |

Bind token state to issuer, client, protected service, approved scopes/targets and connection family. Persist protected token/grant state locally before success; no PostgreSQL dependency. Exact file/key/crypto/safe-write mechanisms remain their separate open review. A failed issuance/rotation write publishes no new successful token result. A committed rotation whose response is lost retains the strict reuse policy; do not claim automatic recovery.

Revoking either access or refresh token revokes that connection's entire family, including its other tokens. Block renewal/new admissions immediately. Remove its queued unforwarded work, stop its progress/result delivery and owner-bound interactions/continuations, close its streams as the supported protocol permits, and request best-effort cancellation for already-forwarded work. Late upstream messages may resolve backend accounting but cannot restore delivery/authority. Preserve D68 uncertain slots until terminal evidence/confirmed local exit, with D63's explicit forced-remote-recovery exception; revocation never proves external work stopped or rolled back. Other app connections are unaffected.

This contract supplies client self-revocation through the standard endpoint. A human-facing connection list/removal UI and its exact product APIs/tools remain the onboarding review; no unshown administration route is implied here.

## Bounds, errors and response handling

Propose16KiB request-body limits on registration/token/revoke/approval forms, at most32 pending approvals across Fetch and one OS prompt at a time. Propose at most256 saved client registrations and256 active connection families. Full/temporarily busy admission uses safe native endpoint errors and429 with Retry-After rather than an unbounded wait. Existing clients/grants remain usable when new registration capacity is full. Human administration/cleanup and exact reusable limit schema remain required onboarding/implementation work before building.

Validate registered targets before any redirect. Invalid redirect/client binding or malformed request gets a local safe error page, never an attacker-selected redirect. Valid-target denial/expiry uses standard authorization error/state/issuer handling. Wrong PKCE, reused/expired code or invalid/expired/revoked refresh uses `invalid_grant`; unsupported grant uses `unsupported_grant_type`; bad scope/resource uses `invalid_scope`/`invalid_target`; client/registration errors follow standard `invalid_client`, `invalid_client_metadata` and `invalid_redirect_uri`. Use appropriate400/401 and safe503 on unavailable persistence/service, with no credential-bearing diagnostics. No automatic POST retry after an uncertain response.

All authorization/token/registration pages/responses carrying state or credentials use no-store; pages are unframeable and no-referrer, with origin/CSRF protections. Public metadata contains no private caller/provider data. Native MCP missing/invalid/expired credentials retains401/discovery; insufficient authority retains403. Provider credential errors remain distinct from Fetch authorization errors and do not trigger a new Fetch approval flow.

## Research and evidence still required

[MCP discovery](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/authorization-server-discovery), [registration](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/client-registration) and [security](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization/security-considerations) informed this proposal. Native code/redirect/resource/issuer/revocation details follow [RFC8252](https://www.rfc-editor.org/rfc/rfc8252.html), [RFC7636](https://www.rfc-editor.org/rfc/rfc7636.html), [RFC7591](https://www.rfc-editor.org/rfc/rfc7591.html), [RFC8707](https://www.rfc-editor.org/rfc/rfc8707.html), [RFC9207](https://www.rfc-editor.org/rfc/rfc9207.html), [RFC7009](https://www.rfc-editor.org/rfc/rfc7009.html) and [RFC9700](https://www.rfc-editor.org/rfc/rfc9700.html). Numeric defaults, scopes and Fetch's resource grouping are proposed Fetch choices, not standards-prescribed values.

Current [Codex documentation](https://learn.chatgpt.com/docs/extend/mcp), [Claude Code documentation](https://code.claude.com/docs/en/mcp), [OpenCode documentation](https://opencode.ai/docs/mcp-servers/) and [released OpenCode provider](https://github.com/anomalyco/opencode/blob/v1.19.4/packages/opencode/src/mcp/oauth-provider.ts) support the registration/renewal comparison. They do not establish live Fetch interoperability. Maintained ToolHive/Fosite and official C# patterns were reused as research, not selected Rust dependencies or copied implementations.

Apple documents [device-owner authentication](https://developer.apple.com/documentation/localauthentication/lapolicy/deviceownerauthentication), [evaluation](https://developer.apple.com/documentation/localauthentication/lacontext/evaluatepolicy(_:localizedreason:reply:)), [context invalidation](https://developer.apple.com/documentation/localauthentication/lacontext/invalidate()) and [reuse duration](https://developer.apple.com/documentation/localauthentication/lacontext/touchidauthenticationallowablereuseduration). This confirms a possible mechanism; actual interactive behavior/freshness/binding remains untested.

After explicit testing authorization, D46-isolated evidence must cover all three clients, DCR metadata defaults, scope omission/subsets and management separation, wrong redirect/resource/PKCE/state/issuer and code replay, Mac denial/unavailable/foreground/late callback, concurrent refresh/lost response, durable restart behavior, revocation of queued/running streams/continuations without cross-client leakage or false rollback, raw data and matching UI/agent guidance. No environment, service, certificate, OS trust, personal client or credential was changed/tested while drafting.
