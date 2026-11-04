# Direct install native build

- Local timestamp / acting role / milestone: 2026-11-03T18:10:43.880970-05:00 / orchestrator, inspected by Orchestrator / direct-install-inputs.
- Requested scope: Implement/test/evaluate direct command/url POST body, no UI.
- Starting repository state: Source implemented after approved contract; unrelated UI/startup/docs changes preserved; Google pending.
- Work performed / changed files: Inspected final source/test diff and ran env DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo build --locked --bin fetch --message-format short. Exit0,0.35s; existing five macOS Security deprecation warnings and unread MCP error-field warning remain. No new direct-input warning. No native binary launch. Source/docs changes are scoped to API/backend/tool schema plus parser dependency and evidence; prior unrelated UI/startup edits remain outside the feature.
- Decisions: No optional behavior or new public route/UI. Retain selected shape/reuse/error and credential boundaries.
- Checks and evidence: Actual results above; disposable fixtures only. No real provider/helper/MCP client/native/Keychain/personal-state or release action.
- Resulting state / acceptance: Scoped backend extension accepted; evidence separates tests/build from live installation/connection.
- Remaining work and handoff: Scoped commit/push and user example; broader provider/native/client acceptance and Google remain pending.
