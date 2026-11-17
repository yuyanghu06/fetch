# Keep the startup panel clear until the server list arrives

- Local timestamp / acting role / milestone: 2026-11-17T15:49:02-05:00 / Orchestrator / Startup and Workspace polish
- Requested scope: Avoid a false No servers installed message immediately before the startup pop-in.
- Starting repository state: Seven focused widget tests passed. The boot state carries an empty array while its first read is pending.
- Work performed / changed files: mod.rs now suppresses the empty state while waiting. The tester is checking both the pending state and a successful reply with no servers.
- Decisions: Show the centered message once an empty list is known. No change to when the read is sent.
- Checks and evidence: Independent source review confirmed the initial flash risk. git diff --check passed; retest and build are pending.
- Resulting state / acceptance: Source corrected, final checks running.
- Remaining work and handoff: Close with final widget results and signed build.
