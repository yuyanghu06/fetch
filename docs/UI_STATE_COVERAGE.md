## November 9 — equal-size visual verification

The glossy port is superseded by the exact-fill/layered-shadow correction. New evidence compares Figma's524×208Button state board with native1xand2xrenders at the same logical bounds, and Figma native content with1248×1128Tools/Settings views using matching visible fixture values. Cold and cached images must both show every background/glyph. Additional observed defects receive pixel checks: white numeric arrow plates and blue/gray checkbox faces must paint above their backgrounds.

Existing panel/key ownership, three-column scrolling, Copy, restart modifier state, remote-terminal restriction and progress reopening checks remain. Captures use actual widget trees with disposable fixtures, not a running app. The Button board is a visual state specimen; its styled Pressed example is not a claim of literal all-gesture font equality. Fonts/blur edges still require honest residual measurements. [Current evidence](../milestones/neumorphic-fidelity/info.md).

## November 9 — native implementation and sample-data renders

The native source now implements the separate neumorphic Figma design. Window-free actual-widget verification covers opening/closing Tools/Settings/Console, correct per-server limit/key bindings and secret clearing, raw JSON disclosure, three-column tools with internal scrolling while the page stays still, round restart clicks with normal/Shift state, remote Terminal rejection, address Copy and retained installation progress reopening.

Representative source-real captures cover Servers closed, tools, tools internally scrolled, inline Settings, Console, Install base/popup and Connect at1248×900and760×520, including an outer-scrolled minimum-window tools view. Fixtures use temporary state and synthetic keys; no tasks, native window, listener, Keychain, provider login or real MCP run. These do not establish every live/error/value combination. Browser approval screens and OS titlebar remain separate. [Evidence](../milestones/neumorphic-dev-sync/info.md).

## Current — inline Settings and full Servers layout rollout

New app board75:2718 now has30 native-window specimens:22 Servers and8 Install/Connect. The6 former standalone Settings frames keep their IDs but now show per-server inline Settings; no-server Settings was superseded by Filesystem Settings because Settings opens from a server card. New Search panels-closed204:4868 provides correct Search expand/close routing. Former manually scrolled key state reuses its identical filled editor state and now uses panel scrolling rather than a separate Settings page.

All22 Servers frames have enlarged windows/shared recessed Installed servers backgrounds. Settings family202:2558 preserves five ready/opening/filled/busy variants; each Settings button identifies the correct server. Inline numeric row uses requested bold titles and arrow appearances; light Copy is transparent. Original literal/source24-state board is unchanged. Structure/rendered checks PASS; actual new gestures were not rerun while user actively used Figma. [Scope/evidence](../milestones/inline-server-settings/info.md).

## November 8 — populated internal-scroll demo

New native design specimens now total29, adding Tools scrolling demo177:3654 (source-reference page remains24). Tools78:1438 and demo177:3654 use six linked cards/three columns/two rows with true internal vertical overflow. The enlarged windows and recessed Installed servers section fit the outer content. Name/Required fields are read-only raised values; full original read_file JSON retained; other five definitions are illustrative. Active navtabs across29 designs use inset/no-glow selection. Actual nested wheel scrolling, navigation and12values independently PASS. [Scope/evidence](../milestones/recessed-tools-demo/info.md).

## November 8 — explicit server panel states

New Figma app page75:2718 now contains28 window specimens (original current-source reference page remains24): added panels-closed170:2698 and state-preserving Console-open recovery170:2818/sign-in170:2918/restarting170:3044. Existing Tools78:1438 has no Console; Console78:1705 and Terminal78:1752/78:1767 show output only as explicit-open specimens. Shared selected state172:1910 has inset shadows and no glow; four-control review172:3525 includes Settings. Both terminal panes fill the card width. Original page scrolling remains. Shift-click force restart is an implementation requirement; Figma does not simulate it. See [scope](../milestones/explicit-server-panels/info.md).

## November 8 — tools belong to the expanded server

