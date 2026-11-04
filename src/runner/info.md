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
