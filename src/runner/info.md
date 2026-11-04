## Saved console history — November 4

log_store.rs saves only already sanitized service-console lines and observed exit facts to private console-log.json beside the server config. Runner::persistent lazily reloads it for native reads/startup. Saves are coalesced off startup and flushed on Quit. Current retention keeps the latest 1 MiB / 4096 lines, 64 KiB per line, with explicit omission notices; this supersedes the historical prefix-only and memory-only descriptions below. Terminal bytes and input never enter this store. Registration removal preserves the file. Independent current-feature checks pass; see milestones/server-console-terminal/evidence.md.

## Continuous console history — November 4

New startup/auth captures now inherit only the previous capture’s already sanitized output and observed exit fact. Fresh IDs and redaction values belong to the new attempt; the old handle is frozen. The existing 1 MiB / 4096-line limit bounds the cumulative current-run history for each server, with an explicit notice. Beginning authentication no longer removes earlier logs. Delete and Quit still remove the memory record.

## November 4 service output capture

startup_diagnostics.rs is the common private per-attempt console capture for local/remote startup and Manager OAuth/helpers. Runner keeps latest handles independently of configured servers; each task owns its handle. Retain the beginning up to1MiB/4096lines (64KiBline), with explicit notices. Sanitize nested JSON/keys/text before memory retention; response-derived redactions are temporary. Capture stderr, invalidstdout without relaxing protocol, MCP logs/error fields, declared startup instructions and failed HTTP replies. Successful frames/tool/token payloads are excluded. Borrowed verification probes cannot freeze parent OAuth capture.

# Shared MCP runner

The runner forwards raw MCP definitions, argument objects, results and supported
live messages. It keeps per-server call queues, concurrency/time limits, ownership,
cancellation, restart and unknown-outcome accounting.

Local servers run unmodified over their normal process input/output connection.
ManagedStdio and private provider credential bridge leases have been removed.
The runner neither patches packages nor reads native provider token files.

Online connection requests can call a shared preflight provided by the application.
That asks the common authenticate HTTP route to renew when needed. The runner
never implements OAuth refresh or browser login itself. The raw runner still works
without that application callback in isolated protocol fixtures.

A bounded remote-authentication check compares the same safe setup operation
without private headers (which must be rejected) and with the saved token (which
must succeed). Public setup alone returns a clear unsupported-proof error; this
check does not claim downstream Google/Slack account access or invoke a tool.

All OS processes/services used in tests must be confined to disposable state.
No passing build establishes real provider login, native UI appearance or universal
protocol/server compatibility.

## Explicit local login startup

connect_for_authentication reuses Runner's lifecycle and an already-ready instance.
For a local process only, it forwards selected login URLs and allows600seconds
for initialize/discover; ordinary/remote setup remains30seconds. Manager bounds
the entire attempt and cancels on Quit. local_setup.rs drains bounded output and
forwards recognized HTTPS browser targets without raw diagnostics or credentials.
This does not implement provider OAuth or native elicitation ownership/replies.

## Native startup failure evidence

startup_diagnostics.rs keeps a bounded sanitized stderr tail while a configured
local server starts. Known environment/argument values, common credential/Bearer
assignments and HTTP(S) URLs are removed before retention. The private snapshot
contains generation, lines and natural exit code/signal; it never enters the
general Runner snapshot or saved records. It freezes at ready/failure and resets
for the next attempt. Provisional local connections are tracked before initialize
but cannot serve tools until ready. Disconnect/Quit await actual process reaping;
repeat close calls recognize an already confirmed exit. Recorded helper logs,
remote bodies and general runtime/tool output are outside this report.
