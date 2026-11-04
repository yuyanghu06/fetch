# Research startup logs and returned errors

- Local timestamp / acting role / milestone: 2026-11-04T14:22:39.006640-05:00 / Researcher (recorded by Orchestrator) / service-startup-logs
- Requested scope: User explicitly authorizes rejected-response/service-startup log feature after earlier explanation-only pause.
- Starting repository state: Shared helper-path repair accepted; generic Vercel and Railway failures remain unexplained. Existing dirty work preserved.
- Work performed / changed files: Read pinned rmcp3.8 source, MCP transport/error/logging specs and maintained MIT VS Code implementation. No code or runtime changes.
- Decisions: Shared memory-only native console; capture stderr/helper streams, nonprotocol stdout as violation without relaxing protocol, MCP errors/log notifications and unsuccessful remote auth responses/stages. Retain attempt beginning with explicit finite-limit marker. Never dump successful protocol/token payloads or global SDK Debug tracing.
- Checks and evidence: rmcp RegistrationFailed embeds response body; token error strings/URLs can be sensitive and global SDK debug logs token result. Current Fetch retains32line16KiB tail and discards helper bodies/returned errors. MCP stderr/stdout and notifications/message semantics confirmed. Source references recorded in docs/architecture/research.md.
- Resulting state / acceptance: Research complete; dependent plan/source work may proceed within explicitly approved logging scope.
- Remaining work and handoff: Shared capture lifecycle, sanitizer, transport/helper/OAuth wiring, independent tests and review. No new login method, provider tests or personal state changes.
