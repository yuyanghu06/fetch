# Fetch architecture records

Updated October28, 2026 after D73. Design/research/documentation only; application implementation remains paused.

## Read before continuing

Start with [PROJECT_CONTEXT.md](../../PROJECT_CONTEXT.md), [NOTES.md](../../NOTES.md) and [confirmed decisions](decisions.md). The **single current plan** is [build-readiness.md](build-readiness.md). [Open questions](open-questions.md) is a short index to its one product-scope question and engineering worklist, not a second approval backlog. Do not revive the superseded ten-group count or old checkpoints.

## Current direction

One local Rust/macOS app; one saved Fetch connection/token per agent at `/mcp`; user-completed provider logins reused by tool calls; selected raw data/transports/native scope/queues/deadlines/restart; encrypted per-server files and Keychain keys; native Settings manual secrets. No shared memory in this release. MIT license and drag-and-drop DMG are selected. No implementation, real client/credential/Keychain/certificate changes, provisioning, live tests or publishing is authorized by these records.

## Contract and evidence references

| Area | Reference / status |
| --- | --- |
| All product decisions | [decisions.md](decisions.md), latest D73. |
| Existing APIs | [api-contract.md](api-contract.md); exact remaining engineering work is gathered once in the implementation checkpoint. |
| App connection | [client-authorization.md](client-authorization.md), D71 presented scope approved; [HTTPS/certificates](local-oauth-trust.md), D70. |
| Settings/secrets | [storage](storage-scope.md), [shown D72 direction and unshown mechanism candidates](secret-storage-settings.md). |
| Tool/native/runtime | [execution](mcp-execution.md), [connection](mcp-connection.md), [management tools](mcp-management-tools.md), [resources/prompts](mcp-resources-prompts.md). |
| Remote/raw/catalog | [remote amendment](remote-api-amendment.md), [protocol adapters](mcp-protocol-adapters.md), [catalog](tool-catalog-status.md), [names](mcp-tool-names.md), [presentation](mcp-presentation.md), [interaction relay](mcp-interaction-relay.md). |
| Integration/research | [GitHub CLI open scope](github-cli-proposal.md), [real-use inventory](real-use-compatibility.md), [research](research.md). No source review establishes live compatibility. |
| Unaccepted historical draft | [credential verification amendment](credential-verification.md), withdrawn from active review by D73; no storage-only200 exception adopted. |
| Actual work | [milestone snapshot](../../milestones/milestone.md), [live to-do](../../milestones/to-do.md) and immutable pass history. |

## Original-session MCP audit

[The source audit](mcp-session-audit.md) covers all seven available Fetch conversations, including the archived workflow thread. It restores already-selected acceptance details and the D68 config-tool execution fields without a new approval loop. Source/approval limits and protection checks are recorded there; the single implementation checkpoint remains the only live plan.

## Maintaining these records

Update decisions as the user answers. Present a new API/meaningful behavior choice completely once; preserve approved behavior. Keep engineering work in the one worklist, not new lists of broad questions. Mark proposed, selected, superseded and actually tested evidence accurately. Preserve secrets/personal state and use D46 isolation for authorized live tests. Historical handoffs and research remain references, never authority to start building.

## Authorized developer backend implementation

D74 starts implementation/push without release. [The first bounded engineering brief](developer-foundation.md) records the researched dependencies, actual developer-only mechanisms and verification limits. Live managed-MCP tests remain for a later joint isolated environment.
