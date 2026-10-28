# Historical project context before D73

October28, 2026. Preserved snapshot, superseded by the current PROJECT_CONTEXT.md and consolidated implementation checkpoint where they differ. This is not a current approval backlog.

# Fetch — shared memory and MCP server management

Latest approval — D72 (October28): encrypted per-server secret files with keys held in Mac Keychain; native Settings manual save/remove with Mac confirmation; no per-tool/automatic-renewal Touch ID. Only this shown direction is approved. Unshown store/crypto/header/bootstrap/timer/error/recovery/signing details remain implementation candidates. Next is [the complete provider verification/failure amendment](../../docs/architecture/credential-verification.md) under D25/D31, with all new clauses proposed; no runtime/test/build authorization.

Latest approval — D71 (October28): the presented account-free app-connection contract is approved, including seven operations, one-time Mac confirmation, separate management scope, lifetimes/renewal/reuse/revocation/restart and admission bounds. Exact unshown wire/OS/storage mechanisms and real client evidence remain open. D72 subsequently selects the shown [Settings/Keychain direction](../../docs/architecture/secret-storage-settings.md); unshown mechanisms remain implementation candidates. Next is provider verification/failure policy; implementation/live testing remains paused.

Latest approval — D70 (October28): the presented local HTTPS/shared listener, unique per-install certificate/key and explicit browser/client trust verification direction are approved. No live certificate/client/VM work or application implementation is authorized. D71 subsequently approves the presented [app connection/token contract](../../docs/architecture/client-authorization.md); unshown mechanisms/evidence remain open. D69 still means no Fetch account/signup/password.

Latest clarification — D69 (October28): no Fetch account, signup or Fetch password. Apps obtain their own Fetch access token through the browser connection to local Fetch. Keep setup simple and preserve the selected standard exchange/renewal direction. That clarification did not itself select HTTPS/certificate trust; D70 subsequently selects the presented HTTPS direction. Exact approval/key/client mechanisms remain open and no runtime implementation/test was approved.

Current checkpoint — October28, 2026: D68 approves the presented ordinary queue/settings/cancellation/restart contract. Setup/server routes, raw tools/calls/results, aliases, management mappings, transports/catalogs/native relays/resources/prompts are reviewed at their recorded scopes through D68. Do not reopen those policies or treat unshown mechanisms as approved.

