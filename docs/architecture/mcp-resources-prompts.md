# Native resources, prompts and remaining feature scope — presented behavior approved

October28, 2026. **Presented behavior approved in D67.** The user accepted the preceding combined presentation with "Cool" then "Next." Native methods/routing/payload integrity/shown budgets/cache framing/catalog changes and explicit live-content/Tasks deferrals are selected at that scope. Exact unshown diagnostic wire fields, numeric errors, codec/bounds and SDK/client evidence remain open. Proposal wording below records the reviewed design and does not expand approval beyond the shown scope. No application implementation is authorized.

## Native methods and shared backend

Use the existing /mcp endpoint, backend operation rules and shared runner/adapters for resources/list, resources/templates/list, resources/read, prompts/list, prompts/get and supported completion/complete. Keep native MCP formats; add no REST resource viewer, Fetch model, recursive endpoint tool or memory-write operation. Exact function/module names remain implementation choices.

Extend D66's owned progress/interaction/continuation handling to resource reads and prompt retrieval where their revisions permit input_required. Bind each flow to its actual method/source/URI or prompt/original parameters. No automatic replay, extra process or new job/status route. Direct REST tool-call behavior stays unchanged.

## Resources and origin routing

Resources are named context/data a client can list/read; templates describe parameterized resource addresses. Preserve original definition/unknown fields except the exported uri/uriTemplate routing selector. Never fetch icons, normalize metadata or interpret URI scheme as Fetch filesystem/network authority.

Propose a Fetch-qualified URI containing a stable source token and original URI. Illustrative candidate:

```text
source:    notes://daily/today
exported:  fetch-resource://s1/notes://daily/today
```

resources/read resolves the authoritative source mapping and strips only the exact reserved prefix. Forward the original URI unchanged, including percent sequences, query and fragment. No generic URL normalization, extra decoding, recursive prefix interpretation or display-label routing. Unknown/hidden/conflicting mappings fail safely before contacting a source. Exact namespace/encoding is a candidate requiring host round-trip evidence, not a proven universal URI algorithm.

Templates need the same lossless routing while retaining RFC6570 expressions. A literal prefix outside the original template is a candidate; base64-encoding the entire template hides variables clients must expand. Prove supported template round trips before advertisement. Embedded-app/special URI schemes are not automatically implemented by basic resource support.

Preserve resources/read payloads, text/blob/unknown fields and contents[].uri. Contents can identify subresources and need not equal the requested qualified URI. D43 ToolResult resource links/embedded URIs also stay unchanged; no raw tool-result rewrite is authorized.

A later unqualified URI routes only when current authorized information establishes one unambiguous source. Unknown dynamic links, overlapping templates, conflicting origins or incomplete evidence produce safe unknown/ambiguity errors with authorized qualified choices where known. Never choose first/last/most-recent server, probe every source, open a local file or fetch an HTTP URL. This is an explicit limitation for unchanged raw links: ambiguous/unknown links need source selection.

## Prompts and argument suggestions

Export stable server-prefixed prompt names with exact original-name mapping, analogous to D45:

```text
source prompt: daily_review
exported:      mcp_s1__daily_review
```

Preserve all other prompt metadata/unknown fields. prompts/get forwards the original name and native string-valued arguments without coercion/default insertion/cross-source lookup. Preserve returned descriptions/messages/media/embedded resources/unknown fields with required native framing. Fetch does not apply the prompt, run a model, change a system prompt or save shared memory automatically.

Relay completion/complete where source/revision/adapter support it. Resolve prompt/template references to their original origin/reference, preserve argument/context values and return upstream suggestions. No Fetch-generated autocomplete, filesystem search, fallback model or request to another source. Do not invent a client capability flag for this standard client-initiated method.

## Permissions, collection, deadlines and errors

Authorize every operation and filter source/item visibility before exposing definitions, content or ambiguity choices. Listing does not grant reading/getting; hidden and unknown targets remain indistinguishable. A qualified URI selects a source, not authority or another caller's credentials.

Propose10 seconds per source for each complete listing traversal, matching tools. Preserve opaque/empty cursors and context; no startup/auth/probe/retry/cache fallback. Conflicting identifiers must not use first-match routing or silent metadata rewriting. Observations are not cross-page/source atomic snapshots.

Known sources without a feature are not failed providers of that feature; do not advertise unsupported methods. Unknown/unready/failed eligible sources are not complete empty catalogs. Keep healthy complete sources usable and disclose incomplete acquisition instead of publishing partial pages as complete.

Propose outer native list metadata _meta["fetch/resources"] and _meta["fetch/prompts"] with complete and issues:[{server_id,code,message}]. Complete catalogs use issues:[]; source acquisition failures set complete:false with safe known issues while returning other complete sources. No duplicated definitions/content or guaranteed model visibility. No extra Fetch listing tool/REST route is added here. Exact native numeric errors, codecs and resource bounds remain review before implementation acceptance.

