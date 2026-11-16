# Connector checks

`checks.rs` is compiled only into the library's test build through the connector
module. It exercises the real stdio bridge, a task-owned HTTPS MCP gateway and a
synthetic upstream peer. Credentials stay in a memory backend and distinct fixture
authorities represent separately approved clients. No production app, Keychain,
personal client configuration, system trust or real provider is used.

The checks cover raw catalog/call/error payloads, concurrent profiles and sessions,
progress and interactive replies, cancellation, catalog notifications, connector
EOF, certificate mismatch and reconnect after a disposable backend restart.

These are protocol fixtures, not actual Claude Code/Codex or native approval
acceptance. Real-client checks require separately verified process/OS confinement.

`client_smoke.rs` is an opt-in executable test target. Without
`FETCH_CONNECTOR_CLIENT_SMOKE=1`, it skips real-client execution. With that switch,
it copies public installed client binaries and the fixture executable into a
private temporary root. A deny-default macOS process sandbox must first allow the
owned file/backend canaries and deny outside files, an unrelated disposable
listener and Keychain service lookup. Only then does it start account-free Codex
app-server discovery and Claude bare streaming-control initialization. It sends
no model turn and has fixed timeouts and owned-process cleanup.

Its explicit `--fixture-stdio` subprocess calls the production connector library
with a synthetic approved memory grant. This checks real client transport
compatibility without adding a credential bypass to the product CLI. Production
CLI/Keychain/native approval acceptance remains separate.

The final Claude bare-mode probe may receive the explicit inert string
`disposable-fixture-not-a-real-key` for its local auth-presence check. It has no
real provider credential, sends no model turn, and remains unable to reach any
public/provider network address. This does not prove provider authentication.

The automatic approval fixtures compose the real connection-request OAuth routes
with the HTTPS gateway. A fake key provider protects only disposable application
records, and fake native confirmation represents the human decision. The app-side
profile store claims the matching request and saves to the memory credential
backend. These checks prove request coalescing, no access before approval,
completion after a waiting future is aborted, request/profile/proof separation,
and pending-proof cleanup without deleting an existing grant. Local proof expiry
is simulated by advancing its disposable saved timestamp; native OS approval and
real-client startup remain manual acceptance, not claims from these fixtures.

## Server-first connector flow — November 12

The existing raw forwarding checks now use fixed native Fetch discovery, selected
server definitions, and the generic call tool. Upstream raw argument/result/error,
progress, cancellation, interaction, reconnect and concurrent-session assertions
remain. A single modern2026-07-28 stdio-to-verified-HTTPS flow covers the protocol
reported in the user's failed Claude attempt. Its public fixture inventory is
explicit, and all saved client grants use the memory backend. No real Claude or
Codex process is launched. Root coordinates final Cargo runs and evidence.

Two focused diagnostic checks reproduce the user-reported connection closure:
an unsupported idless roots-change notification must retain its actual backend
HTTP/code/message rather than claim another connection replied. A genuinely
foreign non-null response ID must still be rejected. The same in-memory check
keeps null-error details and large numbers while redacting fixture credentials.
These do not add roots forwarding or acknowledge unsupported notifications.

The modern subscription regression uses the maintained rmcp3.8.0 client over a
DuplexStream connected to the production connector and disposable HTTPS gateway.
It covers acknowledged subscription IDs, change delivery and cancellation while
ordinary tool calls stay usable. Two raw connectors sharing a saved fixture
profile also reuse client request ID7 to prove independent subscription ownership.
A focused metadata case checks only reserved envelope references are translated;
matching names inside arbitrary nested user data stay unchanged. This is a real
library client fixture, not an installed Claude/Codex or personal-client run.
