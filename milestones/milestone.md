# Fetch milestone snapshot

Updated October 28, 2026. Active checkpoint: **selected MCP MVP source written; user-controlled testing has not started**. [Source milestone](mcp-mvp-implementation/info.md). [Testing handoff](../docs/testing/HANDOFF.md).

## Goal and current state

Complete the D73 selected local Rust/macOS MCP first-release source under D77. Authority/Keychain/lasting HTTPS, private runtime/package installation, server management/auth/restart, the three-transport raw runner/native MCP surface, native API-backed Iced interface and app/DMG construction source are implemented. Scoped feature commits are pushed to origin/main as integrated, through source refinement e576bcc; final handoff/distribution-source documentation is delivered in its own commit. No release or DMG is produced. D78 defers custom GitHub CLI tools; existing MCP imports remain in scope.

## Decisions and boundaries

The [selected specification](../docs/architecture/build-readiness.md), [confirmed decisions](../docs/architecture/decisions.md) and [engineering brief](../docs/architecture/mvp-implementation.md) govern. UI/API/manager stay in one process; shared backend handlers own authority, records and lifecycle. Raw provider JSON is preserved. Native Settings alone submits manual secret values; stored-credential success still requires its authenticated provider check. Ordinary tool calls do not start provider login. Durable configs/credentials stay outside replaceable packages.

Source supports conservative launch/remote endpoint profiles, documented provider verifiers/owned Linear login, raw catalogs and selected native methods. Unknown integrations remain truthful unknown/unsupported branches. Exact limitations and first verification priorities are in the handoff. Shared memory, databases/retention, graph, extra processes, native background Tasks/live resource subscriptions and future project/session organization remain deferred.

## Evidence and limitations

Actual current-pass evidence is **source reading only**, including bounded independent source reviews and corrections. No build, compile check, dependency resolution/lockfile refresh, formatter/linter, test, validation script, app/service, OS/Keychain/trust operation, runtime/package install, provider/MCP/client connection or packaging ran. The user expressly controls starting every such step. Source review acceptance is not runtime/visual/client acceptance. Prior foundation37-test evidence applies only to prior recorded commits.

Cargo.lock remains from the earlier foundation. The new source is uncompiled; first user-started compilation may require fixes. Actual macOS native authority/Keychain/lifecycle, HTTPS/client behavior, import/provisioning/verification support and both protocol revisions remain unverified. Personal configuration/credentials/trust/apps/services are unchanged.

## Next action

Stop at this handoff. The user chooses when to resolve dependencies/compile, finish the isolated environment/account boundaries and run the short roster (Everything, Filesystem, Time, Tavily, Linear), followed by broader native/client acceptance. All roster checks remain not run. The proposed testing folder is not provisioned or a sufficient isolation boundary by itself. Publication is not authorized.

## Source pass history

- [orchestrator-full-mvp-source-authorization](history/2026-10-28_13-49-03-679662-orchestrator-full-mvp-source-authorization.md)
- [coder-durable-app-authorization-source](history/2026-10-28_13-55-35-037181-coder-durable-app-authorization-source.md)
- [evaluator-initial-installation-manager-source-review](history/2026-10-28_14-19-13-498104-evaluator-initial-installation-manager-source-review.md)
- [coder-private-installation-source](history/2026-10-28_14-19-13-498851-coder-private-installation-source.md)
- [coder-raw-runner-and-native-mcp-source](history/2026-10-28_14-20-16-098752-coder-raw-runner-and-native-mcp-source.md)
- [evaluator-final-installation-manager-source-review](history/2026-10-28_14-21-26-711572-evaluator-final-installation-manager-source-review.md)
- [coder-server-management-source](history/2026-10-28_14-21-26-711804-coder-server-management-source.md)
- [evaluator-parent-integration-source-review](history/2026-10-28_14-26-10-790953-evaluator-parent-integration-source-review.md)
- [orchestrator-native-api-ui-source-integration](history/2026-10-28_14-26-10-791201-orchestrator-native-api-ui-source-integration.md)
- [orchestrator-source-handoff-and-packaging](history/2026-10-28_14-27-47-778855-orchestrator-source-handoff-and-packaging.md)

- [Timeout/input source refinement](history/2026-10-28_14-33-43-779186-orchestrator-timeout-and-empty-body-source-refinement.md)

- [Final bounded timeout/input source review](history/2026-10-28_14-34-29-149128-evaluator-final-timeout-and-handoff-source-review.md)

## Prior records

[Earlier rolling snapshot](milestone-before-d77-source.md), [earlier live task record](to-do-before-d77-source.md), [foundation](mcp-dev-foundation/info.md), [architecture handoff](mcp-mvp-architecture/handoff.md) and [test planning](isolated-mcp-testing/info.md) preserve chronology/evidence. Immutable history remains append-only.
