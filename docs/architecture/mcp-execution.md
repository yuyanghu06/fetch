# Queued MCP execution and per-tool settings

D73 reaffirms execution/queues as settled. Gather exact remaining native-admission/profile/wire/bounds mechanisms in the single implementation worklist; do not present ordinary execution as a new broad decision. Unshown behavior is not falsely marked approved.

October28, 2026. **Presented ordinary-call contract approved in D68.** The user accepted the preceding complete presentation with "Good." Shown scheduling/defaults/config fields/uncertainty/restart/aggregate-state behavior are selected; exact unshown wire/profile/error/bounds mechanisms remain open. Proposal wording below preserves the reviewed draft, not an instruction to reapprove selected behavior. D51 selects queued execution with concurrency coordination; D52 selects durable per-tool feature/settings records applied on calls; D53 selects a 120-second execution fallback, not a universal cap; D54 confirms every managed server has its own configurable default; D55 selects automatic initialization on installation and a fresh dispatch-relative deadline for each call. D68 subsequently approves the presented remaining defaults and behavior; only unshown field/schema/mechanism refinements remain proposals. D50 raw call contract and D20 restart remain selected. No application implementation is authorized.

## Shared state, queues and concurrency

D57 selects one managed wrapper/upstream connection per installed server for the current design, shared by authorized Fetch clients. The D56 configurable multi-process extension is deferred for now and retained as a future feature. Accounts, browser pages and other server-owned state are shared, not isolated per agent. Separate servers may still touch the same external resources.

Every ordinary call enters that server's scheduler; new arrivals cannot bypass accepted waiting calls. Propose one active call/server by default, configurable higher through max_concurrent_calls. Different servers run independently without extra wrappers/workers. FIFO means order Fetch forwards requests, not guaranteed server execution or completion order when several calls overlap.

Propose at most 64 accepted waiting calls and 16 MiB of their stored UTF-8 JSON argument buffers per server, excluding active buffers, results and runtime overhead. These bound the waiting queue, not total application memory. Reject a full count/byte budget before forwarding with 409 SERVER_BUSY/execution not_started and safe queue-full guidance. Never create another unbounded set of tasks waiting to enter that queue. A call able to take free active capacity in scheduler order need not consume waiting capacity.

Fetch setup/configuration/authentication/restart operations, their state/progress GETs and catalog collection retain their own approved rules and do not wait behind ordinary tool work. This scheduler proposal covers ordinary tool calls. D67 resource reads/prompt retrieval/completion admission is a separate explicit classification decision; selecting their execution duration did not silently place them inside or outside this tool-call queue. Admission still rejects a stopped/unready server or active restart under D50/D20 rather than starting it or holding work for an unknown future process. Before forwarding, atomically check queue cancellation/expiry, current authorization, server generation/readiness and available concurrency. Never hold a scheduler lock for a full upstream RPC.

## Desired multiple processes per MCP — D56

The user still wants a configurable total process-count limit per logical managed MCP, with queuing distributed among its running processes according to available capacity. When those processes are occupied, calls remain queued under the reviewed policy. D56 records the desired feature; D57 subsequently defers it for now, retaining it for future scope rather than implementing it in the current MVP.

Keep process count distinct from concurrent calls within one process. The current one-wrapper design is selected for now, not permanent rejection of multiple processes. Do not reinterpret max_concurrent_calls as process count or defer all D51 queue/concurrency coordination merely because the process feature may ship later.

Before selecting this extension, review its API/config/tool fields, defaults/bounds, process launch/lifecycle, shared versus per-process queues and distribution, related-call affinity (such as keeping browser navigation and later reads on the same process), credential/state sharing, cancellation/restart/accounting and isolated acceptance tests. No scheduling algorithm, autoscaling, new instance spawning on ordinary calls or process-level isolation guarantee is selected here. No application implementation is authorized.

## Separate queue and execution budgets

Propose a 300-second maximum queue wait, starting at admission. Expiry while unforwarded returns 504 TOOL_CALL_TIMEOUT/execution not_started. Queued cancellation removes only its own entry and never calls the tool. Queue count, byte limit and queue-wait defaults are fixed MVP policy in this proposal, not newly writable config fields.

