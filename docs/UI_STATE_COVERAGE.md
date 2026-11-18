## November 17 — app approval and unknown server sign-in

Connect apps widget fixtures cover a Codex project approval and a shared Claude Code approval at minimum window size. The notice sits beside the matching app key only in the approved scope. The Servers fixture checks that an unknown sign-in status has no warning and that clicking its disabled key starts no action. A server that requires sign-in still shows its warning.

## November 17 — install checkboxes, Instructions Paste and auth glow

Offscreen fixtures verify the selectable and unsupported checkbox palettes, original failure detail on hover, Paste and Copy in both Instructions editors, and neutral key highlights at the two sizes. The install and clipboard replies are simulated. [Evidence](../milestones/history/2026-11-17_18-40-33-760776-tester-verify-install-paste-and-key-glow.md)

## November 17 — install button colors and progress by server

The focused render checks verify the white shortcut, disabled gray and enabled blue footer, and the position and motion of the inline bars. The fixture distinguishes two same-name servers in different configuration files and shows queued, installing, installed and failed results. [Evidence](../milestones/history/2026-11-17_18-28-41-722172-tester-verify-inline-install-progress.md)

## November 17 — install from the Servers heading

Focused widget tests cover the blue header button, compact chooser, results and install footer at minimum window size, picker failure, selection and active-job reopening after dismissal. Replies use fixtures. [Evidence](../milestones/servers-install-modal/info.md)

## November 17 — editable Claude subagent fields

A focused popup fixture edits Name and Description through the native controls. It checks the retained path, Markdown instructions and tools, the Save task and the updated Preview. The file responses are simulated. [Details](../milestones/history/2026-11-17_18-14-15-483250-tester-verify-claude-popup-metadata.md)

## November 17 — Workspace loading label

The Sessions widget fixture checks a centered **Loading...** on the empty initial panel and during a refresh with retained rows. It verifies the label disappears when results arrive. [Check details](../milestones/history/2026-11-17_17-59-58-671554-orchestrator-deliver-workspace-loading-label.md)

## November 17 — toolbar Paste from a single copied card

