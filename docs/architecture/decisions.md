# Confirmed architecture decisions

Recorded from the user's design discussion on October 26, 2026. These are design decisions, not implementation-completion claims. Exact API examples and unresolved details are tracked separately in [api-contract.md](api-contract.md) and [open-questions.md](open-questions.md).

## Current MCP resolution pointers — session audit October28

The original Fetch sessions were checked against this log and the consolidated plan. Older entries retain chronology; an early “open” sentence does not override its later selected resolution. This audit adds no new product decision.

| Earlier entry | Later selected resolution / current interpretation |
| --- | --- |
| D04 explicit-file/discovery boundary | D35 approves the presented bounded explicit-file rules and selection exceptions; D61/D63 subsequently amend remote scope/fields. |
| D09 tool sharing/routes/names/runtime policy | D38/D43/D45/D47/D49/D50/D59–D68 resolve the presented scopes. Individually exposed tools and shared backend APIs stay selected. |
| D10 app authorization routes/scopes/lifetimes | D71 approves the presented contract and D73 reaffirms one saved agent connection. Only unshown wire/implementation/client-proof details remain worklist items. |
| D11 ordinary server/progress fields | D23/D36 and selected route relocation D21 govern, not the early unresolved wording. |
| D13 install concurrency/retry/results | D34/D36 approve the presented single-group/sequential/source-reuse/failure/retry/current-run behavior; no cross-Quit recovery promise. |
| D14 config/manual credential contract | D30/D31 and D72/D73 govern shown behavior; the unaccepted verification exception remains unselected. Exact mechanisms stay one engineering worklist. |
| D15 restart/overlap/readiness | D20/D63/D68 settle the presented local restart/remote reconnect/queue/drain behavior. No automatic config-save restart. |
| D16/D17 discussion sequence | D73 makes the consolidated implementation checkpoint the current work order while retaining full new-contract reviews and historical evidence. |

[The MCP session audit](mcp-session-audit.md) records source coverage and restored acceptance checks; no prior session supplied an omitted implementation go-ahead.

## D01 — Product scope and implementation authority

- Fetch is one application containing shared memory and MCP server management. The current work is the MCP sharing portion, not a separate product.
- The first native release targets macOS. Windows/Linux remain a later direction rather than first-release requirements.
- Build Fetch ourselves with a Rust backend. IBM ContextForge is design inspiration, replacing the earlier assumption that a ContextForge runtime must be the foundation.
- Iced remains the previously selected desktop framework to try. It has not been validated by a Fetch prototype; Electron was rejected earlier.
- The user must participate in architecture and API decisions. Discussion, research, and documentation do not authorize writing the application.
- The current documentation-sync request authorizes documentation writes only. Preserve the stopped implementation and the existing personal MCP gateway/client configurations.
- The user explicitly requested ongoing documentation maintenance as decisions are made. Update these records during the same working turn rather than waiting for another sync request.

## D02 — One Fetch process and explicit application lifecycle

- The UI, local API, and server manager run inside one Fetch process.
- Existing MCP wrappers remain independent child processes; they are not rewritten into the Fetch executable.
- Opening Fetch starts the MCP servers.
- Backgrounding Fetch keeps its servers running.
- The macOS red window-close button backgrounds Fetch.
- Explicitly quitting Fetch stops its managed server processes.

This chooses one application process, not a particular Cargo package/workspace layout. Startup ordering, crash cleanup, startup failures, and in-progress installation behavior on Quit remain to be specified.

## D03 — Localhost access

- The MVP API/MCP listener accepts connections from the same Mac through localhost.
- Cross-device access over Tailscale is deferred.
- This is a listener/access decision. The later D18 decision defers PostgreSQL and its database-host Tailscale setup from the initial MVP; the earlier connection direction is future guidance.
- Exact ports, interface binding details, and OAuth localhost interoperability tests remain implementation-design work.

## D04 — Initial harnesses and discovery

- Discover existing global MCP configurations for Claude Code, Codex, and OpenCode during setup.
- The initial scope is global configuration. Project-scoped import was not selected. The user later added an explicit configuration-file path input; the precise boundary for user-selected files outside the usual global locations remains to be clarified rather than silently expanded.
- Grok bot and Muse are later integrations. The exact Muse harness/product is unidentified.
- Show discovered servers selected by default, with per-server toggles/dropdowns so users can opt out.
- Users initiate installation with an Install button.
- Per-tool import selection is outside the current initial scope; the user accepted server-level setup instead.

## D05 — Use existing definitions without cloning credentials

- Defer the catalogue. It is not an allowlist or an MVP installation requirement.
- Read existing configurations for package identity, requested version, launch settings, paths, and credential-variable requirements.
- Prepare npm/npx and Python/uvx packages using their existing configuration information. Custom/unlisted packages do not require a Fetch catalogue entry.
- Run existing local scripts/executables where they already live. Do not copy their source simply to make a Fetch-managed instance.
- Preserve necessary non-secret settings instead of asking the user to re-enter everything.
- Do not clone whole configuration files, copy credential values, or migrate existing login tokens/caches.
- Provision fresh authentication when required. Reinstalling software alone does not guarantee a fresh login; wrapper-specific cache behavior still needs verification.
- Keep local preparation, npm installation, Python installation, authentication, and common start/stop responsibilities separate. Illustrative function names discussed in chat are not a finalized Rust API.

Recognized package-manager commands can provide download/install information. An arbitrary local script path or opaque shell command may not contain a reproducible install recipe; do not pretend all unknown cases can be inferred or safely copied automatically.

## D06 — Private tooling and per-server installations

- Fetch setup installs the needed tooling for its supported npm/Python server installation paths.
- Provision Fetch-managed copies rather than altering the user's global Node/Python installations.
- The selected macOS package root is:

```text
~/Library/Application Support/Fetch/packages/
```

- This is per-user installed application data, not the repository's `src/packages` directory. Resolve the user's actual home directory; never hardcode the developer's path.
- The user accepted this conceptual split:

```text
packages/
  runtimes/
    node/<version>/       # Node.js, npm, npx
    python/<version>/     # Python interpreter
    uv/<version>/         # uv and uvx
  servers/
    <npm-server-a>/        # Package and its dependencies
    <npm-server-b>/
    <python-server-c>/     # Package and its Python environment
```

- Share runtime installations where compatible; keep installed MCP dependency environments separate per server.
- Existing local scripts remain outside this tree at their original locations.

The exact runtime versions, download sources, integrity checks, server-directory identifiers, package-manager internal layout, and handling of incompatible runtime requirements have not been selected. This folder tree is not an implemented installer.

## D07 — Explicit package updates

- Honor an explicitly requested server-package version.
- If the configuration uses `latest` or omits a version, resolve it at installation and retain the resulting exact installed version.
- Opening Fetch starts the installed version; it does not silently switch to a newer release.
- Change package versions only through an explicit update/reinstall.
- Old-version/cache retention and cleanup have not been decided.

Package pinning does not add the previously deferred MCP configuration history/rollback feature. Normal source Git remains required; database migrations are future work under D18, not initial-MVP prerequisites.

## D08 — Credentials, variable injection, and fresh authentication

- Keep manually entered API keys and newly issued OAuth credentials in the same local credentials area, separated/assigned by server.
- Encrypt this storage. Users manage it through Fetch Settings rather than directly editing a plaintext `.env` file.
- Display/group credential requirements by the server/application that needs them.
- Store references/variable names in ordinary server configuration; supply actual values to the appropriate running wrapper through its environment.
- Each server receives only its own assigned credential variables. Do not pass the complete credential collection to every server.
- Keep provider credential values out of agent prompts, tool definitions, ordinary launch arguments, API/MCP responses, and Fetch logs.
- Actual values must still be available in the authenticating process's memory. Environment injection is not encryption or an OS sandbox.
- Fresh login/API-key provisioning replaces migration of old credentials.
- Installed servers requiring authentication have an Authenticate action beside their rows.
- Browser OAuth uses the user's system-native/default browser where that server supports it.
- Fetch handles token refresh automatically where supported, retaining the needed newly issued tokens and related expiry information.

The encrypted file format/path, key protection/unlocking mechanism, token ownership when a wrapper performs its own OAuth, and the full API-key/macOS-permission setup contract remain open. Neither the app's credential store nor universal OAuth support has been implemented.

## D09 — One MCP interface and common application APIs

- Installed MCP tools appear through one Fetch MCP server. Do not silently replace individually exposed tools with a generic discovery/call-only MCP facade.
- Fetch's own setup actions can be MCP tools alongside installed-server tools.
- The native UI, scripts, ordinary HTTP callers, and MCP tools wrap the same public application APIs. Keep validation, process control, permissions, persistence, and concurrency behavior in the backend.
- Connected agents can help discover, install, and configure servers through Fetch's setup/management tools. A blanket tool-only agent restriction was not selected.
- This does not authorize exposing stored provider credential values. Credential values remain private; setup responses can report variable names, missing requirements, and authentication status.
- Config-file discovery and discovery/listing of tools on already-installed servers are different operations. Only the former's route is currently accepted.

Tool-name collision rules, MCP tool names, grouping/batching of executions, per-tool/server concurrency, caller-session state, and the shared invocation API remain undecided. Older concurrency proposals in NOTES.md remain proposals.

## D10 — Standard OAuth for harness-to-Fetch access

- Use standard OAuth rather than relying on an invented pasted-token exchange as the native harness login protocol.
- Use a standard login/code exchange with automatic credential renewal supported by the harness.
- Each harness keeps its own Fetch login credential; Fetch/wrappers retain the separate GitHub/Supabase/etc. credentials needed upstream.
- The Fetch authorization grants access to its exposed tools, including the selected agent-assisted setup operations. Precise scopes and endpoint permissions still require specification.
- A one-time code alone cannot authenticate repeated tool calls; it must establish ongoing access credentials/session state.

Current client documentation/source supports HTTP MCP OAuth and renewal, but this was research, not a working Fetch test. Exact OAuth endpoints, client registration, expiry/rotation/revocation policy, protocol versions, and callback behavior remain to be designed. See [research.md](research.md).

## D11 — Setup routes and agent guidance

- Group setup API handlers under `src/api/setup/`.
- Originally group setup routes under `/api/v1/setup`; D21 later limits that family to discovery/installation and moves ongoing server operations to `/api/v1/servers`.
- The discovery route was shortened to `GET /api/v1/setup/discovery`; the earlier `/discovery/servers` path is superseded.
- Setup responses include `status` and `next_steps`, with structured actions plus plain-English instructions for agents/users. D22 explicitly extends `next_steps` to all current/future Fetch application API JSON replies, including success/pending/error and empty guidance arrays when appropriate.
- Guidance should tell the caller which setup action remains and whether the agent or user must perform it. Exact action identifiers and complete response schemas remain under review.
- Installation returns immediately with an installation ID; a separate status operation reports progress.
- Instruct agents to proceed only after overall installation success. Backend readiness checks should enforce the prerequisite too.
- Installation success means preparation succeeded, not that every server is authenticated or ready for tool use.
- `GET /api/v1/servers` accepts optional repeated `id=` query parameters. No IDs lists all installed servers; one/more IDs selects their detailed records. Return `servers` as an array even for one requested ID, and report unknown IDs alongside matches in `not_found_ids`. The user approved this input/response behavior after the initial documentation sync; exact detail fields and remaining status/validation behavior are still to be specified.
- `POST /api/v1/servers/{server_id}/authenticate` is accepted. No credential values are supplied in this request. Supported OAuth opens the native/default browser and returns immediately with `202`/`authenticating`; known non-OAuth methods return Settings/permission instructions, and unknown methods report missing setup information. Check progress through the existing server-information operation rather than a new authentication-status route. Per-wrapper implementation and complete error/status details remain to be specified.

