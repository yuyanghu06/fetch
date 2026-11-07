November6 user screenshot confirms actual Codex /mcp now shows Fetch connected (405tools). User wants to note possible context growth and explore choosing managed MCP servers first, then loading their tools as a tree. Proposal only; current context/token cost unmeasured and no catalog/API change approved. See NOTES.md and architecture research/open questions.

November6 final scoped connection result: actual dedicated fetch_codex client now initializes and loads405Fetch tools using saved OAuth access, without another password prompt or connection error. Fixed initial version negotiation;13focused tests and signed native build8.49s/strict verification pass. Existing open Codex needs restart to replace its old failed snapshot. No global/client configuration edits, model task or individual upstream tool calls. See milestones/mcp-version-negotiation/info.md.

November6: Dedicated Codex0.166.0log shows authenticated initialize rejected for2025-06-18offer. Fetch now negotiates its existing supported2025-11-25instead of rejecting initial offer; no extra operating version/client configuration change. New realistic flow fails before/passes after;13focused checks and signed build8.49s/strict verification pass. Await explicit Quit/relaunch and real client tool listing. See milestones/mcp-version-negotiation/info.md.

November 6: Connect apps now has compact shared copyable Claude/Codex command blocks; manual verification checkbox/cleanup prompt removed. Codex approval403 traced to no-referrer suppressing form Origin. Approval HTML now uses same-origin while redirect/token policy and exact origin/cookie/CSRF/Mac/PKCE checks remain. 38focused checks and final signed native build16.15s/strict verification pass; direct review accepts scope. Real user retry remains unobserved. Global client configurations untouched; no-sub-session instruction is session-only, not a permanent project preference. See milestones/oauth-approval-origin/info.md.

November 4: Python package environment change complete. uv owns new packaged-server environments/compatible Python selection; explicit requests and private per-installation state retained.32 focused checks, signed native build and independent ACCEPT. Cross-Mac guide: docs/testing/PYTHON_MCP_SETUP.md. Real Blender install/connection unverified; app not relaunched. Existing saved servers/local-script provisioning unchanged.

November 4: permanent Fetch-only Codex test client created outside checkout at ~/Library/Application Support/Fetch-Development/clients/codex-test, with reusable fetch_codex login/startup command. Configuration-only checks show exactly Fetch and apps/plugins off; no actual login/tool call. Old temporary client removed after exit. [Portable setup guide](docs/testing/CODEX_TEST_CLIENT.md).


November 4 user selection: Figma Remote must show unsupported with its app-approval reason in Fetch’s import/install screen; Figma Desktop and saved entries stay unchanged. Shared-classifier/UI change complete:19focused checks2new/native build5.38s/independentACCEPT. Native app not relaunched. See milestones/figma-remote-unsupported/info.md.

November 4 Figma diagnosis: dynamic registration returns 403 before browser sign-in. Source sends client_name Fetch; Figma staff document approved-name checks, and current metadata does not advertise public identity JSON support. Approval blocking is likely; callback rejection is unproven. Research only, no source/account changes. See docs/architecture/client-identity-adoption.md.

November4 recurring terminal-click selection fix: use adjusted pointer consistently inside a scrolled parent. One actual Scrollable event/render regression reproduces before and passes after;11terminalUI checks1new/native build6.90s/independent ACCEPT. Prior release/trackpad fixes preserved. Native reload observation pending; milestones/terminal-pointer-coordinates/evidence.md.

November 4 selected-folder correction: choosing `.codex` must read only its own `config.toml`, excluding neighboring Claude/OpenCode configs. Fix complete: 10 focused HTTP tests and native build pass; independent Evaluator ACCEPT. Omitted-path/home/single-file behavior stays unchanged. Native app not relaunched. See milestones/selected-discovery-scope/info.md.

November4 terminal selection/scroll fix complete: outside release ends drag; trackpad direction/fractional capture corrected. Original2failures reproduced;10focused actual-widget/render tests2new/native build8.30s and independent ACCEPT. Deliberate selection/current cursor unchanged. Native reload observation pending; see milestones/terminal-selection-scroll/evidence.md.