D55 selects applying the effective configured timeout as a fresh Fetch execution deadline at dispatch: deadline = dispatch time + effective timeout duration, separately from queue wait. Proposed snapshot rule: capture each request's queue-wait and execution-duration budgets at admission; live max_concurrent_calls controls each subsequent dispatch, including previously queued calls; the execution timer starts only at dispatch. Settings/profile changes do not retroactively alter already accepted budgets. Progress does not reset the absolute execution deadline. Propose effective execution timeout precedence:

1. Explicit Fetch per-tool wait override for the exact server/original-tool identity.
2. A current, explicitly supported/documented whole-call server limit plus a proposed 5-second allowance for receiving its result.
3. That server's independently configurable fallback under D54, initially 120 seconds under D53.

No universal timeout can be read from MCP Tool metadata. Do not guess from a timeout argument, descriptions or arbitrary _meta, and do not modify those values. Manual overrides/fallback are Fetch wait durations; the observed server limit is not rewritten or injected into arguments. Unknown/stale observations use the fallback. No null/unlimited execution mode is selected. All values are positive finite integer milliseconds; final representable resource upper bounds remain explicit implementation review before build.

A still-waiting HTTP caller at execution expiry gets 504 TOOL_CALL_TIMEOUT, unknown after possible forwarding; a still-waiting MCP caller gets the selected Fetch-owned failure through native framing. A downstream client/SDK may stop waiting sooner; Fetch cannot force its deadline to extend. Budget propagation and actual client behavior require isolated compatibility tests rather than silently inheriting an SDK default.

## Durable installation profile and actual discovery

D55 selects initializing the Fetch default-timeout setting automatically during installation: a supported explicit server-default source if available under the reviewed source policy, otherwise120 seconds; preserve existing user settings on successful-install reuse. Create execution defaults and a tool-feature profile in the D28 durable server config outside replaceable packages. These examples are proposed data shapes, not selected filenames/encoding or a real installed server:

```json
{
  "execution": {
    "max_concurrent_calls": 1,
    "default_timeout_ms": 120000,
    "tool_timeouts_ms": {}
  },
  "tool_features": {
    "status": "unknown",
    "tools": []
  }
}
```

Installation initializes unknown features without starting/authenticating a server. Populate actual tools only through the first successful authorized startup/readiness collection of the complete tools/list; no new GET writes or install-time probing. Known approved configuration information may seed settings; do not label a host/client timeout as the server's intrinsic limit.

Proposed per-tool feature records use the exact original name, an observed whole-call timeout_ms or null, and provenance identifying an explicitly supported documented source or unknown. Unknown does not mean unlimited or known 120; the effective fallback is a separate Fetch setting. A successful empty discovery is distinct from unknown. Mark prior observations stale when they do not match the running installation/connection/config context; do not apply a stale reported limit to a replacement. Exact format/version/provenance mechanisms remain implementation review.

Preserve configured overrides independently from observed metadata so refresh cannot overwrite user settings. Do not publish partial tool observations as a complete profile. If no usable fresh observation exists, use configured override/fallback rather than claiming discovery success. Metadata persistence failures must not falsely claim a new observation was saved.

Stored features are execution settings/observations, not a replacement for D38's live raw catalog. Keep original Tool definitions, unknown fields and schemas unchanged; do not serve this profile as a stale tools/list fallback or advertise a tool merely because a policy entry exists. New feature types require explicit supported semantics. Provider credentials stay in encrypted Settings-only storage, never this profile.

## Apply settings inside Fetch, not tool arguments

The /mcp handler resolves an alias to the original server/tool identity and uses the saved Fetch options to control scheduling and its fresh per-call execution deadline under D55. The stored value is a duration, not an installation-time absolute timestamp. The model-supplied JSON object is forwarded unchanged. Do not insert timeout/default/feature arguments, alter inputSchema/outputSchema, fabricate protocol metadata or add Fetch text to upstream results. A provider-specific protocol feature would need its own reviewed adapter rather than generic injection.

