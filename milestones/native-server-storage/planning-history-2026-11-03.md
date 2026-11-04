# Planning record before implementation

Historical proposals and decisions; current truth is in docs/architecture/native-server-storage.md.

# Server-owned configuration and authentication

November 3, 2026. Status: the user approved the initial proposal with Keychain
remote-client OAuth, then explicitly paused further coding to review the complete
route/change plan and discuss any server-specific code before it is written.
Partial application and fixture edits exist; they are incomplete and have not
been built or tested. No app, server, Finder, Keychain or personal-state operation
ran. Preserve this work without treating it as completed implementation.

## Standard-behavior condition — November 3

User accepts the timing/status direction as long as it follows normal practice.
The established behavior verified in the official library is saved service-supplied
expiry, a local check before using access, shared renewal/replacement and surfacing
missing/rejected renewal permission for user sign-in. Actual request rejection can
invalidate expected access earlier; transient failures are not automatically logout.
An early background timer is optional application behavior, not a universal harness
cadence or MCP requirement. The chosen same-authenticate-route rule applies to any
Fetch-scheduled renewal; do not infer a new mandatory polling system. GET auth/expiry
metadata is Fetch's reporting schema, not a standard MCP server status method.
Native hidden expiry stays null/unknown and GET stays side-effect-free. Preserve
these distinctions rather than treating the conditional reply as approval of extra
scheduler machinery, unknown auth evidence, or resuming application implementation.

## Proposed timing and GET server-authentication amendment

November 3, 2026. User asks when checks run, how they work, typical harness behavior,
and for authentication status/expiry in existing GET /api/v1/servers. The feature
is requested; exact fields/timing below are proposed for review. Coding stays paused.

How the clock information is obtained: an online sign-in response can include how
long access lasts. The official library saves the token receipt time and lifetime;
Fetch can project access_expires_at from them without decoding the secret or
contacting the service. A time comparison only predicts expiry; only an applicable
real authenticated response establishes accepted access, and grants can be revoked
early. Missing expiry means unknown, not no expiry. Fixed manual keys likewise have
no guessed deadline. Native server-owned credentials remain opaque unless their
supported behavior explicitly reports status/expiry; no token-file parsing/hooks.

Proposed timing preserving the actual authenticate-route decision:

- On initial/reopened online connection, use authenticate to check/reuse/renew saved
  access before connecting. Local native servers continue their own renewal.
- When an online login is saved/renewed with a known expiry, schedule one wakeup
  inside the library's near-expiry window. On wake, call authenticate; do not write
  separate renewal logic. Replace the scheduled time when a new expiry is saved.
- Before actual online requests, a lightweight local expiry check catches sleep,
  missed timers, downtime and already-expired access. When renewal is due, call
  authenticate before dispatch. Do not use SDK auto-refresh through a separate path.
- A real authentication rejection updates status and can invoke quiet authenticate
  recovery. Do not automatically repeat an uncertain tool action. A generic permission
  denial is not automatically an expired login.
- With no known expiry, no guessed periodic deadline: use saved access and handle
  actual rejection. Transient network/storage/service errors remain distinct from
  rejected/missing renewal permission; do not force new sign-in for every error.
- A background request cannot launch a browser or native login helper; needs-user
  outcomes are shown for explicit action. Exact request permission/mode is still open.

Research: inspected official rmcp-v3.8.0 AuthorizationManager::get_access_token loads
saved credentials and compares elapsed time with expires_in before use, using a
30-second early-renewal window; prepare_request calls it. This is an on-use path,
not proof of a permanent polling daemon. Use the compatible selected version's
actual policy; schedule inside the comparison window with clock rounding considered,
not a wakeup that repeatedly finds renewal not due. Claude Code documents automatic
renewal but no exact polling cadence. These findings do not authorize library calls
outside the selected Fetch authenticate route.

### GET response proposal (no new route)

Retain current request/filtering, fetch:manage/server visibility, response envelope,
next_steps, no-store behavior and existing errors. Add only safe metadata to each
existing authentication object:

- status: preserve existing known states unknown|required|authenticating|
  authenticated|not_required|failed. It reflects the latest known applicable check.
- access_expires_at: UTC RFC3339 timestamp or null; describes the access token Fetch
  uses for this MCP connection, not refresh-token lifetime or downstream Google access.
- last_verified_at: UTC RFC3339 timestamp or null; only a real accepted authenticated
  check/activity sets it. Saving secrets, clock comparison, public initialization,
  tools-list success without protected access or browser completion does not.

Example (illustrative times):

