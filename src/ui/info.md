# Native Fetch interface

UI reuse requirement (confirmed October 28): use shared rendering functions or
modules for repeated navigation, buttons, server cards, plain messages/status,
text inputs, tool disclosures and checkboxes, with common theme values. Pages supply
data, state and callbacks. See [the component definitions](../../docs/SHARED_UI_COMPONENTS.md).
Matching Figma components exist. `components.rs` now owns common native buttons,
full-width inputs, checkboxes, navigation, page framing, plain/error messages,
server-card framing and tool/server disclosure rendering. `theme.rs` owns their
current sizes, spacing and padding and keeps the Iced Light appearance. Pages
still supply data, availability and callbacks; these helpers do not call APIs.
`authentication.rs::credential_editor` remains shared by Settings and inline
authentication and uses the same controls/theme. Browser rendering is managed
outside this native module. The current state library is indexed in
[UI_STATE_COVERAGE.md](../../docs/UI_STATE_COVERAGE.md), including all nine auth
phases, individual Settings outcomes and interaction/disabled differences.
The user rejected the earlier styled proposals. Current Figma copies preserve
this source's controls, hierarchy and scrolling; do not introduce badges,
colored notices or two-column Settings as part of copying it.

Iced renders a light native window. The window holds a short-lived in-process Settings session and calls the same HTTPS application APIs as agents. Server records, installation progress and restart outcomes drive controls; next_steps text is never interpreted as UI instructions. Manual secret changes go through the existing credential PATCH and a fresh backend Mac confirmation. Closing the window drops its authority and leaves backend work running; Quit requests backend shutdown.

The D77 interface source was initially delivered without compilation or tests;
later binary-build evidence is recorded in the milestone. The October29 shared
native extraction is awaiting the coordinated build and actual UI checks. The
UI does not edit client configurations, install certificate trust or execute
tools merely to inspect them.

Server/tool details expand individually. Call limits have ordinary controls, while launch/reference/per-tool edits have an optional advanced editor. Manual secrets are hidden on entry; closing the window immediately revokes native authority even if an HTTP request still holds a client clone. Internal HTTPS trusts the installation certificate with built-in public roots disabled. Named monitoring survives status-read errors; retries select failed installation rows only.

The Install MCPs path field has a Choose file… control using file_picker.rs and
the existing AppKit bridge. The native picker shows hidden files, permits one
file, and returns its absolute path without reading its contents. Cancel leaves
the field unchanged; Discover servers remains the explicit API action. Native
UI calls are dispatched to the main queue while Rust awaits completion. This
change does not add an API, automatic import or configuration write.

The Install MCPs page reserves a bounded scrolling area for discovery rows and
installation progress, with its action footer outside the scroll area. The
Install selected servers button stays mounted and is disabled when there is no
selection or another incompatible operation is active. Other pages retain their
whole-page scrolling. This keeps long imports from hiding the install control.

Select all sits in the fixed toolbar above the discovery list. It selects only
supported path/name pairs, including disabled-source supported rows as an
explicit user choice, without changing source flags or starting installation.
Busy, picker and active-install states disable the bulk action. The footer shows
the selected count; reopened windows retain the same minimum dimensions.
Locked native compilation passed; actual layout interaction remains unverified.


Inline authentication is implemented in `authentication.rs` and `mod.rs`.
`authentication.rs` owns card-local authentication presentation/state and the
credential editor shared with Settings. Existing config references supply
credential names; backend authentication statuses drive UI decisions. Human
next_steps text may be displayed but is never executed or used to route actions.
Railway browser sign-in is now implemented in the shared manager; source/build
evidence and live verification remain separate. Other unknown integrations
show their precise unsupported result, never a generic Saved notice.
Source/build review for this change is recorded in the active milestone.

Pending authentication status, installation progress and each named restart now
have independent read-in-flight guards. One slow install/restart no longer
prevents checking a different server's browser sign-in.

## October 30 independent authentication requests

`auth_requests.rs` owns request tokens by server and ordered server-read IDs.
Explicit authentication, inline key saves and Settings key changes use that
tracking instead of the general busy flag. Different servers can have outstanding
requests; a same-server edit/restart/duplicate remains blocked. Closing clears
request ownership, and old completion/status-read messages cannot replace newer
results. The existing shared editor, masking, API calls and layout remain.
Independent tests include helper/Flow states and use a test-only host to exercise
Desktop messages without opening a window; actual native appearance is separate.