The toolbar Paste fixture clicks the same button used for Copy all bundles, supplies a single card copy, and checks for immediate creation with no editor draft. It uses a registered card whose path has a different filename from the resolved definition. [Evidence](../milestones/workspace-agent-actions/info.md#toolbar-paste-follow-up)

## November 17 — model menu and card actions

The [focused Workspace captures](../milestones/workspace-agent-actions/info.md) show four initial model rows, later rows after scrolling, immediate Paste results and a named delete confirmation. Disposable routes verified the registered description, Plan conversion, replacement, create-only additions, and shared config protection.

## November 17 — unnamed subagent conversion

[Focused checks](../milestones/subagent-card-name/info.md) verify that the displayed Codex label is carried into the Claude file when the source has no name, even if the displayed name and source filename differ. Existing explicit names and missing-description validation are preserved.

## November 17 — compact subagent errors and clipboard status

[Final widget evidence](../milestones/subagent-feedback/info.md) covers Copied all!, Pasted all!, five simultaneous errors at minimum window size, Save errors and conversion details on hover, no review checkbox, and busy clipboard button presses captured without starting a window drag. The offscreen fixture does not reproduce the live macOS flash.

## November 17 — bulk subagent clipboard

[Rendered and fixture evidence](../milestones/bulk-subagents/info.md) covers the two icons at minimum window size, selected scope copying, immediate paste, the partial report and refreshed list. Disposable HTTP calls checked exact same-app bytes, an occupied name, cross-app conversion and a review-required definition.

## November 17 — Add subagent states

The [focused Add evidence](../milestones/add-subagent/info.md) includes the blue Add button with white text before the app icons, the blank popup with adjacent Paste and Copy, Save in the scrolled minimum-size popup, and the visible conflict with the draft retained. Tests use a fixture clipboard and temporary files. The actual macOS clipboard remains for live observation.

## November 17 — startup and Workspace polish

[Rendered evidence](../milestones/startup-and-workspace-polish/info.md) covers blank pending startup, three staggered cards and their settled slots, centered confirmed-empty Servers and Subagents states, the blue Finder face and white icon, and both ends of a long subagent preview with its path fixed. The focused tests also exercise manual reload, quiet Copy, the Finder action and existing editor and Escape behavior. Evidence uses offscreen native widgets and fixture responses. No physical trackpad event or live Finder action was run.

## November 16 — Escape keyboard flows

[Focused keyboard evidence](../milestones/workspace-session-error/escape-evidence/info.md) covers a genuinely focused dirty agent editor, session/cache, no-popup state and layered pending approval/install. Tests deliver real widget events then the exact production subscription mapper with actual Shell status; Captured routing is checked explicitly. Two cases pass1.79s; Close state retention and zero-backend dismissal tasks pass. No new visual layout; native physical keyboard observation remains after Quit/reopen.

## November 16 — measured default/light code/combined Copy

[Final evidence](../milestones/workspace-session-error/window-markdown-evidence/info.md) includes1058x789default Servers and session views,760x520inline code and scrolled fence tail. Inline14pxbody/light highlight, literal shared13pxfenced/tool renderer, intact bold/list/quotes, long-line wrap and exact name+blankline+raw result Copy pass.21unique focused cases pass; pixel fixtures use actual dimensions and3observed hover comparisons settle entrance animation before comparison. No native app relaunch/personal history acceptance claimed.

## November 16 — compact paths and gray tool follow-up

[Final follow-up evidence](../milestones/workspace-session-error/compact-followup-evidence/info.md) covers gray tool cards, code-box names with visible separate arrows, readable JSON with a long unbroken value wrapping at760px, exact raw Copy, and shared compact instruction/subagent code paths with14px bare blue Copy/44px single-line height and adjacent Finder.12Workspace cases plus final affected tool rerun pass; normal/minimum captures inspected. Native personal/Finder use remains unobserved.

## November 16 — Workspace file/session corrections

[Focused evidence](../milestones/workspace-session-error/evidence/info.md) includes normal/minimum code paths with Copy/adjacent Finder, blue subagent names, shared monospace file-content previews, rendered session Markdown, white User/gray Assistant cards and single frameless blue tool Copy.12actual-widget checks pass;8native helper checks cover a missing indexed rollout among valid conversations and unchanged sources. Fresh headless views now run layout before operations/events/draw, matching Iced's lifecycle for size-dependent widgets. Actual personal/native observation remains separate.

## November 13 — native conversation and subagent presentation

Workspace consumes complete official messages; tools begin collapsed with name/blue Copy and expand independently. Subagents open a shared modal with parsed settings/Markdown instructions and Edit/Preview, Close and Copy. file_path/finder_button share a compact raised blue Finder control next to supported instruction/subagent paths. Saved success text is removed. Focused popup/API checks pass; actual-widget captures and independent ACCEPT are recorded under milestones/official-session-readers/evidence.

## November 13 — Workspace editor/gesture and clean session fixtures

milestones/workspace-editor-gestures/evidence includes editor blue Copy/white Save, subagents without Paste, no manual registration row, long reply wrapped inside one card with its ending visible after scrolling, popup clearance and first-load/reload stagger frames. Widget checks press actual Cmd/Ctrl+Z after deletion, click Copy without executing clipboard Tasks, and move a continuing outer gesture across the editor before starting a fresh gesture. Parser cases exclude injected context for Claude/Codex while preserving manual input, full replies and tool outputs. Native fixture covers phase ordering only; physical trackpad/traffic-light acceptance remains live user observation. No personal session text used.

## November 12 — raised selectors, blue sessions and split replies

milestones/raised-project-picker/evidence contains the raised closed project filter/open menu, blue session-name row, and popup with a command code block, user turn and assistant reply. Existing Workspace fixtures cover minimum layout, dimmed loading and both app choices. The realistic session case splits a40KB assistant JSON record at the32KB API boundary, verifies automatic continuation, then checks Read more, outside dismissal and cached reopen. Parser fixtures cover native command/output/note wrappers plus existing Claude/Codex formats. No personal histories were read.

## November 12 — Workspace presentation fixtures

milestones/workspace-session-popup/evidence covers Claude/Codex direct instruction editors, unified app choices, subagents, session cards, normal/minimum window sizes, a user/assistant history popup, idle versus gray loading list, Installed servers alignment and an open project menu. Tests cover chunk append/cache/reopen, popup outside dismissal, metadata/tool projection and Codex duplicate event suppression. Native hidden panel reuse is a separate fixture, not a live dialog screenshot or observed click response. Real files/providers were not used.

## November 12 — Workspace unified circular app choices

Workspace removes the Global/Open folder button row and the separate Claude Code/Codex text-button row. The scope caption moves into the heading, allowing Instructions/Subagents/Sessions and their content to move up. The tabs row now has two circular app-logo choices on its right, each40px to match tab height. The shared app_logo_button uses existing logos and shared button/surface rendering: unselected raised, selected/pressed inset with the existing recessed fill. One workspace.app selection controls Instructions (CLAUDE.md versus AGENTS.md), Subagents and Sessions; existing session API app filtering is reused. Folder picker and typed-path Enter/global-empty behavior remain through their existing handlers.

Four Workspace widget checks pass (3.67s), including real logo clicks and correct app-specific instructions/subagents/sessions. After strengthening selected-face contrast, the same complete selection case passes again (2.41s). Both selected states and normal/minimum screenshots were inspected. Final signed development build-only passes (16.05s), including strict signature verification; git diff --check passes. An initial new fixture case needed relayout after injected API replies; its original failing log is preserved and only the fixture was corrected.

Direct UI work without subagents or independent review for this pass. Fixture API tasks are dropped: no personal files, native picker, clipboard, model process or live account/provider/MCP action is executed. No API/auth/permission/storage behavior changed, no Figma/commit/push/release, and Fetch was not relaunched. User must explicitly Quit/reopen to observe the rebuilt UI. Concurrent project-scoped/Workspace backend work is preserved.

## November 12 — project selections and explicit history approval

Actual-widget checks cover explicit Workspace registration and independent
server-filter/install/connection choices, project snippets and the project
approval's actual folder with initially unchecked history access. The minimum
760x520 approval scrolls permissions while keeping Close/Deny/Allow visible.
Eight source-driven captures were exported; project Workspace, Shared filter,
installation, connection and both popup sizes inspected. API/clipboard/Keychain
Tasks were dropped in widget fixtures; actual clients and OS approvals remain
manual acceptance. [Evidence](../milestones/project-connections/evidence.md).

## November 12 — shared approval code, blue Allow and refresh hover

The single approval popup used by both Claude Code and Codex now uses the shared blue primary Allow button. Request code is labeled above the existing shared code block with its Copy control, which copies only the code. Removed the requested Match this code… sentence. Install refresh keeps the shared hover glow/rotation enabled when idle even with an empty path; its existing blank-path handler still performs no discovery and starts no request.

25 existing actual-widget checks pass (9.38s), including request-code Copy, absence of the removed sentence, both app popup flows and empty-path hover animation/no-discovery. Inspected Claude and minimum-size Codex popup PNGs. Signed development build-only/strict signature verification passes (17.68s); git diff --check passes. No new test infrastructure or tests added; existing cases were extended for the requested behavior.

Direct UI work; no subagents for this pass. Fetch was not relaunched. No live provider/MCP, credential/trust/config, API/authentication permission, Figma, commit/push or release changes. Explicit Quit/reopen is needed to see the new executable.

## November 12 — server toggle verified

An actual-widget case covers pending request/disabled duplicate click, rejected Off staying On with real error text in the matching Console, successful Off, and explicit On waiting for named restart completion. Existing normal/minimum screens and action/copy/popup cases remain green (25 widget checks). Three confined HTTP/MCP and two private no-network checks establish backend persistence/busy/guard behavior. Signed build and independent review pass; no live Fetch/provider/macOS result is claimed.

## November 12 — Workspace source fixtures

workspace_tests.rs covers global/project selection, late-scope reply rejection,
Claude/Codex tabs, session disclosure/Read more/collapse/reopen, file editing/save,
converted drafts with unresolved issues, app-reported Codex model selection and
pending-operation save-target stability. Offscreen real-widget captures cover
normal global/project, minimum global controls, expanded session and pasted editor.
Tasks do not execute against personal files or real apps. Native clipboard,
Finder-launched account catalogs and actual user-file acceptance remain untested.
Session MCP is intentionally unavailable in this slice.

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

## November12 — connector popup checked

Five focused UI checks pass for actual popup widgets, close/reopen ownership, delayed approved reply, denial and expiry retry. [Inspected actual-widget specimen](../milestones/local-mcp-connector/evidence/connector-approval-popup.png). No live Mac confirmation/Keychain screenshot or Figma state is claimed; user defers those to manual testing.

## November 12 — inline refresh and shorter Servers feedback

Installed servers Refresh now uses a 1.5-second minimum indicator and continues spinning if the read takes longer. Install MCPs Refresh is now in the path-input row, immediately left of Choose file or folder…. Its three-second minimum is unchanged. Both use the same shared feedback implementation; the caller supplies the duration. Returned data still applies immediately.

Four focused existing UI checks pass: manual refresh pending spin, instant reload/minimum feedback, discovery click/spin/retry, and normal/minimum-window screens. Inspected the 874×630 filled-path and 760×520 Install screenshots. Signed development build-only passes (15.03s), including strict signature verification; git diff --check passes. No new tests were added for this small adjustment. Direct work/review without subagents. No Fetch relaunch or live MCP/provider calls; running Fetch needs explicit Quit/reopen to show the changes. Unrelated edits, credentials, personal settings and services were preserved.

## November 12 — shared refresh and text actions

New actual-widget discovery test clicks Refresh, closes/reopens the existing flow, checks rotation away from the pointer, blocks repeat clicks, applies fast data before feedback ends, and allows retry after fast success or slow failure. Existing Servers refresh/restart/hover and other page-control cases remain. 78 UI checks and signed build pass; three page screenshots inspected. One old Connect command expectation was aligned with the already-current stdio connector. Live Fetch was not relaunched; direct review only. [Evidence](../milestones/shared-button-behavior/evidence/info.md).

## November 12 — connector popup source states

New ui/connector_setup.rs covers pending permission selection, Mac confirmation in progress, approval/denial, saved profile and setup/save errors using shared controls. A focused actual-widget popup specimen is being added; no production screenshot or Figma state is claimed yet.

## November 12 — final reload, copy, startup status and resize UI

Final21 real-widget checks cover instant reload/card re-entry, red Remove halo, Connect order/removed content, all shared copy confirmation/timeout/no background and white shine, starting orange/spin/no unknown, collapsed automatic checks and immediate API-record application. Four visual checks pass; selected final PNGs inspected. Native fixture separately checks resize-ring hits/cursors/controls/teardown. User live resize feel and recurring Keychain access are not claimed passed.

## November 12 — inline restart failure

New actual-widget fixture covers collapsed failed restart header alignment, only the affected server, rejected POST, preserved Console detail, retry progress and immediate succeeded/failed terminal status without a global notice.15 UI/4 visual checks pass; inline failure screenshot inspected and signed build ready.

## November 12 — contain internal scrolling and remove page banners

New actual-widget fixtures verify Console/Tools/Settings wheel containment at top/bottom across new gestures, parent scrolling outside the pane, and absence of global error/notice text across Servers/Install/Connect. Fixed-list fixture now targets its gutter to scroll the outer list.14 widget/4 visual checks pass; shared containment applies to discovery and terminal frames as well. Live manual state remains distinct.

## November 12 — gray key button

Existing timed key-hover fixture passes with gray face; inspected screenshot and signed build pass. Actions/state coverage unchanged.

## November 11 — spin refresh through completion and glow key hover

New actual-click fixtures cover global refresh waiting without hover and terminal success/error, plus per-server restart submission, accepted operation and terminal success without notice. Existing hover/disabled/key halo tests remain.12 widget/4 visual/2 automatic-refresh cases and signed build pass; live animation remains for user relaunch.

## November 11 — remove Retina outline, fix all native hover and align resize

Four visual/ten actual-widget tests now cover the reproduced Retina rim, right shadow fade and visible face at each actual window edge. Hidden native fixture checks each of the three buttons differs on hover and exactly restores on leave, plus frame/content edge equality. Live native cursor/appearance still requires Quit/reopen. No Figma edits.

## November 11, 2026 — expansion, refresh hover, left window controls and right shadow

Native checks now cover first expansion before server I/O, refresh hover progression/rotation/leave/disabled and original restart click, and right-edge shadow fade. Nine actual-widget and three visual checks pass. AppKit native controls have a separate hidden fixture; live Fetch appearance remains unobserved. No Figma changes.

## November 11 — automatic native Console and combined errors

Native fixture coverage passes for failed remote server expansion, captured server output plus Fetch error code/message/details inside Console, Copy of those same lines, unchanged status polling without duplication, and collapse/re-expansion after Console/Tools. Existing panel coverage checks Settings ownership and terminal coverage checks retained sessions. Expansion must remain a display action without authentication or terminal startup. This changes the native expansion default; existing Figma specimens remain unchanged. Results are tracked in [the Console milestone](../milestones/console-on-expansion/info.md).

## November 9 — hover fade, pane response and smaller default

Added source-real timed hover checks on enabled black authentication key at0/75/150ms, no popup overlay and exact idle restoration after leaving. Console/Terminal test uses actual clicks plus full4096-line retained log and a reused fake terminal; pane state updates before HTTP response and measured draw budget is250ms. Native test handles only widget resize events, drops all HTTP tasks. Existing page render loop now captures874×630 default and760×520 minimum; design-sized Tools/Settings and1x/2x Button fidelity remain. Seven widget/two fidelity checks pass. Full-hover and default Console/Settings PNGs visually inspected; native live animation/shell startup remain unobserved. No Figma edit.

## November 9 — tool edge and frameless state

Existing actual Tools test now reproduces the missing right shadow before the fix and verifies the padding shadow after, alongside three-column geometry/nested scrolling. Navigation fixture verifies transparent corner alpha, native drag message from title space and no drag from tab clicks. Five widget/two fidelity checks pass; normal/scrolled Tools and minimum-window RGBA screenshots inspected. Both boot/reopen share transparent/undecorated settings. Actual macOS compositor/drag/resize remain unobserved until user relaunch. Figma unchanged.

## November 9 — native server-list scrolling

Servers has a bounded card scroller (fetch-installed-servers) beneath fixed title/Refresh; navigation remains fixed. An eight-server actual-widget fixture verifies normal/minimum sizes, stable bounds through expansion/scroll, later-card visibility and clicking a scrolled header. Existing nested Tools scroll and page/Settings first-frame checks pass. Normal/minimum scrolled-list and expanded Tools screenshots were inspected. Synthetic/offscreen evidence only; actual running-app behavior after relaunch remains unobserved. Earlier whole-page scrolled Servers specimens below describe the prior layout. Figma unchanged.

## November 9 — ready server hover removed in code

Native ready server identities have no Runtime: ready tooltip. Other runtime statuses retain the existing tooltip, and the shared status dot/disclosure/actions remain. The existing server-panel interaction check and signed build pass; this is source/fixture evidence, not a live hover observation or Figma change.

## November 9 — inline square key actions and outlined information

Save and Remove are now44×44 square icon buttons alongside the credential field: blue with a white Save vector, red with the existing white ×. Shared action set246:2464 covers enabled/disabled states, retaining prior0.58disabled opacity. All four active key editors use one44px row, with916pxfield plus two actions. Figma Remote rows are gray in all three actual discovery screens; screen overrides were corrected using resolved gray RGB values rather than black fallback colors. The information control is now an unfilled gray circular outline with a gray i, including hover state; its exact tooltip message is retained. Current key-row and discovery-dialog screenshots were inspected. Figma only; no Rust changes or subagents.

## November 9 — gray unsupported row with information tooltip

Figma Remote · remote is now gray in the Results, Installing and Failed discovery dialogs, including its unchecked checkbox and label. Its exact approval explanation moved from the separate line into a shared information i hover tooltip (set234:2431; Default234:2421, Hovered234:2426). The row remains unsupported/disabled. Results dialog and tooltip screenshots were inspected. Figma-only, with no Rust changes or subagents.

## November 9 — key visibility eye control

The shared API key entry now includes a small right-aligned eye button. Click switches between Hidden and Visible variants; the visible state uses a crossed-out eye to hide again. Existing name/colon emphasis and placeholder colors remain. Component set231:2987 retains original hidden master226:2385; visible master231:2978. Figma uses the disposable example-token-1234 value for the filled-key reveal demonstration. Empty/named/busy forms retain their placeholder when switched. This specifies the requested UI behavior; native reveal implementation is unchanged.

Shared-state and filled-key specimen screenshots were inspected; reciprocal ON_CLICK → CHANGE_TO links were read back. No actual user credentials were read. No subagents or application source changes.

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
