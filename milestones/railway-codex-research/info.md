# Railway startup and Codex MCP code research

Goal: run actual unmodified Railway MCP startup in a fresh OS-confined no-credential environment, and determine what Codex’s public MCP launch/auth/refresh code could contribute to Fetch.

Scope: task-only disposable packages/runtime/state, verified outside/sibling/network canaries before third-party execution, no personal credentials or client settings, no automatic separate login command, no account tool actions. Research current official Codex repo maintenance/license/dependencies and compare against Fetch’s own raw runner plus pinned official Rust auth library. No code copying or auth redesign selected.

Completion: actual expected/observed startup result with exact versions/protocol request, honest auth/network limits, owned process/package/cache cleanup, primary-source links and practical reuse assessment.

Status: complete for bounded no-credential startup/reuse research. Actual initialize succeeds; tools/list returns -32001notloggedin. Verified confinement and cleanup. [Report](../../docs/architecture/railway-codex-startup-research.md). No actual account login or code copying.
