> **Current authorization — October28, 2026:** The user authorized starting implementation and pushing to `https://github.com/yuyanghu06/fetch.git`. Do not create a release. Main MCP connection tests are deferred to a later isolated virtual environment using the user’s server inventory. This supersedes earlier implementation/push pauses only; it does not authorize personal-client edits, certificate trust changes, credential migration or live MCP tests.

# Fetch API contracts under discussion

Current checkpoint D73: one saved Fetch credential/token per agent for `/mcp`; user-completed provider login reused on ordinary calls. Rust/execution and shown auth/Settings directions are settled; shared memory is deferred from this release; MIT and drag-and-drop DMG selected. Use the single build-readiness.md worklist for remaining implementation mechanisms. The unaccepted verification amendment is withdrawn from active review, not approved; preserve D25/D31. Do not turn older draft headings below into fresh broad approval requests. No implementation/live tests authorized.

Latest checkpoint D71: the presented account-free app-connection contract is approved, including its seven operations, Mac confirmation, scopes, token/renewal/revocation/restart defaults and admission bounds. D70 HTTPS and D69 no-account direction remain selected. Exact unshown wire/OS/storage mechanisms and actual client evidence remain open; next is Settings human authority and key protection/unlock/recovery. No implementation or live tests are authorized.

D68 approves the presented ordinary tool-call queue/concurrency/settings/cancellation/restart contract. Shared default concurrency1 (configurable), FIFO64 waiting/16MiB arguments,300-second queue wait, selected timeout precedence and5-second documented-limit allowance, existing execution config/tool amendment, hot atomic saves, aggregate server-state counts, uncertainty accounting and D63 remote forced recovery are selected. Exact unshown profile/GET wire schema, numeric upper bounds and native resource/prompt admission remain open. Do not re-review selected behavior; implementation remains paused.

D67 approves the presented native resource list/read/templates, prefixed prompt list/get and supported completion on /mcp, qualified source routing with unchanged returned content/raw ToolResult links, safe ambiguous-link errors,10-second per-source lists, configured server read/get budgets, required modern outer framing/private zero-lifetime hints and catalog-list changes. Live per-resource content subscriptions and native background Tasks are deferred for the initial slice, subject to D58 isolated everyday-workflow acceptance. Exact URI codec/SDK/numeric errors/unshown metadata wire fields remain review/evidence work; implementation remains paused.

Last synchronized: October 27, 2026.

These are design records, not implemented endpoints or a generated OpenAPI specification. Approval of a route's purpose does not resolve every validation, authorization, schema, and runtime detail. Resolve the listed gaps with the user before implementation.

## Shared conventions accepted in principle

- Discovery and installation use `/api/v1/setup`; their confirmed setup handler grouping is `src/api/setup/`. After D21, ongoing server management uses `/api/v1/servers` and its existing subroutes. The handler/module grouping for that moved family remains a step-5 choice.
- D48 requires matching descriptive MCP tools for every current/future agent-accessible Fetch product API operation, using the same backend rules and caller permissions. Descriptions explain reads/changes, inputs, results/pending states and human handoffs. The existing Settings-only secret-value exception remains; native OAuth/MCP endpoints retain their protocol roles. Imported-tool calls already provide their matching individual tools, preserving raw upstream metadata.
- The accepted manual credential/environment-value write operation is an exception to ordinary agent setup access: require Settings authorization and keep secret-valued inputs out of MCP tools. Trusted backend OAuth callbacks/refreshes still write newly issued credentials automatically.
- Every current and future Fetch application API returns `next_steps` in its JSON responses, including success, pending and error results (D22). Use an empty array when no follow-up is needed. Entries follow the existing `actor`, `action`, `instructions` structure; endpoint-specific action values/target fields are reviewed with their contracts. This confirmation does not select a new endpoint-wide `status` enum or alter OAuth/MCP protocol envelopes; how guidance fits those protocol responses remains to be designed.
- D32 makes agent-free Fetch UI setup the base path. `next_steps` is agent-facing guidance only, including `actor: user` tasks for an agent to relay. The UI still receives the common field but uses structured API states/results/errors and deterministic native sequencing, not guidance text/action codes or their order, for status/controls. Backend APIs continue enforcing all prerequisites and authority. No connected agent is required for first installation/setup.
- D33 requires `next_steps` and the app’s deterministic sequence to express the same reviewed workflow across all prior/current/future routes: prerequisites, valid order/actions, completion/error/retry rules and explicit user handoffs. Update both when workflow rules change. The UI still follows structured state, not the guidance payload; permissions remain enforced by backend APIs. See the alignment table below.
- Never return credential values or raw source configuration. Return credential-variable names and missing/authentication information where known.
- An opaque installation/server ID is a reference, not an authentication credential. Exact ID generation/format has not been selected.
- This discussion estimated roughly 15–20 HTTP routes including OAuth/MCP support. That was an estimate, not an approved route inventory or a requirement to create that many routes.

## Workflow alignment across all existing routes — D33

This applies the user's consistency requirement retroactively to every previously discussed operation. Agent guidance and native UI sequencing must describe the same underlying reviewed rules. It does not promote unapproved route details to decisions: pending contracts/refinements stay pending, and exact UI screens/sequence branches remain step-6 work. The app never parses the guidance to drive its controls.

| Existing operation | Consistent UI sequence and agent guidance | Preserved backend/approval boundary |
| --- | --- | --- |
| `GET /api/v1/setup/discovery` | Present the same discovered entries/warnings and respect user selection; both proceed through installation API rather than treating discovery as an installation. | Read-only; no secret migration or source writes. D35 discovery contract approved; exact reader/launch grammar/path mechanics remain later implementation work. |
| `POST /api/v1/setup/installations` | Accepted `202` moves to monitoring that returned group ID; neither client treats acceptance as preparation/auth/readiness success. Both follow the approved failed-entry retry/successful-record reuse rules; a status-read failure does not trigger automatic POST replay. | D34 approves presented submission/source/reuse/concurrency/retry rules; exact path/preparer/deadline/cleanup mechanisms remain later work. |
| `GET /api/v1/setup/installations/{installation_id}` | Pending group keeps monitoring; failed items do not cancel other accepted installs. Completed group failure preserves successes and stops dependent setup; retry guidance concerns failed entries. Success leads to inspecting resulting server IDs/states. | Preparation success differs from auth/runtime readiness. D36 approves progress states/schema/errors/polling/same-run lookup; no automatic retry selected. |
| `GET /api/v1/servers` | Native controls and agent guidance reflect the same saved config, auth/runtime state, missing entries and pending restart; neither infers readiness from HTTP read success or assumes unknown auth is unnecessary. | D23 read-only latest-known state; GET does not probe a provider/start login/restart. Named restart filtering remains the same GET. |
| `POST /api/v1/servers/{server_id}/authenticate` | Both follow supported no-auth/OAuth/manual/native/unknown branches, monitor the same auth state and require the stored-credential test before credential-based success; no guessed login or repeated browser opening. | D25/D27/D31 evidence and shared verification-failure code; concrete integrations still need review. Browser/user actions stay explicit. |
| `PATCH /api/v1/servers/{server_id}/config` | Both edit the common launch fields/references, distinguish saved from applied settings, and respect actual restart/auth-evidence status afterward. Guidance must not require a restart when no change is pending or silently apply one. | D30 common-field contract; no inline values/native-settings normalization. Existing auth/restart overlap and no-auto-restart rules remain intact. |
| `PATCH /api/v1/servers/{server_id}/credentials` | Settings accepts values; agent guidance hands that same stage to the user. Both recognize encrypted save/readback plus new-credential verification before `200`, shared test-failure code and truthful stored-versus-verified outcomes. | Settings-only manual writes, no secret MCP input/output. Concrete verifier/error/deletion/batch policies remain open; guidance/UI must be aligned as they are decided, not invent an unchecked-success exception. |
| `POST /api/v1/servers/{server_id}/restart` and named GET monitoring | Both monitor the returned attempt, keep pending until that attempt succeeds/fails, and use the same normal/explicit-force, drain/readiness/failure/retry rules. `202` or a status-read `200` alone cannot finish the spinner/setup step. | D20 exact overlap/deadlines/readiness and captured settings. No automatic force escalation, POST replay or historical job recovery promise. |

For shared stages, align the same state transition/completion meaning rather than identical wording or identical caller powers. `actor: user` is a real human handoff an agent relays; the native UI offers the applicable approved control. Returning instructions does not confer permission or perform the action. Exact final choices for currently pending branches remain visible in [open questions](open-questions.md).

Future acceptance evidence, after implementation authorization: for the same structured states, check that native UI and agent guidance recommend the same allowed stage/outcome and respect permission differences, including failures, busy operations, partial installs, saved-but-unverified credentials and named restarts. Valid wording-only guidance changes must leave UI behavior unchanged; workflow-rule changes must update both clients' corresponding behavior/guidance. This selects no shared workflow-engine implementation or additional instruction field.

## Accepted: discover configured servers

```http
GET /api/v1/setup/discovery
GET /api/v1/setup/discovery?config_path=/Users/example/.codex/config.toml
```

### Input and behavior

- `config_path` is optional and identifies a configuration file on the Mac running Fetch.
- When supplied, inspect that file.
- When omitted, inspect the current user's normal global Claude Code, Codex, and OpenCode configurations.
- Read/parse configuration; do not install or start servers, modify the source configuration, or copy credentials as part of this operation.
- Discover package identity/version requests and launch information for local scripts, plus credential-variable names where they can be identified without exposing values.
- Exact default locations/overrides, input path normalization, format detection, and user-selected non-global-file boundaries still need specification.

### Accepted missing/invalid file behavior

| Situation | Agreed behavior |
| --- | --- |
| A normal default config does not exist | Skip it normally; the user may not have that harness installed. |
| Explicitly supplied file does not exist | `404` configuration file not found. |
| Explicit file cannot be parsed or is not a supported MCP configuration | `422` explaining the problem without raw contents/secrets. |
| A default file is malformed/unsupported | Warn for that file and continue reporting other discovered configurations. |

The preliminary summary above predates D35. Exact error envelopes/codes, unreadable-file handling and partial-entry behavior are approved in the complete discovery contract below.

### Response example discussed

The conversation proposed grouping results by file. The user confirmed the optional-path/default behavior and later the common status/next-step convention. Preserve this example as a working schema, not proof that every field/enum has been individually finalized:

```json
{
  "configs": [
    {
      "path": "/Users/example/.codex/config.toml",
      "harness": "codex",
      "servers": [
        {
          "name": "example-node",
          "type": "npm",
          "package": "@example/node-mcp",
          "version_spec": "1.2.3",
          "credential_variables": ["EXAMPLE_API_KEY"]
        },
        {
          "name": "example-python",
          "type": "pypi",
          "package": "example-python-mcp",
          "version_spec": "latest",
          "credential_variables": []
        },
        {
          "name": "example-local",
          "type": "local_script",
          "executable": "python3",
          "script_path": "/Users/example/tools/server.py",
          "working_directory": "/Users/example/tools",
          "credential_variables": []
        }
      ],
      "warnings": []
    }
  ]
}
```

Notes for completing the schema:

- The sample predates the shared `status`/`next_steps` convention; integrate that convention in the final response review rather than treating this sample as the complete final schema.
- `version_spec` describes the source request, not a version resolved/installed during discovery. Preserve omitted versus explicit version semantics in the final schema.
- `credential_variables` cannot claim to know every required secret from arbitrary commands. Unknown fields/secret classification require a policy; no generic name filter proves arbitrary input is credential-free.
- Exact non-secret argument preservation, unknown launcher representation, duplicate detection, and source identifiers are open.
- Names and package examples are invented; they are not catalogue entries or installations to perform.

### Approved discovery contract

