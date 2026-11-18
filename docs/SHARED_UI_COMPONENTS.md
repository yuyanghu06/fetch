## November 17 — shared key and inline approval

On Connect apps, the approval notice sits immediately right of the matching app's compact key for the approved connection scope. On Servers, the shared key is disabled for unknown sign-in status and the card omits that warning. The key stays available for sign-in required and failed when server changes are allowed.

## November 17 — navigation and install modal

The native navbar has Servers, Connect apps and Workspace. A shared white text button beside the Installed servers Refresh opens a shared modal with the existing install controls. The initial form is compact, then the dialog expands for scrollable results and a fixed install footer. The footer action uses a light gray face and dark gray label when disabled. A shared inline status bar animates while a server is installing and shows the reported result when it finishes.

Selectable unchecked install boxes are white with a light outline. Unsupported and already-installed rows share the same gray box and label, and the installed box keeps its checkmark. A failed installation shows the original error in the shared info tooltip beside **Failed**.

The AGENTS.md and CLAUDE.md editor headers use the shared Paste and Copy controls in that order. The shared key button uses a neutral gray hover glow.

## November 17 — editing existing Claude subagents

The Claude Edit popup uses the shared text inputs for Name and Description above the raw editor. Both change the draft and use the existing Save control.

## November 17 — shared loading overlay

The dimmed Workspace loading overlay reuses the centered panel text layout to display **Loading...**. It keeps underlying content in place while the request runs.

## November 17 — model menu and agent card controls

The existing picker loads from the current app when editing opens and caps the menu at four rows. Agent cards reuse matching 40 px clipboard buttons and a same-size red X. The confirmation uses the shared popup and red action style.

## November 17 — bold Add and compact error marker

Add subagent uses the blue primary button with a bold white label. The shared compact error displays error! and a faint gray info icon that shows the original message on hover. The bulk clipboard icons keep their white faces on press and while busy.

## November 17 — bulk clipboard icons

The Subagents toolbar reuses the raised Paste and Copy glyph buttons before Add subagent. The header keeps the blue Add background and white text. The popup uses the same glyphs as the toolbar.

## November 17 — Add subagent and Paste

Add subagent uses the shared raised text button with the blue background and white text style. The shared popup arranges matching raised Paste and Copy controls side by side. Copy feedback appears to their left. The shared file editor accepts an ID for focusing the blank draft.

## November 17 — startup cards, Finder and empty panels

The shared startup card wrapper waits for each row's turn and draws the whole card as it grows and settles. The shared Finder button uses the accent blue surface and a filled white icon with cutout eyes and face lines. `empty_panel_message` centers the only message in an otherwise empty panel. The expandable file preview uses the existing selectable editor and shared inset styling inside one scroll area.

## November 13 — native conversation and subagent presentation

Workspace consumes complete official messages; tools begin collapsed with name/blue Copy and expand independently. Subagents open a shared modal with parsed settings/Markdown instructions and Edit/Preview, Close and Copy. file_path/finder_button share a compact raised blue Finder control next to supported instruction/subagent paths. Saved success text is removed. Focused popup/API evidence and independent ACCEPT are recorded under milestones/official-session-readers/evidence.

## November 13 — gesture ownership and editor controls

contain_scroll keeps a continuous physical gesture with its initial deepest area; fresh gestures can select a new area. Root native phase forwarding distinguishes finger restart from momentum. Existing stock widgets own their scrolling/focus. Shared Copy has a raised light face and blue SVG for editors/subagent rows/code blocks; dark output Copy retains its dark treatment. Save's disabled/enabled text is white. Shared modal uses48px top and32px other padding to clear traffic lights. Server pop_in accepts a row index for45ms staggered first-load/manual-reload entry. Paste agent and manual project registration controls are removed by user request.

## November 12 — raised project selectors and blue session actions

Project selectors use raised_with_radius with12px corners for their closed surfaces. Their dropdown menu theme and selection behavior are unchanged. entry_card shares the same label/detail layout for sessions and subagents; its primary flag selects the existing blue primary_button for sessions. The session popup reuses code_block for native commands, outputs and tool data, including the existing Copy action. Other prose keeps normal message text.

## November 12 — entry cards, pending panels and menus

entry_card shares the full-width raised label button and muted detail line for subagents/session rows. Session clicks open the existing rounded modal with shared red Close/outside dismissal and raised role-message cards. loading_panel overlays a translucent gray face and blocks interaction while keeping the list's layout. Instructions uses the existing editor directly, without an entry card. picker_menu supplies a warm rounded surface, soft shadow and pale-blue option highlight across project/model menus. Shared is now presented as Available in every project; its underlying scope is unchanged.

## November 12 — Workspace unified circular app choices

Workspace removes the Global/Open folder button row and the separate Claude Code/Codex text-button row. The scope caption moves into the heading, allowing Instructions/Subagents/Sessions and their content to move up. The tabs row now has two circular app-logo choices on its right, each40px to match tab height. The shared app_logo_button uses existing logos and shared button/surface rendering: unselected raised, selected/pressed inset with the existing recessed fill. One workspace.app selection controls Instructions (CLAUDE.md versus AGENTS.md), Subagents and Sessions; existing session API app filtering is reused. Folder picker and typed-path Enter/global-empty behavior remain through their existing handlers.

