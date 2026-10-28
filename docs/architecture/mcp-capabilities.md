# Initial MCP capability scope

D59 subsequently accepts shared functions and runner dispatch behind the existing /mcp endpoint. This resolves internal organization, not the inclusion/exclusion proposals below; those remain on hold under D58.

October 27, 2026. **Initial exclusion proposal on hold under D58.** The user requires actual everyday Claude Code/OpenCode workflows; first assess the real inventory and feature workload in [the compatibility audit](real-use-compatibility.md). No blanket exclusions were approved. D47 connection/catalog channels, D49 management tools and D50 ordinary call rules are selected. The additional initial-release scope/exclusions and modern cache values below are proposals, not previous approvals. D57 defers multiple processes per logical MCP for now; queued calls and call-level concurrency remain selected requirements with some exact parameters still open.

## Current selected surface — October28 update

D61–D66 now select local stdio/remote HTTP/SSE adapters, caller-specific tool catalog views and incomplete diagnostics, native framing/cache defaults, progress and caller-owned roots/sampling/elicitation/continuation relay. Actual owner/capability/SDK support must still be proved; direct REST remains terminal-only. See decisions.md and the individual current contracts.

D67 selects the presented native resource/template/prompt/completion surface and catalog changes, while explicitly deferring live per-resource content subscriptions and native background Tasks subject to D58 actual-workflow acceptance. Exact codecs/SDK/host evidence and unshown wire fields remain open. Earlier blanket exclusions below were never approved; only later explicit decisions establish current limits.

## Proposed included surface

| Feature | Initial scope |
| --- | --- |
| Installed tools | List permitted raw definitions and call individually through D45 aliases and D50 application APIs. |
| Fetch-owned tools | Expose selected setup/management APIs under D48/D49, with descriptive tools and normal caller authority; preserve Settings-only manual credential writes. |
| Terminal results | Preserve complete upstream content/structured data/flags/unknown fields: text, images, audio, resource links and embedded resource blocks are not stripped or duplicated. |
| Tool-catalog change delivery | Keep D47's selected legacy SSE and modern subscriptions/listen channel for supported/permitted tool-catalog notifications. Advertise listChanged only once implemented. |
| Native setup/framing | Keep D47's two revision targets and native authentication/protocol behavior; no new custom next_steps fields in MCP envelopes. |

Apply queue/deadline settings through Fetch rather than altering arguments or impersonating an upstream feature. Tool metadata/source observations do not grant invocation authority. No additional application route, worker, model provider or process instance is introduced by this scope.

## Historical unapproved initial exclusions

| Feature | Meaning and proposed boundary |
| --- | --- |
| Provider-tool progress relay | Do not request/relay upstream incremental progress initially. Progress does not extend the absolute execution deadline. Existing Fetch installation/auth/restart monitoring and structured queue/server state remain unchanged. |
| Separate MCP resources | Do not expose resources/list, resources/read, resource templates or resource subscriptions. Resource blocks/links inside tool results stay raw; Fetch does not automatically follow them. |
| MCP prompts | Do not expose prompts/list/get or prompt-related operations. This does not remove Fetch shared-memory product/API scope. |
| Sampling | Do not handle an upstream request to run the caller's model. Do not create a new LLM-provider integration behind the caller's back. |
| Roots | Do not invent/forward the caller's filesystem-root context. Server launch arguments and ordinary authorized tool inputs keep their selected behavior. |
| Elicitation/continuation | Do not implement server-requested forms/additional user input or modern input_required/MRTR continuation. Fetch's separately selected browser-login/Settings/auth APIs remain, but integrations that require native MCP interaction need explicit later support. |
| MCP background Tasks | Do not expose task lifecycle/handle/poll/result/cancel operations or resource/task notifications. Internal waiting queues and Fetch installation jobs do not implement this MCP extension. |

These original exclusions are on hold pending the D58 inventory/workflow assessment and require user input; they are not already selected because D50 initially covers ordinary terminal calls. Servers/tools that require excluded features will need later support; do not claim universal MCP compatibility.

## Honest discovery, errors and raw metadata

