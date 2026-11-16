# Inline restart failure

Goal: show restart failure beside the server name like its sign-in warning.

Restart failure now appears as a gold warning icon and Restart failed text beside the affected server name, using the existing sign-in-warning style, including collapsed cards. It takes precedence over the sign-in label for that failed attempt; ordinary sign-in labels remain otherwise. Failed restart records and rejected restart submissions both drive the header. Retry hides/clears the previous local failure while its arrow spins; terminal status updates the matching server record immediately so completion clears/restores the correct inline label without waiting for a later list read. Actual returned restart errors are included in the existing per-server Console/Copy text. The old global restart-failure notice is removed; no page banner is restored.

Acceptance: correct shared inline style, per-server failure/retry/completion and original error details, no global notice, focused checks and signed build.

Fifteen real-widget UI checks and four visual checks pass. New scoped fixture verifies header placement and one-server ownership, saved failed status, rejected submission, full returned Console error, retry progress, immediate success/failure status and no global notice. Inspected restart-failed-inline.png; signed development build/strict signature verification and git diff --check pass. Direct primary-agent review ACCEPT; no subagents. No fixture HTTP tasks or real servers executed.

Existing running Fetch, personal accounts/Keychain/client settings, backend API behavior and Figma remain unchanged. Built executable requires explicit user Quit/reopen. Earlier unrelated source/doc changes and unfinished acceptance work are preserved; Keychain prompt issue remains separate.

Status: scoped implementation/checks complete and directly accepted; user live relaunch remains.
