# Restart contract — approved behavior

The user approved the request/result fields on October 26, 2026, then requested one complete contract per turn. On October 27, after the whole remaining contract and a clarification that stopping is one stage of restarting, the user said "Cool all good next." The remaining rules below are now approved as presented. PostgreSQL, retention and cross-launch recovery remain deferred; implementation is still paused. See decision D20 in [decisions.md](decisions.md).

D21 subsequently moves the existing server family from `/api/v1/setup/servers` to `/api/v1/servers`. The URLs and status guidance below use the new path; approved restart behavior is unchanged.

## Approved request and result format

```http
POST /api/v1/servers/{server_id}/restart
```

JSON body: `force_restart` is an optional boolean, default `false`.

```json
{
  "force_restart": false
}
```

Normal restart waits for Fetch-forwarded calls and rejects new calls to this server temporarily. Force begins restart immediately without that wait and may interrupt active calls. It does not waive readiness checking or persist as server configuration.

Return `202` immediately after acceptance. Approved body, with invented identifiers:

```json
{
  "server_id": "github",
  "restart_id": "restart-42",
  "status": "restarting",
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_restart_status",
      "instructions": "GET /api/v1/servers?id=github&restart_id=restart-42 until this restart succeeds or fails."
    }
  ]
}
```

Monitor the named attempt through the accepted query:

```http
GET /api/v1/servers?id=github&restart_id=restart-42
```

Approved nested restart states:

| State | Fields | Settings behavior |
| --- | --- | --- |
| Pending | `status: restarting`; `phase` is `waiting_for_calls`, `stopping`, `starting`, or `checking_readiness`. | Keep loading throughout. |
| Succeeded | `status: succeeded`, `code: SERVICE_STARTED`. | Show success. |
| Failed | `status: failed`, `error.code`, `error.message`. | Stop loading and show the failure. |

## Latest storage amendment

The user subsequently deferred retention and all PostgreSQL functionality from the initial MVP. The earlier 24-hour PostgreSQL result-retention/durable recovery proposal is sidelined. Accepted live `202`/ID/status behavior remains; no replacement persistence or memory-only lifetime has been selected. Read [storage-scope.md](storage-scope.md). The October 27 approval confirms the operational clauses below without selecting a persistence mechanism or retention policy.

## Approved operational rules — October 27, 2026

### Callers, validation and acceptance

- Settings and authenticated Fetch clients authorized for server management may request restart/read its status. Ordinary agents still cannot write manual credential values. Exact OAuth permission implementation belongs to the authentication step, but the backend must enforce the selected operation permission.
- Restart applies to an installed, locally managed wrapper. Accept an empty body as `{}`; reject malformed JSON, non-object bodies, unknown fields and a non-boolean force value. Never echo supplied values or secrets in validation errors.
- Capture the saved launch settings and assigned environment references when accepting the request; resolve their values privately for that server. Apply that captured configuration and its pinned package version. Later saved edits remain pending for another restart; this is internal applied-state tracking, not configuration history/rollback.
- Create one opaque restart ID per accepted attempt. The `Location` header points to the server/restart status URL; clients check it once per second while pending.

### Execution and readiness

- Allow one active restart per server. Another POST for the same server returns `409` with the active `restart_id`, including a second force request; it does not start another job. Different servers can have independent restart jobs. This does not choose ordinary tool-call concurrency or add wrapper instances.
- Normal restart gates new calls and waits up to **60 seconds** for already-forwarded calls. On wait timeout, fail the attempt, keep the old wrapper running and reopen its call admission; saved changes remain unapplied. Never automatically switch to force.
- Once calls drain, request normal termination and wait up to **5 seconds** for confirmed exit. If stopping fails, report failure and do not spawn a duplicate. Keep tracking the old process; the server remains unavailable until a successful retry/recovery.
- Force requests begin forced termination immediately, without call draining or the normal termination grace. Confirm exit within the same 5-second stop deadline before replacement; report failure if it cannot be confirmed. Interrupted calls fail rather than being automatically replayed. Remote effects already submitted are not undone.
- Start only the replacement for this server. Allow **30 seconds** from replacement launch for compatible MCP communication and a successful `tools/list` exchange, including all pages if needed. The managed replacement must still be alive. A valid empty list is allowed; no actual tool invocation is used as a health check.
- On startup/readiness failure, stop/track the failed replacement and report failure; keep this server unavailable until corrected/retried. No automatic rollback or repeated restart loop. Fetch and unrelated wrappers keep running.
- `SERVICE_STARTED` means the replacement's MCP interface is responsive and its tools can be discovered. Provider authentication is separate: successful listing does not prove every GitHub/Supabase action is authorized. If listing itself requires login and fails, readiness fails with authentication guidance. Exact supported protocol/SDK versions and OS process-group cleanup are implementation choices to review later, without changing these observable promises.

### Monitoring, retention and interruption