Available tools now live inside their owning expanded server card in the new Figma design. Shared card153:1912 contains the normal header/actions, Console, Available tools grid and footer. Review153:1981 shows the whole card; app frame78:1438 uses it with vertical scrolling. Standalone heading/grid were removed. Inspect tools links open this expanded specimen; the header collapses back to the ordinary server list. Tool data/shared inputs are preserved. Independent review ACCEPT;123local main definitions/14sets. This relocation is Figma-only; the Rust tool section remains unchanged until the UI port.

Latest label: the new Figma tool section is **Available tools** (formerly Tool definitions). Source/original references are unchanged.

Final inline-controls/tool-card review: independent Evaluator ACCEPT; right alignment, recovery layout and square two-field card pass. Figma-only.

## November 7 — inline output controls and square tool cards

Console and Terminal now sit on the right of the same row as Inspect tools/Settings. Seven active expanded-card masters and seven native specimens reuse the shared output controls; the last button ends at the1072pxcontent edge in every row, including recovery with extra actions. Availability and light-gray treatment stay intact.

The new Tool card146:2456 is a360×360 raised rounded square with two shared inset Text input instances: Name and Required fields. Their values are exposed for specimen edits; read_file uses path from the actual required list. Full original tool JSON is preserved unchanged in a hidden editable Raw definition property. This layout does not implement tool renaming, argument submission or execution. Review frame146:2466 and current app tool78:1456 show the card; a wrapping grid147:2704 holds cards. The filesystem · available origin label is removed. Prior disclosure set75:2331 is retained and marked Legacy. Library122local main definitions/14sets.

Tool cards remain in the current separate Tool definitions section; moving that section into the owning server card is still a recommendation, not selected. Original reference pages and Rust source are unchanged in this Figma-only pass.

## November 7 — authentication states and compact server footer

Authentication keys now use light gray#B8BDC3 when sign-in is confirmed, unnecessary or already underway; required/failed/unknown states retain the black action. Runtime ready alone is not authentication proof. The new designs use the existing authentication fields; there is no backend behavior change. The official [MCP authorization specification](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization) defines optional HTTP authorization, while local stdio servers use separate credential handling. A running local server is not universal proof of a signed-in service account.

Runtime dots are optically centered with the visible name. The user requested then cancelled a per-server settings gear; it has been removed. Header actions remain key/restart/close; the navbar Settings gear is retained. Repeated lowercase server IDs are removed from expanded cards. Console/Terminal controls are light gray#E3E6E9 with availability preserved.

New shared footer139:1940 puts activity counts at left and connection expiry/last verification together at right on one line. Seven expanded-card specimens preserve their exact values. All13headers have three actions, no per-card gear and no redundant direct ID;14output controls updated. Library121local main definitions/14sets. Independent visual Evaluator ACCEPT; no overlap/clipping.

Tool definitions are still separate because src/ui/mod.rs renders the section after the server list. Moving definitions into their matching server card is the assistant's recommendation, not yet a user-approved change. This entire pass is Figma-only; no Rust/native/backend changes or account/MCP execution occurred.

## November 7 — final server header actions

The disclosure is now a plain14px muted chevron with no circular background/shadow, inside a transparent28px click target. Name stays18px Bold; dot stays8px and yellow during starting/restarting. Three44px actions sit on the right: black#1D2023 with filled white key opens authentication; blue with white circular arrow restarts the server; red with white× removes it. All13native headers and8active card masters use shared Server actions132:1678. Busy authentication/restart controls use disabled variants; existing disabled deletion states are retained. Disabled red remains saturated per the user's earlier color instruction.

The new library has120local main definitions/14sets. Review sheet129:1601 now pairs identities and actions. Independent final visual review ACCEPTs the current app headers. These actions specify the new Figma UX; they are not newly wired native buttons or executed backend operations. Rust/original reference pages were unchanged in this pass.

## November 7 — compact server card headers

