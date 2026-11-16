## November13 — whole raw loading and compact tools

The current chunk interface now reads to EOF while the popup is open, stops follow-up dispatch on close and resumes incomplete cached logs. Read more/early assistant targets are removed. Per-log expanded tool IDs keep named results collapsed by default with exact Copy and shared code blocks on expansion. Existing call IDs associate names; no additional guessed record formats were added.9widget cases, signed build and independent frontend review pass. This is preparatory frontend work; modern native reconstruction still awaits official reader/backend integration and user contract input.

## November 13 — gesture routing, local undo and clean history

scroll_gesture.rs owns the current scroll path; root with_scale begins an event from native phase metadata and ends stock Iced transactions only at a fresh gesture. contain_scroll records the deepest initial owner and prevents nested retargeting while preserving boundary capture. Phase-less wheels use180ms inactivity; button/window-leave resets end a transaction. surface pop_in uses generation+row delay for first-load/reload staggering.

workspace.rs stores local undo snapshots/cursor and the saved baseline; focused Cmd/Ctrl+Z restores edits without writing files. Save uses a white-text style; editor/subagent Copy shares components::copy_button. Paste/manual registration controls are removed. project_scope.rs submits the existing registration API after valid folder discovery/list readiness while preserving independent choices. session_history.rs displays manual Codex user events, assistant text and returned tool results; it filters native injected markers/metadata, omits tool-call inputs, joins message text blocks and preserves raw Log content. Fill-width word wrapping and shared modal clearance prevent cropped cards/window-control overlap. See milestones/workspace-editor-gestures for checks and limits.

## November 12 — raised project selectors and command/reply presentation

project_scope.rs wraps closed dropdowns with the existing raised_with_radius surface rather than input_surface. entry_card takes a primary flag so sessions share the blue primary button while subagents retain ordinary buttons. session_history marks native commands/output and tool data for shared code_block rendering, with readable labels and preserved raw Log content. Log.assistant_target tracks each requested next reply; Chunk continues existing read calls until another assistant turn/EOF, only while the popup remains active. Read more updates that target. No API or session-file writer changed.

## November 12 — direct instructions and session presentation

workspace.rs opens the selected instruction descriptor through the existing file API and renders its editor directly. Session cards share entry_card with subagents. session_history.rs incrementally projects returned JSONL into role-labeled records; raw content remains in Log. Workspace renders the cached records inside the shared modal, with existing chunk pagination and outside/Close dismissal. loading_panel keeps and dims existing rows while requests are pending. Theme picker_menu supplies shared rounded/shadowed menus; project-scope labels clarify universal availability without changing API values. file_picker::prepare queues hidden native preparation when a window opens. Fixture checks and limitations: milestones/workspace-session-popup.

## November 12 — Workspace unified circular app choices

Workspace removes the Global/Open folder button row and the separate Claude Code/Codex text-button row. The scope caption moves into the heading, allowing Instructions/Subagents/Sessions and their content to move up. The tabs row now has two circular app-logo choices on its right, each40px to match tab height. The shared app_logo_button uses existing logos and shared button/surface rendering: unselected raised, selected/pressed inset with the existing recessed fill. One workspace.app selection controls Instructions (CLAUDE.md versus AGENTS.md), Subagents and Sessions; existing session API app filtering is reused. Folder picker and typed-path Enter/global-empty behavior remain through their existing handlers.

Four Workspace widget checks pass (3.67s), including real logo clicks and correct app-specific instructions/subagents/sessions. After strengthening selected-face contrast, the same complete selection case passes again (2.41s). Both selected states and normal/minimum screenshots were inspected. Final signed development build-only passes (16.05s), including strict signature verification; git diff --check passes. An initial new fixture case needed relayout after injected API replies; its original failing log is preserved and only the fixture was corrected.

Direct UI work without subagents or independent review for this pass. Fixture API tasks are dropped: no personal files, native picker, clipboard, model process or live account/provider/MCP action is executed. No API/auth/permission/storage behavior changed, no Figma/commit/push/release, and Fetch was not relaunched. User must explicitly Quit/reopen to observe the rebuilt UI. Concurrent project-scoped/Workspace backend work is preserved.

