# Historical task record before D73

October28, 2026. Preserved completed-pass/unfinished-task record. D73 consolidates all actual remaining engineering work into the current build-readiness.md worklist; old broad review tasks/counts are superseded, not new blockers or cancelled features. Use current milestones/to-do.md for live work.

# Fetch milestone to-do

Active milestone: [mcp-mvp-architecture](info.md). Architecture decisions and unresolved details remain in [docs/architecture/](../../docs/architecture/README.md).

## Active remaining decisions — October28

Use the canonical [remaining-decisions checklist](../../docs/architecture/open-questions.md#remaining-decisions-checklist). It preserves the seven-step order and separates user decisions from implementation work and future test evidence. The next [local OAuth HTTPS/trust proposal](../../docs/architecture/local-oauth-trust.md) awaits input. No new architecture choice is approved by this continuation request.

The sections below retain the pass/task record. Later decisions supersede earlier checkpoints; the canonical checklist determines current approval gaps. Application implementation and live tests remain paused.

## Latest MVP storage amendment

- [x] Record the user's decision to sideline retention and exclude PostgreSQL functionality from the initial MVP.
- [x] Audit actual versus planned dependencies and preserve required local metadata/encrypted-auth state without choosing a replacement.
- [x] Finish this amendment's documentation checks/evaluation and append/link pass records — checks passed, documentation ACCEPT.
- [x] Select the storage split with the user: local server config files plus the existing encrypted token/credential store.
- [ ] Review exact filenames/format within the D28 durable server-data tree, safe writes and encryption-key handling in the implementation/storage step; runtime-job and memory storage details remain open.

## Session handoff

- [x] Prepare [the full handoff](handoff.md) with the seven-step outline, APIs/structures, approvals, preferences and evidence references.
- [x] Link it from the milestone snapshot, project instructions/context, notes and architecture index.
- [x] Complete handoff validation/independent evaluation and append/link final pass records — ACCEPT, no material findings.
- [x] Resume architecture discussion from the handoff and latest storage decision — October 27; approved fields preserved and remaining policies still pending.
- [x] Verify/evaluate the resume documentation — independent ACCEPT; append/link inspected pass records.
- [x] Obtain user feedback on the full restart-policy bundle — approved October 27 with "Cool all good next" after clarification; D20 recorded.

## Initial outline and journal pass — completed

- [x] Inspect current records and obtain a read-only Planner inventory/gap audit.
- [x] Draft the remaining review checklist and distinguish accepted routes from incomplete/proposed surfaces.
- [x] Create the supplied history template, rolling journal and folder guides.
- [x] Validate documentation, exact template and initial history records.
- [x] Independently evaluate the documentation/approval boundaries — ACCEPT, no required fixes.
- [x] Append/link the inspected Planner and Orchestrator outcomes.
- [x] Append/link the Evaluator outcome and final integration; the active architecture milestone remains incomplete.

## Restart contract — approved October 27

- [x] Record the selected discussion order without approving new technical choices.
- [x] Draft the bounded restart request/result-fields proposal.
- [x] Finish documentation verification/evaluation and append/link this pass's outcomes — checks passed, documentation ACCEPT.
- [x] Obtain the user's decision on the restart path, optional body flag/default and result states/fields — approved with "Good".
- [x] Record the preference for one complete API contract per turn.
- [x] Draft the remaining restart contract in one reviewable proposal.
- [x] Obtain feedback on readiness/permissions/errors/envelope/deadline/overlap/current-run rules — approved in D20; PostgreSQL retention/recovery remain deferred.
- [x] Record this follow-up research/documentation verification and evaluation in the journal — checks passed; documentation ACCEPT after wording fixes.

## Ordinary server-information contract — approved (D23)

- [x] Finish/document the bounded whole-contract proposal in the existing GET section and inspect Planner findings; user reaffirmed route reuse.
- [x] Validate/evaluate the ordinary GET proposal — independent ACCEPT; pass evidence serialized.
- [x] Apply D21: move the existing server family to `/api/v1/servers`; keep setup discovery/installation and preserve GET query/restart behavior.
- [x] Verify/evaluate D21 path synchronization — independent ACCEPT; serialized/linked evidence.
- [x] Obtain user feedback on record fields, ordinary-query validation/order, envelope, permissions and errors — approved with "Kk good to me then you’re good to move on" (D23), after restart-required/next-steps clarification.
- [x] Confirm `next_steps` for all current/future application API JSON responses (D22).

## Authentication-start contract — general behavior approved (D27)

- [x] Inspect the read-only Planner report and document a bounded full proposal in the existing authenticate POST section.
- [x] Validate/evaluate shared convention/GET approval and auth proposal — independent ACCEPT after summary corrections; evidence serialized.
- [x] Research current MCP auth/token ownership and maintained examples before dependent contract changes; D24 scope recorded.
- [x] Clarify that local GitHub MCP RAM-only OAuth differs from persisted browser/CLI login; primary docs rechecked, no new mechanism selected.
- [x] Record D25 encrypted-store/readback and service-authenticated `2xx` success gate, including the endpoint clarification.
- [ ] Review exact test endpoints, wrapper-owned storage/handoff compatibility, refresh/recheck/cleanup and non-token verification before claiming integration support.
- [x] Verify/evaluate auth/storage research and contract update — independent ACCEPT, evidence serialized.
- [x] Obtain user feedback on reviewed validation/branches, one-flow repeats, 10-minute/provider expiry, errors and restart/disconnection behavior — "Everything else looks good" (D27); concrete auth mechanisms remain open.
- [x] Record per-server encrypted credential configuration files (D26), with D28 later selecting placement; filenames/format/key handling remain open.
- [x] Research GitHub CLI and maintained bridge alternatives; prepare a reviewable proposal and installation-only help prompt.
- [ ] Choose GitHub CLI integration/tool/API scope, installation ownership and fresh-login/refresh arrangement satisfying D25/D26.
- [x] Choose credential-file placement — D28 approves the outside-package `Fetch/servers/<server-id>/` tree; exact filenames/format/key handling remain open.
- [x] Verify/evaluate approval/storage/CLI-proposal pass — independent ACCEPT, evidence serialized.

## Common configuration PATCH — approved (D30)

- [x] Draft and independently evaluate the complete launch-field contract.
- [x] Research native settings conventions and clarify credential references versus values; independent documentation ACCEPT.
- [x] Obtain user decision: keep the three common launch fields and reviewed policies; defer D29 native-settings normalization outside MVP.
- Deferred/superseded: revising MVP config into a universal native-settings interface. User moved normalization/config-file mappings/settings schemas to a separate future scope (D30), not a completed implementation.
- [ ] Select safe argument/launcher validation, reference metadata, file-write and other implementation dependencies in their later steps.

## Credentials PATCH — base reviewed; verification details open

- [x] Inspect Planner report and draft the Settings-only full contract. Root selected proposed auth/restart busy rejection instead of the Planner’s alternate restart snapshot policy; no change to D20 assumed.
- [x] Verify/evaluate D30 approval and credentials proposal — independent documentation ACCEPT, no material findings; pass evidence serialized.
- [x] Review base manual-entry contract with the user; D31 corrects 200 to require the new stored credential test and a shared auth/save verification-failure code. User authorizes moving on.
- [ ] Resolve verifier endpoint/handoff, failure HTTP mapping/deadline, batch/removal/non-auth/no-op cases and saved-but-unverified retention/metadata before implementation; no unverified-200 exception approved.

## Installation-progress GET — approved (D36)

- [x] Inspect Planner report and draft the existing route's complete read contract.
- [x] Verify/evaluate D31 sync and the progress proposal — independent documentation ACCEPT after checkpoint-label corrections; evidence serialized.
- [x] Re-audit remaining progress-read decisions after D35; D34 installer/source/retry/basic Quit rules are established and not reopened.
- [x] Verify/evaluate D35 approval and refreshed remaining progress-read contract — independent documentation ACCEPT, no material findings; evidence serialized.
- [x] Obtain user approval of complete progress read — "good" (D36), including schema/enums/errors/guidance/polling and full same-run result lookup; exact state/storage mechanisms remain open.

## Installation-submission POST — approved (D34)

- [x] Inspect read-only Planner report and draft the existing POST’s whole contract without adding a new route or source snapshot token.
- [x] Verify/evaluate the submission proposal and checkpoint synchronization — independent documentation ACCEPT, no material findings; evidence serialized.
- [x] Obtain user approval of the presented submission contract — "yep good" (D34), including source capture/reuse. D36 subsequently approves separate progress refinements/full current-run lookup; concrete preparer mechanisms remain open.
- [ ] Resolve shared path/source identity, supported preparers, finite deadlines and safe owned-process/staging mechanics in the selected implementation/lifecycle discussion.

## Discovery GET — approved (D35)

- [x] Recheck current official harness global paths/formats/scopes/secret references before dependent planning; research inspected, no personal configs read.
- [x] Inspect Planner proposal and draft full discovery contract with explicit scope/path/format/entry/secret/disabled/error/UI-guidance policies.
- [x] Verify/evaluate D34 approval, research and discovery proposal — independent ACCEPT after correcting stale retry-policy wording; evidence serialized.
- [x] Obtain user approval on exact presented discovery contract — "good next" (D35); concrete reader/launch/path mechanisms remain open; D36 subsequently approves progress read.

## Step 2 — tool sharing, current

- [x] Record D37: complete raw Tool objects, duplicate-name rejection and no incomplete-list publication; supersede six-field projection/extra metadata validation.
- [x] Independently verify/evaluate D37 documentation and current checkpoints — verifier/evaluator ACCEPT, no material findings; no runtime tests.

- [x] Research current MCP tool definitions/pagination/caching/origin boundaries and maintained aggregation designs before dependent planning; no runtime/credential tests.
- [x] Inspect Planner report and draft the first complete tool-listing API contract, including origin identity, caller visibility and freshness/partial/error behavior. D37 subsequently supersedes its metadata projection.
- [x] Independently verify/evaluate D36 step transition, research and the first tool-sharing proposal; ACCEPT after stale status corrections, evidence serialized.
- [x] Obtain user input on revised listing — "sounds good, next" (D38), preserving D37.
- [x] Research installed-tool invocation protocol/error/validation/cancellation/interaction boundaries and maintained gateway implementations.
- [x] Inspect Planner and draft the complete installed-tool invocation contract with raw terminal result/structured uncertainty and explicit step3 dependencies.
- [x] Independently verify/evaluate D38 approval/current call proposal — verifier/evaluator ACCEPT, no material findings.
- [x] Record D39: raw model argument object and raw terminal ToolResult selected; routing outside arguments.
- [x] Independently check/evaluate D39 and revised proposal/current records — verifier/evaluator ACCEPT, no material findings; no runtime tests.
- [x] Record explicit raw POST body and target MCP/server-ID reaffirmation; preserve D39 without blanket route approval.
- [x] Record D40 model-visible wrapper disclosure, preserving upstream definitions/raw input/result.
- [x] Recheck/clarify normal MCP result text/content versus structured data and pre-call descriptions; no Fetch mapping chosen.
- [x] Independently verify/evaluate D40 research/records and proposed disclosure wording — verifier/evaluator ACCEPT, no findings; no model/runtime tests.
- [x] Earlier D41 notice/response_context decision recorded; superseded by D43 raw-only upstream and D44 next_steps reuse.
- [x] Research full MCP payload/schema options, native-media and text-fallback tradeoffs before dependent planning.
- [x] Inspect Planner and draft complete MCP response/schema/fallback/native-media proposal with duplication cost explicit.
- [x] Independently verify/evaluate D41 approval and complete MCP presentation proposal — verifier/evaluator ACCEPT, no findings; no client/model/runtime tests.
- [x] D42 native result/one-notice decision recorded; D43 later removes the notice. Rejected full-envelope/media/schema duplication remains superseded.
- [x] Independently verify/evaluate D42 native mapping/current records — verifier/evaluator ACCEPT after approval-label fixes, no remaining findings; no runtime/model tests.
- [x] Research official error/output-schema rules and maintained SDK isError validation behavior; inspect Planner and draft whole failure/protocol proposal.
- [x] Independently verify/evaluate the complete Fetch-only/protocol failure proposal — verifier/evaluator ACCEPT, no findings; no runtime/client/model tests.
- [x] D43 approves failure mapping with raw-only upstream returns (no added Fetch notice), Fetch-owned explanatory text and native protocol handling.
- [x] D44 reuses next_steps and removes response_context.
- [x] Independently verify/evaluate D43/D44 synchronization — verifier/evaluator ACCEPT after snapshot-label correction, no remaining findings.
- [x] Research current MCP naming recommendations and maintained prefix/collision implementations before dependent planning.
- [x] Inspect Planner and draft complete tool-name alias/collision proposal with stability/exception/canonical-data boundaries.
- [x] Independently verify/evaluate naming proposal and D43/D44 synchronization — documentation ACCEPT, no runtime tests.
- [x] D45 approves the complete presented tool-name alias/collision policy; exact encoding/host limits/storage mechanisms remain later review.
- [x] Research official current/legacy MCP HTTP semantics, three-client transport/OAuth evidence, Origin/Host and issuer-TLS boundary before dependent planning.
- [x] Inspect Planner and draft complete localhost /mcp connection proposal with explicit dual-era methods/sessions/auth/security/reconnect and issuer-TLS dependency.
- [x] Independently verify/evaluate global/project notes and isolation instructions plus D45/D46/full connection proposal — ACCEPT, no findings; no live tests.
- [x] Explain the two dated protocol revisions and their actual connection/session/notification/interaction/resumption differences using current primary sources.
- [x] D47 approves the presented localhost connection contract after protocol/API/tool routing/list clarifications; exact port/SDK/OAuth issuer/TLS/client/execution dependencies remain open.
- [x] Plan/draft the full Fetch-owned management-tool contract; independent verifier ACCEPT, no runtime tests.
- [x] Independently evaluate the management-tool proposal — ACCEPT, no material findings; no runtime tests.
- [x] D48 confirms descriptive tool coverage for current/future agent-accessible product APIs; eight is not a closed permanent inventory. Preserve Settings-only manual credential exception. Independent Evaluator ACCEPT, no findings; no runtime tests.
- [x] D49 approves the presented management-tool names/input/API/result/permission/lifecycle contract after D48; exact unshown mechanisms remain later review.
- [ ] Verify actual supported-client/model delivery after implementation authorization.
- [x] Inspect Planner and existing invocation research; refresh whole HTTP tool-call remainder with exact selector round-trip policy, current D45/D47/D49 approvals and explicit capability boundary.
- [x] Independently verify/evaluate refreshed HTTP call proposal and D49 sync — ACCEPT after stale context correction; remaining API inventory also ACCEPT; no runtime tests.
- [x] D50 approves the presented HTTP call route/input/authority/forwarding/outer outcomes/errors/lifecycle contract; ordinary execution mechanisms remain next.
- [x] Inspect dedicated ordinary execution research: protocol cancellation has no guaranteed acknowledgement/final reply; shared wrappers share application state; maintained gateway lessons recorded.
- [x] Inspect Planner and draft complete execution sharing/admission/deadline/cancellation/recovery/state bundle with tradeoffs and proposed GET field.
- [x] Independently verify execution proposal/D50 sync — Tester ACCEPT after stale summary/count correction; no runtime tests.
- [x] Independently evaluate execution proposal — ACCEPT after snapshot correction and legacy/modern disconnect clarification; no runtime tests.
- [x] D51 requires queued execution with concurrency; prior no-queue proposal superseded, fixed120 draft withdrawn after timeout-discovery question. Independent Evaluator ACCEPT on scope/factual boundary; no runtime tests.
- [x] D52 selects durable per-tool feature/settings records applied by Fetch; D53 selects120-second fallback, not hard cap.
- [x] Inspect queue Planner and D52/D53 follow-up; draft full revised queued execution/settings/API proposal with selected requirements distinguished.
- [x] Independently verify/evaluate revised queued execution/settings/API contract — Tester/Evaluator ACCEPT, no findings; no runtime tests.
- [x] D54 confirms independently configurable per-server default timeout, initially120 seconds, through existing config interfaces.
- [x] D55 selects automatic installation timeout initialization and fresh dispatch-relative Fetch execution deadlines, preserving raw arguments and separate queue wait.
- [x] Obtain user input on the presented queue/concurrency/default/config/hot-apply/GET-state/restart/cancellation bundle — D68. Unshown schema/bounds remain open.
- [x] Research current/legacy capability distinctions, optional progress, catalog change delivery, native cache envelopes and annotation compatibility before dependent scope planning.
- [x] Inspect capability Planner and draft full initial scope proposal preserving D47 catalog channels and raw data.
- [x] Independently verify/evaluate initial capability proposal/D57 sync — Tester/Evaluator ACCEPT, no findings; no runtime tests.
- [x] Supersede blanket initial exclusions under D58; D66 subsequently adopts reviewed progress/client-interaction relay. Remaining native features receive explicit review rather than blanket exclusion.
- [x] Inspect available tool declarations and safe global server metadata; record practical19/18-entry inventory without credential values or live calls. Independent Evaluator ACCEPT, no findings; runtime compatibility untested.
- [x] Review remote HTTP/SSE upstream gap — D61 selects adapters, D63 remote APIs, D65 protocol/listing behavior. Remaining resource/prompt/Tasks scope is tracked separately.
- [x] Inspect/reuse authentication/TLS findings and complete focused primary-source trust research before the next proposal; scope decisions D61–D68 are preserved. [The trust proposal](../../docs/architecture/local-oauth-trust.md) awaits input.
- [ ] Finish exact naming/wire/capability mechanisms; D45 naming, D47 presented connection policy and D49 presented management mapping are already approved.

## UI setup baseline — confirmed (D32)

- [x] Record agent-free UI installation as the base method; `next_steps` is agent-only guidance, not UI status/workflow input. D22 response field remains.
- [x] Verify/evaluate D32 documentation — independent ACCEPT, no material findings; no live UI tests.
- [ ] Define exact UI/post-install branches and first-setup Settings authority in steps 4/6, using structured progress/server states and backend-owned prerequisites.
- [ ] Once authorized, verify UI-only setup and that changing agent guidance alone does not alter UI state/sequence; optional agent setup uses the same APIs.

## Shared app/guidance workflow — confirmed (D33)

- [x] Record consistency rule for all prior/current/future workflows and add the cross-route table covering all existing operations, preserving approval boundaries.
- [x] Verify/evaluate D33 documentation — independent ACCEPT, no material findings; pass evidence serialized.
- [ ] As exact workflows are decided/implemented, keep UI sequencing and agent guidance aligned on prerequisites/order/actions/outcomes/retries and explicit user handoffs; verify both against the same structured states.

## Unfinished design work — selected discussion order

- [x] 1. Complete the initial eight setup/server-management route review: D20/D23/D27/D30/D31/D34/D35/D36 recorded. Concrete verifier/implementation refinements remain explicit later work, not blanket implementation approval.
- [ ] 2. Finish remaining capability/exact wire mechanisms and GitHub CLI scope. Listing/raw arguments/results/presentation/alias/connection/management/call policy are selected in D38/D39/D43/D44/D45/D47/D49/D50; do not reopen superseded D40–D42 notice fields. D68 approves the presented ordinary execution contract; only specific unshown schema/bounds/resource-admission mechanisms remain.
- [x] 3. Presented ordinary queued execution/settings contract approved in D68; track unshown profile/GET encoding, numeric bounds and native resource/prompt admission separately.
- [ ] 4. Design harness OAuth, supported provider-auth integrations, Settings authority and encrypted-store key protection.
- [ ] 5. Choose Rust modules/dependencies, supported config/launcher forms, preparation, identities, local saved records, job ownership (PostgreSQL/schema/migrations deferred) and remaining startup/shutdown behavior.
- [ ] 6. Complete first-client onboarding, connection setup, cleanup verification and selected management controls; clarify memory/website release sequencing without splitting Fetch or bypassing its APIs.
- [ ] 7. Agree meaningful end-to-end checks and macOS distribution prerequisites, signing, explicit updates, license and release destination.
- [ ] Obtain explicit implementation authorization after the required route/architecture decisions.

## Future process-level concurrency — D56 desired, D57 deferred for now

- [x] Record configurable total running processes per MCP with capacity-aware queued-call distribution as a desired feature.
- [x] D57 explicitly defers this feature for now; retain it as future scope and keep one managed process per logical MCP.
- [ ] Review exact process settings/API/tool fields, limits, queue distribution, state/session/credential affinity, lifecycle/restart/cancellation and isolated verification before implementation.
- [ ] Preserve separate D51 queue/call-concurrency work; do not substitute a call limit for process count or spawn extra processes under the current unamended call contract.

## Preserved unfinished later work

- [ ] Once authorized, implement approved APIs and verify real HTTP behavior before their UI/MCP interfaces.
- [ ] Independently test/evaluate each accepted slice; fix/retest material findings.
- [ ] Verify the actual harness/authentication/package/lifecycle flow and fresh macOS installation before release.
- [ ] Package/publish only after the user selects the delivery details and authorizes release.

- [ ] Future separate scope, outside this MVP: standardize individual servers’ native settings/config files through supported schemas/mappings, if the user resumes it (D30).

## Superseded or deferred approaches

- Initial-MVP PostgreSQL setup/connections, database-host Tailscale prerequisites, schemas/migrations and the unapproved 24-hour retained-result/recovery proposal: deferred by the latest user instruction; no PostgreSQL functionality in this MVP. Existing local metadata/credential needs are not cancelled. Future shared-migration work remains deferred rather than completed.

- Held-open restart POST: superseded by the user's immediate `202` plus separate completion decision.
- Response-only restart-ID matching: not selected; the user chose explicit `restart_id` query filtering.
- Catalogue, per-tool import selection, Grok/Muse integration, Windows/Linux and hosted deployment: deferred from the current first MCP release direction; no silent implementation.
- MCP configuration history/rollback and the memory graph: outside the current MVP. Source Git remains required; shared PostgreSQL migrations are deferred with all initial-MVP PostgreSQL functionality.
- Restore/continue the removed October 25 draft: prohibited until the user explicitly authorizes it. No restoration occurred.

## Project notes and isolated testing instructions

- [x] Add global project-scoped notes.md initialization default, preserving existing notes filenames/content.
- [x] Record isolated new-MCP testing rule in global and Fetch AGENTS.md plus project NOTES.md (D46).
- [ ] Select a suitable isolated environment/provider and disposable test fixtures/accounts in the testing plan before any authorized live MCP connection tests.

## Shared MCP handlers — D59

- [x] Record accepted shared-function/runner organization at /mcp, preserving native method/stream/revision handling and backend permissions.
- [x] Review presented native interaction/resource/prompt scope and explicit Tasks/live-content deferrals — D66/D67. Exact SDK/ownership/wire mechanisms and D58 isolated evidence remain open; no separate URL per protocol method.
- [x] Review presented remote API/lifecycle and interaction surfaces — D63/D65 approve remote contracts; D66 keeps direct REST terminal-only and native interaction UI in the caller. Exact implementation mechanisms remain separate.

- [x] Obtain feedback on automatic /mcp streaming defaults — D60 approves streaming tool calls, JSON quick reads and no user toggle; native exceptions and raw results remain.

## Remote upstream review after D60

- [x] Research and document HTTP/SSE upstream scope, source constraints and existing-contract impact without changing selected local-only policy.
- [x] Obtain user input on HTTP/SSE upstream inclusion — D61 selects separate transport functions routed by the shared runner, keeping Fetch client-facing localhost.
- [x] Review the complete existing-route remote API/lifecycle amendment — D63 approves presented behavior and forced-recovery tradeoff. Exact protocol/destination/provider/retirement mechanisms remain separately open.
- [ ] Verify selected remote support only in an authorized D46 isolated environment before any authorized personal rollout.

## Internal tool routing — D62

- [x] Record per-tool Fetch connection-type configuration and internal call-context dispatch, preserving raw arguments/definitions.
- [ ] Resolve exact routing field/reference representation and consistency/update mechanics alongside the existing API/config/dispatch amendments; no new model argument or public route implied.

- [x] Record explicit shared-runner handling for all three selected transports under D61/D62; no stdio-only or config-label-only interpretation.
- [ ] After implementation is authorized and exact contracts are selected, verify shared-runner dispatch/raw results/authority/deadlines across stdio, Streamable HTTP and legacy HTTP+SSE in D46 isolation.

## Combined remote API amendment — presented behavior approved (D63)

- [x] Inspect completed research and Planner report; draft one combined import/config/GET/auth/reconnect proposal after D62.
- [x] Complete independent semantic verification/evaluation — root fixed the missing matching config-tool connection.headers field; Evaluator ACCEPT with no remaining findings.
- [x] Obtain feedback on the presented remote-api-amendment.md contract — user "good" after the complete proposal and TLDR; D63 selects presented behavior/schema/defaults.
- [x] Resolve presented upstream revision/header-annotation/catalog policy — D64/D65.
- [ ] Resolve remaining destination/TLS, provider verification and bounded-resource mechanisms before implementation acceptance.

- [x] Synchronize and independently evaluate D63 approval boundaries — ACCEPT; remaining protocol/security/provider/storage/resource mechanisms stay open.

## Required protocol adaptations after D63 — presented policy reviewed through D65

- [x] Research actual spec/official SDK filtering and required result-marker behavior before dependent planning.
- [x] Plan/draft the complete narrow compatibility bundle, preserving canonical raw data and explicit failure tradeoffs.
- [x] Independently evaluate the protocol-adapter proposal — ACCEPT with no findings; new D37/D43 exceptions remain proposed.
- [x] Obtain user input on marker/publication behavior — D64 approves native marker-only exception and replaces whole-catalog failure with explicitly incomplete valid-tool availability.
- [x] Review partial-origin/rejection schema, native metadata/listing-tool mapping, upstream targets/setup and0/private defaults — D65 accepts presented behavior; actual model visibility remains unverified.
- [ ] After authorization/selection, prove raw capture/rejection observability, unknown-field/numeric fidelity and host/version paths in D46 isolation.

- [x] Synchronize/evaluate D64 partial approval and revised availability direction — ACCEPT; D65 subsequently resolves the presented partial-list/native metadata and version/cache defaults. SDK/client evidence remains open.

## Catalog status and protocol views — presented behavior approved (D65)

- [x] Inspect D64, prior SDK evidence and current/legacy metadata schemas; plan/draft exact partial-list/API/native/tool mapping with view-sensitive rejection.
- [x] Finish independent semantic verification/evaluation — both ACCEPT, no material findings; exact proposed fields/defaults still await input.
- [x] Obtain input on partial/rejected_tools, view input/defaults/native metadata, initial upstream targets and private/zero-TTL catalog defaults — D65 approves the presented bundle.
- [ ] After authorization/selection, verify mixed/all-rejected/true-empty/failed traversal/duplicates/hidden diagnostics, source-versus-export views and actual host/model metadata handling under D46.

- [x] Synchronize/evaluate D65 approval and clarify return locations — ACCEPT; SDK/client/model delivery remains unverified.

## Native progress and interaction relay — presented behavior approved (D66)

- [x] Research legacy ownership/capability and modern MRTR/control-state constraints before dependent planning.
- [x] Inspect Planner and independent semantic verifier reports; draft one native-only relay contract with no extra REST/UI/model fallback.
- [x] Complete final independent evaluation — ACCEPT with no findings; all new native relay clauses remain proposed.
- [x] Obtain user input on presented native progress/roots/sampling/elicitation/continuation, legacy capability limits and original-deadline behavior — D66 approval plus endpoint/function integration clarification.
- [x] Review native resources/prompts and explicit live-content/Tasks deferrals — D67; remaining queue/cancellation/auth mechanisms remain separate.
- [ ] After implementation authorization/selection, verify real caller/revision/decline/continuation/isolation paths under D46 before claiming everyday compatibility.

- [x] Synchronize/evaluate D66 approval and endpoint/function integration explanation — ACCEPT; exact SDK/handle/resource/client evidence and other capabilities remain open.

## Native resources/prompts and remaining scope — presented behavior approved (D67)

- [x] Inspect dedicated primary/maintained-source research and Planner report for URI/template/subresource/prompt integrity.
- [x] Draft one native resource/prompt/completion contract with routing/errors/budgets/events and explicit proposed content-subscription/Tasks deferrals under D58.
- [x] Complete semantic verification/final evaluation — root fixed required modern resource-read framing/cache fields; Evaluator ACCEPT with no remaining findings.
- [x] Obtain user input on the presented resource/prompt/completion and explicit deferral bundle — D67, "Cool" then "Next"; exact unshown wire fields and runtime mechanisms stay open.
- [ ] After authorization/selection, prove actual client URI/template/alias/raw-link/subresource/permission/interaction/event behavior and daily-workflow adequacy under D46.

## Ordinary execution review — presented contract approved (D68)

- [x] Obtain input on the presented ordinary queue/concurrency/budget/config/GET-state/cancel/restart bundle — D68; D63/D66 preserved.
- [ ] Resolve GitHub CLI named operation scope and storage/renewal with the Step4 auth decision; no arbitrary shell/CLI tool approval.
- [x] Independently evaluate D67 approval boundaries and execution checkpoint — ACCEPT; local documentation checks pass.

## Deduplicate future architecture review — October28

- [x] Replace stale broad open questions with selected-versus-missing decision register; preserve immutable history and specific unresolved dependencies.
- [ ] Resolve specific remaining auth/trust/Settings/key/provider choices without reapproving D10/D24–31.
- [ ] Resolve exact unshown execution/profile/GET wire/bounds and native resource/prompt admission before affected implementation.
- [x] Independently evaluate D68 synchronization and deduplicated current register — ACCEPT after stale-checkpoint corrections; no runtime tests.

## Remaining checklist continuation — October28

- [x] Inspect read-only Planner and dedicated Researcher reports; preserve seven-step order and settled/deferred scope.
- [x] Add the canonical remaining-decisions checklist, separate implementation/evidence work and draft the specific local HTTPS/trust proposal.
- [x] Correct affected stale current-status summaries; preserve immutable history and all specific open work.
- [x] Check local Markdown links/anchors and JSON examples —295 files/647 links/31 examples before appended pass records.
- [x] Complete independent documentation evaluation — ACCEPT, no material findings;298 files/650 local links/31 JSON examples passed before final journal append.
- [x] Finish root verification of final journal links —300 Markdown files/659 local links and anchors/31 JSON examples and exact history template pass.
- [x] Obtain user input on presented local HTTPS/certificate direction — D70; exact key/lifecycle/client mechanics and complete OAuth endpoints remain separately tracked.

## Plain-language preference follow-up — October28

- [x] Strengthen the existing communication preference in global AGENTS.md; explain practical meaning and user effort first, using familiar words.
- [x] Verify the requested instruction edit and preserve existing global settings. This is not a product decision; the local HTTPS/trust proposal remains open in the canonical checklist.

## Account-free browser connection and actual MCP comparison — October28

- [x] Record D69: no Fetch account/signup/password; apps receive their own Fetch token through local browser connection; keep setup simple.
- [x] Update the canonical checklist without adopting the proposed HTTPS trust design or a localhost-HTTP exception.
- [x] Inspect dedicated research comparing actual MCP approaches and root's Apple/Claude/Node procedure evidence; record sources/limits.
- [x] Finish documentation verification —301 Markdown files/665 local links and anchors/31 JSON examples passed; independent Evaluator ACCEPT after fixing two stale current account assumptions; final history links/fields checked.
- [x] Review presented local HTTPS and app-connection/token contract — D70/D71; implementation/evidence and unshown mechanisms remain separate.

## Certificate effort clarification — October28

- [x] Document estimated creation versus browser/client verification work and distinguish certificate trust from a successful token/tool/renewal flow.
- [ ] Actual isolated certificate/client proof remains unrun and separately gated by the selected test plan and authorization; no new approval from the estimate request.

## D70 approval and complete app-connection proposal — October28

- [x] Record scoped local HTTPS/certificate direction approval; preserve no-account flow and all implementation/live-test boundaries.
- [x] Inspect dedicated registration/renewal/authority research, read-only permission inventory and dependent complete Planner recommendations.
- [x] Draft the full app-authorization/token route/permissions/Mac-confirmation/lifecycle/default/error proposal without marking it approved.
- [x] Verify/evaluate D70 and the complete proposal — independent ACCEPT after the API scheme-status correction;309 files/689 local links and anchors/31 JSON examples passed before final history append.
- [x] Complete final root check —316 Markdown files/698 local links and anchors/31 JSON examples; D70 unique and next contract proposed.
- [x] Obtain user input on presented app-connection contract — D71; exact unshown mechanisms and future connection-administration UI/API/Settings/key/provider work remain open.

## D71 approval and next Settings/key review — October28

- [x] Record exact presented app-connection approval and close its row in the canonical checklist without adopting unshown linked details.
- [x] Inspect dedicated Mac Keychain/Settings research and dependent Planner recommendations before drafting the complete next proposal.
- [x] Draft complete proposed Settings/key/native authority/unlock/recovery contract on the existing credential API; all new mechanisms await input.
- [x] Verify/evaluate D71/D72 and next review; independent ACCEPT after current-context fix and ACCEPT for the provider amendment, with318 files/721 links/31 JSON checks before append.

## D72 shown Settings/Keychain approval — October28

- [x] Record the user's approval of the shown next direction; distinguish unshown mechanism candidates and avoid another broad review.
- [ ] Resolve precise D31 provider verification/failure policy next; keep GitHub CLI scope/ownership and other concrete provider bindings explicit.
- [x] Serialize specialist/review evidence and checked history fields/links for D71/D72.

## Next provider verification amendment — October28

- [x] Inspect dedicated provider-verifier primary research and the dependent Planner report; preserve D25/D31 and no new test route.
- [x] Draft one complete amendment covering verifier support, total deadline, batch/no-op/removal/non-auth, safe failure/persistence and explicit proposed D31 exception.
- [x] Verify/evaluate new amendment — independent ACCEPT, no material findings;318 files/721 local links and anchors/31 JSON examples before append; pass evidence serialized.
- [ ] Obtain user input on the next amendment; no exception/default/failure policy accepted from earlier D71/D72 approvals.

## Remaining-decision count — October28

- [x] Count canonical open groups after D72 —10, including provider verification, plus separate implementation authorization.
- [x] Distinguish group count from individual choices, review turns, internal work and future test execution; retain all unfinished entries.