Four Workspace widget checks pass (3.67s), including real logo clicks and correct app-specific instructions/subagents/sessions. After strengthening selected-face contrast, the same complete selection case passes again (2.41s). Both selected states and normal/minimum screenshots were inspected. Final signed development build-only passes (16.05s), including strict signature verification; git diff --check passes. An initial new fixture case needed relayout after injected API replies; its original failing log is preserved and only the fixture was corrected.

Direct UI work without subagents or independent review for this pass. Fixture API tasks are dropped: no personal files, native picker, clipboard, model process or live account/provider/MCP action is executed. No API/auth/permission/storage behavior changed, no Figma/commit/push/release, and Fetch was not relaunched. User must explicitly Quit/reopen to observe the rebuilt UI. Concurrent project-scoped/Workspace backend work is preserved.

## November 12 — shared approval code, blue Allow and refresh hover

The single approval popup used by both Claude Code and Codex now uses the shared blue primary Allow button. Request code is labeled above the existing shared code block with its Copy control, which copies only the code. Removed the requested Match this code… sentence. Install refresh keeps the shared hover glow/rotation enabled when idle even with an empty path; its existing blank-path handler still performs no discovery and starts no request.

25 existing actual-widget checks pass (9.38s), including request-code Copy, absence of the removed sentence, both app popup flows and empty-path hover animation/no-discovery. Inspected Claude and minimum-size Codex popup PNGs. Signed development build-only/strict signature verification passes (17.68s); git diff --check passes. No new test infrastructure or tests added; existing cases were extended for the requested behavior.

Direct UI work; no subagents for this pass. Fetch was not relaunched. No live provider/MCP, credential/trust/config, API/authentication permission, Figma, commit/push or release changes. Explicit Quit/reopen is needed to see the new executable.

## November 12 — shared server power toggle

Servers use components::server_toggle, a common 44×22 native switch before the key/restart/remove actions. Blue means enabled, gray means Off. UI state sends the common management API, waits/reconciles pending state, and presents returned errors in the matching Console. Connect-specific authentication remains a black 22px variant of the common icon button. No Figma edit. Thirty focused checks, signed build and independent review pass.

## November 12 — Workspace uses the shared controls

The shared Navigation now has Servers, Install MCPs, Connect apps and Workspace.
Workspace reuses page/recessed/raised surfaces, tabs, inputs, disclosures, Save,
text actions, refresh and contained scrolling. Its file editor uses Iced's native
text editor inside the shared input surface; model choices use the native picker.
Scope labels come from API data. No Figma file was edited for this code pass.

## November 12 — black Connect authentication buttons

The two compact Connect apps authentication buttons now have black faces and white key glyphs. They stay 22px, use the same shared icon-button behavior, and do not change the Servers-page key style.

The existing real-widget inline key/dismiss/reopen check passes (1.86s); the normal-size black-button screenshot was inspected. Signed development build-only/strict signature verification passes (15.46s), and git diff --check passes. Fetch was not relaunched. Server on/off work continues separately through the selected specialist roles.

## November 12 — compact Connect authentication and title gap

Connect apps authentication buttons now use the same 22px size as their logos, with proportionally scaled key glyphs. The shared app_auth_button reuses the standard icon-button rendering/behavior; other icon actions remain 44px. Logo and compact button size share theme::APP_LOGO_SIZE. On Connect only, navigation-to-title layout spacing is halved from 56px (32px gap + 24px title padding) to 28px (16px + 12px).

24 actual-widget UI checks pass (9.39s), including clicks/reopening on both 22px app keys and existing server/popup controls. Normal/minimum Connect screenshots inspected. Signed development build-only passes (10.54s), with strict signature verification; git diff --check passes. Existing warnings remain. No new test was added; the existing inline-auth case now locates the 22px controls. Direct work without subagents. Fetch was not relaunched; explicit Quit/reopen is needed to observe the new build. No live provider/MCP, account, credential, personal settings, trust, Figma, commit/push or release changes.

## November 12 — inline app authentication and recessed Connect panel

Connect apps now uses the shared recessed Servers-style panel with internal scrolling and a fixed title/navigation. Claude Code and Codex each have the existing shared gray authentication key beside their heading/logo; the separate Set up buttons are removed. Their commands and Copy controls remain. The entire address/copy-address/certificate/explanation footer is removed. Approval has the shared red Close action at top right, and clicking the dimmed area hides the Fetch popup. Hiding preserves the request and selected permissions, polling leaves it hidden, and the matching app key reopens it without a new request. Shared modal framing gives approval and discovery the same 32px rounded dimming boundary and outside-click dismissal. A stable outer Stack preserves the underlying scroll state when approval appears or disappears.

Final 24 real-widget UI checks pass (9.02s), including both inline key actions, actual close/outside clicks, inside-click preservation, reopening the same code after polling, transparent popup corners, discovery popup corners, copy controls, and normal/minimum screens. Inspected 874×630 and 760×520 Connect and approval PNGs. Final signed development build-only passes (10.90s), including strict signature verification; git diff --check passes. API tasks from these fixtures are not executed.