## November 12 — independent project choices

project_scope.rs reads and registers projects through the native API. Workspace
registration only saves the folder. Server filtering, installation destination
and app connection each keep their own Shared/project choice. Native Settings
saves membership through the existing config API; API errors stay beside the
selector. Project snippets preserve each app’s format and do not edit client
files. The shared approval popup shows the project folder and leaves session
history access unchecked until explicitly selected. Connector proof/profile
operations use the client plus project together. New UI checks use returned API
fixtures and dropped tasks; verification is coordinated by the Orchestrator.

## November 12 — shared approval code, blue Allow and refresh hover

The single approval popup used by both Claude Code and Codex now uses the shared blue primary Allow button. Request code is labeled above the existing shared code block with its Copy control, which copies only the code. Removed the requested Match this code… sentence. Install refresh keeps the shared hover glow/rotation enabled when idle even with an empty path; its existing blank-path handler still performs no discovery and starts no request.

25 existing actual-widget checks pass (9.38s), including request-code Copy, absence of the removed sentence, both app popup flows and empty-path hover animation/no-discovery. Inspected Claude and minimum-size Codex popup PNGs. Signed development build-only/strict signature verification passes (17.68s); git diff --check passes. No new test infrastructure or tests added; existing cases were extended for the requested behavior.

Direct UI work; no subagents for this pass. Fetch was not relaunched. No live provider/MCP, credential/trust/config, API/authentication permission, Figma, commit/push or release changes. Explicit Quit/reopen is needed to see the new executable.

## November 12 — Workspace

workspace.rs adds global/project folder selection, Instructions/Subagents/Sessions,
Claude/Codex tabs, file editing, copied/conversion drafts, dynamic model choices
and expandable bounded log reads. Every file effect calls the native HTTPS API;
this module does no native file discovery/parsing/writing. Native file paths and
review issues come from the backend. Pending editor operations keep their save
target stable and old scope replies are ignored. Navigation has a fourth shared
Workspace action. Session MCP is deliberately deferred by user selection.

## November 12 — persistent server on/off implementation

Servers share a 44×22 on/off toggle before their existing actions. Page state calls the common restart API, tracks pending state, invalidates old reads, and opens the matching Console for returned power-operation errors. Compact Connect auth controls remain black/22px. Locked library compilation, 30 focused checks, final signed build and independent Evaluator acceptance pass; live user/provider acceptance remains separate.

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

Connector approval now keeps UI-owned proofs across backgrounding; already-approved replies claim using proof-only pinned HTTPS independent of a live native window session. Denial/expiry clears ownership for retry. The focused popup/lifetime checks and source evaluation pass; live native acceptance is deferred by user.

## November 12 — inline refresh and shorter Servers feedback

Installed servers Refresh now uses a 1.5-second minimum indicator and continues spinning if the read takes longer. Install MCPs Refresh is now in the path-input row, immediately left of Choose file or folder…. Its three-second minimum is unchanged. Both use the same shared feedback implementation; the caller supplies the duration. Returned data still applies immediately.

Four focused existing UI checks pass: manual refresh pending spin, instant reload/minimum feedback, discovery click/spin/retry, and normal/minimum-window screens. Inspected the 874×630 filled-path and 760×520 Install screenshots. Signed development build-only passes (15.03s), including strict signature verification; git diff --check passes. No new tests were added for this small adjustment. Direct work/review without subagents. No Fetch relaunch or live MCP/provider calls; running Fetch needs explicit Quit/reopen to show the changes. Unrelated edits, credentials, personal settings and services were preserved.

## November 12 — shared button behavior

