> **Current authorization — October28, 2026:** The user authorized starting implementation and pushing to `https://github.com/yuyanghu06/fetch.git`. Do not create a release. Main MCP connection tests are deferred to a later isolated virtual environment using the user’s server inventory. This supersedes earlier implementation/push pauses only; it does not authorize personal-client edits, certificate trust changes, credential migration or live MCP tests.

# Current implementation checkpoint

October28, 2026. D73 replaces the ten-group approval backlog with [one consolidated implementation plan](build-readiness.md). The earlier count mixed approved product behavior, engineering work and later release/testing prerequisites; it is superseded, not a fresh requirement list.

## Remaining decisions checklist

This is the single-checkpoint index after D73; the older ten-group count is superseded. Use the implementation worklist for engineering details.

## Settled — do not ask again

- One saved Fetch credential/token per agent, one `/mcp` connection for all permitted installed tools; selected OAuth renewal and permissions, no Fetch account.
- User-completed provider/server login reused for later calls. Normal tool calls do not start login or repeat provider verification.
- Rust backend, one Fetch process, approved raw data/transport/native scope and queue/deadline/restart behavior.
- Local HTTPS, per-install certificate direction, encrypted per-server credentials, Keychain-held keys and native Settings manual-secret boundary.
- Shared memory deferred from this first release.
- MIT license and a drag-and-drop macOS DMG.

[Confirmed decisions](decisions.md), especially D73 and its referenced decisions, remain canonical. This index does not enlarge old approval scopes.

## Product-scope choice resolved — D78

The user selected deferral of the new Fetch-owned GitHub CLI adapter in D78; importing existing GitHub MCP definitions remains a separate supported-compatibility task. If included, decide named operations and auth ownership together. [GitHub proposal](github-cli-proposal.md).

## Engineering work, gathered once

The [single worklist](build-readiness.md#one-engineering-worklist--not-separate-product-approval-rounds) contains all remaining module/dependency/wire/file/key/launcher/destination/bounds/lifecycle/onboarding/testing/packaging work. Exact unselected mechanisms remain honest engineering tasks, not another broad auth/storage/queue/Rust approval loop. Any meaningful new API/behavior change is presented once, completely, before affected implementation.

The unaccepted [verification amendment](credential-verification.md) is withdrawn from active review. No new storage-only200 exception or policy in it is approved; preserve D25/D31 until an explicit amendment is selected. Native read/prompt admission and precise mechanisms remain worklist items, not falsely claimed historical approvals.

## Historical D73 authorization and future scope

Application implementation, environment provisioning, real Keychain/certificate/credentials/client changes, live testing and release remain paused until explicitly authorized. D46 isolation and D58 actual workflows remain acceptance requirements. Shared memory remains future same-product work; PostgreSQL/retention, graph, extra processes and recorded native-feature deferrals are not current build blockers.

Historical detailed proposals and append-only history preserve evidence/chronology. They must not be used to recreate the superseded ten-group list.

## Source-history audit

[The MCP session audit](mcp-session-audit.md) reconciles all seven available Fetch conversations. No new product decision or build permission was found; already-selected missing mapping/acceptance details were restored to the existing plan. It does not add another backlog or re-open deferred feature work.

## Current implementation status after D74

The first developer-backend foundation is implemented and pushed; the developer command defaults to `https://localhost:53671`. Native approval/token issuance, lasting identity/Keychain, package installation, desktop UI and MCP sharing remain unfinished. Build/push authorization is already granted; the old D73 pause above is historical. Continue the existing build sequence without requesting that permission again. Main MCP tests remain for the later joint isolated environment. No release is authorized.
