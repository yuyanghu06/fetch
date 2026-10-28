> **Current authorization — October28, 2026:** The user authorized starting implementation and pushing to `https://github.com/yuyanghu06/fetch.git`. Do not create a release. Main MCP connection tests are deferred to a later isolated virtual environment using the user’s server inventory. This supersedes earlier implementation/push pauses only; it does not authorize personal-client edits, certificate trust changes, credential migration or live MCP tests.

# Fetch MCP first-release implementation checkpoint

October28, 2026 — consolidated by D73 after the user requested an end to repeated approval loops. This is the **single current plan**. [Confirmed decisions](decisions.md) govern behavior; [research](research.md) supports engineering choices. Historical estimates/checklists are not current blockers.

## Locked release specification

| Area | Selected behavior |
| --- | --- |
| Product | One local macOS Fetch app with a Rust backend, UI/API/server manager in one process. Shared memory is deferred from this first release; it remains future same-product scope. |
| Agent connection | One Fetch MCP entry per agent at `/mcp`; the agent stores its own Fetch credential/token and uses the selected automatic renewal. No Fetch account. Existing approved permissions remain; this does not create a static-token alternative. |
| Provider access | The user completes each installed server/provider's own authentication when needed. Fetch reuses valid assigned credentials. Tool calls do not open login or demand provider credentials again. Expired/missing/insufficient provider access gets truthful setup guidance. |
| APIs | API-first backend authority; existing selected setup/server/tool/authorization operations and descriptive management tools. Manual secrets remain native Settings-only. Every application JSON response keeps next_steps; UI follows structured state. |
| Raw data | Preserve original Tool JSON, model argument objects and upstream results with only selected protocol adaptations. Routing stays outside arguments. |
| Transports/native methods | Selected stdio/HTTP/SSE upstream adapters, local HTTPS Streamable HTTP endpoint, both reviewed revisions, native relays/resources/prompts/completion and recorded deferrals. |
| Execution | Shared server state; approved queues, concurrency, deadlines, cancellation/uncertainty, restart/reconnect and one managed process/connection per logical server. Do not reopen D68. |
| Saved state | Durable non-secret local records and per-server encrypted credential files outside replaceable packages; Keychain-held keys and approved native human authority direction. No PostgreSQL or substitute database requirement. |
| Distribution | MIT license; drag-and-drop DMG containing Fetch.app and an Applications shortcut. Manual/explicit changes, no automatic personal-client cleanup or implicit publishing. |
| Acceptance | Real Claude Code/OpenCode workflows plus Codex, independent test/evaluation and D46 isolation protecting personal state. A build alone is insufficient. |

## MCP acceptance details restored by the session audit

The [full-session MCP audit](mcp-session-audit.md) recovered these already-selected requirements from original conversations. They are implementation acceptance checks, not new decisions or another review backlog.

