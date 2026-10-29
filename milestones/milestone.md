# Fetch milestone snapshot

## Current delivery — Railway sign-in, inline authentication and request logs

The selected Railway fix is implemented and independently source-reviewed:
Authenticate starts Fetch-owned browser OAuth, uses D80's temporary numeric
127.0.0.1 callback and privately verifies the stored credential through /oauth/me.
Inline server cards show progress/results/errors and share a masked credential
editor with Settings. Old status reads and repeated actions are guarded.

Railway's required CLI5.71.0 is prepared in its own private installation folder;
old installations repair it on explicit Connect. HOME/PATH/token handoff stay
private. Official remote Railway root URLs can be imported without rewriting.
Background token refresh for an already-running local child exposes Restart
required; Fetch does not claim that process's environment changes live.

Every first-party incoming/outgoing HTTP call has safe stderr metadata logs,
including auth outcomes/error codes. No secret bodies/headers/queries/raw URLs.
HTTP completion is headers-ready, not terminal streamed-tool completion.
Install, restart and auth monitoring now run independently.

Final command: cargo build --locked --bin fetch --message-format short,
exit0 in4.66s (prior integrated build19.64s). Existing four Keychain deprecation
warnings and one unread protocol field warning remain. Tester and independent
Evaluator ACCEPT scoped source/build only. No tests/test-target builds, app
launch, provider/MCP/client requests, npm/CLI setup, personal state/credentials/
trust, packaging or release executed. Working runtime remains unproven.

Full MVP is still incomplete: [actual source gaps](../docs/testing/MVP_GAPS.md).
The user explicitly requires operational completeness; do not rename missing
selected features as scope deferrals. [Diagnostics guide](../docs/testing/DIAGNOSTICS.md).

- [Coder and actual build evidence](history/2026-10-28_21-52-54-894807-coder-railway-browser-flow-and-private-runtime-source.md)
- [Independent source verification](history/2026-10-28_21-52-54-895007-tester-railway-callback-and-diagnostics-source-acceptance.md)
- [Independent final acceptance](history/2026-10-28_21-52-54-895117-evaluator-accept-scoped-railway-authentication-and-logging.md)


## Latest documentation — official Figma project

On October 28, the user selected [Fetch App Mockup](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=0-1&t=rEUDUra6cAWf3pxc-1) as Fetch's official Figma project for
ongoing design work. Saved in AGENTS.md, PROJECT_CONTEXT.md, NOTES.md and
[docs/FIGMA.md](../docs/FIGMA.md). Application and testing status below are unchanged.
Documentation-only pass; no builds, tests, app launch or MCP connections.
[Pass record](history/2026-10-28_18-35-39-812156-orchestrator-official-figma-project.md).


## Latest change — bounded Install list and bulk selection

The discovery list now owns a bounded scroll area with a stable widget location
and ID; Install selected servers remains in a fixed footer with selected count.
Header/file picker/discovery/Select all remain above the list. Errors/notices and
progress details scroll within it. Select all picks supported definitions only
(including source-disabled supported entries as explicit choice) without changing
source flags, discovering or installing. Other pages retain previous scrolling.

Independent source verification and Evaluator ACCEPT. Locked native binary build
succeeded, exit0 in10.45seconds; existing warnings only. No new tests or native/UI
execution by agents. Long-list/minimum-window/scroll preservation still need
manual checks after relaunch. The prior user screenshot is evidence of the old
overflow, not verification of the fix. See [scope](install-list-controls/info.md).

Unsupported-entry diagnosis is recorded in the development guide; no importer
behavior was changed or server/network compatibility test performed.

- [Planner report](history/2026-10-28_18-22-18-439052-planner-bounded-install-list-plan.md)
- [Coder report](history/2026-10-28_18-22-18-439053-coder-bounded-install-list-and-select-all.md)
- [Tester report](history/2026-10-28_18-22-18-439054-tester-bounded-install-list-source-verification.md)
- [Evaluator report](history/2026-10-28_18-22-18-439055-evaluator-bounded-install-list-source-acceptance.md)
- [Orchestrator diagnosis/build/integration](history/2026-10-28_18-22-18-439056-orchestrator-install-list-integration.md)


## Latest change — native config-file picker

The Install MCPs screen now has Choose file… beside its editable path field.
It opens a native one-file sheet, includes hidden folders such as .codex, fills
an absolute path and preserves input on Cancel. Selection alone does not read
config content, discover or install. UI/callback lifecycle received independent
source-verification and Evaluator ACCEPT with no requested fixes.

`cargo build --locked --bin fetch --message-format short` succeeded, exit0 in
11.35seconds. No tests, app/dialog, login, install or MCP connections were run by
agents. Interactive checks remain pending after relaunch; prior development path
separation and all earlier unfinished runtime acceptance remain intact.
See [picker scope](config-file-picker/info.md) and the updated development guide.

- [Planner report](history/2026-10-28_17-31-23-020385-planner-config-picker-plan.md)
- [Coder report](history/2026-10-28_17-31-23-020386-coder-config-picker-source.md)
- [Tester report](history/2026-10-28_17-31-23-020387-tester-config-picker-source-verification.md)
- [Evaluator report](history/2026-10-28_17-31-23-020388-evaluator-config-picker-source-acceptance.md)
- [Orchestrator build/integration](history/2026-10-28_17-31-23-020389-orchestrator-config-picker-integration.md)


## Latest evidence — development binary compiles

