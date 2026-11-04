> **November 3 current implementation:** User resumed the scoped refactor.
> Official Rust MCP login/renewal, direct Keychain storage, native-owned credentials,
> retained manual-key PATCH and shared authenticate-route callers are implemented.
> Initial141 fixture checks and native build pass; final review/real confined MCP
> verification are underway. Native setup/folder metadata authoring and real
> provider/Finder/Keychain acceptance remain limits. See [the current plan](native-server-storage.md).

> **November 3 latest decision:** Use the official Rust MCP library for shared
> online login/renewal, preserving native server-owned login and raw forwarding.
> Exact remaining Fetch work and open details are in [the updated plan](native-server-storage.md).
> Further application edits/builds/tests remain paused at the user's request.

> **November 3 latest checkpoint:** Keychain remote-client OAuth is selected.
> Further source edits/builds/tests are paused at the user's request for the full
> change plan. Partial source edits are incomplete and untested. Discuss any
> individual MCP/provider-specific code before writing it. See
> [the full route/change plan](native-server-storage.md).

> **D81 active local work:** Implementation/build/test and protected actual MCP/client checks are authorized; packaging/signing/DMG are excluded. Native build and93 fixtures pass after the October 30 parallel-authentication fix; real UI/provider/MCP proof is still pending. Two focused choices remain unanswered: deletion/non-auth credential-change outcomes under D31, and bounded protected-MCP authentication proof when no separate provider account API exists. Ordinary supported API-key saves retain their required authenticated check. Preserve D25/D31 until answered. See [the current run](../testing/LOCAL_MVP_RUN.md). Older execution-pause paragraphs below are historical.

## November 3 — HTTP handlers and backend folders

Confirmed user direction: `src/api` should contain the raw HTTP route handlers,
with subfolders following URL paths. Separate backend folders are acceptable;
they hold the reusable functions called by those handlers. For example, a handler
for `/api/v1/servers/{server_id}/restart` belongs under the matching servers,
server-ID and restart path, while the restart implementation belongs in the
server-management backend. Route handlers read requests, check request-level
requirements, call shared backend functions and form HTTP responses. Backend
rules and caller permissions must remain shared with the MCP interface.

Current implementation differs: `api/mod.rs` registers most application paths
and implements server listing; `api/setup/mod.rs` handles discovery;
`api/runtime.rs` handles installation, configuration, authentication, restart,
tools and the provider callback. `oauth/mod.rs` and `mcp/mod.rs` also register
HTTP endpoints. These are current facts, not the requested finished layout.
A future refactor must cover those endpoints too, preserve existing URLs and
behavior, and distinguish HTTP adapters from OAuth/MCP implementation logic.
Exact Rust naming for variable and punctuation-containing path segments is an
implementation detail, not a change to public routes. No source was moved in
this explanation pass.

> **October29 current handoff:** five required implementation areas plus isolated
> runtime acceptance/fixes and distribution preparation remain. See
> [MVP_GAPS.md](../testing/MVP_GAPS.md). Railway/auth/logging source compiles;
> no live acceptance follows. Older D77/D79 checkpoint text below is historical.

> **D79 current checkpoint:** Native binary compilation succeeds with the resolved Cargo.lock after a gateway syntax/import fix. Tests, app launch and MCP connections remain prohibited. No runtime/client/native acceptance is claimed. See [the handoff](../testing/HANDOFF.md). Earlier D77 no-compilation statements describe that historical source-only pass.

> **Historical D77/D78 checkpoint:** Selected MVP source is written through [the user-controlled testing handoff](../testing/HANDOFF.md). No current-pass builds, checks, tests, launches, connections or packaging ran. Custom GitHub CLI tools are deferred. Older foundation verification below is historical.

# Fetch architecture records

October30 D82: generic imported credential inputs and trusted descriptions now
drive the approved authentication refactor. Storage/delivery is shared; a trusted
description specifies each check's inputs, request and success condition for one
generic executor. D25/D31 proof remains required. The internal change adds no new
public route/config schema or inferred unknown-provider login. Current progress:
[generic authentication](../../milestones/generic-authentication/info.md).

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
| Shared authentication | [Internal engine and trusted descriptions](authentication-engine.md), D82; imported inputs stay generic and D25/D31 proof is retained. |
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

## Development Keychain prompt investigation

[Repeated prompts](keychain-prompts.md) records read-only findings, ad-hoc
rebuild identity, duplicate key reads, traditional-store UI limitations and
unselected stable-signing/cache/wrapping-key options. No personal Keychain or
signing/storage change was made.