See [api-contract.md](api-contract.md) for exact accepted paths, sample shapes, and remaining route-level questions.

## D12 — Original-harness cleanup

- Provide a prompt the user can copy/paste into their agent to remove the original MCP entries after migration.
- Automatic deletion of the user's original harness entries by Fetch was not selected.
- The exact prompt, ordering/checks before cleanup, and how the single Fetch connection is added to each harness still require design.
- Preserve the existing personal gateway/configurations during development and documentation work.

## D13 — Installation-group failure handling

Confirmed after the resumed setup-block proposal on October 26:

- A failed server installation does not stop preparation of the other selected servers.
- Keep successful installations.
- After the selected installations finish, report overall `failed` if any item failed, with per-server results.
- Guide the agent to resolve/retry only the failed entries through the existing installation endpoint.
- Continue to enforce the agreed overall-success prerequisite before the agent advances the dependent group setup flow.

Exact failure/error schemas, retry idempotency, concurrency, cancellation, and job retention/restart behavior remain open. This policy does not authorize adding rollback/history features.

## D14 — Configuration and server-scoped environment/credential writes

Confirmed on October 26 after approval of the two-route proposal and the user's clarification about environment variables/custom auth tokens:

- `PATCH /api/v1/servers/{server_id}/config` updates non-secret settings/references, such as arguments or working directory. Authorized setup agents and the interface may use it.
- Installation generates the server's stored Fetch configuration automatically from the approved discovered launch/package settings. The config PATCH edits that generated configuration; no separate configuration-creation route is needed for this setup flow. This concerns Fetch's stored configuration, not automatic rewriting of the original harness files.
- `PATCH /api/v1/servers/{server_id}/credentials` accepts manual values through Settings, saves them in encrypted storage, and returns names/status rather than values. Enforce Settings write authorization in the backend; ordinary agent setup access does not authorize this manual-value write.
- Reuse the credentials operation/store for per-MCP environment-variable entries, API keys, custom auth tokens, and related credential data. Do not add a separate environment-variable endpoint by default.
- Bind values to the configured server ID. Do not automatically share them with other server records or pass the full store to every wrapper.
- Configuration records carry references; values are resolved only for that server's permitted runtime/authentication needs. Agent-facing responses remain free of stored values.
- Trusted backend handling of fresh OAuth grants and refreshes may update the store automatically. The Settings-only API restriction is for external/manual credential entry; it does not disable automatic renewal.
- Read non-secret configuration and credential status through the existing server-information operation instead of a new read route.

Exact payload/entry types, deletion/update semantics, key protection, authorization implementation, custom-token binding, and behavior when a running wrapper's values change remain to be designed.

## D15 — Settings restart control, readiness, and a request-only force override

User-specified restart direction on October 26; D20 records the subsequent October 27 approval of its complete presented behavior:

- When Settings receives `restart_required`, automatically expose its Restart control.
- The user explicitly clarified that the Restart control triggers the POST. Do not restart automatically merely because a PATCH returned the flag.
- Clicking it sends the restart request. Return `202` immediately and keep the loading icon/bar active through waiting, stopping, starting, and readiness checking until a separate status result confirms completion. Acceptance alone must not appear as restart success. This latest user choice supersedes their earlier held-open POST selection.
- Reuse installed-server information for restart progress/completion. The user selected `GET /api/v1/servers?id=github&restart_id=restart-42` on October 26: `id` selects the server and `restart_id` selects the particular restart attempt. The user prefers explicit request monitoring as concurrent processes are considered in the future. Ordinary server-information requests can still omit `restart_id`. The earlier response-only placement suggestion was not selected; D20 approves the restart-specific response fields.
- Success requires a service-started result, not just starting to process the restart request. The user approved the `SERVICE_STARTED` wire code; D20 now approves the MCP/tool-list readiness check.
- For normal restart, wait for tool calls already running through Fetch to finish; temporarily reject new calls to that server until it is ready. The user explicitly approved this instead of queueing new calls. Exact error format remains to be specified.
- A temporary per-request `force_restart == true` override starts stopping/restarting immediately, without waiting for those running calls. The user explicitly reaffirmed immediate force behavior. It may interrupt active calls; it does not make startup instantaneous or waive readiness checking. Do not persist the force choice in the server's normal configuration. The optional JSON-body boolean/default is now approved as recorded below; termination behavior is now approved in D20.

Fetch tracks forwarded in-flight tool calls for the accepted normal wait, not whole assistant sessions. Whether to additionally track task handles/background work remains open for the execution step. The October 27 approval in D20 resolves the restart-specific envelope, selector validation, permissions, readiness, timeouts, overlap, error/retry and disconnection behavior. Exact ID representation, local job lifetime/retention, a progress percentage, ordinary tool concurrency and additional wrapper instances are not selected by this contract.

## D16 — Discuss the remaining outline in seven steps

On October 26 the user selected this sequence: (1) API contracts, (2) tool sharing, (3) execution behavior, (4) authentication, (5) implementation structure, (6) onboarding, and (7) testing/delivery. Continue step by step with bounded proposals, beginning with unfinished setup contracts. The sequence alone does not approve any new technical choice or authorize application code. [build-readiness.md](build-readiness.md) contains the current checklist.

## D17 — Approve restart fields and review whole contracts

On October 26 the user answered "Good" to the restart request/result format: accepted POST path, JSON-body optional boolean `force_restart` default false, `202` fields/instructions, nested `restarting` phases, `succeeded`/`SERVICE_STARTED` and `failed`/error fields, with matching loading/success/failure UI behavior. There are now eight accepted setup method/path pairs; full behavior is not all finalized.

The user then requested each complete API contract in one turn, rather than several turns for pieces of the same contract. Preserve the seven-step order, group all remaining meaningful clauses for each contract into one reviewable proposal, and avoid re-asking already approved details. The earlier "Good" approved fields only. The user later approved the remaining [restart contract](restart-contract.md) on October 27 as recorded in D20.

## D18 — Defer PostgreSQL and retention from the initial MVP

The user continued after the handoff on October 26: sideline retention because the initial MVP should have no PostgreSQL functionality. This supersedes initial-MVP PostgreSQL provisioning/connections, schemas/migrations, database-host Tailscale setup and the unapproved 24-hour PostgreSQL restart-result retention proposal. Do not select SQLite, JSON files, memory-only jobs or another storage design without input. Installed-server metadata and encrypted credentials still need local state; memory scope is not automatically cancelled. Accepted asynchronous progress/IDs remain. See [storage-scope.md](storage-scope.md).

## D19 — Local server files and the existing encrypted token store

The user selected local files for server configurations and the encrypted store already discussed for tokens. Save non-secret server IDs/launch settings/paths/package-version metadata and credential references in local config files. Store actual Fetch-managed tokens, refresh credentials, API keys and assigned environment values in the encrypted server-scoped store. Backend APIs own these reads/writes; interfaces do not bypass them. Existing Settings-only manual writes and automatic backend renewal remain intact. Harness clients still retain their own Fetch login credential.

This selected the storage split without choosing encoding/path/layout/key implementation. D26 later selects per-server encrypted credential configuration files and D28 selects their durable directory outside packages; filenames, encoding, atomic writes and key mechanism remain open. Runtime-job lifetime/shared-memory persistence remain unselected; D20 subsequently confirms the restart operational rules. PostgreSQL and retention stay deferred. No application implementation is authorized.

## D20 — Approve the remaining restart contract

On October 27, 2026 the user said "Cool all good next" after the complete remaining restart proposal and a clarification of stopping versus restarting. This approves the presented bundle in [restart-contract.md](restart-contract.md):

- Settings and authenticated management-authorized clients may restart/read status; validate the installed local target and body without echoing secrets.
- Capture saved launch settings/assigned references and use the pinned version; later changes await another restart.
- One active restart per server; overlapping requests, including force, return `409` with its ID. Different servers can restart independently.
- Normal restart waits up to 60 seconds for active calls. On drain timeout keep the old process running, reopen calls and fail; no automatic force escalation.
- Allow 5 seconds to confirm old-process exit, with normal shutdown or immediate forced termination as requested. Never start a duplicate. Interrupted calls fail without replay.
- Allow 30 seconds from replacement launch for compatible MCP communication and complete `tools/list`; allow an empty list and do not execute a tool as a health check. Keep provider authentication distinct.
- Stop/track a failed replacement, report failure and keep that server unavailable until corrected/retried; no automatic rollback or restart loop.
- Return the status URL in `Location`, poll once per second, and enforce the single-server/single-attempt filter. The approved envelope/errors distinguish successful status retrieval from the nested restart outcome.
- Closing UI/disconnecting does not cancel accepted work. Quit stops managed servers. No automatic POST retry; terminal requests may be retried as new attempts. IDs are references, not idempotency keys or credentials.

PostgreSQL, result retention and cross-Quit/crash recovery remain deferred. Exact ID representation, local operation-state lifetime, protocol/library versions and cleanup mechanisms remain open. This is API behavior approval, not implementation authorization. The next contract proposed for review is ordinary installed-server information.

## D21 — Move server routes outside setup

On October 27 the user asked to put `servers` under the base versioned route rather than `setup`. Keep the established `/api/v1` prefix and use `/api/v1/servers` for the existing server resource and its authentication/config/credential/restart operations. The assistant stated this exact path in the same turn; the user did not request removing the `/api` prefix.

- `GET /api/v1/servers`
- `POST /api/v1/servers/{server_id}/authenticate`
- `PATCH /api/v1/servers/{server_id}/config`
- `PATCH /api/v1/servers/{server_id}/credentials`
- `POST /api/v1/servers/{server_id}/restart`

This relocates the existing route family; IDs, query filters, permissions and approved behavior remain intact. Discovery and installation submission/progress stay under `/api/v1/setup`. The old `/api/v1/setup/servers` family is superseded in the design, not a second route family or a compatibility alias. No application endpoints exist to migrate. Rust handler/module placement remains a step-5 decision; moving URLs does not automatically select a new source tree. D23 subsequently approves the ordinary GET response; implementation is still paused.

## D22 — Shared next steps across current and future APIs

On October 27 the user asked to confirm that `next_steps` is standardized as an API return for all current and future routes, then accepted proceeding. Record this as a shared Fetch application API convention, rather than setup-only guidance:

- Every current/future application API JSON response includes `next_steps` for success, pending and error results.
- Use `[]` when no further action is needed. Non-empty entries retain `actor`, `action`, and plain-English `instructions`; endpoint-specific actions/target fields are reviewed with the complete route contract.
- Guidance reports an action; returning it does not execute that action or grant permission.
- This confirmation does not choose or rewrite future OAuth/MCP wire envelopes, redirect/stream/bodyless behavior or a new global `status` enum. How guidance fits protocol-constrained responses remains for their design; do not silently change standard protocol shapes.

## D23 — Approve ordinary server-information GET behavior