Work and review were direct, without subagents. Fetch was not relaunched; live appearance requires explicit Quit/reopen. No provider/MCP call, client configuration, credentials, trust, signing selection, Figma, commit/push or release changes. The separate macOS Touch ID/password dialog remains OS-owned; these dismissal controls hide the Fetch dialog. Earlier product/connector acceptance limits remain.

## November 12 — inline refresh and shorter Servers feedback

Installed servers Refresh now uses a 1.5-second minimum indicator and continues spinning if the read takes longer. Install MCPs Refresh is now in the path-input row, immediately left of Choose file or folder…. Its three-second minimum is unchanged. Both use the same shared feedback implementation; the caller supplies the duration. Returned data still applies immediately.

Four focused existing UI checks pass: manual refresh pending spin, instant reload/minimum feedback, discovery click/spin/retry, and normal/minimum-window screens. Inspected the 874×630 filled-path and 760×520 Install screenshots. Signed development build-only passes (15.03s), including strict signature verification; git diff --check passes. No new tests were added for this small adjustment. Direct work/review without subagents. No Fetch relaunch or live MCP/provider calls; running Fetch needs explicit Quit/reopen to show the changes. Unrelated edits, credentials, personal settings and services were preserved.

## November 12 — sync native buttons across pages

Refresh controls on Servers and Install MCPs now use the same shared rendering and running-action guard: blue hover glow, clockwise hover rotation, continued rotation without hover while running, and no second click during activity. Server-list and discovery reads share RefreshFeedback for a three-second minimum indicator without delaying returned results. Server restart continues spinning through its real pending operation and now also uses the component-owned click guard. Text actions across pages, native connector setup/approval, credentials and Terminal reuse text_button; blue Install/Save actions reuse primary_button. Existing specialized copy/tab/disclosure controls retain their common implementations and purposes.

All native page-level button construction delegates to shared components; specialized console-link and number-arrow content still wraps the shared base Button. Copy, tabs and disclosures retain their existing shared behavior. Figma unchanged. [Implementation and checks](../milestones/shared-button-behavior/info.md).

## November 12 — connector approval popup

The native connector popup reuses shared Button, Checkbox, Message and raised-panel rendering. A single popup module presents scope selection and matching request code for both UI-started and command-started requests; it calls the common approval APIs. No new Figma design or independent widget styling is selected.

## November 12 — final reload, copy, startup status and resize UI

Shared app_heading uses bold18px #666B70 text and22px marks to its right. Shared copy_feedback owns local1.4s plain-text confirmation and icon-only hover (blue lightens; white shines), including the address action, with no added label/hover background. Shared pop_in replays cards only after successful manual reloads; header feedback lasts3s minimum. Server identity/start controls use actual pending API/runtime state; only unknown auth is suppressed while starting. Native resize grip follows the visible rounded boundary. Figma unchanged.

## November 12 — inline restart failure

Shared native Server identity now shows gold warning icon + Restart failed beside the status dot/name when that server has a failed restart and no retry is active. This warning uses the existing sign-in typography/icon/color; it takes precedence while failed. Ordinary auth labels remain otherwise. No Figma write.

## November 12 — contain internal scrolling and remove page banners

All native scroll areas use surface::contain_scroll; shared output_frame captures at both content and frame boundaries, retaining native child behavior. Page rendering no longer includes global error/notice banners beneath navigation. Existing contextual Console/dialog messages remain. No Figma edits.

## November 12 — gray key button

Native authentication key face is gray #B8BDC3, retaining white glyph and blue hover glow. No Figma edit.

## November 11 — spin refresh through completion and glow key hover

Shared refresh_button accepts action availability and operation activity separately. A running glyph rotates regardless of pointer/disabled state until the owning operation clears activity; hover rotation remains when idle. Key and Refresh icon buttons share the stronger150ms blue hover halo. Existing native click behavior stays Iced-owned.

## November 11 — remove Retina outline, fix all native hover and align resize

Shared raster decorations retain face RGB in fully transparent pixels to prevent dark fringes under linear Retina sampling. The page face now reaches native bounds; the previous8px outer margin becomes inner padding so content positions stay unchanged. Shared control behavior/shadow stacks remain.

## November 11, 2026 — expansion, refresh hover, left window controls and right shadow

Refresh icon buttons reuse the shared enhanced glow and display-only hover rotation; native Iced buttons retain click handling. Shadow rasters use logical density with linear sampling, while text/icons retain display density. Panel shadows clipped on the right fade only outside the card into the existing gutter.

## November 11 — Console opens on server expansion

The native server disclosure opens Console every time the card expands, including after another panel was selected. Tools, Settings and Terminal remain explicit panel choices while expanded. Expansion only changes the displayed panel; it does not authenticate or start a terminal. Console displays captured server lines alongside current Fetch authentication, runtime and request/read errors. Error codes/messages/details are retained; Copy uses the same assembled text. Separate authentication/runtime messages below the tabs are removed. This supersedes the native explicit-only Console and service-output-suppresses-error display rules below; Figma specimens have not been changed.