The new neumorphic server cards use a bold18px name, a28px recessed expand button with16px right/down chevron, and an8px runtime dot. Ready is green#77AD8A; stopped gray#9CA3AA; starting/restarting yellow#E2B84F; failed red#C76B6B. Warnings sit beside the dot in smaller amber text, such as Sign-in required. Successful/unneeded authentication labels stay hidden. User rejected the initial22px name/36px button as too large; final sizes are18px/28px.

Shared header set128:1597 has eight variants (four runtime colors × collapsed/expanded), text/visibility properties and bound color variables. Eight active card masters and13native header instances use it; a compact review sheet129:1601 shows states. Library now116local mains/14sets. This pass changes only the new Figma designs; the original app references and Rust source are unchanged. Runtime hover text and clicking the full identity area are implementation requirements, not demonstrated native behavior.

Final November7 follow-up: independent Tester PASS and Evaluator ACCEPT. Evidence: milestones/neumorphic-figma/evidence/; manual installation remains a Figma-only design.

## November 7 — current visual and Install UX refinements

The new Figma designs use convex blue/red controls, bold white text on every blue action, centered white vector × on retained red#C64545, white filled Copy glyphs on dark Console and blue copies on light surfaces. Settings gear is at the right: gray#A6ABB2, white outlined glyph when unselected; darker gray#939BA5 with filled white gear when selected. Blue Refresh retains a white cycle glyph. Checkbox outlines are removed and checks restored; already-installed rows use gray faces/checks/labels. Code blocks and inputs use very light gray#EEEFEF; Settings has32px bottom padding.

Install discovery choices, searching, no results, installation progress and failure now appear in five shared popup variants. The main Install page has its file selector plus a separate manual-install command field, Install MCP action and empty shared Console. These are Figma designs only; manual command execution/API behavior is not selected or implemented. Close returns to the main Install specimen. The library has105local main definitions/13sets. Existing app/browser reference pages are retained.

One explicitly requested Rust change is complete: server headers omit authentication status for authenticated/not_required and retain every other status. Six-state native-view render, locked native build and independent review pass. The temporary exporter is removed; Fetch was not launched. Active original and new Figma headers match this source change. Earlier “Rust/original pages unchanged” statements below describe prior passes.

## November 7 — approved navigation/icon changes

All24new native specimens now use circular Settings gears;14Refresh/Refresh servers controls use cycle glyphs, preserving enabled/disabled state;13server×marks are red. Quit Fetch is absent from new navigation. Baseline exact text comparison below is historical: these user-approved label replacements intentionally differ from the current Rust UI. Three final screenshots cover Servers, selected Settings and empty Install; source/runtime behavior remains unchanged.

## November 7 — new neumorphic design coverage

Separate new pages preserve the24current native state names and29existing browser states, using83 copied component definitions. Native text/visibility matches the source-reference specimens exactly. Both screen pages passed editable-layer/component-ownership checks; no full-screen images or zero-width visible text. Giant native page dividers make groups easy to scan. New links remain illustrative and do not execute product APIs. See [new design pages and limitations](FIGMA.md).

## November 7 — current native specimen index

The [main native board](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-487) has24current editable specimens. Existing13main frame IDs23:487 through23:768 remain; their contents now follow current source. Former Connect Cleanup is now a second current command reference; former Advanced Settings is now the current manual-key scrolled view. These replace older native coverage/pending-Figma statements below. The older153-state catalog is historical and is not a current exhaustive-coverage claim.

| Added current specimen | Figma node |
| --- | --- |
| Servers · Console empty | [65:831](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-831) |
| Servers · Sign-in pending | [65:899](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-899) |
| Servers · Remote console | [65:967](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-967) |
| Servers · Terminal running | [65:1030](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1030) |
| Servers · Terminal exited | [65:1087](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1087) |
| Servers · Restarting | [65:1146](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1146) |
| Install · Looking for definitions | [65:1209](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1209) |
| Install · No results | [65:1258](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1258) |
| Settings · Opening server folder | [65:1307](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1307) |
| Settings · Manual key filled | [65:1365](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1365) |
| Settings · Manual key busy | [65:1423](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=65-1423) |

