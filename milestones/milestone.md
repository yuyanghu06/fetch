# Fetch milestone snapshot

## November 3 — shared authentication router accepted

The user clarified selection uses imported command, arguments and environment
requirements, not README interpretation. Remote defaults to web OAuth; shared
handlers cover API credentials and supported CLI/local helpers. A selected-flow
failure is returned to the user without a fallback login method. Existing public
contracts, account isolation, encrypted readback/proof and responsiveness remain.
Independent Planner completed the bounded common OAuth/dispatcher plan. Coder
implemented private configuration dispatch and common registration/URL/token
requests; Orchestrator inspected the diff. Independent Tester/Evaluator ACCEPT,
99 unique fixture checks pass and locked native binary compilation succeeds
(0.74s). Initial stale alias fixture failure was corrected in test setup only;
no production alias behavior changed. No live provider/helper/native/client
acceptance is claimed. Installed
Google1.11.3 has no storage hook; the user explicitly keeps its encrypted bridge
and integration pending while the common router is completed. Multiple instances
of the same MCP must retain separate methods/credentials/accounts. The existing
same-source installation reuse remains unchanged. User requests a behavior
walkthrough after routing completion. Source/evidence delivered as4f18905;
live remote and local commit matched. Pre-existing documentation edits remain
uncommitted. [Delivery pass](history/2026-11-03_16-55-36-339850-orchestrator-deliver-shared-router.md).
[Verification evidence](shared-mcp-authentication/evidence.md).
[Tester pass](history/2026-11-03_16-54-04-913350-tester-shared-router-fixtures.md).
[Orchestrator pass](history/2026-11-03_16-54-04-913622-orchestrator-shared-router-native-build.md).
[Evaluator pass](history/2026-11-03_16-54-04-913739-evaluator-accept-shared-router.md).
[Implementation pass](history/2026-11-03_16-49-58-499042-coder-shared-authentication-router.md).
[Research and user scope decisions](history/2026-11-03_16-48-24-958601-researcher-google-helper-and-router-scope.md).
[Planner pass](history/2026-11-03_16-46-28-931238-planner-shared-authentication-dispatch.md).



## November 3 — server-header × ready for user testing

Delete API/tool delivered as64cff43. The user-requested shared × now sits beside
every server disclosure and invokes that DELETE operation. Confirmed removal
clears relevant cached state; busy/pending and uncertain-response handling avoid
duplicate deletion. Independent source ACCEPT; native binary build passes and
14 existing UI-handler regression checks pass. No actual app/window/deletion or
personal-state operation ran. [Evidence](history/2026-11-03_16-20-25-000001-orchestrator-delete-button-build-and-ui-checks.md).

Shared authentication remains the next implementation: user emphasized a common
standard harness/MCP web-auth rewrite for all servers, with no Google-specific
route or login engine. Completed upstream/helper research is not live auth proof.

## November 3 — delete server, then shared authentication

Active milestone: [delete-server](delete-server/info.md). User delegates clean
implementation using existing conventions. DELETE /api/v1/servers/{server_id}
will use shared per-server management authority, reject busy work, stop the idle
connection and remove only Fetch-owned state/private packages. HTTP/MCP share
the backend; source files, scripts, runtimes and unrelated accounts remain.
Plan accepted; HTTP/MCP adapters are implemented and inspected; backend cleanup/
lifecycle is implemented; cancellation-owned gates and shutdown handling corrected.
Tester and Evaluator final source ACCEPT, no material findings. Scoped Git
delivery is next. No build/runtime or actual deletion ran.
[Plan evidence](history/2026-11-03_15-11-28-000001-planner-delete-server-contract-and-authentication-queue.md).
[Backend source](history/2026-11-03_15-27-24-000001-coder-safe-server-deletion-backend.md).
[Initial source review](history/2026-11-03_15-27-24-000002-tester-delete-cancellation-finding.md).
[Adapter source evidence](history/2026-11-03_15-16-40-000001-coder-delete-http-and-mcp-adapters.md).

User explicitly queued the shared asynchronous MCP authentication implementation
after deletion delivery. Preserve that full request: pinned upstream reuse,
standard HTTP OAuth including existing application registrations, URL-mode
elicitation/server-owned callbacks, local login helpers with Google first,
independent encrypted account storage/renewal, shared async ownership/cancel/
expiry/UI responsiveness, Railway/Linear compatibility, actual verification and
attribution. Read current AGENTS/November3 notes/authentication-engine first.
No authentication implementation starts before deletion is delivered.

