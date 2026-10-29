> **D81 active local work:** Implementation/build/test and protected actual MCP/client checks are authorized; packaging/signing/DMG are excluded. Native build and68 fixtures pass; real UI/provider/MCP proof is still pending. Two focused choices remain unanswered: storage success versus authenticated status for credential operations, and bounded protected-MCP authentication proof when no separate provider account API exists. Preserve D25/D31 until answered. See [the current run](../testing/LOCAL_MVP_RUN.md). Older execution-pause paragraphs below are historical.

> **October29 current handoff:** five required implementation areas plus isolated
> runtime acceptance/fixes and distribution preparation remain. See
> [MVP_GAPS.md](../testing/MVP_GAPS.md). Railway/auth/logging source compiles;
> no live acceptance follows. Older D77/D79 checkpoint text below is historical.

> **D79 current checkpoint:** Native binary compilation succeeds with the resolved Cargo.lock after a gateway syntax/import fix. Tests, app launch and MCP connections remain prohibited. No runtime/client/native acceptance is claimed. See [the handoff](../testing/HANDOFF.md). Earlier D77 no-compilation statements describe that historical source-only pass.

> **Historical D77/D78 checkpoint:** Selected MVP source is written through [the user-controlled testing handoff](../testing/HANDOFF.md). No current-pass builds, checks, tests, launches, connections or packaging ran. Custom GitHub CLI tools are deferred. Older foundation verification below is historical.

# Fetch architecture records

Updated October28, 2026 after D74. The first Rust developer backend is implemented, tested and pushed. The desktop interface, native app authorization, package installation and MCP sharing are unfinished. Current default: `https://localhost:53671`; no release or live managed-MCP testing.

## Read before continuing

Start with [PROJECT_CONTEXT.md](../../PROJECT_CONTEXT.md), [NOTES.md](../../NOTES.md) and [confirmed decisions](decisions.md). The **single current plan** is [build-readiness.md](build-readiness.md). [Open questions](open-questions.md) is a short index to its one product-scope question and engineering worklist, not a second approval backlog. Do not revive the superseded ten-group count or old checkpoints.

## Selected design recorded before D74

One local Rust/macOS app; one saved Fetch connection/token per agent at `/mcp`; user-completed provider logins reused by tool calls; selected raw data/transports/native scope/queues/deadlines/restart; encrypted per-server files and Keychain keys; native Settings manual secrets. No shared memory in this release. MIT license and drag-and-drop DMG are selected. No implementation, real client/credential/Keychain/certificate changes, provisioning, live tests or publishing is authorized by these records.

## Contract and evidence references

| Area | Reference / status |
| --- | --- |
| All product decisions | [decisions.md](decisions.md), D74 authorizes building/pushing; the current port follow-up is recorded there. |
| Existing APIs | [api-contract.md](api-contract.md); exact remaining engineering work is gathered once in the implementation checkpoint. |
| App connection | [client-authorization.md](client-authorization.md), D71 presented scope approved; [HTTPS/certificates](local-oauth-trust.md), D70. |
| Settings/secrets | [storage](storage-scope.md), [shown D72 direction and unshown mechanism candidates](secret-storage-settings.md). |
| Tool/native/runtime | [execution](mcp-execution.md), [connection](mcp-connection.md), [management tools](mcp-management-tools.md), [resources/prompts](mcp-resources-prompts.md). |
| Remote/raw/catalog | [remote amendment](remote-api-amendment.md), [protocol adapters](mcp-protocol-adapters.md), [catalog](tool-catalog-status.md), [names](mcp-tool-names.md), [presentation](mcp-presentation.md), [interaction relay](mcp-interaction-relay.md). |
| Integration/research | [GitHub CLI open scope](github-cli-proposal.md), [real-use inventory](real-use-compatibility.md), [research](research.md). No source review establishes live compatibility. |
| Isolated testing | [Short batch/proposed layout](isolated-testing.md), [curated five-server roster](../../milestones/isolated-mcp-testing/servers.md): Everything, Filesystem, Time, Tavily and Linear. Layout/access setup remains proposed; all checks not run. |
| Unaccepted historical draft | [credential verification amendment](credential-verification.md), withdrawn from active review by D73; no storage-only200 exception adopted. |
| Actual work | [milestone snapshot](../../milestones/milestone.md), [live to-do](../../milestones/to-do.md) and immutable pass history. |

## Original-session MCP audit

[The source audit](mcp-session-audit.md) covers all seven available Fetch conversations, including the archived workflow thread. It restores already-selected acceptance details and the D68 config-tool execution fields without a new approval loop. Source/approval limits and protection checks are recorded there; the single implementation checkpoint remains the only live plan.

## Maintaining these records

Update decisions as the user answers. Present a new API/meaningful behavior choice completely once; preserve approved behavior. Keep engineering work in the one worklist, not new lists of broad questions. Mark proposed, selected, superseded and actually tested evidence accurately. Preserve secrets/personal state and use D46 isolation for authorized live tests. Historical handoffs and research remain references, never authority to start building.

## Authorized developer backend implementation

D74 starts implementation/push without release. [The first bounded engineering brief](developer-foundation.md) records the researched dependencies, actual developer-only mechanisms and verification limits. Live managed-MCP tests remain for a later joint isolated environment.

## D80 Railway callback exception and current completeness

[D80](decisions.md#d80--temporary-numeric-loopback-callback-for-railway-sign-in)
selects a temporary numeric-loopback HTTP listener for Railway desktop sign-in;
Fetch's product API remains HTTPS. Scoped Railway/auth/logging source now
compiles, but actual native/provider/client proof remains unrun. See
[known required source gaps](../testing/MVP_GAPS.md); the full selected MVP is
not complete merely because its current source compiles.
