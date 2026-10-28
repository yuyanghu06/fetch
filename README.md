# Fetch

Fetch is a local macOS app that lets coding apps share installed MCP servers through one Fetch connection. The selected MVP source is now written: native UI, local HTTPS, app authorization, protected credentials, installation, server management and MCP sharing.

**Prior native compilation succeeded; current runtime behavior remains unverified.** Under D79, `cargo build --locked --bin fetch --message-format short` succeeded and produced `target/debug/fetch`. Those are historical build results. The user now reports running a development build. The development-path changes now also compile successfully; their four new regression tests remain unexecuted.

Read [the testing handoff](docs/testing/HANDOFF.md) before running anything. It lists the build result, remaining verification and supported-import/authentication limits. Cargo.lock now records the resolved dependencies. The prior D79 build reported four warnings for deprecated classic-Keychain APIs and one unread protocol field, with no build errors. The development-state follow-up compiled successfully with the same warning categories; no tests or executable launch were run.

## Development launch

```sh
cargo run --locked --bin fetch -- app
```

Paths resolve from the current user's home on either Mac. Development packages,
runtimes, caches and saved credentials follow the development root. No previous
state is moved. When testing release binaries, override both data directory and
port; see [development paths and fresh Codex setup](docs/testing/DEVELOPMENT.md).

## Source layout

- `src/oauth`, `src/native_auth`, `src/identity`, `src/credentials`: app access, native Mac confirmation, HTTPS identity and Keychain-backed encryption.
- `src/discovery`, `src/installation`, `src/storage`, `src/manager`: private pinned setup and shared server/config/auth/restart rules.
- `src/runner`, `src/mcp`: three upstream transports, raw MCP data, queues and the selected native surface.
- `src/api`, `src/ui`, `src/application.rs`: the shared application APIs, native interface and one-process lifecycle.
- `packaging`, `scripts/package-macos.sh`: app/DMG construction source; not executed.

The native debug app defaults to `~/Library/Application Support/Fetch-Development/` and `https://localhost:53672/mcp`; final release builds default to `~/Library/Application Support/Fetch/` and `https://localhost:53671/mcp`. Launch creates protected app state and starts saved managed connections. The original `dev` command remains a separate fixture-only foundation with ephemeral HTTPS and denied app authority. Neither has been launched during the source or compile-only passes.

Personal client configurations, existing services, credentials and certificate trust were left untouched. The selected isolated MCP test roster is recorded, with every check still not run. No release or DMG has been produced.

[Selected requirements](docs/architecture/build-readiness.md), [decisions](docs/architecture/decisions.md), [current milestone](milestones/milestone.md) and [work remaining](milestones/to-do.md) distinguish source completion from verified acceptance.

MIT licensed. Dependencies retain their own licenses; [the source-stage notices](THIRD_PARTY_NOTICES.md) identify the final notice work needed before binary distribution.
