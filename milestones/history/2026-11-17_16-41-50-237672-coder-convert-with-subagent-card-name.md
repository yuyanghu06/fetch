# Accept the source card name in cross-app conversion

- Local timestamp / acting role / milestone: 2026-11-17T16:41:50-05:00 / Coder / Subagent card name
- Requested scope: Fill a definition's missing name from the source card.
- Starting repository state: Conversion built the name only from the source definition.
- Work performed / changed files: src/api/v1/workspace/mod.rs accepts an optional fallback_name for the existing request. src/workspace/mod.rs and agents.rs pass it through to the existing conversion.
- Decisions: Use the fallback for a missing or blank name only when converting between apps. Preserve an explicit name, source text and description validation.
- Checks and evidence: git diff --check passed. Focused API and clipboard fixtures are pending.
- Resulting state / acceptance: Converter source implemented.
- Remaining work and handoff: Verify inputs carried from the actual clipboard conventions end to end.
