# Keep the expanded console and service history

- Local timestamp / acting role / milestone: 2026-11-04T14:50:41.924188-05:00 / Coder (recorded by Orchestrator) / continuous-server-console
- Requested scope: Always render the console in expanded rows and retain current-run startup-to-auth output.
- Starting repository state: Independent plan accepted; all prior dirty source and documentation preserved.
- Work performed / changed files: src/runner/mod.rs keeps the prior frozen capture and carries history on replacement; src/runner/startup_diagnostics.rs copies only already sanitized history and prior observed exit facts. src/ui/mod.rs creates passive display state, reads expanded-server status, always renders the shared console and reuses state for Authenticate/manual save. src/ui/authentication.rs adds passive/request-reset behavior, ingests logs before state guards, and rebuilds visible output from the cumulative snapshot.
- Decisions: No automatic authentication from display. Fresh attempt IDs and redaction state remain; old handles are frozen. Existing memory limits now bound current-run history per server. No log files, public API fields, credential-store policy or authentication checks changed.
- Checks and evidence: Orchestrator inspected actual source. Coder compared its four files with a pre-edit baseline and reports whitespace pass; no tests/builds executed by Coder. Tester has seven focused additions and is compiling.
- Resulting state / acceptance: Source implemented; independent tests/build/review pending.
- Remaining work and handoff: Verify unpolled startup through auth, collapse/window restore, passive controls, duplicates/exit, privacy and limits; final build and evaluator decision.
