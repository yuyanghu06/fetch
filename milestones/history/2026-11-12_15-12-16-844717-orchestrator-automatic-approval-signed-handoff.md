# Automatic approval signed handoff

- Local timestamp / acting role / milestone: 2026-11-12T15:12:16.844717-05:00 / Orchestrator / local-mcp-connector
- Requested scope: Ordinary missing-profile startup opens Fetch approval; no separate setup/login command.
- Starting repository state: Integrated follow-up;54 focused tests pass and independent Evaluator ACCEPT. Unrelated dirty work preserved.
- Work performed / changed files: Final signed build-only and current architecture/context/journal/evidence synchronization.
- Decisions: Native app retains/claims exact approved pending proof after client timeout. No new routes, scope, grant-before-approval, fake initialization or browser trust change.
- Checks and evidence: ./scripts/run-development.sh --build-only passes18.16s; selected Apple Development identity, separate com.fetch.local.development executable, strict signature verification. git diff --check passes. Prior warnings remain.54 focused checks recorded in evidence.md.
- Resulting state / acceptance: Source/fixtures/signed build accepted. Fetch was not relaunched.
- Remaining work and handoff: User manually checks actual Claude/Codex, Mac approval and Keychain after explicit Quit/reopen. One-time stdio client configuration remains; timed-out startup can require Reconnect. No personal settings/credential/trust/service changes, migration, project scope or release.