Resource reads, prompt gets and completion use the configured server default execution duration, initially120 seconds and configurable—not a universal cap. Tool overrides remain tool-specific rather than guessed from URIs/prompts. D66 interaction rounds retain the original operation deadline. No automatic prompt application/model use/linked-resource fetch/settings write/process start or retained-result job. Ordinary queue/concurrency/cancellation/uncertainty refinements remain their explicit execution review; not every native RPC becomes an ordinary tool call.

### Required modern read/list framing — proposed

The current2026 schema requires resultType on native results and ttlMs/cacheScope on resources/list, resources/templates/list, terminal resources/read and prompts/list. prompts/get and completion/complete require terminal framing but not those cache fields.

Propose Fetch-owned native result framing/cache hints for the applicable list and terminal read results: valid resultType:"complete", ttlMs:0 and cacheScope:"private". Missing legacy framing is supplied only when the source revision/semantics prove completion; an existing invalid/conflicting resultType is a compatibility failure, not overwritten. Supported input_required keeps its control-result framing under D66 and is not converted to completion.

These outer standard cache fields represent Fetch's caller-private/immediately-stale policy, including stricter hints than the upstream supplied where needed. Preserve contents[].uri, text/blob data, messages and unknown payload fields; no cache fields are inserted inside resource definitions/content or prompt messages. This explicitly extends D65's catalog/discovery cache choice to modern terminal resources/read, without modifying any raw ToolResult or adding a new HTTP read route. Do not add these fields to legacy-only envelopes or methods that do not require them. Actual SDK/client handling remains unverified.

## Catalog changes and proposed deferrals

Include supported caller-permitted resource/prompt catalog-list-change notifications through the existing native change infrastructure. Relay known source/backend events only; no background scan/poll/automatic reread. Advertise only implemented supported types.

Explicitly propose deferring two capabilities from the first usable release:

- Per-resource live content subscriptions/updates. Basic list/read/templates and catalog changes remain included in this proposal. Do not advertise/acknowledge resources/subscribe or modern resourceSubscriptions filters until ownership/cancellation/lifetime is selected and implemented.
- Native MCP background Tasks. Durable task handles, status/input/result/cancel, owner/source routing, expiry/reconnect and capacity require a separate lifecycle. Do not advertise/opt into Tasks; preserve raw metadata and existing truthful known-unsupported handling. Never invent completed ToolResults from native task handles.

Ordinary tools returning their own job IDs in terminal ToolResults and exposing separate status tools still pass through raw. Long ordinary calls keep configurable deadlines;120 seconds is not a hard cap. Legacy/current native Task differences stay explicit if adopted later.

D67 approves these presented deferrals; they are not evidence the features are unused. D58 isolated actual-workflow acceptance must prove the slice sufficient. An essential deferred feature is a compatibility gap requiring user review before claiming MVP readiness; no silent adoption or service removal. GitHub CLI integration stays a separate choice.

## Research and future evidence

Dedicated read-only research checked [current resources](https://modelcontextprotocol.io/specification/2026-07-28/server/resources), [resource/subresource schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2026-07-28/schema.ts), [current prompts](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/server/prompts.mdx), [RFC6570](https://datatracker.ietf.org/doc/html/rfc6570), [legacy Tasks](https://modelcontextprotocol.io/specification/2025-11-25/basic/utilities/tasks) and [current Tasks](https://modelcontextprotocol.io/extensions/tasks/overview). Current Tasks are server-directed opted extension returns with tasks/get, tasks/update and tasks/cancel; legacy separate result/list semantics are not a universal current contract.

Reused maintained Apache-2.0 comparisons: [agentgateway](https://github.com/agentgateway/agentgateway/blob/3ef10407860d6f5c0b10dcd13779fc274a7f58f1/crates/agentgateway/src/mcp/handler.rs) prefixes identifiers and rewrites read-content URIs with special ui handling; that payload rewrite is not adopted here. [ContextForge prompts](https://github.com/IBM/mcp-context-forge/blob/ff6a566a4b5c098093db90bbb4d5df483e108026/mcpgateway/services/prompt_service.py) and [resources](https://github.com/IBM/mcp-context-forge/blob/ff6a566a4b5c098093db90bbb4d5df483e108026/mcpgateway/services/resource_service.py) retain identity/scoped URI uniqueness; their Python/SQLAlchemy registry dependencies are not selected for this database-free Rust MVP. No code copied or dependency installed.

Future D46 isolated evidence: equal/unknown/overlapping URIs, exact template/percent/query/fragment/subresource round trips, special-scheme host limits, alias/string/raw prompt/completion routing, hidden identities, pagination/errors/deadlines, owned interactions/catalog events, unsupported subscriptions/Tasks and actual daily workflows. No runtime/provider/client tests, environment, credentials, personal config or application source were created.