November4 normal command-search fix complete: shared startup/setup/Terminal preserves inherited PATH after private programs and executable parent. Explicit PATH/private account folders unchanged.23focused checks1new/native build8.40s and EvaluatorACCEPT. Requires explicit Quit/relaunch/fresh terminal; personal gh/login untested. See milestones/normal-program-path/evidence.md.

November 4: the public Fetch identity file is deployed at https://www.yuyang-hu.org/oauth-client.json and verified directly. Fetch uses rmcp with a temporary HTTP 127.0.0.1 callback; its main HTTPS API and certificate are unchanged. One new feature test, two existing regressions and the native build pass; independent review accepts the change. Real Vercel account sign-in remains untested. See milestones/client-identity/evidence.md.

November4: publicidentity https://www.yuyang-hu.org/oauth-client.json deployed/direct200 verified. Fetchuses rmcpmetadataURL andtemporaryHTTP127.0.0.1 callback forremoteinteractive login; mainHTTPS/certunchanged. Onefeaturetest+twoexistingregressions3/3/nativebuildPASS, finalEvaluatorACCEPT. RealVercelaccountlogin untested. See milestones/client-identity/evidence.md.

November4 GitHub inspection: existing dev entry imports a shared local launcher that already calls /opt/homebrew/bin/gh, gets its token and starts GitHub MCP2.0.2. No downloaded GitHub package/gh exists under its recorded package ID. Private account paths and terminal PATH remain separate concerns; no code/authentication run or change. See NOTES.md/research.md.

November4 GitHub note: user reports its install script does not bundle gh; default to the globally installed gh executable when needed. Recorded selection only, not implemented/verified here; existing account/credential boundaries remain. See NOTES.md and architecture decisions.

November4 Console/Terminal scoped feature complete: account system shell in exact MCP environment, retained pane/screen and persistent sanitized service logs. Blank embedded terminal drawing corrected; actual offscreen prompt/cursor/scroll/clip test passes.182 relevant checks19new, nativebuild and EvaluatorACCEPT. Rebuilt native appearance/real accounts unobserved; Quit then ./scripts/run-development.sh. See milestones/server-console-terminal/evidence.md.

November 4: six extra Fetch provider-login gates are removed; official MCP library owns discovery/callback validation.43 independent auth checks and final locked native build pass; Evaluator ACCEPTs this scoped removal. Existing network, consent and account/storage boundaries remain. No real Vercel sign-in claimed; separate console test wiring is outside this acceptance. See milestones/wrapper-authentication/evidence.md.

November4: user requests expanded Console/Terminal pane switch with system shell in the exact MCP environment and persistent console log. Feature and exact new terminal API/retention contract explicitly approved; implementation active under docs/architecture/server-terminal.md. Credential storage remains service-owned; no global-login copying/fallback selected.

November 4: expanded consoles are always present and keep current-run startup-to-auth history through collapse/window reopen. Fresh local request errors remain visible beside old logs until new output arrives. Independent 150 relevant tests, eight new checks, native build/whitespace and Evaluator ACCEPT. Authentication and credential storage are unchanged. See milestones/continuous-server-console/evidence.md.

November 4 service logs complete: actual startup output/instructions/MCPHTTPerrors/OAuthfailure stages now feed native-only console; synthetic progress/protocolerror replacement removed.184uniquechecks15new, final31UIrecheck/nativebuild/whitespacePASS; EvaluatorACCEPT. Thin-wrapper default saved in AGENTS/decisions/notes. Actual confinedRailway initializes then tools/list returns-32001notloggedin; Codexreuse report recorded, no copy selected. See milestones/service-startup-logs/evidence.md.

November 4 architecture default: build thin wrappers around existing code and outputs unless the user explicitly specifies otherwise. Console displays actual service logs/instructions/errors, no replacement protocol-error/progress summaries. This is recorded in AGENTS.md and docs/architecture/decisions.md.

Earlier November4 logging authorization is now completed by the scope above; no new authflow or actual-account login was selected.

November 4: private program folders in shared startup are implemented and accepted. Independent 60 checks (10 new), confined ordinary/helper execution and locked build pass. Existing Railway needs no reinstall; real login/relaunch untested. See milestones/private-program-paths/evidence.md.

Earlier November 4 Railway diagnosis found @railway/mcp-server 0.1.12 could not find its separately installed CLI. The shared startup repair above now addresses that lookup failure; real account login remains untested. See docs/architecture/research.md.