## November 9 — fading hover glow without text

Native Tooltip wrappers are removed from server actions/identity/Terminal and legacy shared header. Enabled filled shared Buttons now draw a subtle blue halo from one cached decoration, smoothly fading in over150ms through normal redraw events. Idle/disabled controls have no halo; selected controls glow only while hovered. Native input/click behavior remains Iced-owned. Default native window is874×630; no Figma edits. Existing Console line spacing/Copy data remain; only development cosmic-text/swash font work is optimized.

## November 9 — tool-grid shadows and frameless app

Shared native tool_grid preserves three equal-width columns/24px gaps/325.333px card heights with Row/Column so shadows are clipped by the actual scroll viewport, not a tight Grid rectangle. Shared Navigation rendering is wrapped by the page caller with native window-drag behavior on unused title/space; tabs retain normal input capture. User selected a transparent, fully frameless native window. No Figma edit.

## November 9 — fixed native Installed servers section

Servers reuses shared page/recessed/server-card surfaces. The section fills remaining window height with its title/Refresh outside the dedicated card scroll area. Card/panel content changes its own scrollable length, keeping the section and navigation fixed. Existing nested Tools/Settings/output and action rendering remain shared. This is a native layout change; no Figma write.

## November 9 — filled Save icons

Save vectors in shared credential actions now use white fills and blue #95D4F1 outlines/details in both enabled and disabled variants. The enlarged enabled-button screenshot was inspected. Figma only; Remove × and app source unchanged.

## November 9 — inline square key actions and outlined information

Save and Remove are now44×44 square icon buttons alongside the credential field: blue with a white Save vector, red with the existing white ×. Shared action set246:2464 covers enabled/disabled states, retaining prior0.58disabled opacity. All four active key editors use one44px row, with916pxfield plus two actions. Figma Remote rows are gray in all three actual discovery screens; screen overrides were corrected using resolved gray RGB values rather than black fallback colors. The information control is now an unfilled gray circular outline with a gray i, including hover state; its exact tooltip message is retained. Current key-row and discovery-dialog screenshots were inspected. Figma only; no Rust changes or subagents.

## November 9 — gray unsupported row with information tooltip

Figma Remote · remote is now gray in the Results, Installing and Failed discovery dialogs, including its unchecked checkbox and label. Its exact approval explanation moved from the separate line into a shared information i hover tooltip (set234:2431; Default234:2421, Hovered234:2426). The row remains unsupported/disabled. Results dialog and tooltip screenshots were inspected. Figma-only, with no Rust changes or subagents.

## November 9 — key visibility eye control

The shared API key entry now includes a small right-aligned eye button. Click switches between Hidden and Visible variants; the visible state uses a crossed-out eye to hide again. Existing name/colon emphasis and placeholder colors remain. Component set231:2987 retains original hidden master226:2385; visible master231:2978. Figma uses the disposable example-token-1234 value for the filled-key reveal demonstration. Empty/named/busy forms retain their placeholder when switched. This specifies the requested UI behavior; native reveal implementation is unchanged.

Shared-state and filled-key specimen screenshots were inspected; reciprocal ON_CLICK → CHANGE_TO links were read back. No actual user credentials were read. No subagents or application source changes.

## November 9 — requested label emphasis

