# Real-use MCP compatibility audit

October 27, 2026. D58 makes everyday Claude Code/OpenCode use the acceptance bar. **Inventory and source findings, not a working integration or new feature approval.** Existing capability-exclusion proposal is on hold. No app implementation or personal-client connection test is authorized.

## What was inspected

Root read tool declarations already exposed to this conversation and extracted only safe metadata from ~/.claude.json and the configured Codex TOML. No command/env/header values, URL strings, credentials, source excerpts or private content were output. URL entries were classified only as loopback/nonloopback. Config files were not modified and no MCP operation/test was run against these servers.

| Global Claude Code entry | Declared source |
| --- | --- |
| posthog | Nonloopback HTTP |
| apple-calendar | Local npx/stdio |
| apple-notes | Local npx/stdio |
| google-nyu | Local npx/stdio |
| google-personal1 | Local npx/stdio |
| google-personal2 | Local npx/stdio |
| google-work | Local npx/stdio |
| linear | Nonloopback SSE |
| calendly | Nonloopback HTTP |
| ghl | Local npx/stdio |
| slack | Local npx/stdio |
| imessage | Local npx/stdio |
| apple-reminders | Local npx/stdio |
| tavily | Nonloopback HTTP |
| photopea | Local npx/stdio |
| premiere | Local node/stdio |
| pinecone | Local npx/stdio |
| rea | Custom local command |
| ccmux | Custom local command |

There are19 global Claude entries:15 local-command and4 nonloopback HTTP/SSE declarations. This is a declared inventory, not proof that every server is active or compatible. Project/profile layers were not reconstructed.

Codex TOML has18 entries, one disabled. Eight HTTP entries are loopback routes for Apple Calendar/Notes/Reminders, four Google accounts and iMessage; they must not be misclassified as remote SaaS connections or modified during design. Nonloopback entries include Tavily, Vercel and a Supabase connection. Local entries additionally include Blender, GitHub, v0, another Supabase integration, node_repl and ccmux. Existing personal gateway/configuration remains preserved.

The exposed tool catalog also includes native image results (Blender scene views and iMessage attachments), and host/plugin tools for other connected services. Hosted/native app tool declarations are not automatically standalone installable MCP definitions; portability requires actual source/registration information. No inference that every exposed host tool can be imported from the global configs is made.

Claude's executable symlink resolved to a version filename2.1.302 without running it. OpenCode's two standard global JSON/JSONC files and executable on this environment's PATH were not present; OPENCODE_CONFIG/CONFIG_DIR/CONFIG_CONTENT overrides were unset. This does not establish that OpenCode is absent from other machines, workspaces or configuration layers. The user is not required to relist already available server names.

## Immediate scope implications

- Raw terminal image/audio/resource/structured/unknown data preservation remains selected and essential for actual exposed tools; no content cleaning/field whitelist is introduced.
- The four nonloopback Claude entries reveal a real coverage gap in D35's local-child-only source policy. D61 subsequently selects HTTP/SSE transport adapter support; exact API/lifecycle/version amendments remain open. This is separate from the localhost-only listener used by Fetch clients.
- Public client documentation/source establishes resources/prompts/roots and progress behavior that could matter to real workflows. It does not prove any particular configured server uses every capability.
- Build an isolated acceptance set from this inventory and exact client releases. Only after authorized verification/rollout should the user connect their daily clients; no source configs, personal logins, apps or existing services are changed by this audit.

## Practical client evidence

| Feature | Current Claude Code official documentation | OpenCode V1 v1.19.4 inspected source |
| --- | --- | --- |
| Tools | Documented | Implemented |
| Resources | References and list/read | Lists resources/templates and reads resources |
| Prompts | Prompt commands | Lists and retrieves prompts |
| Progress | Task display/idle handling | Requests progress and resets its SDK waiting timer on progress |
| Roots | Launch/additional directories and changes | Advertises roots, returns current directory |
| Elicitation | Form/URL support with version/connection conditions | Not advertised in inspected V1 client |
| Sampling | Not established by reviewed main documentation | Not advertised in inspected V1 client |
| Native MCP Tasks | Public beta CLI2.1.304+ with conditions | Not advertised in inspected V1 client |

These are public docs/source findings, not tests of installed clients. Claude's observed2.1.302 launcher does not establish support for its newer task beta. Automatic client-side backgrounding of ordinary calls is separate from native MCP Tasks. OpenCode V2 development behavior must not be generalized to this pinned V1 release.

