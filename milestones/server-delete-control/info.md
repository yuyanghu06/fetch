# Server header delete control

User requested a small × beside every server disclosure that calls the existing
DELETE route. Shared server_header uses common theme values; the button is
separate from disclosure and available when the server is idle. Per-server
pending state prevents duplicate changes. Confirmed deletion clears cached state
and refreshes; failure/uncertainty reads status without automatic DELETE replay.

Status: source implemented; independent source ACCEPT, native binary build and
14 existing UI-handler checks pass. No actual button click/window/Keychain or
server deletion was executed; user runtime verification and Figma mapping remain.
