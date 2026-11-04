# Complete selected Codex discovery correction

- Local timestamp / acting role / milestone: 2026-11-04T17:43:20.236548-05:00 / Orchestrator / selected-discovery-scope
- Requested scope: Selecting .codex must discover only .codex/config.toml, excluding neighboring Claude/OpenCode servers.
- Starting repository state: Large existing dirty tree preserved; original bug independently reproduced, focused source and test changes accepted by independent evaluation.
- Work performed / changed files: Integrated Planner/Coder/Tester/Evaluator reports. Inspected actual source/test changes, preserved unrelated MCP descriptions and other concurrent work. Synchronized current API contract, architecture decision/index/guides, NOTES.md, PROJECT_CONTEXT.md, source/test guides, existing automatic-discovery scope and milestone journal. No commit/push requested or performed.
- Decisions: Keep change limited to the explicitly selected .codex directory. No added app-specific folders or new route; automatic/home/single-file reads and installation behavior remain unchanged.
- Checks and evidence: Original selected-folder HTTP test failed 3 versus 1 before source fix. Final ten focused HTTP discovery tests and locked native binary build pass; source/test git diff --check passes. Independent Evaluator ACCEPT with no findings.
- Resulting state / acceptance: Scoped source/fixture/build milestone complete. Current records state the latest selected behavior and retain older evidence as historical.
- Remaining work and handoff: Explicitly Quit Fetch, run ./scripts/run-development.sh, then refresh discovery to use the new code. Native window behavior not observed in this pass. No app launch, signing, real MCP execution, source configuration edits or personal credential use. Other active milestones and acceptance limits are preserved.
