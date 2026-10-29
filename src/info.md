# Shared Rust backend

D81 current source includes the application host, real native authority/encrypted
storage, private installer/manager, shared runner/MCP gateway and API-backed UI.
browser.rs now centralizes escaped Fetch-owned HTML rendering; native repeated
rendering/theme lives in ui/components.rs and ui/theme.rs. Generic remote OAuth
metadata/registration/renewal source is in manager/remote_auth.rs; authentication
proof and credential-outcome amendments remain pending. Locked native build and
68 selected fixture tests pass; Clippy completes with warnings. Actual native/
provider/MCP acceptance remains unproved. The paragraphs below retain earlier
foundation/source/compile-pass history and are not current missing-module claims.

`lib.rs` exposes the backend used by interfaces. `main.rs` provides the explicit developer command. `development.rs` owns ephemeral HTTPS and developer-state isolation; it is not a production daemon or lasting certificate implementation. `authority.rs` defines the trusted authorization boundary and currently denies all real grants. `api/` wraps backend services; discovery, storage and credentials have separate guides. No managed server process or MCP relay is started by this increment.

The D77 source pass adds native app authorization, installation/management, the shared runner/MCP gateway and an API-backed Iced interface. application.rs wires one process and a private single-instance activation socket. The original development launcher remains fixture-only; the native app command uses lasting identity and Keychain-backed state when the user explicitly launches it. No D77 source has been compiled or tested.

D79 binary compilation succeeds after a gateway syntax/import correction. No executable, app/service or MCP connection was started; runtime behavior is still unverified.

Current development defaults: native debug app resolves the current home and uses
Library/Application Support/Fetch-Development with port53672; release uses Fetch
with port53671. application.rs exposes side-effect-free path/port selectors and
main.rs applies defaults while preserving explicit overrides. Installer, manager,
credentials and runner retain the selected root. See docs/testing/DEVELOPMENT.md.
The follow-up locked native binary build succeeded in9.74seconds; no runtime
check or regression test was executed.

`diagnostics.rs` owns safe first-party HTTP metadata logging. API middleware,
provider HTTP, native UI requests, direct package downloads and remote MCP
transports use it without logging secrets, URLs, headers or bodies.
