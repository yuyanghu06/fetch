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

## Five implementation areas

| Area | What is still incomplete | What counts as finished | Source to continue from |
| --- | --- | --- | --- |
| 1. Provider setup for everyday servers | Fresh setup and stored-credential verification cover only a small recognized provider list. Other required imports can install but return unknown instead of completing sign-in. | Trace every selected server in the existing inventory through its real login/credential ownership, protected storage, authenticated verification, refresh and restart behavior. Record usable versus missing support per integration; do not equate installation or tools/list with authenticated setup. | src/manager/mod.rs::provider_kind/authenticate/verify_stored; docs/architecture/real-use-compatibility.md |
| 2. General remote OAuth | Registration, browser login and renewal are implemented for particular providers. There is no general protected-resource/issuer discovery and registration path for other remote servers. | The existing Authenticate operation can perform supported remote discovery/registration/sign-in/renewal with validated destinations and the existing verification/credential rules. Handle failures honestly without opening login on normal tool calls. | src/manager/mod.rs::begin_provider_login/refresh_owned; src/runner/transport.rs |
| 3. Credential editing, batches and removal | A successful save currently requires one recognized effective authentication entry with no existing OAuth token. Other environment/custom-header edits, multiple entries and removal-only changes may commit then return verification failure. | Resolve and implement the successful outcomes for these advertised Settings operations consistently with the accepted stored-credential verification requirement. Correctly report committed-but-unverified state and required reauthentication; do not bypass verification or automatically retry an uncertain write. | src/manager/mod.rs::save_credentials/can_verify_batch; docs/architecture/api-contract.md |
| 4. Real configuration import compatibility | Current launcher-argument and URL classification rejects forms found in the recorded everyday-server inventory. The exact official Railway root is now supported; that does not solve other rejected forms. | Support the selected safe forms using disposable fixtures based on the recorded inventory, preserving source identity and credential references without importing secret values or rewriting destinations silently. Record remaining unsupported definitions explicitly. | src/discovery/mod.rs::classify/supported_remote_endpoint; docs/architecture/real-use-compatibility.md |
| 5. Remaining shared UI code extraction | The credential editor is already shared between Settings and inline authentication. Other repeated custom UI rendering and common visual values still need the required shared implementations. | Relevant native/browser pages reuse the common rendering/helpers/theme, preserving current layout, controls, masking, availability and scrolling. Keep matching Figma components and state coverage aligned. Do not introduce the rejected visual redesign. | src/ui/mod.rs; src/ui/authentication.rs::credential_editor; docs/SHARED_UI_COMPONENTS.md |

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
