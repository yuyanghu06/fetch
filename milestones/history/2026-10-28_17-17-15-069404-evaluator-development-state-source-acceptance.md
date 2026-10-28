# development state source acceptance

- Local timestamp / acting role / milestone: 2026-10-28_17-17-15-069404 America/New_York / evaluator / development-state.
- Requested scope: keep all development-owned MCP installs/state separate from final-build state; use paths relative to current home; import normal global Codex definitions and connect a fresh Codex client only to Fetch, alongside main Codex.
- Starting repository state: main at 91d491f with uncommitted first-session planning documentation; no agent-run test/native/client/MCP evidence.
- Work performed / evidence: Independently accepted source scope after reviewing startup/root propagation, CLI overrides, private cwd and tests. Documentation review requested accurate wording about source edits and historical compiler results; Orchestrator applied both wording fixes. No execution or runtime acceptance.
- Decisions: debug Fetch-Development/53672; release Fetch/53671; preserve existing overrides. One Fetch may serve both Codex clients with separate grants/shared provider sessions. No old-state migration or personal settings changes.
- Changed files: source and tests owned as above; Orchestrator owns README, guides, architecture/context and serialized milestone journal.
- Checks and evidence: source inspection only; no new compiled/tested result, no installation/login or certificate trust action.
- Resulting state / acceptance: bounded source/documentation acceptance; runtime acceptance still pending.
- Handoff: rebuild deliberately before using new defaults; prepare authoritative single-Fetch test-client config, empty test credential state and verified client TLS trust; then run selected checks when started by user. Preserve untouched personal config and old state.