Refresh controls on Servers and Install MCPs now use the same shared rendering and running-action guard: blue hover glow, clockwise hover rotation, continued rotation without hover while running, and no second click during activity. Server-list and discovery reads share RefreshFeedback for a three-second minimum indicator without delaying returned results. Server restart continues spinning through its real pending operation and now also uses the component-owned click guard. Text actions across pages, native connector setup/approval, credentials and Terminal reuse text_button; blue Install/Save actions reuse primary_button. Existing specialized copy/tab/disclosure controls retain their common implementations and purposes.

refresh_feedback.rs owns the common request/minimum-feedback state and timer. components.rs owns text_button, primary_button and the refresh running-action guard; page modules supply only labels, actions and availability. 78 UI checks pass (10.59s), including the new real-widget discovery success/failure/retry case and existing hover, server reload, restart, copy, Settings, connector-popup and terminal checks. Inspected Install idle/feedback and minimum-size Connect PNGs. Signed development build-only passes (20.85s) and the launcher’s strict signature verification passes. git diff --check passes. The first wider run found a stale Connect assertion expecting pre-existing HTTP commands; it now checks the current stdio connector command already implemented elsewhere in the working tree. Existing compiler/deprecation warnings remain.

Checks/review were performed directly and sequentially, without subagents or independent review, as requested. No Fetch relaunch, MCP/provider call, client-config edit, credential/trust/signing-selection change, Figma edit, commit/push or release. Running Fetch retains its earlier executable; explicit Quit/reopen is needed for live observation.

## November 12 — connector approval popup

connector_setup.rs owns API-backed client request/proof UI state, selected scopes and native approval popup. Connect apps offers setup and stdio registration commands. The UI uses the four backend operations; it never mints grants, changes trust or edits client configurations. Existing shared controls/focus are reused. Actual production popup/Keychain acceptance remains unobserved.

## November 12 — final reload, copy, startup status and resize UI

copy_feedback.rs wraps native copy actions with local plain Copied! state and glyph-only hover; address action uses it too. surface.rs owns display-only3s reload arrow feedback/240ms card entrance, blue/red glows and existing scroll containment. App headings use gray bold labels with marks. Starting spinner/dot derive from actual server/start request states; successful named records/auth responses update the matched card before a list read completes. Existing pending one-second reads continue for collapsed startup. No API actions are synthesized by rendering.

## November 12 — inline restart failure

Shared server_identity accepts a restart-failed display flag and reuses the gold inline warning. Desktop restart_failures stores rejected submission details per server until retry/removal; actual terminal restart records update the matching server immediately. console_lines includes restart.error and local rejected-submission text unchanged apart from the existing Fetch prefix. No global notice rendering or backend behavior change.

## November 12 — contain internal scrolling and remove page banners

surface::contain_scroll transparently wraps a scroll area, forwarding child input/drawing/operations then consuming wheel events inside its visible bounds. output_frame contains the complete Console/Terminal frame; other native scrollers are wrapped at their shared call sites. No custom scroll positioning/physics. Global self.error/self.notice are no longer drawn under navigation; popup-local rendering and Console errors remain.

## November 12 — gray key button

Shared Icon::Key face is #B8BDC3 for enabled/disabled actions; white icon and enabled hover glow retained.

## November 11 — spin refresh through completion and glow key hover

The Installed servers Refresh button spins from the user's click until its matching list response finishes, including errors, without requiring hover. Its own request sequence keeps ordinary polling separate. A per-server restart button spins from initial submission through the existing restart-status polling until the restart reaches succeeded/failed; other servers do not inherit its animation. Shared glyph state keeps request rotation independent of hover, including when the button is temporarily disabled. Removed the server-restart success notice and its generic Saved acknowledgement; restart failures remain visible. Enabled key buttons now share the stronger blue hover glow already used for refresh. Existing hover spin and key authentication actions remain unchanged.

Request animation is display state only: manual_refresh owns its list sequence, restart_submitting spans POST until the existing restarts map takes over. Shared refresh_button passes activity into the glyph; backend/polling contracts stay unchanged.

## November 11 — remove Retina outline, fix all native hover and align resize