Use the canonical [remaining-decisions checklist](../../docs/architecture/open-questions.md#remaining-decisions-checklist) for the seven-step review order. D70 selects the presented [local OAuth HTTPS/certificate-trust direction](../../docs/architecture/local-oauth-trust.md). D71 selects the presented [app authorization/token behavior](../../docs/architecture/client-authorization.md). D72 selects the shown Settings/Keychain direction. Next is provider verification/failure policy. Standard OAuth/renewal, encrypted per-server credentials, outside-package placement and stored-credential verification are already selected. Exact endpoints, Settings/key/provider integration and later implementation/onboarding/testing choices remain open.

Fetch stays one macOS-first product with shared memory and MCP management, a Rust backend, one Fetch process, local children plus selected remote upstream adapters, and localhost client access. PostgreSQL/retention, multiple managed processes, native-settings normalization, live resource-content subscriptions and native background Tasks are deferred at their recorded scopes. Everyday Claude Code/OpenCode compatibility and the existing Codex target require authorized isolated evidence.

Architecture research/discussion/documentation are authorized. Application source, dependencies, services, credential use/migration, personal client/trust-store changes, live tests and environment provisioning remain paused. A proposal or documentation ACCEPT is not implementation permission.

[Architecture records](../../docs/architecture/README.md) hold current decisions/contracts; [NOTES.md](../../NOTES.md) keeps dated notes; [the milestone snapshot](../milestone.md) records actual work/evidence. Dated continuation checkpoints below are historical where later confirmed decisions supersede them; they do not add current open questions.

## Repository — October 25, 2026

- User-designated Git repository: [yuyanghu06/fetch](https://github.com/yuyanghu06/fetch).
- Git remote URL: `https://github.com/yuyanghu06/fetch.git`.
- The existing local project at `/Users/yuyang/Documents/fetch` has been initialized with a `main` branch and this URL as `origin`. No commit or push was performed during this setup.

## Purpose

Fetch is one application combining shared memory across personal assistants and shared MCP server management. Both capabilities belong in the same user-facing app and interface. The website and desktop app are interfaces to the same product, rather than separate memory and MCP products. The current MCP MVP is a self-built Rust application, with IBM ContextForge used as design inspiration.

Give the user one shared place for preferences and, later, other useful memories that assistants can read and update. D69 selects no Fetch account; local apps connect through the browser to obtain their own token. Earlier cross-computer ambitions remain future scope. Target use cases include Dots, Codex, Muse, Claude Code, and other personal assistants. Compatibility with each tool still needs verification.

The memory portion of the initial MVP saves and shares explicit user preferences as one overarching memory prompt for the user, with simple history and undo. Assistants retrieve that prompt and apply or inject its content through whatever memory/context mechanisms they support. Broader conversation history, project memory, and the memory graph are future possibilities, not initial requirements. The exact MCP management subset for the MVP remains to be designed within this same app.

The intended distribution is an open-source repository that anyone can run directly on their own machine. Each installation uses its operator's own settings/credentials. The initial MVP now defers PostgreSQL and its Tailscale database setup; earlier database guidance is retained for future work. A hosted application and an alternative PostgreSQL host are optional future deployment choices.

## MVP simplification — October 25, 2026

- Use one shared memory prompt rather than a graph of individual memories in the MVP.
- Support viewing and editing the current prompt, inspecting simple history, and undoing changes by restoring earlier content. The exact revision storage, retention policy, and handling of simultaneous edits remain to be designed.
- Defer graph nodes, links, graph visualization, and connection-based node sizing. The earlier graph and node-storage proposals below are retained for possible future work and do not constrain the initial prompt/history schema.
- Leave version control out of the MCP side for now, including MCP configuration history and rollback. Normal source Git still applies; database migrations are deferred from the initial MVP.
- Keep the API-first architecture: memory reads, edits, history, and undo use the same application APIs from every interface. Actual prompt injection and automatic consultation depend on each assistant's supported capabilities.

## Initial MVP storage amendment — October 26, 2026

After the session handoff, the user continued to state that retention should be sidelined and the initial MVP should have no PostgreSQL functionality. [storage-scope.md](../../docs/architecture/storage-scope.md) inventories remaining saved-state needs. PostgreSQL provisioning/connections, database-host Tailscale prerequisites, schemas/migrations and PostgreSQL-backed result retention are deferred from this release. The earlier database direction below is future/historical guidance, not an initial-MVP requirement. The user subsequently chose local files for server configurations and the existing encrypted store for Fetch-managed tokens/credentials. Only credential references go into non-secret configs; D26 later selects each server’s own encrypted credential configuration files. D28 later selects durable files under `~/Library/Application Support/Fetch/servers/<server-id>/`, outside replaceable packages; exact format/names and key handling remain open. No substitute database is selected. Memory/release scope is not silently cancelled. Implementation remains paused.

Current API-review checkpoint, October 27: D34 approves submission, D35 discovery and D36 the final progress-read contract. Current discussion is step 2 tool sharing. D36 includes the shown progress GET refinements; no blanket implementation authorization is inferred. Existing D32 UI-only setup and D33 app/guidance workflow consistency apply to both.

## MCP sharing design discussion — October 26, 2026

The user has resumed architecture/API discussion, not application implementation. The user explicitly authorized synchronizing the discussion into [docs/architecture/](../../docs/architecture/README.md), updating AGENTS.md, and keeping those docs updated as design proceeds. That documentation authorization does not permit coding or restoring the removed draft.

The confirmed MCP MVP is macOS-first, local to the current user, with a self-built Rust backend and IBM ContextForge as design inspiration. One Fetch process contains the UI, localhost API, and server manager, with MCP wrappers as child processes. Opening Fetch starts the servers; backgrounding or red window-close keeps them running; explicit Quit stops them. The later storage amendment above defers PostgreSQL/database-host Tailscale requirements from the initial MVP.

D32 clarifies that Fetch’s normal installation/setup path runs entirely through the UI without a connected agent. UI status and post-install sequencing use structured installation progress and existing server-state fields, while `next_steps` remains agent-only guidance in the common API envelope. Backend prerequisites/permissions stay authoritative; optional agents wrap the same APIs. Exact UI/onboarding branches and initial Settings authority remain open. D33 extends this to all previously discussed and future flows: native UI sequencing and agent guidance must match the same reviewed prerequisites, valid actions/order, completion/failure/retry rules and human handoffs. Update both when the workflow changes; UI logic never depends on guidance text.

Discover global Claude Code, Codex, and OpenCode configurations, with all discovered servers selected by default and per-server controls to opt out. A catalogue is deferred. Use configuration-derived package/launch information to prepare selected npx/uvx servers; run existing local scripts in place. Do not clone entire configs/source trees or migrate credentials. Fetch installation provisions private npm/Python tooling under `~/Library/Application Support/Fetch/packages/`, with shared compatible runtimes and separate server dependency installations. Resolve requested package versions at install and change them only on explicit update/reinstall. Grok bot and Muse come later; the exact Muse harness remains unidentified.

Fresh credentials and OAuth tokens live in the same encrypted local credentials area, managed through Settings, assigned per server, and refreshed automatically where supported. Each harness uses standard OAuth/renewal to access the single Fetch MCP, which exposes installed tools individually and built-in Fetch setup/management tools backed by the same application APIs. Agents can assist setup; responses provide status and next steps without provider credential values. Raw secret migration, arbitrary-wrapper authentication/persistence, and credential-store key management must not be assumed solved.

Discovery/installation handlers retain the confirmed `src/api/setup/` grouping and `/api/v1/setup` URLs. D21 moves ongoing server information/authentication/config/credentials/restart URLs to `/api/v1/servers`; their exact Rust handler grouping remains open. Accepted route boundaries cover discovery, asynchronous installation/progress, installed-server information, authentication start, and per-server configuration/credential writes. Server information accepts repeated optional `id=` values, always returns a `servers` array, and includes `not_found_ids`. Supported OAuth opens the default browser and returns `202`/`authenticating`; other methods return known setup instructions or missing integration information. Installation failures do not stop other selected installs; retain successes, mark the finished group failed if any item failed, and guide retry of failures. The configuration PATCH is available to agents/interface; the credentials PATCH requires Settings authorization for manual values and reuses the encrypted server-scoped store for environment variables and custom auth tokens. Backend OAuth login/refresh writes remain automatic. [api-contract.md](../../docs/architecture/api-contract.md) records accepted paths and incomplete schemas. Applying changes to running wrappers, encryption/key management, broader module grouping, tool/session/concurrency rules, and dependency choices remain open in [open-questions.md](../../docs/architecture/open-questions.md).

The user requires input on every route and major architecture choice, now preferring related proposals grouped to move faster. No silence or partial reply constitutes approval of all suggestions. This discussion remains within the same Fetch product and does not authorize changes to the existing personal MCP setup.

Installation generates each installed server's stored Fetch configuration automatically from the approved discovered settings. The configuration PATCH is for later edits; do not add a separate configuration-creation route for this flow. Original-harness connection setup/cleanup remains separate. For restart, an automatically revealed Settings control sends POST when clicked. The latest selected completion flow returns `202` immediately and keeps the loading icon active until a separate result confirms the replacement is ready, superseding an earlier held-open POST choice. Normal restart waits for already-forwarded tool calls and temporarily rejects new calls; the per-request force override begins stop/start immediately without waiting, still requiring readiness before success. Monitor the particular attempt through `GET /api/v1/servers?id=...&restart_id=...`, as explicitly selected by the user. Ordinary server information can omit the restart filter. On October 27 the user approved the complete presented restart behavior, including the envelope/validation/errors, permissions, 60/5/30-second deadlines, one-active-restart rule, readiness and disconnection/retry behavior. ID representation/local operation-state lifetime, protocol/library/cleanup mechanisms and additional task tracking remain open; retention/recovery remain deferred. Ordinary concurrency/extra wrapper instances are not selected. No restart implementation exists.

## Pre-build outline and milestone journal — October 26, 2026

The user asked what else must be covered before building and having a full API/implementation outline. [build-readiness.md](../../docs/architecture/build-readiness.md) records the current route inventory, missing API/protocol surfaces and proposed remaining review groups. It does not approve those proposals or lift the implementation pause. The user also introduced the milestone-journal workflow: [milestones/milestone.md](../milestone.md) holds the rolling snapshot, [milestones/to-do.md](../to-do.md) preserves unfinished work, and immutable America/New_York history entries record inspected passes. Architecture records remain canonical for decisions.

The user subsequently selected discussing the seven groups in order: API contracts, tool sharing, execution behavior, authentication, implementation structure, onboarding, then testing/delivery. The user approved restart request/result fields, then requested one whole API contract per turn. The user subsequently approved that complete restart bundle on October 27 (D20), with retention/recovery still deferred. D23 subsequently approves ordinary installed-server information. D27 later accepts the reviewed general authentication behavior. D28 subsequently approves the outside-package durable server-data tree. The base Settings-only credentials contract is reviewed; D31 adds verified-200/shared failure requirements and D34 approves the presented submission contract; D35 approves discovery and D36 progress read; current review is step 2 tool sharing. D30 approves common launch-config and defers native-settings normalization. Concrete GitHub CLI tool scope/auth mechanisms remain for steps 2/4, with crypto/file details open and application implementation paused.

The user ended this collaboration by requesting a complete written handoff. [milestones/mcp-mvp-architecture/handoff.md](handoff.md) links the full planned outline, API/source/package structure, preferences, research and evidence. This was the historical step-1 checkpoint: restart fields approved and remaining policies awaiting feedback. The later October 27 approval in D20 supersedes that checkpoint; the handoff did not authorize implementation.

On October 27 the user resumed architecture discussion, clarified stopping versus restarting, then said "Cool all good next" to approve the remaining restart bundle. [The restart contract](../../docs/architecture/restart-contract.md) and D20 record that decision. [Ordinary installed-server information](../../docs/architecture/api-contract.md#approved-complete-record-and-ordinary-get-rules) was subsequently approved (D23). PostgreSQL/retention deferral and local-config/encrypted-token storage remain selected. The user then reaffirmed reusing the server GET and moved the server family out of setup (D21): `/api/v1/servers` with its existing authentication/config/credentials/restart children; discovery and installation stay under `/api/v1/setup`. The established `/api/v1` prefix remains. The user then confirmed shared `next_steps` across current/future application API JSON responses (D22) and approved the ordinary GET (D23). General authentication is now approved (D27); D30 defers broader individual-server settings normalization beyond MVP and approves the existing common launch-config PATCH. D31 subsequently adds the verified-200/shared failure requirement to the reviewed base credentials contract. D34 approves the presented submission contract; D35 approves discovery and D36 progress read; current review is step 2 tool sharing. Application implementation remains paused.

On October 27 the user required actual MCP/provider auth and storage ownership to be part of the contract (D24). [Current research](../../docs/architecture/research.md#actual-mcp-auth-and-token-ownership--rechecked-october-27) confirms HTTP MCP OAuth does not provide universal local-wrapper provider-token export, and server-owned URL login keeps provider tokens outside the MCP client. D25 requires successful encrypted storage/readback followed by `2xx` from the service's authenticated test endpoint using that stored credential before credential-based auth success. Exact endpoints, wrapper-owned integration, non-token verification/recheck/cleanup and concrete key mechanisms remain open; D27 subsequently approves the reviewed general auth deadline/overlap policies.

The user subsequently approved the reviewed general authentication behavior (D27) and explicitly required credentials in each managed server's own encrypted configuration files (D26). D28 later selects the outside-package directory; filenames/encoding/key handling remain open. They proposed [GitHub via GitHub CLI](../../docs/architecture/github-cli-proposal.md) and asked whether credential files belong under package installs. The user subsequently approved [durable server state outside replaceable package folders](../../docs/architecture/storage-scope.md#approved-server-data-placement--d28) in D28. Exact filenames/encoding/key protection remain open. CLI tool/API scope, installation ownership and fresh auth/renewal that meet D25/D26 remain unselected; no plaintext/default-CLI-store exception or implementation is approved.

## Architecture decision — October 22, 2026

On October 25 the user reaffirmed that this applies to the entire repository: every feature, including MCP sharing and management, is exposed through APIs, with interfaces wrapping them. The user wants to decide API behavior and product features collaboratively and avoid unrequested features or overengineering. After initially requesting a one-shot pass, the user explicitly required being prompted for every API route and every major architecture decision, with input on the exact technical specifications before implementation. The user then stopped implementation and requested removal of the draft code. These design principles remain recorded for any future work; they do not authorize resuming it.

The user explicitly chose an API-first architecture and development order:

> Center the entire site around the APIs, especially the POST memory API. Everything else, including website submissions and MCP calls, wraps those APIs. Always build the APIs first.

The intended request flow is:

Website / HTTP client / MCP interface → memory API → database

The POST memory API owns the behavior for saving memory. All clients use the same endpoints and receive the same validation, permission checks, and memory behavior. Interfaces must not bypass the APIs to access the database or maintain separate versions of memory-saving logic.

Build order:

1. Define the API operations, request/response formats, authentication, and permissions for the preference MVP.
2. Implement the APIs and persistence; verify real requests, saved results, and access restrictions.
3. Build the website's shared-prompt editor, history, and undo interface around those working APIs.
4. Add MCP as another API client when it enters scope.

Exact endpoint paths, schemas, and conflict-handling rules remain to be designed. API-first does not require implementing future features before building the MVP interface.

## Storage decision — October 24, 2026 (deferred from the initial MVP)

The user chose PostgreSQL hosted locally on their own machine as the primary database. This supersedes the earlier proposal to use Supabase for primary storage.

The user clarified that, for the default locally hosted database, application database connections should use the database host's Tailscale IP from the initial setup, even when the application and PostgreSQL run on the same machine. This supersedes the earlier interpretation that Tailscale would only be introduced later. Each installation supplies its own connection settings; do not hardcode the user's machine address or credentials.

The existing memory operations and API-first architecture remain in place: website, HTTP, and MCP interfaces use the memory APIs, which access PostgreSQL through the configured database connection. The default local database connection uses the Tailscale address. This database connection belongs to the backend, rather than browser code or individual assistant interfaces.

The future goal is to allow the application to run on Vercel or another host while PostgreSQL stays on the user's own machine. The deployed backend must also have an authorized connection into that user's Tailscale network. A private Tailscale IP by itself is not sufficient to make the database reachable from a hosted runtime; the exact connection method and hosting compatibility remain to be designed and verified.

This records the storage direction. Local PostgreSQL installation, database setup, and Tailscale configuration have not been performed as part of this update.

## Database migration portability — October 24, 2026 (future database work)

The user explicitly requires one shared set of database migrations that supports PostgreSQL hosted locally, Supabase PostgreSQL, and PostgreSQL on a cloud instance. Local hosting is the default, while Supabase and cloud instances remain supported deployment targets to build and verify.

Keep the migration files and their version history in the repository. Switching database hosts must retain the same application schema and migration history rather than introduce separate provider-specific schema branches. Configure the database address and credentials for each deployment, and keep any hosting-specific setup separate from the common migrations. PostgreSQL version and extension requirements must be documented and checked for each supported target.

No database migration files exist yet. Migration tooling and compatibility verification remain implementation work; recording this requirement does not establish that those targets have been tested. Moving existing data between hosts will also require a separate data transfer when needed.

## Product direction

- Unified app (October 25, 2026): shared memory and MCP server management live in the same Fetch interface. Do not split them into separate standalone products; backend service boundaries do not determine the user-facing app boundary.
- Backend direction (October 26, 2026): build Fetch ourselves with a Rust backend and use IBM's open-source ContextForge as design inspiration. This replaces the earlier assumed ContextForge foundation; exact internal architecture, dependencies, and API contracts require discussion before implementation.
- Confirmed native MVP architecture and setup behavior are recorded in [docs/architecture/decisions.md](../../docs/architecture/decisions.md): macOS-first, one Fetch process, localhost access, configuration-derived preparation, private runtime/server installations, encrypted credentials, standard harness OAuth, and agent-assisted setup. This summary does not approve the still-proposed code module tree or remaining API details.
- Desktop direction (October 25, 2026): try Iced for a Rust interface sharing code across Windows, macOS, and Linux. The user does not want Electron. No prototype has been built; the website remains a separate interface choice. Rust is now selected for the backend, while the interface scope for the MCP sharing MVP remains to be designed.
- Proposed MCP management UI within Fetch: show servers in the shared app interface, expand each server to inspect its tools, and offer per-tool concurrency limits (default one) plus a separate server concurrency cap. Add backend sessions/workers on demand when higher limits permit parallel calls. See NOTES.md for the proposal, examples, and unresolved counting and execution details. These features have not been implemented; the exact MVP subset and implementation order remain to be settled.
- MCP version control: out of scope for now. Do not make MCP configuration revision history or rollback an MVP requirement.
- Distribution: an open-source repository that anyone can run locally with their own configuration. Local operation must not depend on a Vercel deployment or the original developer's infrastructure.
- Proposed web framework: Next.js. Initial MVP excludes PostgreSQL; server configurations use local files and tokens use the encrypted store; D28 selects the durable directory outside packages; encoding/filenames/key handling remain open. Earlier local PostgreSQL/Tailscale hosting is future guidance only.
- Future database portability, if selected: the earlier common migration/schema direction across local/Supabase/cloud PostgreSQL remains deferred.
- Website memory area for the MVP: a white, light-themed interface to view and edit the shared memory prompt, inspect its history, and undo changes through the APIs. The same Fetch app also contains MCP server management. The Obsidian-style memory map is deferred beyond the MVP.
- Agent access to shared memory: ordinary authenticated HTTP requests and browser interaction where supported remain selected. D48 subsequently requires descriptive MCP tools for any selected agent-accessible memory APIs; this supersedes the earlier optional-MCP direction to that extent without choosing memory routes or release scope.
- Browser sign-in approach to try: give the agent a single-use website connection link that establishes an authenticated session in the agent's own browser. See NOTES.md for the intended flow and open details.
- Each assistant keeps its own conversation context. The shared preference prompt supplies preferences when the assistant consults it; it does not automatically merge assistants' live context windows. No Fetch account is required under D69.

## Intended onboarding

After connecting an assistant, the user copies a prompt asking it to:

1. Read the shared memory prompt and apply relevant preferences to its local user instructions or memory, where supported.
2. Save missing, durable preferences that the user has explicitly stated and that the assistant can access into the shared prompt through the memory APIs.
3. Persist instructions to check for changes at startup, where the assistant supports this behavior.
4. Save subsequent explicit preference changes back to the shared prompt through the memory APIs.

The final onboarding prompt is not yet settled. Reliable startup checks and authenticated reads/writes must be tested in each assistant.

## Current status

Architecture/API discussion for the MCP sharing MVP resumed on October 26. The user subsequently authorized documentation synchronization into docs/architecture/ and AGENTS.md; application implementation remains stopped. The provisional Rust backend, migrations, scripts, synthetic tests, old API contract, README, build outputs, and generated runtime data were removed on October 25. The new architecture API document is a discussion record, not the restored draft or implemented behavior. No Iced desktop implementation was written or launched. Project records now include the architecture folder, alongside the project-scoped Codex configuration and Git repository/remote.

The draft Fetch service, ContextForge companion, and project-local PostgreSQL instance have been stopped. The existing personal shared-MCP gateway and client configuration remain unchanged. PostgreSQL 17 and cached development dependencies remain installed; there is no running Fetch database or application service.

A private local source-only backup of the removed draft is recorded in NOTES.md; it excludes generated credentials and database data. Do not restore it or resume implementation unless the user asks. The saved product decisions remain for later discussion.

## Supabase development connection — October 24, 2026

- Organization: `yuyang-personal-projects`.
- Project: `fetch` (`puoumrfxgocanhwiuwda`).
- Project-local Codex MCP server: `fetch_supabase`, configured in `.codex/config.toml` with `project_ref=puoumrfxgocanhwiuwda`.
- The initial OAuth authorization succeeded, and a read-only inspection of this project's `public` schema returned no tables. No database changes were made during setup.
- OAuth authorization for the project-pinned URL is complete. Codex reports OAuth active, and the scoped MCP connection successfully listed the project's `public` tables (none yet). The user explicitly authorized completing this reauthentication remotely in the existing signed-in browser.
- This MCP connection is for development access to Supabase. It is separate from the optional memory MCP interface for the application's users. The later local PostgreSQL storage decision supersedes the Supabase storage proposal; the existing development connection has not been removed or changed by that decision.

## Earlier node-storage proposal — October 22, 2026 (deferred beyond MVP)

The October 25 shared-prompt decision supersedes this proposal for the initial implementation. Keep it as possible future graph work; the prompt/history storage schema remains to be designed.

The user proposed starting with two simple tables. Each memory node has:

- A UUID.
- A user ID.
- `created_at`, `updated_at`, and `expires_at` timestamps.
- A JSON payload containing a `raw` field with the content the frontend renders.

The second table's role and the exact meaning and format of `raw`, write/update behavior, and expiration behavior remain to be settled. Do not treat proposed implementation defaults as confirmed decisions.

## Earlier graph behavior — October 22, 2026 (deferred beyond MVP)

The user deferred the graph on October 25. Preserve these earlier choices if graph functionality is revisited; they are not requirements for the shared-prompt MVP.

- The actual memory graph is undirected. A connection between A and B is mutual, regardless of which node initiates it.
- Links represent connections between memory nodes by storing neighboring node UUIDs.
- Do not restrict the memory graph to a DAG or prohibit cycles. The user clarified that the earlier graph terminology referred to the memory map itself, not version history.
- On the white Obsidian-style map, nodes with more connections appear larger.
- Derive display size from the number of connections rather than storing an unrelated manually assigned size. The exact sizing formula remains to be chosen.

Whether neighboring UUID lists live in node JSON or a separate links table remains to be settled before implementing graph storage. Whichever representation is chosen must preserve undirected connections consistently.

For the MVP, memory history means simple revisions and undo of the shared prompt. The storage model remains to be designed; graph history, branching, and merging are not initial requirements. The assistant's earlier DAG interpretation was not a user decision about version history.

D30 limits MVP config editing to Fetch’s common `args`, `working_directory` and environment credential-reference fields and approves the reviewed contract. Normalizing individual servers’ different native settings/config files is deferred beyond MVP; these common fields are not an MCP-wide standard. Settings-only manual value entry and implementation pause remain intact.

D31 requires credential-save 200 to pass the stored-new-credential test as well as encrypted save/readback, with shared CREDENTIAL_VERIFICATION_FAILED in credential/auth/test-failure GET responses. The reviewed base manual contract is accepted subject to concrete verifier/error/edge-case policies; D34 approves the presented submission contract; D35 approves discovery and D36 progress read; current discussion is step 2 tool sharing. D25’s service-authenticated proof and the implementation pause remain intact.

D43 selects raw upstream MCP ToolResults with no added Fetch notice, and explanatory Fetch-owned failures. D44 reuses next_steps and removes response_context. HTTP raw result and original Tool/input/output schemas remain intact. D45 subsequently approves naming policy, D47 connection policy and D49 presented management tools; current step2 reviews the remaining installed-tool HTTP call contract. Exact alias encoding/host limits, protocol compatibility, capabilities and execution mechanisms remain open. No application implementation is authorized.

D45 approves stable server-prefixed MCP aliases, exact alias-to-original-origin mapping, a reserved Fetch namespace, generated exceptional names and stable non-retargeting. Canonical HTTP Tool names/metadata and inputs/results stay unchanged. Exact token/exception encoding and supported-host character/length/persistence mechanisms remain later review. D47 subsequently approves the presented connection policy; current next topic is the remaining installed-tool HTTP call contract.

New MCP connection tests must use a virtual computer/VM or suitably isolated environment protecting the user's real files, credentials, harness configs, apps and services. The rule is in global/project AGENTS.md and NOTES.md; exact environment selection/provisioning and live tests remain unapproved.

D47 approves the presented one-loopback /mcp Streamable HTTP connection policy, legacy2025/modern2026 targets, per-request bearer and local Host/Origin validation, temporary legacy sessions and no call replay on reconnect. Exact unshown wire mechanisms remain later review. Exact HTTPS OAuth issuer/TLS/trust, supported-client tests in isolated environments and later implementation mechanisms remain required.

D47 approves the presented localhost /mcp connection policy: same Fetch port/process, Streamable HTTP legacy2025/modern2026 targets, native setup/methods/temporary session behavior, per-request authority and no replay on reconnect. Exact numeric port/SDK/client releases/HTTPS issuer/TLS/scopes/execution mechanisms stay open. D49 subsequently approves the presented Fetch-owned management mapping; current next topic is the remaining installed-tool HTTP call contract; no code authorized.

D49 approved presented contract: [Fetch-owned management tools](../../docs/architecture/mcp-management-tools.md) maps eight tools to existing APIs, with per-caller authority and Settings-only manual credential entry. D49 now approves their presented names/input/result mapping. Imported model arguments remain unchanged; /mcp only handles protocol/routing.

D48 requires descriptive MCP tools for current/future agent-accessible product API operations, using backend permissions and behavior. Preserve Settings-only manual credentials, native OAuth/MCP protocol roles and raw imported metadata. D49 now approves the presented eight management-tool names/input/result mapping; future selected API scope remains separate.

D49 now approves the presented management-tool contract with D48 descriptive API coverage. Current step2 reviews the remaining installed-tool HTTP route/outer outcome/errors/ordinary lifecycle together; D39/D43/D44 raw arguments/results remain settled. Numeric ordinary timers/concurrency/cancellation remain step3 prerequisites; no build authorization.

D50 accepts the presented HTTP installed-tool call contract. Current bundled review is ordinary execution: shared server state, concurrency, deadlines, cancellation/disconnect and restart accounting. Exact capabilities/GitHub CLI and auth/implementation choices remain open; no building is authorized.

D51 revises the [ordinary execution design](../../docs/architecture/mcp-execution.md): queue/concurrency required; prior no-queue draft superseded and fixed120 withdrawn. D53 subsequently selects120-second fallback, while exact scheduler/override/aggregate GET-state policy remains pending; richer capability scope stays open.

D51 requires queued call scheduling with concurrency coordination. The no-queue proposal is superseded and fixed120-second wait is withdrawn; the user asked about reading individual tool limits. MCP has no standard per-tool timeout field; D53 subsequently selects120-second fallback. Known metadata/override details, separate queue/run deadlines and revised aggregate GET schema remain proposed. No unlimited wait or universal120 cap is selected.

D52 requires durable per-server tool features/settings applied by Fetch on matching calls, preserving raw arguments and no install-time start/auth. D53 selects120 seconds as fallback, not a hard cap over longer specific limits. The full queued execution/settings/GET-state contract is the current next review; no implementation authorized.

The complete queued execution/settings contract is now proposed in mcp-execution.md, covering existing config API/tool extensions and safe aggregate GET state. D51/D52/D53 are selected; queue64/16MiB/300s, C1 adjustable, grace5s and exact schemas/restart/hot-application remain unapproved.

Current next review is the proposed initial MCP capability scope in docs/architecture/mcp-capabilities.md, preserving selected tools/catalog channels and raw data. Additional progress/resource/prompt/interaction/task exclusions await input; authentication is next. D57 process pooling deferred; pending execution parameters are not silently approved.

D61 selects HTTP/SSE upstream support as separate transport functions routed by the shared runner alongside local-process handling; see docs/architecture/remote-upstreams.md. Fetch remains client-facing localhost-only. Exact import/config/lifecycle amendments, upstream revisions, D37 modern-header compatibility and remote uncertain-work recovery remain open; implementation stays paused.

D61/D62 follow-up: the shared runner must handle all three selected transport paths (stdio, Streamable HTTP, legacy HTTP+SSE), not merely store their labels. Their adapters use the same shared backend authority, scheduling and execution controls; remaining exact contracts and implementation stay pending.

D63 approves the presented existing-route remote import/config/GET/reconnect amendment in docs/architecture/remote-api-amendment.md, including fixed imported endpoint identity, editable header references and explicit forced-recovery tradeoff. Separate protocol/security/provider/implementation mechanisms remain open; implementation stays paused.

Current next proposal after D63: docs/architecture/mcp-protocol-adapters.md groups protocol version setup, observable SDK tool rejection with no partial-catalog success, native-only missing legacy resultType marker and modern private/zero-TTL catalog envelopes. D37/D43 compatibility exceptions remain unapproved; canonical HTTP raw values stay unchanged and implementation remains paused.

D65 now accepts the presented docs/architecture/tool-catalog-status.md partial/rejected_tools, raw/2025/2026 views, native metadata/listing-tool context default and initial upstream/cache defaults. Source-versus-export rejection remains explicit; actual SDK/client evidence and separate architecture mechanisms stay open; implementation paused.

D66 approves the presented native progress/roots/model/form/URL/MRTR relay through shared /mcp handlers, with actual caller capability/ownership, safe legacy limits and no Fetch fallback. Direct REST/raw terminal data/original deadlines stay selected; actual SDK/resource/client mechanisms, extra capabilities and implementation remain open.

Current next review after D66: docs/architecture/mcp-resources-prompts.md proposes native resource/prompt/completion forwarding with stable source-qualified selectors, preserved returned payloads and safe ambiguous raw-link handling. Catalog changes included; live-resource subscriptions/native Tasks proposed deferred subject to actual-workflow acceptance. No new REST/UI/model or application implementation; all new clauses await input.