November 4 Google MCP check: actual fresh 1.11.3 npm install contains no app credentials. Its own README requires supplied app settings before login; existing definitions already provide them. No new project requirement is established. Temporary install cleaned; no MCP/login run. See docs/architecture/research.md.

# Fetch — current project context

November 4: missing environment/header values no longer block startup. Configured
local startup stderr and natural exit status now appear in the native console,
sanitized before retention and excluded from ordinary API/MCP/disk records.
Independent 181 checks, final 31 UI checks, locked build and review pass.
[Evidence](milestones/startup-diagnostics/evidence.md). The old Google error still
has no recoverable cause; retry after explicit Quit/signed relaunch to capture it.

November4 authentication details now use a single scrolling monospace console
for safe Fetch progress/errors, retaining host consent inside.28UI checks/final
locked build pass and independent EvaluatorACCEPT. No API/login/raw-output-feed
change or native visual/runtime acceptance. [Evidence](milestones/authentication-console/evidence.md).

November3 standardized Authenticate implemented: native button calls existing
/restart interactive:true; recorded helper or configured local startup, safe host
consent, saved last-known/current distinction, atomic idle guard and awaited Quit
cleanup. /authenticate is deprecated but retained.171independent checks pass,
final targeted32pass/build pass and Evaluator ACCEPT. [Evidence](milestones/standard-authenticate/evidence.md).
No real Google/native/client acceptance or launch; use signed launcher after Quit.

November3: user selects standardized Authenticate startup/link behavior. Existing
local command is fallback when no separate setup command is recorded; interactive
wait and human link consent are being implemented. Last-known status is separate
from current account proof. [Selected scope](milestones/standard-authenticate/info.md).
Earlier research waits below are superseded for this bounded implementation.

November3 research correction: Google1.11.3 can start its own login during normal
startup; Fetch discards the printed sign-in link. Missing native_setup blocks the
current explicit Authenticate action, but is not a universal initial-login
requirement. [Library/client research](docs/architecture/mcp-login-discovery-research.md)
is independently reviewed for core findings; recommendations await user selection,
and no live Google configuration/account was checked.

November3 UI cleanup: Install now opens empty and loads definitions after an
explicit path selection. The compact title hint replaces the long explanation;
the app tagline/accepted notice are removed and disabled Install is gray.22 UI
checks and native build pass; native/Figma appearance remains unobserved. Existing
backend folder rules remain; .claude/.opencode extension awaits user input.

November3: server-owned native login/files and official rmcp online login/renewal
are implemented in the scoped refactor. Both user and background requests use the
existing authenticate route. Manual fixed keys retain the Settings-only PATCH and
save directly in Keychain; config editing is execution settings only. Server GET
reports nullable expiry/check times without performing authentication. Removal
preserves native/legacy/package state. See [current contracts](docs/architecture/native-server-storage.md).

Final151 checks, locked native build and separate confined Everything2026.8.31
check pass. Independent review findings were fixed/retested; Evaluator ACCEPTs
the scoped refactor. Real Keychain, Finder, provider accounts and full coding-client acceptance
remain unrun. Ordinary imports cannot yet author recorded native setup/folder data.
No universal support, release, personal-state edits or credential migration claimed.
[Current work](milestones/milestone.md). Earlier dated evidence follows and does not
override the current ownership contract.


November3 development Keychain prompt fix: successful individual keys are cached
in zeroizing memory until Quit, with OS creation readback, failure exclusion and
shutdown cleanup. After the wrapper refactor this cache applies only to Fetch
application identity/authorization keys; old provider keys remain untouched and
new manual/online grants use direct Keychain. Signed dev launcher uses the user-created local identity
and fixed app identifier across builds. Actual signatures, native build and49Rust
checks plus4launcher fixtures pass; independent review ACCEPT. Use
./scripts/run-development.sh after Quit. Old items may need final approval; actual
native prompt counts are unmeasured. No encrypted-format migration or release work.
See milestones/keychain-prompts/evidence.md. AGENTS.md now requires a per-Mac
first-launch prerequisite/signing check with concrete missing-setup instructions.

# Earlier project context and dated evidence