October 27, 2026: the user replied "good next" to the complete discovery contract (D35). The presented default/custom-file scope, compatible readers, safe grouped metadata/eligibility/default-selection flags, partial failures, request errors and read-only/no-resolution behavior are approved. Official source facts remain in [the research record](research.md#global-discovery-inputs--rechecked-october-27); Fetch's selected scan/fallback policies do not claim to reproduce active host layers. Concrete reader versions/launch grammar/path identity/read-limit mechanisms remain later design choices; no implementation is authorized.

```http
GET /api/v1/setup/discovery
GET /api/v1/setup/discovery?config_path=/absolute/file
```

#### Approved input, callers and default candidates

Allow Fetch Settings and authenticated setup/management clients with server-management permission. Authorize before exposing any file existence/content metadata. No request body is used; reject a non-empty body without echoing it. Accept at most one non-empty/non-whitespace absolute `config_path`; reject relative paths, NUL, repeated parameters and unknown query parameters with `400 INVALID_REQUEST`. No shell, tilde or environment expansion for supplied paths. Explicit inputs select exactly one local regular file, not a directory/glob, URL or recursive scan.

Without a path, inspect the following candidates in this order, resolving built-in `~` from the current user's home directory:

| Harness / parser | Candidate and container |
| --- | --- |
| Claude Code / strict JSON | `~/.claude.json`, top-level `mcpServers` only; exclude nested project-scoped entries. |
| Codex / TOML | A valid absolute existing `CODEX_HOME` visible to Fetch selects `<CODEX_HOME>/config.toml`; otherwise use `~/.codex/config.toml`. Read top-level `mcp_servers`. This is Fetch's selected fallback, not a claim it can see a separate harness process's environment. |
| OpenCode / JSON or JSONC | `~/.config/opencode/opencode.json`, then include `~/.config/opencode/opencode.jsonc` as an additional Fetch candidate; top-level `mcp`. |

The OpenCode `.jsonc` candidate is a Fetch discovery choice, not a claimed official default/preference. If both files exist, show them independently with safe `MULTIPLE_CONFIG_FILES` warning; do not merge or claim host precedence. Missing default candidates are omitted normally. Do not infer unproven Claude sibling-file relocation from `CLAUDE_CONFIG_DIR`, or OpenCode MCP paths from XDG/`OPENCODE_CONFIG_DIR`. Custom locations can be selected explicitly. `OPENCODE_CONFIG` is a host configuration layer, not a replacement global path to silently follow.

Read supported definitions from these files; do not reconstruct profiles, project inheritance, plugins, managed/organizational/inline/remote configuration or each host's active state. Do not scan project directories, traverse Claude `projects[path]`, run a client CLI to resolve configuration, follow includes or query running clients. A known default file that parses as a normal object/table but has no expected global MCP map is an empty successful file, not a warning merely because no global servers are configured.

#### Approved explicit-file recognition and partial failures

Permit a user-selected file outside the normal global locations if it contains exactly one recognized top-level MCP container in a supported format. Inspect that file alone, not its surrounding project. Strict JSON `mcpServers` maps to `claude_code`, TOML `mcp_servers` to `codex`, and JSON/JSONC `mcp` to `opencode`. File extension is a hint, not authority to execute anything; use only these bounded data parsers/container rules, with duplicate-key rejection.

- Explicit missing file: `404 CONFIG_NOT_FOUND`.
- Explicit unreadable/non-regular/invalid-text/unparseable file: safe `422 CONFIG_UNREADABLE` or `INVALID_CONFIG_FILE` as applicable.
- No recognized container, wrong container type or unsupported format: `422 UNSUPPORTED_CONFIG_FILE`; more than one recognized container: `422 AMBIGUOUS_CONFIG_FILE`. An empty recognized map is a valid empty result. No raw parse excerpts.
- A default candidate that exists but cannot be read/parsed/recognized stays visible as a failed source group; continue other candidates. Top-level HTTP `200`/`status: ok` means the scan completed, not that every file/entry was valid. All-missing is `configs: []`; all-failed is a successful scan with failed groups, not fabricated empty success per source.
- Reject duplicate JSON/JSONC keys or TOML duplicate definitions rather than silently taking a final value. File-level parse/container failure produces no partial server list. Within a valid map, one malformed entry yields a blocked row while safe siblings remain available.

These explicit-file rules clarify the previously open boundary: an explicitly chosen recognized project-style file may be read as a single input, but there is no automatic project import/search or effective-layer reconstruction. D35 approves this explicit single-file scope; no recursive/project-layer import is selected.

#### Approved response and server metadata

```json
{
  "status": "ok",
  "configs": [
    {
      "path": "/Users/example/.codex/config.toml",
      "harness": "codex",
      "status": "ok",
      "error": null,
      "servers": [
        {
          "name": "example-node",
          "status": "supported",
          "type": "npm",
          "package": "@example/node-mcp",
          "version_spec": "1.2.3",
          "executable": null,
          "script_path": null,
          "working_directory": null,
          "credential_variables": ["EXAMPLE_API_KEY"],
          "source_enabled": null,
          "selected_by_default": true,
          "error": null
        }
      ],
      "warnings": []
    }
  ],
  "next_steps": [
    {
      "actor": "agent",
      "action": "review_discovery_results",
      "instructions": "Review supported rows and the user's selection, then submit selected file-path/name pairs to the installation API. Source-disabled rows require explicit opt-in."
    }
  ]
}
```

Always return `status`, `configs` and `next_steps`. Source groups always have `path`, `harness`, `status`, `error`, `servers`, `warnings`; use harness `claude_code`, `codex`, `opencode` for the matched reader/schema family. For a supplied custom file, this label is compatibility metadata, not proof which app owns it or what configuration is active. Source status is `ok` or `failed`; errors are null for success or a safe `{code,message}` for the failure described above. Warnings are `{code,message}` objects; the multiple-OpenCode-file warning carries no raw file contents.

Preserve default candidate order; an explicit file produces one group. Sort rows by exact case-sensitive source name. Keep different source files/entries distinct even when names/packages match; do not merge or claim filesystem-alias deduplication. Source path association/canonicalization must remain consistent with D34 once selected in implementation review; this schema does not choose stable IDs or alias identity encoding.

All server-row fields shown are present:

- `name` is the exact source entry name used as installation `server_name`.
- Row `status`: `supported`, `unsupported`, `invalid`. `type`: `npm`, `pypi`, `local_script`, or `null` if not safely classified. `package`/`version_spec` are safe source requests or null, never an installed/resolved version. Omitted requested version remains null, distinct from literal `latest`.
- `executable`, `script_path`, `working_directory` are safely established local-script/launch metadata or null; do not return raw launch commands/argument arrays. In supported rows, null means omitted/not applicable; in blocked rows it may mean not safely established and is not proof of absence.
- `credential_variables` is sorted/deduplicated observed destination/forwarded variable names only, never values. Empty does not mean no auth is needed or every required entry is known. Do not resolve parent/remote environment sources merely to populate it.
- `source_enabled` is an observed literal flag: true, false or null when absent/not established. It is not effective harness state after other layers, nor a newly selected Fetch enabled-setting field.
- `selected_by_default` is true for supported rows except explicit source-disabled rows; false for blocked/disabled-by-default rows. D35 selects this explicit source-disabled default exception to the earlier all-selected direction.
- Row `error` is null when supported; otherwise a safe `UNSUPPORTED_SERVER_DEFINITION` or `INVALID_SERVER_DEFINITION` with reason. D35 originally reported remote HTTP/SSE/WebSocket or remote-executor STDIO as unsupported. D61 subsequently selects HTTP/SSE upstream scope via separate adapters; its discovery metadata/eligibility schema amendment remains to review before implementation. WebSocket/custom remote executors remain unselected. No discovery-time network contact or raw URL exposure is implied. Unknown execution-affecting syntax stays unsupported, not guessed.

#### Approved selection and safe interpretation

Supported source-disabled rows remain visible and explicitly selectable; submitting that path/name tuple opts into preparing a new Fetch installation. It does not modify the source flag, invent a Fetch enabled-setting feature or start the server automatically. Other supported rows are selected by default; unsupported/invalid rows cannot be selected by UI/guidance. The installation API still independently reads/validates the current definition rather than trusting preview eligibility.

Only recognized literal launch forms whose preparation can be safely established are supported. Do not execute shells, interpolation, command/header helpers, include loaders, `env_vars` sources or `{env:...}`/`{file:...}` references. Record safe variable names without copying source environment values. If substitution, unknown working-directory semantics, remote execution or a secret/unknown argument affects package/launch identity, block that row until a supported interpretation is reviewed. Do not resolve a secret to make an otherwise unclear entry importable.

Never return raw source entries/commands/args, environment values, headers, OAuth material, credential-bearing URLs, helper output or source-bearing parser messages. General name-based redaction is not proof of safe arbitrary input. The safe launcher/argument/reader-version support matrix remains an implementation prerequisite; this proposal does not promise all npx/uvx/local commands can be normalized.

#### Approved errors, guidance and side effects

Request errors return `status: failed`, `error: {code,message}`, `configs: []` and `next_steps`:

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Invalid/repeated/unknown path/query input. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN` | Missing/invalid Fetch login or setup/management permission. |
| `404` | `CONFIG_NOT_FOUND` | Explicit selected file missing. |
| `422` | `CONFIG_UNREADABLE`, `INVALID_CONFIG_FILE`, `UNSUPPORTED_CONFIG_FILE`, `AMBIGUOUS_CONFIG_FILE` | Explicit input cannot be safely read/recognized as defined. |
| `500` | `DISCOVERY_FAILED` | Scan-wide internal failure; do not invent source results. |

Default-source failures use the same safe source codes inside HTTP-200 source groups, retaining healthy sources. No raw filesystem/parser/provider diagnostics or metadata before authorization. Return `Cache-Control: no-store`. No server/installation IDs or discovery snapshot tokens, package/runtime downloads, ordinary/helper process starts, auth/provider requests, source writes, credential migration or native-settings standardization occur.

Agent guidance mirrors the structured UI selection flow under D32/D33: `agent/review_discovery_results` reviews eligible rows and user choices then uses existing installation POST; `user/review_discovery_warnings` relays needed source corrections without soliciting secrets; `user/choose_config_file` handles empty/blocked-only results by selecting a supported explicit file. Always return next_steps, with [] if no agent follow-up is needed. Do not guide an empty POST or silently opt into source-disabled rows. The UI uses fields/status/errors/warnings/default flags and its own built-in controls, never this prose.

D34 remains: this read does not freeze definitions. New/failed installation items read the current definition when their turn starts; already successful Fetch associations retain saved state. No unchanged-since-discovery guarantee is added.

#### Proposed discovery acceptance evidence and dependencies

Once authorized, verify every documented source/override/container and global-versus-project exclusion; existing files with no global map; missing/default-failed versus explicit failures; duplicate/ambiguous containers and JSON/JSONC duplicate keys; partial-entry safety and stable ordering; same names/packages across sources; disabled-source opt-in versus default selections; observed variable names without resolution; remote/unknown/shell/helper/relative-source blocking; secret-bearing argument/header/URL/default/reference fixtures without value leakage; no downloads/provider/process/source-write effects; source changes before installation; UI-only setup and matching guidance/eligibility independent of prose.

Supported harness releases, exact launcher/argument grammar, parser libraries, path canonicalization/identity and finite local-read limits remain later implementation choices. No live discovery/parser/application tests or personal-config inspection have run. D35 approves the presented scope/schema/eligibility/error contract; resolve the listed mechanisms before implementation. D36 subsequently approves the separate progress refinements.

## Accepted: submit installation

```http
POST /api/v1/setup/installations
```

### Accepted request shape

One request contains the selected entries from discovery, referenced by file and configuration entry name:

```json
{
  "servers": [
    {
      "config_path": "/Users/example/.codex/config.toml",
      "server_name": "example-node"
    },
    {
      "config_path": "/Users/example/.claude.json",
      "server_name": "example-python"
    }
  ]
}
```

- Fetch reads/resolves these entries; the client does not need to reconstruct commands or resend whole configurations.
- Existing local scripts are prepared using their original paths/non-secret launch settings. npm/uvx package entries use the agreed private runtime and per-server package arrangement.
- Existing credential values are excluded. Fresh authentication is a separate step.
- The operation must retain the approved installation/launch information needed after original harness entries are eventually removed; exact persistent schema is not designed yet.
- Installation owns automatic generation/persistence of each server's Fetch configuration from the discovered non-secret package/launch settings and credential references. Clients do not call a separate config-creation endpoint. Exact creation timing/transaction behavior on failed installations remains to be specified.

### Accepted asynchronous response and prerequisite

- Return immediately with `202 Accepted` and one installation ID tracking the selected group.
- Continue installation in the backend; report per-server results through the progress operation.
- Instruct the agent to proceed only when the overall installation succeeds. If it fails, stop the dependent setup flow and report the problem.
- Backend readiness checks enforce installation prerequisites rather than relying only on agent instructions.

The following response example was accepted in the conversation; enum completeness and action identifiers still need final contract review:

```json
{
  "installation_id": "example-id",
  "status": "installing",
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_installation_status",
      "instructions": "Check progress using this installation ID. Proceed only when status is succeeded. If status is failed, stop setup and report the problem."
    }
  ]
}
```

### Open installation details

- Empty/duplicate entries, conflicts with existing installs, source changes between scan and installation, supported launch forms, and before/after-acceptance validation.
- Sequential versus parallel package preparation; install concurrency was not chosen.
- Partial-failure continuation and retrying failed entries through the same endpoint are now accepted below. D34/D36 below subsequently settle explicit retry/no-exactly-once, polling/current-run lookup and basic Quit behavior; concrete cancellation/cleanup/state mechanisms remain open. PostgreSQL-backed persistence/long-term retention are now deferred under [storage-scope.md](storage-scope.md).
- Exact runtime/package-manager preparation, fresh-auth/cache isolation, package lockfiles, and installed-server IDs/metadata.

### Approved installation-submission contract

October 27, 2026: the user replied "yep good" to the presented installation-submission contract and highlighted source-capture/reuse choices (D34). The shown request/response, validation/active-group/sequential-item rules, current-source capture, pinned-record reuse, preparation boundaries, partial-failure/retry/errors and basic Quit behavior are approved as presented. Concrete source identity/path/preparer/deadline/process-cleanup mechanisms remain later design work. D36 subsequently approves the separately presented progress-GET refinements and full same-run result lookup. Application implementation remains paused.

```http
POST /api/v1/setup/installations
```

Use the accepted `servers: [{config_path, server_name}]` body above. No commands, raw configuration, credentials, package overrides or new snapshot token are added.

#### Approved before-acceptance validation and callers

- Allow Fetch Settings and authenticated setup clients with server-management permission. UI-only installation is the base path (D32), using the same backend permission/prerequisite checks as optional agents. Validate authorization before exposing active-job/source information.
- Require a JSON object containing exactly one non-empty `servers` array. Each item has exactly `config_path` and `server_name`, both non-empty/non-whitespace strings without NUL. `config_path` must be an absolute local file path; no tilde/shell/environment expansion or trimming. Match entry names exactly/case-sensitively. File existence/parse/entry validity is a later item check, not a request-wide failure.
- Reject missing/malformed/non-object body, unknown fields/query parameters, wrong types, empty selections and exact duplicate `(config_path, server_name)` request pairs with `400 INVALID_REQUEST`, without creating a job or echoing rejected values. Shared discovery path normalization and allowed-source boundaries remain to review; this route cannot bypass them. Exact-string duplicate rejection does not claim to resolve filesystem aliases.
- Use one active installation group globally and no additional queue. Another submission, including identical input, returns `409 INSTALLATION_IN_PROGRESS` with the authorized caller's active `installation_id` and existing progress-monitoring guidance. Register/check the active group atomically so concurrent calls cannot create competing groups.
- Register the accepted job/ordered item references before returning `202`. Failure to accept/register returns `500 INSTALLATION_ACCEPTANCE_FAILED`; do not pretend unaccepted work is running. Concrete current-run state mechanism is not selected here.

#### Approved accepted response

```json
{
  "installation_id": "installation-42",
  "status": "installing",
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_installation_status",
      "instructions": "Read this installation's progress after one second. Continue dependent setup only after overall success; otherwise report and resolve failed entries."
    }
  ]
}
```

Retain the accepted response fields. Add `Location` pointing to `/api/v1/setup/installations/{installation_id}` and `Cache-Control: no-store`. UI stores the structured ID and begins its built-in progress monitoring, not executing/parsing guidance. `202` is acceptance only, not successful package preparation, authentication or MCP readiness. D33 requires UI/guidance to agree on these gates.

#### Approved sequential preparation, reuse and source capture

Prepare accepted items sequentially in submitted order. A bad individual source/package/launcher must fail its own item after acceptance, without rejecting the other selections or resetting successful items.

For each item:

1. If its source-entry identity already maps to a successful Fetch-installed record, reuse that saved installation as a successful no-op. Preserve its pinned software, saved launch config and Fetch credentials. Do not reread a changed source definition to silently overwrite/update it. An install retry is not a package-update/config-reset operation.
2. Otherwise, read the current source file/selected entry once when that item starts, normalize the supported non-secret launch definition and capture it for that preparation attempt. Follow the same allowed-source/format/safe-classification policy as discovery. Missing/unreadable/malformed file, absent entry, unsupported launcher or unsafe/unclassifiable settings fail that item safely.
3. Prepare from the captured definition; later source-config edits do not retarget it. The capture is not a copy/freeze of local script code or credentials. Existing scripts still run in place under the selected product design.

The current request contains no discovery fingerprint. It therefore **does not guarantee the exact definition displayed by an earlier discovery**; it selects the definition present when that new/failed item's preparation starts. Do not claim stale-source detection or silently add a snapshot/confirmation feature. This tradeoff must be presented with the contract.

Source-entry association conceptually uses source path plus entry name, not just package name. Different harness entries pointing to the same package stay distinct selections; do not merge credentials/configs based only on package equality. Exact path canonicalization/alias handling, stable IDs and source-to-installed association representation remain step-5 choices before implementation. An already installed association uses saved Fetch state, so later removal of the original entry does not reset/recreate the successful installation.

#### Approved preparation success and data boundaries

- npm/uvx preparation uses the agreed private compatible runtimes and per-server dependency directories. Honor the requested version; resolve/store the exact installed version and keep it pinned. Omitted requested version and literal `latest` retain their distinct saved semantics. No launch-time update or implicit version change on a successful-record reuse.
- Validate supported existing local script launch/path requirements and keep scripts in place; do not copy/edit source trees or start the ordinary MCP server as an installation health check.
- Never migrate original credential values or unrelated auth caches. New installation carries only safe settings, credential-variable names/references and fresh-auth requirements. Reusing an existing successful Fetch installation preserves its own credentials; it is not a migration from personal harness/CLI caches.
- Generate and save Fetch's configuration automatically, not through a separate config-create endpoint. Retain enough safe source/identity/launch/package/runtime/path/pinned-version metadata to work after original harness entries are removed. Durable per-server data uses D28 outside-package directories; exact formats/IDs are not selected by this route.
- Publish the installed record and report item success only after preparation plus safe configuration/metadata save completes and a real installed-server ID is available. Missing credential values do not by themselves prevent preparation success. Preparation does not prove provider auth or MCP readiness, and it does not claim arbitrary wrappers support controlled fresh login.
- Do not automatically authenticate or start/restart a newly prepared ordinary server, edit original harness connections, remove original entries or standardize native-server settings. Follow-up UI stages/agent guidance inspect installed server state through the existing GET and perform only separately approved actions.

#### Approved failure, retries and owned cleanup

After acceptance, source/preparation/save failures appear through progress as item `failed` with safe `INSTALLATION_FAILED` cause; do not retroactively turn them into POST request errors. Omit raw commands/source configs/package-manager output and all credentials.

Continue remaining entries and retain committed successes. The group remains active while any item is pending or installing; once all are terminal it fails if any item failed, otherwise succeeds. This aligns with the progress contract subsequently approved in D36. Overall failure stops dependent setup; both UI/guidance report failures and retry only failed entries through this same POST after resolving causes.

After a terminal group, a later POST creates a new group ID. Failed entries can prepare again; already successful source entries are reused. No idempotency key/exactly-once guarantee or implicit reinstall/update feature is added. A lost response does not undo accepted work: monitor a known ID; an overlapping explicit request can reveal the active job through `409`. Do not blindly replay a POST to recover a status-read failure.

Clean only incomplete candidate/staging artifacts owned by the failed preparation. Do not delete other installations, previously saved credentials, or shared runtimes still used elsewhere. Failed candidates must not be published as usable installed records. Exact safe staging/publication/cleanup and source identity mechanisms still require implementation review.

#### Lifetime proposal and approved basic Quit behavior

Background/red-close/client disconnect does not cancel accepted backend installation. Propose accepted jobs/terminal results remain addressable throughout the same Fetch run, without choosing RAM-only/files/database storage or historical retention. No cross-Quit/crash/relaunch job recovery is promised.

For explicit Quit, propose stopping further item starts, ending Fetch-owned installer work and preserving committed successful installations. Unfinished items are not success. A later submission retries preparation rather than assuming an old job recovered. No automatic resume/retry, cleanup of original harness entries or universal orphan-free cleanup guarantee is selected. Shared runtimes used by existing servers are preserved.

Before implementation, select finite source-read/network/subprocess deadlines and timeout termination/Quit cleanup for each supported preparer in the implementation/lifecycle review. A timeout fails the affected item and permits remaining accepted work to continue under safe owned-process coordination. The proposal does not invent a single numeric timeout for every package/runtime or promise process cleanup before that design is chosen. These unresolved mechanisms are not authorization to wait forever or advertise tests that have not run.

#### Approved request errors and guidance

All request errors have `status: failed`, safe `error: {code,message}` and `next_steps`; no raw input/secrets or target/job metadata before authorization.

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Invalid batch shape/path/name/duplicates/query; no accepted job. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN` | Missing/invalid Fetch login or setup/management permission. |
| `409` | `INSTALLATION_IN_PROGRESS` | One group is already active; include its `installation_id` and monitoring guidance, do not queue. |
| `500` | `INSTALLATION_ACCEPTANCE_FAILED` | Job registration cannot complete; no `202` for unaccepted work. |

