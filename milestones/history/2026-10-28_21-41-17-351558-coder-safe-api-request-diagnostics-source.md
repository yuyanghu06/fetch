# Safe api request diagnostics source

- Local timestamp / acting role / milestone: 2026-10-28T21:41:17.351558-04:00; coder; inline-server-authentication / MVP completion.
- Requested scope: user asks whether all API calls were logged; existing source has no shared request diagnostics.
- Starting repository state: only startup errors use stderr; no request/result logging.
- Work performed / changed files: Coder adds src/diagnostics.rs, lib.rs registration and outer API middleware; root adds task-local request context, upstream send wrapper, fixed application-error codes, and uses wrapper in native client, installer and MCP HTTP transports. Manager Coder integrates provider requests and shared auth outcomes separately.
- Decisions: JSON lines to stderr, generated IDs, UTC timestamps, route templates/fixed operation labels, HTTP status and duration; cancellation before response headers. Never bodies/queries/headers/raw paths/URLs/secret values/provider diagnostics. Streaming/header completion is explicitly not tool completion.
- Checks/evidence: independent source review finds no secret leakage/security blocker; source grep shows direct reqwest send remains only inside diagnostics helper. No tests/app/provider/MCP/credential activity. Compile pending after integration.
- Resulting state / acceptance: logging source implemented, review in progress; no log file is created automatically. Subprocess-internal npm/uv/provider CLI HTTP is not intercepted.
- Remaining work: binary compile and final source review; actual emitted-event and runtime checks remain unrun.