- [Guard/shutdown correction and Tester acceptance](history/2026-11-03_15-32-49-000001-coder-delete-owned-worker-and-tester-acceptance.md)
- [Final deletion source acceptance](history/2026-11-03_15-32-49-000002-evaluator-delete-source-acceptance.md)

## November 3 — installation should leave usable servers running

Active focused milestone: [install-connect](install-connect/info.md). User selects
installation's final stage invoking existing Restart / Connect, retaining usable
connections and reporting authentication.status not_required for credential-free
success without a known login requirement. Keep GET servers status; unknown alone
must not prompt login. Missing/rejected access requires setup; unrelated start
errors remain runtime errors. No automatic login or tool calls. Source implemented;
independent Tester's identity/generation finding is corrected and re-reviewed
ACCEPT. Final Evaluator's late-response gap is also fixed: provider rejection
after timeout/cancel updates status without changing the call's original outcome.
Tester and Evaluator final source ACCEPT; no material findings remain. Source is
committed/pushed as fcf6c34. No builds, tests
or runtime/provider/MCP actions ran for this change.

- [Implementation](history/2026-11-03_14-55-20-000001-coder-install-connect-source.md)
- [Independent source finding](history/2026-11-03_14-55-20-000002-tester-install-connect-generation-finding.md)

The installed-discovery fix was rebased onto newer upstream source, independently
re-reviewed ACCEPT and pushed as9eddd00. Its build/native/Figma evidence remains
pending. [Delivery and new plan](history/2026-11-03_14-45-40-000001-orchestrator-discovery-delivery-and-install-connect-plan.md).

## November 3 — source layout explained and requirement recorded

Current pass: read-only source inspection plus documentation. The user clarified
that `src/api` should hold HTTP route handlers grouped by URL path, while other
folders may contain reusable backend functions. Current implementation differs:
route handlers are combined in `api/runtime.rs` and some HTTP endpoints live in
`oauth`/`mcp`. No application source was reorganized; no builds/tests/runtime
operations ran. Next layout action is a bounded source refactor preserving
existing routes and shared backend behavior, when requested. Existing MVP and
installed-discovery work remains unfinished as recorded below.
[Pass evidence](history/2026-11-03_14-45-09-644674-orchestrator-source-layout-clarification.md).

## November 3 — newer GitHub work discovered during delivery

- [Late-response correction and Tester acceptance](history/2026-11-03_15-01-26-000001-coder-late-auth-errors-and-tester-recheck.md)
- [Final D83 source acceptance](history/2026-11-03_15-01-26-000002-evaluator-install-connect-source-acceptance.md)
- [Actual source delivery](history/2026-11-03_15-02-49-000001-orchestrator-deliver-install-connect-source.md)

- [Identity correction](history/2026-11-03_14-57-40-000001-coder-bind-auth-rejections-to-connection.md)
- [Corrected source acceptance](history/2026-11-03_14-57-40-000002-tester-install-connect-source-acceptance.md)
- [Final review late-response finding](history/2026-11-03_14-59-42-000001-evaluator-late-authentication-rejection-finding.md)

Pushing the installed-discovery fix was rejected because GitHub had twelve newer
commits through a7a6e0d. Fetching revealed D81/D82 work absent from the initial
local checkout: shared UI/theme, documented remote/bridge import support,
issuer-bound generic OAuth, checked credential groups, shared verification data,
parallel authentication and further encrypted-document/token handling. Incoming
records include substantially newer fixture evidence (including110 combined
checks for D82); those records supersede the pre-fetch local readiness summary.
Real native/provider/client acceptance is still not established by those fixtures.

The feature is being rebased onto that work, preserving its auth request tracking
and shared components. The new installed checkbox now lives in components.rs
with styling/opacity in theme.rs. Initial immutable review records describe the
older checkout and earlier implementation; this snapshot corrects their current
applicability. No tests/build/apps/provider/MCP/personal-state actions were run
for this integration. Newly discovered D81 is recorded without treating this
small source fix as an instruction to resume the entire runtime acceptance task.

## November 3 — already-installed discovery rows

