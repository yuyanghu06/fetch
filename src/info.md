# Shared Rust backend

`lib.rs` exposes the backend used by interfaces. `main.rs` provides the explicit developer command. `development.rs` owns ephemeral HTTPS and developer-state isolation; it is not a production daemon or lasting certificate implementation. `authority.rs` defines the trusted authorization boundary and currently denies all real grants. `api/` wraps backend services; discovery, storage and credentials have separate guides. No managed server process or MCP relay is started by this increment.
