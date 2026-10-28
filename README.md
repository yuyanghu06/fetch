# Fetch

Fetch is a local macOS app being built to let your coding apps share installed MCP servers through one Fetch connection. This repository currently contains the first Rust backend increment. It is not ready to replace your existing MCP setup.

## Run the developer server

Install Rust 1.89 or newer, then run from this folder:

```sh
cargo run --locked --bin fetch -- dev
```

The backend listens at `https://localhost:53671`. Stop it with **Ctrl-C**. Use another port if needed:

```sh
cargo run --locked --bin fetch -- dev --port 53672
```

Developer data goes in the ignored `.fetch-dev/` directory. It does not use your normal Fetch Application Support folder or scan your personal client configurations. You can select another private directory with `--data-dir /absolute/path`.

Each run creates a temporary HTTPS certificate. Its private key stays in memory; only the public certificate is saved. The server never adds it to your Mac's trust settings or Keychain. Because it changes on restart, use the certificate from the current run when checking HTTPS:

```sh
curl --cacert .fetch-dev/certificate.pem \
  https://localhost:53671/.well-known/oauth-protected-resource/mcp
```

This returns Fetch's public connection metadata. Opening that HTTPS address in a browser will show an untrusted-certificate warning; there is no browser interface yet. The temporary developer certificate is separate from the planned lasting installation certificate.

## What works in this increment

- A Rust backend with loopback-only HTTPS, strict Host/Origin checks and no-store responses.
- Approved server-list and discovery handlers, with fixture tests for their permissions, inputs and response shapes.
- Read-only Claude Code JSON, Codex TOML and OpenCode JSON/JSONC discovery with bounded reads and safe metadata. The supported launch forms are documented in [the discovery guide](src/discovery/info.md).
- Private local records and encrypted credential-store primitives using an injected key provider. Saved-record persistence currently covers local servers; remote records and execution settings still need implementation. No real Keychain provider or provider-login verification is implemented yet.
- A separate development data directory and a lock preventing two instances from using it at once.

Native Mac approval, OAuth token issuance, the desktop interface, package/runtime installation, provider authentication and MCP sharing are still being built. Protected routes reject requests; there is no developer token or login bypass. OAuth-server metadata returns `503` while authorization is unavailable. Fixture tests inject test authority directly into the backend; the running developer server cannot issue grants.

The headless developer command uses the shared backend library. The planned desktop app will run that library inside the same application process.

## Check the code

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

Tests use disposable files and keys. HTTPS smoke tests connect only to this backend's public metadata with a temporary certificate. They do not start or connect to managed MCP servers, use personal credentials, change trust or edit client configurations. Real MCP tests with your server inventory will happen later in an isolated virtual environment.

## Project records

[Current build plan](docs/architecture/build-readiness.md), [architecture decisions](docs/architecture/decisions.md), and [current milestone](milestones/milestone.md) distinguish selected behavior, implemented code and pending verification. No release has been created.

MIT licensed; dependencies retain their own licenses.
