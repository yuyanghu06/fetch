# Reuse the source card name in converted subagents

- Local timestamp / acting role / milestone: 2026-11-17T16:41:31-05:00 / Orchestrator / Subagent card name
- Requested scope: Fill a missing name during conversion using the displayed source card name, such as coder.
- Starting repository state: The converter checks the source content for name and description. The Fetch card gets its name from the saved file list and Copy carried only the definition content.
- Work performed / changed files: Read native discovery, conversion, single Copy and Copy all. Assigned the existing conversion route, clipboard and verification areas separately.
- Decisions: Pass along the displayed name as an optional fallback in the existing conversion request. Keep the explicit source name, original source bytes, missing-description handling and other conversion notices.
- Checks and evidence: Reviewed current source and the screenshot. No fixture run yet.
- Resulting state / acceptance: Implementation and focused checks are in progress.
- Remaining work and handoff: Prove that a configured Codex name that differs from the filename comes through in a real disposable bulk conversion.
