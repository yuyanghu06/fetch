# First Rust developer-backend increment

October28, 2026. D74 authorizes starting implementation and pushing, without a release. This bounded increment makes the backend runnable; it does not claim the full five-stage MCP plan complete.

## Engineering choices for this increment

Use one Rust edition2024 Cargo package with a shared library and a developer-only command. Axum/Tokio handle existing API routes; axum-server/rustls provide HTTPS with the ring provider. Cargo.lock records actually resolved versions. The later Iced app uses the same backend inside the single Fetch process; this command is not a new production daemon.

The developer command binds IPv4 loopback and publishes https://localhost:4141 by default. A different explicit port is supported, including zero for test allocation. Host must match the configured authority and supplied Origin must match the configured origin. Proxy headers cannot authorize a request. Application JSON includes next_steps and no-store; OAuth/protocol envelopes keep their own formats.

Developer data is explicitly separate from production Application Support. Each run creates an in-memory TLS private key and writes only its public certificate into the selected private development directory. No trust installation, Keychain call, lasting identity, saved agent token or MCP process is involved. This is an explicit developer mechanism, not a substitute for D70's lasting protected installation identity. The planned installation identity and its VM verification remain unfinished.

Discovery uses injected source roots in tests. JSON/JSONC duplicate checks run on decoded AST properties before map conversion, JSON5 extensions are disabled, and TOML uses its duplicate-rejecting parser. Reads are bounded at4MiB. Unknown launch syntax is blocked; no shell/network/helper is invoked. The source guide lists the implemented grammar. Remote metadata uses only the approved safe transport/origin/header-name projection.

Saved non-secret server records and encrypted credential primitives use injected storage/key boundaries. Test keys exist only in tests. There is no production Keychain provider, provider-auth verification, package installation or public save route in this increment. Cipher persistence alone never reports authentication success.

The running developer authority denies all grants until native approval/token handling is implemented. There is no static developer token or browser approval shortcut. Public resource metadata works; authorization-server metadata truthfully reports503 rather than advertising usable grant endpoints. Protected application reads are tested using authority injected only by test code. MCP sharing is unavailable; no native/protocol interoperability is claimed.

## Research before implementation

The dedicated Researcher checked current published crate archives, GitHub maintenance, licenses and prior issue status before dependent code. Axum0.8.9/Tokio1.54.1/axum-server0.8.0/rustls0.23.46/rcgen0.14.10 are compatible source candidates, subsequently verified by the actual build. rcgen's published signing_key API differs from current main. The TLS provider is chosen explicitly to avoid accidental provider ambiguity.

- [Axum](https://github.com/tokio-rs/axum), [Tokio](https://github.com/tokio-rs/tokio), [axum-server0.8 TLS API](https://docs.rs/axum-server/0.8.0/axum_server/tls_rustls/): maintained, MIT.
- [rustls](https://github.com/rustls/rustls), [rcgen](https://github.com/rustls/rcgen): maintained, permissive licenses recorded in dependency metadata.
- [jsonc-parser AST](https://github.com/dprint/jsonc-parser/blob/main/src/ast.rs), [parser options](https://github.com/dprint/jsonc-parser/blob/main/src/parse_to_ast.rs): maintained MIT implementation; defaults permit more than JSONC, so configure explicitly.
- [TOML1.1 key rules](https://toml.io/en/v1.1.0#keys) and published toml1.1.8 parser source: duplicate definitions rejected; do not normalize/merge input before parsing.
- [serde_json RawValue](https://docs.rs/serde_json/latest/serde_json/value/struct.RawValue.html) supports raw canonical copies. Enable raw_value/arbitrary_precision now; tests retain unknown fields and large numbers.
- [rmcp Tool model](https://github.com/modelcontextprotocol/rust-sdk/blob/rmcp-v3.8.0/crates/rmcp/src/model/tool.rs) has fixed fields without a catchall; typed round trips drop unknown upstream fields. Its HTTP client also filters definitions after annotation checks. Do not adopt these models as lossless canonical Tool/result storage. No rmcp dependency is selected for this increment. Closed historical issue1168 is not described as a current defect.
- [Iced daemon](https://docs.rs/iced/0.14.0/iced/fn.daemon.html) supports keeping the process alive after windows close; its actual integration remains future work. Real Keychain/Mac confirmation behavior is deferred to approved isolated evidence.

No upstream application code was copied. Direct source dependencies retain their own licenses. MIT LICENSE applies to Fetch's source. No release or DMG is created in this pass.

## Verification boundary

Formatting, compilation, meaningful fixture tests, actual loopback HTTPS public metadata and independent review cover this increment. They do not establish managed MCP/client/provider compatibility. No managed MCP server is installed, started or contacted; personal config, credentials, trust and desktop-app state are excluded. Real MCP feature testing with the user's inventory happens together later in a suitable isolated virtual environment.
