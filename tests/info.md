# Backend verification

These tests use temporary data, fixture configuration files and test-only authority objects. HTTPS tests open an IPv4 loopback listener on an available port and trust its public certificate only inside that test's HTTP client.

They do not connect MCP servers, contact providers, read personal client configuration, change system certificate trust or access Keychain. Passing these checks does not establish MCP or client compatibility.

Run `cargo test --tests`. The full developer check is `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, then `cargo test`.
