# Current UI state coverage

Updated October 28, 2026. This is the index for the user's request to mock every
currently displayable UI element, including authentication and individual settings.
The official file contains **153 states/specimens**: full page states, reusable
HTML error specimens, widget examples and explicitly labeled native-surface
contracts. It does not enumerate infinitely many server names, values, cursor
positions, error strings or combinations of otherwise covered states.

The first 17 baseline frames remain. Later boards expand them using current source
rather than inventing new screens. The app's authentication UI is an inline panel
inside an expanded server card, not a new product route or modal.

## October 29 source reconciliation needed

The existing153 specimens remain the recorded library. The separate D80 Railway
callback renderer in src/manager/provider_callback.rs is not traced by the
source/node index below. Its seven distinct response texts (wrong method,
unknown path, invalid callback, unmatched/expired state, stopped Fetch, verified
sign-in and failed sign-in) need explicit specimen mapping or additions. No
Figma edit or live callback inspection was performed in this documentation pass;
do not claim this newer renderer is visually covered or silently change the153
count. The source-backed original library remains useful within its snapshot.

## Source and evidence

D81 shared-code reconciliation: native rendering now reuses src/ui/components.rs
and theme.rs, while src/browser.rs supplies the shared escaped browser shell,
messages and controls for all three Fetch-owned renderers. Current text/layout
and the153 reference count are unchanged. The seven Railway paragraph variants
above still need their Figma specimen mapping; shared code is not visual evidence.

- Native rendering: src/ui/mod.rs (servers_view, authentication_view, install_view,
  connect_view, settings_view and the global error/notice slots).
- Shared editor and nine authentication phases: src/ui/authentication.rs
  (Phase::title, Flow::result/observe and credential_editor).
- Browser: src/oauth/mod.rs::approval_page/page_error and
  src/api/runtime.rs::provider_callback.
- Native contracts: src/native_auth/mod.rs and bridge.m; src/ui/file_picker.rs.
- Widget states: installed Iced 0.14 button, text_input, checkbox, scrollable and
  theme palette source. Derived colors retain current Iced Light behavior.
- Direct runtime visual evidence is limited to the previously captured empty
  Servers window. Expanded states are source-derived fixtures. No live auth,
  Settings save/remove, file picker, browser login, app launch, build or test was
  run for this design expansion. Independent review was source-based plus
  representative Figma screenshot inspection, not exhaustive runtime acceptance.

Source changed concurrently during authoring. The final review includes the ninth
NeedsVerification phase and current shared credential editor. UI source snapshot
hashes at 21:50 EDT: mod.rs cc2d423b075ef76b656718201f40d104a756cd9fa685c68fe29c75c0269be5b8;
authentication.rs a3c2dab79bddc6dc84ce688ab6d356af09e3ae7e400431ccff722d196e1e4343.
Function names remain the durable references when source line numbers change.

## What is covered

| Area | Rendering branches and distinctions |
| --- | --- |
| Global messages | Error before notice before page; Install messages inside the scrolling list; defensive client error strings and dynamic backend message slot. |
| Server cards | Collapsed/expanded; optional execution record; ordinary auth/runtime errors; restart-required while ready; tracked restart and read/result notices; uncertain calls/force action; change controls disabled while Inspect/disclosure/navigation stay active. |
| Authentication | All nine phase titles; browser pending with/without local flow; user guidance; manual bindings from environment/headers; zero/multiple references; filled/empty editor; stored-credential verification; separate flow/read/server/runtime error slots; re-verification and reconciliation. |
| Settings | No selected server, found/missing record, no/one/multiple credential rows, raw stored/missing/unknown statuses, edited/invalid/saved call limits, each credential-editor state, confirmation results, storage/verification/removal outcomes, advanced raw config and single-line patches for launch/directory/header/timeout settings. |
| Installation | No definitions, supported/unsupported/source-disabled selections, multiple sources, group/global errors, absolute path without discovery, working/picker/in-progress controls, accepted/existing operation, pending/running/succeeded/failed/read-error and retry. |
| Browser | Requested subset/custom selection/defaults; Mac-unavailable retry form; all 21 hard-coded unstyled HTML error paragraphs; provider callback result HTML; raw unavailable OAuth response body. |
| Native surfaces | App-approval and credential-change Mac reason strings, file picker configuration, native menu/lifecycle and provider/certificate handoffs. Appearance is explicitly OS/provider-owned and unobserved. |
| Controls | Active/hovered/pressed/disabled button; active/hovered/focused/disabled input, placeholder/value/secure/caret/selection; six checkbox combinations; active/hovered/dragged scrollbar; minimum 760×520 client specimen. |