| Selected requirement | Concrete implementation and acceptance check |
| --- | --- |
| Install once / run once | Multiple agents/chats and additional Fetch launches must not duplicate a managed connector/account. Keep one managed process/connection per logical server; do not merge different accounts merely because they share a package. Exact single-instance launch coordination is engineering work. D02/D57. |
| Private runtimes supplied by Fetch setup | Supply Fetch-managed Node/npm/npx, Python and uv/uvx as part of Fetch installation/setup. Do not depend on or modify global installations. Prove setup on a fresh isolated Mac without those global tools. DMG drag/drop alone does not prove runtime setup; bundled versus setup-time provisioning remains engineering work. D06. |
| Runtime/package/data separation | Use Application Support/Fetch/packages/runtimes/{node,python,uv}/<version>/ for compatible shared runtimes and packages/servers/<installation>/ for isolated server dependencies. Keep durable server data outside packages under servers/<server-id>/. Existing local scripts execute in place rather than being copied. D05/D06/D28. |
| Version pinning | Honor the requested version; resolve omitted/latest at installation, retain that resolved selection, and do not update on Fetch reopen/startup. One server's preparation must not alter another's dependency environment. Do not invent an updater API. D07/D34. |
| Discovery and row controls | Supported rows default selected, except explicitly source-disabled rows require opt-in. Unsupported/invalid rows cannot be selected. Provide selection toggles and an Install button; installation validates again. After preparation, show Authenticate for rows requiring auth, with truthful manual/native/unknown branches. No discovery-time startup/login or credential migration. D04/D08/D32/D34/D35. |
| Source-independent installation | Retain enough saved non-secret metadata and references for an installed server to work after its original fixture source entry is removed. Reopen Fetch and call its tool after cleanup; local scripts still depend on their original file path. Preserve successful installs when others fail; retry failed entries only. D05/D13/D14/D34/D36. |
| User-pasted cleanup prompt | Deliver the copyable prompt the user pastes into their agent to remove only the migrated original entries after successful connection/auth/tool checks. Preserve unrelated entries and the Fetch connection. Fetch does not automatically delete personal configurations; tests use disposable fixtures. D12. |
| Restart control and named progress | Show Restart only when applicable; an explicit click invokes the selected POST. Config PATCH alone does not restart. Keep loading until that restart_id's succeeded/SERVICE_STARTED or failed outcome;202 is acceptance, not completion. Preserve normal/force drain and remote reconnect uncertainty rules. D15/D20/D63/D68. |
| Timeout installation and dispatch | Initialize durable per-server/tool settings without install-time server startup/login. Use a supported explicit source default or120000ms fallback, preserve user settings on reuse, and compute a fresh execution deadline at dispatch separately from queue wait. Do not invent Tool metadata or alter arguments. D52–D55/D68. |
| Matching config management tool | fetch__update_server_config exposes the approved execution.max_concurrent_calls/default_timeout_ms/tool_timeouts_ms alongside launch/remote fields. Same API permissions, per-tool visibility and hot/atomic rules; no secret-value input. D49/D63/D68. |
| Streaming defaults | Tool calls stream available progress and the unchanged final result automatically; quick reads use JSON. No user-facing streaming switch or invented partial content. D60. |
| Partial rejection versus incomplete collection | A known rejected definition can leave19 valid tools usable with explicit partial status and safe reason. Failed/truncated page traversal is not advertised as a complete catalog. Preserve unknown/raw Tool data and source-versus-export distinctions; model visibility must be proved. D37/D38/D64/D65. |
| UI and agent guidance agree | Test the same prerequisites, order, completion/failure and retry actions through structured UI state and next_steps. UI never parses guidance text as control logic; human-only handoffs/secret boundaries stay enforced. D22/D32/D33. |
| Native UI and process scope | Provide server/tool inspection through the approved catalog API, retaining the requested expandable server/tool presentation without restoring deferred extra-process controls. Try the selected Iced direction with the Rust backend; Electron was rejected. One Fetch process contains UI/API/manager, wrappers remain children, and background/window-close versus Quit follows selected lifecycle. No MCP config version-history/rollback feature. D01/D02 and original framework discussion. |
| Actual acceptance inventory | Use the already recorded Claude/Codex server metadata and real Claude Code/OpenCode plus Codex workflows; do not ask the user to relist servers. Verify actual native/transport/owner/media/cancellation/restart behavior in D46 isolation, not merely a build or mocked provider login. D46/D58/D66/D67. |

## One build sequence

| Stage | Complete work | Evidence needed before moving on |
| --- | --- | --- |
| 1. Rust foundation and authority | Establish the single app/backend boundaries, shared handlers, local HTTPS, selected app-token flow, local records and protected store. Resolve the engineering worklist below together using completed research. | Real HTTP authorization, storage/readback, permission-denial and error checks in the approved isolated environment before corresponding UI. |
| 2. Server management | Implement selected discovery/install/progress/server/config/credential/auth/restart operations, private Node/npm/npx/Python/uv/uvx runtime provisioning, isolated pinned package preparation and fresh provider setup. | Actual HTTP flows, source/reuse/partial failure, no credential migration, verified required auth, saved-versus-applied state and normal/forced restart. |
| 3. MCP runner and sharing | Implement three upstream transports, catalogs/aliases/raw calls, selected native messages/resources/prompts and existing scheduling. | First HTTP tool calls, then `/mcp`; ownership, raw fidelity, shared processes, queues, deadlines, cancellation and unknown outcomes across clients. |
| 4. Native interface and connection setup | Wrap working APIs with installation/status/Settings/reconnect screens; connect each agent once to Fetch and guide per-server authentication only when needed. | UI-only setup with selection/install/auth/conditional-restart controls, ordinary-agent secret-write denial, user-completed provider flows, one saved agent connection and the user-pasted cleanup prompt only after verification. |
| 5. Isolated acceptance and release | Independent daily-workflow/fresh-install tests, fixes/retests/evaluator acceptance, MIT license file and DMG packaging. | Supported clients/providers actually work; app lifecycle and saved data survive approved updates; release signing/notarization/access and publication authorized when required. |

D74 authorizes source implementation and pushes without release. The first developer-backend foundation is implemented; it does not complete Stage1. Environment provisioning, trust/personal-client changes and live managed-MCP tests remain outside this authorization. Each included new route or meaningful behavior change must be presented completely before implementation; do not reopen selected broad decisions.

## One engineering worklist — not separate product-approval rounds

Resolve these together in the implementation brief before the affected code. Distinguish selected requirements, chosen engineering solutions and genuine compatibility gaps. Exact unchosen details are not reclassified as approved.

