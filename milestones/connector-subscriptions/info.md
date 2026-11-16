# Connector subscription correlation and cancellation

Goal: remove two confirmed modern-protocol transport defects exposed during the user's continuing slow Claude reconnect/tool-fetch failure.

The connector changes outgoing request IDs but must restore their references in subscription acknowledgements/change notices/final results. Modern stdio cancellation must close only the matching owned HTTP request/stream, preserving legacy cancellation and all other connections. No new API, authentication, permission, notification-acceptance policy or startup timeout.

Status: source research/planning complete. Root enables async-read/write transport only for the existing rmcp3.8.0 dev dependency. Independent Tester first reproduces with that maintained client; Coder waits for red-test evidence before applying the bounded fix.

Acceptance: maintained modern SDK can establish/listen/change/list/call/cancel and continue through stdio and pinned HTTPS; equal original IDs across connectors remain separate; existing legacy cancellation/progress/interactions pass. Signed build-only and independent review required. Actual Claude/Codex remain user-manual. Preserve unrelated server-first/Workspace/Off/UI work and personal state.

RED established before code: maintainedSDK acknowledgement timed out3.03seconds. Bounded source fix integrated; all3regressiongroups authored. GREEN/all19and signed build/final review pending. Legacy token cleanup verified in actual final source.

Runtime checks now GREEN: maintainedSDK completefixture0.03seconds, all19connectorchecks0.39seconds. This is isolated fixture timing, not actualClaude latency. Signed build/finalreview pending.

All19connectorchecks pass. Final signed build currently waits on concurrent project-support compile interfaces; root added only its missing data-module export and coordinated owning task. No project API/auth decision inferred. Do not claim new executable ready until integrated build passes.

## Final scoped result

Confirmed modern subscription correlation and cancellation defects are fixed. Maintained rmcp SDK reproduces acknowledgement timeout before the fix (3.03seconds), then completes the isolated startup/subscribe/change/call/cancel/list/call sequence in0.03seconds. Final integrated19connectorchecks pass0.30seconds. Signed development build-only22.53seconds, repeated strict signature verification, scoped whitespace and independent Evaluator ACCEPT pass. Actual installed Claude/Codex use remains manual and is not claimed successful yet. Fetch and clients were not relaunched; user should explicitly Quit/reopen Fetch and Reconnect Claude. No personal settings/credentials/trust/services or project API/auth decisions changed.

The build briefly caught concurrent project core changes before their Caller interfaces were complete. Root added only the missing projects data-module declaration; its owning task completed internal Caller wiring and then froze interfaces for the successful build. No unrelated code was reverted and no project route/auth contract was approved by this pass.

[Evidence/manual steps](evidence.md). Earlier pending-build entries above are historical.
