## Native terminal environment — November 4

terminal_action reuses the normal transport builder for assigned settings, private HOME/XDG, helper PATH and saved working directory, then delegates shell/process IO to TerminalService. Reopening a live session reuses it. No login command is inferred. Deletion closes the owned terminal before unregistering; Quit awaits cleanup before Runner shutdown. Native Settings route authority is separate from ordinary management permission.

## November 4 service-output console

Local setup captures both streams and natural exit in the shared Runner attempt log. Remote auth uses that same private handle for discovery/guard/registration/reuse/callback/verification failures. No inferred commands or provider-specific flow. Only native Settings reads the log; persisted/public errors remain safe existing categories. Synthetic progress prose is removed; show actual service replies/instructions and genuine local failures.

## November 4 shared program lookup

Native transport prefixes the installer-recorded private executable folders to each server’s PATH, the list searched when it runs another program by name. Explicit configured PATH remains after those folders. Otherwise the executable parent is followed by Fetch’s inherited PATH, preserving normal command folders such as Homebrew; /usr/bin,/bin are the fallback only when inherited PATH is absent. Ordinary startup, recorded setup and the native Terminal share this construction. Existing records use installer-known layout without migration/reinstall; no login behavior changes.

# Managed servers

The manager owns installation progress, saved-instance lookup, explicit Connect /
Restart, authentication status and registration removal. HTTP and MCP interfaces
use these same functions. Each instance has its own operation lock; independent
servers can progress together. The runner keeps raw MCP forwarding, queues and
call deadlines.

## Native configuration and login

Native processes run their original executable and arguments. The shared launcher
sets distinct HOME, XDG_CONFIG_HOME and XDG_STATE_HOME locations per instance;
explicit imported working directories are preserved. This helps only servers
that honor those locations. Fetch does not claim that every server keeps its
account state there, and does not read, patch, watch or copy native OAuth tokens.

The native Authenticate operation runs only an exact internally recorded
`_installation.native_setup` descriptor: executable, args, working_directory and
state_directory. There is no package-name switch, guessed auth command or README
interpreter. Missing descriptors return a clear unsupported result. Current public
imports do not invent these descriptors, so existing packages can require their
normal manual setup. Helper completion alone leaves account authentication unknown.

Open server folder uses only a recorded actual state_directory. Unknown, missing
or shared locations fail clearly; the downloaded package and HOME are not guessed
as substitutes. Canonical equivalent state directories cannot silently serve two
instances. The native Settings route controls the Finder operation; agents cannot
open desktop windows with ordinary management authority.

## Remote HTTP OAuth

The official pinned Rust MCP library owns published discovery, registration,
PKCE/code exchange, token interpretation and refresh. `remote_oauth/sdk.rs` supplies
one shared Keychain store and `network.rs` supplies generic HTTPS/destination
protections. There are no Google, Railway or Linear credential adapters.

The Authenticate button, startup checks, early renewal and expired-access checks
use the same actual authenticate HTTP route. The background sender has a random
process-only credential accepted for remote quiet checks/renewal only; it cannot
open a browser, launch native setup, save keys or call tools. The scheduler only
chooses timing. The SDK performs renewal and saves replacement tokens; Fetch does
not run another token refresh implementation. SDK operations stay inside the
route, and the raw runner does not invoke automatic SDK refresh elsewhere.

Remote authentication requires a successful protected transport check in addition
to the Keychain save/readback. A public initialization, browser page or tool list
cannot prove protected access. This check does not prove every downstream provider
account works. Transient SDK renewal errors and rejected/missing login remain
distinct; no failed MCP action is replayed.

## Manually entered fixed keys

The native Settings-only credentials PATCH stores fixed user-entered keys directly
in Keychain and supplies only existing assigned environment/header references.
Fresh request-bound Mac confirmation is preserved. Values assigned to the launcher's
HOME/XDG locations are rejected. A stored response explicitly has verified:false;
no invented provider-specific account check turns a successful save into account
access proof. Native OAuth tokens are never managed through this form.

GET server status reads saved non-secret records and cached runtime/authentication
state only. `access_expires_at` and `last_verified_at` are UTC timestamp strings or
null; GET does not unlock Keychain, probe providers or refresh tokens.

## Removal and verification limits

Deletion closes an idle owned connection and removes only Fetch's registration.
It preserves native files, packages, remote grants, legacy encrypted files and old
keys. No migration, decryption, token revocation or cleanup is implied.

Unit/integration fixtures use disposable records, fake Keychain/network/verification
connections and confined child processes. These are separate from real native
Finder, Keychain, browser/provider and client acceptance. See the current milestone
for actual test results and remaining native compatibility limits.

## Standard start/login implementation

local_auth.rs owns explicit local login attempts. Existing restart accepts
interactive:true and shares this behavior with deprecated Authenticate. It runs
an explicitly recorded helper or the configured MCP command; no provider-specific
login command is inferred. Attempts use atomic idle admission, private bounded
URL observation, host-only GET state and explicit second-request browser consent.
Local JoinHandles are awaited on Quit so helper process groups finish cleanup.
Terminal authentication status/error is saved to managed config and returned as
last_known; current unknown on a new process is never promoted from saved success.
Missing assigned environment values report names and native Settings guidance.
The provider SDK flow and manual-key authority stay in their existing modules.

## Temporary remote sign-in callback

`remote_callback.rs` binds `127.0.0.1` on an available port only when an explicit
remote sign-in needs a new browser session. The bound address is passed to rmcp
before it creates the authorization URL. This small listener serves only the
existing `/oauth/provider/callback` path and reuses the API callback parser and
result page. It checks this listener's expected SDK state before Manager consumes
the flow; rmcp still owns sign-in state, issuer, proof-key and token validation.

Each ProviderFlow owns a close signal. A callback starts graceful shutdown so its
result page can finish; browser failure, expiry or Quit also closes the listener.
The listener task owns the existing ten-minute expiry wait, which is cancelled
when the callback starts. Manager tracks and awaits these tasks on Quit. Saved
login reuse and background renewal do not open a listener. Fetch's main HTTPS
API, local identity and certificates are unchanged. Source is implemented;
focused fixture and native verification results are tracked separately.

November4 sharedinputs no longer inferrequirements: absent assigned environment
and header values are omitted for allstartup paths; present values pass unchanged.
Earlier missinginputname/Settingspreflight description is superseded. Native-only
startupfailurediagnostics are now a separate in-memory reporting integration;
provider/MCP/status records never receive rawdiagnostics.

Only native Settings servers_for reads may add authentication.startup from
Runner's sanitized private buffer. Never add it to servers(), saved config, MCP
responses or next_steps. This reports an actual spawn, rather than imposing input
requirements. Native UI consumes lines/status without executing instructions.

November 4: process_environment.rs adapts the pinned Apache-2.0 Codex MCP environment builder. It inherits the upstream allowed host variables/certificate paths, applies assigned settings plus Fetch's protected HOME/XDG/helper PATH, then removes upstream internal token variables. Native setup and normal startup use the same transport. No provider credential file is opened. Source attribution and the full upstream license/NOTICE are in the repository root. Independent verification is recorded under milestones/codex-startup-environment/.
