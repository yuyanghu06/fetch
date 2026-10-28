# Active D77 source implementation — no testing

- [x] Read selected scope and source, preserve earlier planning and establish exclusive ownership.
- [x] Durable OAuth/native authority/Keychain/lasting TLS identity source written; all verification deferred.
- [x] Private runtime/package preparation and install-progress source written; unverified.
- [x] Shared server-management source integrated; bounded source review accepts it, runtime verification deferred.
- [x] Raw runner/transports/queues/catalog/native MCP source written; all verification deferred.
- [x] Existing HTTP route integration and native API-backed Iced interface source written; source review corrections integrated; uncompiled/untested.
- [ ] App/DMG packaging source, MIT/notices and explicit testing handoff.
- [ ] Scoped commits/pushes as features are integrated.
- [ ] User-started testing/compilation and isolated acceptance: deliberately NOT RUN.

Earlier work and unfinished requirements follow; D77 overrides test execution in the earlier sequence.

---

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

- [x] D78 defers the custom GitHub CLI adapter; existing GitHub MCP imports remain in scope.

## Consolidated engineering and delivery work

- [ ] Resolve the complete engineering worklist together: compatible Rust/Cargo/dependencies; raw protocol/codecs/correlation; native admission/bounds; config/import/package prep; files/Keychain/native authority; provider ownership/verification/refresh; safe verification error edges; remote destinations; lifecycle/recovery; onboarding/selected controls; isolated test environment; source/license/DMG/signing/publication mechanics.
- [x] D74 authorizes starting implementation and pushing; no release. Live MCP tests remain deferred.
- [ ] Stage1: Rust foundation/local HTTPS/API authority/storage; verify real HTTP in approved isolation.
- [ ] Stage2: selected server management, preparation/auth/restart APIs and HTTP evidence.
- [ ] Stage3: three upstream transports, raw MCP/native sharing and scheduling with independent evidence.
- [ ] Stage4: native API-backed setup/Settings/client connection; no personal cleanup until verified and authorized.
- [ ] Stage5: isolated real-client/fresh-install acceptance, fixes/retests/evaluator acceptance, MIT file and DMG; release only when authorized.

## Historical consolidation verification before D74

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
- [x] Commit d62db7e pushed to origin/main; remote SHA matched. Delivery journal follows; no tag/release.
- [ ] Continue full Stage1 native authorization/token/Keychain/lasting identity, then selected preparation/runner/interface work. Current foundation does not complete Stage1 or MCP sharing.
- [ ] Finish planning the later joint MCP environment: D76 selects short behavior/install/auth checks and [the five-server roster](isolated-mcp-testing/servers.md) is curated; location/layout and actual access/auth setup remain proposed/pending. No live managed-MCP tests now.
- [ ] Release/DMG publication: not authorized by D74; do not create a release.

## Developer-port and status follow-up

- [x] Switch the developer default from `4141` to `53671`, keeping explicit overrides and port `0` tests.
- [x] Refresh current milestone/context/README/architecture pointers and label obsolete pre-D74 status as historical.
- [x] Port/docs verification: formatting,37 existing tests, CLI help and independent Evaluator ACCEPT.
- [x] Pushed port/status follow-up 0fa5bce to origin/main and verified the matching remote commit.

## Future project organizer — beyond the first release

- [x] Record the user's Git-like project-folder organizer idea, shared sessions across computers, common Claude Code/Codex context and shared MCP servers/setup in [the project notes](../NOTES.md#october-28-2026--future-project-organizer-for-agent-sessions).
- [ ] In a future version, design project session/context/MCP sharing and easy Mac mini-to-MacBook continuation; explore whether to build on Git. Deferred from the first release, with no mechanism selected.

## Virtual testing environment discussion

- [x] Compare dependency environments, containers and full macOS VMs using primary sources; record UTM/Parallels/Tart tradeoffs and isolation limits.
- [x] Estimate UTM guest/host RAM and storage needs; record the current Mac's read-only capacity check and distinguish estimates from measured runtime requirements.
- [x] Research separate Codex/Claude CLI profiles, actual MCP sandbox coverage and lighter process/test-user options; preserve the reported same-user Keychain gap.
- [x] Record D76's short standard MCP behavior/install/auth scope and location-first, server-selection-next sequence; retain broader MVP acceptance.
- [x] Define a concrete location/layout proposal outside Git and normal Fetch/client state: `/Users/yuyang/Library/Application Support/Fetch-Testing/`; checked absent, not created.
- [x] At the user's subsequent explicit request, curate and record the first five MCPs with source/version, transport, auth and expected harmless checks in [the roster](isolated-mcp-testing/servers.md) and milestone. Curation is complete; execution is not run.
- [x] Independently review the roster/current tracking; Evaluator ACCEPT after correcting the snapshot's stale server-selection checkpoint. This accepts documentation only.
- [ ] Confirm the location/layout and finish actual launch/access/auth prerequisites before preparing the curated servers.
- [ ] Finish the actual file/credential/app/service/network boundary and verification/reset/account/evidence plan for the selected short test scope before provisioning/live tests. The earlier VM-vendor-first task is superseded by the user's resource constraints, not cancelled isolation requirements. See [the testing milestone](isolated-mcp-testing/info.md).

## First MCP roster — all checks not run

Exact procedures/limits are in [the roster](isolated-mcp-testing/servers.md). These entries are test targets selected by Codex at the user's request, not successful integrations or installation permission.

- [ ] **Everything 2.0.1** (`@modelcontextprotocol/server-everything`, npm/Node): install/stdio, list/echo/sum and small structured/resource/prompt checks; separate loopback HTTP and legacy SSE runs. No provider auth; clean environment without secrets.
- [ ] **Filesystem 2.0.1** (`@modelcontextprotocol/server-filesystem`, npm/Node): install with disposable allowed-directory arguments; marker write/read plus refusal of a separate disposable canary. No provider auth.
- [ ] **Time 2026.10.19** (`mcp-server-time`, PyPI/uvx): Python install, current UTC, deterministic UTC-to-Tokyo conversion and invalid-timezone error. No provider auth.
- [ ] **Tavily 0.2.22** (`tavily-mcp`, npm/stdio; optional hosted HTTP): dedicated API key, stored-key authenticated `/usage`, one basic search, explicit invalid-key handling and restart reuse. Hosted option uses bearer-header references. Dedicated test credentials and Fetch implementation pending.
- [ ] **Linear hosted** (`https://mcp.linear.app/mcp/readonly`, HTTP/OAuth): dedicated test workspace, cancel/retry/approve consent, read one fixture issue and check reconnect/restart reuse. Record provider version/date; do not presume forced expiry/refresh. Dedicated account and Fetch implementation pending.
