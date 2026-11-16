# Selected catalog performance implementation

- Local timestamp / acting role / milestone: 2026-11-12T15:48:24.804563-05:00 / Coder / server-first-tools
- Requested scope: Parallel selected tool reads and eliminate repeated alias persistence.
- Starting repository state: Accepted fixed-discovery plan; unrelated dirty Workspace/Off/UI work preserved.
- Work performed / changed files: Performance coder implemented up to8 concurrent selected catalog acquisitions with futures_util buffered, preserving request order, missing IDs, permissions, partial status, raw definitions and live-read semantics. Runner::aliases batch stages a cloned map, assigns ordered mappings, persists once only if changed, then commits. Single alias/tool_definitions delegate to it. Narrow edits inspected in src/api/runtime.rs::list_tools, src/runner/mod.rs alias methods and src/runner/catalog.rs. Concurrent Off/metadata-admission guards preserved. Scoped whitespace checks pass; no Cargo or runtime tests by coder.
- Decisions: No cache/timeout/auth/schema/route changes; cap8 is internal scheduling.
- Checks and evidence: Source diff inspection and scoped whitespace only; focused independent tests still being written.
- Resulting state / acceptance: Source integrated, runtime acceptance pending.
- Remaining work and handoff: Independent barrier/fidelity/alias durability checks and final review/build.
