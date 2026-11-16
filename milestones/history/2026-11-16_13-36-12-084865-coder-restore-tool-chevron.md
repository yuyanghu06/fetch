# Restore visible tool disclosure arrows

- Local timestamp / acting role / milestone: 2026-11-16T13:36:12.084865-05:00 / Coder, integrated by Orchestrator / workspace-session-error
- Requested scope: fix missing state indicator found during requested tool-name code-box visual review.
- Starting repository state:12follow-up checks pass; snapshots show new code_disclosure arrow collapsed by Shrink layout.
- Work performed / changed files: one scoped components.rs change gives code_disclosure chevron explicit18px dimensions through existing icon_at_size.
- Decisions: preserve all other rendering/actions; no shared server disclosure changes.
- Checks and evidence: Coder diff check passes; root confirms one scoped icon call; affected tool fixture/captures and final signed build running.
- Resulting state / acceptance: source stable, final verification pending.
- Remaining work and handoff: inspect visible collapsed/expanded arrows and final Evaluator decision. No app/personal-state actions.
