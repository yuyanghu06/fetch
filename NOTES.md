# Project notes

This is the default document for this project's notes. When the user asks to note something down, update this file unless they name another destination. Date entries and distinguish ideas to try from confirmed decisions and completed work.

Architecture decisions now have a user-selected destination: [docs/architecture/](docs/architecture/README.md). Keep general dated project history here and current architecture/API detail there. Historical entries below may describe superseded ideas.


## October 27, 2026 — Isolate new MCP connection tests

Status: confirmed testing requirement; no environment has been selected, created or run.

- Use virtual computers/virtual machines or suitably isolated test environments to test new MCP connections without interfering with the user's everyday files, credentials, personal harness/MCP configurations, running apps or services.
- Use disposable fixtures and dedicated test accounts/credentials. Dependency isolation alone, such as a Python virtual environment, does not isolate files, OS credentials or desktop apps; the test environment must protect whatever resources the MCP can reach.
- Choose the concrete environment in the testing plan. This note does not authorize provisioning, installs, personal credential use or live connection tests while implementation/testing remain paused.
- The rule is recorded in both the global and Fetch AGENTS.md. The global instructions also require initializing a project-scoped notes.md when missing; Fetch already uses this existing NOTES.md, so no duplicate is created.

## October 27, 2026 — Approve progress read; move to tool sharing

Status: D36 confirmed; setup contract pass advances to step 2, not implementation.