Active focused milestone: [installed discovery](installed-discovery/info.md).
The user requested checked, gray discovery rows for definitions already imported
into this Fetch instance. Backend matching will reuse exact saved source path/name;
UI default/manual/bulk selection and submission will exclude those rows. Distinct
accounts/configuration sources remain separate and backend installation reuse is
preserved. HTTP and MCP discovery must agree. Successful progress will update
displayed rows without rereading source files. Source is implemented in discovery,
the HTTP/MCP adapters and UI. A common eligibility rule excludes installed rows
from defaults, toggles, Select all, count and submitted payload; successful progress
updates rows and preserves failed-new-row retries. Checked disabled rows use a
shared style to dim label text as well as the box. Independent Tester and
Evaluator source/documentation reviews ACCEPT with no material findings. No new
build/runtime result exists; source-only commit/push integration is next.

The user's separate Vercel/Miro/HF questions were answered from recorded source
limits; they do not expand this fix into provider or importer support work.
No build/test/formatter/linter/validation, app/provider/MCP, personal state or
release action ran or is newly authorized.
[Plan evidence](history/2026-11-03_14-27-45-000001-planner-already-installed-discovery.md).
[Implementation evidence](history/2026-11-03_14-31-55-000001-coder-installed-discovery-state.md).
[Independent source review](history/2026-11-03_14-35-00-000001-tester-installed-discovery-source-review.md).
[Final source acceptance](history/2026-11-03_14-34-53-000001-evaluator-installed-discovery-acceptance.md).

## Historical November 3 pre-fetch local-checkout review

This review inspected local 1276516 before fetching newer GitHub commits.
Its provider/import/UI/test conclusions are superseded by the incoming October
29–31 records below; it does not describe the integrated repository.

Active goal: finish the selected MCP MVP so Codex, OpenCode and Claude Code can
each use one Fetch connection for the user's imported servers. Read-only source
inspection confirms the shared gateway, client authorization, transports and
management machinery exist; the four backend gaps in
[MVP_GAPS.md](../docs/testing/MVP_GAPS.md) still prevent claiming general everyday
inventory coverage. Remaining shared UI extraction is a separate required item.

Known provider setup is limited to Tavily/GitHub verification and Linear/Railway
browser integrations. Importing definitions deliberately does not copy their
credentials. General remote OAuth, broader safe launcher/URL imports and correct
credential batch/removal outcomes remain unfinished. Unknown authentication
does not by itself prove a no-auth server cannot connect. Railway local children
still need explicit Restart after background token refresh.

Historical verification remains 37 passing earlier-foundation tests plus later
successful native binary compilation/source reviews; no new runtime evidence
was produced. The native/provider/client acceptance roster remains unrun and its
isolation unprovisioned. No build, test, app/provider/MCP connection, personal-state
change, packaging or release occurred. Starting checkout was clean main at
1276516, matching its local origin/main tracking reference; no remote query made.

- [Connector source-review evidence](history/2026-11-03_14-21-37-000001-researcher-connector-readiness-review.md)
- [Plan, pages and testing-evidence review](history/2026-11-03_14-21-37-000002-planner-plan-pages-and-test-evidence.md)
- [Independent summary acceptance and integration](history/2026-11-03_14-21-37-000003-evaluator-status-summary-acceptance.md)

All four native pages exist: Servers, Install MCPs, Connect apps and Settings,
plus browser approval/callback pages. Connect apps provides setup/certificate
guidance rather than configuring clients automatically. The 153 Figma specimens
are design/source references, not runtime-tested pages. The original seven
discussion areas now map to the consolidated build plan: source spans all core
areas, authentication/import remain incomplete and testing/delivery acceptance
is outstanding. Independent Evaluator ACCEPT covers this factual summary only.
Shared role/workflow files referenced under ~/.codex were unavailable; explicitly
scoped read-only specialists supplied independent review without running checks.

Next: complete a per-server import/authentication coverage record from the
existing inventory, finish its selected missing flows, then obtain authorized
isolated end-to-end evidence before switching everyday clients.

## October31 — existing Google/Slack checks confirmed

Slack auth.test and Google Drive account information can reuse the shared
checker. Google needs its fixed fields=user option allowed in the trusted request.
The checks are known; wiring them to encrypted saved access and handling login
storage/renewal remain. No application source, live request or test result changed.
[Source-backed explanation](history/2026-10-31_13-02-43-203191-orchestrator-confirm-google-slack-account-checks.md).

