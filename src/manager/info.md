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
