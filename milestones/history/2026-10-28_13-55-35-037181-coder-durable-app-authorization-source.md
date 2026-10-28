# Implement durable app authorization and native secret authority source

- Local timestamp / acting role / milestone: 2026-10-28T13:55:35.037181-04:00 / Coder, inspected by Orchestrator / mcp-mvp-implementation.
- Requested scope: D71/D72 authorization, Keychain-backed encryption and lasting TLS source; no testing.
- Starting repository state: d8c917c planning checkpoint; concurrent manager and runner source work remains separately owned.
- Work performed / changed files: authority.rs, oauth/, native_auth/, identity/, credentials/, shared strict JSON framing, Cargo declarations and Objective-C build integration.
- Decisions: Separate application versus provider encrypted records; no plaintext key fallback; native session plus fresh mutation-bound confirmation for manual secrets. D78 separately defers custom GitHub CLI tools.
- Checks and evidence: Source read and specialist report only. No compilation, tests, lint, formatter, validation scripts, Keychain/OS interactions or live requests run. Cargo.lock is intentionally not refreshed; dependency resolution belongs to user-started compilation.
- Resulting state / acceptance: Feature source written and registered; runtime and interoperability acceptance pending. No claim that source compiles.
- Remaining work and handoff: Integrate authority with shared management/native MCP/application launch; user later verifies OS prompts, storage and OAuth behavior.
