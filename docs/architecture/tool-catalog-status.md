# Tool catalog status and protocol views — approved presented behavior

October27, 2026. **Presented bundle approved in D65.** Returning valid tools with explicit incomplete status and the native marker exception were already selected in D64. D65 accepts the presented listing representation, protocol-view selection, native metadata/Fetch-tool mapping and initial version/cache defaults. No new route, SDK, runtime compatibility or application implementation is selected.

## Where the results are returned

- Application HTTP GET /api/v1/tools: root view/complete and origins[].status/rejected_tools in ordinary JSON, alongside raw valid definitions in origins[].tools.
- Native JSON-RPC tools/list sent to POST /mcp: definitions in result.tools and Fetch completeness/issue summaries in result._meta["fetch/catalog"].
- Native tools/call for fetch__list_tools: the application listing JSON/next_steps is emitted as text in result.content[0].text under D49/D65.
- Modern native cache hints live alongside native result fields as result.ttlMs and result.cacheScope in the applicable discovery/catalog response. They are not copied into Tool definitions, upstream tool-call results or application listing JSON.

These are catalog/listing responses. Ordinary imported-tool invocation retains its upstream ToolResult and the approved native marker exception; catalog diagnostics are not appended to its result/content or arguments. Native metadata display/model visibility depends on the client; the Fetch listing tool emits its explanation when called, without automatic follow-up.

## Existing listing route and approved view input

Keep GET /api/v1/tools, existing optional repeated server_id filters, caller permissions/order/no-store/error rules and10-second per-server complete collection deadline. Add optional single view query value: raw,2025-11-25 or2026-07-28. Omitted direct HTTP view means raw. Empty/repeated/unknown view values or other unknown query inputs are400 INVALID_REQUEST. The view is a presentation-compatibility choice, not authorization, upstream version selection or an imported tool argument. Inspecting raw/legacy view does not change the caller's actual native protocol or make a tool rejected for that native mode callable.

Raw preserves the currently selected upstream-compatible catalog: any mandatory rejection on a modern upstream HTTP hop still applies. A2025 or2026 view additionally checks representability for that selected client-facing revision/HTTP transport. A legacy/stdio-origin annotation invalid only for modern HTTP is not globally invalid: it may remain present/usable in raw/legacy views while being excluded with a diagnostic in the modern view. Never resurrect a definition rejected by the actual upstream hop merely because the downstream view is legacy.

Obtain a complete caller-visible raw collection before view processing. No extra upstream list request, connection/login/tool invocation, retry or cached fallback is added just to select a view. Detect duplicate names in the complete pre-filter raw collection before mandatory annotation rejection; duplicate/timeout/RPC/traversal failures retain D37/D38 handling. No generic schema/content sanitizer, unknown-field stripping or inferred metadata execution is introduced.

Add root view echo to the application response. Canonical raw Tool object values remain unchanged for every returned definition; MCP aliases remain separate D45 native export mapping.

## Per-origin status and rejection details

Add rejected_tools:[] to every origin. Its existing fields remain, with status extended as follows:

| Status | Meaning and fields |
| --- | --- |
| available | Complete collection, no applicable rejections. Raw tools may be genuinely empty; actual observed_at, error:null, rejected_tools:[]. |
| partial | Complete collection with known mandatory rejections. Remaining raw tools stay usable; actual observed_at, error:null and nonempty rejected_tools. All-rejected is partial with tools:[], not a valid empty available catalog. |
| unavailable | Existing duplicate/collection/connection failure. tools:[], observed_at:null, existing safe error, rejected_tools:[]. Do not publish earlier pages or guess rejected/unseen totals. |

Each rejection is {name,code,message}. Name is the exact authorized upstream tool name; its origin supplies server identity. The initial code is INVALID_MCP_HEADER_MAPPING with a Fetch-owned safe reason. Do not return rejected header values, raw invalid definitions/provider bodies or hidden names/counts. Only known protocol-required header mapping rejection is in scope; adding other rejection criteria is a separate reviewed policy.

Root complete is true only when every selected visible origin is available and not_found_ids is empty; partial or unavailable makes it false. HTTP200/status:ok means the read completed, including partial/unavailable source results. Existing request errors remain400/401/403/500; no new HTTP error status is invented for a successful partial read. Preserve origin ordering and exact-name ordering for returned tools/rejections.

Invented two-tool source with one rejection (the same behavior applies to19 of20):

```json
{
  "status": "ok",
  "view": "2026-07-28",
  "complete": false,
  "origins": [
    {
      "source": {"kind": "server", "server_id": "server-a"},
      "status": "partial",
      "observed_at": "2026-10-27T23:00:00Z",
      "tools": [
        {"name": "search", "inputSchema": {"type": "object", "properties": {"query": {"type": "string"}}}}
      ],
      "error": null,
      "rejected_tools": [
        {"name": "search_workspace", "code": "INVALID_MCP_HEADER_MAPPING", "message": "This tool declares an invalid MCP HTTP header mapping."}
      ]
    }
  ],
  "not_found_ids": [],
  "next_steps": [
    {"actor": "user", "action": "review_server_settings", "server_id": "server-a", "instructions": "The remaining tools are usable. Review the server compatibility issue; Fetch has not rewritten the rejected definition."}
  ]
}
```