## October31 — global clarity rule saved; Google/Slack explanation corrected

The global AGENTS.md now requires plain descriptions of what works, what is
missing and what must happen next, with the user's example. Readback confirmed
the rule once and preserved existing instructions.

Google already has browser sign-in and token refresh; Slack already has setup
and token/cookie inputs. Fetch still needs a supported way to save their new/
renewed access encrypted and use it on restart, plus saved-key checks and real
account/tool/restart tests. Current source confirms Authenticate returns unknown
for these local servers. No application code or test/build result changed.

- [Actual package login findings](history/2026-10-31_12-49-55-808975-researcher-google-slack-existing-login.md)
- [Global rule and correction](history/2026-10-31_12-49-55-808976-orchestrator-global-clarity-rule-and-login-correction.md)

## October30 delivery — generic authentication committed and pushed

Accepted D82 source/tests/docs are pushed as df094cc; HEAD/tracking/live remote
matched with a clean tree. This delivery-record follow-up is separate.
[Delivery and exact handoff](history/2026-10-30_17-21-02-095274-orchestrator-deliver-generic-authentication.md).
The bounded source/fixture milestone is complete; the parent working MVP remains
incomplete. No actual native/provider/MCP/client readiness or completion alert.

## October 30 — D82 generic authentication accepted

Generic input resolution and the trusted-description verifier are implemented
and independently accepted after lifecycle corrections. Seventeen new cases,
35 targeted manager cases and110 combined checks pass; native binary builds.
Existing check facts are trusted data, input names come from saved imported
bindings, and adding proof preserves standard remote OAuth. D25/D31 remain.
Commit/push delivery is next. Parent local working MVP remains incomplete;
no real provider/native/MCP/client result is claimed.

- [Accept the shared authentication engine](history/2026-10-30_17-19-45-557612-evaluator-accept-shared-authentication-engine.md)
- [Integrate generic authentication and its verified limits](history/2026-10-30_17-19-45-557613-orchestrator-integrate-generic-authentication.md)

## October 30 — generic engine passes corrected fixtures and native build

D82 implementation now resolves imported inputs generically and runs trusted
request descriptions through one verifier. Existing service check facts are data.
Remote OAuth remains independent of descriptor presence; only actual auth refs
or OAuth count as stored authentication. Review findings were fixed and three
regressions added. Independent 17 new cases,35 targeted manager cases and110
combined checks pass; native binary builds in 0.43s. Final evaluation/delivery
remain. Unknown providers still need trusted proof descriptions and compatible
login ownership; no live native/provider/MCP/client success is claimed.

- [Fix login selection independently of verification profiles](history/2026-10-30_17-17-32-765976-coder-preserve-generic-oauth-with-verification-data.md)
- [Verify corrected generic-authentication source and native build](history/2026-10-30_17-17-32-765977-tester-final-generic-authentication-regressions-and-build.md)

## October 30 — generic source compiles; lifecycle review fixes pending

Shared input resolver/verifier and trusted data profiles are implemented. The
first independent 32 manager and107 combined cases pass, but final acceptance is
withheld: review found new descriptions could disable generic OAuth, and unrelated
saved headers could be mistaken for existing authentication. Correct login-mode
selection/evidence, add production-path regressions and rerun before accepting.
No live provider/native/MCP/client proof or readiness notification.

- [Implement shared input resolution and trusted verification descriptions](history/2026-10-30_17-13-48-867235-coder-generic-verifier-source.md)
- [Run initial independent generic-authentication fixtures](history/2026-10-30_17-13-48-867236-tester-generic-authentication-preliminary-fixtures.md)
- [Request corrections to generic login classification](history/2026-10-30_17-13-48-867237-evaluator-request-generic-oauth-lifecycle-fixes.md)

## October 30 — D82 generic-authentication implementation active

User approved the shared engine and trusted descriptions. Planner accepted; one
writer owns Manager/helpers/data and independent Tester/Evaluator own checks and
review. No provider-name gate on input storage/delivery; proof semantics remain
D25/D31. Missing descriptions and wrapper-login limits stay explicit. The
installed CLT toolchain works process-scoped, resolving the prior shell/license
block without a global change. Source/test/build/review work is in progress.

