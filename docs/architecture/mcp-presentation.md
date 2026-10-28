# Fetch MCP response presentation

October 27, 2026. **D43 selects raw upstream results and explanatory Fetch-owned failures; D44 reuses next_steps.** A terminal upstream ToolResult passes through unchanged, with no Fetch notice. Fetch operational failures without an upstream result produce Fetch-owned explanatory text. Exact protocol codes/host compatibility and the remaining MCP surface still need review. No implementation is authorized.

## Current decisions

- D37/D38 preserve complete upstream Tool definitions; D39 preserves model arguments and the upstream ToolResult inside HTTP result.
- D43 refines D42: keep native upstream results completely raw, including errors, and let the agent interpret them. Do not prepend a Fetch notice or append guidance to upstream content.
- D43 approves the reviewed Fetch-only failure/protocol split. Fetch-owned operational failures get explanatory text; no fake upstream payload or success-shaped structuredContent.
- D44 places Fetch explanations/guidance in the existing HTTP next_steps actor/action/instructions structure. Remove the separate response_context field; the MCP adapter renders Fetch-owned guidance as model-visible error text.
- Backend APIs own calls, authority and state. The MCP adapter presents the same outcome rather than invoking providers independently.

See [call contract](api-contract.md#approved-installed-tool-invocation-contract--d50), [D43](decisions.md#d43--approve-fetch-owned-failure-text-and-raw-upstream-results), [D44](decisions.md#d44--reuse-next_steps-for-fetch-explanations) and [error research](research.md#fetch-only-mcp-errors-and-output-schema-validation--october-27).

## Selected terminal upstream-result mapping — D43

Let R be the terminal upstream ToolResult held in the HTTP API's result. Return R as the outgoing MCP tool-result data unchanged:

| Property | Selected treatment |
| --- | --- |
| content | Original entries only, preserving every occurrence/value/order. No Fetch notice, extra classification fields, copies, text conversion or deduplication. |
| structuredContent | Original value/presence; never replace it with the HTTP wrapper. |
| isError | Original value/presence, including upstream tool failures. Let the agent handle them. |
| Other fields | Preserve all original unknown/nested values and metadata as data. Retention does not implement extensions or override Fetch authority. |
| Tool input/output schemas | Original values/presence, with no derived wrapper output schema. |

JSON value fidelity remains required; original whitespace/key order need not be retained. The HTTP raw result remains unchanged too. Standard per-hop protocol request/response framing is handled by the adapter; exact supported-version compatibility still requires review, not silent field loss.

D43 supersedes D42's one-added-notice rule for upstream results. D44 does not add next_steps or its rendering to R. This applies to upstream terminal errors as well as success. Do not replace an upstream error with a generic Fetch interpretation. An upstream response may itself contain both native blocks and text/structured copies; preserve what it returned without generating extra copies.

## Approved Fetch failure and protocol presentation — D43

The user approved this complete failure split with the raw-upstream correction above. It creates no HTTP route, call-status job or new error-state store.

| Situation | Selected presentation |
| --- | --- |
| Terminal upstream ToolResult, including isError true | Raw R under D43; no extra Fetch text. |
| Fetch rejects before forwarding: stopped/unready/not-installed server or restart admission busy | Fetch-owned isError true with explanatory text identifying Fetch, safe code/reason, execution not_started and applicable guidance. No upstream blocks or structuredContent. |
| Timeout, connection loss or uncertain delivery after forwarding may have occurred | Fetch-owned isError true with explanatory text identifying execution unknown. Explain the action may have happened; no automatic replay. No fake upstream response or structuredContent. |
| Authentication/authorization, malformed MCP requests, unknown exposed aliases or upstream JSON-RPC errors | Native authentication/protocol handling with safe provenance. Exact protocol codes/revision adapters remain to review. |

For Fetch-owned operational failures use the backend's safe facts and next_steps instructions; the selected text-only form can use one block without an additional boilerplate notice. Let the supported protocol adapter supply required framing/resultType. Do not invent execution/error/next_steps MCP root fields. No raw provider diagnostics, submitted arguments, credentials or unauthorized target metadata is added to text/protocol errors.

The HTTP wrapper retains its truthful status/execution, safe error and next_steps, with result null when no upstream terminal result was received. Do not store a generated Fetch-owned MCP error as that raw upstream result. Use not_started only when non-dispatch is known; otherwise unknown. A known terminal result wins over a guessed timeout/failure.

### Existing next_steps is the explanation source — D44

Use the existing actor/action/instructions structure for Fetch's explanations and follow-up guidance. The instructions can explain the issue and what the recipient should do; no separate response_context field or duplicated wrapper-explanation payload is introduced. Retain [] when no follow-up/explanation is needed, especially an ordinary upstream result that the agent handles itself.

For example, a Fetch-owned pre-dispatch failure can guide agent/check_server_status with instructions that Fetch did not send the call because the server is not ready. An uncertain outcome can explain that the call may have run and requires assessment before explicit retry. Keep permission-based user handoffs; guidance grants no authority or automatic action.

The MCP adapter renders the relevant Fetch-owned instructions with safe source/code information into error text. It does not copy the entire HTTP envelope or guidance into an upstream R. D22 application JSON retains next_steps; OAuth/MCP protocol envelopes retain their standard formats.

### Native authentication/protocol boundary

Distinguish a peer's JSON-RPC error from a local Fetch timeout/disconnection. Preserve backend authority before target disclosure/forwarding and the Settings-only credential-write boundary. Use the native mechanisms/codes appropriate to the supported version rather than allocating reserved codes or allowing generic exception handlers to change protocol errors into a different class.

Exact numeric mappings/framing and supported-host model/UI delivery still need review. Native protocol error visibility is host-dependent; do not promise universal model delivery. D43 approves the presented policy, not an SDK or unshown protocol codes.

### Schema and lifecycle limits

Fetch-owned isError results omit structuredContent. Official examples and inspected maintained SDKs support text-only errors and skip success-output validation for received isError results; actual supported-host tests remain necessary. Preserve the upstream schema and never add fake structured output or revive a Fetch wrapper schema.

No call-deadline value, ordinary queue/capacity/worker/session policy or cancellation guarantee is selected here. Timeout/disconnection does not prove upstream effects stopped or rolled back. Ending an HTTP wait does not establish D20 work has drained; step3 must select cancellation/tracking. No automatic retry, login, restart, force escalation or duplicate invocation is introduced.

## Superseded approaches

- Full HTTP envelope in MCP structuredContent plus serialized fallback and repeated native blocks: rejected in D42.
- Derived Fetch wrapper output schema or automatic upstream schema transplantation: rejected in D42.
- One Fetch notice prepended to every upstream result: superseded by D43 raw-only upstream presentation.
- Separate HTTP response_context: superseded by D44 reuse of next_steps.

Immutable pass history and earlier research retain the prior proposals; they are not current requirements.

## Acceptance evidence after implementation authorization

- Compare upstream R with outgoing native tool-result data for success and tool-level error: all blocks, order, structured data, flags, unknown fields and schemas unchanged, with no added Fetch notice.
- Verify Fetch-owned failures contain meaningful safe text derived from next_steps, distinguish not_started/unknown and leave HTTP result null without fake structured payload.
- Verify HTTP next_steps and native UI/Fetch-owned MCP guidance describe the same backend facts/actions and respect caller authority.
- Check native auth/malformed/unknown-alias/upstream-RPC handling remains native through the selected SDK and reaches the appropriate UI/model path without leakage.
- Check real host/schema/media compatibility, timeout/drop/races and restart/cancellation accounting without false completion, stopped-effect claims or automatic replay.

No HTTP/MCP/SDK validator, host/model/media delivery, runtime test or dependency installation has been performed. Independent documentation acceptance is not runtime compatibility evidence or implementation authorization.

## Proposed modern framing exception after D63

[Required protocol adaptations](mcp-protocol-adapters.md) now requests explicit input on adding only a missing resultType:"complete" in the native2026 copy of a proven legacy terminal ToolResult. Canonical HTTP R stays unchanged, all existing fields are retained, and invalid/unknown/nonterminal markers cannot be overwritten or guessed complete. D43 raw-only mapping remains selected until this narrow exception is answered; no new Fetch notice, result copy or outputSchema rewrite is proposed.

D64 now approves the presented native-only missing resultType:"complete" addition for proven legacy terminal results. Canonical HTTP R and all existing native fields/content stay unchanged; no conflicting marker overwrite, nonterminal guess, Fetch text or duplicate payload. Partial tool catalogs are separately selected with explicit completeness/rejection disclosure; native metadata visibility remains to verify.

The [native resource/prompt proposal](mcp-resources-prompts.md) preserves D43 raw ToolResult resource links/embedded URIs and returned resource content/subresource URIs. Only listing/request routing selectors receive proposed qualified identities; unknown/ambiguous raw links produce safe selection errors. No new ToolResult payload rewrite is approved.