November 3: the user requests automatic Claude/Codex config discovery without manually locating files. The focused change makes opening Install discover default sources automatically and accepts an optional home/.codex folder or single config file. Selected-server installation stays explicit. See [the scoped milestone](milestones/automatic-config-discovery/info.md); Final merged64 relevant independent fixtures (12 new) and native build pass; Evaluator ACCEPT. Newer installed-row/direct-install/deletion/authentication source is preserved. Actual native picker/window interaction and Figma refresh remain pending. The user clarified the base folder is `.codex` itself.

November3 active install extension: user approves command/url alternatives in the
existing POST body after full contract review, explicitly without UI. Reuse the
shared definition classifier/private installer and existing permission/progress/
Connect/credential rules. Backend source is implemented;142 unique checks and
native compilation pass, independent Tester/Evaluator ACCEPT. No real package
installation/provider/connection acceptance is claimed.

November3 current authentication slice: shared dispatcher implemented,99 fixture
checks and native compilation pass; independent Tester/Evaluator ACCEPT. No real
provider/helper/native/client acceptance is claimed. Dispatch is selected from
imported command/arguments/environment needs, using web OAuth by default for
remote HTTP servers and common API-credential/local-helper handlers. Return
selected-flow errors without silently switching login methods. User subsequently approved the shared helper credential-storage bridge with
Google1.11.3 as first adapter. The shared bridge/adapter are implemented;188
integrated fixture checks, native compilation and independent acceptance pass. Actual
account authentication remains unverified. Multiple instances of the same MCP must independently own
authentication methods, credentials and accounts (including different Google
accounts); no package-global account state. Current same-source installation
reuse is unchanged; the user accepts separate imported entries and does not
need a duplicate-instance control for now. See NOTES.md.

## November 3 — source organization clarified

The user wants HTTP route handlers in `src/api`, grouped into subfolders that
follow the URL path. Backend functions may remain in separate folders grouped
by responsibility. Current source does not fully follow that requirement:
`api/runtime.rs` combines several route families, while `oauth` and `mcp` also
own HTTP adapters. This clarification is recorded; no source reorganization has
been performed. See docs/architecture/README.md for the boundary.

October31 clarification: Google and Slack already have their own sign-in/setup
code. Fetch's missing work is using that setup while saving updated account
access in its encrypted files and checking saved access before success. Google's
current package writes an unencrypted token file; Slack accepts saved token/cookie
inputs but later renewal writes to its own store. No real connect/tool/restart
test has passed. Do not describe this as two login systems needing to be rebuilt.

D82 generic authentication is implemented: imported names resolve through one
input module, trusted data describes checks, and one engine builds/tests them
using encrypted readback. Remote OAuth selection remains independent. Final
17 new/110 combined fixtures and native build pass; independent evaluation
ACCEPTs the scoped change; feature delivery is pushed as df094cc (delivery-record
follow-up separate). Unknown services still need trusted descriptions and
compatible login ownership, and actual native/provider/MCP/client proof is unrun.

D82 (October 30): user approved generic imported credential inputs/common
encrypted delivery and one verifier using trusted request descriptions. Internal
refactor underway; D25/D31 remain and no public verifier schema is added. Prior
93-test/native-build evidence applies to the previous parallel-authentication
source only; this change needs its own tests/build/evaluation. Installed CLT tools
work when selected for the build process; no Mac setting changed.

October 30 scoped authentication fix: POST authenticate still reports verified
success only after checking the saved credential with the service. The UI now
tracks authentication/credential-change requests per server, and backend saves
check all supported assigned API-key references instead of requiring one entry.
Independent 25 new cases and93 combined fixtures pass; native binary builds. This
does not implement unknown provider adapters, storage-only deletion/non-auth
success or real native/provider/MCP/client acceptance. Protected native testing
setup and those remaining gaps are tracked in docs/testing/MVP_GAPS.md.

Latest October29 instruction (D81): finish/build/test the local MVP autonomously,
then install a real MCP through computer interaction and verify it with a separate
test Codex instance. Preserve everyday clients/personal state and actual isolation.
Notify the user only after it is ready for their own testing. No packaging,
signing, notarization or DMG/release work. Safe known imports and shared native/
browser rendering are implemented; generic remote OAuth source is added with
issuer binding. Native build and68 fixtures pass; scoped source review ACCEPT.
Full provider setup,2 credential-policy choices and actual UI/MCP proof remain.
A copied offline test Codex starts with0 MCPs; native test-user setup is at the
administrator Unlock prompt. No account/security changes or real MCP connection.
See [the current run](docs/testing/LOCAL_MVP_RUN.md).

