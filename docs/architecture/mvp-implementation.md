# MVP implementation brief — October 28, 2026

D77 authorizes the full selected source implementation and scoped commits/pushes, with **all testing deferred** to the user's control. The locked specification stays in build-readiness.md. This brief records internal engineering choices; it adds no product routes or permissions.

## Source boundaries

- Authority: Caller/grant ownership, OAuth code/refresh/revoke operations, native Mac confirmation, Keychain-backed encrypted records and lasting TLS identity.
- Manager: private source capture, pinned runtime/package preparation, same-run installation/restart records, safe configuration/authentication/credential operations and live-state overlays.
- Runner: one logical context per server, stdio/HTTP/SSE adapters, raw JSON catalogs/calls, scheduling and native MCP owner-bound messages.
- Integration: one AppState/container, existing HTTP API wrappers, the native Iced interface and explicit app/DMG packaging scripts. UI controls use structured states.

Use existing AES-GCM/atomic private-file primitives and researched Rust HTTP/TLS crates. Add request streaming/process I/O support, an Objective-C bridge for Apple's LocalAuthentication/Keychain, and Iced 0.14's daemon interface so closing a window can keep services alive. No external bridge helper process owns backend authority. The maintained MIT Iced source and version-specific API are references: https://github.com/iced-rs/iced/tree/0.14.0 and https://docs.rs/iced/0.14.0/iced/fn.daemon.html. No source is copied.

Credential success still requires encrypted store/readback plus the service's authenticated endpoint using the stored credential. Unsupported wrapper-owned login/token handoff is reported truthfully, never treated as verified by MCP readiness. Normal calls reuse credentials without opening login. No generic shell/credential tools or unselected custom GitHub CLI adapter is added.

## Deferred execution

Do not run Cargo, tests, checks, formatters, linters, validation scripts, services, applications, runtime/package installers, connection attempts, trust changes or packaging in this implementation pass. Dependency resolution and Cargo.lock refresh are deferred together with first compilation. Do not infer that code compiles, clients connect, OS prompts work or packaging succeeds. The existing isolated-testing plan remains unprovisioned.
