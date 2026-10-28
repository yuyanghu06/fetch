# Correct gateway syntax and compile the native Fetch binary

- Local timestamp / acting role / milestone: 2026-10-28T15:00:36.928523-04:00 / Coder fixes inspected and compiled by Orchestrator / mcp-mvp-compilation.
- Requested scope: D79 keep fixing compiler errors until the binary builds; then commit/push, without tests, app launch or MCP connections.
- Starting repository state: First native build exit101 at src/mcp/mod.rs:151; dependencies resolved into Cargo.lock.
- Work performed / changed files: Separate the server/discover literal from its Rust if match guard in src/mcp/mod.rs. Remove unused Json/StreamExt imports there and HeaderMap in src/api/runtime.rs. Repeat binary compilation with the resolved lockfile.
- Decisions: Minimal compiler fix, unchanged MCP dispatch behavior and API/storage boundaries. Keep selected classic-Keychain APIs; their deprecation warnings do not justify a storage redesign.
- Checks and evidence: `cargo build --locked --bin fetch --message-format short`, exit0, Finished dev profile in12.61s. Generated target/debug/fetch; evidence log target/fetch-build-pass2.log (ignored build output). Four native deprecation warnings and one unread protocol-error field warning remain. No test targets or tests, lints/formatters/validation scripts, executable/app/service launch, native/Keychain/trust actions, MCP/provider/client connections or packaging.
- Resulting state / acceptance: Native development binary compilation succeeds; runtime/visual/provider/client behavior remains unverified.
- Remaining work and handoff: Bounded independent source/build-evidence review, synchronize current records, commit and push compiler fixes/lockfile. Stop before any runtime testing.