## Important source behavior preserved

- The editor labels are Credential entry name, API key or token and Save and
  verify credential. Settings has Remove entry; inline authentication does not.
  Save needs a trimmed name and nonempty value. Remove needs a trimmed name.
  Saving clears the secret immediately.
- Settings editor disabling uses busy/auth_active, not a tracked restart alone.
  Inline failed-manual reconciliation can leave its editor visibly enabled while
  the handler refuses another request; outer change controls and dismissal are
  blocked. Call-limit/patch inputs and save buttons are not globally grayed out.
- Stored rows say stored or missing, not verified. Verification is separate auth
  evidence. Current remove-only writes may commit and then report verification
  failure; the mockup does not invent unconditional removal success.
- NeedsVerification uses its exact re-check title, without Unknown's explanatory
  paragraph or Open server settings button. Unsupported auth uses a generic
  fixture; it does not falsely label Railway unsupported.
- The same authentication error may be rendered in two independent source slots.
  No deduplication is added. Patch entry remains a single-line input.
- OAuth permission order is Catalog, Content, Manage, Tools. Manage defaults off;
  requested others default on. Retry after unavailable Mac confirmation resets
  defaults. Allow/Deny redirects to the client, with no invented Fetch success page.
- Native Touch ID/password and file-navigation pixels are not invented. The
  native board specifies source-owned text/behavior and labels the remaining
  appearance as unobserved. No Keychain unlock, custom password/account form or
  Quit confirmation is implied.

## Maintaining this index

Every new or changed conditional UI branch must update the shared component and
its affected specimens, then refresh this index. Keep current-app structure and
source defaults unless a UI change is explicitly requested. Shared code and Figma
responsibilities are defined in [SHARED_UI_COMPONENTS.md](SHARED_UI_COMPONENTS.md).
Native SF Pro is unavailable in this Figma runtime; calibrated editable Arimo and
source-size metadata remain a documented typography limitation. Existing native
prototype links stay on the baseline board. Expanded boards are state specimens;
invalid inherited links to other pages were removed.

## Board and specimen index

### [01 · Current app — literal UI](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=20-484) — 13

| State/specimen | Figma node |
| --- | --- |
| Servers · Empty | [23:487](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-487) |
| Servers · Populated | [23:515](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-515) |
| Servers · Tools expanded | [23:538](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-538) |
| Servers · Recovery | [23:561](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-561) |
| Install · Before discovery | [23:584](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-584) |
| Install · Discovery results | [23:607](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-607) |
| Install · In progress | [23:630](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-630) |
| Install · Failed and retry | [23:653](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-653) |
| Connect · Setup | [23:676](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-676) |
| Connect · Cleanup | [23:699](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-699) |
| Settings · No server | [23:722](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-722) |
| Settings · Selected server | [23:745](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-745) |
| Settings · Advanced (scrolled) | [23:768](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-768) |

### [03 · Current browser pages](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=20-486) — 4

| State/specimen | Figma node |
| --- | --- |
| Current browser / App approval | [28:386](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=28-386) |
| Current browser / Expired request | [28:433](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=28-433) |
| Current browser / Provider success | [28:437](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=28-437) |
| Current browser / Provider failure | [28:441](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=28-441) |

### [04 · Authentication states](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-348) — 21

| State/specimen | Figma node |
| --- | --- |
| Auth · Starting | [36:933](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-933) |
| Auth · Browser pending | [36:980](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-980) |
| Auth · Pending with guidance | [36:1025](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1025) |
| Auth · Required no binding | [36:1072](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1072) |
| Auth · Required choose reference | [36:1132](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1132) |
| Auth · Required credential filled | [36:1196](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1196) |
| Auth · Saving and verifying | [36:1260](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1260) |
| Auth · Authenticated | [36:1322](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1322) |
| Auth · Not required | [36:1370](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1370) |
| Auth · Unsupported automatic auth | [36:1418](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1418) |
| Auth · Automatic auth failed | [36:1470](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1470) |
| Auth · Manual verification failed | [36:1522](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1522) |
| Auth · Pending status-read error | [36:1587](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1587) |
| Auth · Unrecognized auth response | [36:1633](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1633) |
| Auth · Manual entry validation | [36:1685](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1685) |
| Auth · Failed reconciling | [36:1750](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1750) |
| Auth · Authenticate available | [36:1801](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1801) |
| Auth · Authenticating without local flow | [36:1846](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-1846) |
| Auth · Needs verification | [39:1340](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=39-1340) |
| Auth · Manual failure reconciling | [40:3623](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-3623) |
| Auth · Flow and server error slots | [48:1466](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=48-1466) |

