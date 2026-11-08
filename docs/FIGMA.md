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
