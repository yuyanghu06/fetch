> **Current authorization — October28, 2026:** The user authorized starting implementation and pushing to `https://github.com/yuyanghu06/fetch.git`. Do not create a release. Main MCP connection tests are deferred to a later isolated virtual environment using the user’s server inventory. This supersedes earlier implementation/push pauses only; it does not authorize personal-client edits, certificate trust changes, credential migration or live MCP tests.

# Fetch current work

October28, 2026 after D73. Canonical work is [the single implementation checkpoint](../docs/architecture/build-readiness.md); this file tracks execution/status rather than another approval backlog. [The pre-D73 task record](mcp-mvp-architecture/to-do-before-d73.md) preserves all completed passes and old unfinished entries. Broad duplicate-review tasks are superseded by the grouped worklist, not cancelled product requirements.

## Locked and recorded

- [x] One saved Fetch connection/token per agent at `/mcp`, with separate user-completed provider login reused by tool calls.
- [x] Rust/backend architecture and ordinary queues/deadlines/restart remain selected.
- [x] Shared memory deferred from this first release; preserve future same-product direction.
- [x] MIT and drag-and-drop macOS DMG selected.
- [x] Replace the ten-group count and overlapping roadmap with one locked specification/build sequence/engineering worklist.
- [x] Withdraw unaccepted verification exception draft from active review without adopting it or reopening D25/D31.

## One remaining scope choice

- [ ] Include or defer a new custom GitHub CLI adapter. Recommendation is defer; importing an existing GitHub MCP is separate. If included, decide operations and auth ownership together once.

## Consolidated engineering and delivery work

- [ ] Resolve the complete engineering worklist together: compatible Rust/Cargo/dependencies; raw protocol/codecs/correlation; native admission/bounds; config/import/package prep; files/Keychain/native authority; provider ownership/verification/refresh; safe verification error edges; remote destinations; lifecycle/recovery; onboarding/selected controls; isolated test environment; source/license/DMG/signing/publication mechanics.
- [x] D74 authorizes starting implementation and pushing; no release. Live MCP tests remain deferred.
- [ ] Stage1: Rust foundation/local HTTPS/API authority/storage; verify real HTTP in approved isolation.
- [ ] Stage2: selected server management, preparation/auth/restart APIs and HTTP evidence.
- [ ] Stage3: three upstream transports, raw MCP/native sharing and scheduling with independent evidence.
- [ ] Stage4: native API-backed setup/Settings/client connection; no personal cleanup until verified and authorized.
- [ ] Stage5: isolated real-client/fresh-install acceptance, fixes/retests/evaluator acceptance, MIT file and DMG; release only when authorized.

## Consolidation verification

- [x] Inspect read-only Planner consolidation report and preserve previous context/tasks/history.
- [x] Finish consolidation checks — independent Evaluator ACCEPT;332 Markdown files/668 local links and anchors/31 JSON examples passed before journal append; status fixes and archive links verified, all pass evidence serialized.

Application implementation and live testing remain paused. Exact new API or meaningful behavior changes still need one complete review; routine details do not become individual approval loops. Prior future scope stays preserved: shared memory, PostgreSQL/retention, graph, multiple managed processes, native background Tasks/live content and unselected controls are not silently implemented.


## Pre-build MCP session audit

- [x] Inventory all available primary Fetch sessions, including archive, and inspect original MCP requests/approval context through the current thread.
- [x] Restore the missing D68 management-tool execution input and concrete already-selected acceptance checks in the single plan.
- [x] Add source traceability/later-resolution pointers without a new approval backlog or runtime action.
- [x] Independent Evaluator ACCEPT; all106 protected paragraphs across42 files and293 original journal entries unchanged (terminal-newline boundary documented). Documentation checks passed; source audit, correction, review and integration passes serialized.

## D74 first implementation increment

- [x] Research compatible Rust/TLS/parser dependencies and bound the first code pass.
- [x] Implement a shared Rust backend/developer command, protected read handlers, conservative discovery and private/encrypted local primitives.
- [x] Independent fixture/HTTP/HTTPS checks:37 tests, formatting and strict Clippy passed. Root binary CLI metadata/normal TLS/Ctrl-C smoke passed.
- [x] Final independent Evaluator ACCEPT; documentation/integrity checks passed.
- [ ] Commit and push reviewed first increment to origin, verify remote and record delivery.
- [ ] Continue full Stage1 native authorization/token/Keychain/lasting identity, then selected preparation/runner/interface work. Current foundation does not complete Stage1 or MCP sharing.
- [ ] Plan the later joint isolated MCP environment from the recorded real server inventory; no live managed-MCP tests now.
- [ ] Release/DMG publication: not authorized by D74; do not create a release.
