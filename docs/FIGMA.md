## November 12 — shadow made visible at Dock size

User’s Dock screenshot looked flat beside the large Figma reference. A96px native check preserves the exported image closely (mean0.156331/255,max6), so the available evidence points to faint shadows after reduction, not image loss in AppKit. Live Dock compositing was not independently captured. Kept large mockup267:800 unchanged and created canonical app-icon export310:799 with a wider/darker bevel and outer shadow for small-size readability. Same824px tile at100,100, white fill, dog and corners. PNG/ICNS regenerated72DPI.96px native after-preview inspected and fixture passes(mean0.162326/255,max6); signed build-only26.85s/strict verification passes. Existing running Fetch preserved; user Quit/reopen required.

## November 12 — app icon refreshed from current Figma

Refreshed canonical packaging/Fetch.png and Fetch.icns from the current source logo via export308:799, retaining rounded outward rim and824px visible tile on1024px canvas. PNG remains72DPI; source mockup unchanged. Signed development rebuild passes17.43s and strict verification passes. No Fetch process was found during the read-only precheck and no app was launched. Open the rebuilt target/debug/fetch-development to load the icon. Earlier native assignment checks remain applicable; no redundant fixture rerun for this unchanged design refresh.

## November 12 — rounded outward edge clarified

User clarified “rounder edges” means a curved outward rim across the tile surface, not deeper recessing or another corner-radius increase. Replaced the recessed lighting with a soft raised bevel: inner white highlight40px/20,24/spread10/88%, inner gray42px/-22,-26/spread12/25%, restrained outer shadow18px/6,8/18% and white lift16px/-6,-8/55%. Existing230px smoothed corner shape, white fill, dog793px size/position/artwork/shadow and824px Dock tile footprint retained. Latest export306:799; recessed export304:799 archived. Canonical PNG/ICNS updated72DPI. Composition/canonical/native preview inspected; existing native fixture passes (mean0.174911/255,max15); signed build-only6.89s/strict verification pass. Direct checks; no Fetch relaunch or packaging.

## November 12 — rounder recessed icon and corrected Dock footprint

User requested corners like the neighboring Dock icon, clearer inward indentation, and comparable visible size. Source logo267:800/background283:799 now use230px corners with0.6smoothing. Background has only two inward effects: dark upper-left36px blur/10px spread at16,20 with32%opacity, and white lower-right30px blur/8px spread at-16,-20 with90%opacity. White fill, dog artwork, dog793px size/position and dog shadow retained. Latest export304:799 is1024px with an824px tile at100,100, reducing the previous roughly910px visible face. Previous export297:799 archived. Canonical PNG/ICNS regenerated at72DPI; native fixture/resource checks and signed build6.55s pass. Native preview inspected; direct/sequential checks, no independent new review. Fetch not relaunched; actual neighboring Dock comparison awaits user Quit/reopen.

## November 12 — approved retriever is the app icon

User approved the defined-ear, midpoint-blue retriever with pink tongue and larger dog shadow on the warm-white tile. Canonical exports are packaging/Fetch.png (1024px,72DPI,alpha) and Fetch.icns (ten standard sizes), from Figma export297:799; original mockup267:800 unchanged. Native development embeds the PNG and assigns AppKit’s applicationIconImage on the main queue. Existing future-bundle source names/copies the ICNS. Focused independent native/resource checks, final signed build-only and strict verification pass; independent Evaluator ACCEPT. No Fetch relaunch or bundle/DMG/release run. Explicitly Quit/reopen Fetch to observe the Dock icon; live Dock/Finder appearance remains unobserved.

## November 12 — clearer ears and larger dog shadow

Dog ears now have deeper blue tone and clearer curved attachment shading. Midpoint-blue face, black eyes and smiling mouth/tongue retained. Figma dog group285:801 shadow expanded from14px blur/6px offset to28px blur/10px offset,2px spread and18% opacity. Image283:800 geometry compares unchanged (793×793, absolute237,235); white/background compare unchanged. Screenshot inspected; saved docs/logo-mockups/fetch-dog-defined-ears-shadow-v8.png (821×821 exported bounds). Prior variants retained.

