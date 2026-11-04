# Fetch architecture records

Read [current project context](../../PROJECT_CONTEXT.md), [notes](../../NOTES.md),
[decisions](decisions.md) and [current open work](open-questions.md) before changes.

The current scoped change is [server-owned configuration and shared login](native-server-storage.md).
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