Corrected the dark rounded outlines caused by linear filtering between face pixels and transparent black texture pixels. Fully transparent texels now retain the face RGB at zero opacity; no stroke or shadow stack was removed. Retina edge regression fails before the fix (192/191/187) and passes after (238/237/232). Removed the8px transparent outer page margin and moved it into existing content padding, preserving control/card positions while the visible face reaches the real window bounds. Text/icons remain at native resolution. No API or backend changes.

## November 11, 2026 — expansion, refresh hover, left window controls and right shadow

Implemented the requested native UI changes directly, without subagents. First server expansion draws without server I/O; logical-density soft shadow textures with linear sampling reduce the measured Retina fixture from199–201ms to72–97ms, with8–16ms repeat expansions. Text/icons keep full display resolution. Enabled shared refresh buttons gain a stronger blue glow and a1.2-second clockwise hover rotation; leaving/disabled state stops rotation and existing actions remain. Actual AppKit window buttons move from top-right to top-left24px/top18px. A tracking area and AppKit group-hover hook expose native symbols. Only clipped right-side Panel shadows fade through the existing gutter; left shadows, card bounds and scrolling stay unchanged.

## November 11 — automatic Console on expansion

Expanding a server selects Console directly, including when reopening a card after Tools/Settings/Terminal. The shared Console assembles retained service lines and current Fetch authentication/runtime/request/read errors; Copy uses that same text. Exact returned error messages/codes/details remain visible alongside server output. Card-level authentication/runtime error duplicates are removed. This supersedes the older explicit-only Console and fallback-only display descriptions below. Authentication history and terminal sessions still use their existing owners; expansion does not start either.

## November 9 — native controls and compact panels

The main page uses only inside shadows with fully transparent outside pixels. Native setup runs through the exact Iced window handle; actual top-right AppKit buttons and system-shadow removal live in native_auth. A single outer MouseArea drags unused content; children retain clicks, editing, terminal selection and scrolling. Installed servers title is above the recessed list. Tools is300px high/max744px wide with200px compact cards. output_frame is shared by Console/Terminal, with header Copy and padding; the terminal uses the same13px font and blue-gray palette and retains existing emulator/session behavior.

## November 9 — requested hover and output response

Shared Surface adds a cached blue halo for enabled filled buttons, fading in over150ms through normal redraw events; no animation timer/subscription or repeated rasterization per frame. Hover ends return to idle; selection alone adds no halo. Native tooltip wrappers removed; boot/reopen default is874×630. Actual pane-response measurements and tests are tracked in milestones/hover-glow-pane-response. Existing APIs, logs and terminal sessions are retained.

## November 9 — unclipped tools and frameless window

components::tool_grid uses equal-width native rows/columns, preserving three columns while allowing decorative shadows into scroll padding. mod.rs shares transparent/undecorated window settings between boot and reopen, clears the daemon transparently and wraps navigation in a MouseArea: unused title/space emits native DragWindow, children capture tab clicks normally. This is the user-selected fully frameless appearance. Existing Tools scroll, card content/actions and page dimensions remain. Actual OS window/drag appearance awaits relaunch; source-real tests/screenshots and signed build pass.

## November 9 — bounded Installed servers list

The Servers page fills the available window without an outer page scroller. Its recessed section keeps title/Refresh fixed; fetch-installed-servers scrolls only the padded server cards below them. Opening a card/panel grows list content, not the section. Existing inner tool/settings/output scrollers and backend actions remain. Normal/minimum-window interaction/render verification is tracked in milestones/fixed-installed-server-list.

## November 9 — ready server hover

mod.rs renders ready server identities without the generic runtime tooltip. Other runtime states keep their current tooltip. The colored dot, disclosure and action controls are unchanged. Existing server-panel interaction check and signed build pass; native hover observation is pending relaunch.

## November 9 — responsive tab drawing

surface.rs now shares up to 32 immutable decoration images on the UI thread so tab switches and pressed/selected states reuse earlier shadows. Large surfaces draw preserved corners and constant edge/center strips from compact resvg images; small controls retain full-size images. Each widget still holds its own renderer allocation, display scale is part of the cache key, and clipping/foreground order remain. The cache contains generated decorative pixels only. Colors, filters and input behavior are unchanged. The actual-widget Retina navigation regression covers page switching, the observed stall and identical first/return frames. See milestones/navbar-latency for measured evidence and native-runtime limits.