On October 27, after the complete existing GET response proposal and clarification of `restart_required`/shared next steps, the user said "Kk good to me then you’re good to move on." This approves [the ordinary GET contract](api-contract.md#approved-complete-record-and-ordinary-get-rules):

- The same full record for all/selected reads; safe saved config and credential-presence metadata, distinct runtime/authentication states, `restart_required` and active restart or null.
- Case-sensitive opaque IDs, sorted all-server reads, deduplicated first-requested order, and `200` matches/missing IDs even if all ordinary IDs are unknown.
- Settings/management permission, no process/login/provider/tool/write side effects, no-store responses and active-operation polling once per second.
- Shared success/error envelopes, ordinary request errors, safe per-server errors and targeted `next_steps`.
- The approved restart-filtered minimal view and D20 behavior remain intact. Saved versus applied settings explain why a ready process can still report `restart_required: true`.

Runtime/auth integration mechanisms, ID representation/storage/key handling, config/credential write schemas and protocol compatibility remain later work. No implementation is authorized. Continue step 1 with the complete authentication-start contract. A scheduling estimate of 20–30 more review turns was given for the full seven-step plan; it is not a deadline or a required endpoint count.

## D24 — Account for actual server auth and credential storage

On October 27 the user explicitly required the authentication contract to account for actual MCP-server authentication and credential storage, and asked whether servers have a standard way to return tokens. This confirms a design requirement, not the proposed 10-minute/overlap rules or a specific integration/store implementation.

Before claiming automatic login, persistence or refresh for an integration, define who receives/owns provider tokens, the authoritative store, how its running server gets credentials, who refreshes/replaces them, restart/reopen behavior, fresh-auth isolation and observable completion. Keep Fetch's login credentials separate from provider tokens. Preserve the existing encrypted per-server store for Fetch-managed secrets and Settings-only manual writes.

[Current research](research.md#actual-mcp-auth-and-token-ownership--rechecked-october-27) distinguishes HTTP OAuth token exchange, stdio environment provision and server-owned third-party login. No universal provider-token export or safe access to arbitrary wrapper caches is assumed. [The added integration requirements](api-contract.md#proposed-provider-authentication-and-credential-storage-contract) are proposed means of satisfying this scope requirement. Wrapper-owned storage compatibility, protocol capabilities and exact hooks/key mechanisms remain decisions for the user; no exceptions or new public routes are approved. D27 later accepts the reviewed general route policies; concrete integration choices and implementation remain open/paused.

## D25 — Authentication success requires stored-credential test

On October 27 the user required success only after a successful credential store and a test using the stored credential returns `2xx`. They clarified that the intended target is the service's authenticated test endpoint, not a route named lifeboat.

The credential-based authentication flow must save in the selected encrypted server-scoped store, read back the just-assigned stored credential, use it for that service-authenticated test and report success only after `2xx`. A callback-held token, public health check, tools/list response or credential presence is not a substitute. A failed store/read/request, timeout or non-`2xx` must not produce authentication success. Existing status GETs read the verified state without probing providers on every GET.

[The contract gate](api-contract.md#required-stored-credential-verification--d25) records this confirmed requirement. Already-authenticated POST success also requires testing the retrieved stored credential; no new browser login is needed if it works. Exact per-service harmless test endpoint/method, background recheck/refresh cadence, saved-but-unverified cleanup, native-permission-only verification and wrapper-owned storage integration remain to review. Server-owned third-party tokens must not be exported through MCP to meet the gate; no storage-policy exception is approved. D27 later accepts the previously reviewed general authentication timeout/overlap policies; implementation remains paused.

## D26 — Per-server encrypted credential configuration files

On October 27 the user said, "Everything else looks good. Just make sure you note credentials should be stored in the individual server level configs in encrypted files." Each managed server's credential configuration therefore belongs in that server's own encrypted file(s), rather than an undifferentiated shared plaintext/global config. Include its assigned API keys/environment/custom tokens and Fetch-managed OAuth material. Non-secret configuration can retain references; no secret values enter ordinary API/MCP responses.

This selects per-server encrypted credential-file organization. It does not select the file encoding, exact path/name, encryption/key mechanism or whether all non-secret settings share that encrypted file. Harness-to-Fetch issuer/key storage remains its separate auth-design question. Settings-only manual writes, backend-managed OAuth writes and D25 readback/test gating remain intact. No automatic exception for `gh`'s default credential store or another wrapper cache is approved.

The user then asked whether the encrypted file should live in `packages/servers/<server-id>/`. [The proposed placement](storage-scope.md#approved-server-data-placement--d28) keeps durable per-server config/secrets outside replaceable package installs. The user subsequently accepted that split in D28; it does not authorize creating runtime files.

## D27 — Accept reviewed general authentication behavior

The user's October 27 "Everything else looks good" accepts the previously presented general authentication contract, subject to the confirmed per-server encrypted-file requirement. Its validation/permission/installed-target rules, truthful OAuth/manual/unknown/already-authenticated branches, one active login with repeat reuse, 10-minute/provider-shortened deadline, safe errors, disconnection/Quit and restart-overlap behavior are accepted as reviewed, together with D24/D25 ownership and stored-credential verification requirements.

This does not approve unshown GitHub CLI commands, arbitrary shell access, a CLI-to-MCP adapter design, installation ownership, credential capture/renewal, OS-store/plaintext exceptions or a new HTTP API. The GitHub CLI preference is a proposal under current research, not completed integration support. Concrete provider integrations/test endpoints, background refresh/recheck/cleanup/non-token verification, crypto/key handling and implementation remain open. Keep the seven-step sequence and implementation pause.

## D28 — Separate durable server data from installed packages

On October 27, following the recommended outside-package tree, the user said "Sounds good, let’s do next steps." Save each server's durable configuration and encrypted credentials under `~/Library/Application Support/Fetch/servers/<server-id>/`. Keep downloaded software/dependencies under `packages/servers/` and shared runtimes under `packages/runtimes/`. Package replacement, reinstall and dependency cleanup must preserve the separate server data.

This approves the directory split and per-server organization. The example `config` / `credentials.enc` filenames remain illustrative; encoding, stable server/install ID mapping, safe-write implementation and encryption/key protection remain open. Uninstall/removal retention is not selected. The approval does not choose GitHub CLI operations/install/auth/renewal or grant application implementation permission. Continue step 1 with the existing configuration PATCH contract; tool scope and concrete auth mechanisms retain their later discussion steps. See [the storage layout](storage-scope.md#approved-server-data-placement--d28).

## D29 — Configuration API includes each individual server's settings

On October 27 the user clarified that the server configuration API should be how users and agents interact with the settings of the individual MCP servers themselves. The preceding args/working-directory/reference-only draft is not approved and does not yet cover that scope. Research whether a server's settings are launch flags, environment variables, server-specific files or documented runtime operations; do not assume a universal MCP config read/write operation or file format.

The user also raised agent exposure of API keys. Existing Settings-only secret input and no-secret API/MCP output boundaries remain in force. The shown environment name/reference is not a credential value: the trusted backend, not the agent, must retrieve and supply assigned credentials through the supported private integration. This does not approve arbitrary paths/raw file edits, unrestricted secret-reference rebinding, live reload, a concrete settings schema or exceptions to the Settings-only environment/credential-write rule. Revise and re-present the complete route after the necessary research; application implementation stays paused.

## D30 — Bound MVP config to common launch fields and approve its contract

On October 27, after research into differing native MCP settings, the user said to leave their standardization outside MVP, keep "the standardized fields we're handling," and "everything else looks good, feel free to move on." Defer D29's broader individual-server config normalization, raw/native config mappings, settings discovery/schemas and live-setting management to a separate future scope. These shared fields are Fetch's contract, not a universal MCP settings standard.

The MVP configuration PATCH retains exactly `args`, `working_directory` and `environment` name/credential-reference bindings. This accepts the previously presented complete launch-field contract: omitted fields unchanged, list replacement/default-directory/no-op rules, protected launch prefix and safe arguments, server-local refs/no values, authorized Settings/management agents, auth-active rejection and restart-active edits, serialized safe saves, saved-versus-applied restart tracking, actual-edit auth-evidence invalidation/stale-result protection, no automatic process/login/provider action, response/guidance/errors and uncertain-save readback. See [the approved contract](api-contract.md#approved-common-launch-configuration-patch-contract).

This does not choose per-server native settings schemas, broaden manual environment/credential write authority, select secret handoff/crypto/atomic-write/runtime mechanisms or authorize application implementation. Supported safe launch forms still require implementation review. Preserve D25 stored-credential verification, per-server encrypted files and D28 directory split. Continue step 1 with the existing Settings-only credentials PATCH; discovery/install contracts remain afterward.

## D31 — Credential-save success also verifies; share auth failure code

On October 27 the user required successful credentials PATCH `200` responses to also test the endpoint using the newly stored credential and return a separate code when the test fails. They then asked whether the earlier authenticate endpoint already performs that test and required the same failure code there, followed by "after this you're good to move on."

D25 already gates authentication success on encrypted store/readback and a service-authenticated `2xx`. Apply the new credential-save `200` gate too: save encrypted data, retrieve it through the real store interface, test using that new credential and only then return `200`. A running server's old credential, callback-held input, public health or generic tool listing cannot prove it. Verification failure uses shared `CREDENTIAL_VERIFICATION_FAILED` across the credential-save error, synchronous authenticate error and async auth GET error. Keep storage failure distinct; when a test fails after a successful store, tell callers the storage outcome truthfully without values.

The user authorizes moving on from the reviewed base manual-credentials contract with this correction; this does not approve unshown endpoint/handoff/restart/helper mechanisms, verification deadline, failure HTTP mapping, batch/deletion/no-auth exceptions or rejected-value retention/rollback. The draft's additional success/storage-outcome fields remain proposal refinements. No unverified `200` exception is selected. Reconcile these supported-integration/error details before implementation in the authentication/implementation steps. Continue step 1 with the existing installation-progress GET; remaining individual-route order was not selected, and the seven-step group order remains intact. Application implementation remains paused.

## D32 — UI setup is the default; next_steps guides agents only

On October 27 the user clarified that installing without an agent is Fetch's base UI setup method. The UI must depend on installation-progress status/results and deterministic post-installation sequencing rather than `next_steps`, which is for agents only. Connected-agent assistance is an optional way to use the same APIs, not a prerequisite for initial installation/setup.

Keep D22: every Fetch application API JSON response still includes `next_steps`, including responses consumed by the UI. Its actor/action/instructions are agent-facing guidance. `actor: user` describes a human action for an agent to relay; it is not an executable UI step. The UI does not parse instructions/action strings, use their presence/order to advance setup, or use them as its source for controls/status. It renders structured states/results/errors and its own deterministic presentation/sequence.

Installation progress is the installation-state source. After successful preparation, the UI uses the resulting server IDs and existing server GET's auth/config/runtime/restart fields to determine which approved controls/steps are applicable. Backend APIs retain prerequisite checks, permissions, validation, persistence and process control; the UI sequences API calls without owning a second implementation of those rules. No automatic login, restart, failed-install retry or harness-config edits are approved by this clarification.

Exact UI states/screens/post-install branches and first-setup Settings authority remain for the selected onboarding/auth steps. This does not approve the still-pending complete installation-progress contract, a new UI instruction field/schema, a route, a changed API envelope or application implementation. Verify the base flow without a connected agent and verify that changing valid agent guidance alone does not change UI status/sequence.

## D33 — App sequencing and agent guidance describe the same workflow

On October 27 the user required `next_steps` to match what the app is built to do during a sequence, and vice versa, including all previously discussed steps. Apply this to every current/future Fetch application workflow, not just post-install setup.

The UI and agent guidance must agree on applicable prerequisites, allowed next operations/order, pending/completion conditions, failure handling, retry limits and human handoffs. Both follow the same reviewed API/backend rules; changing a workflow requires updating the applicable UI sequence and agent guidance together. Do not maintain contradictory policies for the two clients.

D32 still applies: the app uses structured API state and its deterministic native sequence, not parsing/executing `next_steps` prose/action strings. D22 retains the common field for agents. This is consistency of behavior, not UI control by guidance. Permission differences remain deliberate and explicit: the UI's Settings can accept values while an agent must hand that stage to the user, never request secret-valued tool input or bypass backend authority.

[The cross-route alignment table](api-contract.md#workflow-alignment-across-all-existing-routes--d33) covers discovery, submission, installation progress, server inspection, authentication, config, credentials and restart/named monitoring. It preserves each route's existing approval status and does not approve pending schema/lifetime/verifier/error choices. Update and verify the corresponding guidance/UI behavior together as those choices are resolved. No workflow-engine library, generic UI instruction schema, new route, automatic action or implementation authorization is selected.

## D34 — Approve installation submission and source/reuse rules

On October 27 the user replied "yep good" after the complete installation-submission contract and its explicit source-capture/reuse tradeoffs. Approve the presented existing POST contract: selected `{config_path, server_name}` array; malformed/empty/exact-duplicate request rejection before acceptance; Settings/authorized setup callers; one active group globally, sequential entries, active-ID `409`; immediate `202`/group monitoring; per-item source/package failure after acceptance with other items continuing and successes retained; generated config/private pinned preparation; failed-entry retries; safe request errors and basic close/Quit handling.

New/failed source entries capture their current definition when preparation starts; there is no discovery-snapshot guarantee. Existing successful Fetch source associations reuse pinned software/settings/credentials rather than importing changed source definitions. Different harness entries remain distinct even when packages match. Installation prepares/saves only; no automatic ordinary-server launch/authentication or original harness edits/cleanup. See [the approved contract](api-contract.md#approved-installation-submission-contract).

This does not select source canonicalization/alias/ID encoding, supported preparer internals, exact finite deadlines/termination/cleanup, full same-run terminal-result retention or cross-launch recovery; those remain explicit later/progress-review choices. It also does not approve separately pending full progress GET schema/errors/lifetime or application implementation. Next complete route proposal is discovery GET, with current harness documentation rechecked before dependent planning. D32 UI-first setup and D33 app/guidance parity remain in force.

## D35 — Approve the discovery contract

On October 27 the user said "good next" after the full discovery contract. Approve the presented existing GET: authorized Settings/setup callers; optional single absolute file; selected default Claude/Codex/OpenCode candidates and Codex fallback; independent OpenCode JSON/JSONC display with warning; compatible top-level container recognition without active-layer reconstruction; explicit single-file input outside default locations; missing/default-versus-explicit error behavior; safe file groups/server metadata; supported/unsupported/invalid eligibility; observed variable/source flags; source-disabled unchecked-by-default but explicit opt-in; distinct entries/order; safe errors/no-store/no execution/secret resolution/source mutation.

The scan's metadata and selection flags drive native UI and matching agent guidance under D32/D33. Discovery does not freeze sources or create snapshot tokens; D34’s installation capture/reuse rules remain intact. See [the approved discovery contract](api-contract.md#approved-discovery-contract).

Approval does not choose reader/preparer releases, exact safe-launch/argument grammar, path canonicalization/identity or finite read/deadline/cleanup/key implementations, and does not authorize application code. Separately pending progress-GET projection/request errors/guidance/current-run lifetime still need a decision; finalize that remaining read contract before moving to step 2 tool sharing. Concrete credential-verifier policies remain in the selected auth/implementation review.

## D36 — Approve progress read and advance to tool sharing

On October 27 the user said "good" after the complete final progress-read contract. Approve the existing GET's always-present group ID/status/ordered server results/next_steps; group installing/succeeded/failed and item pending/installing/succeeded/failed; safe item errors and real installed ID before success; HTTP200 retrieval distinct from job outcome; caller/ID/query validation and request error codes; no-store/max once-per-second active polling/safe GET retry; matching structured UI/agent guidance; and result availability throughout the current Fetch run, including UI/background closure. There is no recovery promise after Quit/crash/relaunch.

D34 installer/source/retry/basic Quit behavior remains unchanged. See [the approved progress contract](api-contract.md#approved-installation-progress-contract). This selects observable same-run lookup, not a database/RAM/file implementation or historical retention feature. Generic job-state mechanisms and older restart-result lifetime choices are not silently selected by the installation-specific rule.

The initial setup/server-management contract review is complete enough to advance to selected step 2: tool sharing/listing/calling/MCP mapping. Concrete D31 verifier/error/batch/removal cases and Settings authority remain for the selected authentication step; reader/preparer/identity/path/deadline/process-cleanup/storage/key mechanisms remain later implementation choices. No automatic actions, new tool routes, tool-name scheme or application implementation are authorized by this approval.

## D37 — Preserve complete raw Tool objects

On October 27 the user asked whether the eight discussed fields were universal, rejected a six-field whitelist because it could discard crucial metadata, and selected retaining raw Tool JSON with only duplicate-name rejection and avoiding incomplete lists. Preserve every received property and nested value, including icons, `_meta`, version-specific `execution` and unknown extension fields. Do not drop/normalize/redact/rewrite Tool contents or add schema/content sanitization. Reject duplicate tool names within a server and publish its tools only after collecting the complete list; identical names on different servers remain distinct by origin.

This supersedes the proposed six-field projection, extra metadata validators and `TOOL_METADATA_UNSAFE` filtering in the [listing contract](api-contract.md#approved-installed-tool-listing-contract). "Raw" preserves JSON content rather than original whitespace/key ordering. Caller authorization and protocol transport/collection remain; Fetch does not insert its credentials, fetch icons or execute metadata simply by passing it through. Preserving an extension field does not implement that extension.

[Current primary-source research](research.md#tool-json-preservation-and-version-differences--october-27) establishes eight current core fields (two required/six optional), with optional `execution` additionally present in 2025-11-25. The user approval concerns preservation and the two checks, not blanket approval of GET path/query/deadline/other listing policies, protocol-version support, invocation or application implementation.

## D38 — Approve the revised installed-tool listing contract

On October 27 the user said "sounds good, next" after the complete tool-listing contract and subsequent raw-Tool clarification. Accept the presented GET `/api/v1/tools`, optional repeated `server_id` filters/caller visibility and duplicate handling, live metadata collection on existing usable connections, complete pagination with 10-second total per-server deadline, truthful per-origin availability/empty/error outcomes, origin plus exact upstream-name identity, common response/next_steps and no-store/no automatic retry/lifecycle/login/invocation behavior. D37 supersedes the earlier six-field projection and unsafe/schema/content filtering: preserve complete Tool JSON with only duplicate-name rejection and complete-list handling as Tool-list validation.

See [the approved listing contract](api-contract.md#approved-installed-tool-listing-contract). The approval follows the presented user-facing contract as revised; it does not approve additional unshown policies, a supported protocol/SDK/library, MCP aliases/endpoint, invocation, new workers/concurrency/session sharing, provider authentication or application implementation. Exact mechanisms and any unshown normative refinements must remain explicit later review. Advance within step 2 to the next complete installed-tool invocation proposal.

## D39 — Forward model arguments and return the raw MCP tool result

On October 27 the user requested the same raw input format as the base MCP and raw MCP response in `result`, then clarified: Fetch should accept the raw model-supplied tool arguments and act as a middleman. Select that arguments-only JSON object, rather than requiring the model to add server/name/arguments routing fields. Preserve the individual tool's input schema and every argument key/nested value without defaults, reconstruction or coercion. Target server/tool selection is outside the model argument object. The backend MCP client supplies normal `tools/call`/JSON-RPC framing internally with the unchanged object as `params.arguments`.

Select complete raw terminal MCP ToolResult preservation in `result`, including unknown/nested fields, content and structured data. Fetch status/error/guidance stays outside that raw result; full outer HTTP shape remains proposed. JSON content is preserved rather than whitespace/key order or the hop-specific JSON-RPC request/response ID. Retained data does not implement extensions or override Fetch authority, and the backend does not inject its stored credentials into arguments/results.

The exact HTTP path is still proposed: `POST /api/v1/servers/{server_id}/tools/{tool_name}/call`, body containing only tool arguments. This supersedes the earlier proposed `POST /api/v1/tools/call` body with Fetch `server_id`/`name`/`arguments`, and the intermediate standard-MCP-params-body interpretation. The user clarified arguments-only, so that interpretation is resolved; do not reintroduce a custom model input wrapper. Lossless path/name mapping, MCP exposed aliases, outer response/errors/authority/lifecycle and remaining step3 execution policies still require review; this is not whole-call-route approval or implementation authorization.

The user subsequently reaffirmed that `{"query":"example"}` itself must be the POST body and referred to forwarding to the MCP by ID, then asked to continue. This confirms D39 direct arguments and target-server identity outside the body; it does not select the exact unshown URL or silently approve the remaining outer response/lifecycle policies. Present the revised complete call contract next without reopening the raw body/result decision.

## D40 — Disclose Fetch's response wrapper to the model

On October 27 the user required that the LLM be made aware that Fetch wraps the raw MCP response. Select an explicit model-visible Fetch-owned explanation: outer status/execution/error/next_steps belong to Fetch; `result` contains the complete unchanged upstream terminal MCP ToolResult, or is `null` when no terminal result was obtained. Do not allow a model to mistake HTTP200/outer forwarding status for tool success or an absent result for an empty upstream response.

The notice must reach the model through the supported client's actual tool context/content. Internal docs, UI-only wording or metadata that the host does not expose is insufficient. Keep the explanation outside the preserved upstream definition/description/schema and raw `result`, retaining D37/D39 and the raw argument body. This requirement does not select an automatic description/schema rewrite.

Possible mechanisms—Fetch-owned model context, explicit outer model-visible result text and/or a response-format marker—remain proposals, including exact wording/field names/channel/versioning. Review MCP wire presentation and outputSchema compatibility before claiming the application HTTP envelope can simply be returned as native MCP structuredContent. Actual delivery in supported clients must be verified before claiming compliance; do not assume `_meta` alone is model-visible. See [research](research.md#model-visible-fetch-wrapper-disclosure--october-27) and [the call-contract requirement](api-contract.md#fetch-explanations-through-next_steps--d43d44).

This accepts disclosure, not the entire pending invocation URL/outer response/error/lifecycle contract, MCP aliases/capabilities or application implementation. Present the disclosure wording/example as a proposal alongside the later MCP mapping review.

## D41 — Approve response_context and separate MCP notice text

On October 27 the user said "good" then "next" after the proposed top-level HTTP `response_context` beside `result` and separate model-visible MCP text rendering. Select that string field for the wrapped tool-call response, explaining Fetch's outer fields and the unchanged upstream `result`; retain common `next_steps`. Render the explanation as a separate Fetch-owned text content block for MCP callers without inserting it into the preserved upstream ToolResult or changing model argument objects. This makes D40's disclosure requirement concrete.

See [the call disclosure contract](api-contract.md#fetch-explanations-through-next_steps--d43d44). The approval concerns the shown notice placement and separate rendering; it does not approve the complete call URL/outer error/lifecycle contract, a full HTTP-to-MCP structuredContent mapping, derived outputSchema, JSON fallback/media duplication, MCP endpoint/aliases/capabilities or application implementation. Exact wording may explain the same reviewed fields/outcomes; no new versioning/marker field is silently selected. Actual supported-client visibility still needs verification after implementation authorization. Advance within step2 to a complete MCP-facing presentation proposal.

## D42 — Keep native MCP results and add one context block

On October 27 the user rejected sending the response three times and requested retaining the raw MCP response with Fetch's context text, without additional content fields to distinguish images/audio. Select native MCP terminal-result presentation: retain the original ToolResult fields, existing content blocks once in their relative order, structuredContent and isError, and add exactly one Fetch-owned text context block to the existing outgoing content array. Existing block types already carry native media; do not create duplicate media blocks or full-envelope serialized fallback. Keep the original outputSchema rather than a derived Fetch wrapper schema.

The HTTP application response still retains top-level response_context and the unchanged raw upstream result. Build the MCP presentation separately so adding its notice cannot mutate that HTTP result. The MCP notice must describe native presentation, not pretend the full HTTP envelope/nested raw result/status fields exist there. Fetch-specific guidance, when selected, belongs to Fetch-owned context outside upstream data. Original input schemas/model argument objects and canonical Tool definitions remain unchanged. Preserved metadata does not activate extensions or override Fetch authority.

This supersedes the unapproved full-HTTP-envelope-in-MCP structuredContent, full serialized fallback, repeated native copies and derived outputSchema proposal. See [selected presentation](mcp-presentation.md). Exact Fetch-only/no-terminal-result failure presentation, native authentication/protocol compatibility and remaining call/endpoint/alias/capability/execution contracts remain unselected. Actual supported-host/model/media delivery still requires testing; no implementation is authorized.

## D43 — Approve Fetch-owned failure text and raw upstream results

On October 27 the user clarified that Fetch-owned failures should return text blocks detailing the issue, while an upstream tool return should be the raw result for the agent to handle; "other than that good :D" accepts the presented failure mapping with this correction. Return upstream terminal ToolResults, including isError errors, unchanged with no Fetch-added notice, explanation, guidance, copied media or schema replacement. This supersedes the one-notice addition in D42; original HTTP result and Tool/input/output schemas stay intact.

Approve the reviewed Fetch-only operational failure split: safe Fetch-labelled text, isError true, known non-dispatch not_started versus possible-dispatch unknown, no synthetic upstream result or structuredContent, no automatic retry/login/restart or effect/rollback guarantee. HTTP result stays null when no upstream terminal result exists. Authentication/authorization, malformed requests, unknown aliases and upstream JSON-RPC errors keep native safe protocol handling. Exact protocol codes/version adapters, SDK/host compatibility, ordinary timers/cancellation/drain accounting and remaining MCP/call surface stay open.

The requirement to identify Fetch-owned data now applies to Fetch-owned explanations, not to added notices on raw upstream MCP results. See [selected presentation](mcp-presentation.md). Approval is not application implementation authorization.

## D44 — Reuse next_steps for Fetch explanations

On October 27 the user asked whether tool-return explanations could use "next turns"; the clarification answer explicitly selected "Put Fetch explanations in next_steps." Reuse the accepted application JSON next_steps field and actor/action/instructions structure. Put Fetch's issue explanations and applicable guidance in those instructions; remove the separate response_context field and do not add a replacement context field. This supersedes D41's separate-field choice.

HTTP still keeps the raw upstream result and its existing structured status/error information. For MCP, render Fetch-owned failure guidance as clear text blocks; do not add it to or rewrite an upstream R under D43. Ordinary upstream success/error is left for the agent to interpret; no needless explanatory entries are required and next_steps may be []. Keep backend permission enforcement and Settings-only human handoffs. Reuse does not authorize custom next_steps fields in OAuth/MCP protocol envelopes or automatic follow-up actions.

Exact native protocol framing/host compatibility and remaining call/endpoint/alias/capability/execution contracts stay open; no code is authorized. See [explanation source](mcp-presentation.md#existing-next_steps-is-the-explanation-source--d44).

## D45 — Approve stable server-prefixed MCP tool names

On October 27 the user said "yes this works for me" to the complete naming policy. Select a stable server prefix for every imported tool, including currently unique names; permanent tags tied to stable Fetch server IDs; an exact alias-to-server/original-name mapping; fetch__ reserved for Fetch-owned tools; generated collision-checked aliases for long/incompatible names; no silent retargeting or reassignment of old aliases; and safe unknown/unauthorized-alias failure without forwarding. Display renames, refreshes and ordinary restarts retain aliases within preserved Fetch data.

Only the MCP-exported name changes. The canonical HTTP Tool objects retain original names/fields, and descriptions, input/output schemas, model arguments and upstream results stay unchanged. Namespace ownership grants no permission or access to Settings-only secret writes. See [approved naming policy](mcp-tool-names.md).

Examples mcp_s1__search/mcp_s2__search illustrate the reviewed prefix policy rather than actual allocated tags. Exact token/fallback encoding, portable character/length policy and supported-host tests, persistence/file/coordination mechanisms and actual builtin names remain explicit review before implementation. No numeric host limit, protocol/SDK, endpoint/transport or execution policy is inferred. Continue within step2 to the complete localhost MCP connection contract; no application implementation is authorized.

## D46 — Isolate new MCP connection tests from the user's environment

On October27 the user required virtual computers/environments for new MCP connection tests without interfering with personal user state, and explicitly asked to add the rule to AGENTS.md. Record it in global and project instructions plus NOTES.md: use a VM/virtual computer or suitably isolated test environment protecting real files, credentials, personal harness/MCP configurations, apps and services, with disposable fixtures and dedicated test credentials. Dependency-only isolation is not enough when an MCP can access host files/credentials/desktop apps.

This selects the testing boundary, not a virtualization vendor, VM image, container design or provisioning flow. The environment/testing plan must be chosen before live tests and does not revoke the application implementation/test pause. The user's separate global notes-file default is also recorded; Fetch reuses existing NOTES.md rather than creating a duplicate.

## D47 — Approve the presented localhost MCP connection contract

On October27 the user said "okok cool you're good" after the complete connection proposal and clarifications about dated MCP revisions, shared localhost API port, tools/call name routing and tools/list discovery. Accept the shown single loopback /mcp endpoint in the existing Fetch process/listener, Streamable HTTP, both 2025-11-25 and 2026-07-28 as compatibility targets, revision-correct setup/POST/GET/DELETE/notification behavior, per-request bearer/permissions and local Host/Origin validation, temporary legacy sessions rather than modern initialization sessions, and no call replay/provider startup/login/restart or stopped-effect guarantee on reconnect.

The public application APIs remain /api/v1/... on the same configured port. MCP clients send tools/list or tools/call to POST /mcp and handle protocol framing; Fetch resolves D45 aliases and forwards unchanged model arguments through backend APIs. Raw upstream results and Fetch-owned next_steps explanations retain D43/D44 behavior.

See [approved connection policy](mcp-connection.md). Actual supported-client releases/SDK compatibility must be tested under D46 isolation. The numeric port, unshown status/header/session/bind-conflict mechanisms, exact OAuth routes/scopes/HTTPS issuer/TLS/trust and ordinary execution/cancellation/drain mechanisms remain selected later review before implementation. No same-port plaintext OAuth issuer is assumed. This approval does not silently select unshown policies or authorize building. Current next step2 contract is Fetch-owned management-tool names/inputs/API mappings; extra protocol capabilities and the GitHub CLI proposal remain open.

## D48 — Give application API operations descriptive MCP tools

On October27 the user said "i think all apis should get tools that describe what they do." Make MCP tool coverage a standing requirement for current and future Fetch product API operations, rather than treating the eight-tool proposal as a closed permanent inventory. Each agent-accessible operation must have a matching tool with a clear description of what it reads/changes, required/optional inputs, results/pending states, permissions and relevant user handoffs. Both interfaces use the same backend rules and caller authority.

Preserve the earlier explicit Settings-only manual credential/value boundary: this general instruction does not explicitly revoke that exception or authorize secret-valued MCP inputs. OAuth/protocol endpoints retain their native roles; this does not turn token exchange, callbacks or POST /mcp into recursively exposed product tools. Imported-tool calls already expose their API operation through the individual installed tools, whose original descriptions/schemas/arguments/results remain preserved; do not rewrite them or add a second generic caller silently.

The same coverage rule applies to future selected product APIs, including memory operations if included, without deciding new routes or memory release scope now. Concrete tool names/input/result mapping in [management-tool proposal](mcp-management-tools.md) still await input; the user approved the general coverage/description requirement, not every unaddressed clause. Application implementation remains paused.

## D49 — Approve the presented Fetch-owned management-tool mapping

On October27, after the complete management-tool proposal and D48 descriptive-API coverage amendment, the user said "you're good with that though next." Accept the presented eight names/inputs/existing API mappings, unchanged backend defaults/caller permissions, Settings-only manual credential exception, single text block containing complete API JSON/next_steps without duplicate structuredContent/outputSchema, 2xx/pending/read versus application/native error handling and no automatic chaining/poll/retry. D48 keeps the inventory open to tools for subsequently selected APIs.

See [management-tool contract](mcp-management-tools.md). Approval covers the clauses presented together and the accepted D48 amendment; it does not choose exact OAuth scopes/Settings proof, SDK/generated-schema implementation, future memory route/release scope, GitHub CLI tools or protocol capabilities. Imported raw definitions/arguments/results remain unchanged. The remaining installed-tool HTTP URL/outer response/error/lifecycle proposal is next within step2. No application implementation is authorized.

## D50 — Approve the presented installed-tool HTTP call contract

On October27 the user replied "godo" to the complete remaining HTTP tool-call contract. Approve POST /api/v1/servers/{server_id}/tools/{tool_name}/call with exact/lossless selectors outside the raw argument-object body, one call/object with the presented shape errors, invocation-specific permission and hidden-server handling, existing connection/one forwarding/no preflight discovery/login/start/restart, and no auth-verification promotion from call success.

Approve the shown no-store outer fields server_id/name/status/execution/result/error/next_steps. A terminal upstream ToolResult uses HTTP200 and completed execution, with status failed for raw isError true and otherwise succeeded. Raw MCP result remains intact; Fetch failures have null result, safe error and guidance. Preserve not_started versus unknown, all presented 400/401/403/404/409/502/503/504/500 codes and unsupported-interaction 409-before/502-after handling. No202/background call job/status route, automatic replay, false cancellation/rollback/drain guarantee or extra upstream notice is introduced.

See [approved presented call contract](api-contract.md#approved-installed-tool-invocation-contract--d50). D50 approves the clauses presented together, not unshown numeric execution timers, concurrency/queues/workers/session sharing/cancellation mechanisms, full MVP capability support, SDK/framework/alias implementation, OAuth/key mechanisms or building. These ordinary execution choices are the next bundled review; exact unsupported-capability scope and GitHub CLI adoption remain explicit pending decisions.

## D51 — Queue tool calls and coordinate concurrency

On October27 the user corrected the proposed execution policy: "make sure calls get added to queues, including handling for concurrency." Queue calls and coordinate their dispatch with concurrency limits rather than rejecting an ordinary call merely because another call is active. This supersedes the assistant's unapproved no-queue/immediate-busy proposal. Exact per-server/global scheduler scope, FIFO/fairness, concurrent/waiting limits, memory bounds, cancellation/dispatch/deadline/restart behavior and settings/read schemas still need the complete revised proposal and user input.

The user then questioned the unapproved fixed120-second limit and asked whether Fetch can read individual tool time limits. This is a question, not approval of unlimited waits or a new fallback. The assistant withdraws its own fixed120 proposal pending a configurable, tool-aware policy review. Current and legacy MCP Tool schemas have no standard discoverable whole-call timeout; arbitrary timeout arguments/custom metadata must not be interpreted or rewritten generically. Known documented integration hints/settings could inform a reviewed policy. Queue wait and dispatched execution timing remain separate decisions.

See [revised execution work](mcp-execution.md) and [research](research.md#queued-execution-and-tool-timeout-discovery--october27). D50's route/raw input/raw result/no durable call job/no replay and D20 restart rules remain selected; queue admission/restart refinements are proposed, not silently approved. Application implementation stays paused.

## D52 — Persist per-tool features in each server's configuration and apply them on calls

On October27 the user requested an installation-config list of tool features such as timeout limits and application of those features on MCP calls. Keep non-secret, server/tool-scoped feature/settings records in the durable per-server configuration selected by D28, outside replaceable packages. Use that recorded information when handling the matching tool call; do not assume MCP universally exports timeouts or other execution limits.

Earlier D39/D50 raw-argument preservation still applies: use Fetch-owned queue/request execution options, not injected extra fields/defaults in the model's raw argument object or fabricated upstream metadata. Known explicitly supported/documented limits may inform Fetch's policy; unknown stays unknown. Exact profile schema/provenance/refresh/override behavior, configuration API extension and initialization-versus-first-discovery sequencing are the revised proposal, not silently finalized. In particular, D34 installation does not automatically start/authenticate a server to discover its tools, and D38 live-listing semantics/raw definitions remain selected. Manual secrets stay in encrypted Settings-only storage; feature/config settings contain no provider credentials.

## D53 — Use120 seconds as the default tool-call fallback

On October27 the user then said "and yeah default to120 seconds, and keep going after." Select120 seconds as the fallback Fetch call-wait duration for a tool without a selected more-specific limit, not as a universal maximum overriding known/configured longer tool limits. Keep queue-wait time separate from execution time; numeric queue-wait defaults and exact deadline start/override precedence/response allowance remain for the complete revised contract.

This resolves the fallback value left open in D51; it does not approve every earlier execution proposal, unlimited waits, arbitrary metadata inference, exact queue/concurrency/settings/GET schemas or application implementation. Continue with the full revised queued execution proposal rather than stopping at confirmation.

## D54 — Give every managed MCP server a configurable default timeout

On October27 the user said "lets make sure all mcps have a configurable default timeout setting." Every managed MCP server has its own durable configurable Fetch default call-wait timeout, initially120 seconds under D53. Use the existing server configuration API/Settings and its corresponding Fetch management tool, subject to their normal caller permissions; this is a non-secret execution setting rather than a Settings-only credential value.

The presented execution.default_timeout_ms represents this setting in milliseconds (initial120000). It is the server fallback when no applicable more-specific per-tool limit applies, not a universal maximum. Keep per-tool settings and raw argument/result preservation; Fetch enforces its own request options without requiring third-party native timeout support or inserting fields into raw tool arguments.

This confirms the per-server configurability requirement and initial value. Exact numeric bounds/validation, other execution members, map/metadata precedence refinements, hot-application/queued-request snapshots, queue limits/GET schema and restart refinements remain proposed in [the complete execution contract](mcp-execution.md). Do not treat this single-item feedback as approval of the entire queued-execution block or application implementation.

## D55 — Initialize timeout on installation and apply it as the call deadline

On October27 the user clarified that the configurable timeout "gets injected as the execution deadline and can be set automatically by the mcp install." Installation initializes the durable Fetch timeout setting automatically: use a supported explicit server-default value when available under the reviewed installation source policy, otherwise the selected120-second default. Do not derive a server-wide default from an arbitrary per-tool timeout or overwrite an existing user setting during D34's successful-install reuse.

Store a duration, not a timestamp that expires after installation. When the matching call leaves its queue for dispatch, Fetch computes a fresh execution deadline from its effective timeout and applies it as its own scheduler/request option. Queue waiting remains separate. D39/D50 raw arguments remain unchanged: no generic deadline/timeout field is inserted into params.arguments or fabricated MCP protocol metadata. No universal upstream deadline field or automatic hard stop of external work is claimed.

This selects automatic installation initialization and per-call deadline application, not universal timeout discovery. Respect D34/D52: do not start/authenticate a server merely to inspect it at install; unknown tool features populate only after authorized discovery. Exact supported source fields/precedence, per-tool grace refinements, SDK clock representation and remaining queued/settings schemas stay their explicit review topics before implementation. Other proposed queue defaults/hot-apply/restart refinements and application implementation are not approved by this clarification.

## D56 — Preserve configurable processes per MCP and queue distribution

On October27 the user explicitly asked to note that they still want concurrency, "maybe not in the mvp," with configurable total concurrent processes per MCP and queuing distributed among those processes according to capacity. Preserve this as a confirmed desired capability: each logical managed MCP can have a configurable process-count limit, with queued calls assigned among its running processes and waiting when relevant capacity is occupied.

Process concurrency is distinct from simultaneous calls within one process/connection. The previously proposed max_concurrent_calls must not silently serve as a process-count setting. D51 queue/concurrency coordination remains selected; D56 does not defer all queuing or call concurrency from MVP. The user left process-level MVP inclusion undecided, not definitely in or definitely out.

Exact setting/API/tool fields, defaults/count bounds, process launch/lifecycle, shared versus per-process queues, distribution/fairness, state/session affinity across multi-step tools, credential/process-state sharing, cancellation/restart accounting and implementation/test scope remain for explicit review. Do not choose round robin, autoscaling, a worker runtime or process isolation as if approved. D56 does not authorize extra on-demand workers on D50's current call path, provisioning, installation or application implementation. See [execution direction](mcp-execution.md#desired-multiple-processes-per-mcp--d56).

## D57 — Defer multiple processes per MCP for now

On October27, after D56 preserved the desired process-count/queue-distribution feature, the user said "i dont want it yet, kep going pls." Defer configuring/running multiple Fetch-managed instances of a logical MCP for now and retain it on the future-feature list. Use one managed MCP process/connection per logical installed server in the current MVP design; do not create process-count APIs, pooled instances or cross-process dispatch now.

This is not cancellation of D56's desired feature and not deferral of D51 queuing or call-level concurrency within a process. Exact existing queue/concurrency/default/schema/hot-apply/restart proposals remain unapproved except requirements explicitly selected in D51–D55. Continue with the next bounded architecture review, preserving those open items before implementation. No application implementation is authorized.

## D58 — Make real Claude Code and OpenCode use the MVP acceptance bar

On October27 the user clarified that they want to test Fetch directly through their everyday Claude Code and OpenCode workflows. Choose initial capability coverage from those real workflows/servers rather than treating every optional MCP feature as unnecessary. The assistant's blanket initial exclusions were never approved and are now on hold pending the actual inventory/compatibility assessment. Preserve the existing Codex target unless the user changes it; Claude Code/OpenCode are the stated everyday-use priorities.

The user then explicitly asked to inspect the already available MCP inventory instead of asking them to relist it. Root inspected only exposed tool declarations and a redacted config inventory (server names, transport/command kinds, enabled flags and loopback-versus-nonloopback classification). No credential values/URLs/arguments were output, no configured server was called, and no client configuration changed. [The practical audit](real-use-compatibility.md) records actual source coverage and unknowns.

D58 is an acceptance requirement, not blanket adoption of resources/prompts/interactions/tasks, remote upstream transports, unshown queue parameters or implementation. The inventory exposes a concrete gap: four Claude global entries use nonloopback HTTP/SSE, currently unsupported by D35's local-child-only scope. Recommend reviewing remote-upstream support while keeping Fetch client-facing localhost; do not silently amend D35 or reconnect personal clients. Preserve D46 isolated connection testing first; later everyday use requires explicit rollout/connection authorization.

## D59 — Share MCP handlers behind the existing endpoint

On October27 the user asked whether the additional protocol functions can all be handled by POST /mcp and said they are fine with individual shared functions for each functionality, invoked by the runner depending on the result. Accept that internal organization: one native MCP entry point, shared backend handlers for the selected functions, and a runner that dispatches incoming methods and handles messages/intermediate results while an operation runs. No separate feature-specific MCP URL is needed merely because an operation is a resource, prompt, progress, interaction or task operation.

Dispatch cannot depend only on a final ToolResult: method requests, notifications, legacy server-initiated requests and modern input_required continuations have different handling. Preserve D47's revision-specific JSON/SSE and GET/DELETE/stream behavior at the same path. Client responses/resume requests return through the appropriate native protocol messages; no Fetch fields are inserted into raw tool arguments/results. Each relay belongs to its original authorized caller and upstream server; sharing functions does not share caller context or grant unsupported capabilities.

This approves the shared-handler/runner structure, not every feature's MVP inclusion, new product API route, remote-upstream transport, exact interaction/task contract, runtime/library choice or application implementation. Existing product HTTP APIs and MCP tools continue using the same backend rules/operations/permissions; the protocol dispatcher does not bypass them or create extra recursive tools. See [the endpoint design](mcp-connection.md#shared-handlers-and-runner--d59). Continue reviewing actual capability coverage under D58, without requiring one new URL per protocol function.

## D60 — Approve automatic MCP streaming defaults

On October27 the user replied "You’ve got it" to the presented recommendation: Fetch chooses response transport automatically, streams tool calls by default with available progress and the unchanged final result, uses ordinary JSON for quick reads such as tool listing, and needs no user-facing streaming switch. Requests needing additional input use the appropriate native MCP interaction flow at the same endpoint. A quiet tool may produce only its final result event; streaming does not manufacture partial output.

This selects the shown response defaults, not all capability scope, remote upstream transports, a streaming application HTTP API or permission to implement. Preserve D47 revision-specific acknowledgement/error/stream/cancellation rules, D59 shared functions and caller ownership, D43 raw terminal results and the reviewed execution deadlines. Progress does not silently extend those deadlines. Exact SDK/wire mechanics and remaining feature contracts stay open. See [automatic streaming defaults](mcp-connection.md#approved-automatic-streaming-defaults--d60).

## D61 — Route HTTP and SSE through separate transport functions

On October27, after clarifying that MCP servers hosted elsewhere are remote, the user said "yeah can we not just add http/sse support as a separate function that we can route for those types of calls?" Select HTTP/SSE upstream support alongside local-process connections, implemented as separate transport functions/adapters selected by the shared runner from the managed server's saved connection type. This extends D59's shared-function structure rather than creating a separate product, client-facing URL or generic tool-call format.

The user subsequently reaffirmed that the shared runner must include handling for all these types. The runner must own dispatch and invocation/message/result handling across all three selected transport paths: local-process stdio, Streamable HTTP and legacy HTTP+SSE. Merely storing the type or providing a stdio-only runner does not satisfy this requirement. Transport-specific mechanics live in its adapter functions, under the same shared runner and backend policies.

The transport adapter sends/receives native MCP messages over its supported connection. Existing backend permissions, queue/deadline controls, server identity and raw argument/terminal-result behavior remain shared. Do not infer transport/routing from arbitrary tool argument keys or returned text. Fetch's client-facing /mcp and application listener remain localhost-only under D47; remote transport selection refers to the upstream hop.

D61 supersedes D35's local-only exclusion at the scope level for supported HTTP/SSE sources. It does not approve unshown import/config/state/reconnect fields, exact upstream protocol versions or fallback, provider auth integrations, D37 header-validation exceptions, remote uncertain-work recovery or application implementation. The prior remote-scope proposal is not blanket-approved by this structural clarification. Preserve no source credential migration and no automatic invocation replay; reconnecting a connection is not restarting the provider. Review the necessary existing-API/lifecycle amendments together. See [HTTP/SSE transport adapters](remote-upstreams.md).

## D62 — Carry per-tool transport routing in the internal call context

On October27 the user clarified that each tool should have its MCP type in its configuration, injected into the call so Fetch knows which function to use. Select a per-tool Fetch-owned routing configuration that identifies the originating managed server/tool and its connection transport. The shared runner receives this routing information alongside the invocation and selects D61's local-process/stdio or HTTP/SSE adapter.

Here type means connection transport, not a different raw model argument format or a different upstream tools/call method. Keep routing in Fetch's internal execution context, outside the model-supplied argument object, raw upstream Tool definition and native forwarded params.arguments. Neither an arguments property called type/transport/server_id nor provider-returned text can override the saved route. Tools from the same connection must route consistently to that connection; no credential values are exposed through this metadata.

D52 durable per-tool settings and D45 exact server/original-tool mapping remain applicable. Exact field names/enums, storage duplication versus references, consistency/update mechanics, protocol adapters and API-visible schema amendments remain implementation/design work, not silently selected here. This clarifies internal routing metadata and transport dispatch; no application implementation, new endpoint, provider credential migration or blanket protocol-feature approval is authorized. See [per-tool routing context](remote-upstreams.md#per-tool-routing-context--d62).

## D63 — Approve the presented remote connection API amendment

On October27, after the complete remote API amendment and the user's requested TLDR, the user said "good." Approve the presented combined existing-route remote behavior: safe HTTP/SSE discovery metadata; source-file/name preparation without package download/network/login; remote server/config/state; header-reference-only config PATCH and its matching existing management-tool input; fixed imported URL/transport identity; shared encrypted credentials and stored-credential verification; and existing restart POST as Connect/Reconnect.

The presented reconnect policy is selected: force_restart defaults false, 202 with existing named-GET monitoring; normal gating/rejection of waiting calls and up-to60-second active-call drain; up-to5-second Fetch-owned context closure; up-to30-second replacement connection/readiness; remote phases and CONNECTION_READY success, distinct safe failure causes. Force skips draining with best-effort cancellation, preserves unknown old outcomes without replay, and allows fresh current-context capacity while exposing unresolved_prior_calls. This is not proof of remote termination or a provider-wide concurrency bound. Opening Fetch attempts saved remote connections once without automatic interactive login; backgrounding retains them; Quit disconnects Fetch without stopping providers.

Approval covers the presented observable amendment and shown schema/defaults, not silent adoption of extra protocol capabilities or unshown implementation mechanisms. Preserve [the amendment's separate prerequisites](remote-api-amendment.md#separate-prerequisites-and-future-acceptance-evidence): exact upstream versions/fallback and D37 annotation compatibility, destination/redirect/TLS/OAuth-discovery policy, actual provider verification/refresh ownership, crypto/storage/reference/ID representation and bounded-resource/retirement mechanics. Existing unapproved ordinary execution parameters remain unapproved. No application implementation, test environment, credential migration, personal configuration edits or live connection tests are authorized.

See [remote connection API amendment](remote-api-amendment.md). The original TLDR summarizes the same presented bundle rather than adding a new route or replacing raw model arguments/results.

## D64 — Approve the native result marker and explicitly incomplete valid-tool lists

On October27 the user said "first one is fine" after concrete examples, then requested returning the19 valid tools while marking the catalog incomplete when one of20 tools is rejected. Approve the narrowly presented D43 exception: canonical HTTP raw terminal result R remains unchanged; the modern native copy may add only a missing required resultType:"complete" when the originating legacy revision/response proves terminal completion. Preserve every existing field/content/value and do not overwrite invalid/conflicting markers, guess nonterminal completion or add Fetch text/result copies.

Replace the assistant's unapproved whole-origin/native-catalog failure proposal with the user's partial-availability direction. After a complete upstream collection, mandatory rejection of a specifically incompatible tool may leave valid tools usable and returned unchanged, with Fetch catalog completeness explicitly false and safe identification/reason for the rejected tool outside raw Tool objects. One rejected definition must not by itself hide the other19, fail the entire affected origin or block the combined native catalog. Keep unaffected origins usable. This is a narrow D37/D38 amendment for known protocol-rejected definitions, not a general field whitelist or permission to return truncated pagination as complete. Duplicate-name/collection failures retain their existing rules; unknown fields alone are not rejection grounds.

Native MCP tools/list has no standard catalog-incomplete flag. A Fetch-owned metadata indication plus existing Fetch listing-tool/API guidance is the proposed delivery mechanism; exact wire names/origin status/rejection schema and client/model visibility need review/isolated evidence. Do not repurpose resultType or nextCursor as a catalog-completeness flag, insert unsupported custom next_steps roots into native envelopes, or claim all clients expose metadata to their models.

The user did not approve the rest of the prior bundle by this answer: upstream exact revision targets/setup policy, native private/zero-TTL catalog defaults and unshown SDK/error/serialization mechanisms remain proposed. No dependency, application implementation, personal configuration change or live test is authorized. See [protocol adaptations](mcp-protocol-adapters.md).

## D65 — Approve catalog status, protocol views and version/cache defaults

On October27 the user said "good" to the complete presented catalog-status/view bundle. Accept available/partial/unavailable origin states, rejected_tools with safe name/code/message, successful partial HTTP reads with complete:false, optional view selection (raw,2025-11-25,2026-07-28), direct HTTP raw default and actual-client-view defaults for native MCP and fetch__list_tools. Preserve caller visibility, source-versus-export rejection boundaries and unchanged raw valid Tool values. Collection/duplicate failure remains unavailable; all-rejected partial differs from a genuinely empty available list.

Accept the presented native tools/list outer _meta["fetch/catalog"] completeness/issue summaries, without Tool/result duplication; clients may ignore metadata and model visibility still needs isolated evidence. The existing Fetch listing tool returns the same API JSON/next_steps in its text result. Its optional view input/contextual default is an explicit D49 amendment, not a new public route, authority grant or imported tool argument.

Accept the presented initial upstream compatibility targets2025-11-25 and2026-07-28, supported version-appropriate setup without authentication downgrade/tool-call replay, and applicable modern native catalog/discovery cache hints ttlMs:0/cacheScope:"private". Targets are not proof that every configured server/release/older protocol works; client cache behavior, SDK/raw capture/numeric fidelity and host delivery still need verification.

The subsequent question about where returns appear asks for delivery clarification, not revocation of the preceding approval. They appear in application GET /api/v1/tools JSON, native POST /mcp tools/list result metadata, and the text ToolResult of fetch__list_tools. Ordinary imported tool-call results retain D43/D64 behavior.

See [the catalog contract](tool-catalog-status.md). Actual SDK/library/serializer/raw interception/error mechanics, provider authentication/refresh, endpoint/redirect/TLS, encrypted storage/key/resource handling and remaining capability scope stay explicit open work. No application implementation, live connection test, provisioning, personal configuration change or credential use is authorized.

## D66 — Approve the presented native progress and interaction relay

On October27 the user replied "yep" to the complete native relay proposal, then asked how it fits the /mcp endpoint and the previously designed server-service functions. Approve the presented native-only progress, roots/workspace, sampling/model, form/URL elicitation and continuation relay through the shared runner. Actual caller capability/provable ownership is required; no Fetch-owned model/dialog/filesystem fallback, cross-client context borrowing, automatic consent or credential-form bypass.

Accept the presented legacy limitation: unowned stdio/background requests receive clear unsupported handling rather than being assigned by active-call count/most-recent client; fixed legacy capabilities remain conservative and cannot be an arbitrary client union. Supported cross-era relay needs real adapter ownership/capability evidence, not only a protocol statement or SDK send option.

Accept one logical call/original timeout across supported continuation rounds, with a temporary owner-bound Fetch handle that restores the provider's opaque state unchanged. Preserve raw model arguments and terminal ToolResults under D43/D64. Control-field/correlation adaptation is outside those preserved values and is not replay of an uncertain initial action. Direct application REST tool calls remain D50 terminal-only; no REST/UI/model/job/status/cancel operation is added.

The delivery question clarifies integration rather than revoking the approval: /mcp dispatches native protocol messages into the shared backend runner; the runner invokes the selected stdio/HTTP/SSE adapter, and supported progress/interaction handlers route server events to their original client. Existing configuration, credential/authentication and process/reconnect services retain their backend ownership and rules. Provider tools are forwarded, not reimplemented inside Fetch. Exact implementation function/module names, SDK correlation, handle representation, resource/cancellation/uncertainty mechanisms and real client compatibility remain unselected or unverified.

See [native relay contract](mcp-interaction-relay.md). Native Tasks/resources/prompts/GitHub scope and remaining execution/auth/destination/TLS/storage/delivery decisions stay open. No application implementation, dependency, environment, personal configuration/credential change or live connection test is authorized.

## D67 — Approve presented native resources/prompts and explicit deferrals

On October28 the user replied "Cool" and then "Next" to the preceding combined resource/prompt/completion and remaining-scope presentation. Approve its presented native methods through /mcp, stable source-prefixed prompt names, source-qualified resource/template selectors, unchanged returned content and raw ToolResult resource links, safe unknown/ambiguous raw-link routing,10-second per-source listing budget and configured server read/get execution duration. Include supported resource/prompt catalog-list changes and required modern native outer framing with caller-private/zero-lifetime cache hints; preserve raw content and D43/D64 tool results.

Explicitly defer live per-resource content subscriptions/updates and native background Tasks from the initial slice. Ordinary tools returning job IDs/status tools and longer configured calls remain supported under existing raw contracts. D58 still requires sufficient actual everyday Claude Code/OpenCode workflows in D46 isolation; an essential deferred feature is a compatibility gap requiring review, not a silently removed server or automatic scope expansion.

Approval is scoped to what was presented. The illustrative resource prefix is a candidate rather than a proved universal URI/template codec. Exact native diagnostic metadata fields, error numbers, representable bounds, SDK/client fidelity and other unshown mechanisms are not silently approved by a linked proposal. Preserve backend authority, original source selectors, string-valued prompt arguments, payload/subresource URIs and no automatic prompt application/model invocation. No additional REST route, personal configuration change, live test or implementation is authorized.

See [native resources/prompts](mcp-resources-prompts.md). Next review is the existing queued execution/settings proposal; GitHub CLI operation/auth ownership remains an explicit dependent Step2/4 decision.

## D68 — Approve presented ordinary tool-call queues and settings

On October28 the user replied "Good" to the complete ordinary tool queue/concurrency presentation. Approve shared one-process/server state and default one concurrent call, configurable higher; FIFO dispatch;64 waiting calls or16MiB queued UTF-8 argument buffers per server;409 SERVER_BUSY/not_started when full;300-second queue wait with504 TOOL_CALL_TIMEOUT/not_started at expiry; and a separate fresh dispatch-relative execution timer.

Approve timeout precedence: exact Fetch per-tool override, then explicitly supported current whole-call server limit plus5seconds for receiving the result, then configured server fallback initially120seconds. Progress and interaction rounds do not reset it. Existing config API/matching tool accept execution.max_concurrent_calls/default_timeout_ms/tool_timeouts_ms; omission preserves settings and null removes a tool override, never means unlimited. Execution-only edits use existing authority/atomic saves, apply without restart/auth invalidation; admitted timeout budgets remain fixed, live concurrency controls later dispatch and lowering never kills active work.

Approve cancellation/uncertainty behavior shown: REST disconnect and modern pending-stream closure request best-effort cancellation; legacy stream loss alone does not. After possible forwarding a timeout has unknown outcome and retains occupied capacity until terminal response or confirmed local child exit, with D63 explicit remote forced-reconnect exception and retained old unknown outcomes. No automatic replay. Restart flushes unforwarded calls with409 SERVER_BUSY/not_started then uses existing drain rules; unexpected server failure flushes waiting calls with503 SERVER_NOT_READY/not_started. Existing server GET exposes aggregate active/queued/uncertain counts and limits without call data/caller identity; uncertain calls count as active. State-driven UI and next_steps express matching actions.

Approval covers presented behavior and shown config field names, not every unshown linked-document detail. Exact stored profile schema/provenance, aggregate GET wire names, setting validation error refinements, representable upper bounds, runtime/SDK/storage implementation and native resource/prompt/completion scheduling classification remain open. Existing source/authority/raw argument/result rules persist. See [execution contract](mcp-execution.md). No application implementation, live tests, client edits or credentials are authorized.

## D69 — Connect apps through the local browser flow without a Fetch account

On October28 the user clarified: "We don’t need a fetch account the apps just need to get an auth token directly through browser connection," adding that it should stay simple because Fetch is local.

Fetch does not require a Fetch account, signup or Fetch password. The browser is the connection/authorization step through which an app obtains its own Fetch access credential. Keep D10's standard code/token exchange and automatic renewal direction; this does not select exposing an access token in a browser URL. There is no new hosted identity-service requirement.

Earlier assistant wording "Fetch's browser login" made an unselected screen/account design sound settled. It should mean the proposed local app-connection flow only. Exact browser approval controls, proof of the local person's authority, OAuth endpoints/registration/scopes/token handling and callback mechanics still need the complete reviewed contract. The user has not selected HTTPS/certificate trust, a localhost-HTTP exception, a local CA, key storage or implementation by this clarification. Local-only access and backend permissions stay selected.

See [the remaining checklist](open-questions.md#remaining-decisions-checklist) and [local connection/trust discussion](local-oauth-trust.md). No account implementation, certificate/client/trust-store change, credential use or live test is authorized.

## D70 — Approve local HTTPS and the certificate verification direction

On October28 the user said "Good to go for this part, next" after the local HTTPS/certificate discussion and the creation-versus-verification effort explanation. Accept the presented local HTTPS direction for Fetch's existing shared loopback listener, with a unique certificate/private key generated for each installation, explicit human certificate trust setup and verification in the browser and supported apps with normal certificate checks enabled. This resolves the scheme/certificate direction; do not re-review it as a broad open choice.

Preserve D69: no Fetch account/signup/password; apps obtain their own token through the local browser connection. Preserve D47's single local listener/process, API/MCP roles and existing permission checks. Broader local-CA trust is not an automatic fallback if limited certificate trust fails. Exact certificate/key storage, unlock, replacement/expiry/cleanup and per-client trust mechanics remain implementation/onboarding work with isolated evidence required. The numeric port and exact published address remain unselected; subsequent endpoint paths are relative to Fetch's published stable local HTTPS origin.

Approval is for the discussed design and verification direction, not a measured schedule, completed compatibility proof, unshown OAuth/token/approval controls, environment provisioning or authority to modify the user's trust store/clients. D46 isolation and the application implementation/live-testing pause remain. Next is the complete account-free app-authorization/token contract.

See [local HTTPS and certificate trust](local-oauth-trust.md), [connection](mcp-connection.md) and [remaining checklist](open-questions.md#remaining-decisions-checklist).

## D71 — Approve the presented account-free app-connection contract

On October28 the user replied "Yes" to the presented complete app-connection contract, including its one-time Mac confirmation and separate management permission. Accept the shown browser request/Allow-Deny/Mac confirmation/code/token flow without a Fetch account or web-collected Mac password; later calls/renewal do not repeat human setup.

Approve the seven shown operations: `GET /.well-known/oauth-protected-resource/mcp`, `GET /.well-known/oauth-authorization-server`, `POST /oauth/register`, `GET /oauth/authorize`, `POST /oauth/authorize`, `POST /oauth/token` and `POST /oauth/revoke`. The coding app owns its callback; no Fetch callback route is added. Registration grants no access. Use initial local app registration and defer website-based client registration as presented. Invalid callbacks do not receive redirects; invalid/expired codes or tokens fail safely, and successful token changes require successful saving.

Approve the shown scopes: `fetch:catalog` for permitted catalogs, `fetch:tools:call` for installed-tool invocation, `fetch:content:read` for resource reads/prompt retrieval/suggestions, and `fetch:manage` for discovery/install/server state/auth/config/restart. First three are default; management must be requested and separately approved and never implies invocation. Neither permits agent manual-secret entry. Grants cover current/future installed servers/tools subject to server/tool restrictions, with the same authority on MCP and ordinary Fetch APIs. Preserve existing native/raw data and backend permission boundaries.

Approve ten-minute pending approval, sixty-second one-use code, one-hour access token and ninety-day refresh inactivity expiry; active use renews automatically. Each refresh replaces the renewal token; reusing a spent token disconnects that connection, including the shown lost-response/overlapping-renewal reconnection cost. Valid saved connections survive reopening Fetch; unfinished approvals do not survive Quit. Revocation stops new/waiting requests and further results, requests cancellation of running work and never claims completed effects were undone. Presented admission bounds are16KiB submitted bodies,32 pending approvals and256 registered apps/active connections with retry responses when full.

Approval is scoped to the presented clauses, not every linked unshown mechanism. Exact Mac-prompt timer/foreground behavior, detailed standard wire/registration defaults, key/storage/crypto implementation, connection-administration APIs/UI and actual SDK/client/rotation/certificate evidence remain explicit implementation/onboarding/test dependencies. Do not reopen the approved routes, permissions, lifetimes or basic approval flow merely to resolve those mechanisms. No application implementation, environment provisioning, OS/Keychain/certificate/client changes or live testing is authorized. Next is Settings human authority and key protection/unlock/recovery.

See [the app-connection contract](client-authorization.md), [remaining checklist](open-questions.md#remaining-decisions-checklist) and [storage boundaries](storage-scope.md).

## D72 — Approve the shown Settings and Keychain direction

On October28, after the assistant explained the next direction in commentary, the user said “yes good” and “i already approve this next one keep going.” Record approval of what was shown: each server's secrets stay in encrypted files; macOS Keychain holds the keys; only Fetch's own native Settings window manually saves/removes secrets with a Mac confirmation; ordinary tool calls and automatic token renewal do not ask for Touch ID every time.

This confirms the concrete direction on top of D26/D28 and D71. Do not ask to approve those broad choices again. The detailed Settings/key draft was not presented in full: traditional versus data-protection Keychain mode, exact cipher/envelope/filenames, private authority header/bootstrap, prompt timer/error codes, key lifecycle/recovery APIs and signing/bridge details remain implementation/storage/onboarding candidates, not silently accepted. Resolve them in the bounded implementation review rather than reopening the shown direction.

Backend API authority, Settings-only manual secrets, successful encrypted storage/readback plus D25/D31 provider verification, per-server isolation and no plaintext fallback remain intact. No new application code, dependency, Keychain/certificate/client/environment change or live test is authorized. Next is the specific remaining provider verification/handoff/failure policies in4d; GitHub CLI operation/auth ownership remains separate dependent work.

See [the Settings/key direction and mechanism candidates](secret-storage-settings.md) and [remaining checklist](open-questions.md#remaining-decisions-checklist).

## D73 — Consolidate the fixed MCP release plan and stop repeated reviews

On October28 the user corrected the remaining-decision list and said the planning had been circling back to old subjects. They require one complete step/plan with everything needed recorded together, not repeated approval of old choices.

- **Agent connection:** each agent stores its Fetch credential/token and connects to the single Fetch `/mcp` interface for all permitted installed tools. User-completed authentication for each installed provider/server is separate and reused. Ordinary calls do not start login, repeat provider verification or ask for a provider credential per tool. Preserve the selected renewable OAuth flow, per-agent/grant permissions and no Fetch account; this is not a newly selected static token or one shared credential across every client.
- **Execution and Rust:** treat queue/concurrency/timeout/restart behavior and the Rust backend/one-process architecture as settled. Exact crates, module/Cargo layout, encodings, native admission and compatible implementation mechanisms are engineering work under the fixed requirements, gathered into one worklist. They must not be presented as fresh broad execution/Rust product decisions. Unselected details are not falsely marked historically approved; genuinely changed API/major behavior still needs input in one complete review.
- **First release scope:** no shared memory in this first release. Preserve it as future functionality in the same Fetch product, not a separate app or a blocker for the MCP release. No memory API/history/undo/UI work is required for this implementation slice.
- **Distribution:** select standard drag-and-drop macOS installation using a DMG containing Fetch.app and an Applications shortcut. Select the MIT open-source license. Actual signing/notarization credentials, installer construction, publication and safe data handling are release work, not a reason to reopen installer/license scope; no deployment/release is authorized.
- **Planning:** replace the ten-group approval count with one implementation checkpoint. Keep locked decisions, a five-stage API-first build sequence, one complete engineering worklist and only real remaining scope choices in the canonical build plan. Reuse research and accepted reviews. Do not split every internal field/helper/library patch version into its own approval request.

The custom Fetch-owned GitHub CLI adapter remains a genuine include/defer question, distinct from importing an existing GitHub MCP. Optional unselected management controls stay outside scope unless a selected workflow requires them; no new routes are silently added. The unaccepted stored-credential-verification amendment/storage-only200 exception is withdrawn from active review, not approved or silently implemented. D25/D31's selected success gate remains intact; exact engineering/error/integration requirements belong in the consolidated worklist.

[build-readiness.md](build-readiness.md) is now the canonical single implementation checkpoint; [open questions](open-questions.md) is a short index, not another approval backlog. Preserve immutable history/research. Application implementation, dependency/service installation, environment provisioning, real credentials/client/Keychain/certificate changes and live testing remain paused until separately authorized.

## Existing product/storage decisions retained

- Shared memory stays part of Fetch: one preference prompt with simple history/undo; the graph and Git-style branching/merging are deferred.
- MCP configuration history/rollback is outside the MVP.
- Initial MVP excludes PostgreSQL functionality. Local config files and encrypted tokens are selected under D19; exact encoding/layout/key handling remain open. Earlier local PostgreSQL/Tailscale guidance is deferred, not a release prerequisite.
- If database work returns, the earlier shared migration/hosting-separation direction remains future guidance. No database schema/migration work is required for the current MVP.
- Localhost-only Fetch listeners remain selected. Neither cross-device application access nor PostgreSQL/Tailscale database setup is required for this release.
- The repository is open source and locally runnable without the original developer's infrastructure. D73 selects MIT and a drag-and-drop macOS DMG; exact signing/notarization, construction, data-preservation and authorized-publication mechanics remain release work.

## D74 — authorize implementation and repository push, without release

October28, 2026. The user said to start building and push to the configured remote, create no release, provide developer-server commands, and work together later on main MCP tests in an isolated virtual environment with the same server inventory. This supersedes prior implementation/push pauses. It does not authorize release publication, personal-client changes, trust-store changes, credential migration or live managed-server connections. The first implementation pass establishes a runnable Rust developer-backend foundation; incomplete features remain explicitly unavailable. Selected product/API behavior stays intact; no shared-memory work is included.

## D75 — use a less common developer port

October28, 2026. The user requested a less common port and current-state documentation. Set the current developer default to53671 instead of4141, retaining explicit --port overrides and port0 test allocation. A local TCP listener check found53671 free, and a targeted public search found no established common software/MCP default there; neither guarantees permanent availability or exclusivity. This changes the developer address only, not the selected HTTPS/authorization rules or trust. Refresh current docs/status, preserve historical records and unfinished work.
