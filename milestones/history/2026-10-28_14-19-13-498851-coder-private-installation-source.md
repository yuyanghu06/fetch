# Implement private pinned installation source

- Local timestamp / acting role / milestone: 2026-10-28T14:19:13.498851-04:00 / coder / mcp-mvp-implementation.
- Requested scope: Selected private preparation and server-management source, with all testing prohibited by D77.
- Starting repository state: bf5dd73 authority source pushed; concurrent runner/manager/UI source active.
- Work performed / changed files: Installation/discovery/storage source now captures non-secret source settings, preserves source associations, serializes jobs, prepares private Node/uv/Python and pinned package environments, and retains same-run progress. Resolved versions come from installed metadata; remote first connection remains pending. Source-only review accepts installer integration after corrections. No package/runtime command or download was executed.
- Decisions: Runtime versions are internal source pins, no global runtime dependency or original secret migration.
- Checks and evidence: Source reading only. No compile, tests, formatter/linter, validation scripts, services, OS actions, package/runtime execution or managed connection.
- Resulting state / acceptance: Source reasoning only, not verified MVP acceptance.
- Remaining work and handoff: Integrate shared runner/manager/native interface; user later controls all runtime verification.