All24screens have editable text/components and no image fills or visible zero-width text. Independent source/visual checking found and corrected empty discovery selection labels and sign-in monospace. Native sample renders and Figma PNGs are in [the evidence folder](../milestones/current-figma-sync/evidence/). Typography substitutions and browser/OS/runtime limits are explicit in [FIGMA.md](FIGMA.md). This finite specimen set does not claim every data value, error combination, desktop interaction or browser branch has been rendered.

## November 6 — remove Connect apps cleanup branch

The manual connected/verified checkbox and revealed cleanup prompt/Copy button are removed from current source. Earlier cleanup specimens are historical and no longer describe a current branch. Both client code blocks retain their separate Copy actions, now beside the commands. Existing Figma specimen counts are not updated or claimed as reconciled.

## November 6 — copyable connection command blocks

Connect apps now renders Claude Code as one command block and Codex as a two-line command block, each with its own enabled Copy button. Source uses a single string for display and clipboard through the shared helper and existing Message::Copy. Endpoint remains derived from the current Fetch origin. No client commands are executed. Native visual/clipboard observation and Figma reconciliation are not claimed; existing specimen counts remain unchanged.

## November 4 — Figma Remote unsupported import

Implemented source change: the official Figma Remote row uses the existing unsupported/disabled selection state and shows its existing row error.message below the checkbox. Figma Desktop and already-installed metadata retain their current behavior. Independent19focused import/install checks (2new), native build and review pass; no Figma specimen update or native screenshot is claimed. Track milestones/figma-remote-unsupported/info.md.

## November4 clicks through a scrolled page

One actual Iced Scrollable+TerminalView event/render scenario covers page scroll before normal click/tiny movement, deliberate shortdrag staying on its clicked row, and plainclick clearing selection. It failed before the pointer-source correction and passes after, alongside10existing terminal cases. Prior direct-widget tests did not cover this parent-input transform. No cursor appearance/Figma specimen-count change; actual rebuilt user native window remains unobserved. See milestones/terminal-pointer-coordinates/evidence.md.

## November4 terminal selection and wheel interaction

Two actual-widget/headless-render scenarios cover deliberate selection, outside release/re-entry without further gray expansion, fractional trackpad capture, older-output visibility, retained scroll on empty output polling and return toward latest output. Both failed on original code; final10focused terminalUI checks2new/native build and independent review pass. Rebuilt native user interaction remains unobserved. No cursor-style or Figma specimen-count change. See milestones/terminal-selection-scroll/.

## November 4 Console/Terminal source states

Expanded servers share Console/Terminal controls. Source branches cover empty/retained Console; URL-only Terminal disabled with reason; local first open; live screen; input/read/close failure; missing older output notice; exited/closed screen with explicit Start terminal; blocked input with available Close; switching/collapse/window reopen; stale responses and per-server independence. Window-free UI, confined backend and actual offscreen prompt/cursor/scroll/clip checks pass under the server-console-terminal milestone. Figma count and native visual acceptance are unchanged.

## November 3 — folder-first Install cleanup

Install now opens with an empty list and visible path/picker controls. Choosing a
path loads discovery immediately; typing one uses Enter/Refresh. Empty Refresh
is disabled. The title has the requested small gray inline folder hint. The long
auto-discovery paragraph, app-wide tagline and accepted-install notice are removed.
Install selected servers uses gray disabled styling while submitting/monitoring;
progress and failures remain visible. The prior automatic-discovery/default-toggle
specimens are superseded. Existing Figma counts are historical; no Figma update
or native screenshot is claimed by this source pass.

## November 3 wrapper refactor — current source changes

