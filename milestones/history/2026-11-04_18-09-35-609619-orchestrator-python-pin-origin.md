# Explain origin of universal Python pin

- Local timestamp / acting role / milestone: 2026-11-04T18:09:35.609619-05:00 / Orchestrator / install diagnostics.
- Requested scope: Explain why Fetch forces a Python version and what other harnesses do.
- Starting repository state: Existing changes preserved; Python per-server environment research already recorded.
- Work performed / changed files: Read D05/D06, installer commit history and original October28 installer pass. Updated research.md, snapshot/to-do and this history only.
- Decisions: Distinguish approved private tooling/compatible runtime sharing from implementation-selected universal source pin. Recommend removing universal-version restriction; no user selection inferred.
- Checks and evidence: D06 explicitly leaves exact versions and incompatible runtime handling unselected. Original installer pass calls runtime versions internal source pins; git history identifies fc9b1ae as initial installer. Prior official fetched Codex/Claude Code and uv references support command/environment responsibility comparison. No runtime tests, app/MCP launch, installation or configuration edits.
- Resulting state / acceptance: Pin is not an MCP requirement and is not required to keep Fetch tooling private. No fabricated historical rationale for exact 3.13.18 selection.
- Remaining work and handoff: Version/environment policy remains proposed; implementation unchanged.
