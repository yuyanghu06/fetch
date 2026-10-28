# MCP tool names and collision handling

October 27, 2026. **Naming policy approved in D45.** The user said "yes this works for me" to the complete presented policy. This selects stable prefixes/exact mapping/reserved namespace/exceptional aliases/stability, not an implemented catalog or an unshown encoding algorithm. D43/D44 raw upstream results and Fetch guidance through next_steps stay selected. D47/D49/D61–D68 subsequently select the presented endpoint/transport, management mappings, native scope and ordinary execution policy. Exact host bounds/token encoding, unshown wire fields and SDK/client evidence remain open; use the [remaining checklist](open-questions.md#remaining-decisions-checklist).

## Approved public naming policy

Always give imported tools a stable server prefix. Reviewed shape:

```text
mcp_<server-token>__<tool-part>
```

Illustrative aliases offered by Fetch (not actual assigned server IDs):

| Stable Fetch origin | Upstream exact name | Exposed alias example |
| --- | --- | --- |
| server-a | search | mcp_s1__search |
| server-b | search | mcp_s2__search |

Reserve `fetch__...` for Fetch-owned management tools. Their actual names and API mappings are a subsequent review; namespace ownership does not grant permissions or allow Settings-only secret writes through an MCP tool.

The server token is assigned to a stable Fetch server_id, not derived from a display label, package name, list position or upstream serverInfo.name. Exact token generation/encoding is not selected by these examples. A portable letters/digits/underscore/hyphen alias profile remains an implementation proposal; keep a readable original tool-name portion when compatible with the selected supported-host policy. MCP's current 1–128-character recommendation is not treated as a universal hard limit, and this proposal chooses no silent 64-character default.

Prefix every imported tool, including names currently unique. Adding another server must not rename existing tools. Hosts may add their own connection/display prefixes; actual supported-client visibility and naming rules need tests rather than claims of identical host presentation.

## Authoritative mapping, stability and permissions

Keep an explicit backend-owned alias-to-identity map:

```text
alias -> (stable Fetch server_id, exact case-sensitive upstream tool name)
```

Lookup is exact/case-sensitive. Never route by splitting a display string, normalizing an original name, guessing a server, or selecting the first match. The adapter resolves the identity then uses the same backend call API with unchanged arguments. A stable alias does not guarantee current readiness or grant invocation permission.

Assign names independently of caller filtering, discovery order and display-name changes. Existing identities retain aliases across unchanged refreshes and normal relaunches in the same preserved Fetch installation. Package refresh/reinstall that retains the same server identity and original tool name does not create a new naming identity. A changed exact upstream name is a different tool identity.

Do not assign a retired alias to a different origin. The same identity may reclaim its previous alias if it reappears. These reservations are non-secret routing-safety metadata, not configuration revision history or a rollback feature. No guarantee spans deletion/loss of Fetch's local state, and no database or historical-job retention is introduced. Exact durable file format/location within the selected local-data boundaries and write/coordination mechanisms remain step5 choices before implementation.

Apply visibility before advertising tool metadata and recheck invocation authority after exact resolution. Unknown/unauthorized aliases follow the selected native safe error boundary; no provider call is forwarded. Missing/unusable mapping must not silently retarget an alias. A mapped but currently unavailable server/tool follows the applicable backend/native error policy; no per-invocation discovery probe is introduced by naming.

## Long/unusual names and collisions

For original names incompatible with the chosen portable alias profile or supported-host length limits, allocate a stable generated portable tool-part with an exact reverse map. Keep the complete original name in canonical metadata and the map. Do not silently drop a tool, blindly truncate, lowercase away distinctions or claim a shortened digest can never collide.

Check every assigned alias against imported aliases and the reserved namespace before advertising it. Safe allocation may resolve collisions without changing existing mappings. If a unique safe mapping cannot be established, surface a clear native catalog/call error rather than publishing ambiguity, overwriting another mapping or forwarding to a guessed tool. Exact aggregate listing failure policy and protocol error code remain the MCP interface contract; no second discovery/status route is added.

Exact token/fallback encoding, maximum portable length and supported-host restrictions still require review before implementation. They are explicit dependencies, not decisions hidden in these examples. Changing the supported-host policy must not silently rename or retarget previously assigned aliases.

## Canonical metadata and call data remain intact

GET /api/v1/tools continues returning complete original upstream Tool objects, including their original name and all unknown metadata. For this approved naming decision only, the exported MCP-facing Tool copy changes **name** to its alias. Keep inputSchema, outputSchema, descriptions, titles, annotations, icons, `_meta` and unknown fields intact. Do not add a Fetch input wrapper or rewrite tool-specific parameters/results.

The backend invokes the exact original name from the identity map. Upstream terminal results pass through raw under D43, with no extra Fetch notice. Fetch-owned failures render next_steps guidance as text under D44; there is no response_context field or full-envelope/media duplication.

## Acceptance evidence after implementation authorization

- Two servers expose search without ambiguity; case-distinct and delimiter-containing original names never collapse or cross-route.
- Unicode/long/nonportable names remain available through canonical metadata and acquire safe stable aliases without losing exact original identity.
- Imported and reserved Fetch namespaces cannot collide; assignment checks reject unavailable/ambiguous mappings before disclosure/dispatch.
- Display renames, order changes, caller filtering, normal refresh/relaunch and tool disappearance/reappearance retain the specified stability, including retired-name protection.
- Every advertised name maps to one correct origin; denied/unknown/stale names never silently invoke another target.
- Canonical Tool JSON, non-name fields in the exported copy, argument objects and upstream results remain unchanged.
- Real supported hosts accept the selected exact character/length policy; local persistence/coordination safely maintains the mapping before stability is claimed.

Source findings are in [research](research.md#mcp-tool-name-aliases-and-collision-handling--october-27). No alias has been generated, application source written, mapping file format selected or actual host/client/tool test performed. D45 user approval covers the presented naming policy only; documentation acceptance does not prove runtime behavior or authorize building.