## November 12 — subtle dog drop shadow

Added one soft shadow to dog cutout group285:801:14px blur,6px downward offset,16% blue-gray opacity. Dog PNG/color/expression and793×793 absolute237,235 image bounds retained; white/background treatment unchanged. Composition screenshot inspected. Saved docs/logo-mockups/fetch-dog-midblue-shadow-v7.png (821×821 including shadow bounds). Unshadowed v6 remains available. Effect remains separately editable in Figma.

## November 12 — midpoint blue logo

Dog PNG now targets #9DD3ED, midway between prior #7DBFE2 and #BCE7F8. Existing simplified face, black highlighted eyes and pink tongue retained. Updated Figma283:800; before/after size and position compare unchanged (793×793, absolute237,235), as do white background/effects/corners. Screenshot inspected. Saved docs/logo-mockups/fetch-dog-midblue-tongue-v6.png,793×793 transparent cutout; earlier variants retained. Generated color target is approximate.

## November 12 — light blue dog with tongue

Returned the simplified retriever to pale baby blue (target #BCE7F8) and added a compact open smile with a pink tongue. Black highlighted eyes retained. Updated Figma image283:800; exact793×793 size and absolute237,235 position restored/verified. White background fill, effects and114px corners compare unchanged. Composition screenshot inspected; transparent cutout saved as docs/logo-mockups/fetch-dog-light-tongue-v5.png (793×793). Prior variants retained; this is a proposal awaiting feedback.

## November 12 — simplified retriever shading

Reduced the heavy muzzle and ear shadows into shallow broad shading and a simple mouth curve; stronger blue and black highlighted eyes retained. Updated Figma image283:800 in logo267:800. Before upload the current dog rectangle was793×793 at117,115 inside group285:801; restored that exact size/position after upload and checked absolute bounds237,235,793×793. Aligned the native silhouette mask to the smaller image. White background, effects and114px corners compare unchanged. Composition screenshot inspected. Saved new transparent PNG docs/logo-mockups/fetch-dog-simplified-v4.png (793×793, alpha); previous PNGs retained. Proposal awaiting visual feedback.

## November 12 — separate PNGs and revised logo contrast

Logo267:800 now contains a separate native warm-white background283:799 and transparent dog group285:801 (image283:800 plus silhouette mask285:799). Exported fetch-dog-transparent.png (1024×1024) and fetch-logo-background.png (1094×1098 including external shadow) under docs/logo-mockups; both have alpha and were visually inspected. Dog recolored to stronger sky blue (target #7DBFE2), with black eyes and tiny white highlights. White fill #F7F6F1 retained. Background retains114px corners; local inner effects broadened to44/56px and close shadow softened to12px. Shared app styles unchanged. Before/after dog rectangle is1024×1024 at0,0 with identity transform; mask and group geometry retained on the final swap. No automatic logo adoption.

## November 12 — lighter retriever and brighter eyes

Revised the rejected logo concept using built-in imagegen: lighter pastel-blue dog (target #BCE7F8), deep-blue eyes with small white catchlights, same silhouette and warm-white background. Updated image fill on Figma267:800 and inspected the composition screenshot. Current frame has114px rounded corners and the existing Raised panel effect style; preserved as found during final readback. New source saved as docs/logo-mockups/retriever-pastel-blue-bright-eyes-v2.png; v1 retained. This is a revised raster proposal, not user acceptance of the logo.

## November 12 — corrected raised logo frame edge

The earlier corner-only edit missed the requested soft raised boundary. Logo frame267:800 now uses the app window’s existing **Neumorphic / Raised panel** effect style (two inner shadows and three outer shadows), plus shared neo/radius/32 corner bindings. Image fill and dog remain unchanged: the shadows belong to the surrounding frame. Figma screenshot inspected. This supersedes the previous zero-frame-effects state; original local PNG remains unchanged.

## November 12 — rounded logo boundary

Logo mockup267:800 now uses32px rounded corners and clips the image to that boundary, matching the current Figma app shell/window surface97:2015 and90:1605. The1024px image, blue retriever and no-shadow treatment are retained. Readback and screenshot inspected; direct visual check, no subagents. Figma only; original PNG and application source unchanged.

## November 11 — retriever logo mockup

Created the requested **logo mockups** Figma page267:799 with square1024px artwork267:800: a light-blue retriever head, warm off-white background and no exterior shadow. The face keeps its soft sculpted shading. Imported generated raster artwork, not editable dog paths or a final adopted logo. Figma image fill/effect readback and composition screenshot inspected. Built-in imagegen used; target palette #95D4F1 / #F7F6F1, with generated shading rather than a pixel-exact color guarantee. [Figma](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=267-800); [saved asset](logo-mockups/retriever-blue-warm-white-v1.png). App source and existing Figma screens unchanged. Direct design work; no subagents.

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

## November 9 — remove gloss and compare the actual pixels

The user rejects the earlier approximate gradient/rim appearance and requires the actual design. Current source uses uniform design fills and all shadow layers from the fresh high-fidelity Button75:2252, Navigation77:1401, server202:2101 and app78:1438 contexts. Matching-size references are preserved in [neumorphic-fidelity](../milestones/neumorphic-fidelity/info.md).

The Button set is compared at524×208with exact component positions; native content is compared at1248×1128against the Figma window after excluding its32pxOS titlebar and40pxpresentation margin. The static Button state sheet is a style specimen; it does not prove literal font matching during every live gesture. Source effects/geometry are exact design values; different font-edge and blur rasterization can still change individual pixels. Measurements, not an invented threshold, describe those residuals.

Navigation uses28pxSemibold/40pxline, text tabs14pxMedium/20pxline; primary/folder labels follow22pxline variants. Server card gaps14px, list20px, footer22px; tool cards have32pxcorners and muted labels with24pxfield-group spacing. Numeric arrows sit inside a44pxgray field with24×36pxwhite strip. Read-only values use actual pale-blue#EAF5FB, not the older prose description. No extra Raw definition text action appears in cards; full original JSON remains in API/state.

The correction retains native input/actions and original SVG bytes. Its shared renderer fixes transparency and draw-order problems found in actual captures. No native window/account/MCP or signed launch is established by those captures.

## November 9 — native development implementation

The approved native neumorphic design is now ported into src/ui. The implementation uses the high-fidelity current Figma context/screenshots for Tools78:1438, inline Settings78:1639, Install78:1498 and Connect78:1593. Current design values are warm-white#F7F6F1, accent#95D4F1, recessed#EAE6DF and read-only tool detail#EAF5FB. This last value corrects earlier warm-white prose.

Native code now follows the three-tab navigation, inline Settings, explicit panels, server actions and nested tools layout. Complete raw tool definitions remain available through a small Raw definition control. Narrow windows retain three columns with readable325px-tall cards and wrapping values. No UI field executes a tool. Discovery uses existing APIs in a popup with retained progress reopening. Manual command installation remains outside implementation pending its execution contract. Browser design specimens and OS titlebar are outside this native-only port.

Exact SVGs are stored under src/ui/assets; fonts/licenses are bundled under assets/fonts. Panel depth uses paired shadows and narrow light/shade rims; stock Iced button depth approximates Figma's multiple inner shadows. Earlier literal native Figma boards now describe the former app baseline, not this implementation. Sample-data native captures and verification are under [neumorphic-dev-sync](../milestones/neumorphic-dev-sync/info.md); they are window-free renders, not a real running-account screenshot.

## Current — Settings is inside each server card

The user approved the recessed Servers/tools layout for all new-design Servers screens. Settings now opens as a tab inside its own server card, alongside Inspect tools, Console and Terminal. The separate new-design Settings page and navbar gear are removed. The temporary fixed-Settings-navigation request was cancelled before implementation. Original source/reference pages and Rust remain unchanged.

The new board has30 native-window specimens,22 under Servers. All22 use the approved1248×1160 windows, shared recessed Installed servers surface197:4166 and raised server cards. The three-column scrolling tools area and raised read-only tool details remain shared. The22 specimens are arranged in two canvas rows without overlap; Install/Connect boards remain separate. Navigation stays in the page; Settings has its own1072×440 scrolling panel.

Shared Settings family202:2558 preserves Filesystem folder/call-limit controls and Search assigned-key/ready/folder-opening/filled/busy controls. [Filesystem Settings](https://www.figma.com/proto/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=78-1639&starting-point-node-id=78%3A1639&scaling=scale-down&content-scaling=fixed) and [Search Settings](https://www.figma.com/proto/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=78-1653&starting-point-node-id=78%3A1653&scaling=scale-down&content-scaling=fixed) have the selected Settings tab inset without glow. Each Settings button targets its own server; it no longer routes Filesystem to Search’s key. Obsolete standalone introduction/selected-server line and the exact short setup sentence requested for deletion are gone.

The two limits are side by side, titled **Time until timeout** and **Max concurrent calls**, both17pxBold with the former heading’s spacing. Fetch call limits heading and verbose field descriptions are removed. Values remain120 and1. Shared number input212:2318 has small right-side up/down arrows. Figma arrows are appearances only, not numeric editing/saving behavior. Light Copy115:1371 is a blue glyph on a transparent40px hit area, with no white surround; prototype clipboard execution is not established.

Independent structure/rendered-appearance checks PASS and final Evaluator ACCEPT. Actual new clicks/scrolling were not repeated because the user was actively using Figma; earlier successful tools scrolling remains historical evidence. [Rollout evidence](../milestones/server-layout-rollout/info.md); [inline Settings record](../milestones/inline-server-settings/info.md). These changes do not implement any Rust/API behavior.

## November 8 — recessed sections, three-column scrolling tools and read-only details

The new tools demo now has a recessed warm-gray #EAE6DF Available tools area177:1914, containing six raised tool cards in three columns/two rows. Its1072×440 viewport scrolls vertically over722.667px of content. The original read_file JSON remains intact; five extra definitions are illustrative fixtures, not live server discovery. Name/Required fields use shared read-only value182:3991: raised warm-white plain text rather than gray inset Text inputs.

Installed servers is recessed as well in Tools expanded78:1438 and demo177:3654. These two trial windows are1248×1160; their page content fits the enlarged viewport without outer cutoff. Other window sizes remain unchanged. Original page scrolling, including navigation, remains. Active Servers/Install/Connect tabs across the29 new native design specimens use the shared Selected state, blue fill/bold white text and inner shadows only; inactive tabs remain raised. The earlier cancelled fixed-navigation/whole-page recess trial stays reverted.

Independent final Evaluator ACCEPT; actual desktop prototype verification PASS: wheel scrolling inside the tool well reaches all second-row cards while navigation, heading, footer and Search remain still; actual Install/Connect tab clicks select correctly. [Open the scrolling demo](https://www.figma.com/proto/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=177-3654&starting-point-node-id=177%3A3654&scaling=scale-down&content-scaling=fixed). Use Fit width and height if preview is at100%. These are Figma-only changes; no Rust or account/server execution. [Scope](../milestones/recessed-tools-demo/info.md).

## November 8 — explicitly opened server panels

The new Figma design now hides Console until Console is clicked. Inspect tools opens Available tools without opening Console; Terminal opens the terminal specimen. The selected control can close the demonstrated panel. Server panels prototype starts at [170:2698](https://www.figma.com/proto/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=170-2698&starting-point-node-id=170%3A2698&scaling=scale-down&content-scaling=fixed). Original page scrolling is retained.

The two extra recovery buttons are removed: normal click on the shared round restart means restart/connect; Shift-click means force restart in the later app implementation. Figma's click trigger cannot distinguish Shift-click, so that modifier behavior is documented rather than simulated.

Shared Button Selected172:1910 uses the existing Inset style with two inner shadows and no glow/drop shadows. Inspect tools, Console and Terminal show it in their open states; review172:3525 includes Settings as well. Both Terminal masters now fill their1072px parent width. Four additional window specimens demonstrate panels closed and Console open while preserving recovery/sign-in/restarting state; new design has28 native-window specimens, while original current-source references remain24. These changes are Figma-only; Rust still retains its existing always-present Console and restart controls until a port is requested. [Scope and checks](../milestones/explicit-server-panels/info.md).

## November 8 — scrolling trial reverted

A one-screen fixed-navigation/recessed-scroll trial on Tools expanded 78:1438 was tested in the actual Figma desktop prototype, then reverted at the user's explicit request. Original scrolling includes navigation and heading again; viewport 1152×820, content 1152×1132, 16px padding, transparent/unshadowed boundary and original prototype starts are restored. No new raised/recessed tool treatment or broader rollout was made. Rust is unchanged. [Record](../milestones/recessed-page-scroll/info.md).

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

# Fetch Figma workspace

## November 7 — navigation and action icon follow-up

The new design removes Quit Fetch from navigation. Settings is a shared44px circular gear button, retaining the blue selected state and Settings prototype destination. Refresh/Refresh servers use the same round control with a cycle glyph, including disabled states; the busy Working…label remains. Server×marks are red **#C64545**, keeping their existing disabled opacity. All24new native screens were updated; per-server Settings text is a separate action and remains.

The new local library now has **88main definitions in11sets**:83copied definitions, Window surface, Settings glyph and3Icon button states. The cycle glyph is imported from Simple Design System. The Settings SVG comes from [Lucide](https://github.com/lucide-icons/lucide/blob/main/icons/settings.svg). New icon buttons use an INSTANCE_SWAP property for glyph selection. Original pages and Rust source are unchanged by this follow-up.

Independent final review ACCEPTs this icon follow-up. Source-text equality checks in the baseline below predate these explicit user-approved label changes. [Evidence/review](../milestones/neumorphic-figma/info.md).


## November 7 — separate neumorphic design pages

The user requested a full visual overhaul on **three new pages**, preserving the current app references until the design is later implemented in Rust:

- [04 · Neumorphic app](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=75-2718):24 editable current native states, grouped under giant Servers, Install MCPs, Connect apps and Settings headings.
- [05 · Neumorphic components](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=75-2251):83 copied main components in10 sets plus standalone components, including clearly labeled legacy copies, plus one new shared Window surface; [compact review sheet](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=94-1300).
- [06 · Neumorphic browser pages](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=75-2719):29 editable approval, callback, permission, retry, HTML-error and raw-response specimens.

Final user-retained palette: **warm white#F7F6F1**, accent **#95D4F1**. The initial#E7E5DB was rejected as too dark; a cooler gray was considered but the user chose to keep the lighter white. Inter and Roboto Mono are the new design fonts. Raised controls use curved inner rims/contact shadows with reduced white glare; inputs are recessed. Internal containers allow shadow overflow. Each native design uses the shared raised 1200×900 Window surface with 40px shadow space inside a transparent 1280×980 presentation stage. The flat base rectangle was removed at the user’s request. Scrolled Settings specimens suppress offscreen navigation shadows.

All53 screen specimens use the new copied components. New variables, text/effect styles and121 illustrative prototype links are separate from current source references. Structure checks found no full-UI image fills, zero-width visible text or instances using old component definitions.24 native states retain source text/visibility exactly; browser source wording and permission selection were independently compared. Prototype links demonstrate navigation only; they do not run backend operations or prove native behavior.

Original native/browser reference-frame fingerprints match their pre-copy snapshots. The original component-page raw checksum differs; no deliberate write targeted it, and exact per-node baseline values were not stored. Preservation evidence therefore does not prove an identical raw component-page snapshot. The existing pages remain present and the literal native/browser reference views are unchanged. [Scope, evidence and review](../milestones/neumorphic-figma/info.md).

Independent Tester PASS and final Evaluator ACCEPT for the current design baseline. The Rust app is unchanged. Larger UX changes are the next discussion; this design pass does not select new operations or API behavior.


## November 7, 2026 — current native app sync

The [current native board](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-487) now has **24 editable native specimens**: 13 existing frames updated in place and 11 additional current states. All four app pages follow current Rust/Iced source and disposable offscreen app renders. Current native details below supersede older pending-reconciliation notes and October28 specimen claims.

- Servers: empty/populated/tool definitions/recovery, always-present Console, sign-in action, remote Terminal restriction, running/exited Terminal, restart, delete controls, expiry/check times and call counts.
- Install: blank initial form, compact hint, discovery results, progress/failure, looking/no-results, installed/unsupported rows, selection controls and fixed action footer.
- Connect apps: current Claude/Codex command blocks with Copy; removed cleanup checkbox/prompt.
- Settings: server folder, Fetch call limits, assigned manual-key editor, opening/busy/filled and scrolled forms. Removed JSON configuration editor.

The 1080×760 app area retains28px padding and actual source gaps; the existing32px OS-titlebar representation is retained separately. Use ordinary purple Iced buttons, shared component instances, plain statuses and gray panels. Current frames have editable text/layout/components and **no full-UI image fills**. Navigation prototype links are illustrative and never run backend actions.

**Typography limit:** the actual renderer resolves normal text to macOS System Font/.SF NS and console text to Menlo. Figma lists SF Pro but renders newly created text blank/zero-width; Menlo is absent. The editable copy therefore uses calibrated Arimo and Roboto Mono. It is not pixel-identical typography. Sample paths/server data and the preserved OS-titlebar representation are not screenshots of live account state.

Independent Tester and Evaluator reviewed the native scope; final Evaluator ACCEPTs the layout/control references with the explicit font limitation. [Evidence and scope](../milestones/current-figma-sync/info.md). Window-free sample rendering used temporary state/in-memory keys; no native app, listener, provider, Keychain prompt or managed MCP was started. Product source was restored exactly after reference export. Older native authentication/settings/setup/control boards are visibly archived; browser pages remain a separate earlier reference and were not refreshed in this native sync.

## Historical October 28 reference

Official file: [Fetch App Mockup](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-487).
File key: `OcgOlmfuoSTJMHTeN753TF`. Selected by the user October 28, 2026.

## Historical 153-state source-snapshot library

October29 coverage follow-up: the separate D80 Railway callback renderer has
seven distinct response texts not yet traced to specimens in this index. Their
mapping needs reconciliation; the recorded153 count is unchanged. This library
does not establish complete coverage of that newer renderer or live behavior.


The user's follow-up requested every current UI element, including authentication
and individual settings. At that checkpoint the file contained **153 states/specimens** across
nine current boards, preserving the rejected-design archives separately.
See [the recorded coverage index](UI_STATE_COVERAGE.md) for its nodes and the
finite source-branch checklist.

- [Authentication states](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-348): 21 full-app specimens, including all nine current module phases.
- [Individual settings](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-349): 28 save/remove/validation/configuration specimens.
- [Setup and server states](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-350): 26 additional restart/catalog/discovery/install/picker states.
- [Browser errors and requests](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-351): requested/custom/retry forms, all 21 hard-coded HTML errors and a raw-response specimen.
- [Native contracts](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=33-352) and [control/message specimens](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=38-381): OS-owned handoffs, widget interactions, native-client errors and minimum window.

The latest inline authentication and shared credential editor source are included.
Credential rows use stored/missing; the exact Save and verify credential label,
secret clearing and control availability are preserved. Native prompts are
source contracts, not fabricated OS screenshots. Arbitrary data/error values and
all combinations are represented by reusable slots, not an exhaustive permutation
claim. Independent source and representative screenshot review accepted the
reviewed scope after corrections; no new app/runtime action occurred.

## Current UI copies — October 28 correction

The user rejected the first styled designs because they did not look like the
application. The current boards reproduce the existing UI from source, using an
unmodified screenshot of the already-open Fetch window as the visual reference.
Future work must start from these copies, not the archived redesign.

- [Current native UI](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-487) — 13 editable frames.
- [Current shared components](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=20-485) — seven native families plus browser form controls.
- [Current browser HTML](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=28-386) — four editable frames.
- [Exact native screenshot reference](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=23-796) — the unchanged captured empty Servers window.

| Current page | Copies and sample states |
| --- | --- |
| Servers | Empty, populated, tools expanded, authentication/uncertain-call recovery |
| Install MCPs | Before discovery, selected definitions, installation in progress, failed/retry |
| Connect apps | Setup instructions, verified cleanup handoff |
| Settings | No selected server, selected server and private fields, advanced settings shown at a scrolled position |
| Browser | App approval, expired request, provider callback success and failure |

Native windows have a 1080×760 client area plus a 32-pixel macOS title-bar
representation. Content keeps the source's 28-pixel outer padding, 20-pixel
navigation spacing, white background, purple Iced buttons, plain text statuses,
small corner radii and single-column forms. Long pages keep their actual
scrolling. Install keeps its header and action footer outside the scrolling
definitions/progress area. The advanced Settings frame shows the lower part of
the same scrolling page rather than a newly invented layout.

Data examples occupy the actual widgets. No dashboard counters, status pills,
separate client cards, colored alert panels or two-column Settings have been
added. Raw tool/configuration JSON remains plain text as rendered by the source.
Sample paths use /Users/example and contain no credential values.

There are 59 native prototype links for navigation and selected workflow
transitions. Copy/save/install/confirmation are illustrative; prototype links do
not execute backend operations. Shared definitions and code-reuse requirements
are in [SHARED_UI_COMPONENTS.md](SHARED_UI_COMPONENTS.md).

## Evidence and limits

October29 follow-up: the separate D80 Railway callback renderer still needs
explicit mapping into the specimen index. The existing153 count is preserved;
coverage of that newer renderer is not assumed. See the reconciliation note in
[UI_STATE_COVERAGE.md](UI_STATE_COVERAGE.md). No Figma work was done for this note.


The existing Fetch process/window was identified read-only; only that window was
captured. No new app instance was started, no page/button was operated, and no
API, credential, certificate trust or managed MCP action was performed. Empty
Servers is directly screenshot-backed. Other states are reconstructed from
src/ui/mod.rs; browser pages follow src/oauth/mod.rs and src/api/runtime.rs.
Iced 0.14 dependency source supplied default colors, padding, radii, font sizes,
line heights and disabled appearance. Source review and Figma screenshot
inspection are not application tests.

Figma cannot render the native SF Pro face in this environment. Editable Arimo
text is calibrated against the captured native window, with original source font
sizes retained as node metadata. This is a known typography difference; do not
claim pixel-identical native fonts. The unmodified screenshot is the exact
native rendering reference. Browser serif text uses a compatible Tinos face;
browser/OS-owned controls can also vary by platform.

## Rejected earlier proposal

The original 17 styled frames and their components are preserved on pages named
Archive. They were rejected as copies of the current app. Their earlier
62-link count, Inter typography, badges, two-column Settings and dashboard
layout are historical, not the current UI specification. Original linked page
0:1 is now an archive. User review of the corrected copies remains open.