Current UI uses one shared masked fixed-key editor in Settings/inline panels,
with stored-but-unverified result distinct from auth. Open-folder is a shared
async native action with per-server busy/error/stale-response handling. The raw
config patch editor is removed; call limits remain. Unknown auth offers explicit
Authenticate, and nullable expiry/last-verified metadata is displayed without
triggering checks. Window-free fixtures cover these branches; current native
window/Finder/Mac confirmation/provider behavior is unobserved. Earlier Figma
node/state inventory below remains historical and needs reconciliation with this
source; no updated visual or exhaustive value-state acceptance is claimed.

# Current UI state coverage

November 3 bulk-selection change: the Install toolbar offers Select all with no
or partial selection, and Unselect all when every selectable row is selected.
The latter clears the selection. Existing installed/unsupported exclusions and
busy/picker/install disabling remain. These labels have not yet been reconciled
with the Figma specimens or observed in a running window.

November 3 server-delete control: a shared × appears beside collapsed/expanded
server headings, with idle/busy/pending/status-refresh tooltip states. Confirmed
deletion clears the card and cached data; uncertain responses keep status checking
without replay. Native compilation and14 existing UI checks pass, but this new
control has not been visually/runtime verified or mapped into the153-specimen
Figma library. Its count remains unchanged.

Updated October 28, 2026. This is the index for the user's request to mock every
currently displayable UI element, including authentication and individual settings.
The official file contains **153 states/specimens**: full page states, reusable
HTML error specimens, widget examples and explicitly labeled native-surface
contracts. It does not enumerate infinitely many server names, values, cursor
positions, error strings or combinations of otherwise covered states.

The first 17 baseline frames remain. Later boards expand them using current source
rather than inventing new screens. The app's authentication UI is an inline panel
inside an expanded server card, not a new product route or modal.

## October 30 independent authentication state coverage

Authentication and credential-change requests now track each server separately.
The 153 Figma specimens and visual layout remain unchanged; no new Figma frames
or native screenshots are claimed. The following source-driven fixtures in
src/ui/auth_tests.rs cover the newly possible combinations using the existing
shared card/editor/buttons and their existing enabled/disabled appearances:

| Combination | Source fixture and evidence |
| --- | --- |
| GitHub starting while Tavily independently finishes | desktop_authenticate_messages_overlap_and_report_independent_results; Desktop messages, no window/Task execution. |
| Same-server repeat/restart blocked while another server remains usable | desktop_repeated_same_target_and_restart_do_not_replace_pending_authentication plus independent-service cases. |
| Settings save while another server authenticates | desktop_settings_save_has_its_own_request_and_clears_the_entered_secret; existing shared masked editor. |
| Old status/request/window response after a newer result | desktop_rejects_status_reads_started_before_authentication_completed and desktop_old_window_and_old_auth_replies_cannot_overwrite_reopened_state. |
| Failed manual save while an older key still authenticates | rejected_manual_save_is_not_approved_by_an_older_valid_server_credential; keep that save's failure visible. |

All14 new UI fixtures pass, including five Desktop message cases. This verifies
state transitions and handler wiring; actual overlapping native interaction,
appearance and scrolling remain runtime checks. The Figma library's individual
phase/control specimens are reused; it does not claim captured simultaneous
service workflows.

## October 29 source reconciliation needed

November 3 D83 source change: installation now tries
the normal Connect operation and report its result through GET servers. A usable
credential-free server with no known login requirement reports not_required and
shows no Authenticate button or inline authentication box, including a stale
previous flow. Unknown alone also does not expose Authenticate. Required/failed
authentication keeps its existing controls; unrelated launch errors retain their
runtime presentation. Reconcile these changed branches and the installation
connection-attempt state in Figma; the recorded153 count and historical fixtures
do not establish coverage of the changed behavior or runtime appearance.

November 3 source change: discovery now distinguishes already-installed entries
from new selectable rows. The installed specimen must show a checked disabled
checkbox and an "already installed" suffix, including when the source is disabled
or has become unsupported. Counts and Select all exclude these rows. Successful
installation progress changes its matching row to this state without rediscovery;
failed siblings remain retryable. This additional rendering branch has not been
mapped into the recorded Figma library or observed in a running window. The
153-specimen count remains unchanged; add/reconcile this specimen in a future
design pass rather than claiming existing visual coverage.