- The user replied "good" to the final complete progress-read contract. [D36](docs/architecture/decisions.md#d36--approve-progress-read-and-advance-to-tool-sharing) approves response/enums/errors/guidance/polling and status/result availability for the current Fetch run; no Quit/crash recovery promise or storage mechanism is selected.
- D34 installer/source/retry/basic Quit rules remain unchanged. Concrete verifier/error edge cases and reader/preparer/identity/deadline/cleanup/key mechanisms remain in their later review steps.
- Step 2 starts with tool listing/origin identity/caller visibility/freshness/metadata exposure. Dedicated read-only primary-spec/gateway research is complete before dependent planning. No invocation path, wire alias encoding, library or protocol support version is selected.
- No tool/server/package/credential behavior has been tested or personal configurations inspected; application code remains paused.

## October 27, 2026 — Approve discovery; finalize progress read contract

Status: D35 confirmed; discovery presented contract approved, no application implementation.

- The user replied "good next" to the full discovery contract. [D35](docs/architecture/decisions.md#d35--approve-the-discovery-contract) records default/custom-file scope, safe metadata/eligibility/selection, source-disabled opt-in, partial failures and safe read/error rules.
- Keep official source facts distinct from Fetch's chosen scan/fallback policy. Reader releases, exact supported launch grammar/path identity/read limits and other mechanisms remain later choices. No personal config/credential reads were performed.
- The remaining setup contract awaiting an explicit final decision is installation-progress GET’s exact response/request-error/guidance/current-run result-lifetime. Installer work/retry/source/Quit behavior is already approved via D34 and is not reopened. The agent read-error guidance now offers the same safe GET retry as the native UI, respecting D33.
- Finalize that read contract, then move to step 2 tool sharing. Credential verification integration details remain for auth/implementation review; the shared gate/code is unchanged.

## October 27, 2026 — Approve installation submission; review discovery next

Status: D34 confirmed; presented submission rules approved, implementation mechanisms still open.

- The user replied "yep good" to the full POST contract and source-capture/reuse rules. [D34](docs/architecture/decisions.md#d34--approve-installation-submission-and-sourcereuse-rules) records one active group/sequential items, immediate acceptance, bad-batch versus per-item failures, retained successes, retries/private preparation and request errors.
- New/failed entries use the definition captured when their turn starts; no earlier-discovery fingerprint is promised. Successful Fetch entries retain pinned software/settings/credentials rather than importing changed source definitions. Different harness entries are not silently merged by package.
- Concrete source identity/canonicalization, preparer support, finite deadlines/process cleanup, complete same-run result retention and the separately pending progress refinements are not selected by this approval. No code is authorized.
- Next route is discovery GET. Recheck current official harness config paths/formats/overrides/scopes without reading the user’s actual configs or credentials, then prepare one complete proposal.

## October 27, 2026 — Continue with installation submission

Status: complete installation-submission POST proposal prepared; new policies remain unapproved.

- The user said "keep going bro" after UI/guidance clarifications. Continue step 1 with [the installation-submission proposal](docs/architecture/api-contract.md#approved-installation-submission-contract). This is continuation, not inferred approval of every pending progress-GET detail or permission to implement.
- The proposal keeps the existing path/name request and async group ID, with malformed-batch rejection, one global active job/sequential items, per-item source/preparation failures, safe capture when each item starts, successful-record reuse and explicit retry/cleanup/Quit boundaries.
- No discovery snapshot token is added. New/failed entries use the current source definition when their preparation starts; existing successful entries retain pinned Fetch config/credentials rather than importing changed definitions. Source identity/canonicalization and finite preparer deadlines/cleanup remain implementation-review dependencies.
- UI-only setup and agent guidance follow the same structured outcomes/sequence. No source packages, services, personal configs or credentials were changed.

## October 27, 2026 — Match app sequencing and agent guidance across all flows

Status: D33 confirmed, retroactively across every existing route; current full progress contract remains pending.

- The user requires next_steps to match the app’s built-in sequence and vice versa, including all previous steps. Both follow the same reviewed prerequisites, allowed operations/order, completion, failure/retry behavior and user handoffs.
- [The cross-route table](docs/architecture/api-contract.md#workflow-alignment-across-all-existing-routes--d33) covers discovery, install submission/progress, server inspection, auth, config, credentials and restart/named monitoring. Existing route approval boundaries remain intact.
- This preserves D32: UI uses structured state and its deterministic sequence, not guidance text. Settings-versus-agent permissions remain distinct; the agent relays a user action where it cannot perform the UI’s manual-value step. No secret-valued agent input or authority bypass is selected.
- Future checks compare UI behavior and guidance for the same states, including failures and partial/busy work. Wording-only guidance edits must not change UI flow; workflow-rule changes update both. No new workflow engine/schema/route or application implementation is authorized.

## October 27, 2026 — Agent-free UI setup is the base path

Status: D32 confirmed; complete progress contract and exact onboarding branches remain pending.

- The user clarified that installation without an agent is Fetch’s base UI setup method. The UI uses progress status/results and deterministic post-install sequencing rather than `next_steps`; agent assistance is optional.
- `next_steps` remains in every application JSON response under D22 but is guidance for agents only. Even `actor: user` means a task an agent relays to the user; it does not define UI controls or workflow. UI presentation follows structured state/errors, not instruction/action strings.
- Backend APIs keep validation, permissions, prerequisites and process/persistence behavior. UI calls the same APIs and uses server IDs/auth/config/runtime/restart fields after installation. No automatic login/retry/restart or personal harness edits are selected.
- Add agent-free first-setup and guidance-independent UI behavior to future verification; exact Settings authority and UI sequencing are later design work. No application source or live UI test was produced.

## October 27, 2026 — Require verification before credential-save 200

Status: D31 test gate and shared failure outcome confirmed; base manual-credentials contract reviewed, concrete verification policies remain open.

- The user requires credentials PATCH success to test the endpoint with the newly stored credential. Authentication already has the D25 encrypted-store/readback/service-authenticated-2xx gate; both must now report `CREDENTIAL_VERIFICATION_FAILED` for a failed required test, including async auth status through the existing GET.
- Storage failures remain distinct. A test failure after a committed store must say the storage outcome truthfully; no values are returned. Testing a running server with an old credential or generic health/tools listing does not satisfy the gate.
- The user said to move on after this amendment. Exact endpoint/private handoff, failure HTTP mapping, deadline, batch/no-op/removal/non-auth handling and rejected-value retention remain integration/error-policy decisions before implementation. No unchecked-200 exemption or automatic restart/rollback is selected.
- Next whole route: existing installation-progress GET; no new route or implementation authorization.

## October 27, 2026 — Defer native-settings standardization; approve common config

Status: D30 confirmed; common launch-config PATCH approved, broader D29 scope deferred beyond MVP.

- The user said normalizing scattered native MCP settings should be a separate scope outside MVP, keeping the common fields already discussed, and "everything else looks good, feel free to move on."
- [D30](docs/architecture/decisions.md#d30--bound-mvp-config-to-common-launch-fields-and-approve-its-contract) approves the previously reviewed `args`, `working_directory` and environment name/reference contract. These are Fetch's shared launch fields, not a universal MCP settings standard. Credential values stay outside agent config inputs/results.
- Broader native config-file discovery/mapping/editing, per-server settings schemas and live-setting standardization are deferred, not implemented. Safe support for selected launcher arguments still needs its implementation design.
- Next full route review is Settings-only credential/environment-value PATCH. It must distinguish successfully saving credentials from D25 authenticated-service verification. No application implementation is authorized.

## October 27, 2026 — Individual MCP-server settings and secret references

Status: scope clarified in D29; configuration draft not approved and being revised after source research.

- The user asked why an API key would be sent to the configuration tool and raised agent exposure. The example's `EXAMPLE_API_KEY` is a variable name and `credential_ref` is an opaque saved-entry reference, not the value. The intended private path is Settings/trusted OAuth → encrypted per-server storage → trusted backend retrieval → assigned server's supported interface. Secret values do not belong in agent tool inputs or Fetch management outputs.
- The user clarified that the same configuration API must be how users and agents interact with each individual MCP server's own settings. The earlier three-field launch-only proposal is therefore incomplete; this is not approval of its exact schema/policies.
- [Primary-source research](docs/architecture/research.md#individual-server-settings-and-credential-references--october-27) confirms no universal settings-file/get-set API: filesystem uses launch directories, Playwright accepts flags/environment/optional JSON, and memory’s JSONL file is application data. Server-specific tools may exist. Safe mapping, saved versus live reads, types, apply/restart/auth effects and permissions still need a revised contract.
- The existing Settings-only manual environment/credential-write boundary is not silently waived. No raw arbitrary file access, credential migration or application implementation is authorized.

## October 27, 2026 — Approve durable server-data placement; continue API review

Status: directory split confirmed (D28); configuration PATCH is the next proposal, not approved or implemented.

- The user accepted the recommendation with "Sounds good, let’s do next steps." [D28](docs/architecture/decisions.md#d28--separate-durable-server-data-from-installed-packages) places durable server config/encrypted credentials under `~/Library/Application Support/Fetch/servers/<server-id>/`, outside downloaded packages. Replacing/reinstalling software must preserve that data.
- Exact filenames/encoding, ID mapping, safe writes, key protection and removal/uninstall retention remain open. No runtime directories or application source were created.
- Continue step 1 with [the complete configuration PATCH proposal](docs/architecture/api-contract.md#approved-common-launch-configuration-patch-contract), which remains unapproved. GitHub CLI operation/API scope belongs in tool-sharing review; its concrete fresh auth/renewal belongs in authentication review. D25/D26 remain required, with no CLI storage exception.

## October 27, 2026 — Per-server encrypted files and GitHub CLI proposal

Status: reviewed general auth behavior accepted and per-server encrypted credential files required; exact placement and GitHub CLI design await decisions.

- The user proposed preferring GitHub CLI, guiding its installation and exposing CLI-backed GitHub tools rather than providing explicit GitHub MCP authentication support. Research is checking feasibility/refresh/storage; exact tool scope, adapter, setup and auth mechanism are not approved by the proposal.
- The user then said "Everything else looks good" and required credentials in individual server-level configs in encrypted files. D26 records per-server encrypted credential configuration; D27 records acceptance of the previously reviewed general authentication behavior, preserving concrete integration/crypto decisions and the implementation pause.
- They asked whether credentials should live inside `packages/servers/<server-id>/`. [The current layout recommendation](docs/architecture/storage-scope.md#approved-server-data-placement--d28) places durable per-server config/credentials in a separate root `servers/<id>/` tree so package updates/reinstalls do not own their lifetime. This path remains proposed, not a chosen standard.
- Application Support is the macOS app-data convention; internal layout is Fetch’s choice. No actual runtime files, package installations, CLI login, credential migration or personal config edits were performed.

## October 27, 2026 — Actual auth/storage ownership and verified success

Status: actual integration coverage and stored-credential success gate confirmed; concrete mechanisms and previous auth deadline/overlap policies remain pending.

- The user required actual MCP-server auth and credential storage to be accounted for and asked whether provider tokens have a standard return path. D24 records that scope. Dedicated current-source research distinguishes HTTP OAuth access to MCP from stdio credential provision and server-owned provider login; there is no universal local MCP provider-token export.
- The user required a successful store AND a `2xx` test using the stored credential before auth success. They clarified that “lifeboat endpoint” meant the service's authenticated test endpoint. D25 records encrypted store, readback and that test, preserving GET's latest-known-state read behavior.
- [Research](docs/architecture/research.md#actual-mcp-auth-and-token-ownership--rechecked-october-27) compares current GitHub MCP/SDK implementations, licenses and lifecycle limits; GitHub local OAuth is memory-only, and SDK client storage hooks are not a remote token-export mechanism. No integration was run.
- [Auth integration requirements](docs/architecture/api-contract.md#proposed-provider-authentication-and-credential-storage-contract) and [the confirmed success gate](docs/architecture/api-contract.md#required-stored-credential-verification--d25) make ownership, encrypted-store compatibility, handoff, refresh/restart and test evidence explicit. Exact endpoint/method, recheck/cleanup/non-token verification and wrapper-owned storage arrangement remain decisions; no cache exception, new route or implementation is approved.

## October 27, 2026 — Standardize next steps and approve server information

Status: shared convention and ordinary GET approved; application implementation remains paused.

- The user confirmed `next_steps` as a shared return for current/future routes, then said "Kk good to me then you’re good to move on." D22 records the application API JSON convention: success/pending/error responses include the array, empty when nothing is required; actor/action/instructions remain the common shape. Protocol-envelope mapping remains for OAuth/MCP design.
- D23 records approval of the complete ordinary `GET /api/v1/servers` contract, after clarification that `restart_required` means saved settings still await application even if the process is ready. This does not approve write contracts or auth integration mechanisms.
- Continue step 1 with the full authentication-start contract using the same server GET for progress. No app code, runtime, credentials or personal harness settings changed.
- For scheduling, estimated 20–30 further review turns for the existing full seven-step plan, including six remaining current API contracts and later architecture/verification choices with revision allowance. This is an estimate, not a release deadline or endpoint-count target.

## October 27, 2026 — Move servers outside setup

Status: server-route placement changed as requested; GET field proposals and implementation remain unapproved.

- The user asked to move `servers` under the base versioned route so it no longer lives in setup. D21 uses the existing `/api/v1` prefix: `GET /api/v1/servers`, with existing authentication/config/credentials/restart children moved together. Discovery/installation remain under `/api/v1/setup`.
- This is relocation/reuse of the existing family, not new routes or aliases. Restart filters and D20 behavior remain intact. Exact Rust handler/module grouping is still a later decision.
- Updated canonical paths and progress instructions. Earlier dated notes, immutable history and the original handoff body retain historical URLs; the handoff continuation notice and current records take precedence. No application code exists to migrate.
- Independent Evaluator accepted the preceding ordinary-GET documentation proposal, checking 43 Markdown files, 11 JSON examples, 190 local links and 20 history entries before this path change. That accepts documentation, not pending fields or implementation.

## October 27, 2026 — Approve restart and reuse server information

Status: restart bundle approved; existing GET response additions proposed; implementation remains paused.

- After the complete remaining restart proposal and clarification of stopping/restarting, the user said "Cool all good next." Recorded D20 and updated the approved [restart contract](docs/architecture/restart-contract.md), including the 60/5/30-second limits, permissions/readiness/overlap/errors, captured settings and polling/disconnection/retry behavior. PostgreSQL/retention and cross-launch recovery stay deferred; local job lifetime remains open.
- Next, the user explicitly reaffirmed reusing `GET /api/v1/setup/servers`. The [ordinary-read proposal](docs/architecture/api-contract.md#approved-complete-record-and-ordinary-get-rules) is in that existing route's canonical section. It proposes safe config/credential-presence information, runtime/auth state and remaining input/result/error rules, not another endpoint. New fields are not approved by the route-reuse instruction.
- A read-only Planner recommended one full ordinary record, explicit unknown auth/state, no secret values or raw imported commands, ordered/deduplicated ID selection, and no side effects on GET. The Orchestrator kept the proposal bounded, omitted optional source/auth-method inventories, and serialized documentation updates. No application work or personal configurations changed.

## October 27, 2026 — Resume architecture discussion

Status: discussion resumed; remaining restart policies await feedback and application implementation remains paused.

- Read the handoff and current API/decision/storage records. Continue step 1 with the complete remaining [restart contract](docs/architecture/restart-contract.md), preserving approved request/result fields and the seven-step order.
- Preserve the latest storage split: local non-secret server files plus encrypted tokens/credentials. PostgreSQL and retention remain deferred; file layout, key protection and operation-state lifetime remain open.
- The request to continue does not approve deadlines, readiness, permissions, overlap, errors or the filtered response envelope. Present those together for the user's decision.
- Updated the resume checkpoint and milestone records only. No application, runtime, service, credential or personal harness changes.

## October 26, 2026 — Select local configuration files and encrypted token storage

Status: storage split selected by the user; exact implementation details and application code remain unapproved.

- The user chose local files for server configurations and the previously selected encrypted store for tokens. Save non-secret server/launch/path/package-version metadata and credential references in config files; keep actual Fetch-managed tokens, API keys and assigned environment values encrypted.
- Backend APIs remain responsible for reads/writes, Settings authorization and credential injection/renewal; interfaces do not edit these files through a separate behavior path. Harnesses still retain their own Fetch client credential.
- JSON/TOML encoding, exact locations, one-file-versus-many layout, write coordination and encryption-key protection were not selected. Job-state lifetime and optional memory persistence remain open. PostgreSQL/retention stay deferred; no substitute database or code authorized.
- Updated canonical records and the milestone/handoff so a future agent preserves this choice. Documentation self-checks passed 37 Markdown files/9 JSON examples; journal entry appended. No independent/application verification was claimed.

## October 26, 2026 — Defer PostgreSQL functionality and retention from the initial MVP

Status: latest user scope decision confirmed; non-PostgreSQL persistence design unselected; implementation remains paused.

- The user continued after the handoff to sideline retention because the initial MVP should have no PostgreSQL functionality, and asked what else currently depends on databases.
- A read-only audit found no app source/Cargo/SQL/manifests or actual runtime DB dependency. Earlier database prerequisites were plans, not implemented functionality.
- Saved server IDs/config/launch/version metadata, encrypted credentials/OAuth and possibly shared prompt/history still need local saved/protected state. Live progress needs backend state. None inherently requires PostgreSQL; no JSON/SQLite/Keychain or memory-only design was selected.
- Deferred initial-MVP PostgreSQL provisioning/connections, database-host Tailscale setup, schema/migrations, 24-hour restart-result retention and database-backed recovery assumptions. Preserve current-operation IDs/status APIs, existing credential security and same-product memory direction.
- Documentation checks and independent evaluation passed; two stale current migration/tooling requirements were corrected. Audit/review/integration records are linked from the milestone; architecture remains incomplete.
- [storage-scope.md](docs/architecture/storage-scope.md), current records and the handoff are updated. Existing PostgreSQL installations, development connections and personal MCP configurations were not removed or changed. Remaining restart policies are still unanswered.

## October 26, 2026 — Prepare the final session handoff

Status: handoff documentation requested; architecture remains incomplete and implementation paused.

- The user said this is the last work together in this session and requested documenting everything with milestone references so another agent can continue, including the full planned outline/structure, current step and their preferences.
- [handoff.md](milestones/mcp-mvp-architecture/handoff.md) records the exact checkpoint, seven-step order, eight-route inventory/approval boundaries, accepted backend/import/auth/storage/lifecycle choices, proposed source tree versus confirmed package tree, research/evidence limits, implementation/test/release plan and reading map.
- Current position is step 1. The earlier "Good" approved restart wire fields; the subsequently presented remaining policy bundle is unanswered. Its deadlines/retention/overlap/readiness/envelope/permissions/recovery defaults remain proposals. This handoff request does not approve them.
- Preserve one whole contract per turn, plain-English explanations, user input on each route/major choice, API-first behavior, same-turn docs and serialized immutable milestone history. No code, package/service/configuration changes or new-thread messaging are authorized by the handoff.
- Handoff validation and independent evaluation passed 31 Markdown files/9 JSON examples plus links/anchors/format/history; ACCEPT with no material findings. Final immutable review/integration entries are linked from the milestone. Handoff complete; architecture milestone remains incomplete.

## October 26, 2026 — Approve restart fields and bundle each API contract into one turn

Status: request/result format and review preference confirmed; remaining restart policies proposed, not approved.

- The user answered "Good" to the displayed POST path, optional/default-false JSON `force_restart`, `202` fields/instructions and restart states/phases/success code/error fields. Settings loading/success/failure behavior is approved. Eight setup method/path pairs are now accepted; operational contracts still have gaps.
- The user explicitly requested each complete API contract in one turn, instead of multiple turns for its pieces. Preserve steps 1–7 and present the remaining restart clauses together before moving to another contract.
- [restart-contract.md](docs/architecture/restart-contract.md) preserves approved fields and proposes the remaining readiness/permissions/envelope/errors, one-active-restart rule, numeric deadlines, settings capture, result retention and interruption behavior. Those new clauses await feedback; the earlier "Good" does not approve them.
- Read-only primary-source follow-up supports checking replacement liveness and valid version-appropriate `tools/list`, with provider authentication reported separately. No real restart, login or tool invocation was tested. Documentation only; code remains paused.
- Documentation checks passed 28 Markdown files/9 JSON examples; independent review accepted the consolidated record after correcting two stale approval-status lead-ins. New operational policies remain unapproved and the architecture milestone remains incomplete.

## October 26, 2026 — Select the seven-step discussion order

Status: discussion order confirmed; technical proposals and implementation remain unapproved.

- The user asked to proceed step by step through 1–7: API contracts, tool sharing, execution behavior, authentication, implementation structure, onboarding and testing/delivery.
- Updated the checklist to match that exact sequence, preserving unfinished topics from the earlier grouping. Start step 1 with the unfinished restart request/result fields, then complete remaining setup contracts before step 2.
- Proposed for feedback: JSON-body `force_restart` optional/default false, `202` result fields/instructions, and restart status/phases with `SERVICE_STARTED` on readiness-confirmed success. This does not approve the full server-information envelope or readiness/error/timeout policies.
- Documentation checks passed for 24 Markdown files and 10 JSON examples; independent evaluation returned ACCEPT for approval boundaries, not the proposed API fields. Passes are recorded in the milestone journal.
- Documentation only; application implementation remains paused.

## October 26, 2026 — Review what remains before building and adopt the milestone journal

Status: documentation/planning work requested; remaining design proposals and implementation are not approved.

- The user asked what else should be covered to have the full API/build outline. A read-only Planner audit found seven accepted method/path pairs plus the working restart operation; filters are not extra routes and the earlier 15–20 estimate is not a target.
- [build-readiness.md](docs/architecture/build-readiness.md) inventories remaining tool/MCP/OAuth surfaces, contract details, permissions, installation/storage/lifecycle, onboarding and verification/release choices. Its grouped review order and acceptance demonstration are proposals for user feedback.
- New user instructions require a rolling milestone snapshot/to-do, immutable timestamped history, a supplied history template and milestone/folder guides. Initialized those documentation records; the Orchestrator serializes specialist reports and retains canonical decisions in docs/architecture/.
- Documentation format/link/JSON/template/history checks passed, and independent evaluation returned ACCEPT. This is documentation evidence only; application behavior remains untested. Current state and unfinished work are in [the milestone snapshot](milestones/milestone.md) and [to-do list](milestones/to-do.md).
- No code, runtime installations, services, credentials, harness configurations, commits or deployments were changed. Application implementation remains paused.

## October 26, 2026 — Select an explicit restart-ID query for monitoring

Status: confirmed query format; application implementation remains paused.

- The user selected `GET /api/v1/setup/servers?id=github&restart_id=restart-42` so the monitoring request explicitly identifies the server and restart attempt.
- Their stated reason is easier request monitoring when concurrent processes are considered in the future. This does not itself approve parallel restarts or multiple wrapper instances; those policies remain to be designed.
- The previous response-only placement suggestion was not selected. Ordinary server information still works without `restart_id`; including it selects the requested attempt rather than only the latest restart.
- Exact response fields, ID generation/retention, unknown/expired/mismatched IDs, and no-server/multi-server validation remain under discussion. The loading indicator stays active until this attempt's readiness is confirmed; force restart still bypasses waiting for active calls.
- Updated the architecture records only; no implementation or runtime/configuration changes.

## October 26, 2026 — Approve restart status through server information and an attempt ID

Status: historical checkpoint confirming route reuse and restart-attempt identity; the later entry above records the chosen query placement. Application implementation remains paused.

- The user approved reusing `GET /api/v1/setup/servers?id=...` for restart progress/completion, with a `restart_id` distinguishing the requested restart.
- They asked why an additional ID is useful beyond future parallel services and whether it would be a query parameter. Explain that one server can have multiple restart attempts over time; old readiness/success must not be confused with the new request's result.
- Proposed, not yet approved: return the ID in the `202` body and server's restart-status record, keep the GET query unchanged, and have Settings compare the IDs. An extra `restart_id=` query is an alternative, not selected by this approval.
- Follow-up clarification: the user asked how the URL contains the restart ID. In the response-only proposal, it does not: Settings already knows the server ID, remembers the returned restart ID, requests the server information URL, and compares IDs inside the response. Explicitly selecting an attempt in the URL is the alternative query-parameter design. This question does not approve either placement.
- Exact response fields, ID format, retention, superseded/missing-attempt behavior, readiness, and errors remain open. An attempt ID alone does not prevent duplicate POST execution or add restart history as a product feature.
- Only documentation was updated.

## October 26, 2026 — Select immediate restart acceptance and confirm immediate force behavior

Status: confirmed latest restart decisions; supersedes the earlier held-open POST choice. Application implementation remains paused.

- The user reconsidered completion reporting and selected immediate `202` followed by a separate success/status result. Keep the restart loading icon active the entire time until actual readiness is confirmed; do not treat acceptance as completion.
- Approved waiting for tool calls already running through Fetch during normal restart, while temporarily rejecting new calls to that server until it is ready. Queueing new calls was not selected.
- Explicitly reaffirmed that force restart starts stopping/restarting immediately without waiting for active calls. The flag is per request, may interrupt calls, and does not bypass the replacement's readiness check.
- At this point reusing installed-server information and adding a restart-attempt identifier were proposals; the later entry above records their approval. Exact wire codes, readiness, additional task tracking, polling, termination, timeout, and error handling remain under discussion.
- Only documentation changed; no application implementation, service operations, or personal configuration changes occurred.

## October 26, 2026 — Keep restart POST open until readiness (superseded)

Status: historical choice, subsequently superseded by immediate `202` plus separate completion in the entry above; application implementation remains paused.

- The user explicitly chose to keep the restart POST open until the new server is ready, rather than returning immediate `202` acceptance and polling.
- Settings keeps showing loading while awaiting the response; actual service-started success is still required. Installation progress and browser-authentication start keep their separately selected immediate-return behavior.
- Exact readiness checks, response bodies/status codes, activity tracking, timeout, and caller-disconnection handling remain under discussion. No application code or services were changed.

## October 26, 2026 — Specify restart control, readiness feedback, and force override

Status: user-specified restart requirements recorded; complete API contract still under discussion.

- Settings exposes the Restart UI automatically when `restart_required` is returned. The user clarified that activating the Restart control sends the POST; merely saving does not start restart automatically.
- Show loading until restart actually succeeds, requiring a service-started result rather than counting request acceptance/process creation as success.
- Normally wait for the current run/activity to finish where it can be checked. A temporary per-request force-restart flag set true bypasses the wait; it is not a persistent server setting.
- Research distinguishes Fetch-visible tool calls/observable tasks from an entire agent run or unreported background work. A proposed Fetch readiness code needs MCP responsiveness; there is no universal child-process `SERVICE_STARTED` exit code. Force cannot guarantee undoing remote API effects.
- At this point the completion choice was pending; the later entry above records the user's selection of a held-open POST. Exact wire/readiness/error semantics, timeout policy, and active-work boundaries remain open.
- Only documentation was updated; application implementation remains paused.

## October 26, 2026 — Clarify automatic Fetch configuration creation

Status: user-requested setup direction clarified; no application implementation authorized.

- The user asked to rely on automatically generated configurations instead of adding separate creation routes. The installation operation is responsible for deriving/saving each server's Fetch configuration from discovered non-secret settings and references; PATCH edits it later.
- This concerns Fetch's stored server configuration. It does not automatically authorize rewriting original Claude Code/Codex/OpenCode files or decide how the single Fetch connection is added to those harnesses.
- At this point the restart operation and apply policy were proposals. The later entry above records the user's UI/wait/force requirements while keeping remaining wire/readiness choices open.

## October 26, 2026 — Approve configuration and server-scoped credential/environment writes

Status: confirmed API/credential direction; application implementation remains paused.

- The user approved `PATCH /api/v1/setup/servers/{server_id}/config` for non-secret settings/references, available to authorized setup agents and the interface.
- Approved `PATCH /api/v1/setup/servers/{server_id}/credentials` for manual values entered through Settings, with encrypted storage, names/status-only responses, and backend-enforced Settings authorization rather than ordinary agent write access.
- The user explicitly clarified that the same credential operation/store should handle per-MCP environment-variable entries and custom auth tokens, scoped to one server. Do not create a separate environment-variable API by default or share those values across servers.
- Automatic trusted backend OAuth login/refresh writes remain part of the earlier accepted behavior. The Settings-only restriction concerns external manual entry, not disabling backend refresh.
- Exact payloads, encryption/key management, and the effect of edits on running wrappers remain unresolved. The next apply/restart policy is a proposal, not accepted by this approval.

## October 26, 2026 — Approve authentication start and installation-failure policy

Status: confirmed design decisions; application implementation remains paused.

- The user approved both parts of the resumed setup block: continue other selected installations after one fails, keep successful installations, mark the finished group failed if any item failed, and guide retries of failed entries through the existing installation endpoint.
- Approved `POST /api/v1/setup/servers/{server_id}/authenticate`: no credential values in this request, supported OAuth opens the native/default browser and returns immediately with `202`/`authenticating`, other known methods return Settings/permission instructions, and unknown methods report missing setup information.
- Agents check authentication progress through `GET /api/v1/setup/servers?id=...`; no separate authentication-progress route is selected.
- Updated the architecture records. At this point, configuration/credential-write routes and permissions were still proposals; the later entry above records their approval. No application code or service changes were authorized or performed.

## October 26, 2026 — Approve server-ID filtering and maintain docs continuously

Status: confirmed design/documentation instructions; no application implementation authorized.

- The user answered yes to the pending installed-server input/missing-ID proposal: repeated optional `id=` query parameters, no IDs for all installed servers, an array of matches even for one requested ID, and `not_found_ids` alongside matches for unknown IDs.
- Updated the architecture contract, confirmed decisions, open questions, index, and current project summary to reflect that approval. Exact server-record fields and remaining error/status details still require contract review.
- The user explicitly requested that the docs continue to be updated as discussion proceeds. AGENTS.md now requires recording decisions and resolving open items in the same working turn, without waiting for another sync request.
- At this point, the exact authentication route and install-group failure-continuation/retry policy were unapproved. They were subsequently approved in the entry above. This ID-filter approval did not authorize application coding.

## October 26, 2026 — Synchronize the MCP architecture discussion

Status: user-authorized documentation synchronization. Application implementation remains paused.

- The user asked to save the discussion in a folder under docs for future/runtime agents, and to make AGENTS.md identify it as the architecture-decision destination.
- Created the architecture index, confirmed-decision log, API discussion record, open-question list, research references, and draft implementation/testing/release roadmap in [docs/architecture/](docs/architecture/README.md).
- Updated AGENTS.md with required architecture reading/maintenance instructions and aligned PROJECT_CONTEXT.md with the current decisions.
- Current confirmed direction includes macOS-first, one Fetch process with MCP child processes, localhost access, configuration-derived npm/uvx preparation and local scripts run in place, no MVP catalogue, private runtime/server packages, explicit package updates, encrypted per-server credentials/fresh authentication, standard harness OAuth, and agent-assisted setup.
- Current accepted API boundaries: `GET /api/v1/setup/discovery`, `POST /api/v1/setup/installations`, and `GET /api/v1/setup/installations/{installation_id}`. Their remaining exact contract details are marked in the API document.
- At the initial sync, ID-filter details, the authenticate route, and the install-group failure policy were still proposals. The later approval entries above resolve those decisions; remaining implementation details stay marked in the architecture contract.
- The suggestion to regroup backend modules under `src/app/` was paused for explanation and never confirmed. Only the setup-handler folder `src/api/setup/` is accepted at that level of source organization.
- This request authorizes documentation edits, not application code, package installation, database changes, service startup, credential migration, personal-client reconfiguration, Git commits/pushes, or deployment. No such actions were performed as part of this sync.

## October 26, 2026 — Design the MCP sharing MVP with a Rust backend

Status: historical snapshot of the beginning of this discussion; later decisions are recorded in [docs/architecture/](docs/architecture/README.md). In particular, configuration-derived fresh installation replaces cloning configurations/credentials. Application implementation remains paused.

- The user wants to develop the common MCP server sharing portion of Fetch first in this discussion. Shared memory and MCP management remain capabilities of the same application.
- Rust is now the selected backend language, superseding the earlier unresolved backend-language question. The user wants Fetch built by us, with IBM ContextForge as design inspiration; the earlier requirement to build on ContextForge is no longer the assumed foundation.
- Fetch runs natively on the user's computer and prioritizes that current user. Do not expand this MVP into a multi-person service without the user's decision.
- During installation, automatically discover global MCP server definitions in the global configuration files of common coding harnesses. The user initially named Claude Code and Codex, then explicitly included OpenCode in the initial scope too. Project-scoped configuration import is not part of the stated requirement.
- Write scripts that copy those MCP configurations into Fetch, duplicating the definitions in the app. Start shared instances through Fetch so other harnesses can connect through a single Fetch MCP server exposing all the imported servers' tools. Exact tool naming, API contracts, credential handling, connection/session behavior, and runtime policies remain undecided.
- Tell users how to delete the original MCP server entries by providing a prompt they can copy and paste into their agents. This is the user-selected cleanup approach; do not infer authorization for Fetch or this discussion to delete or rewrite the existing configurations automatically. The prompt text and exact migration sequence remain to be discussed.
- Grok bot and Muse are later integrations, outside the initial harness scope. The exact Muse harness/product has not yet been identified; do not assume which one the user means.
- The user explicitly said not to code yet, and clarified that the assistant must not make decisions without them. Autonomous work is limited to research along the user's stated direction and recording their instructions. Architecture, implementation behavior, frameworks, API routes, grouping, batching, and other unresolved choices must be brought to the user for their input rather than silently chosen. Do not treat proposals or silence as accepted decisions.
- Keep every product operation behind the application APIs, with the UI and other clients wrapping them. Exact API routes and request/response formats still require the user's decision before implementation.
- Open questions include how discovery handles each harness's global configuration rules; duplicate/conflicting definitions; later synchronization, if any; tool-name collisions; individual versus batched calls; credentials; shared-state and process/session details; concurrency; and the remaining MVP behavior. The single Fetch MCP exposing imported tools is user-selected; generic discovery/call wrappers or alternate grouping must not be substituted without their decision. Earlier concurrency ideas remain proposals with unresolved details.
- The October 25 implementation stop still applies to code and restoration of the removed draft. Preserve the existing personal shared-MCP gateway and assistant configurations.

## October 25, 2026 — Stop implementation and strip the draft code

Status: user-requested removal completed. Do not resume implementation unless the user asks.

- The user said to stop implementation and strip the code because they did not want to make the technical decisions now.
- Removed the draft backend, shared types, migrations, scripts, synthetic tests, API contract, README, Cargo files, build outputs, and generated `.runtime/` data. No Iced UI code had been written.
- Stopped the draft Fetch service, ContextForge companion, and isolated project PostgreSQL instance. Preserved the existing personal shared-MCP gateway and client settings.
- Preserved AGENTS.md, PROJECT_CONTEXT.md, NOTES.md, the existing project-scoped `.codex/` configuration, and `.git/` with the user's remote. No commit or push was performed.
- A private source-only backup is stored at `/Users/yuyang/.codex/archives/fetch/2026-10-25-draft-836f5023.tar.gz`; generated credentials and database data are not included. Restore it only if the user asks.
- PostgreSQL 17 and cached development dependencies remain installed, but the project's services are stopped. Shared runtime packages and Homebrew dependencies were not uninstalled.
- The API-native direction and requirement to discuss every route/major architecture choice remain recorded for any future work; no decision or answer is required from the user now.

## October 25, 2026 — Discuss every API route and major architecture choice before implementing it

Status: required workflow for future implementation; the later stop/removal entry above governs current work.

- Prompt the user for EVERY API route and EVERY major architectural decision, and get their input on the exact technical design specifications before implementation.
- Work through these choices collaboratively. Explain each route's purpose, inputs, outputs, data/process changes, and meaningful tradeoffs; do not silently choose product behavior or overengineer hypothetical cases.
- The existing API contract and Rust backend from the initial pass are provisional drafts for review. A synthetic real tool call through ContextForge has worked, but verification is incomplete and no final acceptance has occurred.
- UI work is paused. Its preparation was read-only; no Iced application code has been written or launched. Preserve existing draft work rather than discarding it while design review proceeds.
- The first pending architecture question is whether sharing lives in a separate Rust service alongside ContextForge, inside a Python ContextForge extension, or in a Rust replacement gateway. No new answer has been accepted yet.

## October 25, 2026 — Build an API-native MCP sharing pass with a UI

- The user authorized building the MCP sharing feature alongside a UI to see and manage it.
- The entire repository must be API-native: all feature behavior is exposed through APIs, and the UI and other interfaces wrap them.
- The user wants to collaborate on defining each API and feature, decide product behavior personally, and avoid creative liberties or overengineering for hypothetical edge cases.
- The latest instruction is to try one-shotting it for now. Implement one minimal end-to-end pass from the requirements already described, and write down the exact API contract for review rather than treating additional ideas as accepted requirements.
- Keep the existing personal shared-MCP setup intact while implementing Fetch separately. The new work remains within the unified Fetch app; it does not authorize a separate memory product, MCP version control, or the deferred graph.

## October 25, 2026 — Use the Fetch GitHub repository

- The user designated `https://github.com/yuyanghu06/fetch.git` as this project's Git repository.
- Initialized Git in the existing `/Users/yuyang/Documents/fetch` folder on `main` and set this URL as `origin`, preserving the project documents and local configuration.
- A read-only remote check succeeded. No commit or push was performed during setup.

## October 25, 2026 — Keep memory and MCP management in one Fetch app

Status: confirmed product requirement; not implemented.

- Shared memory and MCP server management must live in the same Fetch application and interface. They are two capabilities of one product, not separate standalone apps.
- The website and Iced desktop app are interfaces to that same product. The exact screens, navigation, and MCP management subset for the MVP remain to be designed.
- A separate ContextForge backend service can remain an internal implementation detail; it does not make MCP management a separate user-facing application.
- Preserve the simplified memory scope: one shared prompt with simple history and undo, with the graph deferred. MCP version control remains out of scope for now.
- The optional MCP interface for accessing shared memory is distinct from Fetch's MCP server management capability. Browser and ordinary HTTP memory access remain supported directions.

## October 25, 2026 — Simplify the memory MVP to one shared prompt

Status: confirmed MVP scope; not implemented. This supersedes the memory graph as an initial requirement.

- Store one overarching shared memory prompt for the user, initially containing explicit user preferences. Assistants can retrieve that prompt and apply or inject its content into their available memory/context mechanisms where supported. Fetch cannot guarantee automatic injection or startup execution in every assistant.
- Provide simple history and undo for changes to that shared prompt. Users should be able to inspect earlier content and restore it. Git-style branching and merging are not part of this scope; revision storage, retention, and simultaneous-edit handling remain to be designed.
- Defer the memory graph, individual graph nodes, links, and connection-based sizing beyond the MVP. Preserve the earlier graph ideas as possible future work rather than requirements for the initial implementation.
- Keep version control out of the MCP side for now. Do not add MCP configuration revisions, branches, merges, or rollback as MVP requirements. This does not remove normal Git history for application source code or versioned database migrations.
- Preserve the API-first build order. Reading, editing, inspecting history, and undoing memory changes must use the same application APIs across the website, desktop app, HTTP clients, and any memory MCP interface.
- The initial memory interface can show and edit the shared prompt, with its history and undo, instead of presenting a graph. Keep the light/white visual direction.
- Iced remains the desktop framework to try. The Rust backend question and the detailed scope of Fetch's MCP management features remain as previously recorded; this decision does not settle them. The later same-app clarification above confirms that both capabilities belong in Fetch.

## October 25, 2026 — Try Iced for the desktop interface

Status: user-selected approach to try; no application code or prototype has been built.

- The desktop application should support Windows, macOS, and Linux using a shared codebase. The user does not want Electron.
- The user chose to try Iced for the Rust desktop interface. This is a trial direction rather than a claim that the framework has been validated for the product.
- The user also asked about using a Rust backend. That backend choice remains under discussion; do not treat the question as a decision to replace ContextForge or rewrite its gateway.
- Preserve ordinary HTTP/browser access, the API-first build order, and the existing PostgreSQL storage requirements. Choosing Iced for the desktop interface does not settle the website implementation.

## October 24, 2026 — Explore per-tool concurrency limits and on-demand MCP capacity

Status: user-requested feature proposal; not implemented. The user intends to build on IBM ContextForge.

### Shared UI

- Bring configured MCP servers together in one shared UI.
- Each server can be expanded to show the tools it exposes.
- Let users configure an individual concurrency limit for each tool: how many calls to that tool may be running at the same time across caller sessions using that configured server/account.
- Also expose a separate setting limiting how many MCP servers can run concurrently. The exact counting rule—distinct configured servers, backend worker instances, or both—and whether the cap is per server or installation-wide remain to be specified.

### Default and on-demand behavior

- Default each tool's concurrency limit to one.
- Start from one backend session/worker when a server is in use. Create additional capacity only when incoming calls need it and the configured limits permit it; do not pre-launch the maximum number of instances.
- When a tool's limit is raised above one, concurrent calls from separate client sessions should be able to use independent execution capacity immediately instead of all waiting behind the existing worker's queue.
- Example: with a tool limit of two, the first call uses available capacity and a second overlapping call can start an additional session/worker. This means two active executions in total. With the default limit of one, a second call must wait or be declined according to the eventual capacity policy.
- Enforce tool limits together with the applicable server capacity limit. The behavior when either limit is reached, reuse of idle capacity, startup failures, and idle shutdown rules remain to be designed.

### Implementation questions to preserve

- A new MCP connection/session does not necessarily create an independent backend process or bypass a backend's shared execution queue. HTTP servers may handle independent sessions in one process; a serialized command-line connector may require additional worker processes to gain parallel execution.
- Research ContextForge's existing per-client session handling and similar maintained open-source worker pools before choosing an implementation. Preserve caller-session isolation when sharing or reusing backend capacity.
- Additional workers must use the correct account and permissions. Independent connections do not automatically isolate shared application resources, such as the same document, calendar event, or Blender scene.
- The user has requested that this idea be documented. Runtime configuration, servers, and application code have not been changed by this note.

## October 24, 2026 — Keep the same database migrations across PostgreSQL hosts

Status: confirmed requirement; migrations have not been implemented.

- Maintain one version-controlled set of database migrations for local PostgreSQL, Supabase PostgreSQL, and PostgreSQL on a cloud instance.
- Local PostgreSQL through Tailscale remains the default. The user wants the option to move to Supabase or a cloud instance later while preserving the same application schema and migration history.
- Database connection settings vary by deployment. The Tailscale IP requirement applies to the default locally hosted database; alternative database hosts use their configured connection settings.
- Keep hosting-specific setup separate from the shared migrations and verify compatibility with the supported targets during implementation. Migration tooling, PostgreSQL version, and required extensions remain to be chosen.
- No migration files were found in the project when recording this requirement. No migrations or database changes were performed. A future move of existing data will require data transfer in addition to applying the shared schema migrations.

## October 24, 2026 — Use local PostgreSQL through Tailscale and distribute as open source

Status: confirmed storage direction; implementation has not started.

- The primary database will be PostgreSQL hosted directly on the user's own machine. This replaces the earlier Supabase primary-storage proposal.
- The user clarified that, for the default locally hosted database, application database connections must use the machine's Tailscale IP even when the application and PostgreSQL are on the same machine. This supersedes the earlier interpretation that Tailscale would be introduced only in the future.
- The future goal is to deploy the application on Vercel or another host while keeping PostgreSQL on the user's own machine. The hosted backend's authorized connection into the Tailscale network remains to be designed and verified; the private IP alone does not establish connectivity.
- The intended product is an open-source repository that anyone can run directly on their own machine. Each installation supplies its own PostgreSQL connection and credentials, with Tailscale for the default locally hosted database. Machine-specific settings and secrets must remain outside the repository.
- Keep the same memory operations and API-first architecture. Website submissions, ordinary HTTP clients, and MCP interfaces continue to use the shared memory APIs, which access the database.
- The exact remote access setup, PostgreSQL installation method, authentication, and backups remain to be designed.
- This update records the decision only. It does not install PostgreSQL, configure Tailscale, migrate data, or change the existing Supabase development connection.

## October 24, 2026 — Try single-use links for agent browser sign-in

The user wants to try single-use website connection links as an easy way to authorize an agent's own browser session.

Intended flow:

1. The user generates a single-use connection link on our website.
2. The user gives that link to the agent.
3. The agent opens the link in its own browser.
4. Our website exchanges the single-use token for an authenticated session in that browser.
5. The agent uses the signed-in website to access shared memories, with website operations going through the same memory APIs as other interfaces.

This flow establishes a browser session. Configuring credentials for direct API calls or storing them in an agent's credential vault is a separate connection flow.

Status: approach to try; not implemented or verified with Muse or Dots. Link expiration, permissions, session lifetime, and revocation details remain to be designed.

### October 27, 2026 — First tool-sharing contract proposed

After D36, recorded current MCP tools/schema/pagination/cache research and maintained ContextForge/agentgateway comparisons. The complete installed-tool listing proposal is in [the API contract](docs/architecture/api-contract.md#approved-installed-tool-listing-contract): live metadata on existing connections, complete per-server results, proposed 10-second deadline, filtered origin identities and bounded untrusted metadata projection. It remains unapproved; invocation, built-in tool mapping and MCP wire aliases/capabilities follow separately. No implementation or live checks.

### October 27, 2026 — Preserve raw MCP Tool JSON (D37)

The user rejected the six-field whitelist and chose complete raw tool objects, with only duplicate-name rejection and no incomplete-list publication as tool-list checks. Preserve all properties/nested values, including icons, `_meta`, version-specific/unknown fields. Current MCP has eight core fields (two required), while 2025-11-25 also included `execution`. Full-object preservation does not select extension execution or approve remaining GET policies/code. [D37](docs/architecture/decisions.md#d37--preserve-complete-raw-tool-objects) and the revised listing contract record the correction.

### October 27, 2026 — Listing approved; invocation next (D38)

The user said "sounds good, next" after the complete listing proposal and raw-Tool clarification. D38 records approval at the presented scope as revised by D37. Continue within step 2 to one complete installed-tool invocation API proposal; no invocation route, worker/session/concurrency policy, MCP alias/endpoint/extension implementation or code is approved by continuation.

### October 27, 2026 — Ordinary tool invocation proposed and remaining-turn estimate

The full proposed POST call contract follows D38 listing approval: exact server/name/arguments forwarding, one terminal raw result, tool-error versus protocol/transport uncertainty, no automatic replay/lifecycle/auth test, and an explicit ordinary-call capability boundary. Result preservation and validation ownership are new proposals; D37 covers definitions. Step3 must select finite deadlines/queue/worker/shared-state/cancellation/tracking before code. Research compared primary MCP sources and maintained gateways; no live call was made.

The user asked how many review turns remain. Current estimate: 12–18 more substantive review turns before ready to build, including this call contract; authentication/storage is the largest uncertainty and clarifications can add turns. This is an estimate, not a fixed route count or implementation authorization.

### October 27, 2026 — Direct model arguments through Fetch (D39)

The user clarified that Fetch should act as a middleman: accept exactly the argument object the model supplies to the original tool and pass it to the upstream MCP. Models keep each imported tool's input schema. Fetch resolves server/tool outside those arguments and privately constructs normal MCP transport framing. D39 also selects raw terminal ToolResult in `result`; Fetch's outer response remains separate/proposed. The revised HTTP path places server/tool in the URL and is a proposal, not approved by the input clarification. Other call lifecycle/error/capability policies and application code remain unapproved.

### October 27, 2026 — Raw body reconfirmed; complete revised call routing next

The user explicitly reaffirmed that `{"query":"example"}` is the POST body, referred to sending it to the MCP by ID, and said to keep going. D39 remains selected; the server ID and tool name must be resolved outside the body. Present the whole revised call contract with its proposed server/tool URL and outer result/status/next_steps, without asking again about the settled argument shape. The exact URL and remaining policies were not finalized merely by the body clarification. No new implementation authorization.

### October 27, 2026 — Tell the LLM about Fetch's wrapper (D40)

The user requires wrapper transparency to the model. Record D40: explicitly explain the outer Fetch fields versus unchanged upstream `result`, including null/no-result cases, through actual model-visible context/content. Keep notice outside original Tool JSON and raw result. Candidate Fetch context/outer text/format marker and exact MCP schema mapping remain proposals; merely adding metadata or silently rewriting upstream descriptions/outputSchema does not establish delivery. Dedicated primary-source research/Planner inspected; no live client/model test or implementation.

### October 27, 2026 — Concrete wrapper notice placement proposed

The user asked where the notice belongs in Fetch's response. Proposed top-level HTTP `response_context` beside unchanged `result`; the MCP adapter would render its text as a separate model-visible MCP content block. This corrects any ambiguity between the current application HTTP envelope (no outer content field) and an outgoing MCP ToolResult. Exact field/wording/rendering and complete MCP output-schema mapping remain unapproved.

### October 27, 2026 — response_context approved; MCP presentation next (D41)

The user said "good" then "next" after the concrete top-level HTTP response_context plus separate model-visible MCP text proposal. Record D41 narrowly: field/notice rendering selected, raw Tool/input/result preserved. Continue within step2 to the full MCP response/schema mapping; do not treat this as approval of wrapper structuredContent, derived schemas, serialized fallback, native media handling, endpoint/aliases/capabilities or the full call contract/code.

### October 27, 2026 — Complete MCP response presentation proposed

After D41, dedicated source research/Planner inspected two viable mappings. The full proposed presentation retains complete HTTP envelope E in structuredContent, separate response_context/full serialized E/native blocks in content, and a derived Fetch-facing outputSchema. Raw GET Tools/input/result stay intact. This requires explicit approval because the MCP-facing output schema changes and payload/media can be duplicated roughly three times; no truncation or native-media loss is proposed. Endpoint/aliases/capabilities, full call remainder and execution policies stay open; no implementation or live client tests.

### October 27, 2026 — Native MCP result plus one context block (D42)

The user rejected triplication and requested the raw MCP response with context text. D42 selects native terminal ToolResult fields/content once plus one Fetch notice block, keeping upstream structuredContent/isError/outputSchema. The HTTP wrapper/raw result remains intact. Supersede full-envelope structuredContent, serialized JSON fallback, media copies and derived output schema; do not recreate them. Native media types already exist in upstream blocks. MCP notice wording must fit native presentation; exact Fetch-only/no-result/protocol handling remains open, with no code or live compatibility test authorized.

### October 27, 2026 — Fetch-only MCP failure presentation proposed

After the user asked to continue, dedicated research checked official errors/output-schema rules and maintained SDK validation paths. Python/TypeScript skip output validation for received isError results; the official page documents text-only errors but no separate blanket exemption beside outputSchema requirements. The whole next proposal preserves D42 upstream results, uses a clearly Fetch-owned one-text isError failure for stopped/busy/pre-dispatch or uncertain timeout/drop, leaves HTTP raw result null, and keeps authentication/protocol errors native. Exact timers/protocol adapters/host delivery remain later work. No new approval, software install or live call/model test.

### October 27, 2026 — Raw upstream returns; Fetch explanations in next_steps (D43/D44)

The user approved the failure mapping except that upstream ToolResults must pass through raw for the agent to handle. Fetch-owned failures get explanatory text; no added Fetch notice on upstream success or error (D43). The user then explicitly clarified that explanations should use next_steps. Reuse actor/action/instructions and remove response_context (D44). HTTP raw result remains untouched; MCP renders Fetch-owned failure guidance as text without inserting fields into upstream results or protocol envelopes. Continue to tool-name alias design; no implementation authorization.

### October 27, 2026 — MCP tool-name aliases proposed

Dedicated current-spec/source research and Planner inspected naming/collision approaches. The whole proposal prefixes every imported tool using a stable server token, reserves fetch__ for future Fetch-owned names, maps aliases to exact server/name identities and allocates generated portable aliases for unusual/long names. Canonical GET Tool JSON and argument/result formats stay unchanged. Examples mcp_s1__search/mcp_s2__search are illustrative; naming rules, persistence/reservation policy and exact encoding/limits remain unapproved. No actual alias generation or runtime tests.

### October 27, 2026 — Naming approved; MCP connection next (D45)

The user approved the complete presented alias policy with "yes this works for me." D45 selects stable prefixes/exact origin lookup, reserved Fetch namespace, generated exceptional aliases, stable non-retargeting and preservation of canonical Tool data/arguments/results. Encoding, host limits and persistence implementation remain open. Next whole review is the localhost MCP connection contract; public source research must distinguish documented client transport support from unverified protocol-version compatibility. No code/SDK/port/endpoint is approved by naming approval.

### October 27, 2026 — Complete MCP connection contract proposed

After naming approval, dedicated research/Planner prepared one loopback /mcp Streamable HTTP proposal with explicit legacy2025 and modern2026 behavior, per-request authorization, local Host/Origin policy, ephemeral client sessions/streams and no tool-call replay on reconnect. Current client transport docs do not prove exact revision compatibility. The standard OAuth issuer/login/token endpoints require HTTPS; issuer/TLS/trust remains step4, separate from the candidate HTTP loopback resource. These are unapproved connection choices; no default numeric port/SDK/call timer is selected. All actual connection tests must be isolated under D46; no environment or live test was run.

### October 27, 2026 — Connection policy approved; Fetch-owned tools next (D47)

The user approved the presented connection contract after clarifying protocol revisions, the shared localhost API port, /mcp tool routing and tool-list discovery. D47 selects the presented route/transport/compatibility/auth/request/reconnect policy, not unshown exact wire codes, port/SDK/client releases, OAuth HTTPS issuer/TLS or ordinary execution mechanisms. Models still supply raw tool arguments; /mcp handles framing/alias routing to the same backend APIs. Next whole contract is Fetch-owned management-tool names/inputs/API mappings; manual secret writes remain Settings-only.

### October 27, 2026 — Management-tool proposal and argument-routing clarification

The /mcp handler reads tools/call name/arguments and routes imported arguments unchanged through the backend API to the upstream server. There is no new input wrapper. Fetch-owned management tools separately extract URL/query selectors. A Planner audited the complete eight-tool proposal in [mcp-management-tools.md](docs/architecture/mcp-management-tools.md), including Settings-only manual credentials and single-copy API JSON text results. Those names/inputs/result rules await user input; D47 connection approval does not approve them or building.

### October 27, 2026 — Present the complete Fetch management-tool contract

The user asked to continue after the /mcp argument-routing clarification. Present the already independently reviewed eight-tool inventory, exact inputs/API mappings, defaults, permissions, single API JSON text result, pending/read/error semantics and lifecycle together. The management-tool proposal still awaits user input; this continuation does not approve it or authorize application implementation.

### October 27, 2026 — Every application API operation needs a descriptive tool (D48)

The user requested tools that describe what all APIs do. Record this as an ongoing coverage requirement, not a permanently fixed eight-tool inventory. Describe Fetch-owned reads/changes, inputs, results/pending states, permissions and human handoffs. Keep the established Settings-only manual credential exception and native OAuth/MCP roles; preserve imported definitions. Exact management names/result mapping and future routes still await input. See [D48](docs/architecture/decisions.md#d48--give-application-api-operations-descriptive-mcp-tools).

### October 27, 2026 — Management mapping accepted; remaining HTTP call contract next (D49)

The user said "you're good with that though next" after the complete management-tool contract and D48 amendment. D49 accepts the presented names/inputs/API mappings/results/permission/lifecycle rules, with descriptive tools for future selected APIs and Settings-only manual credentials. Unshown mechanisms and implementation remain unapproved. Continue within step2 to the remaining installed-tool HTTP route/outer outcome/errors/lifecycle contract; raw model arguments and upstream results stay settled.

### October 27, 2026 — Complete remaining HTTP tool-call proposal

Inspected the read-only Planner audit and existing invocation research. Refined the proposed exact selector encoding/one-time decoding and D45 alias resolution; no schema/argument rewriting or pre-call listing/auth/start is added. The HTTP outer response distinguishes received tool errors from missing/uncertain results, keeps next_steps and no-store, and has an explicit safe error table. Ordinary terminal calls remain the route boundary; full MVP capability choices and numeric execution mechanisms still need later review. The whole HTTP remainder is proposed, not approved or implemented.

### October27, 2026 — Remaining subjects and major API inventory

The user asked how many subjects still need feedback and whether major APIs remain. The Orchestrator estimated roughly 8–12 bundled review turns, depending on questions: finish tool-sharing, then execution, auth/storage, implementation, onboarding and testing/delivery. This is an estimate, not a promise. Major remaining API review covers the pending tool-call contract, OAuth endpoint family and any selected client-connection operations; memory read/edit/history/undo remains conditional on release sequencing. Update/reinstall route versus reuse is open and extra start/stop/removal controls are optional. Corrected build-readiness's stale tool-listing/management statuses to D38/D49; no new scope/route/implementation is approved by this question.

### October27, 2026 — Faster architecture review batches

The user asked to start covering subjects more and faster. Bundle related complete decisions, reuse existing research/independent reviews, avoid repeating settled choices or turning routine internal details into separate approval turns. Present the already reviewed complete remaining HTTP tool-call contract now; numeric execution/cancellation/concurrency choices remain the next bundle. This pacing instruction does not approve pending policies or implementation.

### October27, 2026 — HTTP call contract accepted; execution next (D50)

The user replied "godo" to the complete remaining tool-call contract. D50 accepts the presented route/input/authority/forwarding/outer outcome/error/lifecycle/unsupported-interaction rules, preserving raw upstream data and next_steps. Numeric execution, sharing, cancellation and remaining capability scope were explicitly left for the next bundled review. Dedicated research now informs those choices; no implementation is authorized.

### October27, 2026 — Complete ordinary execution proposal

Dedicated primary-source research finished before Planner. The proposed bundle uses one shared wrapper/connection and one ordinary call/server, different servers in parallel, immediate busy rejection, fixed120-second wait and best-effort cancellation. Uncertain calls retain capacity until terminal RPC response or confirmed child exit; a no-reply cancellation can require explicit force restart, without automatic replay or rollback promises. The proposed tool_call_state GET field keeps state-driven UI honest without exposing callers/arguments/results. Shared browser/account state is explicit. All new execution clauses await feedback; D50 call policy remains approved and no runtime work is authorized.

Execution-proposal clarification before presentation: native legacy2025 MCP stream loss alone does not imply cancellation; modern2026 does. Ordinary Fetch HTTP disconnect is proposed as cancellation. Root checked both primary specs and made these triggers explicit, keeping the shared deadline/uncertainty/recovery policy and no-replay rule.

### October27, 2026 — Queues required; individual tool timeout question (D51)

The user requires calls to enter queues with concurrency handling, replacing the unapproved no-queue proposal. The user then questioned120 seconds and asked whether Fetch can read individual tool limits. Research confirms no standardized Tool timeout field, so input/custom metadata cannot universally determine an end-to-end deadline. The assistant withdraws its own fixed120 draft; configurable fallback/overrides and any recognized metadata interpretation remain proposals, not unlimited waiting or approved defaults. Queue wait and execution timing must be separate. Revised concurrency/queue/restart/aggregate-state planning continues; implementation remains paused.

### October27, 2026 — Durable tool settings and120-second fallback (D52/D53)

The user requested an installation-config tool-feature list, including timeout limits, applied during MCP calls; then explicitly selected120 seconds as the default and asked to continue. D52/D53 record durable per-server non-secret feature/settings profiles and a120-second fallback rather than a hard cap. Fetch applies its execution options without changing raw model arguments. Actual metadata discovery may require the first authorized server connection; installation must not silently start/login to a server. Exact schemas, provenance/override/grace, queue/concurrency and live application remain the next full proposal; code remains paused.

### October27, 2026 — Revised queued execution/settings contract prepared

Inspected the prior queue Planner report and D52/D53 follow-up. The new whole proposal keeps120 seconds as fallback, with explicit per-tool overrides/known documented limits, and proposes FIFO scheduling, configurable concurrency,64 waiting entries/16 MiB/300-second queue wait,5-second known-limit allowance, config PATCH/GET/tool amendments and aggregate state. Profiles initialize unknown without install-time startup and populate through authorized readiness discovery. Per-tool map edits merge visible keys and null removes an override, protecting hidden entries; no unlimited null or raw-argument injection. All newly proposed values/schemas/restart/hot-apply details await input; no code is authorized.

### October27, 2026 — Configurable default timeout for every managed MCP (D54)

The user explicitly requires every managed server to have its own configurable default timeout. Store the Fetch setting durably per server, initially120 seconds, and expose it through the existing config API/Settings/matching tool under ordinary config permissions. The shown execution.default_timeout_ms expresses milliseconds; per-tool limits remain more specific and raw arguments stay unchanged. This does not require native server timeout support or approve the rest of the proposed queue/config/hot-apply/restart block. No implementation is authorized.

### October27, 2026 — Install initializes timeout; handler applies execution deadline (D55)

The user clarified automatic install-time timeout setup and injection as the execution deadline. Store a timeout duration in durable server config; initialize from a supported explicit server-default source when available, otherwise120 seconds. On dispatch compute a fresh deadline from the effective duration; queue wait is separate, existing user settings survive install reuse and raw arguments/protocol metadata are not modified. Automatic initialization does not mean universal metadata discovery or starting/login during installation. Other proposed execution rules and implementation remain unapproved.

### October27, 2026 — Keep configurable processes and distributed queues (D56)

The user still wants a configurable total number of concurrent running processes for each MCP, distributing queued calls among those processes and waiting according to available capacity. They said "maybe not in the mvp": preserve the feature without deciding release scope. This is process-level concurrency, separate from simultaneous calls in one process. Exact queue distribution, limits/settings, related-call state affinity, credentials and lifecycle remain for review; no new processes or implementation are authorized. See [D56](docs/architecture/decisions.md#d56--preserve-configurable-processes-per-mcp-and-queue-distribution).

### October27, 2026 — Multiple processes deferred for now (D57)

The user clarified "i dont want it yet, kep going pls." Defer D56's configurable multiple MCP process instances/queue distribution for now and preserve it as a future feature. The current design uses one Fetch-managed process per logical MCP; ordinary queued calls and call-level concurrency remain separate. This does not approve every pending queue/config parameter or authorize implementation. Continue with remaining MVP protocol capability scope before authentication.

### October27, 2026 — Complete initial MCP capability proposal

After deferring process replicas and asking to continue, the next whole proposal keeps installed/Fetch tool listing/calling, complete raw terminal results and selected catalog notifications. It proposes leaving provider-tool progress relay, separate resource/prompt operations, sampling/roots/elicitation/continuations and MCP Tasks out of the initial release, with private/zero-TTL modern catalog framing. These exclusions are not yet approved; raw definitions/metadata stay preserved and unsupported interactions get truthful existing errors. Fetch installation/auth/restart progress remains. No GitHub/memory scope decision or implementation is implied.

### October27, 2026 — Real daily-use acceptance and inventory inspection (D58)

The user wants to test the MVP through everyday Claude Code/OpenCode use and asked why the assistant requested a server list already available. Root inspected exposed tool declarations plus safe global config metadata:19 Claude entries,18 Codex entries (one disabled), four nonloopback Claude HTTP/SSE services, local gateway entries and native image tools. No credentials/URLs/arguments were output or server test calls made. The tool-only blanket exclusion proposal is on hold; [the audit](docs/architecture/real-use-compatibility.md) explains actual client support, extra relay/state work and a proposed remote-upstream scope gap. OpenCode default files/executable were not found here, not proof of absence elsewhere. Actual client/feature/transport acceptance remains to verify in isolation before authorized personal rollout.

### October27, 2026 — Shared functions behind /mcp (D59)

The user asked whether the protocol functions can share POST /mcp and accepted individual shared functions invoked by a runner depending on results. Record one endpoint with native-method dispatch and live event/intermediate-result handling, preserving raw arguments/terminal results and each caller's context. D47 streaming/revision channels remain. This settles organization, not blanket capability inclusion, remote transport or permission to begin application implementation. See [D59](docs/architecture/decisions.md#d59--share-mcp-handlers-behind-the-existing-endpoint).

### October27, 2026 — Streaming-default question

The user asks whether streaming can be the default or needs a default mode. Recommend Fetch choosing automatically: SSE for tool execution/ongoing operations, plain JSON for quick reads, native acknowledgement/error behavior retained, with no user toggle. Streaming relays messages and available progress; final tool data remains raw and quiet tools need not produce partial results. This is a proposal awaiting feedback, not an accepted default. See docs/architecture/mcp-connection.md.

### October27, 2026 — Automatic streaming approved (D60)

The user said "You’ve got it" to automatic streaming for tool calls, ordinary JSON for quick reads and no user-facing streaming mode. Relay available progress and preserve the unchanged final result; a quiet tool can send only its final event. Native input flows remain at /mcp. This resolves the earlier proposal above, not all additional capability scope or implementation.

### October27, 2026 — Remote upstream scope proposal after D60

After "Next," the review moves to covering existing PostHog/Linear/Calendly/Tavily HTTP/SSE entries. Recommend saving a connection definition rather than downloading a package, preserving raw tool data/permissions/queues and fresh encrypted per-server authentication, while Fetch stays localhost-only. A remote reconnect cannot restart the provider or prove unfinished work stopped. Exact API amendments, version/header-annotation handling and remote uncertainty recovery remain explicit follow-up contracts. See [the proposal](docs/architecture/remote-upstreams.md); no scope approval or live test is inferred.

### October27, 2026 — HTTP/SSE as routed transport functions (D61)

The user said HTTP/SSE support can be a separate function routed for those calls. Select HTTP/SSE upstream support alongside local processes, using saved server connection type to choose the transport adapter in the existing runner. Shared permissions, queues/deadlines and raw arguments/results stay above that adapter; no new client-facing route or model argument format is needed. This resolves remote inclusion/organization from the earlier proposal, not all exact import/settings/reconnect/version/annotation policies or implementation. See [D61](docs/architecture/decisions.md#d61--route-http-and-sse-through-separate-transport-functions).

### October27, 2026 — Per-tool transport configuration and dispatch context (D62)

The user wants each tool configuration to carry the MCP connection type, injected into calls so Fetch knows which function to invoke. Record that as Fetch-owned internal routing context passed alongside raw arguments, selecting stdio versus HTTP/SSE handling. Do not insert routing fields into the model argument object or rewrite raw upstream tool definitions. All tools from one connection route consistently; exact physical fields/references remain unselected. See [D62](docs/architecture/decisions.md#d62--carry-per-tool-transport-routing-in-the-internal-call-context).

### October27, 2026 — Shared runner handles every selected transport

The user reaffirmed that the shared runner must include handling for all these types: stdio, Streamable HTTP and legacy HTTP+SSE. Record actual dispatch/invocation/message/result handling through each transport adapter under the same shared permissions, queues and deadlines. This clarifies D61/D62 rather than selecting extra protocol capabilities or authorizing implementation.

### October27, 2026 — Complete remote API amendment proposed

After "next," the review advances to one combined remote connection API amendment: same import/progress/server/config/auth/restart routes, remote config metadata, header-reference edits and Connect/Reconnect semantics. Recommend keeping imported endpoint/transport fixed so agent management cannot redirect stored credentials to a different host; changing endpoint requires a new distinct import/fresh auth. Proposed forced reconnect retains unknown old outcomes but permits a fresh current connection capacity budget, without replay or a remote-stop guarantee. See [the complete proposal](docs/architecture/remote-api-amendment.md). All new clauses await user input; no runtime work or personal config changes occurred.

### October27, 2026 — Remote API amendment approved (D63)

After the complete proposal and requested TLDR, the user said "good." Accept the presented existing-route remote preparation/config/state/auth/reconnect behavior, fixed imported URL/transport identity, editable header references and no source credential migration. Normal reconnect waits; force permits fresh connection capacity while preserving unknown prior outcomes/count and never replaying calls or claiming provider termination. Presented reconnect budgets are60/5/30 seconds. Separate protocol/security/provider/storage mechanisms remain open and no implementation/live testing is authorized. See [D63](docs/architecture/decisions.md#d63--approve-the-presented-remote-connection-api-amendment).

### October27, 2026 — Protocol compatibility exceptions proposed

After "next," dedicated primary-source/SDK research found that modern HTTP SDKs may silently filter invalid header annotations and modern native Result requires resultType absent from legacy results. Propose raw capture/rejection observability, explicit whole-origin/native-catalog failure instead of partial success, and only a required missing marker in the modern native copy while canonical HTTP raw R stays unchanged. One bad origin can block a modern catalog request; this tradeoff is explicit. Upstream revision policy/private zero-TTL envelopes are also proposed. See [the bundle](docs/architecture/mcp-protocol-adapters.md); no D37/D43 exception, SDK or build is approved yet.

### October27, 2026 — Marker approved; return19 valid tools with incomplete status (D64)

The user said the first result-marker example is fine, and asked why Fetch cannot return the other19 tools while reporting incomplete after one invalid definition. Select that direction: keep valid tools raw/usable, mark catalog completeness false and explain rejected identity/reason outside Tool objects; do not block the whole server or combined native list merely for one rejected tool. The native-only required legacy marker exception is approved; canonical HTTP raw R stays unchanged. Exact partial-origin/rejection schema and native metadata/client visibility need review. Earlier upstream-version/cache defaults remain unapproved. See [D64](docs/architecture/decisions.md#d64--approve-the-native-result-marker-and-explicitly-incomplete-valid-tool-lists).

### October27, 2026 — Incomplete-catalog fields and view delivery proposed

After "keep going next pls," draft the exact D64 mapping: partial source status, rejected_tools reasons, raw/client-protocol views and outer native Fetch metadata. An annotation ignored legitimately by legacy/stdio must not globally hide that tool because a modern export rejects it. Propose direct HTTP raw default, native actual-client view, and a documented matching fetch__list_tools context default; no imported arguments/definitions/results change. Metadata alone may not reach the model; the existing listing tool emits JSON/guidance when called. Upstream targets/cache defaults are included for input, not approved. See [the complete proposal](docs/architecture/tool-catalog-status.md).

### October27, 2026 — Catalog status/view and remaining defaults approved (D65)

The user said "good" to the complete listing bundle, then asked where returns appear. Accept statuses/rejection details, view selection/defaults, native _meta["fetch/catalog"] issue summaries, initial upstream2025-11-25/2026-07-28 targets and modern ttlMs0/private catalog hints. HTTPGET returns JSON; native tools/list returns Tool definitions plus outer metadata; fetch__list_tools emits API JSON/next_steps in its text result. The later delivery question does not revoke approval. Model metadata visibility/SDKfidelity still need isolated evidence; no app implementation/live tests authorized. See [D65](docs/architecture/decisions.md#d65--approve-catalog-status-protocol-views-and-versioncache-defaults).

### October27, 2026 — Native progress and interaction relay proposed

After "next," the remaining step2 discussion moves to native progress and requests for the caller's workspace/model/form/URL interaction. Recommend relay through the shared /mcp runner only when caller ownership/capabilities are provable; no Fetch-owned model/dialog/filesystem fallback. Older stdio/background requests may have no dependable parent and must not be assigned by active-call count or most recent client. Modern continuation uses a temporary owner-bound Fetch handle while restoring the provider's opaque state unchanged, one logical call/deadline and raw final result. Direct REST stays terminal-only; Tasks/resources/prompts remain separate. See [the complete proposal](docs/architecture/mcp-interaction-relay.md); no feature/implementation approval inferred.

### October27, 2026 — Native relay approved; endpoint/function integration clarified (D66)

The user said "yep" to native progress/client-interaction relay, then asked how it fits /mcp and the existing designed service functions. Accept the presented ownership/capability/legacy limitations, same-call deadline/temporary continuation handle and preserved raw arguments/final result. The endpoint dispatches to the shared runner; stdio/HTTP/SSE functions carry messages; progress/workspace/model/form handlers relay supported events to the owning client. Existing backend config/auth/credentials/reconnect services stay shared, and provider tool functions are not reimplemented. Direct REST remains terminal-only; no application code or live tests authorized. See [D66](docs/architecture/decisions.md#d66--approve-the-presented-native-progress-and-interaction-relay).

### October28, 2026 — Native resources/prompts and remaining scope proposed

After "next?", finish remaining native sharing scope with resources/list/templates/read, prompts/list/get and supported argument suggestions through /mcp. Propose stable source-qualified discovery selectors while preserving resource-read payload/subresource URIs and D43 raw tool-result links. Unknown/ambiguous unqualified links require source selection, not background probing/file/HTTP fetch. Propose10-second per-source lists, configured server budget for reads/get and existing owned interactions/catalog-change delivery. Explicitly propose deferring live resource-content subscriptions/native Tasks only if actual isolated workflows are sufficient; ordinary tools returning job IDs still work. All new clauses await input. See [the proposal](docs/architecture/mcp-resources-prompts.md).

## October28, 2026 — native resource/prompt approval and execution review

D67 approves the presented native resource list/read/templates, prefixed prompt list/get and supported completion on /mcp, qualified source routing with unchanged returned content/raw ToolResult links, safe ambiguous-link errors,10-second per-source lists, configured server read/get budgets, required modern outer framing/private zero-lifetime hints and catalog-list changes. Live per-resource content subscriptions and native background Tasks are deferred for the initial slice, subject to D58 isolated everyday-workflow acceptance. Exact URI codec/SDK/numeric errors/unshown metadata wire fields remain review/evidence work; implementation remains paused. See [D67](docs/architecture/decisions.md#d67--approve-presented-native-resourcesprompts-and-explicit-deferrals). Next review is queued execution/settings; GitHub CLI scope/auth ownership remains explicit dependent work.

## October28, 2026 — queue approval and no duplicate reviews

The user approved the presented ordinary queue/settings policy (D68), then requested checking for duplicate discussion. Canonical open questions were rewritten as settled-versus-specific-missing choices; stale queue, remote transport, raw tool, management mapping and general auth questions were removed from the active register. Approved policies should be referenced, not presented for approval again. Keep genuinely missing schemas/mechanisms/evidence explicit and preserve dated history.

## October28, 2026 — remaining checklist and next trust proposal

The user asked to keep going and keep updating the remaining decisions checklist. [Open questions](docs/architecture/open-questions.md#remaining-decisions-checklist) now contains the canonical seven-step checklist, separate from internal implementation work, future isolated evidence and deferred features. Approved queue/native/remote/catalog policies are not reopened. Stale current-status summaries were corrected while immutable history was preserved.

Read-only Planner and Researcher passes support a proposed [local HTTPS/certificate-trust arrangement](docs/architecture/local-oauth-trust.md): one loopback listener, unique certificate per installation and explicit human trust setup, preferably certificate/hostname-limited where actual clients prove support. Wider CA trust needs a separate decision if necessary. This is a proposal awaiting input, not a new confirmed decision or client compatibility claim. No certificate, trust store, personal client, credential, runtime dependency, environment or service was changed or tested. Application implementation remains paused.

## October28, 2026 — account-free local app connection (D69)

The user questioned the phrase “Fetch's browser login,” then clarified: “We don’t need a fetch account the apps just need to get an auth token directly through browser connection.” They added that it should not be complicated because Fetch is local. Record no Fetch account, signup or password; browser connection is for an app to obtain its own Fetch token. Keep D10's standard exchange/renewal direction. [D69](docs/architecture/decisions.md#d69--connect-apps-through-the-local-browser-flow-without-a-fetch-account) records the scoped decision; exact approval controls, HTTP/HTTPS/certificate trust and complete auth wire contracts remain open.

The requested [other-MCP comparison](docs/architecture/local-oauth-trust.md#how-other-mcps-approach-this--october28-research) now covers local stdio, ToolHive's local HTTP exception, hosted GitHub HTTPS and the official C# development HTTPS authorization sample. It qualifies the earlier certificate recommendation; no source/runtime/certificate/trust-store/client or credential changes occurred.

## October28, 2026 — HTTPS/certificate direction approved; app authorization proposed

After the certificate creation/verification effort discussion the user said “Good to go for this part, next.” [D70](docs/architecture/decisions.md#d70--approve-local-https-and-the-certificate-verification-direction) records the presented local HTTPS/shared-listener, unique certificate/key and explicit trust/normal-verification direction. This is not authority to create an environment, change actual certificate/client trust or resume application implementation. Exact key/certificate lifecycle and compatibility evidence remain open.

The next [complete app connection/token proposal](docs/architecture/client-authorization.md) keeps D69 no-account browser connection and proposes standard routes, public registration, separately approved management scope, a one-time Mac confirmation for human approval, token lifetimes/renewal/reuse/revocation and restart/error/bounds behavior. Dedicated Researcher and read-only Planner evidence were inspected. All new clauses remain proposed, including the Mac confirmation; D70 does not approve them.

## October28, 2026 — app-connection contract approved (D71)

The user replied “Yes” to the complete presented app-connection contract, including the one-time Mac confirmation and separate management permission. [D71](docs/architecture/decisions.md#d71--approve-the-presented-account-free-app-connection-contract) records the exact shown scope: seven operations, four scopes/default use versus optional management, current/future server/tool coverage under backend restrictions, lifetimes/renewal/reuse/revocation/restart behavior and admission bounds. It does not adopt every linked unshown detail or authorize implementation/live tests.

Next is concrete Settings human authority and key protection/unlock/recovery. General encrypted per-server files, D28 placement, no-account flow and reviewed app token behavior are settled; avoid re-reviewing those broad choices.

## October28, 2026 — complete Settings/key protection proposal

After D71 approval, dedicated source research and a read-only dependent Planner support [the next complete Settings/key proposal](docs/architecture/secret-storage-settings.md). It proposes traditional login Keychain per-server keys for locally built Fetch, config.json plus credentials.enc, standard authenticated encryption and separate app-token/TLS identity. Manual secret entry would initially use only Fetch's own native Settings window, with a temporary private capability sent through the existing HTTPS credential PATCH and fresh Mac confirmation per save/removal. Ordinary OAuth cannot obtain this authority; automatic backend refresh remains automatic.

These are new proposals, not implications of earlier encrypted-storage/Settings-only decisions. Unlock/denial/corruption/key-loss behavior preserves old data; recovery controls and exact libraries/signing/serialization remain explicit future review. No system or runtime change occurred.

## October28, 2026 — shown Settings/Keychain direction approved (D72)

After the commentary explaining encrypted per-server files/Keychain keys, native Settings manual save/remove with Mac confirmation and no Touch ID on ordinary calls/automatic renewal, the user said “yes good” and “i already approve this next one keep going.” [D72](docs/architecture/decisions.md#d72--approve-the-shown-settings-and-keychain-direction) records that shown direction. The longer draft's exact Keychain mode/crypto/header/bootstrap/timers/errors/recovery was unshown, so those remain implementation/storage/onboarding candidates. Do not reopen the approved broad choice. Continue to precise provider verification/failure policies while preserving D25/D31 and implementation/test pause.

## October28, 2026 — complete provider verification/failure amendment proposed

The next [amendment](docs/architecture/credential-verification.md) uses existing authenticate/credentials/status operations with the selected stored-credential success gate. It proposes a30-second shared check budget, concrete safe failure/persistence handling, unknown/unbound SET preflight rejection and no automatic rollback/retry. It explicitly asks for a narrow storage-only200 exception for removal, reviewed non-auth-only changes/no-ops and empty batches; authentication-value SETs still require the real test, even when unchanged. This exception and all new policies remain unapproved. GitHub/Slack source examples are not complete integration approval.

## October28, 2026 — current remaining-decision count

The user asked how many decisions remain. Root counted10 open groups in the canonical checklist after D72, including the pending provider-verification amendment, plus separate explicit implementation authorization. These are grouped subjects with multiple details, not a promised number of turns. Routine internal work and actually running future tests are separate from design decisions. No pending proposal was approved by the count question.

## October28, 2026 — consolidate the plan and stop repeated reviews (D73)

The user clarified that each agent stores a Fetch credential/token and connects once to `/mcp`, while users complete individual installed-provider auth flows separately. They challenged execution and Rust being listed as unfinished, deferred shared memory from this release, and selected standard drag-to-Applications Mac installation and MIT-style licensing. D73 records MIT and a DMG as the concrete selection.

The user then said planning had been circling for three days and requested the full step with everything needed noted together. The prior ten-group count wrongly mixed locked product decisions, engineering work and later test/release prerequisites. It is superseded. [The single implementation checkpoint](docs/architecture/build-readiness.md) now contains the locked release specification, five build stages and one complete engineering worklist. [The current index](docs/architecture/open-questions.md) has one remaining product-scope question: a new custom GitHub CLI adapter, distinct from imported GitHub MCP compatibility. Unselected optional controls are not silently added.

The unaccepted provider-verification amendment/storage-only200 exception is withdrawn from active review; it was not approved or rejected by this clarification. Preserve D25/D31 and resolve necessary concrete engineering behavior in one implementation brief. Original pre-consolidation context/tasks are preserved in the milestone folder; immutable history is unchanged. The user has not authorized application implementation, real client/credential/Keychain changes, environment provisioning, live tests or release.

## October28, 2026 — pre-build MCP source-session audit

The user requested checking previous sessions before starting. [The audit](docs/architecture/mcp-session-audit.md) reconciles seven available primary Fetch conversations, including the archived workflow thread, using original user/assistant approval context. It found one missing approved management-tool input (D68 execution settings), restored compressed runtime/setup/UI/cleanup/source-independent/streaming/deadline/catalog acceptance details and added later-resolution pointers to prevent old “open” wording reopening topics. No new product decision or build authority was inferred; no application or live-system action occurred.

## October28, 2026 — implementation and push authorized

The user authorized starting the build and pushing to the existing GitHub repository, with no release. Provide developer-server instructions. Main MCP feature tests will happen together later in an isolated virtual environment with the same server inventory. Begin with the Rust backend foundation; preserve existing personal tools, credentials and client configuration. See D74 and the active development milestone.

### October28, 2026 — first developer backend implemented

New Rust source now provides ephemeral loopback HTTPS, protected read handlers, bounded safe discovery and private/encrypted store primitives.37 fixture/integration tests plus formatting/strict Clippy passed; a binary CLI HTTPS metadata/Ctrl-C smoke passed. No MCP server connection, personal config/credential/trust/Keychain change or release occurred. Native approval/token issuance, lasting identity, actual Keychain/provider/runtime execution, desktop UI and MCP sharing remain incomplete. Final independent review and authorized source push follow.
