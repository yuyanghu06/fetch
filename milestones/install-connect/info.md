# Start installed servers and report authentication requirements

Goal: installation leaves usable MCP servers running by default. GET
/api/v1/servers remains the runtime/authentication status source.

Approved scope: after preparation invoke the same backend Restart / Connect
operation; successful credential-free connections with no known login requirement
become authentication.status not_required. Missing/rejected access becomes
required; unrelated failures remain runtime errors. Do not open login, invoke
tools or create authenticated evidence from readiness. Skip ready reused imports,
preserve successful preparation on later connection failure and honor shutdown.
The UI hides auth controls for not_required and does not prompt for unknown alone.

Acceptance: no-auth install ready/not_required and kept running; required login
reported without automatic browser launch; real launch errors preserved; ready
reuse uninterrupted; mixed successes retained; later auth challenges/shutdown
remain correct. Status: source implemented; identity-binding finding fixed and
independently re-reviewed ACCEPT. Evaluator's detached-response finding is also
corrected; Tester/Evaluator final source ACCEPT. Git delivery pending. Source-only
work here adds no builds/tests/apps/providers/MCP or personal-state operations.