Accepted/busy guidance uses `agent/check_installation_status`, the relevant existing group GET URL and one-second polling. Invalid requests use safe correction guidance where meaningful; never ask an agent for credential values. `next_steps` always exists, with `[]` when no agent action is available. UI uses structured status/ID/errors and its own matching deterministic sequence; neither guidance nor UI grants permissions or performs a hidden retry.

#### Proposed acceptance evidence and remaining dependencies

Once implementation is authorized, verify malformed/empty/duplicate request versus per-item file/entry/package failure; single active group under concurrent requests; sequential continuation and retained successes; reuse without config/credential/version resets; source edits before/after capture and source removal after successful installation; distinct harness entries sharing packages; private runtimes/pinned versions/direct scripts/no credentials migration; safe publication and owned failed cleanup; same-run lookup/UI closure and selected timeout/Quit behavior; uncertain response and busy-ID recovery without duplicate installs; UI-only base setup and matching agent guidance from the same rules.

Supported launch/preparer/source-reader details, discovery file/path boundaries, stable identities, saved formats/safe writes, finite deadlines and process cleanup remain explicit later design dependencies. No app/package/HTTP/credential tests have run. D34 approves the presented observable submission contract. Resolve the listed mechanisms before implementation; D36 separately approves progress-GET refinements and complete current-run result lookup.

## Accepted: installation progress

```http
GET /api/v1/setup/installations/{installation_id}
```

- Report overall installation status and per-server progress/results.
- While work continues, tell the agent to check again and not proceed until overall success.
- Once preparation succeeds, guide the remaining authentication/configuration steps.
- Installation success does not itself mean authenticated or ready for tool calls.

Working response example discussed and accepted in purpose/shape:

```json
{
  "installation_id": "example-id",
  "status": "installing",
  "servers": [
    {
      "config_path": "/Users/example/.codex/config.toml",
      "server_name": "example-node",
      "status": "succeeded"
    },
    {
      "config_path": "/Users/example/.claude.json",
      "server_name": "example-python",
      "status": "installing"
    }
  ],
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_installation_status",
      "instructions": "Installation is still running. Check again and proceed only when the overall status is succeeded."
    }
  ]
}
```

D36 approves the full read contract below, including its schema, current-run lookup and errors.

### Approved installation-progress contract

October 27: the user replied "good" to the complete remaining progress-read contract (D36). The exact read projection/group/item enums/request errors/guidance/polling and full same-run lookup guarantee are approved. D34's installer/source/retry/basic Quit rules stay intact. This closes the reviewed setup/server-management route pass and moves discussion to step 2 tool sharing, not application implementation. Concrete credential-verifier and implementation choices remain explicit later work.

Read the accepted submission’s path/name/order metadata and actual backend results. D34 source definitions are captured only when new/failed preparation starts, not at discovery or as a whole-group source snapshot. Do not install/retry, reread source configs, open login, probe providers, call tools, start/stop processes or change saved data. This read does not test credentials or assert MCP readiness.

#### Approved input and callers

Settings and authenticated setup/management clients with server-management permission may read progress. Enforce authorization before target existence. Installation IDs are opaque/case-sensitive; match their decoded value exactly. Empty/whitespace-only IDs and any query parameters return `400 INVALID_REQUEST`; do not select an ID-format regex or normalization. No pagination/percentages/stage selector is added. Return entries in accepted submission order from captured metadata, not freshly reread files.

#### Approved response and states

```json
{
  "installation_id": "installation-42",
  "status": "installing",
  "servers": [
    {
      "config_path": "/Users/example/.codex/config.toml",
      "server_name": "example-node",
      "server_id": "server-a",
      "status": "succeeded",
      "error": null
    },
    {
      "config_path": "/Users/example/.claude.json",
      "server_name": "example-python",
      "server_id": null,
      "status": "installing",
      "error": null
    }
  ],
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_installation_status",
      "instructions": "Read this installation URL again after one second. Continue dependent setup only when the whole group is succeeded."
    }
  ]
}
```

All shown fields are present. Group status is `installing`, `succeeded` or `failed`; item status is `pending`, `installing`, `succeeded` or `failed`.

- `pending`: accepted preparation not yet started. `installing`: backend preparation underway, without implied percentage or invented stage.
- An item `succeeded` only after preparation and its generated saved config complete, with a real `server_id` available. It does not assert running/authenticated/MCP-ready state. `server_id` is `null` until actually known; a failed item may retain a known ID without implying usability. Exact identity/creation mechanisms remain later work.
- `error` is `null` except item `failed`, where it is `{code: INSTALLATION_FAILED, message: ...}` with a safe cause. No raw package-manager/process/source configuration or credentials. No duplicate root error is required for terminal job failure.
- While any item is pending/installing, group status stays installing, even after another fails. Finish other accepted items and retain successes. Once all are terminal, group status is failed if any failed, otherwise succeeded.
- HTTP `200` means progress was read, including a group whose operation status is failed. Do not convert installation failure to a read error or pretend a read error changed the real operation outcome.

The submit/install contract owns empty-submission validation, source changes, preparation limits/deadlines and active-work cleanup. GET only reports actual accepted outcomes; it does not select a read-side installation timeout or fabricate an empty completed group.

#### Confirmed UI consumption requirement — D32

The default user path is Fetch's own installation UI with no connected agent. The installation POST/GET and subsequent server-management APIs support both the UI and optional agent clients. D22's common `next_steps` field remains in responses; D32 makes it agent-only guidance. The UI uses installation group/per-item status, resulting IDs and structured errors for presentation, then its defined post-install sequence and existing server GET fields. It must not infer success/actions from `next_steps.instructions`, action strings, ordering or presence.

UI mapping uses the approved D36 progress states: show/monitor progress while installing; show actual failed items and retained successes when failed; after succeeded, inspect the resulting server IDs through existing GET and show applicable auth/Settings/restart controls from structured state. No automatic retry/login/restart is selected. The backend still enforces dependencies and permissions on every operation; deterministic UI sequencing is not a second installation/auth implementation. Exact onboarding branches/UI state design remain for step 6, with first-setup authority in step 4.

A `next_steps` entry with `actor: user` lets an agent ask the user to act. It is not a UI instruction stream. No additional `ui_steps`/raw instructions field or route is introduced. D36 subsequently approves the complete progress read contract; exact UI screens/branches remain later work.

#### Approved guidance, monitoring and lifetime

Always return `next_steps` as agent-facing guidance only, using the shared actor/action/instructions structure. D32 UI status/sequence uses structured fields instead:

- Installing: `agent/check_installation_status`, this exact installation GET URL and one-second polling; do not proceed with dependent setup yet.
- Succeeded: `agent/check_server_status`, existing server GET URLs/IDs and instructions to follow their auth/config guidance. Installation success itself proves neither verified credentials nor MCP readiness.
- Failed: `agent/review_installation_failures`, stop dependent setup, report failed entries, resolve causes and retry only those through the existing installation POST. This is guidance, not an automatic retry or permission grant. Existing successful installs remain intact.

Return `Cache-Control: no-store`; poll at most once per second while work is active. GET is safe to repeat. A lost/read-failed GET does not fail/cancel/recreate the installation; retry the GET, not automatically the submit POST.

Approved minimum observable lifetime: accepted jobs and terminal outcomes stay addressable throughout the same Fetch run. Background/UI closure does not discard status or cancel accepted work. This selects no memory-only/file/database mechanism or long-term retention. No ID recovery is promised across Quit/crash/relaunch; unavailable prior-run IDs use not-found, and fresh startup must not fabricate success for an old job. D34’s approved basic Quit behavior applies. Exact finite deadlines, process termination and safe cleanup mechanisms remain for implementation/lifecycle review; this GET neither reopens them nor promises recovery.

#### Approved progress request errors

```json
{
  "status": "failed",
  "error": {
    "code": "INSTALLATION_STATUS_UNAVAILABLE",
    "message": "Fetch could not read installation progress."
  },
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_installation_status",
      "instructions": "Progress could not be read. Retry the same GET; do not resubmit installation."
    }
  ]
}
```