```json
"authentication": {
  "status": "authenticated",
  "access_expires_at": "2026-11-04T02:00:00Z",
  "last_verified_at": "2026-11-04T01:00:00Z",
  "error": null
}
```

For an unobserved native login use unknown and null metadata. If known online
access expires without successful renewal, authenticated no longer describes current
access: project unknown (or keep authenticating/required/failed when those apply),
retain last_verified_at and actual expired time so the UI can explain it needs a
check/renewal. Do not invent a new status enum silently. Future expiry alone never
proves a grant has not been revoked. No renewal-token expiry is promised unless
explicitly known; this proposal does not add that field.

GET only reads the saved/live projection, with cheap expiry comparison. It must not
start the library's refreshing token getter, read OS secret values for each poll,
contact providers, launch helpers/browser, save credentials or run a renewal. Fetch's
background/connection code owns checks even when nobody reads GET. Corrupt/unavailable
state follows existing read errors rather than secret leakage or false verified
status. No token/key values or new provider-file inspection are exposed.

Primary evidence:
[official token expiry/use/refresh source](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.8.0/crates/rmcp/src/transport/auth.rs),
[Claude Code automatic renewal](https://code.claude.com/docs/en/mcp#authenticate-with-remote-mcp-servers).

## Latest agreements — manual keys through PATCH; recorded native setup

November 3, 2026. User confirms the retained credentials route is for keys entered
manually, with shared Keychain saving/delivery, using PATCH when applicable. This
selects that narrow purpose, not unshown save/check/remove semantics. Native server
OAuth stays server-owned; automatic remote OAuth remains owned by authenticate
and the selected library. No new manual refresh-token engine or provider hooks.
Native Settings-only secret authority and no secret values in responses remain.

User also confirms authenticate should read setup information and execute existing
required functions. Asked whether that means recorded commands or interpreting
written setup documents, user explicitly selects **Run recorded setup commands**.
Authenticate reads the exact command assigned to this instance and uses one shared
launcher for the unmodified native helper; no README interpreter, shell inference,
package patches or per-provider login/refresh code. The command's executable,
arguments, working folder and supported state locations must be recorded explicitly.
The source/UI/import contract for recording those facts still needs review; current
installation records do not establish a universal native login command. Do not
infer authorization to execute commands or resume coding from this plan decision.

Background behavior was requested for explanation, not yet approved in this reply.
Proposed simple mechanism: at each Fetch launch create an unpredictable process-owned
secret kept only in memory. Scheduled HTTPS requests must present it; it is accepted
only for check/renew of existing online logins through authenticate. It cannot start
browser sign-in, launch a native setup helper, save manual keys, edit settings, or
invoke tools. It expires on Quit and is never provided to MCP servers/apps or saved
as a provider credential. Verify the app's own HTTPS certificate and perform normal
request validation. The exact input/default/status/error and internal authority
mechanism remain proposed for the complete route amendment before implementation.
If renewal cannot proceed, record needs-sign-in; the user can click Authenticate
for the existing explicit login path. No unexpected background browser windows.

## Current accepted plan — official library selected; no more code yet

November 3, 2026. User explicitly approves the official Rust MCP library for the
online sign-in/renewal work and requests an updated plan showing exactly what
Fetch must still handle before any more application code. Use its auth-only
functions/store interface behind the existing authenticate route and preserve
Fetch's raw MCP runner. This replaces maintaining custom remote OAuth steps.
It does not authorize resuming source edits, dependency resolution, builds or tests.

Selected boundaries:

- Local MCPs/helpers keep their normal login, callback, storage and refresh code.
  Do not patch dependencies or redirect native token writes into Fetch.
- Online MCPs keep their accounts and any downstream provider logins. Fetch acts
  as their client; the official library performs common client login/renewal rules.
- Authenticate button and background job call the actual existing authenticate
  POST; its handler invokes the shared library. No independent renewal code in
  the scheduler or per-server handlers. Library automatic renewal must not create
  another path outside that selected route.
- Remote-client grants use the selected Keychain storage with per-instance/root
  ownership. Existing legacy credentials/files/keys are untouched.
- PATCH /credentials stays for user-entered fixed keys with shared Keychain saving
  and delivery, as now confirmed. Exact save/check/remove behavior still needs review.
  Do not recreate the old native-provider token engine through it.

### Exactly what Fetch still owns

| Work | Fetch's actual responsibility | Why it remains |
| --- | --- | --- |
| Installation and instance records | Prepare original supported packages; remember each instance ID, command/arguments or URL, transport, working folder and applicable data locations. | The library cannot decide which servers the user selected or where Fetch installed them. Keep source definitions/credentials untouched. |
| Owned process/connection control | Start/stop local servers, open/close online connections, preserve per-instance state and implement explicit Restart/Reconnect/Quit. | This is the wrapper's connection work. The existing runner can remain; no provider auth belongs here. |
| Calling the library | For one saved instance, pass its connection and allowed login context to the library from the authenticate handler; map results into existing pending/success/error states. | A library is called by Fetch; it does not discover Fetch's own records/routes/UI by itself. One generic integration for all compatible online servers. |
| Browser and callback connection | Open the library-provided sign-in page only when permitted, receive the browser's return, and hand it to the same library session. Bind the return to the pending instance/user request and preserve deadlines/checks. | The library handles OAuth rules, but Fetch hosts the local receiving route and presents human actions. Any additional callback listener/URL needs contract review first. |
| One Keychain connection | Implement the library's shared load/save/coordination boundary, isolate root+instance+service identity, save replacement grants and read them back before durable success. Handle denial without plaintext fallback. | Persistent client access must be kept somewhere. Keychain is the user's choice; no new custom encrypted provider files or per-server secret adapters. |
| Scheduling and route authority | Decide when saved online access needs checking and call authenticate; authorize the actual self-HTTPS request using Fetch-owned restricted authority, verify its certificate and keep one renewal in flight per instance. | Early timing and literal route calls are selected Fetch behavior, not library/provider requirements. They must not borrow personal client tokens or native-window authority. Exact request authority remains to review. |
| Forwarding requests and reporting results | Route to the selected server, preserve raw upstream definitions/arguments/results and supported live messages, and distinguish connection readiness from actual account access. Do not replay uncertain actions. | Fetch sits between apps and servers; its raw runner and status layer remain. Public tool lists/browser pages alone are not account-auth proof. |
| Fetch access and execution policy | Keep apps signing into Fetch, caller/tool permissions, queues, concurrency/time limits, cancellation and restart coordination. | These are existing Fetch product requirements. The upstream server cannot decide which original app Fetch permitted to use a tool. Preserve separate TLS/app identity protection. |
| User controls and native folder access | Keep UI/status/call-limit controls; open only a known actual native config/state folder on a local human request. | These are Fetch's UI features. Folder location/native account separation cannot be inferred universally from package path or HOME. |
| Confirmed manual-key purpose | Through the retained PATCH route, save user-entered fixed keys in Keychain and supply only the assigned environment/header value through shared delivery. | Needed only for entries requiring user-supplied values and choosing Fetch to supply them. Purpose selected; exact save/check/remove behavior pending. Prefer native setup; no general .env loader or automatic renewal invention. |
| Removal and old-state preservation | Close idle owned connections and remove Fetch registration, preserving existing packages/native files/legacy keys and grants per the current request. | Wrapper removal is separate from provider logout and deleting account state. |

### Work delegated completely

Server/native-helper code: downstream account login, callbacks, native token files,
provider refresh/update and actual tool work. Fetch does not read/watch/intercept
those files to keep copies. Some native servers require manual setup; no universal
local authenticate command is available.

Official auth library: standard remote login-address discovery, supported client
registration, sign-in request construction, code exchange, token parsing/expiry
and refresh/replacement mechanics. Fetch supplies generic OS/network/UI/storage
connections and respects its existing security/ownership rules. Do not reimplement
these common steps or introduce service-name branches in Fetch.

### Exact implementation scope after remaining plan review

- Add a compatible pinned official rmcp auth dependency; check license/dependency
  constraints and features. Use only auth/store/network interfaces needed here;
  do not replace the raw runner/protocol models or add a universal proxy process.
- Replace custom remote discovery/login/refresh logic in manager/authentication
  and manager/remote_auth with thin calls into that library, retaining required
  destination/certificate/callback checks through generic integration.
- Remove Google helper interception/private bridge, ManagedStdio bridge leases,
  local Railway token injection and other provider-store-only functions/profiles.
  Discuss existing Railway extra-CLI installation before changing its package-specific
  preparation. Do not add a per-server replacement.
- Keep application OAuth/TLS storage independent; provider-store removal must not
  remove Fetch's own /oauth/token, app grant permissions or HTTPS identity.
- Integrate one Keychain store with the library rather than leaving a second
  parallel token format/refresh owner. Existing partial remote_oauth code is not
  accepted merely because it was written; adapt or remove duplicated logic after
  the library interface is fixed, preserving runtime data.
- Restore/narrow the credentials route only after its exact manual-key contract
  is agreed. Complete execution-only config, native folder route, UI/backend records,
  guidance and state-preserving removal against the full agreed contracts.
- Update affected guides/current architecture/UI-state coverage, then independent
  confined tests/evaluation, fixes and integration. No actual native/provider/client
  compatibility claim from source or fake Keychain/Finder fixtures.

### Remaining concrete choices — not new broad approval rounds

1. Background requests: recommend renew/check only; return needs-sign-in rather
   than open a browser. User action may open the browser. Present the exact request
   input/default/result/error amendment together before implementing it.
2. Scheduled self-request authority: present how Fetch creates a process-owned
   credential restricted to this endpoint and verifies the local HTTPS certificate.
   No new public scope/route or personal credential is silently selected.
3. Manual-key route: user confirms manually entered fixed keys via PATCH and shared
   Keychain delivery. Agree exact native form/authority, assigned input representation,
   removal and storage-vs-authentication outcomes; do not reopen selected purpose.
4. Native setup/folder/account separation: user selects recorded setup commands run
   through authenticate. Agree where exact commands/state paths are recorded. Unknown
   behavior stays explicit; no document interpreter, guessed command/folder, watcher
   or package patch. Any MCP-specific support is presented first.

Only the first two categories in the table (connection work and per-instance
records) plus presenting required access are inherent to wrapping a managed MCP.
Keychain, early timer, self-HTTP route, UI, permissions/queues and manual forms are
Fetch choices/requirements. The selected ones remain; they do not justify taking
over native server authentication. Library source/API evidence supports this plan,
but no implementation integration or runtime acceptance has been verified.

## Earlier research — a wrapper with the least Fetch-owned code

November 3, 2026. The user wants the full lifecycle explained and requests checking
what MCP servers already handle before assigning work to Fetch. Distinguish things
required to connect from conveniences/requirements selected for Fetch. No new
server-specific implementation, SDK dependency or testing is authorized by this
research request; coding remains paused.

Research division below; official library selected by the latest decision above:

| Step | Let existing server/helper do | Reuse a common library for | Fetch must connect or provide |
| --- | --- | --- | --- |
| Add | Original package and its supported setup | Existing package preparation where applicable | Remember command or URL, instance ID and selected launch info; do not copy old secrets. Remote service needs no local server install. |
| Start | Run its original entry point and read its normal state | Standard process/HTTP connection | Start/stop the owned local process or open the saved remote connection. Separate native locations only where server supports them. |
| First local sign-in | Native helper/browser/callback/token files/renewal | Its own existing auth library | User handoff and supported folder access. No universal local login command exists; do not invent one or patch its functions. |
| First remote sign-in | Hosted service owns account approval and its downstream service logins | Official Rust MCP OAuth library discovers login addresses, obtains/renews client access | Existing authenticate route calls library, opens browser for explicit user login, receives callback and supplies one Keychain store. |
| Remember login | Local native storage/refresh remains untouched | Common credential load/save/coordination interface for remote grants | Keychain integration because user selected it; no custom encrypted provider file format. |
| Normal tool use | Execute requested tool; local/hosted server owns any downstream provider access | Standard protocol client where it preserves selected behavior | Forward raw tool definitions/arguments/results with existing routing, caller permissions, queues and limits. |
| Renewal | Local server/helper renews its own provider login | Shared remote OAuth renewal/replacement code | Scheduler timing only; actual existing authenticate POST is the selected entry point. Route uses same shared library for all compatible remote services and saves replacements via common Keychain store. |
| Off and reopen | Local native account files survive shutdown | Library loads/renews remote grant on next use when permitted | Stop owned local processes on Quit, reload instance state on reopen. Expired short-lived access can be renewed after downtime; invalid renewal permission requires user sign-in. |
| File edits | Server reads its native config by its supported behavior | No Fetch config translation | Open actual known folder; explicit restart/reconnect when required. No file watcher. |
| Remove | Native account/session data remains | Standard disconnect | Close idle owned connection and remove registration, preserving native/legacy state, packages and grants under current safeguards. |

A local MCP that already handles Google access can own Google token renewal and
storage without Fetch reading/intercepting them. The online server can likewise
own its Google connection. Fetch still needs proof of permission to contact that
online MCP; it cannot delegate away presenting that proof. Shared library code can
implement the request, renewal and replacement details instead of Fetch doing so.

Manual API-key storage/delivery is conditional, not universally required: if the
server has its own supported setup, use that first. A server expecting a startup
key still needs a value supplied somehow. The retained credentials route could
provide generic Keychain/environment/header convenience; exact scope and save/
verification behavior remain under discussion. No assumed .env support.

Mandatory client work is establishing a connection, presenting required access and
forwarding messages. Native process control is needed for local servers Fetch owns.
Remembered login across app restarts requires persistent client-side state somewhere,
not necessarily a custom Fetch store. The user's selected Keychain location, early
renewal scheduling, literal self-HTTP authenticate call, UI/status, isolated account
locations, API authorization and queues are Fetch requirements or choices; they are
not separate provider requirements and do not justify per-server auth hooks.

## Selected code reduction — official auth library

Selected: use the official Rust MCP library's OAuth-only functions and its
credential-store interface behind the existing authenticate route. Keep Fetch's
raw-message runner. Replace custom remote discovery/login/refresh code instead of
just moving it into a shared file. One Keychain read/save implementation and one
browser/callback/status integration serve all standards-compatible remote services.
The user selected this dependency/integration direction. Source edits remain paused
until the remaining concrete plan details are reviewed.

The library's automatic refresh must not bypass the selected authenticate route.
Use its explicit auth/renewal functions at that route; keep request/refresh locking,
network destination/TLS/callback protections, no uncertain-call replay and native
raw-data behavior. Check supported integration/API/dependency versions before any
implementation; documentation/source research is not proof that this integration
works. Source manifest currently contains no rmcp dependency.

A universal mcp-remote proxy is an alternative, not recommended by this research:
it owns login/token files but adds a Node process, normally uses readable files
rather than selected Keychain storage, and has retry/message-transformation behavior
that needs separate raw-protocol evidence. Existing helpers explicitly chosen by a
server can still be used unmodified if they satisfy the agreed requirements.

Unknown native login commands, config folder locations, account separation and
nonstandard remote login remain concrete compatibility limits. Do not write special
Google/Linear/Railway login/refresh/storage patches or package-specific folder/no-auth
rules without first showing the user why one is necessary and the alternatives.
Actual authenticated service access is distinct from installation, tools-list success
or a browser completion page; do not label unknown downstream account access proven.

Primary research: [MCP client/server authorization roles](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization),
[official Rust OAuth design](https://github.com/modelcontextprotocol/rust-sdk/blob/main/docs/OAUTH_SUPPORT.md),
[versioned Rust auth/store implementation](https://raw.githubusercontent.com/modelcontextprotocol/rust-sdk/rmcp-v3.8.0/crates/rmcp/src/transport/auth.rs),
[mcp-remote lifecycle/storage](https://github.com/punkpeye/mcp-remote#readme).
Researcher compared maintenance, Apache-2.0/MIT licenses, dependencies and
storage/retry/message behavior. No packages were executed or credentials accessed.

## Latest decision — both callers use the authenticate route

November 3, 2026. User explicitly requires the Authenticate button and background
renewal to call POST /api/v1/servers/{server_id}/authenticate, because that route
already chooses how to handle the saved login. Both call the actual Fetch route;
do not silently substitute a background-only refresh function or another endpoint.
The route delegates to one shared implementation for all standards-compatible
remote services. It owns reuse of valid access, renewal, replacement access/refresh
values, Keychain save/readback, applicable authenticated checks and resulting status.
The scheduler owns timing only. Preserve per-instance request serialization so
concurrent callers do not rotate the same saved grant independently. No per-server
refresh implementation or package interception is selected.

Remaining route details to present before implementation: distinguish a request
that may open a browser from a scheduled renewal request. Recommendation: scheduled
requests only renew existing access and return a needs-sign-in result when renewal
cannot proceed; user-initiated requests may start browser login. Exact body/defaults
and pending/errors need review rather than silent introduction. The scheduled
self-request must preserve actual HTTPS validation and caller authorization; its
internal caller identity must be planned, not fabricated from a native window
session or a personal client token. No new public route is requested. Local server
native credential ownership remains unchanged. Coding/builds/tests stay paused.

## Latest follow-up — retain the credentials route

The user asks to keep PATCH /api/v1/servers/{server_id}/credentials and use Keychain
unless native servers can already handle the need. Retain the route in the plan;
its removal is superseded. Further application edits remain paused. The current
partial source still removes it and must not be mistaken for the settled contract.

Native server/helper OAuth can own its normal token files and refresh; that does
not mean every server reads arbitrary macOS Keychain entries. Saving a key is
separate from supplying it through the environment/header the server accepts.

Recommendation awaiting the user's scope decision: retain the route only for
manually supplied API keys/fixed tokens that lack native setup, storing directly
in Keychain per root/server/name with shared environment/header delivery using
imported bindings. Keep local native OAuth tokens and refresh inside the server;
remote HTTP OAuth uses Fetch's already-selected shared flow/Keychain storage,
without accepting manual refresh-token edits through this route. No package hooks,
new per-provider refresh handlers, legacy migration or plaintext fallback.

This recommendation would also retain a small native Settings API-key form and
requires reviewing the save result: Keychain storage success alone must not claim
that provider authentication works. Exact input/output, removal authority, delivery
and verification behavior are not yet accepted. Existing Settings-only authority,
no secret readback/API responses and per-instance separation remain the starting
boundary. No replacement credential engine is authorized by this follow-up alone.

## Earlier requested direction — amended by the follow-up above

Run unmodified MCP servers. Users edit the files and use the login/storage/refresh
process supported by each server or its existing helper. Replace Fetch's manual
server configuration and credential editors with Open server folder. Remove the
provider credential API/store, dedicated provider encryption keys in code, Google
interception/private bridge and package-specific credential redirection. Do not
create a substitute provider credential engine. Existing credential files and
keys must be left untouched; removing code does not authorize deleting data.

Preserve HTTPS/TLS, callback checks, caller permissions, Fetch's own app OAuth,
raw MCP definitions/arguments/results, queues and independent server progress.
Keep installation records and required launch information. No watchers, automatic
restarts, dependency patches, per-server adapters, personal client edits or
personal-credential tests are selected. Native storage is not necessarily encrypted.

## Inspected source inventory at 515c7b1

- API registers PATCH /api/v1/servers/{server_id}/credentials. There is no separate
  credential-save MCP tool; remove credential behavior from descriptions/guidance.
- PATCH /api/v1/servers/{server_id}/config and fetch__update_server_config expose
  launch arguments, working directory, environment/header credential references
  and Fetch call limits. There is no GET /config route.
- Settings and server cards share a credential editor; Settings also has a JSON
  configuration-patch editor and Fetch call limits.
- credentials is used both by provider files and by application OAuth/TLS. Removing
  the entire module would break Fetch's own authorization and HTTPS identity.
- manager owns remote-client OAuth grants/refresh and local Railway token injection.
  Google helper code intercepts its OAuth implementation through a private bridge;
  runner ManagedStdio attaches that bridge. These local redirections must go.
- Generic child startup clears inherited environment, sets an instance HOME and
  uses imported working_directory or that HOME. Imported secret values are discarded.
- Current deletion removes native HOME, legacy provider files/keys and packages;
  that must not remain an implicit way to delete protected state under this request.

## Initial approved contracts — further implementation paused for full review

1. Remove PATCH /credentials entirely, with no alias. Remove native secret-save
   capabilities, credential status lists, editor messages and store-write proof.
2. Keep PATCH /config and fetch__update_server_config for Fetch-owned execution
   only: max_concurrent_calls, default_timeout_ms, tool_timeouts_ms with current
   bounds, merge/remove semantics, management/tool permissions and no automatic
   restart. Reject args, working_directory, environment and connection edits.
   Internal saved launch/package records remain. GET /servers keeps safe launch
   metadata and truthful authentication/runtime state, without provider-store status.
3. Add POST /api/v1/servers/{server_id}/open-folder for native Fetch Settings only.
   It accepts an empty body, no query parameters and no caller-supplied path. A
   native Settings session identifies human UI authority; agent bearer credentials
   do not authorize desktop windows. No credential-save Mac confirmation is needed.
   The backend resolves the actual supported instance state folder and asynchronously
   opens it using /usr/bin/open (Finder on macOS); it never executes a shell or opens
   arbitrary user input. Return 200 with server_id, status:opened, next_steps:[]
   only after opener success; that means an open request succeeded, not visual proof.
   Errors: 400 INVALID_REQUEST, 403 FORBIDDEN, 404 SERVER_NOT_FOUND,
   409 SERVER_FOLDER_UNAVAILABLE, 503 SERVER_FOLDER_OPEN_FAILED, each with next_steps.
   An unknown location must fail clearly. No file content/credential read or edit
   operation and no MCP desktop-opening tool is added; agents hand the action to users.
4. Keep authenticate/restart endpoints and truthful pending/failure state. Standard
   HTTP OAuth uses the existing shared protocol flow. Local startup/authentication
   uses supported unmodified entry points only; helper command discovery cannot be
   guessed. If no shared supported invocation is available, report unsupported setup
   and the user handoff. Browser completion/initialize/tools-list alone cannot prove
   provider account access; use applicable authenticated evidence when available,
   otherwise retain unknown/unverified status. No arbitrary verification tool call.
5. DELETE /servers/{id} still requires management permission and an idle connection,
   but removes only its Fetch registration and stops its owned connection. Preserve
   native/legacy state, old keys and installed packages, because some servers keep
   credentials inside packages. No automatic cleanup, migration or revocation of old
   account credentials. A separate cleanup action is outside this refactor.

## Remote HTTP OAuth decision — Keychain selected

Fetch is the OAuth client of a remote MCP server. The remote service cannot keep
Fetch's client-side access and refresh tokens on Fetch's behalf. This is distinct
from a local MCP server keeping its own Google/Slack/provider credentials.

Recommendation: store only these Fetch-owned remote-client grants directly in
macOS Keychain, separately for each Fetch root and server instance, bound to the
remote resource, issuer and OAuth client ID. Store registration ID, access/refresh
values and expiry together; use the existing shared OAuth flow, no new provider
login engine or provider editors. Do not read/migrate old encrypted provider files
or keys; existing installations may require fresh explicit login. Require successful Keychain write and readback of each new or rotated grant
before reporting durable success; storage failure returns a clear
failure, never a silent plaintext fallback or authenticated claim. Require a successful authenticated request to the protected MCP endpoint before
claiming remote transport authentication (not downstream provider-account access); successful persistence alone is
not authentication proof. Tokens never appear in API/MCP output/logs. OS operations
run off the UI/async executor. Preserve existing refresh expiry, issuer/resource,
PKCE/state and callback protections. Removed registrations leave old state intact
under this request's no-deletion safeguard.

Alternative: keep remote-client tokens only in memory. This avoids new persistent
remote storage, but every Fetch restart requires remote sign-in again. Do not
silently choose this loss of remembered login. Fetch's existing application OAuth
and TLS protection remain in either option; this is not removal of all encryption.

## Honest native compatibility limits

An instance HOME (and supported separate config/state paths) helps only when the
server honors it. Servers may use package-relative files, an explicit working
folder, OS credential storage or remote state; do not assume their package or
HOME contains the actual configuration. Resolve and test supported native location
behavior before enabling folder access. Multiple locations must be described
truthfully rather than hiding one behind a guessed folder.

Environment-only API-key servers and custom-header remote entries currently depend
on discarded imported values plus Fetch references. Removing those references does
not make a server read .env automatically. Without its own supported setup, report
unsupported. No universal .env/header loader or new launch-file engine is proposed.
Google's normal helper requires its own client configuration; native account login
and storage cannot be inferred from removal of the interception code. Shared
startup is not universal server/account compatibility.

## Verification and acceptance

After agreement: plan, implement shared backend and API/UI surfaces, independently
test, evaluate, fix/retest and integrate. Use disposable state and the existing
verified deny-default macOS fixture confinement, with outside-file/network canaries,
for child-server checks. Use in-memory test substitutes for Keychain and native
window operations; these do not prove native Keychain/Finder or actual account
login. Native human confirmation/live provider tests require the protected native
session and dedicated credentials. Preserve everyday state, clients and services.

Check removed routes/fields/tools, actual folder resolution and denied authority,
separate native instances, untouched legacy files/keys, explicit restart behavior,
remote token rotation/failure/issuer ownership, Fetch OAuth/TLS, raw MCP forwarding,
queue/concurrency and responsiveness. Record actual test results separately from
source inspection and document unsupported native behaviors. No release work.

## Primary references checked by the Researcher

- [MCP HTTP authorization](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization): the MCP client receives and presents its access tokens; stdio login is outside this HTTP authorization flow.
- [Apple Keychain Services](https://developer.apple.com/documentation/security/keychain-services): native OS storage proposed for remote client grants, separate from server provider credentials.
- [Official Rust SDK OAuth design](https://github.com/modelcontextprotocol/rust-sdk/blob/main/docs/OAUTH_SUPPORT.md): shared client authentication with a storage boundary. No SDK replacement/dependency addition is proposed here.
- [Google 1.11.3 native authentication source](https://raw.githubusercontent.com/a-bonus/google-docs-mcp/v1.11.3/src/auth.ts) and [CLI entry](https://raw.githubusercontent.com/a-bonus/google-docs-mcp/v1.11.3/src/index.ts): native helper owns tokens; application configuration and token locations differ, and the browser page precedes completion.
- [mcp-remote native auth storage](https://github.com/punkpeye/mcp-remote/blob/main/src/lib/mcp-auth-config.ts): an existing shared helper with its own storage; reference only, not a selected universal proxy. Researcher checked MIT license/maintenance/dependencies/issues; the official Rust SDK is Apache-2.0. No upstream code is copied.


## Complete change plan — further source work paused

The user requests all route/major changes be planned before further edits. Tell the
user before writing code specific to an individual MCP package/provider, including
special refresh/token formats, login commands, callback exceptions, package patches,
folder rules, additional CLIs and package-specific no-auth assumptions. No such new
special cases are selected. The Keychain choice remains approved.

| Route | Planned change |
| --- | --- |
| PATCH /api/v1/servers/{server_id}/credentials | Retain in plan per latest user follow-up; proposed manual-key-only Keychain behavior awaits exact scope/contract decision. Earlier removal is superseded. |
| PATCH /api/v1/servers/{server_id}/config | Keep only Fetch execution settings; reject former launch/env/header fields. |
| POST /api/v1/servers/{server_id}/open-folder | Only new route; native Settings authority, empty request, actual supported folder only, bounded async Finder request and errors as specified above. |
| POST /api/v1/servers/{server_id}/authenticate | Keep URL; generic remote OAuth/Keychain checks, local server/helper owns native login; unsupported native invocation stays explicit. No manual Fetch credential-entry branch. |
| GET /api/v1/servers | Remove provider credential-store status and related guidance; keep safe installation/launch info, auth/runtime/execution/restart state. |
| POST /api/v1/servers/{server_id}/restart | Keep URL and accepted-operation/drain/force/capacity rules; launch unmodified local server without decrypting/injecting provider secrets; remote uses its own client grant. File changes require explicit action. |
| DELETE /api/v1/servers/{server_id} | Close idle owned connection and remove registration; preserve state, credentials, keys and packages. |
| GET /oauth/provider/callback | Keep path for shared remote OAuth only; retain state, PKCE, issuer/resource and one-use/deadline protections. Remove local provider-specific callback ownership/redirection. |
| GET /api/v1/setup/discovery | Keep request/import behavior; remove references to configuring Fetch provider storage from result guidance. Continue discarding source secret values. |
| POST /api/v1/setup/installations | Keep selected inputs, identity/reuse and progress/Connect behavior; internal records stop creating provider-store status. Review existing extra Railway CLI preparation separately before changing that package-specific behavior. |
| GET /api/v1/setup/installations/{installation_id} | Keep contract; align guidance with native setup and truthful status. |

Unchanged public operations: GET /api/v1/tools, POST
/api/v1/servers/{server_id}/tools/{tool_name}/call, /mcp methods and raw upstream
definitions/arguments/results, Fetch's /oauth/register, /oauth/authorize,
/oauth/token, /oauth/revoke and discovery metadata. Backend integration changes
must preserve these contracts, caller permissions and TLS.

MCP management tools reuse the same backend functions. Narrow
fetch__update_server_config to execution only; update authenticate/get/delete
and setup guidance. No new credential/folder mutation MCP tool. Native folder
opening remains a human UI operation.

Non-route work: remove provider store interfaces, provider key creation/deletion
paths, Google helper patch/private socket and ManagedStdio; preserve application
TLS/app OAuth encryption. Add only the approved direct Keychain remote-grant
boundary with async worker I/O, instance/root/issuer/resource/client ownership,
rotation write/readback, no fallback/migration and shared protocol refresh.
Local native refresh is entirely the server/helper's responsibility. No SDK swap
or extra proxy/runtime dependency is selected by this plan.

UI: remove Settings and inline credential forms, arbitrary config JSON patch
editor and provider-store status. Keep Fetch call limits and truthful pending/error
states. Reuse an async Open server folder control; never block other servers/UI.
Keep docs/UI coverage/shared-component guides aligned after implementation.

Outstanding concrete compatibility details to discuss before dependent code:

- There is no common local MCP login command. Do not guess auth/login arguments
  or add a Google helper adapter. Where native startup does not perform supported
  login, report the limitation and let the user run its existing setup.
- A private HOME/XDG path does not prove actual native storage or account isolation.
  Folder resolution needs established native path information; Google can have split
  app-config/token locations. No guessed package folder or new user-path format.
- Environment-only API keys/custom headers lose their Fetch reference source. No
  general .env/header loader or copying personal secrets is selected. Such entries
  require supported native setup or an explicit unsupported result.
- Existing Railway extra-CLI installation is already package-specific. Review its
  necessity/native login behavior before retaining, removing or replacing it; no new
  Railway refresh/token mapping is selected.

Verification after the plan is accepted: removed route/fields, native authority
and folder outcomes, legacy-state preservation including removal/reuse, separate
instances, remote refresh/rotation/rejection/readback failure, preserved app OAuth/
TLS/raw MCP/queues and responsiveness. Independent disposable sandbox canaries
must pass before child tests; fake Keychain/Finder evidence is labeled as such.
Actual account login/Finder/native approval requires the protected native session.
No tests/builds or runtime acceptance have run for the partial refactor.
