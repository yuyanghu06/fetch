# Remote connection API amendment — approved presented behavior

October27, 2026. **Presented observable amendment approved in D63 after the complete proposal and requested TLDR.** D61/D62 already select stdio, Streamable HTTP and legacy HTTP+SSE handling in the shared runner, with per-tool internal routing metadata. The existing-route remote behavior, shown schemas and reconnect defaults are selected. Exact unshown serialization/implementation mechanisms and the separately listed protocol/auth/destination/security prerequisites remain open. This does not authorize application implementation or live tests.

## Existing routes and authority

Reuse discovery, installation submission/progress, ordinary/named server GET, config PATCH, credentials PATCH, authenticate POST and restart POST. No new public route, generic call wrapper or tool-argument envelope is introduced. Existing management permissions, authorization-before-target disclosure, no-store responses, source selection, installation serialization/reuse/partial success and D22 next_steps conventions remain. Invocation authority remains distinct from management/listing.

Existing local-server behavior remains as approved; the following is a remote-record variant. Settings-only manual secret entry remains an exception to agent management. Exact permission implementation/Settings proof and provider integration mechanisms are separate open work.

## Discovery and preparation

GET /api/v1/setup/discovery recognizes supported literal HTTP/SSE definitions in the already selected sources without network contact. Use type remote for these rows; retain existing row fields and null package/version/executable/script/cwd fields. Add connection to discovery rows: null for local rows; for remote rows an object with transport (streamable_http or sse), origin (safe scheme/host/port), and header_names (observed header names only, sorted and case-insensitively deduplicated). The discovery origin contains no path/query/userinfo. This is safe metadata, not a usable credential-bearing URL returned to an agent.

Eligibility/default selection follows D35 once a supported interpretation is established. Unknown helpers/interpolation, unsupported transport or an endpoint whose non-secret literal identity cannot be safely established yields the existing blocked-row code and safe guidance. No arbitrary command/header helper, secret resolution or remote-version probe is run. Never strip query components to manufacture another endpoint or use a name filter as proof that a URL is non-secret. Source credential/header values are not migrated. Exact reader support and safe endpoint interpretation remain prerequisites for each supported definition.

POST /api/v1/setup/installations keeps exactly servers:[{config_path,server_name}]. A remote item saves a durable connection definition instead of downloading a runtime/package. Use the existing pending/installing/succeeded/failed item and installing/succeeded/failed group states; installing means preparation, not a package download guarantee. Preparation success requires the actual saved record/server ID, not a connection or login. Existing source-at-item-start capture, successful-install reuse, failed-item retry and retained successes apply. Reusing an installation never silently retargets its saved endpoint or imports new source credentials.

No connection, authentication, provider test or tools/list occurs merely to discover/prepare a remote item. Installation GET remains a side-effect-free read with existing errors, polling and same-run lifetime. A newly prepared remote record starts runtime.status stopped and restart_required true until an explicit successful first connection applies it. Installed does not mean authenticated/ready.

## Ordinary server GET and saved connection shape

GET /api/v1/servers keeps its existing filters/order/errors/envelope. For remote records config is the following variant (invented values):

```json
{
  "kind": "remote",
  "package": null,
  "executable": null,
  "args": [],
  "working_directory": null,
  "environment": [],
  "connection": {
    "transport": "streamable_http",
    "url": "https://example.com/mcp",
    "headers": [
      {"name": "Authorization", "credential_ref": "remote-auth"}
    ]
  }
}
```

Transport is streamable_http or sse. The returned endpoint is the safely established saved non-secret literal, not a raw source entry or redacted-but-changed URL. Local GET records retain their existing shape without a required connection field. Remote launch placeholders indicate not applicable; do not invent a process ID, runtime binary or package version.

For remote records runtime.status describes Fetch's client connection: stopped means disconnected, starting means connecting, ready means compatible MCP responsiveness/complete tool discovery, restarting means a reconnect attempt, failed means a failed connection and unknown means insufficient evidence. Runtime/authentication status/error shapes remain the existing fields; tools/list is never D25/D31 credential verification. GET reads known state without probing/network work.

GET returns saved configuration. Calls use the currently applied connection/routing context until an explicit successful reconnect applies a captured configuration. Edits cannot redirect a call already associated with an older context. D62 per-tool routing remains internal; exact physical duplication/reference representation is not selected here.

Return remote-only unresolved_prior_calls as a safe nonnegative server-level count: calls from forcibly retired remote contexts whose outcomes remain unknown. It is not a count of proven running jobs or a result history. Do not expose caller IDs, arguments, results or secrets. Its count lasts for the current Fetch run; no cross-Quit recovery promise is added, and disappearance across Quit is not evidence of stopped provider work.

## Config PATCH: mutable header references, fixed endpoint identity

For the MVP the imported remote URL and transport are read-only like a local launch entrypoint. A different endpoint needs an intentionally distinct import/fresh credential setup; successful D34 source reuse must not retarget an old record. This prevents a management agent from directing existing stored credentials at a different host. It adds no uninstall/replacement operation or source-file editor.

