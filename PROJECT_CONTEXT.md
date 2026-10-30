# Fetch — current project context

D82 generic authentication is implemented: imported names resolve through one
input module, trusted data describes checks, and one engine builds/tests them
using encrypted readback. Remote OAuth selection remains independent. Final
17 new/110 combined fixtures and native build pass; independent evaluation
ACCEPTs the scoped change and delivery follows. Unknown services still need trusted descriptions and
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