October29 handoff: the full working tree is saved/pushed as a15eacc. There are
five known implementation areas remaining, followed by isolated runtime
acceptance/fixes and distribution preparation. [MVP_GAPS.md](docs/testing/MVP_GAPS.md)
is the current list with finish criteria. Source/build completion, Figma coverage
and actual working-product acceptance remain distinct; no percentage/ETA is justified.

Latest Figma expansion: 153 current UI states/specimens now cover native and
browser branches, all nine inline-auth phases, individual Settings outcomes,
widget states and OS-owned source contracts. The shared credential editor matches
current src/ui/authentication.rs and its Settings/inline reuse. See
[the complete UI coverage index](docs/UI_STATE_COVERAGE.md). These are source
fixtures with bounded visual review, not runtime acceptance or every data permutation.

The user reaffirmed a fully working selected MVP. Source audit found required
authentication/credential/import workflows incomplete; see [actual gaps](docs/testing/MVP_GAPS.md).
Inline auth UI and safe HTTP diagnostics are implemented in source; Railway
DCR/PKCE, D80 temporary numeric-loopback callback and private CLI preparation are
implemented. Final binary builds in4.66s; independent scoped source acceptance.
Local running processes need explicit Restart after token refresh.
No runtime/MCP/provider test result is claimed.

Latest Figma correction, October 28: the user rejected the styled proposals and
requested literal copies of the current UI for future iteration. The first three
Figma pages now hold 13 native and four browser copies, current shared components,
and an unmodified screenshot of the already-open empty Servers window. Use
[the current inventory](docs/FIGMA.md); earlier styled boards are archived.
Keep actual single-column/scrolling layouts and Iced controls. Native-font
rendering requires a calibrated Figma substitute, explicitly documented.

Confirmed October 28 UI rule: commonly reused elements must be shared components
in Figma and in code. Reuse the seven defined families and a common theme instead
of per-page copies. See [shared component definitions](docs/SHARED_UI_COMPONENTS.md).
The Figma components exist. The Settings/inline credential editor is shared in
code; remaining repeated custom UI helpers/theme still need extraction.

Initial October 28 Figma delivery (subsequently rejected as too styled): 17 editable screen/state
mockups for every current native page and browser connection page, including
populated sample data, empty/busy/error states, seven shared component families
and 62 native prototype links. Those proposals are archived; use the corrected
current-app copies linked above. App source and runtime acceptance are unchanged.

Official Figma project (user-selected October 28, 2026): [Fetch App Mockup](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=0-1&t=rEUDUra6cAWf3pxc-1).
Use this existing file for ongoing Fetch design work. See [Figma reference](docs/FIGMA.md).


Latest Install UI: discovery/progress has a bounded independent scroll area;
Install selected servers stays in a fixed footer with selected count. Top Select
all selects supported definitions only without editing source flags. Independent
source acceptance and native build succeeded (10.45seconds); interactive layout
checks remain unrun. Import rules were not broadened.

Latest UI change: Choose file… beside the Install MCPs config path opens the
standard macOS file picker, including hidden folders. Selection fills the path;
Cancel preserves it, and Discover servers stays explicit. Independent source
review accepted; locked native compilation succeeded in11.35seconds. Actual
dialog interaction has not been exercised by agents.

Current follow-up: development/final-build state separation is implemented and
source-reviewed and compiled successfully, with four unexecuted path tests. The user selected global Codex
import and a fresh test Codex client sharing the same development Fetch endpoint
with the main client. Paths resolve from the current home on either Mac. Read
[development setup](docs/testing/DEVELOPMENT.md); no state migration, client setup,
provider installation/login or agent-run test occurred. The binary-only build
`cargo build --locked --bin fetch --message-format short` succeeded in9.74seconds.

October 28 follow-up: testing planning has resumed. The user reports running the
development build after accepting the Xcode license. No successful native launch
or current test result has been observed by agents. See
[the first-session plan](docs/testing/FIRST_SESSION.md) and current milestone;
older execution-pause statements below describe the prior source/compile pass.