Shared tool-card labels Name and Required fields are now Inter Bold. Shared key-entry name and colon are Inter Bold; the API key or token placeholder is grayer (#8F9499), using neo/credential-placeholder. Filled masked-key text retains its previous color. Tool-card and key-entry screenshots were inspected. Figma only; no source changes or subagents.

## November 9, 2026 — restart button animation

User requirement: while a server is restarting, its restart button's circular-arrow icon should spin. Recorded only in this pass; no Figma animation or Rust implementation was changed or verified.

## November 9 — restarting status on hover

The separate Restarting message below the server card is removed in Figma. Starting/restarting headers retain the compact yellow dot normally; hover shows an information i and the exact message in a dark tooltip. Shared set228:2391 (default228:2384, hover228:2386) is used in both Starting header variants. Restarting master77:1507 supplies “Restarting… waiting for this attempt to finish.” through its Status message property. Current app specimens inherit it. The tooltip and Restarting screen78:1782 were visually inspected; the temporary hover check frame was removed. This is Figma-only, performed directly without subagents. Other runtime-color headers were not changed. The manual-install background remains only a recommendation.

## November 9 — requested Settings and tool-field edits

User-directed Figma edits only, performed by the root agent without subagents. Reverted the earlier self-chosen wording/layout/hover additions: original Install/Connect/settings wording restored; six new hover variants, bold close glyph, seven hover styles and seven hover links removed; earlier control effects restored. The removed key-selection buttons were recreated for rollback, then explicitly removed again at the user's request.

Current requested design: four active Settings credential states say **API keys/fixed tokens** and have no description. Shared API key entry component226:2385 places entry name, colon and token in one44px row; filled keys stay masked. Remove entry is red with bold white text. Search Settings viewport height is498px so the full form/actions fit. Twelve authentication-control instances on current Settings screens explicitly use key glyph132:1667, correcting the gear overrides. Shared read-only tool fields182:3991 and the two fields in tool card146:2456 use **#EAF5FB**, through neo/tool-field variable226:2384. Original reference pages and Rust code were not changed.

Visual checks: current filled-key app specimen78:1875 shows full form/actions, requested title, paired field, red Remove and key icon; tool card146:2456 shows pale-blue fields. Both screenshots were inspected. Checks were direct, not independent. This is Figma-only; it does not establish native implementation or runtime behavior.

Latest design recommendation, not applied: distinguish the manual-install section using the same recessed warm-gray section background as Installed servers, retaining a light command field and dark console. Wait for the user's selection before changing it. Continue applying only the user's specific edits.

## November 9 — Figma edits are user-directed

The user clarified that they will specify each Figma change. Apply only those requested edits, quickly and without subagents; do not choose additional UX changes independently.

Before this clarification, the root agent changed shared Install/manual-key instructions, timeout/simultaneous-call labels, discovery hint layout, and Connect wording; added six hover variants, a bold close glyph and seven hover transitions; removed four redundant Search key-selection buttons after the user's question. Selected/active effects were made inset without drop shadows. The current Figma canvas is newer than the November 8 repository descriptions (server Settings are already inside server cards). These changes have structural tool evidence but final visual review was interrupted by the user's clarification. No Rust source, running app, credentials or client settings changed. No independent agents were used. Do not describe this pass as visually accepted or continue its self-chosen edits.

## November 9 — precise shared rendering replaces the glossy approximation

surface.rs now paints flat fills with the full Figma shadow stacks through the existing pinned resvg implementation. Its Button/TextInput builders delegate actual native behavior to Iced. Decoration is cached per widget and rendered at the window's current display scale. Straight RGBA conversion fixes the SVG transparency issue; retained in-memory images, one image filter batch and a later foreground layer keep backgrounds, icons, checkboxes and number arrows correctly visible from the initial frame. No invented diagonal gradient or hard rim remains.

Shared typography uses actual Inter400/500/600/700faces. Navigation, tabs, raised panels/controls, blue actions, selected inset surfaces, gray inputs,32px tool cards,12px read-only details and glyph slots follow current Figma values. Scrollers are hidden at rest and neutral gray while used; scroll behavior is unchanged. Complete tool JSON is preserved behind the two displayed fields; the earlier extra Raw definition control is removed to match the design. [Verification/limits](../milestones/neumorphic-fidelity/info.md).

## November 9 — native shared components now implement the design

src/ui/components.rs and theme.rs share the three-tab Navigation, text/round buttons, selected/output tabs, inputs/checkboxes, page/server/tool surfaces, read-only tool values, code Copy glyphs and Console/Terminal framing. surface.rs forwards the wrapped widget's layout/events/overlays/operations while drawing paired panel shadows and narrow raised/recessed rims. Selected controls have no resting glow; hover can glow, and the removal × thickens on hover. Disabled removal stays saturated red.

Native Settings reuses the same credential_editor and existing API operations. Number arrows use the exact local Figma8×5SVGs and existing timeout/concurrency bounds. Console/Terminal remain gray tabs. Tool fields use current Figma pale-blue#EAF5FB, superseding earlier warm-white prose; values wrap in narrow cards and full raw definitions remain accessible. Bundled Inter regular/semibold/bold and Roboto Mono keep typography portable. No font is installed globally. See [scope/evidence](../milestones/neumorphic-dev-sync/info.md).

## Current — shared recessed sections and inline Settings

Recessed section surface197:4166 is reused behind all22 new-design Installed servers sections; its warm-gray fill and Inset effect are shared, with raised server cards above it. Stage/window sizes are consistent across those states. Settings card family202:2558 has five data-preserving variants: Filesystem ready, Search ready, folder opening, key filled and key busy. Existing Button/Text input/manual-key editor components remain reused; selected Settings uses Selected172:1910 with inner shadows only.

Settings forms use a1072×440 scrolling panel. The former separate Settings specimens are now inline states; no navbar Settings control or new-design Settings page remains. Shared Navigation77:1401 has Fetch plus three tabs only. Source/reference navigation remains unchanged.

Number input212:2318 adds value text and small up/down arrows at the right. Two500px inputs share a row with24px gap; bold17px Time until timeout / Max concurrent calls labels have20px label-to-input spacing. Generic heading/long field descriptions and requested helper sentence are removed. Values120/1 remain unchanged; arrows are visual prototype controls, with no new execution/bounds rules selected.

Light Copy115:1371 has no background/stroke/effect; its blue glyph and40px area remain. Dark Console glyph treatment is retained. Actual prototype clipboard actions were not present/tested; do not claim runtime copying from appearance. [Current scope](../milestones/inline-server-settings/info.md).

## November 8 — recessed tool area and display-only details

Shared Available tools area177:1914 is warm-gray/inset with vertical overflow and a padded wrapping grid153:1930. Six demo cards occupy three columns/two rows; each is325.333px square with24px gaps/padding. Shared read-only value182:3991 replaces the gray input instances inside Tool card146:2456. The new warm-white raised value has a Value text property and no editing/action behavior. Original full raw Tool JSON remains hidden/preserved; five added definitions are illustrative fixtures.

Navigation77:1401 reuses Selected172:1910 for the active blue/bold-white tab with two inner shadows/no glow; other tabs remain raised. Existing links/layout persist. Installed servers recess and larger windows are trialed on the two tools screens, using shared warm-gray fill alias and Inset style. No Rust/source-reference change. [Current demo](../milestones/recessed-tools-demo/info.md).

## November 8 — explicit panel controls and inset selection

New-design shared Button set75:2252 adds Selected172:1910. Selected controls stay enabled and use two INNER_SHADOW effects from the existing Inset style, without glow or raised DROP_SHADOW. The Label text property is bound on the selected variant; disabled remains separate. Review172:3525 shows Inspect tools, Settings, Console and Terminal selected.

Expanded default/Tools/recovery/sign-in/restarting cards hide output by default. Explicit Console-open specimens override output visibility while preserving their state. Individual controls own prototype actions; Console/Terminal parent rows no longer act as Console links. New-design tools have no automatic Console. Original source/reference components remain separate.

The shared round restart replaces separate recovery Restart / Connect and Force restart buttons. Normal click restarts/connects; Shift-click force restarts (may interrupt work), recorded in shared actions132:1678 guidance. Figma cannot execute that modifier-click distinction. Terminal77:1453/77:1469 fill1072px parents. These are selected Figma design requirements for a later Rust port, not new native behavior.

## November 8 — selected and hover states

Selected controls must not glow. Glow appears only while hovering, including on the selected Settings gear shown in the user’s reference. Hovering over the red × must make the × bold. These rules update the intended shared control states; this note does not establish that Figma variants or app code have been updated.

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

## November 7 — shared round icon actions

New navigation removes Quit Fetch and replaces Settings text with a44pxgear. Shared Icon button set100:1473 has Raised/Active/Disabled states and a swappable icon. Settings glyph100:1456 and imported Refresh cw100:1462 preserve common round geometry/shadows; active Settings stays blue, disabled Refresh stays dim. neo/danger#C64545 colors server×marks without changing deletion availability. New local component count88/11sets; old code/reference components are unchanged.

## November 7 — independent neumorphic component copies

The [new component page](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=75-2251) copies all 83 main component definitions, with separate variables/styles, plus one new shared Window surface (84 main definitions total). All24 new native and29 browser specimens point to these copies, including nested instances. Legacy inline-auth families remain copied and labeled Legacy; new current screens use the active families. [Review sheet](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=94-1300) demonstrates raised/hover/pressed/disabled buttons, inputs, checkboxes and navigation.

Shared design values: warm white#F7F6F1, blue#95D4F1, Inter/Roboto Mono, convex rounded edges and softened white highlights. Existing current-code components and Rust source were not changed. These are design references for future implementation; current and proposed UI must remain separate until then.

## November 7 — Figma reconciled with current native source

The current native screens now reuse the updated Navigation (no tagline), Button, Checkbox, Text input, Server card, Tool disclosure and Settings-only Credential editor. Updated source sizes/gaps are retained. New shared families: [Console](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=60-747), [Terminal](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=60-752), [output controls](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=60-753), [command block](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=60-758). Server card family27:370 includes remote, terminal running/exited, sign-in pending and restarting variants. Source modules remain unchanged.

Older inline-authentication component families are labeled Legacy; they no longer describe the app. Manual credential editing is Settings-only, with Save manual key and Remove entry. See [current Figma scope and font limits](FIGMA.md); older registry paragraphs below remain historical where superseded.

## November 6 — remove manual cleanup confirmation

Connect apps no longer renders the connected/verified checkbox or its conditional cleanup prompt/Copy button. The two shared client command blocks remain. No original MCP entries or client settings are edited by this change.

## Copyable command blocks — November 6

components::code_block shares the padded rounded container, monospace command text and Copy button beside the commands for Claude Code and Codex on Connect apps. The caller supplies the complete command string and existing clipboard callback. Height follows content; common padding, spacing, button and text size use the existing theme. Figma reconciliation and observed native clipboard/appearance remain separate.

## November 4 unsupported import reasons

Import rows reuse the existing checkbox and components::message to show each server's supplied error.message. Unsupported rows remain disabled through the shared installable check. Figma Remote adds data/reason to this existing pattern; no new component styling or Figma edit is selected.

## November 4 Console/Terminal source

components::output_controls shares Console and Terminal buttons for every expanded server; caller supplies actions and local/remote availability. components::terminal_pane provides the shared terminal surface. server_terminal.rs supplies the retained iced_term model/API state; authentication_view supplies the existing persistent Console. This extends current source, without claiming Figma/native visual reconciliation.

## November 3 wrapper refactor — current source

The shared credential editor now handles only manually entered fixed keys saved
in Keychain. It says stored, not authenticated; account evidence is separate.
Settings/inline views share it and the async Open server folder control. The JSON
server-config editor is removed; Fetch call limits remain. Authentication supports
explicit native setup/online sign-in, unknown states, and connection expiry/last
verified text. Earlier Figma specimens below are historical source references;
new folder/stored/expiry states need design reconciliation and native visual proof.
No Figma file was edited or native appearance acceptance claimed in this pass.

# Shared UI components

Confirmed October 28: commonly reused UI elements must be shared components in
Figma and code. The user subsequently rejected the styled mockups and requested
literal copies of the current app. This registry now follows that correction.

Use the [current component page](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=20-485).
The earlier styled components are archived; their appearance is not approved
for implementation.

The source-state expansion also defines shared
[Credential editor](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=34-438),
[Inline authentication](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-556),
[Authentication server card](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=36-929)
and [Browser connection error page](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=42-740)
families. See [all state coverage](UI_STATE_COVERAGE.md).

The credential editor is already shared in current code through
src/ui/authentication.rs::credential_editor and is used by both Settings and
inline authentication. Keep that common implementation: exact labels,
10-pixel spacing, masked input, enabled conditions and optional Remove action.
Source code changed concurrently during this design pass; the nine-phase
authentication module, including NeedsVerification, is represented in Figma.

| Current family | What is shared | Caller supplies |
| --- | --- | --- |
| [Navigation](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=22-484) | Fetch text, five ordinary purple navigation/Quit buttons and the existing tagline | Navigation and Quit callbacks. The current source has no active-page pill treatment. OS window controls remain OS-owned. |
| [Button](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=21-492) | Current Iced primary appearance, 5/10 padding, small corners and enabled/disabled states | Label, action and enabled state. Preserve explicit label-size overrides for server/tool disclosure buttons. Current Remove entry is also an ordinary primary button. |
| [Text input](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=21-493) | Current full-width Iced input with 5-pixel padding and 2-pixel corners | Value/placeholder, input callback, width, enabled state and masked-entry behavior. Descriptions remain separate text in the current layout. |
| [Checkbox](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=21-508) | Current 16-pixel Iced control, 8-pixel label gap and selected/unselected/disabled states | Label, selected state, enabled state and toggle callback. |
| [Server card](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=27-370) | Gray rounded container, disclosure button, ID, action row, plain errors and call counts | Identity, runtime/authentication labels, expansion, available actions, errors and counts. Figma has collapsed, expanded and recovery variants. Code must retain the source's actual combinations rather than hard-code three sample states. |
| [Message](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=29-346) | Plain inline message text or the existing padded error container | Text and applicable context. Do not replace it with a colored alert panel without a separate UI change. |
| [Tool disclosure](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=29-354) | Current tool disclosure button, 10-pixel surrounding padding and raw JSON text | Tool name, expansion and preserved raw definition. |

Browser HTML keeps its own shared controls:
[checkbox](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=32-355)
and [button](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=32-356).
Do not apply the native app's purple Iced appearance to browser-default controls.

Status labels are ordinary text inside the server disclosure and progress rows.
The earlier standalone colored Status badges are not part of the current app.
The archived Window header, Field and Notice proposals likewise do not override
the actual Navigation, Text input and Message structure above.

## Required use in code

- Implement repeated native rendering in shared Rust/Iced functions or modules.
  Pages pass data, current state and callbacks rather than copying widget trees
  or styles. Browser pages use shared HTML helpers and appropriate common styles.
  Reuse within each platform; no new framework is selected.
- Keep common colors, typography, padding, gaps and corners in the shared theme,
  matching the current Iced Light defaults until an explicit UI change is made.
  Update shared components when changing their appearance, then inspect all
  pages that reuse them.
- Preserve current hierarchy, text, scrolling and availability rules when
  copying the app. New layouts or cosmetic treatments are separate changes,
  not routine translation from code to Figma.
- Keep API calls, authorization, validation, persistence, process control and
  workflow decisions in their existing owning layers. Common components own
  repeated rendering, not a separate backend.
- Use options and composition for real state combinations. Mockup names, paths,
  sample call counts and credential placeholders are fixtures, not production
  component defaults. Continue to mask private entry.
- Extend this registry as repeated patterns appear. Do not force unrelated
  controls into one helper solely to share code.

## Current implementation state

November 3: components::server_header pairs the existing disclosure with a small
× button and tooltip using common theme size/padding. It is shared across all
server rows; the caller supplies busy/pending availability and deletion action.
The button calls the existing DELETE API. New Figma specimen reconciliation is
pending; a passing native build does not establish observed appearance.

Shared Figma components and linked instances exist. Under D81, native code now
uses src/ui/components.rs and theme.rs for repeated navigation, controls, page
framing, messages, server cards and disclosures; the credential editor remains
shared. src/browser.rs supplies escaped document/message/control helpers used by
client approval, product provider callback and Railway callback rendering.
Existing layout/labels/light defaults/masking/availability/scrolling are preserved.
The native build and68 fixture checks pass; no actual native visual or live login
acceptance is claimed. Code Connect and the Railway Figma specimen follow-up are
still separate reference work.

November 3 discovery usage: an already-installed row reuses the existing checked
disabled Checkbox state and appends "already installed" to its label. It is not
part of the active installation selection or count. New supported rows retain
ordinary checked/unchecked selection and explicit opt-in for source-disabled
entries. Keep the installed-page specimen reconciliation tracked in
[UI_STATE_COVERAGE.md](UI_STATE_COVERAGE.md); no new Figma/runtime evidence exists.
The shared components::installed_checkbox and theme::installed_checkbox functions
also dim the label using the theme's text color, because the default Iced disabled
style grays the box only.


Review populated, empty, busy, disabled, error, expanded and scrolled states when
code changes are selected. Preserve the fixed Install footer and actual long
Settings layout. Existing authorization still governs builds, tests and runtime
work. See [FIGMA.md](FIGMA.md) for screenshot evidence and typography limits.

October 30 authentication availability follow-up: the shared credential editor and
card layouts are unchanged. Their callers now supply per-server request state,
so a request for one server leaves another server's Authenticate/key-save controls
available. Same-server actions and stale replies remain guarded. The source
state combinations and14 passing UI fixtures are recorded in UI_STATE_COVERAGE.md;
they do not add Figma frames or establish actual native visual acceptance.

November 3 Install cleanup: the Install footer still uses the shared Button with
its existing enabled behavior. Its theme style uses gray for disabled status and
the ordinary primary style when enabled. The small gray title hint also uses a
shared theme function. The app-wide tagline is removed. Figma reconciliation is
pending; the existing button family remains the reference for other controls.

November3 Authenticate follow-up: the inline authentication panel no longer embeds
the manual credential editor or unconditional folder action. It uses the shared
Button for Open sign-in at HOST after the backend supplies a pending host. Manual
values remain in Settings through the same credential_editor. Existing prose
about inline editor usage above describes the previous source. Figma specimens
need reconciliation; no visual acceptance is claimed by this source pass.

## Authentication console — November 4

The shared console is always present in an expanded server row, even before authentication or when no output exists. It refreshes through ordinary status reads and never starts authentication merely by being displayed. Captured service history spans startup through later authentication within the same Fetch run, including collapse/window reopen. The existing memory limit applies to that cumulative history; no log files are added. A new request failure stays visible alongside old logs until the service supplies new output for that request.

One shared Console uses compact monospace text, dark background and internal scrolling. It wraps actual startup service logs/instructions/errors and observed process exit status from the native-only memory capture. Synthetic Fetch authentication phase/host-progress prose is removed. Genuine local request/read errors are fallback only until actual service output arrives; old fallback lines are then removed. In-console host consent, row controls and shared Settings credential editor remain. Credential values are redacted before retention; successful protocol/tool/token bodies are excluded. Capture retains the beginning up to1MiB/4096lines,64KiB perline, with explicit notices. No logfile or native/Figma appearance proof.

November 4 Copy follow-up: Console includes a small fixed Copy button beside the commands,
using the shared button and console-action style. The caller supplies an action
that copies all displayed log lines in order, separated by newlines. The header
stays visible while the console body scrolls.

## Workspace rendering — November 16

`file_content` is shared by instruction editing and the read-only subagent instruction preview:12px monospace, inset input surface, contained scrolling and common padding. Edit still opens the exact full definition. `file_path` reuses the code-box surface, wraps long paths, copies the exact path and places Finder immediately after the box. Subagent name actions use the shared blue primary button. `conversation_message` uses theme-owned white User/gray Assistant fills. Normal session text uses cached Iced Markdown; tool output remains exact in its disclosure. `frameless_copy` is a transparent blue SVG action at the tool card top right; `code_block_without_copy` avoids a duplicate output control. These are code-only changes; no Figma edits or personal-definition formatting.

## Compact paths and tool cards — November 16 follow-up

All Workspace instruction/subagent paths reuse `file_path`:8px vertical padding and a bare14px blue Copy SVG in a28px transparent hit area, with Finder adjacent. This replaces the earlier raised path Copy and feedback-width reservation. Other application Copy controls keep their existing size/style. `code_disclosure` reuses the inset code-box surface for collapsible session tool names. Tool cards reuse the assistant-gray message surface. JSON object/array string output gets a cached indented display with glyph wrapping; raw History/Copy content stays exact. No native reader/API or personal-file changes.

## Session Markdown and Copy — November 16 follow-up

`session_markdown` reuses cached Iced items with the supported Viewer override: fenced code calls the exact `code_block_without_copy` helper (shared13px monospace/light inset/glyph wrap); inline code uses normal14pxBODY_FONT/INK with CODE_BACKGROUND. Headings/lists/quotes remain native Markdown rendering and link actions stay inert. Tool-card Copy now emits displayed tool name + two newlines + original result, explicitly superseding earlier result-only behavior; the pretty display cache never supplies clipboard text. Opening window defaults to measured1058x789logical units, min760x520.

## Popup Escape behavior — November 16

The native window event subscription handles Escape for current Fetch popups, including events captured by focused controls. It reuses the existing Close action for exactly the topmost visible popup, in the same order as drawing: connector approval, Workspace subagent, Workspace session, Install discovery. State stays retained as with Close; no-popup Escape has no action. Any new Fetch popup must be included in this central dismissal order with its own existing Close action. OS-owned dialogs keep OS keyboard behavior.
