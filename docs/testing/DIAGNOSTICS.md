# Fetch request diagnostics

October 28, 2026. Source implementation; actual runtime event checks are unrun.

Fetch writes one JSON object per line to standard error, visible in the terminal
that launches it. No log file is created automatically. First-party code logs:

- Every incoming application, OAuth and MCP HTTP request, including boundary
  rejection, unknown routes and wrong methods.
- Native UI requests to Fetch, provider metadata/registration/token/verification
  requests, direct package downloads and outgoing MCP HTTP requests.
- Known authentication outcomes and Fetch-owned application error codes.

Fields include generated request_id, UTC timestamp, registered route template or
fixed operation, HTTP status and elapsed_ms. Upstream events include a safe
parent_request_id when a request context exists. Background jobs can have no
incoming request context. No bodies, argument objects, credentials, query values,
headers, raw paths/URLs, provider error text or child stderr are logged.

request_completed/upstream_request_completed mean response headers were received
or prepared. They do not prove the complete response stream or tool call finished.
A cancelled future before headers emits a cancellation event. Requests performed
inside npm, uv or third-party server/CLI processes are not intercepted by this
first-party logger. This is diagnostics coverage, not a provider-test result.