- A restart-filtered GET requires exactly one non-empty `id` and one non-empty `restart_id`. Reject missing, repeated or multiple selectors for this filtered form with `400`. Ordinary GETs without the restart filter keep the previously accepted list/multi-ID behavior.
- Found attempts return HTTP `200`, including pending and failed attempts: HTTP success means the status was retrieved; the nested restart status reports the operation outcome. Do not interpret HTTP `200` alone as `SERVICE_STARTED`.
- A transport/status-read failure is not evidence that the restart itself failed. Show a status-retrieval error and permit another GET without resubmitting the POST or replacing the last known job outcome with success/failure.
- The filtered view uses top-level `status: ok`, `servers`, `not_found_ids` and `next_steps`; only the nested attempt determines restart completion.
- Unknown server, unknown restart, or an ID belonging to a different server returns `404`; never substitute a different attempt's result. Error replies retain `servers: []` and `not_found_ids` for unknown server IDs. Exact error codes are listed below.
- **Deferred by the user:** PostgreSQL result persistence, the proposed 24-hour retention/expiry window and historical job recovery. Current-operation progress is still needed for polling; exact local state/lifetime behavior is open. Do not assume all records are discarded or that a replacement database is selected. IDs remain opaque references, not credentials.
- UI closure or caller disconnection after acceptance does not cancel the backend job. Explicit Quit still stops managed servers. Cross-Quit/crash attempt recovery is deferred with the unselected persistence/retention design; do not promise old IDs survive relaunch. A later startup must not be mistaken for the original attempt's success.
- No automatic POST retries. While an attempt is active, a repeated POST receives the active-ID `409`. A POST accepted after a terminal result creates a new attempt; the restart ID itself is not an idempotency key.

Approved terminal success response for the restart-filtered GET; other ordinary server-detail fields belong to that route's full contract:

```json
{
  "status": "ok",
  "servers": [
    {
      "server_id": "github",
      "restart": {
        "restart_id": "restart-42",
        "status": "succeeded",
        "code": "SERVICE_STARTED"
      }
    }
  ],
  "not_found_ids": [],
  "next_steps": []
}
```

Pending responses use the approved nested `restarting`/`phase` fields and check-status guidance. Failure responses use the approved nested error object and guidance to correct/retry; no secret values or raw wrapper stderr appear. `phase` is present only while pending; success has `code` and no `error`; failure has `error` and no success `code`.

### Approved errors for this operation

| HTTP result | Code and meaning |
| --- | --- |
| `400` | `INVALID_REQUEST`: body/selector validation failure. |
| `401` / `403` | `UNAUTHENTICATED` / `FORBIDDEN`: Fetch login absent/invalid or management permission missing. |
| `404` | `SERVER_NOT_FOUND` or `RESTART_NOT_FOUND`; a server/restart mismatch uses `RESTART_NOT_FOUND`. |
| `409` | `SERVER_NOT_INSTALLED` or `RESTART_IN_PROGRESS`; the latter includes the active restart ID for monitoring. |
| `500` | POST `RESTART_ACCEPTANCE_FAILED`: Fetch cannot create/schedule the job; do not return `202` for unaccepted work. GET `RESTART_STATUS_UNAVAILABLE`: status cannot be read, so the job outcome remains unknown. |
| Status GET `200`, nested `failed` | `RESTART_WAIT_TIMEOUT`, `SERVER_STOP_FAILED`, `SERVER_START_FAILED`, `SERVER_READY_TIMEOUT`, or `SERVER_AUTH_REQUIRED`. These are job failures after acceptance, not failures to read status. |

Approved request-level error shape; restart-filtered GET errors also include the empty server array and missing-server list described above:

```json
{
  "status": "failed",
  "error": {
    "code": "RESTART_IN_PROGRESS",
    "message": "This server already has a restart in progress."
  },
  "restart_id": "restart-42",
  "next_steps": [
    {
      "actor": "agent",
      "action": "check_restart_status",
      "instructions": "Check the active restart instead of submitting another request."
    }
  ]
}
```

Only in-progress conflicts include that active ID. Other request errors use `status`, `error` and relevant `next_steps`. IDs/messages are examples; no generated secrets enter the response.

D33 requires restart guidance and native UI sequencing to agree on the approved attempt/state/force/wait/readiness/failure rules. The UI drives its spinner/controls from the named attempt’s structured result, not next_steps text; agents receive matching guidance. No permission or retry behavior is added.

## Proposed acceptance evidence

Verify actual HTTP acceptance and attempt-specific polling; normal draining/new-call rejection; immediate force/interrupted-call failure; each timeout and no duplicate process; correct captured versus later settings; caller/UI disconnection; current-operation lookup/errors; denied callers; tool-list readiness with valid empty/paginated lists; independent provider-auth state; Quit cleanup and the selected current-run behavior; unaffected other wrappers. Database-backed retention/recovery tests are deferred. These tests are not implemented or run.

Primary grounding: [MCP tool discovery](https://modelcontextprotocol.io/specification/2026-07-28/server/tools#listing-tools) distinguishes listing definitions from executing tools and permits empty/authorization-dependent lists. [Versioning](https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning) requires version-appropriate communication; an older initialization sequence must not be assumed universally. These sources do not choose Fetch's proposed timeouts, permissions, retention or overlap policy.

This operation is approved for the behavior shown above. Continue step 1 with the next complete setup contract. Exact ID representation, supported protocol/library versions, process cleanup implementation and local operation-state lifetime remain for their designated design steps. No application tests have run and no implementation is authorized.
