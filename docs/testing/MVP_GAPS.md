# Remaining work for a working Fetch MVP

Updated October 29, 2026 after the full working tree was committed and pushed.
This is the current remaining-work list. Architecture decisions still define
scope; this document records actual implementation and evidence gaps.

There are **five known implementation areas**, followed by **two completion
stages**: isolated runtime acceptance and distribution preparation. The first
four implementation areas affect backend behavior. The fifth is the user's
required shared UI structure. These are broad areas, not seven equal-sized
fixes or seven remaining endpoints. Some areas overlap. They do not support an
honest completion percentage or finish date; runtime checks may find more work.

## October 29 source reassessment

A fresh read-only Planner review and independent Evaluator review at `1276516`
confirm these five known implementation groups. The main app, client approval,
encrypted storage, server runner and shared MCP endpoint have substantial source
implementations. This bounded review found no additional missing major subsystem;
it does not establish that the existing implementations work end to end.

For a **working local MVP**, complete areas 1–5 and stage 6 below. Stage 7 is the
separate work needed to distribute the application. Provider setup and general
remote sign-in are the largest implementation work and overlap. Known import
forms and credential editing/removal are medium tasks; remaining shared UI code
is smaller. These are relative source-based assessments, not time estimates.
The amount of repair after real native/provider/client checks remains unknown.

Recommended order: fix known imports and record setup support for the existing
server inventory; finish shared remote login and required provider-specific
setup; correct credential outcomes; finish the shared UI helpers; run authorized
isolated workflows and fix actual failures. Most of this is engineering under
settled decisions. The user still chooses the protected testing arrangement and
starts execution, then completes required provider consent. If credential-success
behavior must change, present that one concrete contract change before implementing.

See [the source-assessment milestone](../../milestones/mvp-readiness-review/info.md)
for independent findings and limits. No tests, builds, app launches or connections
were run for this reassessment.

## Five implementation areas

D81 progress: exact public import/bridge forms and native/browser shared renderers
are implemented and fixture-tested; real visual/installation acceptance is still
pending. Generic remote discovery/registration/renewal source now exists, with
issuer-bound callbacks, but completing authenticated success is blocked on the
pending verification choice and provider-specific proof. Existing credential
batch/removal behavior remains unchanged pending its focused decision. Broader
everyday-provider/wrapper setup is still unfinished. Read [the active run](LOCAL_MVP_RUN.md)
for actual68-test/build evidence and native-environment handoff. The table below describes the remaining work after these source passes.

| Area | What is still incomplete | What counts as finished | Source to continue from |
| --- | --- | --- | --- |
| 1. Provider setup for everyday servers | Fresh setup and stored-credential verification cover only a small recognized provider list. Other required imports can install but return unknown instead of completing sign-in. | Trace every selected server in the existing inventory through its real login/credential ownership, protected storage, authenticated verification, refresh and restart behavior. Record usable versus missing support per integration; do not equate installation or tools/list with authenticated setup. | src/manager/mod.rs::provider_kind/authenticate/verify_stored; docs/architecture/real-use-compatibility.md |
| 2. General remote OAuth | Generic discovery/registration/PKCE/renewal source is implemented with pinned destinations and exact callback issuer binding. Verified authentication for providers without a separate account API awaits the explicit proof-rule decision; actual login/renewal remains unrun. | Complete the approved verification integration, exercise real supported remote sign-in/renewal and report provider-side client restrictions honestly. No login on ordinary tool calls. | src/manager/remote_auth.rs; src/manager/mod.rs::begin_provider_login/refresh_owned |
| 3. Credential editing, batches and removal | A successful save currently requires one recognized effective authentication entry with no existing OAuth token. Other environment/custom-header edits, multiple entries and removal-only changes may commit then return verification failure. | Resolve and implement the successful outcomes for these advertised Settings operations consistently with the accepted stored-credential verification requirement. Correctly report committed-but-unverified state and required reauthentication; do not bypass verification or automatically retry an uncertain write. | src/manager/mod.rs::save_credentials/can_verify_batch; docs/architecture/api-contract.md |
| 4. Real configuration import compatibility | Precise Miro/Vercel/v0/HF endpoints, narrow HF queries, proven mcp-remote bridge forms and curated Everything/Time launcher forms now pass seven fixtures. Full inventory setup and real installation/connection compatibility remain unverified; unknown grammars stay explicit. | Exercise selected safe inventory forms through the actual UI installer/client, preserving source identity and fresh credential references; resolve concrete remaining compatibility findings. | src/discovery/mod.rs; tests/import_compatibility.rs; docs/architecture/real-use-compatibility.md |
| 5. Shared UI code and acceptance | Native components/theme and browser document/message/form helpers are implemented and compiled. Real visual states, later approved credential-result changes and Railway Figma mapping remain unverified/pending. | Preserve actual layout/masking/actions/scrolling in native checks; integrate final structured credential outcomes and maintain the matching reference index. | src/ui/components.rs; src/ui/theme.rs; src/browser.rs; docs/SHARED_UI_COMPONENTS.md |

