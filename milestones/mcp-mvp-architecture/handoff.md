> October 27 continuation: D38 approves the revised [tool-listing contract](../../docs/architecture/api-contract.md#approved-installed-tool-listing-contract); current step 2 prepares installed-tool invocation. Older checkpoints below are historical. No application implementation is authorized.

> October 27 continuation: D36 approves progress read; current step 2 reviews the unapproved [installed-tool listing contract](../../docs/architecture/api-contract.md#approved-installed-tool-listing-contract). Earlier body checkpoints are historical; implementation remains paused.

# Fetch architecture handoff

Snapshot: October 26, 2026, America/New_York. The user asked to end this collaboration with enough written context for another agent to continue. This handoff records the discussion; it does not approve outstanding proposals or authorize application implementation. Later explicit user decisions and current canonical architecture records take precedence over this dated snapshot.

## October 27 continuation after this handoff

The user subsequently approved the remaining restart bundle with "Cool all good next" (D20). The user then reaffirmed reusing the GET and moved the server family to `/api/v1/servers` (D21); discovery/installation stay in `/api/v1/setup`. The user later confirmed shared `next_steps` (D22) and approved ordinary server information (D23); the general authentication contract was subsequently reviewed, with D24 actual auth/storage ownership and D25 stored-credential `2xx` service test required before credential-based success. D26 later selects per-server encrypted credential configuration files and D27 accepts reviewed general auth behavior. D28 later approves durable server data under `Fetch/servers/<server-id>/`, outside replaceable packages. D29 later clarifies that configuration PATCH must include individual-server settings; its earlier three-field draft is unapproved and under revision. Current step 1 is that revised contract; GitHub CLI tool scope/auth mechanisms and file/crypto details remain open. No implementation is authorized. The original checkpoint and URLs below are historical and superseded. See [the approved restart contract](../../docs/architecture/restart-contract.md), [the approved server-information contract](../../docs/architecture/api-contract.md#approved-complete-record-and-ordinary-get-rules) and [the current snapshot](../milestone.md). PostgreSQL/retention remain deferred; implementation is paused.

D30 subsequently approves the common three-field launch-config contract and defers D29 native-server settings normalization beyond MVP. The active next contract is Settings-only credentials PATCH; no implementation authorization follows.

D31 later adds verified credential-save 200/shared auth failure code and advances the current review to installation-progress GET. Concrete test integrations/error policies remain open; current records supersede this original snapshot.

D32 clarifies UI-only setup is the base path, driven by structured progress/server states and deterministic UI sequence, with next_steps for agents only. Exact onboarding/authority remains open; no full progress-contract approval or implementation follows.

D33 requires consistent app sequencing and agent guidance across all previously discussed/future routes, while UI remains driven by structured state. The cross-route table in the current API contract preserves pending approvals and explicit user handoffs.

The user subsequently requested continuing; current step 1 reviews the complete installation-submission POST proposal. The progress-GET exact refinements remain pending rather than silently approved. No implementation authorization follows.

D34 subsequently approves the presented installation-submission/source-capture/reuse contract. Current step 1 reviews discovery GET after fresh official-doc research; new discovery policies, separate progress refinements and implementation mechanisms remain pending. No personal configs were inspected or code authorized.

D35 subsequently approves the discovery contract. Current final setup review is installation-progress GET’s remaining read projection/errors/guidance/current-run lookup, with D34 installer behavior already agreed. Finalize it before step 2 tool sharing; no implementation authorization follows.

## Latest scope amendment after this handoff

The user continued on October 26 to sideline retention and exclude all PostgreSQL functionality from the initial MVP. Read [storage-scope.md](../../docs/architecture/storage-scope.md). No PostgreSQL installation/connection, database-host Tailscale setup, schema/migration or PostgreSQL-backed retained job results are MVP prerequisites. The user subsequently chose local files for server configs and the existing encrypted store for Fetch-managed tokens/credentials; config files hold references, never token values. Encoding/paths/layout and key handling remain open; no substitute database is selected. The audit found no application source or actual runtime DB dependency. This supersedes the earlier database prerequisite statements; it does not approve other pending restart policies or authorize code.

## Start here: exact checkpoint

We are on **step 1 of 7: finish API contracts**, discussing restart. Steps 2–7 have not been completed. Eight setup method/path pairs have been selected; their full operational contracts are not all finalized.

The user answered **"Good"** to the restart request/result format. They then asked for **each complete API contract in one turn**, rather than several turns for pieces of the same contract. A single consolidated remaining restart proposal was presented. The user has **not approved that remaining bundle**; their next message requested this handoff.

Read [restart-contract.md](../../docs/architecture/restart-contract.md). Its first section is approved. Its remaining rules—including **60-second call draining, 5-second stopping, 30-second readiness, one active restart per server, permission/error rules, the filtered-GET envelope and recovery policy—are proposals only**. Do not silently use those numbers or policies as requirements. The former 24-hour PostgreSQL retention/recovery proposal is now deferred entirely, not waiting for approval as an MVP requirement.

The next agent should obtain feedback on that whole remaining contract in one review, without reopening approved fields or asking a new question for each clause. After addressing feedback, move to the next complete setup contract within step 1. No order among the remaining individual setup contracts has been explicitly selected. Follow the broader 1–7 order below.

**Application implementation remains paused.** Do not create source/scaffolding, restore the removed October 25 draft, install dependencies/runtime packages, start services, migrate credentials, edit personal harness configurations or deploy. Discuss each route/major architecture choice with the user and obtain explicit implementation authorization before building. Documentation updates are authorized.

## Reading map and sources of truth

| Record | Purpose |
| --- | --- |
| [AGENTS.md](../../AGENTS.md) | Required project instructions, API-first rule, review preference, implementation pause and milestone workflow. |
| [PROJECT_CONTEXT.md](../../PROJECT_CONTEXT.md) | Product direction, earlier memory/database choices and concise current context. |
| [NOTES.md](../../NOTES.md) | Dated user intent and history. Older snapshots may be superseded. |
| [Architecture index](../../docs/architecture/README.md) | Entry point for the architecture folder. |
| [decisions.md](../../docs/architecture/decisions.md) | Confirmed decisions D01–D19, including PostgreSQL/retention deferral and the selected local-file/encrypted-token split. |
| [api-contract.md](../../docs/architecture/api-contract.md) | Selected setup routes, approved shapes and remaining gaps. |
| [storage-scope.md](../../docs/architecture/storage-scope.md) | Latest no-PostgreSQL MVP decision and saved-state dependency audit. |
| [restart-contract.md](../../docs/architecture/restart-contract.md) | Approved restart fields plus the single unapproved remaining contract proposal. |
| [open-questions.md](../../docs/architecture/open-questions.md) | Current unresolved choices; do not promote them to decisions. |
| [build-readiness.md](../../docs/architecture/build-readiness.md) | Complete planned surface inventory, seven review groups and proposed acceptance demonstration. |
| [research.md](../../docs/architecture/research.md) | Primary sources, open-source comparisons and limits of what was verified. |
| [roadmap.md](../../docs/architecture/roadmap.md) | Proposed implementation → testing → release sequence. |
| [Milestone snapshot](../milestone.md), [to-do](../to-do.md), [history guide](../history/info.md) | Rolling actual state, unfinished work and immutable pass records. |

These files collectively retain the design; a future agent should not require this chat to continue. Architecture records remain canonical, while the milestone tracks progress and evidence.

## User preferences and working rules

- October 27 feedback preference: explain concrete disagreements respectfully; recommend cleaner/simpler/more secure alternatives with their tradeoffs, leaving decisions with the user.
- Use concise, plain English. Explain what a route reads/changes, its inputs/results and why a choice matters. Explain technical terms with concrete examples.
- The user decides every API route and major architecture choice, including grouping/batching tool calls. Do not infer approval from silence, a partial reply or a documentation request.
- Follow steps 1–7 in order. **One complete API contract per turn**, covering method/path, input/result, permissions, saved/runtime effects, errors, progress/readiness and meaningful concurrency/deadline/retry rules together. Ask for feedback on that contract as a whole; do not split one contract into repeated approval turns.
- Preserve accepted choices. The earlier slow step-1A/step-1B approach was superseded by the whole-contract preference. "Good" approved the shown restart wire format, not later unshown defaults.
- APIs own permissions, validation, process control, concurrency and persistence. UI, MCP and scripts wrap the same public application APIs; no direct database or independent behavior path in an interface.
- Keep docs current in the same turn as new decisions. Preserve proposals as proposals; record superseded ideas clearly. Do not expand the MVP through hypothetical features.
- Research unfamiliar implementation mechanisms first, including maintained open-source comparisons with clear licenses/dependencies/issues. Check primary sources and selected versions; research is not proof of working Fetch integration.
- Use the shared roles/workflow at `~/.codex/AGENT_WORKFLOW.md` and `~/.codex/agents/`; do not duplicate role definitions in the repo. The primary agent is Orchestrator, with at most three specialists, no specialist spawning, exclusive writer ownership and independent checks when coding is authorized.
- The Orchestrator serializes milestone updates after each meaningful pass: inspect evidence; refresh snapshot/to-do/affected folder guides; append immutable history using [the exact template](../../docs/templates/history-entry.md), America/New_York time and collision-safe microseconds. Preserve unfinished tasks. Never rewrite previous history entries.
- No completion notification was requested. Do not send a text/reminder. Do not create/message another thread merely because a handoff is requested.

## Full planned discussion outline

| Step | Work and expected deliverable | Status at handoff |
| --- | --- | --- |
| **1. API contracts** | Complete the eight setup contracts: exact input/result examples, permissions, state changes, errors, progress and readiness. Preserve shared `status`/`next_steps`. Keep pending clauses distinct from approved ones. | **Active. Restart fields approved; remaining complete restart proposal awaits feedback. Other setup contracts still have gaps.** |
| **2. Tool sharing** | Choose application APIs for listing/calling installed tools, Fetch MCP endpoint/transport/protocol versions, built-in setup-tool mappings, duplicate-name handling and schema/result preservation. | Open; no tool-list/call HTTP routes or MCP URL selected. Imported MCP tools must remain individually exposed. |
| **3. Execution behavior** | Decide simultaneous calls, shared server/session state, limits, waiting/rejection, cancellation, timeouts and any batching. | Open. Restart draining/rejection does not settle ordinary concurrency, worker counts or extra wrapper instances. |
| **4. Authentication** | Design harness-to-Fetch OAuth separately from upstream login; client registration/scopes/renewal/access removal, supported provider integrations, Settings authority and encrypted-store key protection. | High-level direction selected; mechanisms and endpoint inventory open. |
| **5. Implementation structure** | Review Cargo/modules/libraries, supported import/launcher forms, runtime provisioning, stable IDs, non-PostgreSQL local-state/job ownership and remaining lifecycle/failure handling. | Rust/process/package direction selected; full source tree, dependencies, schema and cleanup details unselected. |
| **6. Onboarding** | Establish first-client connection before setup tools are reachable; connect all three harnesses, API-backed screens, migration checks/cleanup prompt and remaining management scope. Clarify memory/website release sequencing where relevant. | High-level install/auth/cleanup UX selected; exact connection/bootstrap/control routes open. |
| **7. Testing and delivery** | Agree real HTTP/MCP/harness tests, fresh macOS install, prerequisites, signing/notarization, explicit updates, license and packaging/release destination. | Roadmap and test scenarios are proposals; no application tests/build/release exist. |

This is a design discussion sequence, not implementation authorization. For each included capability, the finished outline must identify its API/protocol operation, allowed caller, exact examples, saved/runtime/credential effects and acceptance test, with approved/open/deferred status. Optional management features should be included or deferred explicitly rather than added to hit an endpoint count.

## Confirmed product and backend direction

- Fetch is **one product with shared memory and MCP management**. Current work focuses MCP sharing; do not split products or erase existing memory scope. Memory is one shared preference prompt with simple history/undo; graph/branching/merging are deferred. Memory APIs must precede their interfaces. Whether memory/website ships alongside this native release still needs clarification.
- Self-built **Rust backend**, IBM ContextForge as inspiration rather than a mandatory Python runtime. Iced is the earlier desktop framework to try, not a validated prototype; Electron was rejected. Next.js remains a proposed web-interface choice.
- **macOS first**, current user, client-facing API/MCP on **localhost**. One Fetch process contains UI/API/manager; MCP wrappers are child processes. Opening Fetch starts managed servers; background/red-close keeps them running; explicit Quit stops them. Startup ordering, failures, crash/process-group cleanup and in-progress job behavior need remaining decisions.
- Discover **global Claude Code, Codex and OpenCode configs**. Optional explicit config-file path is supported; exact overrides/default-path rules and non-global supplied-file boundary remain open. Grok bot and unidentified Muse come later.
- Discovered servers selected by default; per-server toggles/dropdowns and Install button. Catalogue deferred, per-tool import selection deferred. Prepare recognized npm/npx and Python/uvx entries from config metadata; run existing local scripts where they are. Do not clone entire configs/source or migrate secrets.
- Resolve explicit versions as requested; resolve omitted/latest once at install and pin. App launch does not update packages. Changes require explicit update/reinstall, whose full operation is still to be designed.
- Provision private tooling during Fetch installation. Share compatible runtimes but isolate each server's dependencies. Exact artifacts/checks/downloads/commands/cache cleanup remain open.
- Fresh authentication; encrypted local server-scoped store holds environment entries, API keys, custom tokens and OAuth/refresh/expiry data. Ordinary config stores references; each wrapper receives only its own assigned values. Secrets stay out of prompts, ordinary args, responses and logs. Runtime environment injection still supplies actual values to that process; it is not encryption or sandboxing.
- Manual credential/environment writes require backend-enforced **Settings authority**, not ordinary agent access. Trusted backend OAuth grant/refresh writes remain automatic. Exact store path/format, key protection, permission implementation and per-wrapper token handoff remain open.
- Each harness uses standard OAuth with renewal to access Fetch and retains its own Fetch credential. Provider credentials are separate. Local stdio scripts do not universally support controllable browser OAuth, token export/persistence or refresh; do not promise every custom server will.
- One Fetch MCP exposes imported tools individually plus built-in setup/management tools wrapping the same application APIs. Names, collisions, capabilities, sessions and concurrency remain open.
- **Initial MVP has no PostgreSQL functionality; retention is sidelined.** Server settings use local files; actual tokens/credentials remain in the encrypted store. File encoding/layout/safe writes and key handling remain open. Earlier PostgreSQL/Tailscale and shared-migration guidance is future scope only if database work returns. An upstream MCP talking to a database-backed service does not make PostgreSQL a Fetch dependency.
- MCP configuration history/rollback is outside the MVP; source Git remains required; database migrations are deferred from this MVP. Preserve the existing personal MCP gateway/configurations during development. Original harness entries are removed via a user copy/paste agent prompt after migration verification, not automatic Fetch deletion.

## API inventory and approval boundaries

| Selected method/path | Already established | Still incomplete |
| --- | --- | --- |
| `GET /api/v1/setup/discovery` | Optional `config_path`, otherwise global defaults; read only. Missing defaults skipped; explicit missing `404`; explicit malformed/unsupported `422`; bad default warns/continues without raw secrets. | Complete schemas, formats/paths, unsupported/ambiguous/secret-bearing entries, duplicates and remaining errors. |
| `POST /api/v1/setup/installations` | Selected `{config_path, server_name}` entries; backend resolves; `202` with group installation ID; auto-generates Fetch configs; proceed only after overall success. | Complete validation/states, repeated requests, source changes, preparation limits, metadata/persistence and concurrency. |
| `GET /api/v1/setup/installations/{installation_id}` | Overall/per-server progress and next steps; prepared does not imply authenticated. Continue other installs on failure, retain successes, overall failed if any fail; retry failed entries through installation. | Full states/errors, polling/current-run lifetime and retry details; long-term retention/recovery deferred. |
| `GET /api/v1/setup/servers` | Optional repeated `id`, no IDs means all; always `servers` array and `not_found_ids`. Auth progress reuses it. Restart uses explicit `id` plus `restart_id`. | Full normal-server fields/envelope/errors/validation; restart-filtered changes remain in pending bundle. |
| `POST /api/v1/setup/servers/{server_id}/authenticate` | No credentials input. Supported OAuth opens default/native browser, immediate `202`/`authenticating`; known alternatives return Settings/permission instructions, unknown integrations report missing information. | Concrete wrapper integrations, full schemas/errors/repeat behavior and persistent auth lifecycle. |
| `PATCH /api/v1/setup/servers/{server_id}/config` | Authorized agents/interface update non-secret launch settings/references. Installation already creates configs; no separate creation/read route. | Exact fields/merge/unset/validation/application behavior. |
| `PATCH /api/v1/setup/servers/{server_id}/credentials` | Manual Settings-only writes, encrypted server-scoped values; includes env/custom tokens; names/status only; backend OAuth writes automatic. | Full schema/unset/errors, key handling and actual Settings authority. |
| `POST /api/v1/setup/servers/{server_id}/restart` | Optional JSON boolean `force_restart` default false; immediate `202` with IDs/status/next steps; normal wait/reject, force immediate; monitor named attempt; approved result states below. | Entire remaining consolidated policy bundle still awaiting user feedback. |

Approved restart result states are `restarting` with phases `waiting_for_calls`, `stopping`, `starting`, `checking_readiness`; `succeeded` with `code: SERVICE_STARTED`; or `failed` with `error.code`/`error.message`. Settings loads throughout pending, shows success only for actual readiness, or stops loading to show failure. The accepted polling form is `GET /api/v1/setup/servers?id=github&restart_id=restart-42` (example IDs).

No separate auth-progress, restart-progress or config-creation/read routes were selected. Query filters are not extra routes. The old 15–20 count was an estimate, not a target. Tool-list/call APIs, Fetch MCP endpoint, OAuth routes, harness connection setup and explicit update/reinstall need design; do not invent paths silently. Other start/stop/removal/cancellation controls require scope decisions first.

## Source and package layout

No application source exists. This is the earlier **working source proposal**, not an approved full tree:

```text
src/
  main.rs
  ui/
  api/
    setup/          # This grouping is confirmed
  discovery/
  installation/
  runtime/
  mcp/
  credentials/
  storage/
```

The user proposed putting discovery/installation/runtime/credentials/storage under `src/app/`, immediately paused to ask about Rust conventions, and **never confirmed it**. Do not regroup automatically. One Fetch process does not select one Cargo package versus a workspace. HTTP/async/MCP/local-storage/crypto choices still need review. PostgreSQL libraries, schema/migrations and database-host setup are deferred. Local files and the encrypted token store are selected; implementation specifics remain open.

The **confirmed installed-package layout** is outside source:

```text
~/Library/Application Support/Fetch/packages/
  runtimes/
    node/<version>/       # Node/npm/npx
    python/<version>/
    uv/<version>/         # uv/uvx
  servers/
    <server-specific installation>/
```

Do not install downloads in `src/packages`; that earlier literal source-directory idea was corrected. Existing local scripts stay at their original paths. Directory IDs, compatible runtime choices, package-manager internals and integrity/download policy remain unselected.

## Build, verification and release plan

The detailed [roadmap](../../docs/architecture/roadmap.md) is a proposal: bound release → agree architecture/dependencies/contracts → obtain implementation authorization → backend/config/storage/credentials → discovery/package preparation → shared runtime/tool APIs → combined MCP → harness setup/cleanup → native API-backed interface → independent full-flow verification → macOS package/release when authorized.

Verify each API through actual HTTP before its corresponding interface. Use the shared team: plan/research first; owned implementation slices; independent Tester; Evaluator ACCEPT/REQUEST CHANGES; fix/retest/integrate with serialized journal updates. API/major choices still require user input, not specialist approval alone.

Proposed meaningful evidence includes three-harness config fixtures, no secret migration, private installs and pinned versions, real supported authentication/renewal, server-specific environment assignment, agent denial on manual credential writes, two harnesses sharing the intended process with correct responses under the chosen concurrency policy, all three clients, normal/force restarts matched by attempt ID, background/Quit/reopen, and successful operation after approved original-entry cleanup. A mocked OAuth exchange or passing build would not prove real interoperability.

Fresh macOS install without PostgreSQL prerequisites, installer format, signing/notarization, explicit updates/uninstall/data retention, source license and release destination remain to be decided. Catalogue, graph, Windows/Linux, Grok/Muse and optional hosted deployment are later scope. No deployment is authorized.

## Actual evidence and research limits

Completed work is documentation, primary-source/open-source research, a read-only Planner audit and independent documentation evaluations. Markdown/link/JSON/template/history checks passed as recorded in [milestone history](../milestone.md#pass-history). Evaluator ACCEPT means documentation consistency, **not** user acceptance of pending policies or application readiness.

No Fetch implementation, dependencies, migrations, managed MCP launch, live Fetch HTTP/MCP/OAuth/database test, desktop prototype, installer, commit/push or deployment was produced. Git was still `main` with project documentation/tool-state paths untracked at the handoff inspection; verify actual state before future edits and preserve other people's changes. Repository identity is recorded in PROJECT_CONTEXT.md.

Research references and caveats are in [research.md](../../docs/architecture/research.md): official harness OAuth docs/source; MCP protocol/tools/authorization/versioning; IBM ContextForge, Goose, ToolHive, mcpd and agentgateway comparisons; npm/uvx preparation; Rust/Iced lifecycle. Current protocol versions differ from older universal-initialize assumptions. A process spawn is not readiness; `tools/list` can be empty and auth-dependent. Listing does not execute a tool or prove every provider action is authorized. GitHub's documented local OAuth token lifecycle and arbitrary-wrapper persistence need concrete integration; encrypted Fetch storage alone does not solve handoff/refresh. Package reinstall alone does not guarantee fresh auth. Recheck time-sensitive source/library details when implementing.

## Handoff completion versus milestone completion

Final handoff validation and independent review passed 31 Markdown files, 9 JSON examples, local links/anchors/format/history checks with no material findings. The review/integration entries are linked from the milestone snapshot.

This handoff deliverable can be complete while the architecture milestone remains active/incomplete. The next agent should read the records, resume at the pending whole restart contract, retain all unfinished to-do items and obtain the user's decisions. Do not mark steps 1–7 complete, assume the latest handoff request approves the restart defaults, or start a one-shot build.
