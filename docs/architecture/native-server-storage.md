# Server-owned configuration and shared login

November 3, 2026. Scoped refactor implemented and independently accepted.
Final151 relevant checks, separate real confined no-account MCP check and locked
native binary build pass. This is not a full-MVP or universal support claim.
User's “you're good to start” resumes this scoped implementation/testing. No release,
signing, personal-client edits, existing-state migration or real provider credential
use is authorized. [Earlier planning](../../milestones/native-server-storage/planning-history-2026-11-03.md)
and decisions.md preserve superseded proposals.

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

## Implemented public contracts

| Route | Current behavior |
| --- | --- |
| PATCH /api/v1/servers/{id}/credentials | Retained per user's revision. Native Settings only, fresh request-bound Mac confirmation. Existing set:[{name,value}]/remove:[name] body, only assigned fixed-key references. Saves directly in Keychain, returns status:stored, verified:false, changed/affected names/restart_required/next_steps. It does not claim authenticated account access, read native tokens or expose secret values. |
| PATCH /api/v1/servers/{id}/config | Fetch execution settings only: concurrency, default timeout and per-tool overrides. Existing bounds/permissions/merge/remove semantics. Former args/cwd/env/header editing is rejected; no JSON config editor remains. |
| POST /api/v1/servers/{id}/authenticate | Existing route invokes one shared backend and official library for online login; uses exact recorded helper for native setup. Optional interactive boolean defaults true for explicit legacy requests. interactive:false cannot open a browser or run native setup; missing/rejected renewal asks for explicit sign-in. |
| POST /api/v1/servers/{id}/open-folder | Native Settings only; empty body/no query/no supplied path. Open actual recorded native state directory in Finder asynchronously. Unknown/missing location409 SERVER_FOLDER_UNAVAILABLE; opener failure503; ordinary malformed/forbidden/missing-server errors retained. No agent desktop-opening MCP tool. |
| GET /api/v1/servers | Existing visibility/filtering/response conventions. Authentication includes status/error and nullable RFC3339 access_expires_at/last_verified_at. Expired unrenewed authenticated status becomes unknown while retaining timestamps. No Keychain/token reads, provider request, renewal, native helper or browser action. Reads do not await another server's authentication lock. |
| POST /api/v1/servers/{id}/restart | Same normal/force/drain/admission/outcome rules. Native unmodified launch and saved fixed-key inputs. Online same-grant renewal updates headers; new login or changed manual process inputs require explicit reconnect/restart. |
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

Internal _installation.native_setup records contain an exact absolute executable,
args, working_directory and actual state_directory. One shared launcher runs that
command; no README interpreter, guessed login arguments or provider recipes.
It uses the same per-instance HOME/XDG locations as ordinary startup where honored,
checks duplicate canonical native state directories, bounds owned helper lifetime
and cleans owned processes. Folder opening uses this recorded location; package or
HOME is not assumed to be the actual configuration folder.

**Current ordinary imports have no approved command/path authoring field.** Existing
records lacking the descriptor return SERVER_SETUP_UNSUPPORTED or
SERVER_FOLDER_UNAVAILABLE. The runner is implemented; automatic native command/path
discovery and a UI/API for supplying missing descriptors are not. New public fields
or package-specific recipes need the user's input first. Servers ignoring assigned
locations or using machine-global OS credentials may prevent account separation;
no universal support is claimed.

Existing Railway extra-CLI package preparation is retained, not redesigned in this
pass. Its native launch/login compatibility needs separate concrete review; no new
Railway authentication adapter/token injection has been added. Remote OAuth remains
bounded by the official library and existing HTTPS/destination/issuer checks;
unsupported services return clear errors rather than protocol downgrades.
In particular, metadata must advertise issuer-bound callback validation and
discovered authorization support; services lacking these are unsupported.

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