All request failures use `status: failed`, safe `error: {code,message}` and `next_steps`. The read-error example gives the agent the same safe-GET-retry option that native UI can offer under D33; it is not a command to automatically replay a mutation. Other errors use relevant safe guidance or [] when no next action is known. Do not leak target/source details before authorization. The error response is distinct from a retrieved HTTP-200 group with failed items.

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Invalid ID/query. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN` | Missing/invalid Fetch login or management permission. |
| `404` | `INSTALLATION_NOT_FOUND` | No addressable accepted job for this ID. |
| `500` | `INSTALLATION_STATUS_UNAVAILABLE` | Required progress cannot be read; preserve actual known job outcome, no automatic resubmit. |

#### Proposed progress acceptance evidence

Once authorized, verify mixed pending/active/failed completion and retained successes; real IDs before successful items without auth/readiness claims; submission order/captured metadata after sources change/disappear; no read-side install/provider/source/write effects; denied/unknown/read-failed versus retrieved failed jobs; guidance and one-second polling; current-run terminal lookup/UI-close behavior without invented percentages or cross-launch recovery; agent-free UI installation and post-install sequencing from structured state, invariant when only valid agent guidance changes. No app/HTTP tests have run. D36 approves the presented schema/error/action/lifetime choices; underlying state/identity/storage/cleanup mechanisms remain implementation decisions.

## Accepted: installed-server information and ID filtering

Accepted route:

```http
GET /api/v1/servers
```

### Confirmed user requirement

The user explicitly requested input of a single server ID or multiple IDs to obtain more information about those servers, then approved the query/response behavior below. The operation supports listing installed servers and inspecting selected ones, with non-secret configuration/runtime/authentication information.

### Accepted input and missing-ID behavior

Use repeated optional `id` query parameters:

```http
GET /api/v1/servers
GET /api/v1/servers?id=server-a
GET /api/v1/servers?id=server-a&id=server-b
```

- No IDs: all installed servers.
- One/more IDs: detailed records for the matches.
- Always return a `servers` array, even for one requested ID.
- Return found records plus `not_found_ids` for missing requested IDs.

These details were initially pending when the user requested the documentation sync. The user subsequently quoted the input/missing-ID question and answered yes. The repeated query syntax, all-versus-selected behavior, always-array response, and `not_found_ids` are now confirmed.

D23 approves the full ordinary record, ID validation/deduplication/order and request/error behavior below, including `200` for all-unknown ordinary IDs. Provider credential values must not appear in server records.

The user also approved reusing this operation for restart progress/completion and explicitly selected an additional `restart_id` query parameter, for example `GET /api/v1/servers?id=github&restart_id=restart-42`. This selects the named attempt for the selected server. Omit `restart_id` for ordinary server information. Restart-specific fields, filtered-query validation and errors are approved under D20; see [restart-contract.md](restart-contract.md). D23 approves the ordinary GET field/envelope/validation rules below.

### Approved complete record and ordinary GET rules

October 27, 2026: after approving restart, the user explicitly reaffirmed reusing the existing GET, then moved the server family outside setup (D21). The resulting route is `GET /api/v1/servers`, preserving the existing `/api/v1` base. After clarification of `restart_required` and the shared `next_steps` convention, the user said "Kk good to me then you’re good to move on." D23 approves the fields and rules below for the existing GET. This does not approve config/credential PATCH schemas or auth integration mechanisms.

Read Fetch's saved installed-server settings, safe credential-presence metadata and latest known runtime/authentication state. Do not reread original harness files, start/restart processes, open login, validate credentials against providers, invoke tools or change saved settings. GET does not wait for a server to become ready. Use the same full server record for all-versus-selected requests; no separate summary selector or pagination is proposed for this local MVP.

#### Approved ordinary input and defaults

- No `id`: all installed records, sorted by case-sensitive `server_id`.
- Repeated `id`: matches in first-requested order, deduplicating identical IDs. `not_found_ids` uses the same first-requested order.
- IDs are opaque and case-sensitive; match their decoded value exactly. Reject empty/whitespace-only IDs and unknown query parameters with `400`; do not trim, lowercase or invent an ID-format regex. ID representation remains a later implementation choice.
- An empty installation or all-unknown ordinary ID query returns `200`/`status: ok`, with empty `servers` and the applicable `not_found_ids`. Missing ordinary IDs do not turn a successful read into `404`.
- Settings and authenticated clients with server-management permission may read the same safe fields. OAuth scope names/enforcement mechanisms remain for step 4. IDs do not confer access.
- Return `Cache-Control: no-store` for ordinary reads. Poll at most once per second while monitoring an active operation, using GET only. No automatic POST retry is implied.

#### Approved ordinary response

Invented names, package and paths; this is not an installed server. Stored credential presence intentionally does not establish authenticated status.

```json
{
  "status": "ok",
  "servers": [
    {
      "server_id": "server-a",
      "name": "Example server",
      "runtime": {"status": "ready", "error": null},
      "authentication": {"status": "unknown", "error": null},
      "restart_required": false,
      "restart": null,
      "config": {
        "kind": "npm",
        "package": {
          "name": "@example/server",
          "requested_version": null,
          "installed_version": "1.2.3"
        },
        "executable": "/example/fetch/node",
        "args": ["/example/fetch/server.js"],
        "working_directory": null,
        "environment": [
          {"name": "EXAMPLE_API_KEY", "credential_ref": "example-key"}
        ]
      },
      "credentials": [
        {"name": "example-key", "status": "stored"}
      ]
    }
  ],
  "not_found_ids": [],
  "next_steps": []
}
```

Approved fields, always present unless a nested operation specifies otherwise:

| Field | Meaning and allowed values |
| --- | --- |
| `server_id`, `name` | Stable installed-record reference and display name; display names need not be unique. Stable ID generation remains step 5. |
| `runtime.status` | `stopped`, `starting`, `ready`, `restarting`, `failed`, `unknown`. `ready` means known MCP responsiveness/tool discovery, not provider authorization or a fresh health probe on GET. `unknown` means Fetch lacks evidence of the state. |
| `runtime.error` | `null` except in `failed`, where it contains `{code: SERVER_UNAVAILABLE, message: ...}` with a safe explanation. The restart object retains its approved, more precise failure codes. |
| `authentication.status` | `not_required`, `required`, `authenticating`, `authenticated`, `failed`, `unknown`. Confirm `authenticated` only from a supported integration's evidence, never credential presence or tool listing alone; it does not guarantee every action is permitted. `unknown` is not `not_required`. |
| `authentication.error` | `null` except in `failed`, where it contains a safe `{code, message}`: `CREDENTIAL_VERIFICATION_FAILED` for a failed required stored-credential test (D31), otherwise `AUTHENTICATION_FAILED`. Exact provider login methods and integration behavior remain in the authentication contracts/step 4. |
| `restart_required` | Saved launch/config/manual environment changes await application. A successful restart clears this only if its captured settings match the latest saved settings; an edit during restart keeps it true. This is applied-state tracking, not revision history. Automatic OAuth refresh behavior remains integration-specific. |
| `restart` | Active restart with its approved ID/status/phase, otherwise `null` in the ordinary view. This projection neither deletes completed attempts nor chooses their lifetime. Use the approved `restart_id` query for a particular result. |
| `config.kind`, `package` | `npm`, `pypi` or `local_script`. `package` is the shown object for package installs and `null` for local scripts. Requested version `null` means omitted, distinct from literal `latest`; installed version is the pinned resolved version. |
| `config.executable`, `args`, `working_directory` | Normalized saved non-secret launch settings; arguments are a string array, working directory a string or `null` for the launcher's saved default. GET returns saved rather than currently applied settings. No raw imported command/config is returned. |
| `config.environment` | Environment-variable names and opaque references to this server's assigned entries; never values. The reference in the example points to the manual entry named `example-key`. Exact storage/ID representation is not selected here. |
| `credentials` | Known assigned manual environment/custom credential entries, with `name` and `status: stored`, `missing` or `unknown`. Presence is not validity. Hide OAuth access/refresh material behind authentication status. Empty means no known entries, not a claim that arbitrary wrappers need none. |

A successful read is HTTP `200`/`status: ok` even when a listed server is stopped, failed or needs login. Its fields describe that server's condition. Per-server runtime/authentication errors never replace the top-level read status. All errors/messages omit raw stderr, provider bodies and stored credential values.

Returning launch fields safely depends on the discovery/install/config contracts settling secret classification. Do not implement a generic name-based redaction shortcut. If Fetch cannot safely read/serialize required records, fail the read as below rather than treating records as missing. This read contract selects no file encoding, path layout or encryption-key mechanism.

#### Approved ordinary next steps

Use the accepted `actor`/`action`/`instructions` shape, with an additional `server_id` on each ordinary per-server step so guidance for multiple records has an explicit target. Approved actions are:

| Actor / action | When used |
| --- | --- |
| `agent` / `check_server_status` | Runtime startup or authentication is pending; check this same server GET again after one second. |
| `agent` / `check_restart_status` | Restart is active; instructions give the approved server/attempt-specific GET URL. |
| `agent` / `authenticate_server` | Login is required; use the accepted authenticate POST to start supported login or obtain the appropriate user instructions. GET itself does not initiate login. |
| `user` / `review_server_settings` | Manual entries are missing, saved changes need restart, a server/auth flow failed, or setup is unknown; instructions explain the particular issue. Displaying guidance does not perform its action. |

For example, a pending login can return:

```json
{
  "actor": "agent",
  "action": "check_server_status",
  "instructions": "Check this server again after one second while authentication is pending.",
  "server_id": "server-a"
}
```

Return zero or more applicable steps in server-response order; within a server, pending-operation monitoring comes first. No steps means no known setup action, not proof that every external tool action will succeed. The already-approved restart-filtered response/next-step shape is unchanged.

#### Approved ordinary request errors

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Invalid ordinary query or unknown query parameter. |
| `401` | `UNAUTHENTICATED` | Fetch login absent/invalid. |
| `403` | `FORBIDDEN` | Server-management permission absent. |
| `500` | `SERVER_INFO_UNAVAILABLE` | Fetch cannot safely read the required server/config/status metadata; never report an unreadable record as absent. |

Ordinary error replies have `status: failed`, `error: {code, message}`, `servers: []`, `not_found_ids: []`, and `next_steps: []`. Validate authorization before exposing any server information or missing IDs. GET is safe to repeat; a transport/read error does not change any operation result.

#### Approved restart-filtered exception

The approved `restart_id` query selects exactly one server/attempt and retains [the restart contract](restart-contract.md) unchanged, including its minimal `server_id`/`restart` record, `200` nested outcomes, `400` selector checks, `404` missing/mismatched IDs and `RESTART_STATUS_UNAVAILABLE` read failure. Ordinary full fields are not required in that filtered response. Omit `restart_id` to read saved settings and general state. This is one GET route with two accepted query purposes, not another endpoint or job-history feature.

#### Proposed acceptance evidence

Actual HTTP checks, once implementation is authorized: all/selected/duplicate/unknown ID ordering and empty results; pending/failed/unknown runtime/auth versus read success; server-scoped credential presence without values, including deliberately embedded-secret fixtures; reads after original harness entries are removed; no process/login/tool/file-write side effects; denied callers and unreadable records; config edits during restart keep `restart_required`; ordinary active versus named completed attempts follow their separate views. None has been implemented or run.

## Accepted: start server authentication

Accepted route:

```http
POST /api/v1/servers/{server_id}/authenticate
```

The user approved the resumed setup block on October 26, including this behavior:

- Input is the installed server ID in the path; no credential values are supplied in this request.
- For a supported browser OAuth flow, Fetch opens the system/default browser and returns immediately with `202 Accepted`.
- User action completes sign-in. The agent checks the existing `GET /api/v1/servers?id=...` operation for authentication status, avoiding a separate auth-progress route.
- For API-key or macOS-permission setup, return the known Settings/native-permission instructions rather than pretending to start OAuth.
- If a custom wrapper's login mechanism cannot be determined or controlled, report the missing setup/integration information; do not invent an OAuth flow.

Illustrative response for the accepted OAuth branch; complete endpoint-wide schemas/action vocabulary still need final review:

```json
{
  "server_id": "server-a",
  "status": "authenticating",
  "next_steps": [
    {
      "actor": "user",
      "action": "complete_browser_login",
      "instructions": "Complete sign-in in the browser Fetch opened."
    },
    {
      "actor": "agent",
      "action": "check_server_status",
      "instructions": "Check this server using the server-information API. Continue its setup only after it is authenticated."
    }
  ]
}
```

No token/credential values are returned. D23 approves the server GET authentication status fields. Actual provider integrations and token handoff/persistence remain unresolved; the general route-level rules below are approved in D27.

### Approved general authentication-start contract

October 27, 2026. This uses the existing authenticate POST and approved server GET, not another route or auth-attempt ID. Existing method/path/OAuth/manual/unknown branches remain accepted in purpose. D27 accepts the previously reviewed general validation, response, overlap, deadline and retry rules, subject to D26 per-server encrypted credentials. Concrete integration/storage/test mechanisms and the new GitHub CLI proposal remain open. This API proposal does not choose OAuth wire endpoints, crypto/key handling or concrete wrapper integrations.

#### Approved input, callers and side effects

- Accept no body or `{}`. Reject malformed JSON, non-object bodies, any body fields or query parameters with `400 INVALID_REQUEST`; never echo rejected values. Manual credentials/auth codes/tokens do not belong in this input.
- Settings and authenticated clients with server-management permission may call it. Check authorization before revealing whether the target exists.
- Unknown server: `404 SERVER_NOT_FOUND`. Known record whose installation is unfinished: `409 SERVER_NOT_INSTALLED`. Require installation, not MCP readiness; some supported integrations need login before tool discovery can succeed.
- A currently active restart returns `409 SERVER_BUSY` with guidance to monitor its existing server/restart GET. Check this after the target prerequisite and before creating/reusing a login flow.
- Read saved server/integration metadata and known auth state. Only an explicitly supported integration may start its known login flow. Fetch-owned OAuth may open the default browser and save fresh credentials in the selected encrypted, server-scoped store; wrapper-owned OAuth must meet the ownership/storage contract below rather than being assumed to export its tokens. Do not reinstall/update packages or automatically restart the ordinary server here. Login-helper requirements belong to the concrete integration design.
- Return `Cache-Control: no-store`. `202` includes `Location` pointing to `GET /api/v1/servers?id={server_id}`. No credential/token/code/login-URL values or raw provider response bodies are returned.

#### Approved branch results

After the prerequisite checks, reuse any active login before deciding whether to start another:

| Condition | HTTP / top-level `status` | Result |
| --- | --- | --- |
| This server already has an active login | `202` / `authenticating` | Guide the existing flow; do not reopen the browser, replace the flow or extend its deadline. |
| Login is known unnecessary | `200` / `not_required` | No browser, `next_steps: []`. |
| Supported integration already confirms login | `200` / `authenticated` | No new browser login is needed if the existing credential works. Read it back from its successful encrypted storage and perform the authenticated test before this POST returns success; do not use presence or an old unverified result. |
| Supported browser OAuth can begin | `202` / `authenticating` | Begin one flow, open the default browser, return immediately and guide GET polling. |
| Known manual API-key/custom-token or macOS-permission setup | `200` / `required` | Specific Settings/native-permission instructions; do not claim browser OAuth or write manual values. |
| Login mechanism cannot be determined/controlled | `200` / `unknown` | Explain missing setup/integration information; do not guess OAuth or tell the agent to proceed. |

Each successful branch returns `server_id`, `status`, and `next_steps`. The existing OAuth example above remains its response shape. Different servers can authenticate independently. Guidance uses these reviewed route-specific actions:

- `user` / `complete_browser_login`: complete the supported login already opened.
- `agent` / `check_server_status`: GET this server once per second while its authentication is pending; proceed only after `authenticated` or confirmed `not_required`.
- `user` / `review_server_settings`: explain known manual/native setup, missing integration information or how to correct a failure. Never ask for a secret in an agent prompt.
- `agent` / `check_restart_status`: for a busy restart, point to its approved server/attempt-specific GET.

Instructions are specific to the server; the root `server_id` identifies it. This contract retains the common actor/action/instructions structure without requiring an extra per-step target for a single-server response. Empty arrays remain present under D22.

#### Proposed provider authentication and credential-storage contract

October 27 follow-up: the user explicitly requires actual server authentication and credential storage to be accounted for. This is a confirmed scope requirement (D24), not approval of the particular mechanisms below. D27 later accepts the reviewed general route policies. [Current standards/implementation research](research.md#actual-mcp-auth-and-token-ownership--rechecked-october-27) shows no universal local-MCP provider-token export.

Keep three credential boundaries distinct: the assistant's access to Fetch; Fetch's access to an MCP where applicable; and the MCP's access to GitHub/Google/another provider. HTTP OAuth token endpoints standardize the relevant client token exchange. That does not make provider secrets normal MCP tool results. Local stdio credential provision/login remains integration-specific; third-party tokens obtained through URL elicitation stay owned by that server, outside the MCP client.

For every automatically supported integration, review a complete internal agreement covering:

| Question | Required definition/evidence before claiming support |
| --- | --- |
| Who performs login and receives tokens? | Name Fetch's trusted backend or the wrapper as the provider OAuth client; identify how login starts and callbacks/results belong to the intended server/account. Browser opening or URL consent alone is not completion. |
| Where do secrets live? | Identify the authoritative store and whether tokens exist only in process memory or survive restart. Fetch-managed tokens/keys/environment values use each server’s encrypted credential configuration files (D26); non-secret config may hold references. Key protection/file encoding remain open. |
| How does the running process get access? | Use its documented environment/token-provider/storage interface. Supply only assigned credentials. No token scraping from stdout, stderr, ordinary tool results or unrelated existing caches. A private provisioning interface is not a new public credential-return API. |
| Who refreshes? | Identify Fetch or the wrapper as owner, how new/rotated credentials replace previous ones safely and how expiry/no-refresh/failure is reported. Do not promise refresh tokens always exist or assume a wrapper refreshes Fetch's separate copy. |
| What happens on restart/reopen? | Verify fresh credentials are used without migration, encrypted stored state can be reapplied through the documented interface, and memory-only login loss triggers truthful reauthentication guidance. Current usable login does not itself prove durable login. |
| What proves authentication? | A supported integration's completion/handoff/storage evidence. Mere credential presence, process liveness, tools/list or URL acceptance cannot prove provider login. Keep D23's current-state semantics; do not claim every provider action is permitted. |
| What happens when support is missing? | Return known Settings/native-permission instructions or unknown integration guidance through the existing POST/GET. Do not advertise automatic login, persistence or renewal that Fetch cannot control/observe. |

A Fetch-owned provider login with a documented process handoff can use the chosen encrypted store directly. A wrapper-owned login needs a separately reviewed store/provisioning arrangement compatible with the user's storage requirements. Its private memory or disk cache is not automatically part of Fetch's encrypted store, and this document authorizes neither plaintext-cache exceptions nor token export forbidden by the protocol. Whether to adapt such a wrapper, use another supported login method or defer that automatic integration remains a user choice. Servers can remain installable without claiming automatic auth support; this is not a new package catalogue/allowlist.

The GitHub local server's documented memory-only OAuth storage is an example requiring that review, not proof that Fetch can persist its token. HTTP upstream auth and URL-elicitation capability/consent are researched context; neither is silently added to the initial local-server MVP. The accepted general 10-minute/overlap/retry policy (D27) does not resolve these concrete integration requirements.

When an integration is eventually selected, verify login success, encrypted persistence where promised, proper server-only handoff, expiry/refresh/rotation, logout/revocation behavior, restart/reopen, old-cache isolation and failure reporting without leaked values. Exact logout operations/public routes are not added by listing that integration-lifecycle requirement.

#### Required stored-credential verification — D25

The user explicitly requires authentication success only after a successful credential store and a `2xx` test using that stored credential. They clarified that the test target is **the service's authenticated test endpoint**, not an endpoint literally named lifeboat. This success requirement is confirmed; D27 later approves the reviewed general route policies; D25 itself did not approve them.

For each credential-based authentication flow, require this ordered gate:

1. Save the credential for the selected server in the approved encrypted store, confirming the write succeeded.
2. Read back that flow's assigned credential through the store's normal retrieval path. Do not test only the fresh token still held in the OAuth callback, a stale entry or an unrelated cached login.
3. Use the retrieved credential for a test request to the service's documented authenticated endpoint.
4. Only after that request returns `2xx` may this flow report authentication success or set `authentication.status: authenticated`.

Store/read failure, request timeout, transport failure or non-`2xx` leaves this flow unsuccessful. A public unauthenticated health check cannot satisfy the requirement. This is a provider/service authentication test, not the MCP `tools/list` readiness check. The selected test should be a harmless read which requires the credential; exact endpoint/method and its authentication/success semantics are integration-specific choices still to review. Do not infer permission for all provider actions from this result.

The gate applies before this flow's success is reported. Subsequent status GETs read that verified result without making a new provider request, preserving D23's read-only/latest-known-state contract. The already-authenticated POST branch also reads back and tests the existing stored credential before returning success, without forcing a new browser login or rewriting an unchanged credential. Additional background recheck/refresh cadence remains an operational policy to review; it does not waive this explicit authentication-operation gate. A configured manual credential can only be called authenticated after the same gate; mere presence still yields no proof.

For server-owned login, the compatible integration must establish the same storage/readback/authenticated-test outcome within the correct credential owner, without exporting third-party tokens through MCP. A memory-only wrapper login does not establish a successful encrypted-store write. If the supported interface cannot establish the required result, do not advertise verified automatic authentication; return truthful required/unknown integration guidance. How such wrapper-owned storage can meet the existing encrypted-store rule is unresolved and requires the user's decision. No storage exception or unsupported token export is selected.

Known no-auth and native-permission-only paths do not invent an HTTP credential test. How those non-token paths report their own successful permission verification must be specified in their supported integrations; this credential-based gate does not settle that policy. Safe retention/replacement/cleanup of a saved-but-unverified credential and recheck/refresh policy also remain for review. No new test endpoint is added to Fetch by this requirement.

#### Shared required verification outcome — D31

October 27: the user requires a successful credentials PATCH `200` to include a test with the newly stored credential, and the same verification-failure code from the earlier authenticate operation. D25 already required authentication success only after encrypted storage/readback and service-authenticated `2xx`; this amendment adds the shared failure outcome, not a claim that storage previously proved login.

Use `CREDENTIAL_VERIFICATION_FAILED` consistently when that required test rejects/fails, times out, cannot run or cannot prove use of the current stored credential. Apply it to a synchronous authenticate error, credentials PATCH verification error and GET `authentication.error.code` after an accepted asynchronous auth flow fails its test. Other login denial/cancellation/expiry/start errors retain their established codes; a failed encrypted save/readback remains a storage failure, not a test failure. Exact non-`200` HTTP mapping and safe cause details remain part of integration/error-policy review; no unshown timeout or generic server call is selected.

A test must actually use the read-back new credential and prove the required authentication. Public health, `tools/list`, or contacting a still-running process that retains an old environment does not satisfy the requirement. The latest phrase "MCP server endpoint" does not establish a universal stdio HTTP endpoint or silently replace D25's previously clarified service-authenticated test with generic MCP readiness. Review the concrete endpoint/request and private handoff with each supported integration before implementation. Never emit raw credential/provider data.

#### Approved login lifecycle, deadline and repeats

- Allow one active login per server. Use a **10-minute overall deadline from acceptance**, shortened by an earlier known provider expiry. D27 accepts this Fetch policy; it is not a protocol requirement. Repeated requests do not extend it.
- While active, server GET reports `authentication.status: authenticating` and `error: null`.
- Set credential-based `authenticated` only after supported login/handoff and the D25 successful encrypted-store write, readback and `2xx` service-authentication test. This describes current verified login; continuing durability/renewal still depends on the supported integration. Browser opening, URL consent, tools/list and stored-value presence are not substitutes.
- Denial, detected cancellation, expiry, accepted-flow startup/storage failure or interrupted login reports GET `authentication.status: failed` and `{code: AUTHENTICATION_FAILED, message: ...}`. A failed required stored-credential test instead reports the shared `CREDENTIAL_VERIFICATION_FAILED` (D31). The safe message states the cause; retain D23's existing error shape/code rather than adding hidden nested enums.
- Closing a browser tab may be unobservable. Without a cancellation/denial result, keep the flow pending until its deadline; do not claim tab closure was detected.
- After terminal success/failure, another explicit POST evaluates current auth state and may start a fresh flow if still needed. No automatic browser/login retry. A repeated POST while pending joins the existing flow, including after an uncertain client response.
- Only the currently active flow may update auth state. Late callbacks/results from expired or replaced flows must not overwrite a newer login.
- UI closure or client disconnection after acceptance does not cancel the flow. Quit retains its selected process-stop behavior. Cross-Quit/crash recovery and local operation-state lifetime remain unselected/deferred; do not promise a flow survives reopening.
- A later permitted restart does not wait for login. If that restart invalidates the supported integration's flow, report authentication failure; an independent browser flow need not fail solely because the ordinary server restarted. D27 accepts this overlap policy.

#### Approved authentication request errors

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Invalid body/query; no values echoed. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN` | Fetch login absent/invalid or management permission absent. |
| `404` | `SERVER_NOT_FOUND` | Unknown server. |
| `409` | `SERVER_NOT_INSTALLED` / `SERVER_BUSY` | Installation unfinished or restart active; do not create login. |
| `500` | `AUTHENTICATION_START_FAILED` | Fetch cannot accept/start this flow; no `202` for unaccepted work. Failures after acceptance appear in the approved GET auth state. |

