# Local MCP connector

Goal: let coding clients launch a bundled stdio command that verifies the existing
Fetch installation's HTTPS certificate itself.

Status November12: implementation/signed build delivered, independent source and
fixture ACCEPT.54 focused checks pass. User explicitly defers actual native Mac
approval/Keychain and successful Claude/Codex MCP acceptance to manual testing.
Do not mark those complete or infer them from fixture success.

See [current design/source](../../docs/architecture/local-mcp-connector.md) and
[evidence/manual handoff](evidence.md). `evidence/` contains the inspected actual
widget popup rendering, not a live app screenshot.

`fetch connect --client ...` now opens/focuses Fetch and requests approval automatically
when its profile has no saved grant. Separate setup/login is optional. Pending/approved proof
ownership survives backgrounding, credentials stay separate, and closing a
connector never stops Fetch/shared servers. Notification token expiry can require
client reconnect; no uncertain call replay is allowed.

No personal client/trust/credential edits, migration, project scope, release,
packaging or agent-run native relaunch are part of this work. Complete the
milestone only after remaining manual acceptance is observed/recorded.

Automatic first-approval follow-up passes54 focused checks, independent review ACCEPT and signed build/strict verification. Native app completion persists beyond coding-client startup timeout. See current architecture record.

First user-manual Claude check now reaches connected but fails tools fetch/connection closes. Actual client acceptance remains unresolved.54 prior focused checks cover fixtures, not this newly observed live failure. See evidence.md and latest journal.

Parallel collection versus server-first discovery researched. No catalog/API default changed; invocation and client approval behavior need review. See canonical research.md November12 follow-up.
