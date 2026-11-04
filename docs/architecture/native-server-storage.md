## November 4 — requested HOME matching during command lookup

User asks for global commands to use normal HOME/Keychain and local installed CLIs to retain instance-local login. Research establishes that PATH itself selects executables and cannot change per-command environment; Fetch starts an unchanged shell, which launches descendants without a Fetch callback. Shared Node/Python runtimes can run private applications, and absolute helper paths bypass PATH wrappers. No location-only automatic switching is implemented. Existing private HOME and local package precedence remain.

A wrapper can explicitly choose an application environment and let helpers inherit it, with coverage limited to launches through that wrapper. This would require a new bounded design; no universal interception or custom shell is selected. Private instance Keychain creation/selection through Apple commands remains a source-supported but untested alternative with potential unlock prompts; no Keychain state was changed. See milestone normal-system-home for pending work.

November4 latest clarification: locally installed Railway CLI login must remain with its Fetch instance. The normal-HOME proposal below is withdrawn; private HOME was restored before runtime execution. Package/helper paths remain. No alternative Keychain storage policy is selected.

## November 4 proposal — normal HOME for Keychain compatibility

User reports gh auth login Keychain Not Found in Fetch terminal and requests a system-context fix. Apple Security directly uses HOME to locate user Keychain paths: [home preference](https://github.com/apple-oss-distributions/Security/blob/db15acbe6a7f257a859ad9a3bb86097bfe0679d9/OSX/libsecurity_keychain/lib/DLDBListCFPref.cpp#L615), [path construction](https://github.com/apple-oss-distributions/Security/blob/db15acbe6a7f257a859ad9a3bb86097bfe0679d9/OSX/libsecurity_keychain/lib/StorageManager.cpp#L2055). This provides a concrete failure mechanism; user-specific repair is not yet tested.

Proposed: inherit normal HOME via existing Codex builder for all local MCP/setup/terminal starts; retain private XDG directories/packages/helper paths/cwd and service-owned storage. No provider-specific code or Keychain reset. This replaces the earlier private-HOME boundary and awaits explicit choice after explaining shared service logins. gh uses Keychain service/username names independent of config folder, so private XDG config does not isolate Keychain tokens. [Research/scope](../../milestones/normal-system-home/info.md).

## November 4 selected default program lookup — inherited PATH

User approved preserving normal command-search folders after server-private programs. Shared native transport now applies the existing inherited-environment builder before composing PATH: private program folders first; explicitly assigned PATH remains the suffix when present; otherwise executable parent then Fetch's inherited PATH, with /usr/bin,/bin only when no inherited value exists. This supersedes the earlier system-only default below. Ordinary startup, recorded setup and the native terminal reuse this one builder. Private HOME/XDG/account folders and service-owned credentials are unchanged. No shell-profile execution or login command is added. Scoped verification passes23focused checks (1new), native build and independent review; see milestones/normal-program-path/evidence.md. Native reload/actual account checks remain unobserved.

# Server-owned configuration and shared login

November 3, 2026. Scoped refactor implemented and independently accepted.
Final151 relevant checks, separate real confined no-account MCP check and locked
native binary build pass. This is not a full-MVP or universal support claim.
User's “you're good to start” resumes this scoped implementation/testing. No release,
signing, personal-client edits, existing-state migration or real provider credential
use is authorized. [Earlier planning](../../milestones/native-server-storage/planning-history-2026-11-03.md)
and decisions.md preserve superseded proposals.

## November4 selected startup behavior

User explicitly requests just starting the configured server and reporting its
outcome, removing the missing-input check for all startups. Shared input resolution
now supplies saved assigned environment/header values when present and omits
missing ones. Names imported from a configuration are not Fetch-enforced required
fields. Server startup, native configuration fallback and remote HTTP responses
determine what is needed. No provider-specific optional/required list is inferred.

This applies to ordinary app startup, post-install Connect, Restart, explicit
interactive start/login and recorded helpers through the shared resolver. The
synthetic SERVER_INPUT_REQUIRED branch is removed. Missing values do not trigger
a Fetch Settings prerequisite or prevent an execution/connection attempt. Existing
authority, malformed-config/path/header validation, work safety, process ownership
and SDK grant behavior remain. No route/input schema changes, automatic source
value copying or native token migration are selected. Existing ready/failure
reporting remains; raw stderr/exact exit-code streaming is not added here.

## November4 startup error reporting follow-up

User's Google start now reaches execution but reports only generic connection
ended. Nativeconsole will receive bounded sanitized local startupstderr and
actual exitcode/signal, with no guessed providerfailurecause. Capture before
retention in ephemeralRunner state, clear nextattempt and freeze ready/failure.
Only nativeSettings GETservers exposes authentication.startup with generation,
lines,exit_code,signal; ordinaryManagement/MCP responses and savedconfig excludeit.
FullURLs/knownassignedvalues/credentialassignments are removed before storage.
No newroute, disklog, providercredentialread or localprerequisite is introduced.
This is startupfailureevidence, not an all-purpose rawtool/logstream.

## November 4 selected private program search folders

User approved the shared startup repair after Railway could not find its already installed CLI. Installer-owned executable folders are stored as relative paths in private `_installation.program_directories`. Shared native startup prefixes the instance’s contained folders to its configured PATH, or to the existing executable-parent/system default. Both ordinary startup and recorded setup use this construction. Existing rows without metadata use only the installer’s known layout in memory, with no migration, reinstall or status-read writes. Provider dependency knowledge stays in installation; native startup does not select a provider or login method. No public API fields/routes, broad inherited environment, recursive scan or credential changes. Implementation/test status belongs in the milestone.

## November 4 authorized service startup and rejection logs

User explicitly resumed code for the rejection/service-log feature. The existing native-only authentication.startup console field will include ordinary local/remote startup, explicit setup output and remote OAuth failure details, using one per-attempt memory capture. Service stderr/setup stdout and stderr, nonprotocol stdout as a violation, standard MCP log notifications, MCP error fields, unsuccessful HTTP response status/body and exact OAuth failed stage all pass through a common sanitizer before retention. Successful protocol/tool/token responses and request payloads are excluded. Hosted service internals cannot be captured unless returned by that service.

Retain the beginning of an attempt with1MiB4096line implementation limits and explicit truncation notices, replacing the silent32line tail. Only the existing native Settings projection reads details; ordinary public errors/results, saved records and MCP output remain unchanged. No disk log, new route, new authentication method or provider/account execution is selected. Tasks hold their own capture handle so old attempts cannot mutate a newer one; short verification connections borrow the capture without ending it. Source/test progress belongs in milestones/service-startup-logs/.

## November 4 continuous console follow-up

The user requests an always-present console in every expanded server and retained output from startup through authentication. The existing native-only memory field now represents bounded cumulative history for that server during the current app run. New attempts carry already sanitized lines and earlier observed exit facts, with fresh attempt IDs and secret-redaction state; old handles remain frozen. Display state is passive and does not invoke authentication. No new route or JSON field, log file, credential-store change or authentication-check change is introduced. Limits remain explicit. See milestones/continuous-server-console/ for actual verification.

## Ownership

- Unmodified local MCP servers and existing helpers own native account login,
  callbacks, credential files and provider refresh. Fetch does not intercept them.
- Online MCPs own their downstream service accounts. Fetch still presents its own
  client access to that MCP. Official rmcp3.8.0 auth code implements metadata,
  registration, browser-flow rules, token exchange/expiry/renewal/replacement.
- Fetch owns instance records, process/connection control, API/native UI authority,
  browser/callback presentation, one Keychain storage connection, permissions,
  queues/deadlines and raw MCP forwarding. SDK protocol models do not replace the
  raw runner; no universal proxy executable or provider-specific refresh code.

## November3 route steering — start owns explicit login

User requests the button call the start API and deprecate, not remove, the auth
route. The existing start/connect operation is POST /api/v1/servers/{id}/restart;
no duplicate /start endpoint is added. Its existing force_restart body gains
optional interactive:boolean default false. interactive:true selects the shared
server-owned login path described below, without forcing a running server to stop.
force_restart:true together with interactive:true is invalid. Pending login returns
202 authenticating with the existing server-status polling location; settled
outcomes return200. Ordinary restart defaults/drain/permissions remain unchanged.

Native Authenticate and Open sign-in controls both call /restart with interactive
true. Matching fetch__restart_server exposes the same option. The old authenticate
HTTP route/tool remain available for compatibility and restricted background
renewal; the HTTP route emits a dated Deprecation header and successor Link. No
removal date or Sunset is selected. Remote login still uses the existing SDK; this
changes the user-facing entry point, not native provider token ownership.

This later route choice supersedes the below plan's direct button/auth-route
wording. Backend login/status/privacy/wait behavior stays the same.

## November3 selected Authenticate follow-up — implemented

User selects the research-backed approach: local Authenticate uses a recorded
helper when supplied, otherwise its existing configured server command. Existing
POST /api/v1/servers/{id}/authenticate keeps its optional interactive boolean and
permissions; native interactive:false cannot launch setup/open a browser. Local
attempts return202 authenticating promptly and use existing GET polling, with an
interactive600second limit including initialize. Ordinary startup keeps its30second
limits. Ready instances are not restarted merely to discover login. Configured missing environment/header values are omitted; the server decides
requirements and Fetch reports the attempted startup/connection outcome.

GET authentication adds sign_in:null or {host,opened}, representing only a selected
HTTPS sign-in URL from that attempt. No raw URL/query or diagnostic text is exposed.
Native UI shows the host and an explicit Open sign-in action; another interactive
request on the same Authenticate route opens the pending validated URL once. The
first call never opens arbitrary diagnostic URLs. Pending links stay in memory
and expire with the attempt; no new browser endpoint or body field is introduced.
Existing remote SDK OAuth/browser handling is unchanged.

Terminal {status,error} is remembered in the managed config's existing authentication
field; never save authenticating, sign-in URLs or tokens there. GET exposes saved
last_known separately while a fresh current process starts with unknown until
evidence arrives. A ready server/helper exit still does not prove account login.
The inline auth panel shows setup/sign-in progress and truthful guidance instead
of unconditional folder/manual-key fallback; manual input remains native Settings.

Native URL elicitation requires a human-response/owning-context contract that is
not implemented by existing setup. This first startup/link implementation does not
advertise that unsupported capability. Existing supported remote OAuth/relay remain.
No provider-specific command inference, OAuth implementation, token interception,
config migration or new public setup-command authoring fields are selected here.

## Implemented public contracts

| Route | Current behavior |
| --- | --- |
| PATCH /api/v1/servers/{id}/credentials | Retained per user's revision. Native Settings only, fresh request-bound Mac confirmation. Existing set:[{name,value}]/remove:[name] body, only assigned fixed-key references. Saves directly in Keychain, returns status:stored, verified:false, changed/affected names/restart_required/next_steps. It does not claim authenticated account access, read native tokens or expose secret values. |
| PATCH /api/v1/servers/{id}/config | Fetch execution settings only: concurrency, default timeout and per-tool overrides. Existing bounds/permissions/merge/remove semantics. Former args/cwd/env/header editing is rejected; no JSON config editor remains. |
| POST /api/v1/servers/{id}/authenticate | Deprecated compatibility route, retained with dated Deprecation and successor Link. Existing optional interactive boolean/default true and restricted background renewal remain; use /restart interactive:true for new explicit login requests. |
| POST /api/v1/servers/{id}/open-folder | Native Settings only; empty body/no query/no supplied path. Open actual recorded native state directory in Finder asynchronously. Unknown/missing location409 SERVER_FOLDER_UNAVAILABLE; opener failure503; ordinary malformed/forbidden/missing-server errors retained. No agent desktop-opening MCP tool. |
| GET /api/v1/servers | Existing visibility/filtering/response conventions. Authentication includes current status/error, saved last_known:{status,error}, transient sign_in:null or {host,opened}, and nullable RFC3339 access_expires_at/last_verified_at. Expired unrenewed authenticated status becomes unknown while retaining timestamps. No Keychain/token reads, provider request, renewal, native helper or browser action. Reads do not await another server's authentication lock. |
| POST /api/v1/servers/{id}/restart | Canonical start/connect route. Optional interactive:true starts shared server-owned login, uses a recorded helper or configured local command and presents host consent. Default false keeps ordinary normal/force/drain/admission rules. Interactive plus force is rejected. |
| DELETE /api/v1/servers/{id} | Stop idle owned context and remove registration only. Native state, packages, legacy encrypted files/keys, remote grants and other instances survive. Busy/stop/storage failures retain truthful outcomes. |
| GET /oauth/provider/callback | Existing shared callback route, bound to the pending library session/state/issuer/deadline and single instance. No local Railway login interception/listener. |

Discovery/install/progress inputs/reuse remain. New records omit obsolete provider
credential-store status; internal launch/package records stay. Fetch's own
/oauth/register, /oauth/authorize, /oauth/token, /oauth/revoke, HTTPS identity,
caller permissions and raw tool/MCP data contracts remain. MCP config tool is
execution-only; authenticate tool accepts the same optional interactive setting.
Every application response retains next_steps; protocol envelopes stay native.

## One authenticate path, normal timing

UI user action and scheduled renewal call the actual authenticate HTTP route.
The application supplies a random per-run memory-only x-fetch-background-auth
credential, recognized only on that endpoint for interactive:false. It cannot
open a browser/helper, edit keys/settings, install servers or invoke tools. It is
invalid after Quit and never sent to MCPs. Self-requests verify only Fetch's own
certificate, use pinned localhost/loopback and no proxy; no system trust edits.

The scheduler sleeps until inside the pinned library's30-second early window
(current wakeup29seconds before known expiry), then calls the same route. The
library decides expiry/renewal; no custom rotating-token marker/state machine.
Before using a remote connection, a cheap cached-expiry check also enters that
route when due. Initial/reopened connections check saved new-format grants through
that route; auth-free remote connections need no invented OAuth setup.

Unknown expiry is null and gets no guessed timer. A service may reject access
early; actual failures are reported without repeating an uncertain tool action.
Network/storage/temporary service errors differ from definitive renewal rejection.
One instance's auth work is serialized; other instances and GET status stay usable.
GET is a reporting surface, not the scheduler or a live account-test operation.

## Keychain and actual evidence

One common direct-Keychain store holds independent SDK and manually supplied fixed
key sections per Fetch root/instance. Section updates preserve each other and
require write/readback before success. SDK grants bind endpoint, metadata/issuer
and client ID, with its refresh coordination. No plaintext fallback or legacy
provider file/key access, migration, decryption, deletion or provider-token bridge.
Existing application AES-GCM files/keys remain only for Fetch TLS identity and
coding-app authorizations; provider IDs are refused before any file/key lookup.

The same generic safe remote setup check compares rejection without private
headers and acceptance with them before claiming transport authentication. Public
setup alone returns unsupported proof; no arbitrary tool is called to test login.
This proves connection access, not downstream Google/Slack accounts or each header
individually. Native setup command success stays unknown unless applicable access
is actually proved; no browser/exit-code/tools-list proof substitution.

## Native setup and compatibility limits

November3 follow-up: [library/client research](mcp-login-discovery-research.md)
corrects the initial Google diagnosis. The recorded command is required by this
implementation's explicit Authenticate path, not by every server's initial login.
Google1.11.3 can start its own login at ordinary startup and prints a link to
stderr; Fetch currently discards that output. Supplying native_setup alone would
not make that interaction usable. No live Google instance was inspected or tested.

Internal _installation.native_setup records contain an exact absolute executable,
args, working_directory and actual state_directory. One shared launcher runs that
command; no README interpreter, guessed login arguments or provider recipes.
It uses the same per-instance HOME/XDG locations as ordinary startup where honored,
checks duplicate canonical native state directories, bounds owned helper lifetime
and cleans owned processes. Folder opening uses this recorded location; package or
HOME is not assumed to be the actual configuration folder.

**Current ordinary imports have no approved command/path authoring field.** Existing
records lacking the descriptor now use configured local startup for explicit login;
folder opening still returns SERVER_FOLDER_UNAVAILABLE. A malformed supplied setup
descriptor remains unsupported. Automatic native command/path discovery and a
UI/API for supplying missing descriptors are not implemented. New public fields
or package-specific recipes need the user's input first. Servers ignoring assigned
locations or using machine-global OS credentials may prevent account separation;
no universal support is claimed.

Existing Railway extra-CLI package preparation is retained, not redesigned in this
pass. Its native launch/login compatibility needs separate concrete review; no new
Railway authentication adapter/token injection has been added. Remote OAuth remains
bounded by the official library and existing HTTPS/destination/issuer checks;
unsupported services return clear errors rather than protocol downgrades.
November 4 wrapper amendment supersedes the earlier Fetch-imposed issuer-support
restriction: rmcp owns metadata discovery/fallback and callback issuer validation.
Fetch no longer requires advertised issuer-response support, mandatory issuer
metadata or discovered-only metadata, and does not prevalidate unused endpoints.
Actual OAuth requests/browser destinations still use existing network/consent
checks. Callback queries tolerate unrelated provider fields, while duplicate
state/code/error/iss remain invalid. Saved-login reuse checks instance endpoint,
saved issuer and client identity, not equality of the entire metadata document.
The saved format is unchanged; no migration or automatic credential deletion.
See [wrapper-authentication evidence](../../milestones/wrapper-authentication/info.md).

## Evidence and remaining acceptance

Final frozen-source suite passes151checks (lib84,backend24,HTTPS7,direct10,
paths4,imports7,isolated native2,MVP12,raw1), with the explicit-staging real target
ignored in that routine run. Separate original Everything2026.8.31 check passes1/1:
13tools, echo/restart/removal, canaries and complete cleanup. Total152 checks exercised.
Latest locked native binary build passes (0.47s incremental); initial native build
also passed25.89s. Independent Evaluator ACCEPTs the scoped refactor after fixes.
[Detailed evidence](../../milestones/native-server-storage/evidence.md).

Tests use fake Keychain/native-confirmation seams and disposable services. Native
MCP peers run in verified deny-default macOS confinement with outside-file,
sibling-account and network canaries. They prove separate native files, restart
reuse, raw payloads, responsiveness and state-preserving removal. Actual self-HTTPS
sender and SDK rotation, transient errors/readback failures, binding guards and
GET behavior are exercised. No personal credentials/settings/services are changed.

Actual provider consent/account reuse, native Finder/Keychain/Mac confirmation,
full coding-client compatibility and Figma/native visual acceptance remain unrun.
A build/fixture test is not proof of those behaviors. Independent acceptance applies only to the scoped code/refactor and evidence
above; those remaining product checks are not complete.

Primary references: [official Rust auth source](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.8.0/crates/rmcp/src/transport/auth.rs),
[MCP authorization](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization).


## November 4 selected public client identity and IP callback

Remote interactive login passes https://www.yuyang-hu.org/oauth-client.json through rmcp's existing metadata-URL option. Temporary callback is GET http://127.0.0.1:<allocated-port>/oauth/provider/callback, bound before creating the session. Inputs and resultHTML remain existingcode/state/error/optionaliss semantics with state-bound pendinginstance ownership; duplicatecriticalfields remain invalid and unrelatedproviderfields allowed. This callback-only listener closes oncompletion/failure/original10minexpiry/Quit. Saved grant reuse/renewal creates no listener. Register http://127.0.0.1/oauth/provider/callback in the publicdocument using the standardnative-loopback portexception.

Existing applicationHTTPSAPI/maincallbackcompatibilityroute, savedTLSidentity, credentials, permissions and humanbrowserconsent remain. No completeapplicationrouter or managementAPI is exposed overHTTP. Public JSON deployment on the user's canonicalwww personal site is authorized; laterdedicateddomain remains futurework. See milestones/client-identity/info.md for actual status/evidence.