Final fidelity verification: independent 54 UI checks pass; after the final rendering fixes, two fidelity and three affected interaction/render checks pass again. Locked binary build passes in 6.69 seconds. Cold/cached rendered pages are identical, and first-frame backgrounds/glyphs/checkbox boxes/number plates are verified. Independent Evaluator ACCEPTs the visual correction. Literal native/Figma pixel equality is not established; measured font/blur-edge differences are recorded in milestones/neumorphic-fidelity/info.md. No native app was launched.

## November 9 — flat faces and exact Figma shadow layers

The earlier gradients and hard rims are replaced by generated decoration SVGs using Figma's actual fill, radius, shadow colors/offsets/blur and layer order. The pinned resvg library renders them; pixels are demultiplied once before Iced draws raw RGBA. This avoids the observed dark SVG halos. Surface Tree state retains one raster/allocation, reused until geometry/effect/color or physical display dimensions change. Window Opened/Rescaled supplies current DPI through a scoped draw wrapper; native logical layout stays unchanged.

Decorations use the same nearest image batch as original SVG glyphs, and child content draws in a later viewport layer. This preserves first-frame backgrounds, white icon edges, checkbox boxes and white numeric strips. Native Button/TextInput widgets still own input/editing/clipboard/secure masking. Debug optimization is restricted to resvg/tiny-skia filters; Fetch remains a debug build. No external services or third-party source patches are introduced.

Geometry/typography now follow the current contexts; complete Tool JSON remains in API/state while cards show only the design's Name/Required fields. Actual verification and residual font/blur-edge differences are tracked in milestones/neumorphic-fidelity/info.md.


Final November 9 verification: 52 focused UI checks pass; 3 affected actual-widget tests pass again after the final wrapping fix; locked binary build passes in 6.01 seconds. Sixteen fresh sample-data native-view captures and source/evidence receive independent Evaluator ACCEPT. Actual native launch is untested because this Mac lacks the selected dedicated development signer. See milestones/neumorphic-dev-sync/info.md.

## November 9 — native neumorphic design port

Shared warm-white/blue tokens and controls now drive three navigation tabs, recessed Installed servers/tools sections, round header actions and inline server Settings. surface.rs draws paired panel shadows and narrow raised/recessed rims while forwarding layout, events, overlays and widget operations. Inter regular/semibold/bold and Roboto Mono are bundled with their SIL licenses; no global fonts are installed. Exact Figma SVGs live under assets/.

Console is hidden until explicitly opened. Tools/Settings/Console/Terminal select one panel per server; hiding a terminal preserves its session and output. Settings keeps existing folder/manual-key/execution APIs; the two numeric arrows use existing bounds. Shift-click restart sends the existing force flag. Tool cards show Name/Required fields with full raw JSON available through Raw definition; reading/scrolling cards does not call tools. Discovery results/progress appear in a popup and can be reopened after closing. Manual command installation remains unimplemented because no execution contract is selected. Native OS titlebar remains system-owned.

Current verification is tracked in milestones/neumorphic-dev-sync; early diagnostic captures do not establish final visual acceptance.

## November 7 — automatic server list refresh

The open app schedules GET /api/v1/servers every60seconds through the same refresh() and NativeClient used by the Refresh button. AutomaticRefresh reuses existing pending-read tracking and skips while a read is pending or the native client is absent. Faster one-second console/auth/deletion/restart/installation checks and manual Refresh remain. Closing the window still revokes its session and removes the client; this adds no background management authority.

## November 7 — hide healthy authentication labels

servers_view keeps the server name and runtime in every card header. It omits the authentication suffix for authenticated/not_required; other statuses retain their exact suffix. This changes display only. Independent six-state Desktop::view render and locked binary build pass; no app was launched. The neumorphic UI and manual-install popup designs remain Figma-only.