Request errors return `status: failed`, `error: {code, message}` and `next_steps`. Do not expose target details on authorization failure. The busy branch's safe instructions include the existing attempt-specific monitoring URL; ordinary error replies have no credential/provider details. Example:

```json
{
  "status": "failed",
  "error": {
    "code": "AUTHENTICATION_START_FAILED",
    "message": "Fetch could not start sign-in."
  },
  "next_steps": []
}
```

#### Dependencies and evidence still required

Actual login/storage ownership is now explicitly part of this contract review (D24); do not finalize an automatic-support claim while that agreement is missing. Step 4 still selects the concrete login control, fresh-auth isolation, handoff/persistence, permissions and renewal mechanisms for each supported wrapper. Arbitrary local wrappers do not universally supply a controllable login mechanism; this contract does not promise universal OAuth or working integrations. Exact OAuth wire formats/scopes/key handling remain open. No package installs, login or implementation is authorized by this proposal.

Once implementation is authorized, verify: truthful no-auth/already-auth/manual/native/unknown branches; one flow/browser on repeated/concurrent requests and no extended deadline; GET progress/success/failure; denial, undetectable tab closure, expiry and late callbacks; denied callers/invalid secret-valued bodies without leakage; client/UI disconnection; restart overlap; Quit behavior. These are proposed checks, not tests run.

## Accepted batch-failure policy

The user approved the resumed setup block on October 26:

1. Continue other installations when one server fails.
2. Keep successful installations.
3. Once the selected installations have finished, mark the overall group failed if any selected server failed, with per-server explanations.
4. Guide retry of failed entries through the installation endpoint.

These items were initially pending because the user had only responded to the earlier server-ID proposal. The later explicit approval of both resumed proposals resolves that status. It does not settle exact error formats, retry idempotency/concurrency, or recovery after Quit/restart.

## Accepted: configuration and credential/environment entry

The user approved this two-route block on October 26 and then explicitly extended the credential operation/store to server-local environment entries and custom auth tokens:

| Route | Accepted behavior |
| --- | --- |
| `PATCH /api/v1/servers/{server_id}/config` | Update specified non-secret server settings, such as server arguments, working directory, or credential-variable references. Available to authorized setup agents and the interface. Reuse server information for reads; do not create another config-read route. |
| `PATCH /api/v1/servers/{server_id}/credentials` | Accept new credential/environment-variable values entered through Fetch Settings, including custom auth tokens; save them in the encrypted per-server store and return names/status only. Restrict manual write authorization to Settings rather than the ordinary agent credential, and do not expose a secret-valued MCP tool input. |

The user clarified that Fetch configurations are generated automatically during installation, so a separate configuration-creation route is unnecessary. The config PATCH edits an installed server's generated settings. This does not settle how to add Fetch's own MCP connection to the external harness configurations; that remains a separate onboarding question.

The same store handles per-server environment entries, including ordinary configuration values, API keys, and custom tokens. Keep each entry assigned to its server ID, with names/references in server configuration and no stored values echoed to agents. Do not create a second environment-variable endpoint without discussing it.

The Settings-only restriction applies to manual/external writes. Trusted backend OAuth grant/refresh processing still updates its corresponding stored credentials automatically, as previously selected.

The exact backend authorization mechanism is not designed; merely omitting an MCP tool would not enforce the restriction on the HTTP API. Exact request fields, update/removal semantics, response envelopes, error codes, custom-token bindings, encryption/key handling, and the effect of changes on running wrappers are unresolved.

### Approved common launch-configuration PATCH contract

October 27, 2026: after credential-reference clarification and research, the user deferred standardizing the individual servers' differing settings to a separate post-MVP scope. "For now settings configs should just handle the standardized fields we're handling. Everything else looks good, feel free to move on" approves the previously presented common launch-field contract (D30). These are Fetch's shared fields, not an MCP-standard settings schema. Manual values remain on the separate Settings-only credentials route. Implementation remains paused.

```http
PATCH /api/v1/servers/{server_id}/config
```

Read the current saved server configuration, merge the supplied fields, validate the resulting definition and save it in the D28 durable server-data directory. Do not reread/edit original harness files, update packages/runtimes, write credential values, launch/restart processes, open login or call providers/tools.

#### Approved input and replacement rules

```json
{
  "args": ["/example/fetch/server.js", "--timeout", "30"],
  "working_directory": "/Users/example/projects",
  "environment": [
    {"name": "EXAMPLE_API_KEY", "credential_ref": "stored-entry-7"}
  ]
}
```

The example is invented, not a real package/launcher to install. `EXAMPLE_API_KEY` is only the destination variable’s name; `stored-entry-7` is an opaque reference, not the API key. The agent neither submits nor receives the stored value. A config edit only saves a binding. At the selected launch/apply point, the trusted backend retrieves/decrypts the assigned value privately and supplies it directly to that server through its supported interface; exact handoff/key mechanisms remain step-4 decisions. Secret-valued requests remain restricted to the Settings-only credentials route.

D29's broader native-server settings scope is deferred by D30. The MVP does not normalize arbitrary config files, discover/write every server-specific setting, add raw-file APIs or promise live native-settings changes. The existing common launch fields can still express supported arguments and credential bindings; exact safe launcher validation remains an implementation prerequisite.

Only the following three common fields are writable:

| Field | Approved meaning |
| --- | --- |
| `args` | Full normalized string array, matching GET's representation. Keep the installed launcher/script/package entrypoint prefix byte-for-byte unchanged; only supported trailing server arguments may differ. Replace the entire list; omitted means unchanged. `[]` is valid only if it preserves a valid installed launch definition. |
| `working_directory` | Existing accessible absolute directory, or `null` to restore the saved launcher default. Omitted means unchanged. No shell/tilde/environment expansion, directory creation or copying. |
| `environment` | Full replacement list of `{name, credential_ref}` bindings. `[]` removes editable bindings, not encrypted entries. Names must be unique and match `[A-Za-z_][A-Za-z0-9_]*`; refs are non-empty/non-whitespace opaque strings, matched exactly within this server's assigned credential context. No values, paths or cross-server lookup. A missing local value may remain a missing requirement through GET. Omitted means unchanged. |

`{}` and supplied values identical to the saved definition are valid no-ops. Missing body, malformed JSON, non-object body, unknown fields/query parameters, wrong types and duplicate environment names return `400 INVALID_REQUEST`. Only `working_directory` accepts `null`. Invalid environment-name/ref content is `422 INVALID_CONFIG`; no rejected values are echoed.

The server ID/display name, `kind`, package/version, runtime executable and launch entrypoint are read-only here. The environment list covers editable bindings, not backend-owned runtime/auth material. Package/runtime/entrypoint changes require their separately reviewed operations.

Never accept inline credential values through arguments or ordinary config fields. A supported launcher/integration must establish the editable non-secret argument options. Protected prefix changes, inline credentials and arguments whose secrecy cannot be established return `422 INVALID_CONFIG` with safe guidance. Name-based filtering is not a universal detector. An unchanged known-safe GET array remains a no-op; exact protected-prefix identification and supported option schemas are integration prerequisites before implementation, not a claim of universal arbitrary-argument support.

#### Approved callers, overlap and saving

- Allow Settings and authenticated clients with server-management permission. Authorize before exposing target existence or metadata.
- Unknown server: `404 SERVER_NOT_FOUND`; known unfinished installation: `409 SERVER_NOT_INSTALLED`. Installation is required, MCP readiness is not.
- Active authentication: `409 SERVER_BUSY`, with existing-server-GET monitoring guidance. Do not change an active login's settings.
- Active restart: allow edits. The accepted restart retains its captured settings (D20); any newer saved changes remain for a subsequent restart.
- Serialize writes per server. Read/merge/validate/save is one operation. For overlapping supplied fields the last committed write wins; omitted fields preserve earlier committed updates. Whole arrays are replaced, not appended/merged. No edit-version/precondition or configuration-history feature is added.
- Validate the entire resulting definition before saving. Validation failures change nothing. Complete a safe save before `200`; a save failure must not leave a partial/corrupt record. Exact atomic-write/coordination implementation belongs to step 5. This does not promise crash recovery or choose a retention policy.
- Reference validation may inspect server-scoped metadata; do not decrypt/copy/replace/delete credential values merely to save references. Ordinary non-secret config contains no secret values.

#### Approved application and result

The current process keeps its applied settings. Recompute `restart_required` from saved versus applied launch/manual-environment state. A no-op preserves the existing flag. Restoring settings already matching applied state can clear the flag only when no other unapplied change remains. During restart, its captured settings govern that attempt; success clears the flag only if they match the latest saved settings. Saving never restarts automatically.

Any actual change among these launch/reference fields invalidates saved-configuration authentication evidence: report `authentication.status: unknown` and `error: null` until the supported integration establishes the appropriate result. Do not delete credentials, revoke the running process's access, probe a provider or claim authentication success as part of this save. Apply the saved change and its auth-evidence invalidation together; a late refresh/test result tied to older settings must not authenticate the new configuration. A no-op preserves known auth state. A later explicit authenticate POST may test the applicable existing stored credential under D25 without forcing a fresh browser login. The live process may still work with its old applied settings; GET's authentication field must not imply that newly saved settings passed D25. The exact supported integration's later no-auth/native/credential verification and refresh remain step-4 work.

```json
{
  "server_id": "server-a",
  "status": "ok",
  "changed": true,
  "restart_required": true,
  "next_steps": [
    {
      "actor": "user",
      "action": "review_server_settings",
      "instructions": "The settings were saved. Choose Restart in Settings to apply them."
    }
  ]
}
```

Always return `server_id`, `status: ok`, `changed`, `restart_required` and `next_steps` on `200`. `changed: false` means saved values were identical; it can coexist with a previously pending restart. Read the complete saved definition through the approved GET, rather than duplicating it in this response or creating a new read route. Return `Cache-Control: no-store`.

`next_steps` uses the common actor/action/instructions shape. `user/review_server_settings` explains unapplied changes, missing bindings or unknown authentication after an actual edit, without asking an agent for values. Explain that the saved configuration still needs authentication verification; the existing authenticate POST can establish it through the supported integration. Saving itself performs no verification. For an active restart, use `agent/check_restart_status` first and include the approved server/attempt-specific GET URL in instructions; if newer changes remain, explain that another restart will be needed afterward. For `SERVER_BUSY` from active login, use `agent/check_server_status` with the existing server GET and one-second polling guidance. When nothing needs attention use `[]`. Returning guidance performs no action and grants no permissions.

A lost response/client/UI disconnection does not undo a committed save. Read GET before retrying an uncertain result. Repeating the same replacement patch does not accumulate entries, but last-writer-wins still applies if other callers changed supplied fields in between. No asynchronous configuration job or new progress route is added.

#### Approved configuration errors

