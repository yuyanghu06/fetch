# Shared Rust backend

`lib.rs` exposes the backend used by interfaces. `main.rs` provides the explicit developer command. `development.rs` owns ephemeral HTTPS and developer-state isolation; it is not a production daemon or lasting certificate implementation. `authority.rs` defines the trusted authorization boundary and currently denies all real grants. `api/` wraps backend services; discovery, storage and credentials have separate guides. No managed server process or MCP relay is started by this increment.

The D77 source pass adds native app authorization, installation/management, the shared runner/MCP gateway and an API-backed Iced interface. application.rs wires one process and a private single-instance activation socket. The original development launcher remains fixture-only; the native app command uses lasting identity and Keychain-backed state when the user explicitly launches it. No D77 source has been compiled or tested.