`cargo build --locked --bin fetch --message-format short` succeeded with exit0
in9.74seconds after the development-state changes. The binary was not executed.
Four native Keychain deprecation warnings and one unread protocol-field warning
remain. The four new regression tests remain unexecuted. No native/MCP/client
login, package installation, personal config/trust change, packaging or release.
See [compilation record](history/2026-10-28_17-18-50-885130-orchestrator-development-binary-compilation.md). Older source-only pass descriptions
below are retained as chronology; current verification adds compilation only.

## Current milestone — portable development state

User goal: import supported normal global Codex definitions into development
Fetch, then connect a fresh test Codex client with Fetch as its only MCP. Main
Codex may connect to the same instance. One shared Fetch manages servers/provider
sessions; clients receive distinct grants.

Implemented native debug defaults: current-home-relative
`~/Library/Application Support/Fetch-Development/`, port53672. Release retains
`~/Library/Application Support/Fetch/`, port53671. Installer/runtimes/caches,
repository/credentials/application identity, persistent runner state and
Keychain names follow the selected root. Existing API contracts are unchanged.
Explicit paths/ports win. Null stdio cwd now defaults to private server home;
explicit imported directories and original scripts remain unchanged.

Source and documentation independently reviewed/accepted; four regression tests
added but not run. No new agent-run build/test/app/Keychain/client/MCP/install or
trust result. The user reported running the pre-change development build. No
old-state migration, global client edit, cleanup, commit/push or release.

Next: deliberately rebuild before using defaults; prepare source-backed
single-Fetch test profile, fresh credentials and normal TLS trust, then execute
selected checks when started. See [development guide](../docs/testing/DEVELOPMENT.md)
and [milestone scope](development-state/info.md). Prior history below is retained.

- [Planner report](history/2026-10-28_17-17-15-069401-planner-portable-development-state-plan.md)
- [Coder report](history/2026-10-28_17-17-15-069402-coder-separate-development-defaults.md)
- [Tester report](history/2026-10-28_17-17-15-069403-tester-portable-development-regression-source.md)
- [Evaluator report](history/2026-10-28_17-17-15-069404-evaluator-development-state-source-acceptance.md)
- [Orchestrator integration](history/2026-10-28_17-17-15-069405-orchestrator-integrate-development-state.md)


## October 28 follow-up — testing preparation

The user resumed testing planning, accepted the Xcode license after a reported
compiler failure, and reports running the development build. A successful native
launch has not been observed by agents. Read [the first-session plan](../docs/testing/FIRST_SESSION.md).
Planner and Tester inspected source independently; Evaluator accepted the bounded
plan. No agents ran builds, tests, services, apps or connections in this pass.
The five-server roster is still not run; actual isolation remains unprovisioned.
Previous compile-only evidence below is historical, not current runtime evidence.

- [Planner report](history/2026-10-28_15-14-39-691532-planner-first-session-plan.md)
- [Tester report](history/2026-10-28_15-14-39-691533-tester-first-session-test-inventory.md)
- [Evaluator report](history/2026-10-28_15-14-39-691534-evaluator-first-session-plan-acceptance.md)
- [Orchestrator report](history/2026-10-28_15-14-39-691535-orchestrator-first-session-planning-integration.md)


October 28, 2026. Active milestone: [MCP MVP compilation](mcp-mvp-compilation/info.md). D79 compile-only milestone is complete: the native binary builds and fixes/lockfile were pushed as70e0f78. **Tests, app launch and MCP connections remain prohibited.**

## Current state and boundaries

The selected source exists through f9e3dab, including native UI, shared APIs, manager, raw runner/gateway and protected authority. First compilation resolved Cargo.lock and exposed a gateway syntax error. After the syntax/import fixes, locked native binary compilation succeeded. Native classic-Keychain APIs emit deprecation warnings; storage behavior is unchanged. Module/data boundaries and selected APIs remain those in the architecture records.

No tests/test targets, binary/application/service launch, MCP/provider connections, managed server/runtime setup, Keychain/trust/credential actions, packaging or release. Compilation does not establish runtime/client/native acceptance.

## Verification and next action

First command: `cargo build --bin fetch --message-format short`, exit101 at src/mcp/mod.rs:151. Corrected build: `cargo build --locked --bin fetch --message-format short`, exit0 in12.61seconds, producing target/debug/fetch. Toolchain1.95.0; dependencies resolved under manifest Rust1.89 compatibility. Four native classic-Keychain deprecation warnings and one unread protocol-error field warning remain. Independent source/build-evidence review ACCEPT. Fixes and Cargo.lock are committed/pushed as70e0f785fcc560f1af234ea6c9c6a9aadb97fe90; local HEAD and origin/main agreed after push. Stop; do not run the executable. Every runtime/test step remains for the user's separate instruction.

## Current pass history

- [Plan compilation-only build fixes](history/2026-10-28_14-59-16-596517-planner-compile-only-plan.md)
- [Run first native binary compilation](history/2026-10-28_14-59-16-596619-orchestrator-first-binary-compilation.md)

- [Compiler fix and successful binary build](history/2026-10-28_15-00-36-928523-coder-binary-build-error-fix.md)

- [Independent compiler-fix acceptance](history/2026-10-28_15-02-42-900848-evaluator-accept-compile-fix-delivery.md)

- [Compile-fix delivery](history/2026-10-28_15-03-44-962679-orchestrator-compile-fix-delivery.md)

## Prior source and future acceptance

[Source milestone](mcp-mvp-implementation/info.md), [source handoff](../docs/testing/HANDOFF.md), [earlier snapshot](milestone-before-d77-source.md), [deferred tasks](to-do-before-d77-source.md) and [isolated testing](isolated-mcp-testing/info.md) retain prior records. D78 custom GitHub CLI deferral and all earlier scope deferrals remain. The five-server roster is still not run and the isolation boundary unprovisioned.
