# Explain Google authentication gap

- Local timestamp / acting role / milestone: 2026-11-03T21:49:55.213550-05:00 / Orchestrator / native-server-storage.
- Requested scope: Explain screenshot errors and why Authenticate is not starting Google sign-in.
- Starting repository state: main135873a; prior sync journal edits and concurrent UI edits present.
- Work performed / changed files: Read manager authenticate/native-state, installer/storage and UI code plus current architecture records; updated snapshot/to-do and this entry only.
- Decisions: No new per-server behavior, public contract, live-account inspection or source changes. Preserve concurrent UI work.
- Checks and evidence: src/manager/mod.rs authenticate_inner requires native_setup_for on nonremote records; src/manager/native_state.rs requires recorded command/folder and emits exact screenshot folder message; src/installation/info.md confirms ordinary imports lack metadata authoring/detection; src/ui/mod.rs authentication_view unconditionally offers imported references and folder fallback in applicable states. Git history shows3008546 removed previous Google/helper bridge modules. Read-only inspection, no tests/build/runtime or provider operations.
- Resulting state / acceptance: Cause identified in current source; no fix or actual Google OAuth result claimed. The saved instance/config was not inspected, so this does not establish its exact kind or fields.
- Remaining work and handoff: Agree how to record native command/folder metadata and invoke supported server-owned Google login; retain architecture review requirement and protected live testing. AGENT_WORKFLOW.md is absent at repository root; this bounded explanation used direct source tracing without delegated coding.
