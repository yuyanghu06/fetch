## November 8 — pending hover-state correction

Selected controls must not glow. Glow appears only while hovering, including on the selected Settings gear shown in the user’s reference. Hovering over the red × must make the × bold. Requirements recorded only. Apply to shared Figma controls and check selected versus hover appearance in a later design pass; carry the same rules into the later Rust port.

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

Final header action/review containers allow shadow overflow; refreshed evidence preserves the selected neumorphic depth.

## November 7 — final server header actions

The disclosure is now a plain14px muted chevron with no circular background/shadow, inside a transparent28px click target. Name stays18px Bold; dot stays8px and yellow during starting/restarting. Three44px actions sit on the right: black#1D2023 with filled white key opens authentication; blue with white circular arrow restarts the server; red with white× removes it. All13native headers and8active card masters use shared Server actions132:1678. Busy authentication/restart controls use disabled variants; existing disabled deletion states are retained. Disabled red remains saturated per the user's earlier color instruction.

The new library has120local main definitions/14sets. Review sheet129:1601 now pairs identities and actions. Independent final visual review ACCEPTs the current app headers. These actions specify the new Figma UX; they are not newly wired native buttons or executed backend operations. Rust/original reference pages were unchanged in this pass.

## November 7 — compact server card headers

The new neumorphic server cards use a bold18px name, a28px recessed expand button with16px right/down chevron, and an8px runtime dot. Ready is green#77AD8A; stopped gray#9CA3AA; starting/restarting yellow#E2B84F; failed red#C76B6B. Warnings sit beside the dot in smaller amber text, such as Sign-in required. Successful/unneeded authentication labels stay hidden. User rejected the initial22px name/36px button as too large; final sizes are18px/28px.

Shared header set128:1597 has eight variants (four runtime colors × collapsed/expanded), text/visibility properties and bound color variables. Eight active card masters and13native header instances use it; a compact review sheet129:1601 shows states. Library now116local mains/14sets. This pass changes only the new Figma designs; the original app references and Rust source are unchanged. Runtime hover text and clicking the full identity area are implementation requirements, not demonstrated native behavior.

Independent Tester PASS and final Evaluator ACCEPT for all six Install layouts. Latest installing evidence includes the gray installed row.

## November 7 — current visual and Install UX refinements

The new Figma designs use convex blue/red controls, bold white text on every blue action, centered white vector × on retained red#C64545, white filled Copy glyphs on dark Console and blue copies on light surfaces. Settings gear is at the right: gray#A6ABB2, white outlined glyph when unselected; darker gray#939BA5 with filled white gear when selected. Blue Refresh retains a white cycle glyph. Checkbox outlines are removed and checks restored; already-installed rows use gray faces/checks/labels. Code blocks and inputs use very light gray#EEEFEF; Settings has32px bottom padding.

Install discovery choices, searching, no results, installation progress and failure now appear in five shared popup variants. The main Install page has its file selector plus a separate manual-install command field, Install MCP action and empty shared Console. These are Figma designs only; manual command execution/API behavior is not selected or implemented. Close returns to the main Install specimen. The library has105local main definitions/13sets. Existing app/browser reference pages are retained.

One explicitly requested Rust change is complete: server headers omit authentication status for authenticated/not_required and retain every other status. Six-state native-view render, locked native build and independent review pass. The temporary exporter is removed; Fetch was not launched. Active original and new Figma headers match this source change. Earlier “Rust/original pages unchanged” statements below describe prior passes.

Latest shared components: Discovery selector121:1450; Manual installation120:1384; Dialog set120:1519 with variants120:1403/1434/1467/1501/1510. Final independent popup review ACCEPTs this slice.

# Neumorphic Figma redesign

Three separate new pages in the official file:

- [App](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=75-2718):24 native states under giant page-name headings.
- [Components](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=75-2251):83 copied main definitions plus one shared Window surface (84 total), 10 component sets, standalone families, labeled legacy copies and a compact review sheet.
- [Browser](https://www.figma.com/design/OcgOlmfuoSTJMHTeN753TF/Fetch-App-Mockup?node-id=75-2719):29approval/callback/permission/retry/error/raw-response specimens.

Native presentation stages are transparent: the flat base rectangle is removed.

Final palette is warm white#F7F6F1 and blue#95D4F1. User steered stronger outward curvature, softer white highlights and uncut edge shadows. Full app windows have raised1200×900shells inside1280×980stages with40px outer shadow space. New design uses supported Inter and Roboto Mono. Internal rows do not clip shadows; actual app/content scrolling boundaries remain. Hidden offscreen navigation does not leave a strip in scrolled Settings.

Structure checks: every screen instance points to the new component page; no full-screen image fills or zero-width visible text.24 native states preserve source text/visibility exactly; browser wording/selection checked independently.121prototype links navigate new specimens only, illustrating rather than executing actions. Existing native/browser page fingerprints match the pre-copy state. Original component-page raw checksum differs; no deliberate original write occurred, but a per-node baseline is unavailable, so identical raw library preservation is not proven.

The evidence/ folder contains representative screenshots and the organized native overview. A local working ledger lives at /tmp/design-system-state-fetch-neumorphic.json; committed docs retain stable page/frame links. No app/backend/API source, accounts, credentials or running services changed.

Status: complete as the design baseline. Independent Tester PASS and Evaluator ACCEPT after final surface/transparent-stage screenshot review. Existing pages are retained; future Rust implementation and major UX changes remain separate work.

## Navigation/icon follow-up

Quit Fetch removed from new navigation; Settings becomes a44pxround gear with active blue state; Refresh becomes a cycle glyph with disabled variant; server×marks use#C64545.24gears/14refreshes/13redmarks applied. Local library now88main definitions/11sets plus imported Refresh glyph. Final screenshots: evidence/nav-icons-servers.png, nav-icons-settings.png, nav-icons-install.png. Independent Evaluator ACCEPT; Rust/original pages unchanged.
