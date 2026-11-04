# Record wrapper architecture and service-output console

- Local timestamp / acting role / milestone: 2026-11-04T14:34:35.978695-05:00 / Orchestrator / service-startup-logs
- Requested scope: User says console should just wrap service logs, never replace returned errors with protocol error; explicitly requires remembering wrapper-default architecture across this project.
- Starting repository state: Logging source ready; independent review found sanitizer/detail issues; testing active.
- Work performed / changed files: Recorded durable default in AGENTS.md, canonical decisions/index, PROJECT_CONTEXT.md and NOTES.md. Updated UI record_state to remove synthetic phase/host prose and suppress generic error fallback when actual service output arrives. Removed protocol-error wording from RunError.rpc. Fixed shared sanitizer JSON keys/numeric sensitive values and bounded response-derived redaction retention; SDK writer fixed error-label masking and rejected HTTP status visibility.
- Decisions: Wrap existing implementation/output by default; preserve credential/access boundaries and already explicitly selected product behavior. No broad rewrite/source import/automatic login inferred.
- Checks and evidence: Source inspection plus evaluator findings; independent tests compiling. No new runtime success claimed. Actual Railway research reproduced service-supplied login instructions and -32001tools/list error in isolated fresh HOME, not inferred commands.
- Resulting state / acceptance: Architecture recorded and scoped source corrections implemented; independent testing/review pending.
- Remaining work and handoff: Complete logging fixture/regression/build checks, evaluator acceptance, research evidence and final docs.