Updated October 28, 2026 after D77/D78. The selected MCP MVP source is written through the testing handoff; **D79 native binary compilation succeeded; tests, app launch and MCP connections remain prohibited**. Read [the handoff](docs/testing/HANDOFF.md), [selected requirements](docs/architecture/build-readiness.md), [decisions](docs/architecture/decisions.md) and [milestone](milestones/milestone.md).

## Current source

One Rust/macOS Fetch process owns the native Iced interface, localhost HTTPS API and MCP server manager. Source covers safe discovery/private pinned runtime and package preparation, durable local/remote configuration, Keychain-backed encrypted credentials, native Settings confirmation, account-free renewable app OAuth, the three selected upstream transports, queues/deadlines/restart and selected native tools/resources/prompts/relay behavior. UI wraps the same APIs and follows structured state.

Each coding app keeps its own Fetch connection/token at `/mcp`; provider login remains separate and is reused by normal calls. Raw provider Tool JSON, argument objects and results are preserved. Manual secret values remain native Settings-only. Exact source-stage supported import/authentication limits are documented in the handoff and folder guides; source implementation is not client compatibility evidence.

The native debug default is `https://localhost:53672` under `~/Library/Application Support/Fetch-Development/`; release uses `https://localhost:53671` under `~/Library/Application Support/Fetch/`. The old developer launcher remains fixture-only/ephemeral with denied authority. Launching the native app later creates protected installation/Keychain state and starts saved connections. Nothing has been launched or provisioned in D77. Personal clients/configurations/credentials/trust/services remain untouched.

## Latest authorization

D77 authorizes full selected source implementation and scoped commits/pushes. The user expressly reserves control of **all testing**: no build/check/lint/format/validation/service/app/connection/package/packaging execution until they start it. Read-only source review is recorded separately. D79 now authorizes dependency resolution/binary compilation and build-error fixes followed by commit/push. Cargo.lock is refreshed and the gateway match-guard syntax/imports are corrected and locked native binary compilation now succeeds. Existing 37-test foundation evidence applies only to its prior commits. No release or DMG is created/authorized.

D78 explicitly defers new custom GitHub CLI tools. Importing an existing GitHub MCP remains separate selected scope. Shared memory, PostgreSQL/retention, graph, extra managed processes, native background Tasks/live resource subscriptions and future project-session sharing remain deferred. Preserve future same-product memory direction and the dated organizer note.

## Testing handoff

The curated short roster remains Everything, Filesystem, Time, Tavily and Linear, all not run. The proposed `~/Library/Application Support/Fetch-Testing/` layout and actual file/Keychain/app/service/network isolation/account boundary remain unprovisioned. A data folder or client profile alone is not sufficient isolation. Compilation alone is now authorized and successful. The user still chooses when to launch or start testing.

Repository: this checkout (location may differ by Mac), branch `main`, origin `https://github.com/yuyanghu06/fetch.git`. Scoped feature commits are pushed as source is integrated. Use NOTES.md for dated general notes, docs/architecture for decisions and milestones/history for immutable actual pass evidence.

D79 compiler fixes and resolved Cargo.lock are committed/pushed as70e0f78; local HEAD and origin/main agreed. The compile-only milestone is complete. No tests, app/service launch or MCP connections have run.

November 4: the console now has a fixed top-right Copy button that uses the existing
clipboard action for all current-server log lines. Locked build and review pass;
native clipboard use has not been observed.

November 4 actual Google startup cause identified: original shared definitions
supply nonempty client ID/secret values, while import omits those values. Neither
original cached nor managed package has credentials.json. No values copied;
preserving application setup during import remains a proposed follow-up.

November 4: user approved adapting actual Codex MCP environment-building code. Preserve Fetch's private HOME/XDG/helper PATH overrides and service-owned credentials; no new public fields/dependencies or auth/store import. [Current scoped milestone](milestones/codex-startup-environment/info.md).

November4 Codex environment replacement complete: actual pinned helper/variable lists adapted with Fetch protected HOME/XDG/helper PATH overrides. Independent32checks/native build and Evaluator ACCEPT; full library regression awaits concurrent terminal test wiring. Service credential ownership and auth behavior unchanged. See milestones/codex-startup-environment/evidence.md.
