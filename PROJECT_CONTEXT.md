# Fetch — current project context

Updated October 28, 2026 after D77/D78. The selected MCP MVP source is written through the testing handoff; **no testing or compilation has started**. Read [the handoff](docs/testing/HANDOFF.md), [selected requirements](docs/architecture/build-readiness.md), [decisions](docs/architecture/decisions.md) and [milestone](milestones/milestone.md).

## Current source

One Rust/macOS Fetch process owns the native Iced interface, localhost HTTPS API and MCP server manager. Source covers safe discovery/private pinned runtime and package preparation, durable local/remote configuration, Keychain-backed encrypted credentials, native Settings confirmation, account-free renewable app OAuth, the three selected upstream transports, queues/deadlines/restart and selected native tools/resources/prompts/relay behavior. UI wraps the same APIs and follows structured state.

Each coding app keeps its own Fetch connection/token at `/mcp`; provider login remains separate and is reused by normal calls. Raw provider Tool JSON, argument objects and results are preserved. Manual secret values remain native Settings-only. Exact source-stage supported import/authentication limits are documented in the handoff and folder guides; source implementation is not client compatibility evidence.

The native default is `https://localhost:53671`. The old developer launcher remains fixture-only/ephemeral with denied authority. Launching the native app later creates protected installation/Keychain state and starts saved connections. Nothing has been launched or provisioned in D77. Personal clients/configurations/credentials/trust/services remain untouched.

## Latest authorization

D77 authorizes full selected source implementation and scoped commits/pushes. The user expressly reserves control of **all testing**: no build/check/lint/format/validation/service/app/connection/package/packaging execution until they start it. Read-only source review is recorded separately. Cargo.lock refresh and first compilation are pending. Existing 37-test foundation evidence applies only to its prior commits. No release or DMG is created/authorized.

D78 explicitly defers new custom GitHub CLI tools. Importing an existing GitHub MCP remains separate selected scope. Shared memory, PostgreSQL/retention, graph, extra managed processes, native background Tasks/live resource subscriptions and future project-session sharing remain deferred. Preserve future same-product memory direction and the dated organizer note.

## Testing handoff

The curated short roster remains Everything, Filesystem, Time, Tavily and Linear, all not run. The proposed `~/Library/Application Support/Fetch-Testing/` layout and actual file/Keychain/app/service/network isolation/account boundary remain unprovisioned. A data folder or client profile alone is not sufficient isolation. The user will choose when to compile and start testing.

Repository: `/Users/yuyang/Documents/fetch`, branch `main`, origin `https://github.com/yuyanghu06/fetch.git`. Scoped feature commits are pushed as source is integrated. Use NOTES.md for dated general notes, docs/architecture for decisions and milestones/history for immutable actual pass evidence.