### [05 · Individual settings states](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-349) — 28

| State/specimen | Figma node |
| --- | --- |
| Settings · Call limits edited | [37:384](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-384) |
| Settings · Call limits invalid | [37:450](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-450) |
| Settings · Call limits saved | [37:518](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-518) |
| Settings · Credential empty | [37:586](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-586) |
| Settings · Credential named | [37:652](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-652) |
| Settings · Credential filled | [37:718](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-718) |
| Settings · Credential whitespace name | [37:784](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-784) |
| Settings · Credential saving | [37:854](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-854) |
| Settings · Credential saved | [37:924](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-924) |
| Settings · Credential stored but unverified | [37:993](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-993) |
| Settings · Removal committed verification failed | [37:1062](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1062) |
| Settings · Confirmation cancelled | [37:1131](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1131) |
| Settings · Confirmation timed out | [37:1199](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1199) |
| Settings · Confirmation unavailable | [37:1267](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1267) |
| Settings · Settings authority expired | [37:1335](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1335) |
| Settings · Credential storage error | [37:1403](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1403) |
| Settings · Credential name validation | [37:1471](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1471) |
| Settings · Authentication or restart blocks save | [37:1532](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1532) |
| Settings · Multiple credential statuses | [37:1600](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1600) |
| Settings · Selected record missing | [37:1674](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1674) |
| Settings · Advanced valid patch | [37:1733](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1733) |
| Settings · Advanced invalid input | [37:1809](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1809) |
| Settings · Advanced validation error at top | [37:1887](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1887) |
| Settings · Advanced save result | [37:1965](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=37-1965) |
| Settings · Restart guard with enabled editor | [40:3479](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-3479) |
| Settings · Selected record without credentials | [46:1170](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=46-1170) |
| Settings · Advanced launch arguments and directory | [46:1224](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=46-1224) |
| Settings · Advanced remote header reference | [46:1287](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=46-1287) |

### [06 · Setup and server states](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-350) — 26

| State/specimen | Figma node |
| --- | --- |
| State · Restart required while ready | [40:384](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-384) |
| State · Restart in progress | [40:427](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-427) |
| State · Restart status read error | [40:471](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-471) |
| State · Restart succeeded | [40:517](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-517) |
| State · Restart failed | [40:560](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-560) |
| State · Separate authentication and runtime errors | [40:606](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-606) |
| State · Execution record absent | [40:655](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-655) |
| State · Global error and notice ordering | [40:695](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-695) |
| State · Catalog available | [40:740](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-740) |
| State · Catalog unavailable | [40:779](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-779) |
| State · Catalog multiple origins | [40:817](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-817) |
| State · Discovery working | [40:864](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-864) |
| State · File picker open controls | [40:908](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-908) |
| State · Discovery global failure | [40:952](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-952) |
| State · Discovery multiple sources and group error | [40:995](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-995) |
| State · Source-disabled explicit selection | [40:1048](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1048) |
| State · Existing installation monitored | [40:1117](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1117) |
| State · Installation pending | [40:1188](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1188) |
| State · Installation succeeded | [40:1239](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1239) |
| State · Installation monitor error | [40:1312](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1312) |
| State · Picker failed to open | [40:1366](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1366) |
| State · Picker unusable path | [40:1409](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1409) |
| State · Picker callback closed | [40:1452](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1452) |
| State · Absolute path selected without discovery | [40:1495](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1495) |
| State · Cleanup prompt scrolled | [40:1536](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=40-1536) |
| State · Unsupported definitions only | [46:1917](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=46-1917) |

### [07 · Browser errors and requests](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-351) — 25

