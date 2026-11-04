# Review single-connector source readiness

- Local timestamp / acting role / milestone: November 3, 2026, 14:21:37 EST / Researcher, inspected and serialized by Orchestrator / selected MCP MVP readiness.
- Requested scope: Summarize current functionality and missing work before using Fetch as the sole MCP connection for Codex, OpenCode and Claude Code.
- Starting repository state: Clean main at 1276516; local origin/main reference matched. No remote query.
- Work performed / changed files: Read discovery, installation, manager, runner, gateway, OAuth and recorded gap/inventory files. Specialist owned no files. Orchestrator updated snapshot/to-do and appended this record.
- Decisions: No scope or product-policy changes. Preserve fresh authentication instead of credential migration and the existing runtime restrictions.
- Checks and evidence: Read-only source review: src/manager/mod.rs:213-225 writes credentials before its restrictive verification branch; :266-295 and :510-512 show provider-specific authentication; src/discovery/mod.rs:323 and :574 show restrictive launcher/endpoint support. src/mcp, src/oauth and src/runner contain shared gateway, client grants, three transports, queues and selected native relay source. src/runner/transport.rs refuses reverse requests without proven ownership. Existing historical tests/build evidence unchanged.
- Resulting state / acceptance: Four canonical backend gaps confirmed. Core connector source exists; arbitrary inventory compatibility and operational completion are unproved. Unknown authentication does not necessarily prevent no-auth connections. Railway local children require explicit Restart after background refresh.
- Remaining work and handoff: Finish per-inventory import/auth/credential coverage and required UI extraction; prepare authorized isolation, exercise real clients/providers and resolve findings. No builds, tests, app/provider/MCP calls, personal-state writes or release performed.
