# Nested scrolling and page-level messages

Goal: keep scrolling inside every internal pane at either boundary and remove all messages shown beneath navigation, with a removal inventory.

All native scroll areas now use the shared contain_scroll wrapper: the child receives wheel input first, then any unhandled wheel input inside its visible bounds is consumed, including at top/bottom or when content fits. Shared Console/Terminal frames contain wheel input over their header/padding as well. Tools, Settings, discovery, installed servers and the outer scrolling page use the same rule. Pointer movement outside an internal pane leaves the surrounding list usable. Removed both error and notice rendering directly below navigation on every native page; retained existing contextual Console/installation-dialog messages and underlying response/error state. Exact removed text/categories are inventoried in removed-messages.md. Recorded both user rules in project AGENTS.md.

Acceptance: real nested pane top/bottom/outside scrolling, no global messages on all native pages, existing visual/state checks and signed build.

Fourteen actual-widget UI tests and four visual tests pass; final signed development build/strict signature verification and git diff --check pass. New fixture checks Console/Tools/Settings at both boundaries, repeated new gestures and outside-pane outer scrolling. New three-page fixture verifies global error/notice text is absent. The existing fixed-list test initially failed because it tried to scroll the outer list over the newly contained Console; corrected its pointer to the list gutter, matching the requested behavior, then all14 pass. No new live server, account or native Fetch session was started. Direct sequential review ACCEPT, without subagents.

Running Fetch is unchanged until the user explicitly Quits/reopens the built executable. No API/backend transport/error content, account/Keychain, personal settings, Figma, release or unrelated source changes. This removes the page-wide display rather than relocating all messages; contextual errors remain only where they already existed. Prior unfinished acceptance and Keychain prompt diagnosis remain.

Status: scoped direct review ACCEPT; live user relaunch pending. [Removed messages](removed-messages.md).