All error replies use `status: failed`, `error: {code, message}` and `next_steps`. Do not return input/config values, raw file/process/provider output or target details on unauthorized requests.

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Body/query/schema validation failed; no changes. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN` | Fetch login absent/invalid or management permission missing. |
| `404` | `SERVER_NOT_FOUND` | Unknown target after authorization. |
| `409` | `SERVER_NOT_INSTALLED` / `SERVER_BUSY` | Installation unfinished or authentication active. |
| `422` | `INVALID_CONFIG` | Invalid directory/binding, protected launch change or unsupported/unsafe arguments. |
| `500` | `CONFIG_WRITE_FAILED` | Safe read/validation/save cannot complete. Re-read before retrying an uncertain result. |

#### Proposed acceptance evidence and dependencies

Once implementation is authorized, verify omitted fields survive; full-list replacement/removal/default reset and no-ops; safe GET-to-PATCH args round trips and protected-prefix/secret/unknown-option rejection without leakage; no package/credential/process/provider/original-file side effects; missing local references and no cross-server resolution; denial before target disclosure; concurrent unrelated/overlapping fields; valid records after failed saves; auth-active rejection; restart-active captured-versus-new settings; authentication evidence invalidation only on actual edits; restored applied settings versus other pending changes; and uncertain response/readback behavior.

Supported launcher/argument schemas, reference metadata, saved/applied comparison and exact file/write/key mechanisms remain implementation dependencies to review in their selected steps. None of the HTTP/storage/auth behavior has been implemented or tested. D30 approves the reviewed observable contract. Exact mechanisms and application implementation remain unselected/paused.

### Proposed complete credentials PATCH contract — awaiting feedback

October 27, 2026. The path/purpose and Settings-only manual-value boundary are accepted. The user has reviewed the base batch/save/removal/result/overlap contract and authorized moving on after adding the D31 verified-`200` gate/shared failure code; the new verification-specific HTTP mapping, response fields and edge-case policies below remain explicitly open/proposed, not blanket approvals. Common config PATCH is approved (D30); native-server settings normalization is deferred. This route stores manual server-local environment values, API keys and custom tokens. Backend OAuth grant/refresh writes remain trusted internal processing, not manual inputs here.

```http
PATCH /api/v1/servers/{server_id}/credentials
```

The example value is only a placeholder. Real values are entered privately in Fetch Settings, never through an agent prompt or MCP tool:

```json
{
  "set": [
    {"name": "stored-entry-7", "value": "<entered privately in Settings>"}
  ],
  "remove": ["obsolete-entry"]
}
```

#### Proposed exact input

- `set` and `remove` are optional arrays; omitted means no operation. `{}` and empty arrays are valid no-ops. Require a JSON object body; reject missing/malformed/non-object bodies, unknown fields/query parameters and wrong types with `400 INVALID_REQUEST`.
- Every set item has exactly `name` and `value`. `name` identifies the same manual server-local entry exposed as GET credential `name` and referenced by config `credential_ref`. Set creates or replaces that entry, without creating/changing environment bindings. `remove` is a list of manual entry names; removing an already absent entry is a no-op.
- Match names exactly, case-sensitively within the selected server; no trimming, path interpretation, global lookup or cross-server resolution. Names must be non-empty/non-whitespace strings without NUL. Invalid name/value content is `422 INVALID_CREDENTIAL_ENTRY`.
- Values are strings, preserved exactly without trimming/shell expansion. Empty string is valid for an intentionally empty environment value; removal is explicit, not `null` or empty string. Reject NUL because these entries support process-environment delivery. Presence is not proof of a valid API key.
- Reject duplicate names in either list or names present in both lists with `400 INVALID_REQUEST`. Only manual entries are editable: backend-owned OAuth grant/access/refresh records and reserved runtime-auth entries are protected; targeting them returns `422 INVALID_CREDENTIAL_ENTRY`.
- Treat all manual values as secret, including ordinary environment values. Request schema does not select file encoding, crypto format or a database.

#### Proposed authority, prerequisites and overlap

Require backend-verified Settings manual-write authority **and** server-management permission. Ordinary agent/setup OAuth credentials get `403`, even by direct HTTP. No secret-valued MCP tool is exposed. Hiding a tool is not the backend authorization mechanism; concrete Settings authority/key handling remain step-4 choices.

Authorize before revealing existence. Unknown target is `404 SERVER_NOT_FOUND`; known unfinished installation is `409 SERVER_NOT_INSTALLED`. Installation is required, not MCP readiness.

Reject manual writes during **either active authentication or active restart** with `409 SERVER_BUSY`. For login, guide the existing server GET; for restart, guide the approved named server/restart GET, polling once per second. This is a newly proposed credentials-route restriction. D30 still permits non-secret config edits during restart; D20 did not select a secret-value snapshot mechanism. Waiting avoids changing manual values underneath a login/test or accepted restart. Check overlap together with the write, not only before acquiring its coordination boundary. Automatic backend refresh coordination remains integration-specific; no existing auth lifecycle is silently rewritten.

#### Proposed encrypted saving and concurrency

- Validate the entire batch before mutation, then save the resulting manual-entry changes as one complete encrypted operation in this server's D28 durable directory, preserving unrelated manual entries and protected OAuth/runtime-auth material.
- Serialize per-server writes with related config/credential/auth-state changes and trusted automatic writes. Concurrent requests preserve unrelated entries; last committed operation wins for the same entry. No revision/history or edit-version feature is introduced.
- Complete successful encrypted save, **private readback and the required successful test using that stored credential** before `200`; do not leave a partially applied batch/corrupt record on failure. No plaintext fallback or plaintext temporary credential file. Exact encryption/key availability/safe-write/locking mechanisms remain to choose; no crash-recovery guarantee is added.
- Compare values privately for no-op detection. An unchanged batch verifies the existing readable state without an unnecessary rewrite; a changed batch requires encrypted save/readback before the test can establish success. An unchanged value cannot bypass the required verification gate merely because it was already stored. Do not echo values, partial/masked versions, lengths or fingerprints in management replies/errors/logs. Response `changed` describes this authorized operation, not a credential-return API.
- If save/readback fails or the key/store is unavailable, return safe `500 CREDENTIALS_WRITE_FAILED`; never claim stored or authenticated success. An uncertain failure may require user retry; a complete record may have committed before readback/response failure. No automatic rollback or blind retry is promised.

#### Proposed removal, application and auth state

Removing a stored value does not change config bindings. Any remaining reference is `missing` through GET; guide the user to supply a replacement in Settings or deliberately change/remove the binding through the approved config PATCH. Never silently substitute an empty value. Removal is local storage deletion, not provider revocation and not erasure of the old value already held by a running process.

The current process retains its applied environment until the selected launch/restart point. Recompute `restart_required` from saved versus applied launch/manual-environment state: changes to referenced values can require restart; creating/removing/changing an unreferenced entry alone does not. Preserve other unapplied differences. A no-op preserves the previous flag. Config-reference changes remain on config PATCH; manual-value saving does not bind or restart automatically.

Any actual manual-entry change first invalidates previous authentication evidence: set GET `authentication.status: unknown` and `error: null` while the new state is unverified. The required test then records the supported integration’s verified outcome: credential-based success is `authenticated` only for the current credential/config state; failed verification reports `failed` with `CREDENTIAL_VERIFICATION_FAILED`. A no-op does not invalidate evidence merely by writing, but its `200` still requires the newly required test. Apply value change and invalidation together, and do not let a test/refresh result based on older values authenticate the new saved state. Exact internal comparison/generation/handoff mechanisms remain implementation work.

D31 supersedes the earlier storage-only `200` proposal. After the encrypted save/readback, this operation must perform the supported authenticated test using the newly stored credential **before** returning `200`. It must not test a callback-held input copy, public health, generic `tools/list`, or an old running process that has not received the new credential. Test success must be associated with the same saved credential/config state; stale results cannot produce success.

A failed/unavailable/timed-out/unproven test returns the shared `CREDENTIAL_VERIFICATION_FAILED`, also used by authenticate and its GET result. No `200` or authenticated success is allowed for an unverified credential. Save/readback failures retain `CREDENTIALS_WRITE_FAILED` and do not proceed to verification. Verification never implies permission to run an arbitrary side-effecting tool, open a new browser login or automatically restart the managed server. The supported endpoint/private handoff, test deadline, per-entry versus resulting-server batch proof, no-op/removal-only/non-auth-environment/native-permission handling and exact failure HTTP mapping must be selected during integration design. No unchecked `200` exemption is silently approved.

If storage committed before verification failed, report that fact without leaking values. Proposed explicit failure metadata is `credentials_stored: true` when known, `false` when known no write committed, or `null` when storage outcome is uncertain. This field is proposed, not yet user-selected. Do not silently assert the old value remains, add automatic rollback, or revoke a provider token. Retain-versus-restore policy for a rejected saved value remains open; a revised full error policy must make the actual persistence outcome clear. No new generic test route is added.

#### Proposed result and guidance

```json
{
  "server_id": "server-a",
  "status": "ok",
  "changed": true,
  "credentials": [
    {"name": "stored-entry-7", "status": "stored"},
    {"name": "obsolete-entry", "status": "missing"}
  ],
  "restart_required": true,
  "next_steps": [
    {
      "actor": "user",
      "action": "review_server_settings",
      "instructions": "The values were stored and the required credential test passed. Restart through Settings if launch changes are still pending."
    }
  ]
}
```

A `200` may be returned only after the required stored-credential verification succeeds. Return the shown base fields; no additional success-code field is selected. `credentials` includes affected names only in set-list then remove-list order; no duplicates. A confirmed removed/absent name is `missing`. Use existing GET for all known assigned metadata, not a new read route. `changed: false` means the resulting entries are identical; earlier pending actions can remain. An empty batch returns an empty affected list. This response has no credential values. Do not use this example to approve an unchecked removal, no-auth or unsupported-verifier success; their exact handling remains open under D31.

Guidance retains actor/action/instructions. `user/review_server_settings` explains missing bindings, pending application or a failed/unavailable test, without soliciting values through an agent. The credential PATCH must perform its own required verification before `200`; it cannot defer the test to a next-step authenticate call while reporting success. Unsupported cases explain the integration limitation, not fictional OAuth or a new test route. Busy cases use `agent/check_server_status` or `agent/check_restart_status` and the existing monitoring URL. Include `next_steps: []` when nothing needs attention. Guidance itself performs no action or permission grant.

Return `Cache-Control: no-store`. Disconnecting does not undo a completed save. GET can confirm names/status/pending changes, **not which of two different secret values is stored**. After an uncertain response, Settings may explicitly retry the same replacement batch; it does not append entries, but later writers can still be overwritten. Do not add a secret readback/hash API or automatically replay an uncertain mutation.

#### Proposed credential errors

All errors contain `status: failed`, safe `error: {code,message}` and `next_steps`. Never echo submitted values/raw request bodies, storage/provider/process diagnostics or unauthorized target details.

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Invalid body/schema/query, duplicate or conflicting names. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN` | Fetch login missing/invalid, management permission or Settings write authority missing. |
| `404` | `SERVER_NOT_FOUND` | Unknown server after authorization. |
| `409` | `SERVER_NOT_INSTALLED` / `SERVER_BUSY` | Installation unfinished or authentication/restart active; no manual mutation. |
| `422` | `INVALID_CREDENTIAL_ENTRY` | Invalid names/values or protected OAuth/runtime-auth entry. |
| `500` | `CREDENTIALS_WRITE_FAILED` | Required store/key/read/save/readback cannot safely complete. |
| Non-`200`, exact mapping pending | `CREDENTIAL_VERIFICATION_FAILED` | Required test failed or could not establish the new stored credential works. Same code in authenticate/GET verification failures; storage outcome must be stated truthfully. |

#### Proposed acceptance evidence and dependencies

Once implementation is authorized, verify Settings-only direct HTTP enforcement/no secret MCP input; set/update/remove/no-op/empty-string/duplicates/conflicts/NUL; protected/cross-server isolation; whole-batch save/readback failures and no plaintext/value leakage; referenced removal versus unchanged config/missing status; unreferenced changes versus other pending restart state; concurrent unrelated/overlapping/automatic writes; active login/restart rejection; private auth-evidence invalidation/stale-result rejection; only the documented authenticated verification request, no unrelated process/login/revocation effects; no `200` or authenticated success from storage alone; same verification-failure code in both routes and async GET, new-versus-old credential proof, saved-but-unverified/error metadata; and uncertain response/retry without secret readback.

Exact entry/storage/key/locking mechanisms, Settings authority, manual-auth integration tests and process credential application remain later decisions, not assumed complete by this API proposal. No app/HTTP/credential tests have run. The user accepts moving on with D31, but the listed verification-specific policies/mechanisms still need review before implementation; do not treat their absence as an unchecked-success exemption.

## Accepted: restart contract

The user approved the displayed request/result structure with "Good" on October 26 and the remaining operational bundle with "Cool all good next" on October 27 after a stopping/restarting clarification (D20). Restart is one of eight accepted setup/server-management method/path pairs. Implementation remains paused.

```http
POST /api/v1/servers/{server_id}/restart
GET /api/v1/servers?id=github&restart_id=restart-42
```

Accepted request: JSON-body `force_restart` is an optional boolean, default `false`. Accepted `202` body: `server_id`, `restart_id`, `status: restarting`, and structured `next_steps` using `actor: agent`, `action: check_restart_status`, and instructions to check the selected attempt until it succeeds or fails.

Accepted nested restart result: `status: restarting` with `phase` in `waiting_for_calls`, `stopping`, `starting`, `checking_readiness`; or `status: succeeded` with `code: SERVICE_STARTED`; or `status: failed` with `error.code`/`error.message`. Settings keeps loading while pending, shows actual success, or stops loading and shows failure.

Existing approved behavior remains: control-triggered POST, immediate `202`, normal draining/temporary rejection of new calls, immediate request-only force, server-scoped secrets, and attempt-specific status query. The restart ID identifies the attempt, not the server or an authentication credential.