Advertise only capabilities actually selected and implemented on each hop. Do not copy a downstream host's roots/sampling/elicitation support into Fetch's upstream client declaration when Fetch cannot route those interactions. Preserve revision-correct setup, subscriptions and cancelled-request behavior from D47 rather than flattening protocols.

D37/D45 still preserve every original Tool field except the approved MCP alias name. Keep legacy taskSupport, unknown annotations and extensions; do not filter required-task tools or rewrite schemas merely to hide unsupported execution. Retention of a field is not a claim that Fetch implements its feature.

D50 handles known unsupported call requirements before dispatch with409 TOOL_INTERACTION_UNSUPPORTED/not_started. Unexpected non-terminal/interaction/task replies after forwarding use502 TOOL_INTERACTION_UNSUPPORTED/unknown and no fabricated terminal result or automatic continuation. These events do not by themselves release unresolved active capacity under the pending execution design. D43 authentication, malformed/unknown-method/alias and upstream JSON-RPC failures stay native safe errors rather than fabricated ToolResults. Calling an unexposed resource/prompt method receives revision-correct unsupported-method handling.

Preserve raw tool-result resource links/embedded blocks without fetching them. The protocol recommends resources support for embedded resources but does not mandate it; tools-only support is an explicit proxy tradeoff. Actual downstream host handling of media, schemas and protocol versions still needs D46-isolated evidence.

## Catalog channel and proposed modern cache fields

Do not broadly disable subscriptions: that would remove D47's already selected tool-catalog change channel. Send only implemented, requested and caller-permitted catalog notifications, using each revision's mechanism. Do not broadcast unrequested resource/task notification types, add a background upstream scan/poll, invoke tools or test provider credentials to manufacture catalog changes.

For modern2026 complete native tools/list and server/discover envelopes, propose ttlMs:0 and cacheScope:private: clients should fetch fresh metadata and not share caller-specific catalogs across authorization contexts. These are outer native protocol fields, not additions inside preserved Tool objects or legacy envelopes. Exact revision/SDK framing stays implementation review. D38 live application GET remains no-store with no cached/stale fallback; a stored tool-feature profile does not replace the live catalog.

## Compatibility and acceptance

Current HTTP clients may reject/filter definitions with invalid x-mcp-header annotations under protocol requirements; upstream stdio clients may ignore those annotations. Do not silently sanitize D37 definitions or promise every raw tool will render/call in every host. Choose and test exact SDK/host annotation handling before implementation acceptance; this scope selects no new header/credential forwarding authority.

After authorization, within D46 isolation, verify both selected revisions' declarations/setup/catalog channels, caller filtering, raw definitions and terminal media/structured/unknown fields, no automatic resource retrieval, optional-progress omission without deadline reset, selected unsupported interactions/methods without fake completion, modern private/zero-TTL framing, and actual host annotation/schema behavior. Verify Fetch's own install/auth/restart progress still works, no extra LLM/filesystem/user-context interaction occurs, and no task/process/job API appears implicitly.

GitHub CLI adoption/auth integration, exact queue/config defaults, SDK/source/library/resource limits, memory release sequencing and authentication mechanisms remain separate open decisions. Do not treat this capability scope as service-specific support cancellation or implementation authorization. [Research](research.md#initial-mcp-capability-scope--october27), [connection policy](mcp-connection.md), [execution proposal](mcp-execution.md) and [presentation](mcp-presentation.md) retain their approval boundaries.

A new [native progress/interaction relay proposal](mcp-interaction-relay.md) presents affirmative scope for progress, roots/sampling/elicitation and MRTR under actual caller ownership. Its legacy/default/error/continuation limitations are explicit. These new clauses await input; the earlier blanket exclusions remain on hold and Tasks/resources/prompts remain separate choices.

D66 now approves the presented affirmative native progress/roots/sampling/elicitation/MRTR relay scope, subject to real owner/capability support and safe rejection of unowned legacy requests. The prior blanket optional-feature exclusions remain superseded/on hold; Tasks/resources/prompts/GitHub remain separate unselected choices.

The complete [resources/prompts proposal](mcp-resources-prompts.md) now requests input on native basic resource/prompt/completion operations and catalog-change delivery, plus explicit first-release live-content subscription/native Tasks deferrals. No blanket exclusion or feature adoption is inferred; actual everyday workflow evidence remains D58 acceptance.
