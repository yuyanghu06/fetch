# Explain project install records and client precedence

- Local timestamp / acting role / milestone: 2026-11-12T13:39:23.406923-05:00 / Orchestrator / project-scoped-servers.
- Requested scope: Clarify proposed installation recording and whether Claude Code/Codex project MCP settings override global settings.
- Starting repository state: Scope storage in config.json selected; project connections and implementation still proposed. Unrelated modifications preserved.
- Work performed / changed files: Applied openai-docs skill for current Codex documentation; fetched official MCP/config precedence and Claude MCP pages. Updated project-scoped-servers.md, checkpoint and milestone guides/journal. No code or actual client config changes.
- Decisions: No new user approval inferred from question. Proposed project file stores one named Fetch connection, while Fetch stores each managed server's scope/settings.
- Checks and evidence: Official Claude docs show local>project>user and whole-entry name precedence. Official Codex docs confirm project .codex/config.toml MCP support (trusted only), project URL value above global, CLI above project. Older no-Codex-project-support instruction is outdated as a factual capability claim; no sync tooling change follows. Different names remain available. Read-only direct review; no independent specialist needed for this short factual check.
- Resulting state / acceptance: Explanation source-backed; no install, personal/project client edit, runtime, login or trust operation performed.
- Remaining work and handoff: User review of installation/connection behavior and full API contracts before implementation. Existing Claude certificate issue and other pending acceptance remain unresolved.
