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
