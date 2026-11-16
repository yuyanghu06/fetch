# Local MCP connector evidence — November 12, 2026

## Implemented

- `fetch connect --client <profile>`: MCP stdio to the running, saved-certificate
  verified HTTPS backend. Missing-profile startup now opens/focuses Fetch for
  approval automatically; startup grants no authority.
- `fetch connect setup --client <profile>`: explicit setup launches/focuses the
  app and requests the native popup. It does not edit coding-client configs.
- Connect apps retains an optional inline key beside Claude Code/Codex. Native popup selects requested
  permissions, requires fresh Mac confirmation, then claims/saves a separate
  connector profile. Claim ownership survives window backgrounding.
- Four request/list/decision/claim operations reuse existing OAuth families,
  refresh and revoke. Connector profiles use their own Keychain namespace and
  cross-process refresh locks; fixtures inject memory credentials only.

## Focused automated results

| Check | Result |
| --- | --- |
| `cargo test --locked --lib oauth::` | 30 passed. Filter includes existing remote_oauth checks, three new native connection flows and browser approval regression. |
| `cargo test --locked --lib connector::` | Final 8 passed. Includes actual disposable HTTPS/gateway, raw payload/error details, concurrent equal IDs/profiles, interactions/progress, notifications, immediate cancellation, EOF, saved-grant reconnect, mid-call backend restart/no replay, wrong certificate and refresh/profile isolation. |
| `cargo test --locked --lib connector_setup` | 5 passed. Actual-widget popup plus window close/reopen, late approved reply, denied confirmation and expiry/retry ownership. Tasks that would use real Keychain are not executed. |
| `cargo test --locked --bin fetch cli_tests` | 1 passed; command/help/setup parsing without starting the app. |
| Connect-page command and copy regressions | 2 passed via the compiled library test executable after an extra Cargo compilation hit disk pressure. No functional source changes intervened; subsequent signed build compiled current source. |
| Signed `connect --help` and `connect setup --help` | Passed; no app, credentials or MCP connection invoked. |
| `./scripts/run-development.sh --build-only` | Passed with selected Apple Development identity; `com.fetch.local.development` executable and strict signature verification. App not launched. |
| Scoped `git diff --check` | Passed. |

Total focused passing tests: 46. Existing macOS deprecated-API/dead-code warnings
remain; no unrelated cleanup was performed. First attempts caught and corrected
new fixture import/router/TLS unwrap/bounded-read lifetime errors. Fixture roots
were changed to private0700, retaining production storage checks. The immediate
cancel test was corrected to preserve existing uncertain-call accounting rather
than incorrectly requiring cancellation to prove upstream work had stopped.

The additional Connect regression Cargo attempt hit `No space left on device`
in Rust's incremental cache. The signed build subsequently succeeded; the
already compiled test executable passed both checks. Disk inspection showed
4.6GiB free afterward. No pre-existing caches, source, executable, user files or
services were deleted to resolve it.

## Independent review

Initial Evaluator REQUEST CHANGES identified immediate cancellation overtaking
registration and window closure losing claim ownership. Fixes add per-request
dispatch receipt ordering, preserve proofs while backgrounded, and use proof-only
pinned HTTPS to save an already-approved connection after its window closes.
The final Evaluator ACCEPTs scoped source/fixture work. It does not claim actual
client or production Keychain acceptance.

## Visual evidence

[connector-approval-popup.png](evidence/connector-approval-popup.png) was rendered
from actual native widgets with disposable request data at874×630 and inspected.
It shows scope selection, request code and Allow/Deny using shared components.
It is not a running Fetch/native Mac-confirmation screenshot. The screenshot was
encoded from raw renderer pixels using Python's standard PNG/zlib facilities;
Pillow was unavailable, and no image packages were installed.

## Real-client attempts and explicit limits

The opt-in `connector_clients` fixture executable uses the production bridge with
memory grants, copied public client executables and task-only configs. Before
clients ran, deny-default confinement proved own file/backend access, denied
outside read/write, denied another disposable listener, and denied actual
bootstrap lookup for securityd/securityd.xpc/SecurityServer. All owned process
groups and temporary roots were cleaned on completion/failure.

Final measured MCP request counts:

| Client | MCP initialize | tools/list | tools/call | Outcome |
| --- | --- | --- | --- | --- |
| Codex | 0 | 0 | 0 | App-server control initialization worked; MCP status failed because managed-preferences synchronization was denied. |
| Claude Code | 0 | 0 | 0 | An inert synthetic key marker and task-local temp folder were used, with no model turn. Startup attempted blocked security execution and waited for remote managed-settings loading. |

No successful actual-client MCP connection/call is claimed. The sandbox was not
weakened to access personal settings, credentials, services or public network.
The user explicitly chose **manual testing later** for real Claude/Codex and
Mac approval checks. Production Keychain storage, OS popup/focus and actual client
sign-in/use remain manual acceptance items. No personal client settings, trust,
credentials or existing services were changed and no native Fetch was launched.

