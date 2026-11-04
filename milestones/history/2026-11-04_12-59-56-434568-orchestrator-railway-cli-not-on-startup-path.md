# Railway private CLI cannot be found at startup

- Local timestamp / acting role / milestone: 2026-11-04T12:59:56.434568-05:00 / Orchestrator / startup-diagnostics
- Requested scope: Diagnose user's Railway Authenticate failure; Google investigation no longer active.
- Starting repository state: Existing uncommitted source/documentation preserved.
- Work performed / changed files: Read official Railway docs, registry metadata and wrapper archive in memory; inspected installed wrapper/version and CLI file presence, shared definition metadata and Fetch installer/startup source. Documentation updates only.
- Decisions: Recommend including the already installed private CLI in the process search path; no per-server implementation approved or performed.
- Checks and evidence: User log matches wrapper ENOENT branch. Installed version 0.1.12 and private CLI files present. Installer prepares CLI but startup env_clear/default PATH omits its directory. System railway exists outside that PATH. Read-only checks only; no MCP/account execution.
- Resulting state / acceptance: Immediate startup cause identified. No evidence account credentials were reached/rejected.
- Remaining work and handoff: Discuss selected scoped repair, implement/test after direction; real sign-in remains untested.
