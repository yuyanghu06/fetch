# Fetch

Fetch is a local macOS app that lets coding apps share installed MCP servers through one Fetch connection. The selected MVP source is now written: native UI, local HTTPS, app authorization, protected credentials, installation, server management and MCP sharing.

**Testing has not started.** This implementation pass ran no builds, checks, formatters, linters, tests, apps, services, connections or packaging. The new source is uncompiled and unverified. The user controls when verification begins.

Read [the testing handoff](docs/testing/HANDOFF.md) before running anything. It lists the implemented source, exact remaining verification, supported-import/authentication limits and the first build command. Cargo.lock still belongs to the earlier foundation; dependency resolution is deliberately deferred to that first user-started build.

## Source layout

- `src/oauth`, `src/native_auth`, `src/identity`, `src/credentials`: app access, native Mac confirmation, HTTPS identity and Keychain-backed encryption.
- `src/discovery`, `src/installation`, `src/storage`, `src/manager`: private pinned setup and shared server/config/auth/restart rules.
- `src/runner`, `src/mcp`: three upstream transports, raw MCP data, queues and the selected native surface.
- `src/api`, `src/ui`, `src/application.rs`: the shared application APIs, native interface and one-process lifecycle.
- `packaging`, `scripts/package-macos.sh`: app/DMG construction source; not executed.

The native app's default MCP endpoint is `https://localhost:53671/mcp`. Launching it later creates protected app state and starts saved managed connections. The original `dev` command remains a separate fixture-only foundation with ephemeral HTTPS and denied app authority. Neither has been launched during this pass.

Personal client configurations, existing services, credentials and certificate trust were left untouched. The selected isolated MCP test roster is recorded, with every check still not run. No release or DMG has been produced.

[Selected requirements](docs/architecture/build-readiness.md), [decisions](docs/architecture/decisions.md), [current milestone](milestones/milestone.md) and [work remaining](milestones/to-do.md) distinguish source completion from verified acceptance.

MIT licensed. Dependencies retain their own licenses; [the source-stage notices](THIRD_PARTY_NOTICES.md) identify the final notice work needed before binary distribution.
