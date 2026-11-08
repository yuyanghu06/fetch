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