- [Plan](history/2026-10-30_17-05-50-737001-planner-generic-authentication-implementation.md)
- [Scope and acceptance](generic-authentication/info.md)

## October 30 — generic authentication design under review

The user questions connector-specific hardcoding and asks for reusable functions
using imported credential names. Source/research confirm those inputs can drive
shared storage/delivery and standard HTTP OAuth. Direct service proof is a Fetch
D25 requirement, not a universal MCP health method. Current Manager recognition,
auth-name lists and request cases are hardcoded. A shared engine with trusted
request descriptions is proposed; no implementation/success-rule change approved.
No new product tests/builds or live connector actions occurred. The current shell
now blocks developer-tool Git/Python execution on an Xcode-license requirement;
prior 93-test/build evidence is historical and unchanged.

- [Primary research](history/2026-10-30_16-51-25-000001-researcher-generic-authentication-boundaries.md)
- [Clarification and handoff](history/2026-10-30_16-51-25-000002-orchestrator-generic-authentication-clarification.md)

## October 30 delivery — scoped feature committed and pushed

Accepted code/tests are delivered as b194de7; local HEAD, tracking ref and live
remote matched with a clean tree after push. This delivery-record follow-up is
separate. [Delivery and remaining testing handoff](history/2026-10-30_16-25-37-211141-orchestrator-deliver-parallel-authentication.md).
The parallel-authentication milestone is complete for source/fixture scope; the
parent local MVP remains incomplete. No real native/provider/MCP/client result
or completion alert is claimed.

## October 30 — scoped authentication fix accepted

POST authenticate keeps its saved-key authenticated check; supported named
API-key batches check every required covered value. Authentication and Settings
credential requests now track each server independently. Independent25 new
fixtures and93 combined checks pass, the native binary builds and final
Evaluator ACCEPTs the bounded change. Commit/push delivery is next; no live
native/provider/MCP/client readiness is claimed.

The user can begin the no-login reference-server subset after the protected
native session exists. Broader provider setup, generic remote authentication
proof, deletion/non-auth outcomes and actual installation/client/restart
acceptance remain the concrete full-MVP blockers.

- [Accept the scoped authentication fix](history/2026-10-30_16-24-52-062867-evaluator-accept-parallel-authentication.md)
- [Integrate checked credential batches and parallel service authentication](history/2026-10-30_16-24-52-062868-orchestrator-integrate-parallel-authentication-and-blockers.md)

## October 30 — authentication fixes pass independent fixtures and native build

The scoped change now passes 25 new authentication/UI checks and the combined
93-test suite; the locked native binary builds in 0.54s. Five new tests exercise
actual Desktop message handling without a window or executing Tasks. POST still
requires the saved-key authenticated service check. Supported named API-key
references can be verified in batches; per-server UI requests no longer block
one another. Final evaluation and commit/push are pending.

Remaining MVP blockers are broader provider/wrapper setup, generic remote login
proof where no supported service checker exists, credential deletion/non-auth
outcomes, protected native-session setup and actual installation/provider/client/
restart acceptance. No source fixture proves those live workflows.

- [Independent fixture/build pass](history/2026-10-30_16-22-11-273949-tester-parallel-authentication-fixtures-and-build.md)
- [Bounded milestone and criteria](parallel-authentication/info.md)
- [Current remaining work](../docs/testing/MVP_GAPS.md)

## October 30 — authentication source implemented; independent checks in progress

Backend source now checks supported saved credential groups without a one-entry
shortcut. UI source tracks authentication and credential-change requests per
server; same-server ownership and old-response protection remain. Both writers
compiled their source successfully. A test-only host factory enables actual
Desktop message tests without a window, Keychain or native authority prompt.
Independent fixtures/build and final evaluation are pending. This is not full
MVP/native/provider acceptance.

- [Implement supported multi-entry authentication verification](history/2026-10-30_16-20-04-162340-coder-backend-checked-credential-groups.md)
- [Implement independent authentication request tracking in the UI](history/2026-10-30_16-20-04-162341-coder-ui-independent-authentication-requests.md)
- [Provide safe host construction for independent Desktop update tests](history/2026-10-30_16-20-04-162342-orchestrator-safe-window-free-ui-test-fixture.md)