Remote PATCH /api/v1/servers/{server_id}/config accepts this additional remote-only shape:

```json
{
  "connection": {
    "headers": [
      {"name": "Authorization", "credential_ref": "remote-auth"}
    ]
  }
}
```

The connection object is a partial patch: omitted headers are unchanged; a supplied array fully replaces editable bindings; [] removes bindings without deleting encrypted entries; {} is a no-op. Neither connection nor headers accepts null. Remote launch fields or attempts to change url/transport are 422 INVALID_CONFIG; malformed/wrong-type/unknown fields are 400 INVALID_REQUEST. No new execution-setting approval follows; that separate proposal retains its status.

Each header binding is exactly name and credential_ref. Names must be valid HTTP field names and case-insensitively unique. References resolve only to this server's assigned entries; a missing value remains a missing requirement, not an imported source value. Values remain encrypted/Settings-only. Fetch-owned transport/protocol headers cannot be overridden, including Host, framing/content-negotiation and MCP metadata headers. Authorization is allowed only through a supported manual-credential integration; it cannot override a Fetch-owned OAuth Authorization header. Exact reserved-name/runtime header construction mechanisms remain protocol/implementation work, not arbitrary user-header execution.

Use existing config authorization, active-auth rejection, per-server serialization, full validation/atomic safe save, no-op and saved-versus-applied rules. An actual binding change sets authentication.status unknown/error null and recomputes restart_required, without deleting credentials or connecting/testing automatically. Edits during reconnect are allowed but do not change that attempt's captured settings. A no-op preserves prior pending/auth state. Config 200 retains server_id/status/changed/restart_required/next_steps, without a duplicate config body. Config save does not require provider verification because it saves references only; credential-save 200 still requires D25/D31 verification.

## Matching Fetch-owned tool amendments — presented behavior approved

Add the shown optional connection:{headers?:[{name,credential_ref}]} to fetch__update_server_config's existing input schema beside server_id and the accepted local launch fields. The adapter places the supplied connection object in the existing config PATCH body unchanged; backend server-kind validation, no-op/replacement/authority/error rules remain identical. A connection object on a local server is rejected as INVALID_CONFIG; the model cannot edit remote url/transport through this field. No inline header values, separate remote-config tool or generic call wrapper is introduced. Existing execution fields remain independently proposed, not accepted here.

fetch__discover_servers, fetch__install_servers, fetch__get_installation and fetch__get_servers keep their inputs and return the amended API JSON using D49's existing text-block presentation. Update descriptions for remote preparation/state/uncertain-work counts. fetch__authenticate_server keeps its input and auth boundaries. fetch__restart_server keeps server_id/force_restart inputs but describes the remote Connect/Reconnect variant, phases/results and force risk. D48 coverage does not create duplicate APIs or bypass Settings-only secret entry.

## Authentication and credential writes

The existing authenticate POST and Settings-only credential PATCH apply to explicitly supported remote integrations with their existing shapes. Fetch obtains fresh provider authorization; no harness-to-Fetch bearer token, source-config token/header or helper-held secret is passed through as upstream authority. Use the same per-server encrypted storage and successful save/readback plus authenticated service-endpoint 2xx gate. MCP readiness and browser consent alone do not promote authentication.

Concrete provider endpoints, refresh/token ownership, TLS/destination rules and unsupported integration handling remain the selected authentication review. A missing integration returns truthful existing guidance rather than claiming universal remote OAuth or a working stored credential.

## Connect/reconnect through the existing restart operation

For remote records POST /api/v1/servers/{server_id}/restart means Connect/Reconnect Fetch's client connection. Existing force_restart boolean/default false, management permission, body validation, installed prerequisite, one active attempt, 202/Location and restart_id monitoring apply. Do not create a provider-process restart or replay operation.

Add operation:reconnect to remote acceptance replies and nested remote restart records. Existing local responses remain unchanged. Remote status still uses restarting/succeeded/failed, so HTTP200 on status GET means the read succeeded. The UI uses operation/config.kind plus structured phase/result, with labels Connect/Reconnect rather than promising a provider restart. The existing fetch__restart_server description must explain both variants; no new management tool is required solely for this distinction.

Approved presented remote attempt rules and defaults:

1. Capture the saved connection configuration and private server-scoped credential context. Gate new calls; queued not-forwarded calls fail with 409 SERVER_BUSY/execution not_started and no automatic requeue.
2. Normal mode waits up to60 seconds for already-forwarded work, including unresolved current-context calls. On timeout fail RESTART_WAIT_TIMEOUT, preserve the existing context/admission where usable and leave saved edits unapplied. Never silently force.
3. Close/retire Fetch's old client context within5 seconds. This is local connection-resource closure, not confirmation that remote work stopped or mandatory acknowledgement of a session deletion. If local closure fails, report CONNECTION_CLOSE_FAILED and do not create a second current context.
4. Allow30 seconds from beginning the replacement connection to establish compatible communication and complete readiness/tools-list pages. No tool invocation tests readiness. A valid empty list is allowed. Unavailable credentials/unsupported auth produces SERVER_AUTH_REQUIRED guidance without automatic browser login.
5. Remote pending phases are waiting_for_calls, disconnecting, connecting and checking_readiness, skipping phases not applicable to a first connection. Success code is CONNECTION_READY, not SERVICE_STARTED. Clear restart_required only if captured settings match the latest saved settings. Edits during the attempt remain pending.
6. Connection/readiness failure uses CONNECTION_FAILED or CONNECTION_READY_TIMEOUT (or auth guidance above), closes/tracks the replacement Fetch context and leaves it unavailable. No automatic rollback/retry/reconnect loop. Request-level acceptance/read/overlap errors remain the existing restart family and are not job success.

Backgrounding/red-close retains connections. On opening Fetch, the manager makes one connection attempt for each saved installed remote record, analogous to local startup, without interactive login or tool replay; ordinary GET shows starting/ready/failed/auth state. Discovery/preparation/config-save still do not connect immediately. Explicit Quit closes Fetch-owned connections, not remote providers or their already accepted work. No cross-Quit attempt/result recovery is promised.

### Explicit force-reconnect tradeoff

force_restart:true skips draining, requests best-effort cancellation and retires the old Fetch context. Calls without terminal outcomes remain execution unknown; no replay and no old reply/continuation is delivered to another caller or retargeted to the replacement context. A new context is created only after Fetch-owned old resources close.

Allow fresh current-context admission capacity after this explicitly requested force recovery. This is a user-chosen override of waiting for old work; it does not release capacity because remote termination was proved. The configured concurrency limit then bounds current Fetch requests, not all work still running at the provider. Increment unresolved_prior_calls for retired unknown outcomes and retain a visible warning; do not call that count proof of running work or silently turn old outcomes into completed/not_started. Correlation/retirement resource bounds remain implementation prerequisites.

Normal reconnect still counts uncertain current-context work as occupied and may time out. D63 selects this force rule as a remote recovery override, not a D20 child-exit guarantee. Existing unapproved queue sizes/concurrency/timeout refinements remain unapproved.

## Errors and matching UI/agent sequence

Use existing safe API envelopes/no-store/next_steps. Management authorization failure is401/403 before target disclosure; unknown server404; unfinished install or active incompatible operation409; bad input400; invalid/fixed endpoint/header-reference change422; safe save/acceptance/read failure500. Discovery row failures and installation item failures keep their accepted schemas/codes rather than introducing a second job system. Reconnect job failures are HTTP200 nested failed with the remote codes above. Calls retain D50 not_started/unknown and raw terminal ToolResults; they are not reinterpreted as successful merely because reconnection succeeded.

The deterministic UI and next_steps must agree: prepare, inspect server state, resolve missing credentials/authentication through existing Settings/auth routes, Connect/Reconnect to apply, then monitor the named attempt at most once per second. Nothing reads guidance text as UI control. Use existing check_server_status/check_restart_status/authenticate_server/review_server_settings actions with remote-aware instructions. During forced recovery explain that old remote work may continue and that no invocation will be replayed. No automatic POST retry, credential migration, original harness cleanup or personal rollout follows.

## Separate prerequisites and future acceptance evidence

This contract does not silently resolve upstream protocol versions/fallback, D37 versus modern HTTP header-annotation filtering, allowed endpoint/redirect/legacy announced POST/OAuth-discovery targets, TLS policy, actual provider verification endpoints, exact field persistence or resource bounds. Do not follow arbitrary redirects/destinations or forward credentials across origins; existing intended loopback/private gateways are not blanket-denied by this proposal. If a connection cannot meet a selected policy, report compatibility failure rather than pretending it is ready. Those protocol/auth/implementation choices remain explicit before compatible execution can be accepted.

After implementation is authorized, D46 isolated evidence must cover mixed local/remote imports without contact or secret migration; partial failures/reuse; safe GET metadata; immutable identity and header-reference replacement/no-ops; auth/busy/save errors; saved-versus-applied routing; normal/forced connect phases/deadlines/failures; retained uncertainty/fresh-capacity tradeoff; no replay/caller leak; raw definitions/arguments/media/results; UI/guidance alignment; opening/background/Quit and denied callers. No such runtime tests or personal client/service operations have been run.

The next [required protocol adaptation proposal](mcp-protocol-adapters.md) presents the separately reserved annotation/version compatibility questions and native result-marker exception; it does not reopen D63 remote config/reconnect behavior or imply a chosen SDK/build.

October28 status correction: D64/D65 subsequently resolve the presented upstream revisions, annotation rejection/catalog mapping, native result marker and cache defaults. D68 approves the shown execution config/ordinary scheduler amendment. Destination/TLS/header reservation, provider verification, unshown schema/bounds and actual SDK/client proof remain open; [the canonical checklist](open-questions.md#remaining-decisions-checklist) controls the next review.