| State/specimen | Figma node |
| --- | --- |
| Browser · Requested permission subset | [42:744](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-744) |
| Browser · Custom permission selection | [42:774](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-774) |
| Browser · Mac confirmation unavailable retry | [42:822](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-822) |
| HTML error / The connection request is invalid. | [42:853](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-853) |
| HTML error / Fetch requires an authorization code with PKCE S256. | [42:857](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-857) |
| HTML error / The requesting app is missing. | [42:861](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-861) |
| HTML error / The callback address is missing. | [42:865](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-865) |
| HTML error / The PKCE challenge is missing. | [42:869](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-869) |
| HTML error / The PKCE challenge is invalid. | [42:873](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-873) |
| HTML error / This request targets a different Fetch service. | [42:877](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-877) |
| HTML error / The requested permissions are invalid. | [42:881](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-881) |
| HTML error / Fetch authorization is unavailable. | [42:885](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-885) |
| HTML error / The requesting app is not registered. | [42:889](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-889) |
| HTML error / The callback address does not match the app registration. | [42:893](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-893) |
| HTML error / This approval must come from Fetch's own connection page. | [42:897](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-897) |
| HTML error / Open a fresh Fetch connection page. | [42:901](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-901) |
| HTML error / The approval is invalid. | [42:905](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-905) |
| HTML error / The approval is missing. | [42:909](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-909) |
| HTML error / This connection request has ended. Connect again from the app. | [42:913](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-913) |
| HTML error / This connection approval is invalid or expired. | [42:917](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-917) |
| HTML error / Choose only permissions requested by this app. | [42:921](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-921) |
| HTML error / This connection request has ended. | [42:925](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-925) |
| HTML error / The app is no longer registered. | [42:929](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-929) |
| HTML error / The callback is invalid. | [42:933](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-933) |
| Protocol response body · temporarily unavailable | [42:937](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-937) |

### [08 · Native surface contracts](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-352) — 6

| State/specimen | Figma node |
| --- | --- |
| Mac confirmation · app connection | [43:384](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=43-384) |
| Mac confirmation · save and remove | [43:389](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=43-389) |
| File picker · configured sheet | [43:394](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=43-394) |
| Provider browser handoff | [43:399](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=43-399) |
| Certificate trust handoff | [43:404](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=43-404) |
| Close, Show and Quit lifecycle | [43:409](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=43-409) |

### [09 · Control and message specimens](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=38-381) — 30

| State/specimen | Figma node |
| --- | --- |
| Specimen / Button · Enabled | [44:386](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-386) |
| Specimen / Button · Hovered | [44:390](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-390) |
| Specimen / Button · Pressed | [44:394](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-394) |
| Specimen / Button · Disabled | [44:398](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-398) |
| Specimen / Input · Enabled | [44:402](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-402) |
| Specimen / Input · Hovered | [44:406](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-406) |
| Specimen / Input · Focused | [44:410](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-410) |
| Specimen / Input · Disabled | [44:414](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-414) |
| Specimen / Input · Secure | [44:418](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-418) |
| Specimen / Input · Caret | [44:423](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-423) |
| Specimen / Input · Selection | [44:428](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-428) |
| Specimen / Input · empty placeholder | [44:432](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-432) |
| Specimen / Checkbox · Selected | [44:439](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-439) |
| Specimen / Checkbox · Unselected | [44:445](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-445) |
| Specimen / Checkbox · Disabled | [44:451](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-451) |
| Specimen / Checkbox · Hovered selected | [44:458](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-458) |
| Specimen / Checkbox · Hovered unselected | [44:464](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-464) |
| Specimen / Checkbox · Disabled selected | [44:471](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-471) |
| Specimen / Scrollbar · Active | [44:475](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-475) |
| Specimen / Scrollbar · Hovered | [44:479](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-479) |
| Specimen / Scrollbar · Dragged | [44:483](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-483) |
| Client error specimen 1 | [44:485](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-485) |
| Client error specimen 2 | [44:489](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-489) |
| Client error specimen 3 | [44:493](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-493) |
| Client error specimen 4 | [44:497](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-497) |
| Client error specimen 5 | [44:501](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-501) |
| Client error specimen 6 | [44:505](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-505) |
| Client error specimen 7 | [44:509](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-509) |
| Client error specimen 8 | [44:513](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-513) |
| Minimum window · 760×520 client | [44:517](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=44-517) |