## October 30 — authentication-route clarification and bounded fix plan

User reaffirmed POST authenticate's saved-key service check and requested parallel
authentication across services. Read-only Planner/source review confirms PATCH's
one-entry shortcut and the UI's one outstanding auth-request slot are separate
implementation limitations; backend locks are already per server. Plan: retain
D25/D31, track UI requests per server, verify actual required credential groups,
and check independent success/failure, same-server duplicates and stale results.
No application changes/tests/runtime actions in this pass. Deletion-only success
and protected-MCP proof remain unselected; protected native testing remains pending.

- [Planner contract/source findings](history/2026-10-30_16-12-16-172362-planner-authentication-route-and-parallel-services.md)
- [Orchestrator clarification and handoff](history/2026-10-30_16-12-16-172363-orchestrator-authentication-plan-clarification.md)

D81 scoped source/evidence delivery is pushed through f6563b3 with matching local/
remote SHA and a clean tree at that checkpoint. [Delivery and exact pending handoff](history/2026-10-29_12-24-46-000001-orchestrator-scoped-local-source-delivery.md).
This record's documentation follow-up is separate; no local MVP completion is claimed.

## Active — local working MVP under D81

The user now authorizes finishing the selected source, local build/test execution,
computer-use installation of a real MCP and a separate test Codex instance. No
packaging/signing/DMG. [Scope](local-working-mvp/info.md),
[actual run and protection prerequisites](../docs/testing/LOCAL_MVP_RUN.md).
Planner and primary-source Researcher reports accepted; imports, remote auth and
shared UI source assigned separately. Two credential-contract choices are pending.
Native login-Keychain/owner-confirmation isolation remains a concrete prerequisite;
do not treat a fresh profile as sufficient. No current-pass runtime acceptance yet.
All earlier incomplete scope remains tracked below.

- [Implementation plan](history/2026-10-29_12-01-40-000001-planner-local-working-mvp-plan.md)
- [OAuth/isolation primary-source research](history/2026-10-29_12-01-40-000002-researcher-remote-auth-and-isolation.md)
- [Shared native UI source pass](history/2026-10-29_12-01-40-000003-coder-native-shared-ui-source.md)

Native/browser shared helpers, known import forms and generic remote OAuth source
are implemented. Final locked native build exit 0; independent combined suite68/68
PASS. Initial import assertion failure was corrected/retested. Source review found
and resolved issuer mix-up; final scoped ACCEPT. Clippy exits 0 with72 warnings.
No live UI/MCP/provider acceptance: a copied offline Codex starts with0 servers;
CUA test-user setup is at administrator Unlock. No account/password/security/trust
change or real MCP connection. Pending2 credential choices and protected native
session are the next required inputs. Do not mark working-local completion or notify.

- [Imports source and fixture correction](history/2026-10-29_12-16-09-000001-coder-import-compatibility-source.md)
- [Generic remote OAuth source](history/2026-10-29_12-16-09-000002-coder-generic-remote-oauth-source.md)
- [Browser/integrated native build](history/2026-10-29_12-16-09-000003-orchestrator-shared-browser-and-first-build.md)
- [Independent fixture runs/failure/retests](history/2026-10-29_12-16-09-000004-tester-fixture-failure-and-retest.md)
- [Issuer-mixup review finding](history/2026-10-29_12-16-09-000005-evaluator-oauth-issuer-mixup-finding.md)
- [Issuer binding fix and final checks](history/2026-10-29_12-16-09-000006-coder-fix-oauth-issuer-binding.md)
- [Separate offline Codex and Mac handoff](history/2026-10-29_12-16-09-000007-orchestrator-confined-offline-codex-and-handoff.md)

## October 29 — reassess the work remaining for a working MVP

At `1276516`, fresh read-only source inspection and independent specialist
reviews confirm five known implementation areas. Provider setup/general remote
login are the largest overlapping work; import/credential outcomes are medium;
shared UI extraction is smaller. Native/client/provider correctness remains
unverified. A working local MVP requires those areas plus isolated end-to-end
acceptance/fixes; distribution is a separate later stage. No percent or date is
supported. Existing app/API/manager/runner/storage boundaries and product scope
are preserved. No tests, builds, launches, connections or personal-state changes.