Claude sources: [resources/prompts](https://code.claude.com/docs/en/mcp#use-mcp-resources), [elicitation](https://code.claude.com/docs/en/mcp#respond-to-mcp-elicitation-requests), [roots](https://code.claude.com/docs/en/mcp#option-3-add-a-local-stdio-server), [timeouts/progress](https://code.claude.com/docs/en/mcp#time-limits-on-mcp-tool-calls), [conditional Tasks](https://code.claude.com/docs/en/mcp#mcp-tasks-that-run-on-the-server).

OpenCode sources: [v1.19.4 release](https://github.com/anomalyco/opencode/releases/tag/v1.19.4), [capabilities/roots](https://github.com/anomalyco/opencode/blob/v1.19.4/packages/opencode/src/mcp/index.ts#L39), [prompt/resource methods](https://github.com/anomalyco/opencode/blob/v1.19.4/packages/opencode/src/mcp/index.ts#L768), [catalog/progress](https://github.com/anomalyco/opencode/blob/v1.19.4/packages/opencode/src/mcp/catalog.ts#L54). This source release is MIT, released October22 and inspected with SDK1.29.0; no runtime dependency is selected.

## What each additional protocol feature requires from Fetch

| Feature | Work beyond returning a finished ToolResult |
| --- | --- |
| Resource operations | Backend list/read/template forwarding, pagination/permissions and exact origin/URI routing. Define duplicate-URI handling without silently rewriting raw resource links or choosing the first server. Optional subscriptions add event routing. |
| Prompt operations | Backend list/get forwarding, origin-aware names, arguments and returned messages. The calling client selects/uses the template; Fetch need not run a model. |
| Progress | Correlate upstream progress tokens with owning caller/request/generation, relay only its events and clean up/rate-limit them. An HTTP/UI event channel needs an explicit contract if exposed there. Fetch's absolute deadline stays absolute even if OpenCode resets its own SDK wait. |
| Sampling | Route the server's model request to the originating capable client and route its answer/error back. No Fetch-owned model/provider key is required for relay. Do not claim support where the caller lacks it. |
| Roots | Ask only the correct client for its permitted workspace context and forward that answer. A shared legacy server's initialization/global request may have no clear caller; define context ownership rather than broadcast or use the last caller. Roots do not enforce OS isolation. |
| Elicitation | Relay form/URL requests to the originating capable client's UI, preserve accept/decline/cancel and return its response. Native Fetch UI/direct HTTP need an interaction path if supported there. Sensitive credentials cannot be requested in forms; URL-mode/Fetch credential ownership still require their selected policies. |
| Modern continuation | Preserve opaque requestState and inputRequests/inputResponses with origin/authorization/expiry; resume the correct operation with fresh request IDs. Bridge to legacy callbacks where supported. Extend D50's terminal-only HTTP contract explicitly without changing raw argument objects or treating intermediate data as completion. |
| MCP Tasks | Forward status/input/result/cancel lifecycle operations; keep owner/origin task-handle routing, expiry/reconnect authorization and decide queue/deadline/concurrency accounting. Upstream can own execution/results; Fetch need not invent another job engine or copy all results. D50 ordinary-call semantics need an explicit amendment. |
| Catalog changes/cache hints | Implement caller-permitted change-event delivery and revision-specific native list metadata. Already selected notifications remain; proposed private/zero-TTL values do not require a shared cache or result transformation. |

Sampling/roots/elicitation share a two-way relay foundation; they are not three independent model/UI products. Legacy callbacks and modern MRTR differ, so full transparency needs protocol adapters and owner mapping, not only final JSON copying. Intermediate JSON may be handled by the client SDK rather than shown as text to its model.

[Current sampling](https://modelcontextprotocol.io/specification/2026-07-28/client/sampling) and [roots](https://modelcontextprotocol.io/specification/2026-07-28/client/roots) are deprecated, not removed; compatibility may still matter. [Elicitation](https://modelcontextprotocol.io/specification/2026-07-28/client/elicitation) defines client UI/consent and form/URL boundaries. [MRTR](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr), [progress](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/progress), [resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources), [prompts](https://modelcontextprotocol.io/specification/2025-11-25/server/prompts) and [Tasks](https://modelcontextprotocol.io/extensions/tasks/overview) are primary references.

No additional feature, remote transport or route is approved by this workload explanation. No live capability discovery, test call, package install, server/client start or personal configuration mutation occurred.

D59 follow-up: the user accepts shared functions per selected functionality behind /mcp, with the runner dispatching methods and handling intermediate/live messages. This reduces endpoint duplication; it does not remove owner/capability/routing work in the table or approve all feature/remote transport scope.

D61 selects [HTTP/SSE upstream adapter support](remote-upstreams.md) to address the recorded connection-type coverage gap while preserving localhost client access. Exact API/lifecycle/version/header-annotation policies remain open, and no live compatibility is established.

D63 subsequently approves the presented remote API/config/reconnect behavior. Inventory/source findings still do not prove live compatibility; remaining upstream-version/header/destination/provider mechanisms and D46 isolated runtime evidence stay open.
