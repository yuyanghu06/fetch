# automatic connector proof and app handoff

- Local timestamp / acting role / milestone: 2026-11-12T15:07:01.986922-05:00 / coder / local-mcp-connector
- Requested scope: Automatic first approval without a separate terminal setup/login command.
- Starting repository state: Existing connector source/fixture ACCEPT and signed build; substantial unrelated dirty work and later inline Connect UI retained.
- Work performed / changed files: Inspected connector/profile implementation, native app cold/warm intent, one-shot API wake and native claim/save integration. Fixed observed concurrent first-use private-directory race with unchanged permission validation. Added focused activation/wake/CLI checks. Unrelated UI and dirty files preserved.
- Decisions: Existing four routes and approved permission/Mac confirmation unchanged. Missing grant may open Fetch. Client timeout can require Reconnect.
- Checks and evidence: Inspected actual source and reports. Root follow-up fixture run in progress; no final result asserted. Earlier46 checks are prior baseline only.
- Resulting state / acceptance: Source integrated; final verification/evaluation/build pending.
- Remaining work and handoff: Finish isolated fixtures, resolve evaluator findings, build-only; user owns actual client/Mac acceptance. No config/trust/service/release changes.