Areas 1–4 are the immediate product-completion priority. Use the existing
inventory; do not ask the user to relist it. Shared UI extraction is a separate
required implementation item, not another authentication backend.

## Two completion stages after and alongside implementation

| Stage | Remaining work | Evidence needed |
| --- | --- | --- |
| 6. Isolated runtime acceptance and fixes | Select/prepare the approved isolation, disposable data, dedicated accounts and fresh client profile. Exercise native approval/Keychain/TLS/window/Quit, install/auth/provider calls and reuse, credential errors, raw protocol/media, queues/deadlines/cancellation/restart/revocation, and Claude Code/OpenCode/Codex workflows. Include Railway and the existing short roster; the roster is not the whole acceptance inventory. | Actual expected/observed results, failures fixed and rerun, independent review, fresh-install and repeated-session evidence. A passing binary build or source review cannot substitute. Existing older foundation results do not verify the latest native/provider changes. |
| 7. Distribution preparation | Collect exact resolved dependency/runtime notices; execute the selected app/DMG packaging, signing/notarization and fresh-install checks when authorized. Packaging scripts and a source notice inventory exist; a finished verified distribution does not. | Complete notices and inspected/signed/notarized artifacts as selected, plus explicit permission for any release/publication. Git commits/pushes do not authorize a release. |

Tests, app/service launches, managed MCP/provider connections, actual package
setup, credential/trust/personal-client changes and packaging execution remain
subject to the existing restrictions. The October 29 request authorizes saving
the full working tree and documenting its status, not running those activities.

## What is already implemented or recorded

- Railway browser authentication uses Fetch-owned public registration and PKCE
  (the standard protection that binds a login response to the app that began it),
  D80's temporary numeric-loopback callback, encrypted registration/token storage
  and code that requires a successful authenticated /oauth/me identity check
  using the stored token before reporting authenticated.
- Railway's required CLI5.71.0 has private preparation/explicit-Connect repair
  source, private HOME/PATH/token handoff and automatic CLI updates disabled.
  The official HTTPS root at mcp.railway.com is an exact supported import profile.
- Inline authentication, shared masked credential entry, guarded status updates
  and independent installation/restart/auth polling are implemented.
- Safe first-party incoming/outgoing HTTP metadata logging is implemented. See
  [DIAGNOSTICS.md](DIAGNOSTICS.md) for coverage and stream/subprocess limits.
- The final October 28 binary build succeeded in4.66seconds after the integrated
  build in19.64seconds. Scoped source reviews accepted the changes. No live
  Railway sign-in or current native/provider/client acceptance was performed.
- The official Figma project and153-state library are recorded, with fixture,
  visual-review and native-font limits in [UI_STATE_COVERAGE.md](../UI_STATE_COVERAGE.md).
  This is design/reference work, not implementation or passing runtime evidence.

Known Railway limitation: after background refresh, an already-running local
child retains its old environment token until explicit Restart. The API exposes
restart_required and guidance. This must stay visible; seamless live token
replacement is not implemented or being claimed.

Design follow-up: the coverage index does not yet trace the separate D80 Railway
callback renderer in src/manager/provider_callback.rs. Its seven distinct response
texts need explicit specimen mapping/reconciliation. Do not assume the existing
generic provider callback frames cover them, or add them to the153 count without
evidence. This is UI-reference maintenance within area5, not an extra backend.

## Explicitly outside the selected first release

Custom Fetch-owned GitHub CLI tools (existing GitHub MCP imports stay in scope),
shared memory, PostgreSQL/retention, graph, extra managed processes, native
background Tasks, live resource subscriptions and future project/session sharing
retain their recorded deferred status. They are not counted as unfinished MVP
work above. Source revision history and the newer decisions remain intact.
