# automatic approval request changes

- Local timestamp / acting role / milestone: 2026-11-12T15:11:35.625075-05:00 / evaluator / local-mcp-connector
- Requested scope: Automatic Fetch approval popup from ordinary missing-profile client startup.
- Starting repository state: Follow-up source integrated, preserving unrelated dirty work and inline Connect UI.
- Work performed / changed files: Independent review found denial waiting until expiry, repeated reopening after manual window closure, and late Mac denial after window session loss. Requested exact pending cleanup and one-shot wake; no other auth change.
- Decisions: Reuse existing four routes, separate profile credentials and native Mac approval. No fake initialization; timeout can require client Reconnect.
- Checks and evidence: Scoped git diff --check passes. Logs under /tmp/fetch-connector-auto-final.log, /tmp/fetch-connector-ui-auto-final.log, /tmp/fetch-auto-activation-final.log and /tmp/fetch-auto-cli.log. Signed build-only now running; no result yet.
- Resulting state / acceptance: Source/fixtures accepted; signed build pending.
- Remaining work and handoff: Finish build-only and document outcome; user tests actual clients/native approval. No personal config/trust edits, app relaunch, migration or release.