[Assessment scope and completion](mvp-readiness-review/info.md).
Next: known imports and server setup coverage, shared/provider login, credential
outcomes and UI helpers, then user-started isolated workflows. All unfinished
work below is preserved. This assessment changes documentation only and remains
local; it does not mark the operational MVP complete.

- [Planner source findings](history/2026-10-29_11-46-15-000001-planner-mvp-remaining-work-source-review.md)
- [Independent Evaluator ACCEPT](history/2026-10-29_11-46-15-000002-evaluator-mvp-local-readiness-review.md)
- [Orchestrator integration](history/2026-10-29_11-46-15-000003-orchestrator-integrate-mvp-readiness-assessment.md)

## October 29 handoff — full tree saved and remaining work counted

Documentation review is complete; debug/release paths and stale completion
wording are corrected. [Independent review](history/2026-10-29_11-36-36-000139-evaluator-remaining-work-documentation-review.md) and
[full-tree handoff](history/2026-10-29_11-36-36-001631-orchestrator-full-tree-push-and-documented-remainder.md) record the scope/evidence.
The final documentation commit contains this updated handoff; Git history and
the delivery response identify its hash. Earlier source/build evidence is unchanged.


The complete tracked/untracked working tree was committed and pushed as a15eacc,
including the Figma153-state index, shared-component guidance and pending history.
Current known remainder: five implementation areas (four core backend areas plus
remaining shared UI extraction), then isolated runtime acceptance/fixes and
separate distribution preparation. These are broad areas, not a percentage or
seven equal-sized fixes. [Current remaining-work list](../docs/testing/MVP_GAPS.md)
contains the source locations and finish criteria.

Railway/auth/logging source is implemented and compiled; Figma reference work is
recorded. Live native/provider/client verification remains unrun. The separate
Railway temporary-callback renderer still needs explicit Figma-state mapping.
No builds, tests, app/provider/MCP calls or personal-state actions in this
October29 documentation/push pass. No release is authorized.


## Current delivery — Railway sign-in, inline authentication and request logs

Scoped code is committed/pushed as9def537; local HEAD and origin/main matched.
[Delivery record](history/2026-10-28_21-55-32-489892-orchestrator-deliver-railway-auth-and-diagnostics.md). No runtime acceptance is claimed.

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
exit 0 in4.66s (prior integrated build19.64s). Existing four Keychain deprecation
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

## Latest Figma delivery — complete current UI state library

The official file now contains 153 source-driven states/specimens: the 17-frame
baseline plus 21 authentication, 28 individual Settings, 26 setup/server,
25 browser request/error, six native-surface contracts and 30 control/message/
minimum-window specimens. All nine current authentication phases and the shared
credential_editor are represented. Current UI structure and scrolling remain;
native OS appearance is explicitly unobserved rather than fabricated.

Source inventory and representative Figma review independently accepted the
reviewed scope after corrections. No application source/runtime action was part
of this design pass. Concurrent operational-MVP implementation above is separate.
See [the complete coverage/node index](../docs/UI_STATE_COVERAGE.md) and
[integration record](history/2026-10-28_21-57-19-222233-orchestrator-complete-current-ui-state-library.md). User visual feedback and previous
runtime acceptance remain open. Current SF Pro rendering limitations remain.

## Prior correction — literal current-app UI baseline

The user rejected the styled mockups. Current boards now reproduce the source's
actual UI: 13 native states, four browser HTML states, seven current native shared
component families and separate browser form controls. An unmodified screenshot
of the already-open empty Servers window sits beside its editable counterpart.
Current navigation, Iced controls, plain text status/errors, single-column forms
and scrolling are preserved. There are 59 native prototype links.

The original three boards are archived as rejected proposals; do not implement
their added dashboard, badges, alert panels or two-column Settings. Figma cannot
render native SF Pro: calibrated editable Arimo is used with original sizes in
metadata. Other states are source-based fixtures, not observed runtime states.
No app launch/operation, source change, test, build or managed connection occurred.
See [current inventory](../docs/FIGMA.md) and
[correction record](history/2026-10-28_21-17-10-000001-orchestrator-literal-current-ui-correction.md).

## Historical planning checkpoint — inline authentication before the fix

Superseded by the implemented/compiled Railway delivery above. The following
paragraph records the earlier diagnosis, not current provider support or build status.

