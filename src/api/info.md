# HTTP application routes

D81: provider callbacks now retain one optional iss value and reject duplicates;
generic OAuth validates its exact saved issuer before exchanging codes. Shared
browser HTML is supplied by src/browser.rs without changing API authority.
Independent fixture HTTP/MCP/TLS checks pass within the combined68-case suite;
actual native/provider/client operation remains unproved. Older no-execution
paragraphs below describe their original source/compile passes.

Handlers use shared backend services. Application JSON responses include `next_steps` and are never cached. Host (or HTTP/2 authority) and optional Origin must match the configured local address; proxy headers grant no authority. The fixture-only developer mode keeps authorization-server metadata unavailable; the native application wires the implemented OAuth service. Protected reads authorize before query validation or filesystem metadata. No developer token bypass exists. HTTP mock-authority tests live outside production code.

runtime.rs adapts the selected installation/server/config/credential/auth/restart/tool routes to shared backend services. Native Settings authority is checked before manual-value parsing and Mac confirmation. Installed-tool arguments/results use original RawValue JSON. The management-tool adapter calls the same services and preserves application JSON guidance. All new handlers are unverified source in D77; no HTTP requests have been made.

D79 removes an unused import during native binary compilation. The binary builds successfully; HTTP/native/client behavior remains untested and no listener was launched.

The outer router layer logs every incoming application/OAuth/MCP HTTP request,
including boundary/fallback failures, through diagnostics.rs. Fetch-owned error
codes and shared authentication outcomes are logged with generated request
context; response streams and raw provider data remain unchanged.