The notification HTTP stream uses the token with which it opened. Its expiry,
revocation or backend restart can close the connector; the coding client must
reconnect, reusing its saved grant. Transparent uninterrupted stream renewal is
not claimed. Calls with uncertain outcomes are never automatically replayed.

## Manual handoff

1. Explicitly Quit the currently running Fetch, then reopen through
   `./scripts/run-development.sh` to use the signed build.
2. If not already configured, use the displayed Connect-apps stdio registration
   command once to configure that client's Fetch entry. Existing HTTP entries
   need deliberate user replacement; this task never edits them automatically.
3. Start/reconnect Fetch from the coding client. A profile with no saved grant
   automatically opens Fetch's popup. Choose permissions and complete the Mac
   confirmation. No separate setup/login command is needed. If the client stopped
   waiting, use its normal Reconnect action after approving.
4. Confirm the client lists/calls tools, reconnects and remains independent of a
   second connector. Record actual production outcomes before completing native
   acceptance. No release, packaging, project-scoping or migration was performed.

## Automatic startup follow-up — November 12

Normal missing-profile startup now opens Fetch's popup without a separate setup/login command. Native app claims/saves a durable pending proof after approval even if the connector was killed. Cold/warm activation carries only a nonsecret client label; all approval remains in the existing four routes. Explicit denial and late Mac denial clear only the matching proof, and window closure stays hidden.

54 focused checks pass:13 connector/profile/gateway tests,6 popup/state tests,30 OAuth tests,1 activation test,1 inline-key/dismissal regression,2 Connect command/copy regressions and1 CLI parse test. Tests inject memory keys/credentials and disposable servers; production Keychain tasks scheduled by UI fixtures are not executed. The first activation fixture used a default temporary directory mode; corrected the fixture to0700 while preserving production private-folder checks. Independent final source review ACCEPT. Final ./scripts/run-development.sh --build-only passes in18.16seconds with strict signature verification. Fetch was not relaunched.

Real clients/Mac approval remain explicitly deferred to manual testing. Client startup timeout may require normal Reconnect after approval. Direct HTTP browser OAuth still has its separate certificate trust issue; stdio/native approval does not claim to repair browser trust.

## November 12 — first manual Claude connector failure and walkthrough

User changed the project-local Claude Fetch registration to the bundled stdio command. Read-only inspection confirms a valid seven-argument command with one complete data-directory argument, claude-code profile and port53672. Existing Fetch started after the signed build, owns activation.sock and responds200 to certificate-verified public authorization metadata. Do not reinstall or alter client settings as a guessed fix.

Initial screenshot shows connecting; subsequent screenshot shows connected, tools fetch failed, protocol2026-07-28, capabilities none and connection closed. Real Claude tool/catalog acceptance therefore remains failed, despite54 prior isolated checks. User suspects slow loading of the full catalog. Source list_tools visits server catalogs sequentially, so latency is plausible but unmeasured; do not present it as established cause or change discovery/timeout behavior without evidence.

User manually clicked Reconnect while root performed read-only process inspection. Existing connector sample reached stdin reading and had an established HTTPS connection; no sampled Keychain or profile-lock wait was observed. This is consistent with leaving initial approval and entering forwarding, not proof of complete MCP/tool discovery. No additional client was launched by agent, no saved credentials were read/exported and no app/services were restarted. Task-owned raw stack reports were removed after recording this nonsecret summary.

UI explanation confirmed from source: the Connect-apps key itself creates a request with client_name claude-code, even if Claude is closed. After explicit permission/Mac approval, native UI claims and saves that profile. Later normal connector startup reuses it and skips the popup. Approval belongs to a saved installation/client profile, not a running process or verified application identity. Pending requests use existing four routes; direct browser HTTPS auth is unchanged. User asks for an exact step-by-step route/file explanation before further implementation; no application source changed in this diagnostic pass.

Next action: explain current implementation, then inspect the failing discovery/catalog exchange and elapsed time. Do not treat the manual failure as accepted completion or silently change permission/identity/catalog architecture.


## November12 — why tool loading can be slow

Source inspection confirms three avoidable costs in tools/list: gateway list_tools awaits each server catalog sequentially; Runner.catalog re-requests upstream tools/list rather than using cached_catalog; each exported tool calls Runner.alias, which rewrites and synchronizes the full aliases file and containing directory even for existing mappings. collect gives its response wait a ten-second overall deadline per server (preflight/start can add further wait), so multiple slow servers can accumulate. This establishes a latency-prone code path, not measured attribution of the user's live connection closure. No source/timeout/catalog/auth behavior changed. Next: measure per-server collection and alias persistence using disposable fixtures, then review a bounded repair. Live client failure remains unresolved.

[Source inspection record](../history/2026-11-12_15-34-40-718087-orchestrator-connector-catalog-latency-source-findings.md).