Authenticate currently drops required/unknown outcomes into a generic Saved notice.
Implement dedicated per-server progress/results and shared inline credential entry
through existing native Settings APIs. Railway has no supported login adapter;
show that blocker honestly. Planner and initial source complete; independent source review underway. First binary
build failed on a shared spacing type; correction/rebuild pending.
No tests or live authentication started. [Scope](inline-server-authentication/info.md).
[Independent source findings](history/2026-10-28_21-20-58-629950-tester-inline-authentication-source-findings.md).
[Coder/first-build record](history/2026-10-28_21-20-38-780249-coder-inline-authentication-source.md).
[Planner record](history/2026-10-28_21-17-07-990957-planner-inline-server-authentication.md).

## Latest requirement — shared UI components in code

The user explicitly requires commonly reused UI elements to be defined once and
shared in code as well as Figma. Seven families now have responsibility/input
definitions and a recorded reuse rule in [the component registry](../docs/SHARED_UI_COMPONENTS.md),
AGENTS.md, the native UI guide and Figma descriptions/guide. Shared theme values
and platform-appropriate rendering helpers are required. Figma instances exist;
the credential editor is already shared; remaining custom UI extraction is
pending. The linked requirement-recording pass changed documentation/Figma only;
the later authentication implementation above added the shared editor.
[Pass record](history/2026-10-28_20-49-03-000001-orchestrator-shared-ui-code-requirement.md).

## Historical delivery — styled designs, subsequently rejected

The official Figma file now contains 17 editable frames: 13 current native
page/state designs and four browser approval/callback states. Populated sample
data covers running servers, active/queued calls, partial tool catalogs,
discovery, installation and retry, settings, credentials and app connection.
Seven shared component families are used as linked instances across pages;
62 native prototype links cover navigation and selected workflow transitions.

Screenshots were inspected and clipped content corrected; all native page body
bounds fit 1080×760 frames. This is Figma design inspection, not independent
review or app/runtime testing. Designs remain proposals; no application source,
personal state, trust, build or managed MCP execution changed.
See [inventory](../docs/FIGMA.md), [scope](figma-app-pages/info.md) and
[pass record](history/2026-10-28_20-46-58-000001-orchestrator-complete-figma-pages.md).
Earlier unfinished runtime checks remain below.

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
succeeded, exit 0 in10.45seconds; existing warnings only. No new tests or native/UI
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

`cargo build --locked --bin fetch --message-format short` succeeded, exit 0 in
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

`cargo build --locked --bin fetch --message-format short` succeeded with exit 0
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

First command: `cargo build --bin fetch --message-format short`, exit101 at src/mcp/mod.rs:151. Corrected build: `cargo build --locked --bin fetch --message-format short`, exit 0 in12.61seconds, producing target/debug/fetch. Toolchain1.95.0; dependencies resolved under manifest Rust1.89 compatibility. Four native classic-Keychain deprecation warnings and one unread protocol-error field warning remain. Independent source/build-evidence review ACCEPT. Fixes and Cargo.lock are committed/pushed as70e0f785fcc560f1af234ea6c9c6a9aadb97fe90; local HEAD and origin/main agreed after push. Stop; do not run the executable. Every runtime/test step remains for the user's separate instruction.

## Current pass history

- [Plan compilation-only build fixes](history/2026-10-28_14-59-16-596517-planner-compile-only-plan.md)
- [Run first native binary compilation](history/2026-10-28_14-59-16-596619-orchestrator-first-binary-compilation.md)

- [Compiler fix and successful binary build](history/2026-10-28_15-00-36-928523-coder-binary-build-error-fix.md)

- [Independent compiler-fix acceptance](history/2026-10-28_15-02-42-900848-evaluator-accept-compile-fix-delivery.md)

- [Compile-fix delivery](history/2026-10-28_15-03-44-962679-orchestrator-compile-fix-delivery.md)

## Prior source and future acceptance

[Source milestone](mcp-mvp-implementation/info.md), [source handoff](../docs/testing/HANDOFF.md), [earlier snapshot](milestone-before-d77-source.md), [deferred tasks](to-do-before-d77-source.md) and [isolated testing](isolated-mcp-testing/info.md) retain prior records. D78 custom GitHub CLI deferral and all earlier scope deferrals remain. The five-server roster is still not run and the isolation boundary unprovisioned.
