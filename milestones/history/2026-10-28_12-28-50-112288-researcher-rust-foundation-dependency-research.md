# Research Rust backend dependencies

- Local timestamp / acting role / milestone: 2026-10-28T12:28:50.112288-04:00 / researcher / mcp-dev-foundation
- Requested scope: Read-only maintained licensed primary-source comparisons before dependent implementation.
- Starting repository state: User authorized build/push without release; runtime code absent at pass start.
- Work performed / changed files: Researcher inspected published crate metadata/archives and repositories; root recorded developer-foundation.md. No research-owned source writes.
- Decisions: Axum/Tokio/rustls/rcgen foundation; explicit ring; typed rmcp not canonical due unknown-field loss; strict JSONC flags and decoded duplicate checks.
- Checks and evidence: Exact versions/APIs/licenses/maintenance/known issue status in developer-foundation.md; source compatibility is not live client proof.
- Resulting state / acceptance: Research complete before dependent implementation.
- Remaining work and handoff: Compile the lockfile and independently test fixtures; Keychain/Iced/MCP integration later.