## November 6 — remove cleanup confirmation

Connect apps no longer contains the manual connected/verified checkbox, its revealed cleanup prompt or Copy cleanup prompt button. Associated local state/message/reset code and unused text size are removed. Copyable client command blocks remain, with Copy alongside the commands and no separate blank header row.

## Copyable connection commands — November 6

Connect apps uses components::code_block for Claude Code and Codex instructions. Each fitted-height rounded block contains monospace commands and its own Copy button alongside them. The helper sends the same displayed string through Message::Copy and the existing clipboard action, preserving the Codex newline and current Fetch address. Clicking Copy does not execute commands or edit coding-client configurations.

November 4 unsupported import rows: install_view displays each server error.message using the shared Message component below its existing disabled checkbox. Figma Remote returns unsupported/unselected; no new layout/component or automatic server action.

## Mouse input in a scrolled server page — November4

TerminalView uses the parent-adjusted Cursor for both button presses and movement. Iced Scrollable leaves the original mouse event in window coordinates while translating its child cursor; mixing them can select several rows on a tiny click movement. A two-line vendored routing fix keeps both consistent, without changing intentional selection or cursor style. Actual outer-Scrollable input/render regression reproduces before and passes after;11focused terminalUI checks/native build/review pass. See milestones/terminal-pointer-coordinates/evidence.md; native user observation remains separate.

## Terminal selection and trackpad handling — November4

The vendored terminal view ends selection drag after a left release outside the pane and accepts wheel gestures only over its visible area. Pixel wheel direction matches line wheel and sub-line movements stay captured, so the surrounding page does not consume a terminal trackpad gesture. Intentional selection and current text cursor style remain. Focused actual-widget/render verification passes10checks2new plus native build and independent review; see milestones/terminal-selection-scroll/evidence.md. Rebuilt native observation remains separate.

## Console and Terminal — November 4

server_terminal.rs retains each server's selected pane, existing iced_term parser/screen, session ID and byte cursor. It calls only native terminal APIs; the backend owns shell/environment/processes. Shared output_controls and terminal_pane live in components.rs. Console/Terminal switching and collapse preserve screens; exited screens require explicit Start terminal to replace. URL-only remote servers show a disabled Terminal button and reason. Writes stay ordered, Close bypasses blocked input, stale window/session responses are ignored, and output polling continues while Console is selected. Raw terminal bytes are not persistent console log entries. Native visual acceptance remains separate.

Continuous-console verification: 150 relevant checks, including eight new cases, native build and independent review pass. New request errors are distinguished from old output. Native window appearance and actual accounts were not tested.

## Permanent expanded console — November 4

Every expanded server renders the shared console, including idle and not-required states with no prior authentication request. Ordinary status reads create passive display state and refresh expanded consoles. Displaying a console does not authenticate or mark a server busy. Authenticate/manual saves reuse its history; cumulative backend snapshots replace the displayed service portion to avoid duplication. Log updates do not overwrite a manual-storage verdict. A new request records which output was already present, so old logs cannot hide its new local failure; the fallback is removed only after genuinely new service output. No synthetic progress or credential-store changes.

## November 4 console wraps service output

The shared console renders captured service logs/instructions/errors and observed process exit status. It no longer invents authentication phase or host-progress log entries. Generic localAPIerror fallback is removed when service output arrives; genuine Fetch request/read failures remain visible. Sign-in state/consent controls, scrolling and Copy remain. Credential values never become console input.

# Native Fetch interface

The native UI calls the same HTTPS APIs as other clients. It has no server process,
configuration-file, credential-delivery or authentication implementation of its own.
Closing the window revokes its native Settings session; explicit Quit stops Fetch.

## Server setup and manual keys

Settings keeps Fetch call limits and the shared Open server folder action. The
folder POST accepts only an installed instance ID and sends no body. Backend
resolution and the system opener decide whether opening succeeds. Unknown native
locations are reported as errors; the UI does not assume the package folder is the
configuration folder. Folder requests track each server separately and do not block
another server's authentication. Stale replies from a closed window are ignored.
The arbitrary launch/configuration JSON editor is removed.

