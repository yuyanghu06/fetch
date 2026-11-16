# Refresh request animation and key hover

Goal: spin clicked refresh/restart arrows throughout their operation, remove the per-server success message, and strengthen key hover.

The Installed servers Refresh button spins from the user's click until its matching list response finishes, including errors, without requiring hover. Its own request sequence keeps ordinary polling separate. A per-server restart button spins from initial submission through the existing restart-status polling until the restart reaches succeeded/failed; other servers do not inherit its animation. Shared glyph state keeps request rotation independent of hover, including when the button is temporarily disabled. Removed the server-restart success notice and its generic Saved acknowledgement; restart failures remain visible. Enabled key buttons now share the stronger blue hover glow already used for refresh. Existing hover spin and key authentication actions remain unchanged.

Acceptance: actual-click timed rendering before/after replies and restart terminal status, no success notice, existing hover/disabled behavior and visuals, signed build.

Twelve real-widget UI checks, four visual checks and two automatic-refresh checks pass. New actual-click fixtures verify global refresh spins without a pointer, stops on success/error, and individual restart continues through submission/status then stops without the success notice. Existing timed key-hover screenshot was inspected. Signed development build and strict signature verification pass; git diff --check passes. No task/HTTP/real server is executed by UI fixtures. Direct sequential review ACCEPT; no subagents.

Running Fetch was preserved and not relaunched. No account, Keychain, personal client, backend API or Figma changes. Actual native animation still needs user Quit/reopen into the new binary. Prior Keychain prompt investigation remains unresolved; this rebuild does not claim a prompt fix. Existing unrelated UI/console/native/docs work and unfinished acceptance remain.

Status: requested source/fixture scope complete; user live relaunch remains.
