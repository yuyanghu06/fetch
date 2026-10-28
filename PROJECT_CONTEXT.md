# Fetch — current project context

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
