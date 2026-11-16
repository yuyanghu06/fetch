# Scoped server operations and membership guards

- Local timestamp / acting role / milestone: 2026-11-12 17-03-55-549191 America/New_York / Backend Coder / project-connections.
- Requested scope: Implement the approved server assignment and current membership rules.
- Starting repository state: Reusable core existed; project routes/auth were being integrated in the shared dirty tree.
- Work performed / changed files: Manager per-server scope read/write guards serialize native reassignment against caller-aware restart/authentication/config/deletion; detached work retains guards. Initial hidden-ID leak fixed, implicit inventory never echoes hidden IDs. Manager helper exposes membership checks for metadata readers.
- Decisions: Preserve existing internal/global wrappers and trusted unbound fixture callers; real OAuth callers carry exact resources. Idle moves preserve power/cwd/logins; no broad migration.
- Checks and evidence: Parent inspected source and independent evaluation. Initial two core/four combined scenarios passed; final extended project scenarios pass0.36s.
- Resulting state / acceptance: Scope mutation/read boundaries implemented; no personal processes or configuration changed.
- Remaining work and handoff: Later final metadata filtering and integration/test/evaluation entries carry final acceptance.