## Native MCP tools/list metadata and matching Fetch tool

Native tools/list selects its actual negotiated/per-request client revision as the view. Publish valid aliased Tool definitions under D45 with no other per-Tool edits. Add the following Fetch-owned outer metadata in either selected revision's tools/list result; no copy of tools/results is placed in it:

```json
{
  "_meta": {
    "fetch/catalog": {
      "view": "2026-07-28",
      "complete": false,
      "issues": [
        {
          "server_id": "server-a",
          "status": "partial",
          "error": null,
          "rejected_tools": [
            {"name": "search_workspace", "code": "INVALID_MCP_HEADER_MAPPING", "message": "This tool declares an invalid MCP HTTP header mapping."}
          ]
        }
      ]
    }
  }
}
```

This is the metadata portion only. Issues contains caller-visible partial/unavailable origins in response order, with existing unavailable errors where applicable; available origins are omitted from issues. On a complete catalog use complete:true/issues:[]. Fetch's own management tools remain available; a provider rejection does not fail the whole native list. Rejected tools are not offered as callable for the affected view; stale/known incompatible invocation is rejected before forwarding through existing safe unsupported/not-started handling, without a new argument field or automatic discovery/retry. Compatible raw/legacy views are not globally poisoned.

Native required resultType:"complete" means the protocol response finished; it is independent of Fetch's catalog complete:false. Do not fabricate nextCursor for rejected definitions or insert custom next_steps roots. A real pagination cursor, if an independently selected native pagination design uses one, remains solely for actual pages.

Extend fetch__list_tools input with optional view in the same enum. A supplied view is forwarded to the existing query unchanged. If omitted on this MCP management tool, use its actual caller protocol view rather than the direct HTTP raw default, so its explanations match that caller's native catalog. Callers can explicitly request raw. This context default is a D65-approved D49 mapping amendment, not a silent inserted model argument or authority change.

The tool still returns the complete API JSON/next_steps in its existing single text block. HTTP200 partial reads are successful Fetch tool reads, not isError:true simply because tools were rejected. Its Fetch-owned description must explain: "Lists installed tool definitions and reports unavailable servers or rejected tools. Valid remaining tools stay usable. Omit view to inspect this MCP client's catalog, or choose raw or a supported protocol view. Check complete, origins.status and rejected_tools before assuming every upstream tool is available."

The optional native metadata is not guaranteed visible to a host's model/UI. The listing tool emits the explanation when called; do not claim all hosts automatically expose it or automatically invoke the tool. No original provider description/schema/result is rewritten and no unsolicited follow-up call is introduced. Actual client behavior needs isolated evidence.

## Upstream and modern catalog defaults — presented behavior approved

Use2025-11-25 and2026-07-28 as initial upstream compatibility targets, separately from already selected client-facing D47 targets. Select only mutually supported implemented revisions through version-appropriate read-only setup/discovery. Legacy SSE uses its supported legacy setup; no claim every older revision is covered. Recognized setup compatibility signals may select another supported revision, but authentication/TLS/network errors cannot trigger downgrade. Never use/replay a tool call to discover versions or silently change revision after uncertain execution.

For applicable modern native discovery/catalog envelopes use ttlMs:0 and cacheScope:"private": immediately stale, caller-specific cache hints. Do not insert these into raw Tool objects, terminal results, legacy-only envelopes or canonical HTTP JSON. They do not add a result cache or stale fallback, and client compliance is not assumed. D38 application GET remains a live read even if an upstream SDK has its own cache.

The D64 marker exception and D63 remote lifecycle remain unchanged. Exact SDK/raw-capture/numeric fidelity, endpoint/TLS/OAuth/storage/resource mechanisms and extra protocol capabilities are still implementation/review prerequisites, not approved by this mapping proposal.

## Sources and acceptance limits

[Modern ListToolsResult](https://modelcontextprotocol.io/specification/2026-07-28/schema#listtoolsresult) and [legacy Result schema](https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2025-11-25/schema.ts#L85) permit result _meta, without a standard catalog-complete field. fetch/catalog follows the documented metadata naming syntax and uses a non-reserved Fetch prefix. Standard protocol fields do not make Fetch's completeness key a universally understood native feature.

Reuse the completed official SDK/version/annotation research in [protocol adapters](mcp-protocol-adapters.md); no SDK/dependency is selected. Future D46 isolated evidence must cover full/partial/all-rejected/unavailable/true-empty, duplicates before rejection, failed traversal, source versus export view differences, permitted/hidden identities, summaries without Tool copies, matching API/tool/native defaults, raw unknown/numeric fidelity, valid header transport, no invocation replay, live reads/private cache hints and actual client metadata/model visibility. No app source, live client/provider test, configuration/credential edit, environment or dependency was created by this pass.
