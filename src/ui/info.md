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

Opening Install discovers normal global definitions automatically. Optional file
or folder selection uses file_picker.rs and the native AppKit picker, including
hidden locations. Selecting a path alone does not install anything. Existing
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
