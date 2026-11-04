[Python package environments](../../milestones/python-server-environments/info.md) tracks the selected uv-owned private environment/Python preparation change. New package installs will stop forcing a single Python patch; existing server records remain unchanged.

[Figma registration findings](client-identity-adoption.md#figma-registration-rejection--november-4-2026) explain the reported 403, approved-client policy and why the public Fetch identity file does not bypass it. Research only; callback rejection remains unproven.

[Public client identity adoption research](client-identity-adoption.md) explains current CIMD support across hosted services, the older registration fallback and what the shared Fetch file does. Research only; no new implementation selected.

Publicidentity/IPcallback scopecomplete: canonicalwwwJSONdirect200, rmcpmetadataURL, temporaryHTTP127callback,3focusedchecks/nativebuild/reviewPASS. RealVercelaccountacceptance untested. See milestones/client-identity/evidence.md.

November 4: six redundant Fetch pre-login/saved-login/callback compatibility gates are removed. Official rmcp owns discovery/callback validation;43 independent checks and native build pass, Evaluator ACCEPT. Real Vercel sign-in remains untested. See the latest decisions and milestones/wrapper-authentication/evidence.md.

# Fetch architecture records

November 4 selected-folder correction: choosing `.codex` must read only its own `config.toml`; parent Claude/OpenCode discovery is superseded. See the current discovery API contract and latest decision. Implementation is tracked in [selected discovery scope](../../milestones/selected-discovery-scope/info.md).

[Approved Server Console/Terminal contract](server-terminal.md) records the system-shell pane, native-only GET/POST resource and persistent sanitized logs. Scoped implementation/checks/build/review are accepted; see milestones/server-console-terminal/evidence.md for actual outcomes and native limits.

Fetch’s default is a thin wrapper around existing code and outputs, unless the user explicitly specifies otherwise. See the November 4 decision in decisions.md and the top of AGENTS.md. Preserve actual upstream behavior/errors; do not invent server-specific workflows.

Read [current project context](../../PROJECT_CONTEXT.md), [notes](../../NOTES.md),
[decisions](decisions.md) and [current open work](open-questions.md) before changes.

The current scoped change is [server-owned configuration and shared login](native-server-storage.md).
November4 [actual Railway/Codex research](railway-codex-startup-research.md) records confined startup/rejection evidence and Codex source reuse limits.
November3 follow-up [login research](mcp-login-discovery-research.md) compares
libraries and coding clients, and corrects the Google startup explanation:
Google can begin its own login on startup. The [implemented start/login follow-up](../../milestones/standard-authenticate/info.md)
now presents its selected link through host consent instead of discarding it.
Local MCPs own native credentials; official rmcp owns standard online OAuth.
Fetch keeps manual-key PATCH, shared Keychain saving, process/connection control,
permissions and unchanged raw MCP forwarding. Existing provider files/keys remain
untouched. Older encrypted-provider requirements are superseded, not erased.

| Area | Current reference |
| --- | --- |
| API route/body/permission changes | [API contracts](api-contract.md), [current wrapper amendment](native-server-storage.md) |
| Login and storage | [authentication](authentication-engine.md), [storage scope](storage-scope.md), [Settings authority](secret-storage-settings.md) |
| Coding-app access to Fetch | [client authorization](client-authorization.md), [local HTTPS](local-oauth-trust.md) |
| Raw MCP behavior | [connection](mcp-connection.md), [execution](mcp-execution.md), [management tools](mcp-management-tools.md), [raw protocol](mcp-protocol-adapters.md) |
| Tools and interaction | [catalog](tool-catalog-status.md), [names](mcp-tool-names.md), [presentation](mcp-presentation.md), [resources/prompts](mcp-resources-prompts.md), [relay](mcp-interaction-relay.md) |
| Actual work and evidence | [milestone](../../milestones/milestone.md), [to-do](../../milestones/to-do.md), [scoped evidence](../../milestones/native-server-storage/evidence.md) |
| Remaining product acceptance | [real-use compatibility](real-use-compatibility.md), [MVP gaps](../testing/MVP_GAPS.md), [protected run](../testing/LOCAL_MVP_RUN.md) |
| Earlier plans/research | [build-readiness](build-readiness.md), [research](research.md), [original audit](mcp-session-audit.md), [historical index](../../milestones/native-server-storage/historical-architecture-index.md) |

HTTP handlers belong in src/api, grouped by URL path; backend responsibilities may
use separate folders. Existing api/runtime.rs still combines route families; no
broad reorganization is claimed. New open-folder handler follows the URL layout.

Update decisions and current records when the user answers. Present new API or
substantial behavior choices completely once; do not reopen settled choices.
Keep historical evidence, mark superseded requirements and distinguish source,
fixture results and actual native/provider/client acceptance. No packaging/release,
personal client edits, credential migration or broad live-account testing follows
from recording an architecture decision.