| Engineering area | What must be made concrete / preserved |
| --- | --- |
| Rust/Cargo/dependencies | Module/package boundaries, HTTP/async/TLS/native UI/MCP/serialization/crypto libraries and compatible versions. Rust and the one-process design are fixed. Preserve existing source-grouping decisions; validate fidelity rather than trusting SDK defaults. |
| Raw protocol and selectors | Exact aliases/resource URI codecs, request/continuation ownership, revision framing/diagnostics and raw unknown-field/number/media preservation. Respect selected narrow marker/catalog exceptions. |
| Native admission and bounds | Classify remaining resource/prompt/completion admission consistently with approved server capacity/deadlines; implement finite body/result/metadata/queue limits and execution profile/GET encodings. No new broad queue review. |
| Config/import/packages | Supported global formats/paths/launcher grammar; safe source/server/install identity; private pinned npm/uv preparation; finite preparation deadlines, staging/cache/integrity and owned-process cleanup. |
| Files/keys/native authority | Exact filenames/envelope/key mechanism, classic versus data-protection Keychain/build identity, atomic writes/readback, unlock/key-loss behavior and native Settings bootstrap. [Settings mechanism candidates](secret-storage-settings.md) remain reference proposals, not unseen approvals. |
| Provider integration | Per-source ownership, private credential handoff, authenticated verification, refresh and restart behavior. Reuse existing login/token decisions; do not promise universal wrapper token export, infer auth from readiness or repeat checks on normal tool calls. |
| Verification error edges | Preserve D25/D31 success gate. [The unaccepted verification draft](credential-verification.md) is reference only; its timeout/status/retention/preflight/storage-only exception is not selected. Resolve concrete requirements once in the implementation brief without inventing unchecked success or another broad login review. |
| Remote destinations | Fixed endpoint identity, header references, permitted destinations/redirects/TLS and bounded metadata/network behavior. Never replay uncertain actions or forward credentials to guessed targets. |
| Lifecycle/recovery | Startup/bind conflicts, background/Quit/reopen, child cleanup/crash handling, state-generation checks and selected same-run status lifetime; no unsupported recovery/history promise. |
| Onboarding/management | First trusted native setup, three-client connection mechanics, permission-based handoffs and verified cleanup. Implement only selected controls. Human connection administration, custom update/remove/start/stop APIs are not silently added. |
| Isolated verification | Choose a suitable isolated Mac environment and disposable data/accounts; actual authorization required before provisioning/tests. Cover Keychain/native UI/desktop MCP state as well as HTTP. No ordinary temporary folder as a substitute. [Researched options and proposed operating rules](isolated-testing.md) now compare a lighter client-profile/process-restriction/test-user arrangement with the earlier VM proposal; no concrete boundary is selected or verified. |
| Packaging/source release | Add MIT LICENSE with accurate attribution and required dependency notices; app bundle and DMG; stable signing identity/notarization when available/required; explicit update/data preservation and authorized release destination. Do not erase data or publish implicitly. |

Unexpected implementation questions get one consolidated update with the concrete conflict and recommendation. Ask the user only where new product behavior, an unapproved route/major choice, external access/cost or changed accepted behavior actually requires input. Keep all other work in this engineering list rather than creating more approval counts.

## Resolved product-scope choice — D78

**Custom GitHub CLI tools:** a new Fetch-owned CLI adapter is not the same as importing the user's existing GitHub MCP. Recommendation: defer building the custom adapter for this first slice and support imported definitions according to their verified compatibility. The user selected deferral in D78; do not implement the custom adapter. If the user wants the adapter now, review its named operations/API mapping and credential/refresh ownership together once. [Existing proposal](github-cli-proposal.md).

Other unselected optional controls stay outside the build unless a selected workflow requires them. Their exact future API remains unselected. No separate shared-memory, graph, PostgreSQL, process-pooling or native-Tasks design is required for this slice.

## Current checkpoint

- D73 locks the clarified release scope and planning approach; D01–D72 retain their selected scopes and explicit supersessions.
- D78 resolves the custom GitHub CLI adapter question by deferring it. Engineering mechanisms are grouped above, not presented as completed facts or ten new product decisions.
- Implementation/push is authorized by D74. Continue native authorization/token issuance and protected lasting identity/Keychain, then the existing build sequence. Live managed-MCP tests and personal-client changes remain deferred; no release is authorized.


## Current D77 implementation boundary

Implement the full selected MVP source and push scoped commits. All testing, builds/checks/lints/formatters, service/app launches, runtime/package installation, trust changes and packaging execution are deferred to the user. API handlers are written before UI wrappers, with runtime verification deliberately postponed. See [the implementation brief](mvp-implementation.md). Source completion cannot establish runtime acceptance.
