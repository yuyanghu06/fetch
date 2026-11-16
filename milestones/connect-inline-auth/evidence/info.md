# Connect apps UI evidence

ui-tests.txt records the final 24 passing native widget checks. inline-auth-test.txt records the initial focused key/dismiss/reopen check before the later recessed-panel change; the final UI suite includes that case with the final layout. signed-build.txt records the final signed development build (10.90s) through the existing launcher and strict verification.

Connect PNGs show the final recessed panel at normal/minimum sizes. connect-inline-auth PNGs come from actual-click fixtures (the compact one scrolls to the Codex key). inline-auth-popup PNGs show final rounded dimming and the shared red Close control; install-popup shows the shared discovery framing. Captures are offscreen widget renders; raw buffers were converted to PNG with Python's standard library and removed.

No live app, Touch ID, client connection, MCP or provider acceptance is established. Review was direct, without subagents. The initial disk-full attempt and bounded stale compilation-cache cleanup are recorded in the pass history.

Latest follow-up: compact-auth-spacing-tests.txt records 24 passing checks after the 22px buttons and halved title gap; compact-auth-spacing-build.txt records the final signed build (10.54s). PNGs now reflect these final sizes/spacing. The prior logs retain their original pass scope.

Black-button follow-up: black-auth-test.txt records the passing existing actual-click test (1.86s); black-auth-build.txt records signed build/strict verification (15.46s). connect-black-auth PNGs show the black 22px controls. Earlier PNGs retain their original gray-button scope.

Latest popup/hover follow-up: approval-code-blue-allow-refresh-tests.txt records25 passing widget checks (9.38s); approval-code-blue-allow-refresh-build.txt records17.68s signed build/strict verification. connector-approval-popup and inline-auth-popup PNGs now show blue Allow, request-code Copy and removed sentence. Initial empty-path hover rotation is measured by timed actual-widget pixel comparisons; its handler starts no request. Earlier acceptance remains scoped to its original logs.
