# Local MCP connector — research and plan

## Automatic first approval — selected November 12, 2026

Claude/Codex starts its configured Fetch command. If that client profile has no saved approval, the connector opens/focuses Fetch and requests the native permission popup automatically. The user approves there; no separate setup/login command is required. This supersedes the earlier mandatory setup step. Registration of the Fetch command in the coding client is still a separate one-time action.

Reuse the four existing connection routes and token families. Persist a private pending proof in the installation/profile's separate Keychain record; never put it in activation IPC or safe list responses. Native Fetch is the sole claimant, so it can save approval after the coding client stops waiting. Both warm activation.sock and cold app --connect-client carry only a bounded nonsecret client label. UI creates the same request through the pinned HTTPS client; activation grants no authority. Concurrent startups share one pending request, and proof cleanup preserves existing approved grants.

Claude's current startup default is about30seconds; Codex's is10seconds. Approval does not fabricate a successful MCP handshake or extend the parent's deadline. If it times out, approve in Fetch then use the coding client's Reconnect action. Research: [Claude environment reference](https://code.claude.com/docs/en/env-vars), [Claude MCP](https://code.claude.com/docs/en/mcp). Actual client/Mac checks remain user-manual. 54 focused checks pass, independent final source review ACCEPT, and signed development build/strict signature verification pass. Fetch was not relaunched.

## Current state — November 12, 2026

User selected native setup launch/focus/popup and said go ahead. Connector, four
approval operations and Connect-apps UI are implemented. Final independent
Evaluator ACCEPT covers source/fixtures;54 focused tests and signed development
build/strict signature verification pass. Actual native popup/Keychain and real
Claude/Codex MCP success remain unproven; user explicitly chose manual testing
later. [Evidence and manual handoff](../../milestones/local-mcp-connector/evidence.md).

The connector verifies the saved installation certificate; it does not disable
TLS, install trust, borrow existing client credentials, edit client configs or
stop the shared backend. UI-first setup is available; a separate terminal login
is not required. Optional explicit setup command launches/focuses Fetch. Normal
connect now automatically opens Fetch approval when its profile has no saved grant.
Existing approved profiles attach to the running backend.

The initial notification-stream token can expire and require client reconnect;
no transparent stream renewal or uncertain-call replay is promised. Earlier
proposal/pending sections below preserve discussion history and are superseded
where they conflict with this current state and the latest implementation notes.

November 12, 2026. User authorizes scoped work subject to reviewing new API and
connection-approval decisions. This is a proposal, not approval of new routes.
Project-scoped server work is separate and stays in project-scoped-servers.md.

User subsequently selected native setup launch/focus/popup and said “go ahead.”
Implementation is underway using the reviewed four operations. The earlier
proposal/status sections remain historical; the latest implementation section
below governs. Focused tests are underway; no real-client/native success is
claimed until evidence is recorded.

## Intended use

A coding client launches `fetch connect` as an MCP server over standard input and
output. This small process forwards messages to an already running Fetch app.
It verifies the saved installation certificate itself, so Claude/Codex do not
need a custom certificate setting for this hop. On first connection it opens Fetch for approval. It never stops
shared servers, changes system trust or obtains access merely by starting.

`connect` fits the existing CLI: src/main.rs currently has `app` and `dev` only.
A per-client connection option is proposed so Claude Code and Codex keep separate
saved credentials; a supplied client name is a label, not verified app identity.
Exact CLI options remain part of the plan below.

## Current source findings

- `src/main.rs`: default starts the native app; a connector branch must bypass
  ApplicationHost::prepare/start/shutdown and all app-ownership behavior.
- `src/mcp/mod.rs`: existing raw JSON /mcp gateway supports HTTP JSON/SSE, sessions,
  progress, cancellation, catalog changes and supported interactive replies.
  Call ownership currently uses grant owner plus request ID. Concurrent connectors
  reusing one saved grant need distinct request IDs or a reviewed session fix.
- `src/runner/raw.rs` and `src/runner/transport.rs`: existing raw helpers, newline
  framing, SSE decoder and HTTP lifecycle are reuse candidates. Do not use the
  whole upstream connection object as if it were a transparent client bridge.
- `src/application.rs`: internal HTTP client has a saved-cert-only trust pattern,
  no proxy/redirects and loopback address resolution. Reuse its TLS construction
  without exposing its native Settings capability to the connector.
- `src/oauth/mod.rs`: current browser authorization-code flow selects scopes,
  requires fresh Mac confirmation and issues per-connection access/refresh
  families. The browser page is still HTTPS localhost and has independent trust.
  Existing refresh/revoke/persistence should remain the common backend rules.
- `src/remote_oauth/mod.rs`: common Keychain backend and disposable credential
  storage seam exist. Connector credentials need their own namespace; do not read
  or copy clients' existing credentials, upstream server tokens or Settings keys.

## Maintained implementation research

Dedicated read-only Researcher checked these primary implementations:

- [mcp-remote](https://github.com/punkpeye/mcp-remote) is MIT, with recent
  November 2026 maintenance. Its [proxy](https://github.com/punkpeye/mcp-remote/blob/main/src/lib/utils.ts)
  and [dependencies](https://github.com/punkpeye/mcp-remote/blob/main/package.json)
  provide a useful bidirectional transport reference. Bundling it adds Node,
  Express/Undici and its own auth/cache behavior. Do not adopt its optional
  payload filtering or protocol transformations as raw preservation.
- [Official Rust SDK](https://github.com/modelcontextprotocol/rust-sdk) is actively
  maintained; Fetch already pins rmcp 3.8.0. That published crate is Apache-2.0;
  repository license transition retains older MIT contributions. Tokio, serde,
  reqwest and transport helpers are relevant existing dependencies.
- Source inspection of pinned rmcp shows its high-level HTTP proxy uses typed
  messages. Tool/CallToolResult types do not retain all unknown fields, and
  CallToolResult can insert missing content:[]. Its modern transport also filters
  some definitions by header annotations. A high-level typed client/server proxy
  is therefore unsuitable for Fetch's promised raw payload preservation.
  [Tool model](https://github.com/modelcontextprotocol/rust-sdk/blob/main/crates/rmcp/src/model/tool.rs),
  [HTTP transport](https://github.com/modelcontextprotocol/rust-sdk/blob/main/crates/rmcp/src/transport/streamable_http_client.rs).

Recommendation: reuse reqwest/rustls and Fetch's raw framing/SSE pieces at the
transport boundary. Retain supported JSON-RPC error code/message/data and unknown
payload fields. Any required request-ID/session correlation is outside preserved
Tool definitions, argument objects and result/error payloads.

Concrete regression lessons from mcp-remote include
[orphaned callback listeners](https://github.com/punkpeye/mcp-remote/issues/253),
[forwarding errors leaving requests waiting](https://github.com/punkpeye/mcp-remote/issues/293)
and [refresh coordination fixes](https://github.com/punkpeye/mcp-remote/releases).
These are historical findings, not claims all remain broken in current releases.

## Connection approval decision — awaiting the user

A. Reuse current browser approval. This minimizes auth changes and preserves
existing browser scopes/PKCE/Mac confirmation. The connector verifies its own
requests, but the external browser still needs explicit certificate trust. It
does not deliver a browser-certificate-free setup.

B. Recommended for the user's stated inconvenience: approve the connection in
Fetch's native window, retaining scope choices and fresh Mac confirmation, then
reuse the same grant persistence, expiry, refresh and revocation rules. This needs
a reviewed enrollment/approval API and UI. Starting the command only requests
access. It never grants access or borrows another client's grant.

A pinned Fetch-owned webview could wrap the existing browser flow, but adds
certificate challenges, browser cookies, redirect capture and native handoff.
It is not an existing automatic fix. Likewise a [device flow](https://www.rfc-editor.org/rfc/rfc8628.html)
still requires approval UI and new token behavior. Neither is silently selected.

## Isolated testing findings

Dedicated read-only Tester found reusable pieces:

- tests/development_https.rs: ephemeral HTTPS listener/certificate and process
  trust, including wrong-certificate/hostname rejection. Its basic backend denies
  authority; combine its TLS harness with the real McpGateway.
- tests/local_mvp_acceptance.rs: gateway/runner fixture. The simple fixture caller
  has one fixed owner; tests need distinct real fixture grant families to prove
  separate clients, plus repeated IDs within one client profile.
- src/oauth/tests.rs: registration/approval/code/token paths using memory keys and
  fake native confirmation, without real Keychain or OS dialogs.
- tests/native_state/confinement.rs: deny-default process sandbox and disposable
  canaries. Its current network denial must be narrowed only for the task-owned
  listener and tested; a temporary HOME alone is not isolation.
- tests/isolated_native_mcp.rs: real synthetic peer, raw payload and process
  lifetime evidence. No package installation is necessary for this connector.

Focused acceptance: raw list/call/error round-trip; progress, catalog notification
and supported interaction; two client owners using the same IDs; two concurrent
sessions sharing one profile; cancellation without disturbing another caller;
closing one connector leaves the app/server/other client alive; saved-grant reuse,
revocation, connector restart and backend restart without replaying uncertain calls.

Actual Claude/Codex tests require confined executables and fresh configurations,
no inherited hooks/plugins/MCP definitions, dedicated test credentials and verified
file/network denial canaries. Current docs do not establish a ready isolated Mac
user/VM for production native approval/Keychain tests. Existing persistent Codex
test state must not be reused automatically. Do not claim actual client or native
approval acceptance from fixture tests. No actual client/server/account/Keychain,
trust or personal configuration operation has run in this pass.

## Bounded implementation plan — Planner pass

This is one milestone. No project selection, server-specific startup/login,
release packaging, notarization, credential migration, personal client edits or
OS trust changes are included. `fetch connect` uses the existing binary; making
it available on PATH or changing the app bundle is not assumed in this pass.
Use the explicit signed executable path in disposable test-client configurations.

1. Add a connector branch that never creates ApplicationHost or owns app/server
   shutdown. Proposed ordinary commands are `fetch connect --client claude-code`
   and `fetch connect --client codex`. They select separate saved local profiles;
   those names are display labels, not cryptographic identity. Resolve the same
   development/release root and port defaults as the app, with explicit existing
   style data-dir/port options for the protected test instance. Never discover or
   attach to another arbitrary endpoint/certificate automatically.
2. Reuse reqwest/rustls pinned-certificate construction, exact localhost hostname
   validation, loopback resolution, no proxy/redirect and no built-in trust roots.
   Read only the public certificate and this profile's connector grant, not the
   app's encrypted grants, native Settings capability, upstream credential files
   or existing Claude/Codex logins. Certificate replacement fails visibly; no
   disabled-verification fallback, trust install or automatic identity reset.
3. Bridge raw newline JSON-RPC to the existing /mcp HTTP JSON/SSE transport. Reuse
   raw helpers and SSE framing; keep the backend's supported negotiation and
   method behavior. Do not advertise unsupported capabilities or filter unknown
   Tool/result/error fields. Supported notifications, progress, cancellation and
   interactive requests/replies must flow in both directions. Stdout is MCP only;
   connector-owned failures go to stderr and, where needed, matching protocol
   errors clearly distinct from preserved backend/upstream errors.
4. Give outgoing connector requests random per-process ID namespaces and reverse
   the mapping on responses and cancellation references. This handles equal IDs
   from concurrent sessions of the same saved profile without changing arguments
   or adding backend grant authority. Cleanup closes only this HTTP session and
   its in-flight requests; never revoke the profile or stop shared servers on EOF.
5. Store approved connector grants using the existing Keychain backend under a
   distinct connector namespace, separated by installation and profile. Use the
   existing OAuth refresh/revoke endpoints and common issuance/persistence rules.
   Serialize refresh with a per-profile cross-process lock and reread credentials
   under that lock: rotating refresh replay currently revokes the whole family.
   No plaintext fallback or copying/migration of existing client credentials.
6. On backend loss/restart, report the failure and end the broken connector
   connection so the coding client can start a fresh one with its approved grant.
   Never silently replay a dispatched tool call whose outcome is unknown. Test
   that fresh startup works after Fetch returns. Automatic transparent recovery
   of the same stdio session is not selected; do not claim it exists.
7. Add focused disposable transport/approval fixtures, then independently test
   real clients in a proven isolated environment. Evaluate source and actual
   evidence, fix material findings, rerun only affected checks and integrate.

### First approval without coding-client startup timeouts

Propose a human-run `fetch connect login --client claude-code` (and a separate
Codex profile), then configure each client to launch ordinary `fetch connect`.
The login mode presents human instructions; ordinary mode remains stdio-only and
noninteractive. Without an approved grant, ordinary mode reports that prerequisite
and exposes no tools. It cannot wait indefinitely or automatically approve a
request. The one-time login command and native approval are a meaningful proposed
setup behavior, awaiting the user's decision with option B.

### Complete proposed native approval operations (option B only)

All four operations are proposed, not approved. They use the existing HTTPS
listener and common backend state. Application JSON responses retain next_steps
and no-store. No ordinary MCP grant or tool can approve client connections.

| Method/path | Caller and input | Pending/success and authority |
| --- | --- | --- |
| POST /api/v1/connections/requests | Connector, with untrusted display label and requested existing scopes; no existing grant needed to request access | Create pending request only. Return opaque request ID, private random claim secret, a display matching code and expiry. No tool access, native capability or token is issued. |
| POST /api/v1/connections/requests/{id}/claim | Request owner presents that request's claim secret in POST body, never URL | 202 while pending. After human approval, one successful claim returns a distinct client ID and ordinary access/refresh credentials, with expiry/scopes. Only possession of this request's proof can claim; another pending request cannot be claimed by ID/name. |
| GET /api/v1/connections/requests | Existing native Settings authority only | Read safe pending details for Fetch's window: display label, matching code, requested scopes, expiry/state. Never return claim secrets or credentials. Read does not approve or open a Mac prompt. |
| POST /api/v1/connections/requests/{id}/decision | Existing native Settings authority, explicit allow/deny and selected scope subset | Deny grants nothing. Allow freezes this request, asks for the existing fresh request-bound Mac confirmation, and marks it claimable only on success. Never approve unrequested scopes or use a prior Mac unlock as approval. |

Use the existing ten-minute pending lifetime, 32-request admission bound and
120-second maximum confirmation capped by remaining request lifetime. Default
requested scopes are catalog, tool calls and content read; management must be
requested separately and remains unchecked by default. No selected scope means
deny. The connector and Fetch window show a matching code so the user can choose
the intended request; a supplied Claude/Codex name is not verified identity.

Proposed result/error policy: invalid input/scopes => 400 INVALID_REQUEST;
unauthorized native operations => the existing unauthenticated/forbidden behavior;
unknown request or wrong claim secret => indistinguishable 404
CONNECTION_REQUEST_NOT_FOUND; valid-owner denial => 403 CONNECTION_DENIED;
valid-owner expiry => 410 CONNECTION_REQUEST_EXPIRED; pending/in-progress claim
=> 202 with poll interval 2 seconds; admission/busy => 429 with Retry-After;
storage/native-confirmation unavailable => existing specific failure/no success.
Mac denial cancels approval; unavailable confirmation allows an explicit retry
while the request remains valid, matching current browser behavior. No automatic
prompt retries. No request mutation or stale callback can approve a changed or
expired request. No token/claim secret goes into logs, guidance or native lists.

Reuse the existing one-hour access lifetime, refresh rotation/expiry, durable
family persistence and revocation. Factor the existing issuer rather than
registering fake OAuth redirect URIs or creating a weaker token type. Native
connection records can be refresh-capable without pretending to have completed
the browser authorization-code redirect flow. Existing browser OAuth stays intact.
Pending approval is memory-only and ends on Fetch restart; approved saved grants
remain durable. Claim is single-use, without an exactly-once delivery promise:
if the response is lost or the connector cannot save the grant, report that fact
and require explicit new approval rather than pretending setup succeeded.

### Exclusive implementation ownership once decisions are accepted

- Connector Coder: new src/connector/ modules, src/main.rs branch, narrow lib.rs
  export and common raw/SSE/TLS helper exposure. Avoid unrelated source edits.
- Auth/UI Coder after option B approval: new connection-request handlers,
  narrowly factored oauth issuance, dedicated native approval module and minimal
  existing UI wiring. Preserve unrelated OAuth/credentials/UI changes in tree.
  Load shared UI definitions before that UI pass.
- Tester: focused fixture/integration tests and confinement evidence; use distinct
  approved owners and same-profile sessions, not fixed-owner fixture shortcuts.
- Evaluator: read-only review of payload fidelity, authority, lifetime and evidence.
- Orchestrator: serial docs/journal updates and accepted-change integration.

### Acceptance and remaining practical limit

The focused test set covers unapproved/denied/revoked access; raw definitions,
large-number/unknown-field arguments/results and error data; valid/wrong cert;
notifications/progress and one actual supported interactive request; equal request
IDs from distinct clients and same-profile concurrent connectors; cancel one call
while another succeeds; refresh sharing; EOF leaves shared app/server alive;
connector reconnect and backend restart without replay. Use fixture native
confirmation and memory credentials for automated tests.

Real Claude Code and Codex must each initialize/list/call through their actual
stdio client path with disposable settings and no inherited hooks/plugins/servers.
Use verified resource confinement and dedicated client test accounts. Native
approval/Keychain checks require an established isolated Mac user/VM or equivalent
verified environment; existing docs do not establish one is ready. Do not reuse
personal login or persistent test state. No actual-client/native acceptance is
claimed until those actions are observed.

The user subsequently selected setup launching/focusing Fetch with a native
approval popup; see the latest dated section below. That settles presentation,
not the exact amended enrollment APIs or setup trigger. Review the consolidated
contract before new authentication implementation.

## UI-first setup alternative — November 12 clarification

User asks whether setup can happen in Fetch UI or on a website instead of the
proposed human-run `fetch connect login` command. No implementation or exact new
authentication contract has been approved by this question.

Recommended revised presentation: Connect apps -> choose Claude Code or Codex ->
review permissions -> Allow plus fresh Mac confirmation -> save that named
connector's approved credentials using its own Keychain namespace -> show the
client registration command. The installed client subsequently launches ordinary
`fetch connect --client <profile>`. A separate human-run login command need not
be required for this UI-first flow.

This is an alternative to the earlier connector-initiated request/claim setup,
not behavior already delivered by the four proposed routes. The backend must
still own client/grant creation, approval and credential persistence under a
reviewed native-only API; the UI must not mint credentials itself or expose a
Settings capability to connectors. Exact changes to the proposed operations and
ownership of saving the connector grant require review before implementation.
The earlier login command remains an optional proposal, not a selected requirement.

The minimal UI-first proposal displays a copyable registration command. Fetch
writing Claude/Codex configuration directly is a separate product choice and is
not authorized by discussing UI approval. Personal configurations remain intact.

A website can provide instructions and start a reviewed handoff to the local app.
A localhost HTTPS approval page retains the browser certificate issue. A public
website cannot by itself provision local Keychain credentials or change local
client configurations; that requires a local-app action and reviewed handoff.
No hosted authentication, deep-link scheme or website implementation is selected.
Recommend native Fetch setup for this bounded local connector.

## Selected setup presentation — November 12 follow-up

User specifies that the install/setup process launches Fetch, focuses it on the
OS, and shows a popup screen. This selects native UI approval presentation over
a required terminal login step. Reuse an already running Fetch instance; bring
its window forward rather than starting a second backend. User approval and the
existing fresh Mac confirmation remain required before granting any access.
The popup should identify the requested client connection, show requested
permissions with a selectable subset, and offer Allow/Deny. Keep independent
Claude Code and Codex credentials.

This is a setup-time launch requirement. It amends the earlier assumption that
setup always requires the user to have already opened Fetch. Ordinary forwarding
still attaches to the existing backend and reports unavailable status; closing a
connector must never stop Fetch/shared servers. No general service auto-launch,
release packaging or OS trust installation follows from this selection.

Trigger detail remains to resolve in the concrete setup contract: client add
commands can merely register configuration and do not universally start their MCP
server immediately. Do not promise that `claude mcp add` or `codex mcp add` alone
opens a popup. A reviewed Fetch setup action/helper can launch/focus the app, or
the client's first connector startup can request the popup. The setup must handle
the real client startup deadline without granting access or hanging indefinitely.
No helper name, app-location/PATH installation, modified client add command,
new API fields or permission bypass is approved just by selecting this UX.

The earlier connector-initiated/native-window alternatives need consolidation
into this selected popup presentation before auth code. This is not approval of
the previous four-operation contract unchanged. Resolve enrollment proofs, safe
handoff, credential persistence and exact setup API together, preserving the
existing common grant issuer/refresh/revoke and native-only approval authority.

## November 12 — go-ahead and source implementation underway

User says go ahead after selecting native setup launch/focus/popup. Planner
confirmed UI-first setup and optional `fetch connect setup --client ...` fit the
same four reviewed operations; no new route or permission is needed. Ordinary
`fetch connect --client ...` is transport-only and needs saved approval.

Auth source implements the four operations in src/oauth/connections.rs and uses
common OAuth family issuance/refresh/revoke. Root's native popup uses the APIs,
selects requested scopes, confirms through Mac and only claims UI-owned proofs;
external setup callers retain their own proof/credential ownership. UI-created
requests save via the shared connector profile store. The popup reuses shared UI
controls and existing Iced focus, whose macOS implementation activates the app.
Connect apps displays stdio registration commands; Copy never edits client config.

New src/connector code and CLI wiring are being integrated. Profiles reuse the
existing native Keychain backend in an independent connector namespace. Tests use
memory credentials only. A fixture-only harness=false test target is permitted
for isolated actual Claude/Codex protocol smoke through production connector code;
it is not a production auth bypass or evidence of real Keychain approval.

No running personal Fetch/client, saved credentials, trust store, server inventory
or personal configuration has been changed. Builds/tests use owned fixtures;
source/build success alone does not establish native popup/Keychain acceptance.

## November 12 — initial evidence and review fixes

Initial focused OAuth/remote-renewal selection passes30 and raw bridge/profile
selection passes7, including real task-owned HTTPS+gateway fixtures, unknown
payload fields/errors, concurrent client/same-profile IDs, interactions/progress,
cancellation, reconnect/backend restart, different-certificate rejection and
rotating-refresh serialization. Tests use injected memory credentials, not
production Keychain. Fixture temporary roots were corrected to private0700;
production checks were retained.

Independent implementation review requested fixes for cancellation overtaking a
newly dispatched call and UI claim proofs lost when the window closes. Dispatch
now waits for authenticated response headers before sending the corresponding
cancellation. UI-owned proofs persist across backgrounding and clear on Quit or
terminal request expiry/denial. Focused regressions are being retested; no final
ACCEPT yet. Real-client smoke has not yet passed confinement, and no real native
approval/Keychain action has run.

## November 12 — first manual Claude connector failure and walkthrough

User changed the project-local Claude Fetch registration to the bundled stdio command. Read-only inspection confirms a valid seven-argument command with one complete data-directory argument, claude-code profile and port53672. Existing Fetch started after the signed build, owns activation.sock and responds200 to certificate-verified public authorization metadata. Do not reinstall or alter client settings as a guessed fix.

Initial screenshot shows connecting; subsequent screenshot shows connected, tools fetch failed, protocol2026-07-28, capabilities none and connection closed. Real Claude tool/catalog acceptance therefore remains failed, despite54 prior isolated checks. User suspects slow loading of the full catalog. Source list_tools visits server catalogs sequentially, so latency is plausible but unmeasured; do not present it as established cause or change discovery/timeout behavior without evidence.

User manually clicked Reconnect while root performed read-only process inspection. Existing connector sample reached stdin reading and had an established HTTPS connection; no sampled Keychain or profile-lock wait was observed. This is consistent with leaving initial approval and entering forwarding, not proof of complete MCP/tool discovery. No additional client was launched by agent, no saved credentials were read/exported and no app/services were restarted. Task-owned raw stack reports were removed after recording this nonsecret summary.

UI explanation confirmed from source: the Connect-apps key itself creates a request with client_name claude-code, even if Claude is closed. After explicit permission/Mac approval, native UI claims and saves that profile. Later normal connector startup reuses it and skips the popup. Approval belongs to a saved installation/client profile, not a running process or verified application identity. Pending requests use existing four routes; direct browser HTTPS auth is unchanged. User asks for an exact step-by-step route/file explanation before further implementation; no application source changed in this diagnostic pass.

Next action: explain current implementation, then inspect the failing discovery/catalog exchange and elapsed time. Do not treat the manual failure as accepted completion or silently change permission/identity/catalog architecture.


## November12 — remaining modern subscription transport defects

User reports slow reconnect and tools fetch connection closed even after server-first build. Read-only sampling of their existing connector shows Runtime::block_on and a stdin read, not runtime shutdown; backend public metadata responds200 in0.003289seconds. The connector later exits. No agent-launched coding client or credential access occurred. The original tool-count attribution remains unproven.

Research identifies two actual source bugs: outgoing subscription listen IDs are rewritten but protocol-owned subscriptionId metadata is not restored, so maintained SDK acknowledgement validation fails; modern stdio cancellations are incorrectly POSTed as notifications to HTTP, where they are rejected, instead of dropping only the matching stream. This provides a concrete possible chain (ack mismatch→timeout→cancel→fatal rejection), not a captured exact Claude trace.

Bounded repair restores only protocol subscription-ID references from this connector's active subscription map and adds separate per-request modern cancellation tokens; existing legacy dispatch receipt/POST, permissions, raw tool payloads, progress tokens and global connector shutdown remain. No new route/auth/policy or speculative runtime shutdown fix. Independent real-rmcp-client fixture precedes implementation; actual installed clients remain manual.

Primary evidence: [subscription protocol](https://raw.githubusercontent.com/modelcontextprotocol/modelcontextprotocol/main/docs/specification/2026-07-28/basic/patterns/subscriptions.mdx), [modern transport](https://modelcontextprotocol.io/specification/2026-07-28/basic/transports/streamable-http), existing official rmcp3.8.0 service/client.rs acknowledgement validation. SDK use is test-only async transport, no copied implementation or new runtime auth/library.

## November12 — subscription fix accepted

Confirmed modern subscription correlation and cancellation defects are fixed. Maintained rmcp SDK reproduces acknowledgement timeout before the fix (3.03seconds), then completes the isolated startup/subscribe/change/call/cancel/list/call sequence in0.03seconds. Final integrated19connectorchecks pass0.30seconds. Signed development build-only22.53seconds, repeated strict signature verification, scoped whitespace and independent Evaluator ACCEPT pass. Actual installed Claude/Codex use remains manual and is not claimed successful yet. Fetch and clients were not relaunched; user should explicitly Quit/reopen Fetch and Reconnect Claude. No personal settings/credentials/trust/services or project API/auth decisions changed.

The build briefly caught concurrent project core changes before their Caller interfaces were complete. Root added only the missing projects data-module declaration; its owning task completed internal Caller wiring and then froze interfaces for the successful build. No unrelated code was reverted and no project route/auth contract was approved by this pass.
[Evidence/manual handoff](../../milestones/connector-subscriptions/evidence.md).