The existing153 specimens remain the recorded library. The separate D80 Railway
callback renderer in src/manager/provider_callback.rs is not traced by the
source/node index below. Its seven distinct response texts (wrong method,
unknown path, invalid callback, unmatched/expired state, stopped Fetch, verified
sign-in and failed sign-in) need explicit specimen mapping or additions. No
Figma edit or live callback inspection was performed in this documentation pass;
do not claim this newer renderer is visually covered or silently change the 153
count. The source-backed original library remains useful within its snapshot.

## Source and evidence

D81 shared-code reconciliation: native rendering now reuses src/ui/components.rs
and theme.rs, while src/browser.rs supplies the shared escaped browser shell,
messages and controls for all three Fetch-owned renderers. Current text/layout
and the 153 reference count are unchanged. The seven Railway paragraph variants
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


D82 changes backend unknown-auth guidance to explain that imported inputs can
be stored/supplied while a trusted verification description is missing. Existing
Unknown inline phase/guidance rendering is reused; no new visual branch or Figma
frame is claimed. Generic input names remain dynamic data in the shared editor.
Actual native/authentication visual acceptance remains unrun.


## November 3 Install discovery changes

The current source starts default discovery on first opening Install MCPs. Default mode hides the path input behind Use a different file or folder…. Explicit mode reveals an absolute file/folder input, Choose file or folder… and Use default locations. Refresh servers replaces Discover servers; an empty in-flight read says Looking for server definitions…. The native picker permits one file or directory and keeps hidden files visible. Existing list scrolling, selection and install footer remain. The 153 Figma specimens predate these text/control changes; no new Figma frames or native screenshots are claimed. Independent Desktop message fixtures and backend fixtures cover behavior; native appearance, picker interaction and updated Figma specimens remain pending.

## November3 standardized Authenticate source follow-up

Cover pending without a link; pending with unopened sign-in host and explicit
Open button; request-in-flight disabling that button; opened host waiting for
completion; completed unknown account access; missing Settings inputs; failed
start; remembered last-known result distinguished from current unknown. Inline
folder/manual-key fallback is removed; Settings still uses shared credential_editor.
Both sign-in controls use /restart interactive:true; legacy /authenticate is kept
deprecated. Source fixtures are being verified; native screenshots/Figma remain
pending and no all-server/account-success claim is made.

## November4 authentication console source states

Continuous-console follow-up: cover always-present expanded consoles for ready, stopped, failed and not-required states, including empty/no-flow rendering. Check passive reads/GET polling do not start authentication or mark rows busy; Authenticate/manual save preserve history; collapse/re-expand/window restore rehydrate it; repeated/cumulative snapshots and carried exit facts do not duplicate; old history cannot hide a fresh local request error. Actual initial-startup-to-auth history is checked with a confined fixture. Independent 150 relevant tests (eight new), locked build and review pass; see milestones/continuous-server-console/evidence.md. Native appearance remains unobserved.

Cover an empty new attempt until actual output; pending host-consent controls without invented log entries; captured stderr/helperstdout/MCPinstructions and returned errors; actual process exit/signal; local request/read fallback removed when service output arrives (including a fallback already cleared by a successful poll); unchanged polls/repeated output/generations; independent instances; Copy. Sign-in/manual Settings authority and state remain unchanged.

Logging source uses one native-only per-attempt capture with explicit output limits/redaction. Actual confined fixtures and library/UI checks are recorded under milestones/service-startup-logs/evidence.md when finalized. These are not native screenshots/Figma or live-provider acceptance. Earlier28/31console results apply to older progress/tail behavior.

Console Copy follow-up: a fixed Copy button beside the commands copies the current server
log lines with newline separators, including lines outside the visible scroll
area. The host-consent control stays in the scroll body. Native clipboard/window
behavior is not observed by source/build checks.
