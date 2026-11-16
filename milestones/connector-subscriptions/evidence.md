# Connector subscriptions — evidence

Current status: confirmed transport defects corrected; independent Evaluator ACCEPT for source, maintained SDK regression, all19integrated connector checks and signed development build. New executable is ready for user-manual retry. Actual live Claude/Codex success has not yet been observed.

## What was wrong

The connector changes request IDs to keep simultaneous connections separate. It failed to restore those IDs in protocol-owned subscription acknowledgements and notices. The maintained modern client waits for an acknowledgement belonging to its original ID, so the wrong ID leaves it waiting.

Modern stdio cancellation was also being forwarded as a notification to the HTTP backend. Modern HTTP cancellation requires dropping only that request's response stream. The invalid notification can instead cause a backend refusal and close unrelated outstanding discovery in the connector.

The fix restores only reserved subscription IDs using this connector's active subscription map and closes only the cancelled request/stream. ACKs retain mappings; terminal replies retire them. Tool data, progress tokens, global backend lifetime, legacy cancellation dispatch ordering and authentication are unchanged.

## Reproduction and checks

- Before source fix: official rmcp3.8.0 client over production stdio bridge + owned pinned HTTPS timed out waiting for subscription acknowledgement (test exit101,3.03seconds).
- After source fix: that same client completes startup, subscribe, notification, generic tool call, cancel and further list/call in0.03seconds. This is fixture timing, not measured actual Claude latency.
- All19connector checks pass in0.39seconds. Includes two same-profile connections both using original ID7, isolated cancellation, protocol-only nested metadata changes, raw results/errors, legacy progress/interactions/immediate cancellation, reconnect/restart/no replay and approval-profile handling.
- Final integrated connector suite passes19tests in0.30seconds after the shared project interfaces were completed.
- Signed ./scripts/run-development.sh --build-only passes22.53seconds; repeated codesign --verify --strict confirms valid executable/designated requirement.
- Source/whitespace checks and final independent Evaluator ACCEPT pass.

Logs: /tmp/fetch-subscription-sdk-red.log, /tmp/fetch-subscription-sdk-green.log, /tmp/fetch-subscription-all-tests.log, /tmp/fetch-subscription-signed-build.log.

Tests reuse the existing official rmcp dependency with a test-only async-read/write transport feature. No installed Claude/Codex, personal settings, credentials, Keychain, system trust, provider account or existing service was used or changed. The live user's reconnect process was sampled read-only and the existing public Fetch metadata endpoint measured200/0.003289seconds; sample showed normal Runtime::block_on/stdin, not shutdown. Task-owned raw samples removed.

## Shared-build issue

Concurrent project core changes entered storage/manager while this task tested. Root added only a missing pub mod projects declaration for the existing data registry; no project route/auth/authority semantics added. The subsequent build needs the owning task's Caller::project_id interface. The existing project/Workspace task was contacted and completed only the necessary internal Caller interfaces, with no new route/auth behavior. Retry build and integrated19checks pass. Its separate pending route/auth review remains separate; this task does not approve it. Running apps and user state were preserved; the signed executable was atomically replaced only after successful build/signature verification.

Actual Claude/Codex success remains user-manual. The source defects and SDK reproduction are concrete; the exact live Claude request sequence has not been captured, so do not claim the particular session failure is proven solved before retry.

## Manual next action

Explicitly Quit Fetch, reopen the signed development executable, then use Reconnect in Claude. Reuse existing stdio registration and approved profile; no reinstall or configuration edit is needed for this build. Record actual connection/tool outcome before claiming the live issue resolved.