Bind every upstream request ID to its owning caller/invocation and server/connection generation. No late reply may be delivered to another caller or a newer generation. No provider test, login, discovery preflight, restart or automatic replay occurs on the ordinary call path.

## Proposed config PATCH and management-tool amendment

D54 confirms the configurable per-server default represented by execution.default_timeout_ms, initially120000. The broader proposed amendment extends the existing config PATCH and fetch__update_server_config input schema with one optional Fetch-owned execution object. Its only writable members are max_concurrent_calls, default_timeout_ms and tool_timeouts_ms. This is an explicit amendment to D30's launch-only fields, not normalization of arbitrary native MCP settings.

```json
{
  "execution": {
    "max_concurrent_calls": 2,
    "default_timeout_ms": 120000,
    "tool_timeouts_ms": {
      "search": 300000,
      "old-tool": null
    }
  }
}
```

Omitted fields remain unchanged. execution:{} is a no-op. Merge supplied timeout-map entries by exact tool name: a positive integer sets its Fetch override, null removes that override and resumes known-limit/fallback selection, and an empty map is a no-op. Null never means unlimited. Per-entry updates preserve policy entries hidden from a caller rather than replacing an entire filtered map. Existing args/environment array replacement and working_directory semantics stay D30.

Use existing config-write authority. Tool-feature/profile/override names additionally obey D38 metadata visibility; a management reader cannot use this extension to discover hidden tools. Settings may manage its full server; other callers may only read/edit authorized per-tool entries. Filter profile/override metadata before disclosure; lack of list authority does not fabricate an empty complete profile. No source/provenance fields or reported limits are caller-writable, and no raw provider values are accepted.

Malformed/wrong-type/unknown/protected fields use400 INVALID_REQUEST; nonpositive/invalid setting content uses422 INVALID_CONFIG. Existing401/403/404/500 save and server rules remain. Never echo rejected values. Exact numeric implementation upper bounds and schema encoding remain review before build; proposed defaults are not promises of unbounded resource allocation.

Validate/save a mixed launch/execution patch atomically before publication; if it fails, apply none. Execution-only changes take effect for new admissions, preserve provider-auth evidence and existing restart_required, and do not restart the child. Launch-field changes retain D30 auth invalidation/restart_required behavior. Raising concurrency can dispatch waiting entries; lowering it leaves active calls intact and pauses new starts until usage falls below the new cap. Existing accepted requests keep captured budgets.

## Cancellation and unresolved active work

On execution timeout or an actual cancel signal, attempt revision-correct upstream cancellation and end the caller wait. Propose ordinary Fetch HTTP disconnect as cancellation; modern MCP stream closure is cancellation. Legacy MCP SSE loss alone is not cancellation: preserve the queued/dispatched request until explicit cancel, its captured deadline or completion, without inventing replay/resumption. Background/red-close alone must not deliberately cancel work; Quit retains selected shutdown behavior.

Cancellation may have no acknowledgement or final reply. After possible forwarding, retain the occupied active slot until a terminal ordinary upstream RPC response (ToolResult or protocol error) or confirmed managed-child exit. A locally cancelled future, stream closure or elapsed timer cannot clear it. Intermediate interaction/task responses are not terminal completion. Uncertain slots count within max_concurrent_calls, not outside it. Other free slots can still dispatch work in scheduler order.

Discard late result payloads once the owning wait ends; a genuine terminal response can release capacity without turning the caller's earlier unknown outcome into success. No call-result history/job/status/cancel API is introduced. D63 explicitly permits fresh current-context capacity after authorized forced remote reconnect, preserving unresolved_prior_calls separately; this is not proof the old remote work stopped. Outside that selected exception, a server that suppresses its cancellation reply may retain capacity until explicit force recovery; no automatic kill/restart or action replay is permitted. Keep other caller identities, request IDs, arguments and results out of queue/status/error data.

## Restart, failure and generation changes

Propose accepting restart atomically closes admission and removes queued unforwarded entries with 409 SERVER_BUSY/not_started, then D20 drains only already-forwarded calls up to 60 seconds. If normal restart later fails, do not reinsert removed entries. An uncertain active slot continues counting after the restart gate reopens.