Use a server's own setup first. For imported environment/header inputs requiring a
manual fixed key, Settings and inline authentication reuse the single masked
`authentication::credential_editor`. The existing credentials PATCH requires fresh
Mac confirmation and direct Keychain saving. The UI moves the submitted value out
of its field before awaiting the request, clears other stale entry fields, and
never logs secret-bearing messages. A storage result is displayed separately from
account authentication; even an older authenticated snapshot cannot approve a
failed or uncertain save. Native OAuth files and remote OAuth renewal are not
manual key edits.

Explicit Authenticate is available for unknown, required or failed status and sends interactive:true. Unknown never means that login is unnecessary; recorded native setup runs or returns a clear unsupported result. Authentication, manual saves and
server reads have per-instance request ownership in auth_requests.rs. Pending
responses are monitored without opening repeated browser flows. Unknown native
login behavior remains an honest setup limitation. Expanded server cards display
safe connection expiry and last-verified timestamps; null values stay unknown.
The UI displays human next_steps text, but never executes it or uses it to select
workflow actions. Raw tool definitions are shown without rewriting them.

## Shared controls and existing installation behavior

components.rs owns ordinary buttons, inputs, checkboxes, navigation, page/message
framing, server cards and disclosures. theme.rs owns common visual values and the
Iced light appearance. The shared server_header includes registration removal;
confirmed removal clears only UI caches and explains that files and credentials
were preserved. Server/tool details expand independently.

Install starts with an empty list and an always-visible path input and picker.
Choosing a file/folder starts discovery through the existing API; a typed path
uses Enter or Refresh servers. Blank paths never request global discovery. The
native AppKit picker includes hidden locations. Selecting a path does not install
anything. The title has a small gray folder hint; the old explanatory paragraph
and app-wide tagline are removed. During installation the footer button is gray
and disabled, and no installation-accepted notice is shown. Existing
source/name associations show checked disabled installed rows; these do not enter
manual/bulk selection or install payloads. Progress updates rows, while failed
siblings remain retryable. The discovery list scrolls inside a bounded area; the
Select all toolbar and Install selected servers footer remain visible.

## Evidence boundaries

Historical builds/UI fixtures describe their original source in milestones.
New manual-Keychain/folder/auth-mode changes require the current integrated
checks; no native window, Finder, Mac confirmation or real account acceptance is
established by their source or window-free fixtures. Figma/state reconciliation
is tracked centrally in docs/UI_STATE_COVERAGE.md.

## Standardized sign-in panel

Authenticate and its pending Open sign-in at HOST control call the existing
/restart start/connect API with interactive:true. The second control remains
available for an offered unopened host while other same-server changes stay
blocked. It uses the backend-held pending link, never a path/URL supplied by the
UI. authentication.rs tracks safe host/opened state through existing polling and
clears it when the attempt ends. No generic folder/key form is rendered inside
this panel; assigned manual values remain in native Settings using the shared
credential editor. Last-known status is labeled separately from current proof.
Concurrent folder-first Install work is preserved. Native visual checks pending.

## Authentication console presentation

The inline auth view now targets a single scrolling monospace console. Flow owns
per-attempt safe progress/error history; rendering remains in shared UI components
and colors/sizes in theme. The existing host-consent action stays inside the
console. Do not turn instructions into workflow or expose input values/raw URLs.
A separate raw process-output feature needs an explicit privacy/delivery decision;
this presentation work changes no application route or server startup behavior.

The console also appends native-only sanitized startup stderr and natural exit
status from authentication.startup. It deduplicates repeated/rolling tails by
process generation. A failed ordinary Start/Connect creates a console from status
without issuing an authentication request. Known values and full HTTP(S) URLs are
filtered before retention upstream, rather than only hidden by the widget.

The shared console keeps a small Copy button fixed at its top right. The caller
passes the current log joined with newlines to the existing clipboard action;
scrolling log content and host consent remain below the fixed header.