The user requires **one complete contract per turn**. [restart-contract.md](restart-contract.md) now records the approved payloads, permissions, settings capture, one-active-attempt rule, 60/5/30-second deadlines, readiness, filtered-GET envelope/validation/errors, polling, retry and interruption behavior. PostgreSQL-backed retention/durable recovery remain deferred under [storage-scope.md](storage-scope.md). Exact IDs, protocol/library versions, cleanup implementation and local operation-state lifetime remain later design work. D23 separately approves the ordinary GET record fields and query/error behavior above. D36 approves the remaining installation-progress read contract. D38 approves the revised listing contract. D50 subsequently approves the presented installed-tool invocation contract and D68 ordinary execution; the current [remaining checklist](open-questions.md#remaining-decisions-checklist) starts with the local OAuth trust choice.

## Approved installed-tool listing contract

October 27, 2026: the user said "sounds good, next" after the whole listing proposal and D37 raw-Tool clarification. D38 accepts the presented listing contract as revised by D37, including path/filter, live complete collection, 10-second per-server deadline, caller visibility, response/availability, safe errors/no-store and no automatic lifecycle/auth/invocation. Approval does not select additional unshown policies or application implementation. This contract lists installed-server tools; Fetch's built-in setup-tool catalog/mappings, HTTP invocation and MCP endpoint/alias/capability mapping remain subsequent step-2 reviews.

```http
GET /api/v1/tools
GET /api/v1/tools?server_id=server-a&server_id=server-b
GET /api/v1/tools?view=2026-07-28
```

### Approved request and permissions

Use Fetch Settings authority or authenticated tool-list permission; exact OAuth scopes remain step 4. Authorize before disclosing source/tool metadata. Listing permission does not grant invocation permission.

No request body; reject a non-empty body. With no filter, select all installed servers visible to this caller in case-sensitive server-ID order. Repeated `server_id` selects opaque, case-sensitive IDs, removes exact duplicates and preserves first-requested order. Empty/whitespace/NUL IDs and unknown query parameters return `400 INVALID_REQUEST`. Unknown IDs and IDs the caller cannot list are indistinguishable in `not_found_ids`, in first-requested order; they expose no other metadata. Missing overall list authority is `403`, not an empty successful list. Apply any caller-specific tool visibility before exposing tool metadata; an empty available list means empty for this caller.

D65 adds optional single view=raw|2025-11-25|2026-07-28; direct HTTP omission means raw. Reject empty/repeated/unsupported view values with400 INVALID_REQUEST. View selection does not change caller authority or the actual negotiated protocol. See [the approved view contract](tool-catalog-status.md).

### Approved live collection and lifecycle

Each GET requests MCP `tools/list` metadata using each selected server's existing usable connection and its applicable upstream credential context. Do not create a new connection/session/worker, start/restart a child, open login, invoke a tool or perform a service-authentication test. A server that advertises no tool-list capability, is disconnected, starting, stopped, restarting, failed or otherwise unusable is unavailable with `SERVER_NOT_READY`. Listing success does not set authentication status or prove provider credentials work.

Follow all upstream pages with a **10-second total deadline per server**, including its pages; this is an approved Fetch default, not an MCP requirement. A missing `nextCursor` ends traversal; an empty-string cursor remains a valid next cursor. Do not automatically retry a failed traversal. Timeout/nontermination or protocol/collection failure makes that server unavailable. D37 rejects duplicate tool names within one server and requires collecting the complete list before publication. Do not add field/schema/content sanitization. Never publish earlier pages as a complete or empty success.

Do not serve cached lists or stale fallback; return `Cache-Control: no-store`. Concurrent GETs may read metadata without mixing caller/credential contexts or exposing partial state. This does not select worker counts or ordinary invocation concurrency. On request cancellation, abandon/cancel only that request's metadata collection, preserving the connection, server and unrelated work. Exact upstream RPC cancellation and execution admission mechanisms remain implementation choices.

Tool pages/servers can change during collection. The response is an observation, not a cross-page or cross-server atomic snapshot. Existing connection changes invalidate that server's in-progress traversal rather than mixing old and replacement connections.

### Approved response

```json
{
  "status": "ok",
  "view": "raw",
  "complete": true,
  "origins": [
    {
      "source": {"kind": "server", "server_id": "server-a"},
      "status": "available",
      "observed_at": "2026-10-27T15:00:00Z",
      "tools": [
        {
          "name": "search",
          "inputSchema": {
            "type": "object",
            "properties": {"query": {"type": "string"}},
            "required": ["query"]
          }
        }
      ],
      "error": null,
      "rejected_tools": []
    }
  ],
  "not_found_ids": [],
  "next_steps": []
}
```

Always include these root/origin fields. D65 adds root view and origin rejected_tools. `available` means a complete caller-visible collection with no applicable rejected definitions; `tools: []` is valid. `observed_at` is the UTC RFC3339 traversal-completion time. Sort tools by exact case-sensitive upstream name. `unavailable` has `tools: []`, `observed_at: null` and safe `error: {code,message}`. No stale tools are included.

D65 adds `partial`: complete collection with known mandatory rejections, actual observed_at, remaining raw tools, error:null and nonempty safe rejected_tools. All-rejected partial differs from a genuine empty available result. Unavailable retains its existing fields and rejected_tools:[]. Rejection details contain exact authorized name, code INVALID_MCP_HEADER_MAPPING and safe Fetch-owned message; no rejected raw values/provider diagnostics.

`complete` is true only if every selected visible server is available and `not_found_ids` is empty; partial or unavailable makes it false. No visible servers/no requested IDs gives `200`, `complete: true` and empty arrays. A request with missing IDs or failed servers still returns `200`/`status: ok` with `complete: false`: aggregate read success differs from each server's outcome. Preserve selected server order independently of response timing.

### Approved identity and raw Tool preservation — D37/D38

The installed-tool identity is **source kind + Fetch server ID + exact upstream tool name**. Equal names across servers remain distinct; only duplicates within the same server's complete list are rejected. Display labels and upstream `serverInfo.name` are not identities. No flat MCP alias encoding or opaque tool-ID scheme is selected.

The user explicitly selects the complete received Tool JSON object in `tools`: preserve every property and nested value, including `icons`, `_meta`, `execution` when present, unknown extension fields and complete schemas. Do not reconstruct tools from a fixed field whitelist, drop unrecognized fields, rename/normalize contents, fill defaults, redact/rewrite descriptions or schemas, or add schema/content validation and `TOOL_METADATA_UNSAFE` filtering. This supersedes the earlier six-field projection and additional metadata-validation proposal. "Raw" means unchanged JSON content; JSON serialization need not preserve whitespace, equivalent escaping or object-key order byte for byte. Preserve nested arrays, JSON types and numeric values without lossy conversion. Fetch's surrounding origin/status/error envelope is separate from the received Tool object.

D37 originally selected only duplicate-name rejection and complete-list handling as Tool-list validation. D64/D65 add only mandatory HTTP header-mapping rejection with explicitly incomplete valid-tool availability for the applicable source/client view; duplicate and collection failures keep existing unavailable semantics. Existing Fetch caller authorization and basic protocol decoding/collection still apply; they are not an additional metadata-cleaning policy. Fetch never inserts its stored credentials into Tool metadata. Preserve metadata as data without fetching icons, resolving external schema references or executing descriptions/extension hints. Preservation does not implement an extension, grant invocation permission or establish provider authentication. No generic metadata confidentiality claim is made.

The current researched MCP2026-07-28 core schema has eight fields: required `name`/`inputSchema` and optional `title`/`description`/`outputSchema`/`annotations`/`icons`/`_meta`. They are not mandatory on every server, an exhaustive cross-version whitelist or a Fetch support-version decision. MCP2025-11-25 also included optional `execution`; preserve it if received. See [the version comparison](research.md#tool-json-preservation-and-version-differences--october-27).

### Approved errors and matching guidance

Request failures use safe `{status: failed, error: {code,message}, next_steps}` with no raw request/RPC/provider bodies:

| HTTP | Code | Meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | Non-empty body, malformed/empty filter or unsupported query. |
| `401` | `UNAUTHENTICATED` | Fetch authentication missing/invalid. |
| `403` | `FORBIDDEN` | Caller lacks overall listing permission. |
| `500` | `TOOL_LIST_UNAVAILABLE` | Fetch cannot perform the aggregate read. |

Per-server unavailable codes: `SERVER_NOT_READY`, `TOOL_LIST_TIMEOUT`, `TOOL_LIST_UNAVAILABLE` (upstream RPC/disconnect/collection failure), `TOOL_METADATA_INVALID` (duplicate tool names within one server). D38 accepts the presented error family after D37 narrows metadata rejection to duplicate names. These remain origin errors in an aggregate `200`, even if every selected origin is unavailable. Never return raw diagnostic bodies, unauthorized metadata or partial pages. Authorized Tool objects themselves are preserved in full under D37.

Always include `next_steps`. Use `[]` when no follow-up is needed. On an unavailable origin, an agent may be guided with `actor: agent`, `action: check_server_status`, the affected `server_id` and safe instructions to inspect existing server GET state before choosing an approved remedy. Missing IDs may use `actor: agent`, `action: list_servers`, instructions to inspect visible server IDs. Do not prescribe login/restart from listing failure alone or automatically perform one. Structured origin states/errors drive the UI; it does not parse guidance. User-triggered refresh/safe GET retry may re-list; no automatic polling/retry is introduced. D32/D33 require matching native app and agent workflow rules.

### Acceptance evidence to collect after implementation authorization

After implementation authorization, check cross-server equal-name identity versus within-server duplicates; empty versus unavailable/all-failed aggregates; exact filter/order/missing/permission behavior; complete multi-page collection including empty cursors/deadline/cancellation/connection replacement without partial publication; caller/upstream-context isolation; complete Tool objects including unknown/nested/version-specific fields without projection or schema/content sanitization; only metadata RPC side effects, no starts/login/invocation/auth-success mutation; and matching UI/guidance behavior. No runtime, HTTP, MCP, provider or credential tests have run.

## Approved installed-tool invocation contract — D50

October 27, 2026: next whole step-2 contract after D38 listing approval. D39 selects arguments-only input compatibility and complete raw terminal result preservation; D43 approves Fetch-owned failure text/native protocol handling and raw-only upstream ToolResults with no added notice. D44 reuses next_steps for Fetch explanations and supersedes response_context. D50 approves the presented remaining HTTP route/envelope/error/lifecycle contract. Exact protocol/host compatibility and unshown execution mechanisms remain later review. [Invocation research](research.md#installed-tool-invocation-boundaries--october-27) informed the plan; D37 covers definitions; D39 subsequently selects arguments/result preservation without originally approving the remaining route or execution policy; D50 subsequently approves the presented route contract, while execution mechanisms stay open. D68 subsequently approves the presented ordinary deadlines/concurrency/shared state/cancellation contract; only its unshown schema/bounds/native-resource admission and implementation mechanisms remain open.

```http
POST /api/v1/servers/{server_id}/tools/{tool_name}/call
```

```json
{
  "query": "example"
}
```

### Approved presented input, authority and forwarding

The user explicitly reconfirmed raw POST arguments and forwarding to the MCP by ID, then asked to continue. The selected body/result were settled in D39; the user subsequently approved the presented URL and remaining outer call contract in D50. Numeric execution and implementation choices remain separate.

**D39 selected input:** the body is exactly the model-supplied tool-argument JSON object. All its members, including any named `name`, `server_id`, `arguments` or `_meta`, are tool data, not Fetch routing/protocol fields. Preserve all keys and nested values without a whitelist, schema reconstruction, coercion, lossy numeric conversion, inserted defaults or Fetch credential injection. The backend passes this same object as upstream `tools/call` `params.arguments`. The ordinary MCP client/transport supplies the method, target name, JSON-RPC request ID/envelope and supported transport context internally; the model does not build that envelope or add Fetch fields.

**D50 selected routing:** select the exact server and tool outside the body using the path above. For example `POST /api/v1/servers/server-a/tools/search/call` with `{"query":"example"}`. Percent-encode each selector independently as one UTF-8 path component, then decode exactly once into the original case-sensitive value. Do not convert `+` to space, case-fold, normalize Unicode or decode a literal percent escape twice: original `/` and literal `%2F` encode as `%2F` and `%252F`, respectively. No new upstream name-character whitelist is added. Encoded separators/dot-segment names must survive the actual client/router chain; if a chosen stack cannot preserve them, resolve that compatibility gap with the user before implementation rather than silently restricting names or changing routes. The Fetch MCP adapter resolves D45 aliases to their exact server/original-name mapping, retains the tool's input schema, and calls this same backend API; HTTP uses the original upstream name, not the MCP alias. Argument field names/schema do not change.

D50 HTTP input checks: require one JSON-object body; `{}` is valid empty arguments, while missing/malformed/non-object JSON or duplicate JSON keys is `400 INVALID_REQUEST`. The server/tool path selectors must be non-empty/NUL-free, case-sensitive strings; no query parameters. Do not treat extra tool-argument keys as unknown Fetch fields. This is one call, not a batch. Any JSON-property names are forwarded as tool data; protocol progress/task/continuation metadata remains outside this arguments body and needs separate capability design.

Fetch validates its HTTP routing/protocol framing and permissions; the upstream server owns tool-specific argument validation. The approved presented contract adds no duplicated argument/output JSON Schema validation or content rewriting. This is separate from the selected raw-argument compatibility and requires review with actual SDK/protocol behavior before implementation. Do not replace legitimate tool-error results or strip their fields.

Require authenticated tool-invocation authority for the selected server/name, including the UI when it offers this operation. Listing or generic management authority alone does not grant arbitrary invocation. Authorize before target disclosure/dispatch; return indistinguishable `404 SERVER_NOT_FOUND` for unknown or target-invisible server IDs after overall authorization. Exact scopes/Settings/client mechanisms remain step 4. No caller-supplied metadata may replace backend identity/permissions/credentials. This route cannot bypass the Settings-only Fetch credential-write boundary or become a built-in management operation by matching a tool name.

Use the target server's existing usable connection and applicable upstream credential context; forward the resolved exact tool name and the unchanged argument object once. No extra `tools/list`, provider-auth test, login, start/restart or new worker is performed by this route. No unconditional `authentication.status == authenticated` gate: local/no-auth tools exist and runtime/listing/auth evidence differ. Supported integration prerequisites and actual upstream access controls still apply. A successful call does not update D25/D31 verified-auth state or substitute for the required stored-credential endpoint test.

### Approved presented lifecycle and result

Wait for one terminal upstream result in this HTTP request. No `202`, persistent call job, call-status route, durable result lookup, idempotency/exactly-once guarantee or automatic retry. D20 admission rejects new calls during restart before dispatch. Ordinary capacity/queue/worker/session-sharing policy remains step 3; this route does not select it. Finite call deadlines/defaults/overrides, cancellation and active-work accounting must be selected there before implementation; the listing deadline is not a call deadline.

D50 outer HTTP response: return `Cache-Control: no-store` and the following JSON fields; no argument echo. D39 preserves `result` itself; D50 approves these presented outer fields:

```json
{
  "server_id": "server-a",
  "name": "search",
  "status": "succeeded",
  "execution": "completed",
  "result": {
    "content": [{"type": "text", "text": "Example result"}],
    "isError": false
  },
  "error": null,
  "next_steps": []
}
```

The example is an illustrative terminal result; actual fields depend on the supported protocol/server and are not reconstructed to match the example. `server_id`/`name` contain authorized valid references when available, otherwise `null` on request failures. Never disclose unauthorized metadata or echo rejected input. `status` is `succeeded` or `failed`.

| `execution` | Meaning |
| --- | --- |
| `not_started` | Fetch knows it did not forward the request. |
| `completed` | Fetch received a terminal upstream ToolResult. This does not promise that every external/background effect has ended. |
| `unknown` | Forwarding occurred or may have occurred, but Fetch has no terminal result establishing the outcome. |

**D39 selected raw result:** preserve the complete received terminal ToolResult as `result`, including all content blocks, `structuredContent` of any JSON type, `isError`, `_meta`, `resultType` and unknown/nested fields. Preserve JSON types/arrays/numeric values without lossy conversion. Do not strip/rewrite/default fields, concatenate content into plain text, fetch referenced resources or execute metadata. Fetch's outer status/error/next_steps fields stay outside the raw result. `result` means the complete upstream MCP ToolResult value, not its JSON-RPC transport wrapper/request ID; protocol transport framing remains handled by the MCP client. Never insert Fetch credential values. Result preservation does not make arbitrary server-controlled content confidential or authoritative.

For a terminal ToolResult, return HTTP `200`, `execution: completed`, `error: null`, raw `result`; `isError: true` yields `status: failed`, otherwise `status: succeeded`. This distinguishes a received tool-level failure from failing to receive a result. The tool's original error content remains intact; `200` alone must not trigger UI success. For request/protocol/transport failure, use `status: failed`, `result: null` and safe `error: {code,message}` with truthful execution classification.

### Fetch explanations through next_steps — D43/D44

D43 requires raw-only upstream MCP ToolResults, including upstream isError results, with no extra Fetch notice or interpretation. D44 removes response_context and places Fetch's explanations/guidance in the existing next_steps actor/action/instructions structure. The HTTP raw result stays untouched. Fetch-owned failures render their instructions into explanatory MCP text; no complete HTTP wrapper/media copies or invented structured output.

Illustrative Fetch-owned HTTP failure fields (outer call schema approved at the presented D50 scope):

```json
{
  "status": "failed",
  "execution": "not_started",
  "result": null,
  "error": {"code": "SERVER_NOT_READY", "message": "The MCP server is not ready."},
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_server_status",
      "instructions": "Fetch did not send the call because the server is not ready. Check its status before trying again."
    }
  ]
}
```

Use [] when there is no explanation/follow-up needed. An upstream result is left to the agent; do not manufacture an extra Fetch interpretation. Keep UI sequences and Fetch-owned MCP guidance aligned with the same backend outcome and permission handoffs. Detailed selected [failure/protocol presentation](mcp-presentation.md#approved-fetch-failure-and-protocol-presentation--d43) and [next_steps use](mcp-presentation.md#existing-next_steps-is-the-explanation-source--d44) remain the source for interface mapping. Standard OAuth/MCP envelopes are not rewritten.

Actual supported-client guidance delivery/schema/media/error compatibility still needs testing. D50 subsequently approves the presented HTTP URL/outer lifecycle contract; actual integration/SDK/host compatibility and application implementation remain unapproved.

### Approved presented errors and uncertainty

| HTTP | Code | Execution meaning |
| --- | --- | --- |
| `400` | `INVALID_REQUEST` | `not_started`; invalid HTTP body/path shape. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN` | `not_started`; no target metadata. |
| `404` | `SERVER_NOT_FOUND` | `not_started`; unknown or invisible server. |
| `409` | `SERVER_NOT_INSTALLED` / `SERVER_BUSY` | `not_started`; unfinished install or D20 restart admission. |
| `503` | `SERVER_NOT_READY` | `not_started` when an unusable connection is rejected before forwarding. |
| `502` | `TOOL_RPC_ERROR` | Upstream JSON-RPC error rather than ToolResult; conservatively `unknown` after forwarding. |
| `502` | `TOOL_CALL_UNAVAILABLE` | Disconnect/unusable response after forwarding; `unknown` unless non-dispatch is proven. |
| `504` | `TOOL_CALL_TIMEOUT` | `unknown` after forwarding; duration/defaults remain step 3. |
| `500` | `TOOL_CALL_FAILED` | Local failure; `not_started` only when proven, otherwise `unknown`. |

Do not forward raw RPC/provider diagnostic bodies or submitted arguments in Fetch errors/logs. Unknown tool names may be reported as upstream RPC errors; no discovery preflight is added merely to manufacture a tool-specific `404`. Safe protocol error detail must not disclose credential material.

A timeout, HTTP disconnection, lost response or cooperative cancellation does not prove a tool stopped or effects were rolled back. Do not automatically replay, including from UI/MCP clients or because an upstream annotation suggests idempotence. Explicit retry after an unknown outcome requires assessing whether the action already happened. Normal restart draining still tracks forwarded work; ending a client wait cannot by itself mark it drained. Forced restart/Quit can interrupt a call without undoing external effects. Exact cancellation/active-work resolution mechanisms remain step 3/5; no new process/session or historical recovery is selected here.

### Approved presented ordinary-call capability boundary and matching guidance

This first route covers terminal ordinary calls; D50 approves this presented route boundary, not final selection of all MVP protocol capabilities. Do not advertise unsupported progress/task/sampling/elicitation/roots/continuation capabilities on its behalf. D37's retained extension fields do not implement them. Known unsupported interaction requirements may reject before forwarding with `409 TOOL_INTERACTION_UNSUPPORTED`, `execution: not_started`. Unexpected non-terminal/interaction/task responses after forwarding return `502 TOOL_INTERACTION_UNSUPPORTED`, `execution: unknown`, `result: null`; never claim terminal completion or continue automatically. Exact supported capabilities/version adapters and any additional interaction/progress APIs remain the subsequent step-2 review, not silently deferred product removal.

Always include `next_steps`. Upstream terminal results normally use `[]`; let the agent interpret the raw upstream success/error without an added Fetch explanation. Fetch-owned failure instructions explain the issue and next action without automatic replay. Pre-dispatch server failures can guide `agent/check_server_status` using the existing server GET. Unknown outcomes can use `user/review_tool_outcome` with safe instructions to assess whether the action happened before an explicit retry; never invent a call-status URL. Settings-only actions remain a human handoff. D50 accepts these presented guidance actions with the contract. Native UI uses structured `status`/`execution`/`result.isError`/errors, while guidance describes the same reviewed actions under D32/D33. Exact UI screens remain step 6.

### Approved presented acceptance evidence and remaining dependencies

After implementation authorization, independently verify unchanged model arguments, including empty/nested/unknown/routing-like property names; target routing entirely outside the body; HTTP shape/target/caller rejection before dispatch; same-named tools on separate servers; exact raw terminal result/extension/multimedia preservation and tool-error versus RPC/local failure; no extra discovery/auth/start/retry; unsupported interactions before/after dispatch; timeout/disconnect/force-restart uncertainty without premature drain accounting; and matching native UI/agent guidance. These are future checks, not tests run.

D68 approves the presented ordinary execution/queue/cancellation/restart behavior. Before implementation, resolve only its unshown schema/bounds/native-resource admission, exact authority/integration prerequisites (step4), SDK/raw serialization/error/codecs (steps2/5), and other specific gaps in the [remaining checklist](open-questions.md#remaining-decisions-checklist). D45/D47/D49 and D61–D67 settle the presented naming/connection/management/transport/native scope. No design approval authorizes building.

### Queued execution and settings amendment — presented behavior approved (D68)

The [execution contract](mcp-execution.md) records D68's selected configurable concurrency, FIFO64/16MiB waiting queue, separate300-second queue wait, deadline precedence/120-second fallback, shown execution config fields, hot atomic application, aggregate GET semantics, cancellation/uncertainty and restart/failure handling. Exact unshown profile/provenance/GET wire names, upper bounds/validation and native resource/prompt/completion admission remain open. D50 raw calls, D20 restart and D63 forced remote recovery stay selected; no new route, durable call job or implementation is authorized.

## Superseded or not selected

- The former `/api/v1/setup/servers` route family moved to `/api/v1/servers` under D21. Do not implement both families or add an alias. This is a path change for existing operations, not additional endpoints.

- `GET /api/v1/discovery/servers` and `GET /api/v1/setup/discovery/servers` were earlier discovery paths. The accepted current path is `/api/v1/setup/discovery`.
- A synchronous installation request was considered; asynchronous acceptance plus progress was selected for installation. For restart, the user initially selected a held-open POST, then explicitly superseded that choice with immediate `202` plus separate completion while keeping the loading icon active throughout.
- A blanket separation restricting all connected agents to imported tool calls was not selected. Agents are intended to help with Fetch setup/management too, while provider secrets stay private.
- D47 selects the presented /mcp connection policy and D49 selects the presented management-tool mapping. D50 selects the presented imported-tool HTTP invocation contract; exact OAuth routes and package update/uninstall scope remain unselected. Configuration/credential-write paths and the Settings authorization boundary are accepted. Restart behavior is approved as recorded above; implementation mechanisms and deferred local job-state choices remain open.

D43 approves [Fetch failure/protocol presentation](mcp-presentation.md#approved-fetch-failure-and-protocol-presentation--d43) with raw-only upstream returns; D44 reuses next_steps. Exact protocol/host compatibility remains before implementation.

D47 approves the presented [MCP connection policy](mcp-connection.md) separately from application JSON conventions; it adds no custom next_steps fields to protocol envelopes. Exact port/SDK/OAuth issuer/TLS and unshown execution mechanisms still need review; D50 approves the HTTP call contract and D68 its presented ordinary execution amendment.

The [initial MCP capability proposal](mcp-capabilities.md) is historical. D58 stopped blanket exclusions; D61–D67 subsequently select the reviewed remote/native surface, cache defaults and explicit live-content/Tasks deferrals. Actual client/SDK fidelity remains unproved; no application implementation follows.

D58 requires [real-use acceptance](real-use-compatibility.md). D61–D67 resolve the presented remote/progress/interaction/resource/prompt scope and explicit Tasks/live-content deferrals; unshown mechanisms and isolated everyday-workflow evidence remain open. Raw D50 behavior persists with its explicit reviewed amendments.

D59 selects shared backend functions and runner dispatch behind the existing /mcp endpoint. Existing product APIs keep their accepted contracts; additional native protocol methods need no distinct feature URL. D61–D67 subsequently select the presented transport/relay/resource/prompt scope and Tasks/live-content deferrals; unshown mechanisms remain open. See [shared handlers](mcp-connection.md#shared-handlers-and-runner--d59).

D60 approves automatic native /mcp tool-call streaming and JSON quick reads with no user-facing toggle. This does not change the existing /api/v1 JSON response contracts or imply additional capability adoption; terminal upstream ToolResults remain raw.

D61 selects [HTTP/SSE upstream support via separate transport functions](remote-upstreams.md), superseding D35 local-only exclusion at the scope level. D63 subsequently approves presented discovery/server/config/reconnect and remote lifecycle amendments. Existing local process-restart guarantees do not apply to a provider process; separate destination/auth/implementation gaps remain explicit.

D62 adds Fetch-owned per-tool transport routing configuration and passes it to the runner in internal execution context. This is outside raw Tool definitions, the HTTP argument-object body and upstream params.arguments; routing-like argument names remain ordinary tool data. No new public request/config fields are selected by this clarification.

## Combined remote-connection amendment — presented behavior approved in D63

D63 approves the presented [remote API amendment](remote-api-amendment.md): existing discovery/import/progress routes, shown remote server/config variant with fixed endpoint identity/editable header references, shared auth/credential rules, existing restart route as Connect/Reconnect, remote phases/results/deadlines and explicit forced-recovery uncertainty/capacity tradeoff. Exact protocol/header/destination/auth and unshown implementation mechanisms remain separately tracked prerequisites.

D63 subsequently approves the presented combined remote API amendment after the complete proposal and requested TLDR. The current accepted remote behavior/schema/defaults are in [remote-api-amendment.md](remote-api-amendment.md); separately identified protocol/auth/destination/storage and unshown implementation mechanisms remain open. This supersedes the earlier proposal checkpoint above without approving application implementation.

## Required protocol adaptations — presented behavior approved through D65

D64 selects [explicit partial-valid-tool availability and the native marker exception](mcp-protocol-adapters.md). Preserve raw data before SDK filtering; after known protocol rejection return valid tools unchanged/usable with explicit incomplete status and safe rejected identity/reason. Do not fail the whole origin/native catalog solely for one rejected tool. Canonical HTTP result stays unchanged; the modern native copy may add only the missing required complete marker to a proven legacy terminal result. D65 subsequently approves the presented partial-list/native metadata and upstream/cache defaults; actual SDK/client evidence and unshown error/serialization mechanisms remain open.

D64 approves the native-only missing legacy resultType marker exception and changes protocol-rejected catalogs to explicit partial availability: valid Tool objects remain unchanged/usable, aggregate complete:false and safe rejected identity/reason outside tools. Do not fail an otherwise usable source merely for one rejected definition. D65 subsequently approves the presented per-origin status/completeness/rejection fields and native metadata mapping; duplicate/collection failures retain D37/D38 rules. Runtime fidelity remains unproved.

## Incomplete-catalog schema and views — approved presented behavior in D65

D65 approves the presented [catalog-status contract](tool-catalog-status.md): existing GET /api/v1/tools view input/echo, partial/rejected_tools, native Fetch metadata, matching listing-tool actual-client default, initial upstream targets and modern private/zero-TTL catalog defaults. Source-versus-export rejection, duplicate/failed traversal, permissions and raw valid-data rules remain explicit; actual SDK/native/host fidelity is not verified.

## Native progress and interaction relay — presented behavior approved in D66

D66 approves the presented [native relay behavior](mcp-interaction-relay.md) on existing /mcp adapters/shared backend invocation context: progress, workspace/model/form requests and modern continuation. Direct REST remains terminal-only under D50 with no new interaction/job/status/cancel route or argument fields. Raw terminal data and D64 marker behavior stay selected; actual SDK/control/resource/client mechanisms remain open.

D66 approves the presented native progress/client-interaction relay and clarifies [endpoint/runner/handler integration](mcp-interaction-relay.md#how-the-planned-endpoint-runner-and-functions-fit-together). Existing shared backend operations/authority and transport functions are reused; ordinary REST remains terminal-only and imported tool arguments/terminal results retain D43/D64. No new REST interaction/job/UI/model route or implementation is selected.

## Native resources/prompts and remaining scope — presented behavior approved (D67)

The complete [resources/prompts proposal](mcp-resources-prompts.md) presents native resource listing/templates/reading, prompt listing/getting and supported completion on /mcp, with qualified routing selectors and preserved returned payloads/raw ToolResult links. No new REST/resource UI/model/memory operation is selected. D67 approves the shown10-second listing/server-default read budgets, modern outer framing/cache hints, catalog changes and live-content/Tasks deferrals. Exact unshown diagnostic wire fields, URI codec and runtime evidence remain open.

## Local OAuth listener/trust — presented direction approved (D70)

D70 selects the presented [local HTTPS/certificate direction](local-oauth-trust.md) for the shared loopback listener, unique per-install certificate/key, explicit human trust and normal browser/client verification. No localhost-HTTP exception, wider-CA fallback or actual runtime/trust-store action is approved. Exact key/certificate lifecycle and client mechanics/evidence remain open.

D69 confirms no Fetch account/signup/password: apps obtain their own Fetch token through local browser connection. Exact approval/authority and token wire contracts are now presented together in [the next proposed contract](client-authorization.md), not selected by D69/D70. This adds no account API.

## Account-free app authorization and token endpoints — presented behavior approved (D71)

[client-authorization.md](client-authorization.md) presents the seven method/path operations, standard inputs/results/errors, local browser plus proposed Mac confirmation, explicit scopes, renewal/reuse/revocation/restart and bounded admission together. OAuth keeps native formats without application next_steps. It adds no provider callback, Fetch account or Settings secret-write power. Exact human connection administration API/UI and key/storage implementation remain later work; D71 accepts the presented clauses; unshown wire/OS/storage mechanisms remain open.

D71 approval scope: use the canonical decision and contract status rather than older proposal labels. No new account, Settings credential authority, source/dependency choice or runtime/client proof follows.

## Settings authority and secret storage — proposed after D71

[secret-storage-settings.md](secret-storage-settings.md) proposes a concrete amendment to the existing credential PATCH: temporary backend-minted native-window capability sent over existing local HTTPS in X-Fetch-Settings-Capability; authority/target checks before prompts; fresh request-bound Mac confirmation for manual set/remove saves; per-server login-Keychain encryption keys, authenticated encrypted files and safe failure/recovery. It adds no public method/path or direct UI persistence path. Native-only manual entry, the private authority header, per-save confirmation, exact store/file/algorithm and new safe error mappings are all proposed, not selected. Existing provider test/no-op/batch verification policies remain4d work.

D72 selects only the shown native Settings/Keychain direction (encrypted per-server files, Keychain keys, native manual changes with Mac confirmation and no per-call/renewal prompts). The exact private authority header, Keychain/cipher/file/timer/error/recovery candidates in secret-storage-settings.md remain unshown and unselected. Continue the concrete D31 verifier/failure policy review without reapproving the broad Settings/storage boundary.

## Historical stored-credential verification amendment — unaccepted, withdrawn from active review by D73

[credential-verification.md](credential-verification.md) presents the remaining policy for existing authenticate POST, credentials PATCH and server GET: reviewed private verifier binding, preflight unknown/unbound SET restriction,30-second total readback/handoff/probe budget, service2xx plus documented success,422/502/504 shared verification failure versus500 storage failure, committed-value retention and safe stored/changed/verification metadata. GET stays read-only. The explicit proposed D31 exception permits200storage-only/not_applicable for pure removal, known non-auth-only changes/no-ops and empty batches; auth SETs including unchanged values retain the full gate. None of these new clauses was approved; D73 withdraws this draft from active review. Preserve existing D25/D31 and resolve required concrete integration/error behavior in the single implementation brief. Custom GitHub CLI scope remains the one separate product choice.


## October 28 implementation note — Railway auth and diagnostics

The existing authenticate POST now recognizes official Railway package/remote
identities and starts Fetch-owned public-client DCR/PKCE sign-in. D80 approves
the auxiliary one-use HTTP127.0.0.1 native callback, separate from product HTTPS
APIs. Existing response statuses, permissions, empty input, raw-data policy,
encrypted readback and service-authenticated verification gate are unchanged.
Source compiles and is scoped-source-reviewed; no live authentication claim.
First-party request logging excludes body/header/query/URL values. Broader
selected incomplete workflows remain in ../testing/MVP_GAPS.md.