Explicit force preserves D20 confirmed-stop/readiness rules. Clear unforwarded queue entries before stopping; forwarded interrupted calls remain unknown. If the child/connection becomes unusable without restart, fail unforwarded entries safely with 503 SERVER_NOT_READY/not_started rather than carrying them into a new generation; unresolved local active slots release only with required terminal/confirmed-child-exit evidence; remote contexts follow the explicit D63 forced-recovery exception without erasing unknown prior outcomes. Quit/crash does not create queue recovery or automatic requeue. No stopping path promises external effects were undone.

## Proposed aggregate server GET state

Keep runtime.status unchanged and add one tool_execution object to each existing server record, including restart-filtered views:

```json
{
  "tool_execution": {
    "active_calls": 1,
    "queued_calls": 4,
    "uncertain_calls": 0,
    "max_concurrent_calls": 1,
    "max_queued_calls": 64,
    "max_queued_bytes": 16777216,
    "queue_wait_timeout_ms": 300000
  }
}
```

uncertain_calls is included in active_calls. State reads are local/no-probe/no-store under existing GET authority. No caller/call identifiers, arguments or results appear. This replaces the earlier unapproved single tool_call_state proposal. Empty/free counts do not establish a previous action succeeded. Configured settings/features appear under the safe config view with additional tool-metadata visibility controls described above.

The UI uses these structured counts/states; next_steps gives the same reviewed permitted actions and uncertainty/recovery handoffs under D32/D33. Existing HTTP call response fields and raw MCP presentation stay D50/D43/D44. Queue removal/full/expiry and settings edits do not automatically submit another call.

## Acceptance and open implementation dependencies

After authorization, test in a D46-isolated environment: FIFO dispatch/no bypass at C greater than one, waiting count/byte boundaries with no hidden queue, separate deadlines/immutable admitted budgets,120 fallback versus longer documented/manual limits,5-second allowance, unknown initial profile/no install startup or GET writes, current-versus-stale observations, raw argument/result fidelity, partial-authority map edits without hidden-entry loss/disclosure, atomic mixed PATCH/no-auth-invalidation for execution-only changes, hot capacity changes, cancel/expiry/dispatch races, uncertain-slot accounting, late replies/owner isolation, child death and restart flushing without replay, and matching UI/guidance.

D68 approves presented queue64/16MiB/300seconds, default/configurable concurrency1,5second allowance, shown execution config members, hot application, restart/failure/uncertainty policy and aggregate GET semantics. Exact unshown profile/GET wire fields, numeric upper bounds and error refinements remain open; do not ask again about selected defaults. Payload/result/ingress limits, concurrency upper bounds, SDK/timer/process/persistence mechanisms and full richer MCP/GitHub/auth scope remain explicit before implementation. No source, dependency, fixture/environment or live MCP test was created or run.

## Per-tool transport context — D62

Per-tool Fetch configuration includes the information needed to route to the owning server/transport adapter. Carry it alongside the invocation through scheduling/dispatch rather than injecting fields into raw arguments or deriving executable routing from returned content. Tools on the same connection must route consistently. Exact physical fields/reference layout and update mechanics remain open; shared caller permissions and execution policy remain in effect.

D61/D62 follow-up: shared-runner coverage includes stdio, Streamable HTTP and legacy HTTP+SSE dispatch/message/result handling through their transport functions. Shared permissions, scheduling and deadline rules apply across these paths; exact version/cancellation/remote uncertainty refinements remain open, not silently derived from local child behavior.

The [approved native relay](mcp-interaction-relay.md) treats interaction rounds as one logical call under its original deadline, without premature completion/reservation release or automatic uncertain-call replay. D68 subsequently selects the presented ordinary cancellation/drain/uncertainty/queue behavior and defaults; exact unshown mechanisms remain open.

D66 approves the presented native relay as one logical operation across control rounds with its original deadline and no premature completion/reservation release. D68 subsequently approves the presented ordinary queue/concurrency/cancellation/uncertainty/recovery parameters at its shown scope; remaining mechanisms are not approved merely by D66.