## Automatic first connector approval — November 12

Connector approval automatically adopts exactly matching durable pending proofs and saves approved client credentials even after the requesting stdio process exits. The private app activation intent carries only the client label; all request/decision/claim behavior uses existing APIs.


Conversation presentation now consumes complete native messages; raw JSONL parsing is being removed. Workspace subagent preview uses the existing definition parser and Iced Markdown. Native readers own branch/history selection.


Final November13 state: normalized full conversation messages replace custom raw-log parsing. Failed tool output:null uses its actual recorded error for display/Copy. Subagent popup renders native parsed settings and cached Markdown; Edit reuses the instruction editor. Focused widget and DTO checks pass; source snapshots are documented under milestones/official-session-readers/evidence.

Workspace session-list failures are kept separately from file/discovery errors.
They appear only under Sessions, preserve the reader's actual message, and clear
when that request restarts or succeeds or the selected app/folder changes. A
background history failure does not add an error above instructions or subagents.

Workspace subagent names use the shared blue primary action. Their preview keeps
parsed settings above an exact instruction text area, using the same shared
12px monospace file renderer as AGENTS.md/CLAUDE.md. Preview permits selection,
Copy and scrolling but ignores editing actions; Edit retains the full original
file. Instruction/subagent paths use the common inset code-box style with exact
path Copy and an adjacent Finder action; long paths wrap inside the window.

Complete session messages cache Markdown at API response time. Ordinary user and
assistant messages render that Markdown in white and gray cards respectively;
named tool outputs keep their existing exact-output disclosure and Copy. Original
message/source strings remain unchanged. A failed session list shows its actual
error without the misleading empty-list sentence.

Session tool-result cards have one blue Copy glyph in a transparent hit area at
the top right. Expanding output does not add another Copy control. This uses
session-specific shared helpers; other Copy controls retain their normal faces.

The headless ViewFixture lays out each fresh view before operating, dispatching events or drawing it, matching Iced's runtime layout step. This initializes size-dependent path children before checking their actual Copy/Finder controls.

Session tool-result cards now use the same gray face as assistant cards. Their
collapsible names use the shared inset code-box style; the header retains its
single bare blue20px Copy glyph. On conversation load, valid JSON object/array
strings get a separate pretty display cache. Expanded output wraps even long
unbroken values by glyph. History text and Copy retain the exact original bytes.

File-path code boxes use8px vertical/12px horizontal padding and a bare blue14px
Copy glyph in a28px transparent hit area. The shared responsive helper reserves
that actual width, with the28px Finder action immediately after the code box.
The ordinary full-size application Copy controls remain unchanged.

Session Markdown uses Iced's supported Viewer with the shared light inset code
box for fenced blocks. Fences call the same code_block_without_copy helper as tool results, keeping
exact code text, shared13px monospace and glyph wrapping; inline code uses the light code background, normal ink and the same
14px body font as surrounding prose. Cached Markdown and the normal heading,
bold, list and quote renderers are retained, and links remain inert.

Tool-result Copy now returns the displayed tool name, two newlines, then its
original raw result. This supersedes output-only Copy for those cards; readable
JSON remains display-only. The native opening window is1058×789 logical pixels,
matching the measured current window; minimum size remains760×520.

Default-size pixel fixtures derive row strides and capture names from actual view dimensions. Three hover comparisons use a consistent redraw timeline after the existing initial card entrance settles; application animation behavior is unchanged.

Escape is handled by the window-wide event subscription even when a focused
editor captures the key. It dismisses one topmost visible popup in drawing
order: connector approval, Workspace subagent, Workspace session, then Install
discovery. The existing close actions preserve approval requests, editor drafts
and cached history. Keys from another window and Escape with no visible popup
do nothing. Modifier and scale-factor event handling is unchanged.

Keyboard fixtures expose the actual widget Shell status and then call the exact production subscription mapper, since headless widget dispatch does not execute subscriptions. Escape feature cases cover focused editing and layered pending popups without running API tasks or native apps.
