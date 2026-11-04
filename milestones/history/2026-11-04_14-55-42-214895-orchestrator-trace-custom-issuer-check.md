# Trace the custom issuer requirement

- Local timestamp / acting role / milestone: 2026-11-04T14:55:42.214895-05:00 / Orchestrator / explanation only.
- Requested scope: Explain why Fetch has a custom authentication check contrary to the wrapper architecture.
- Starting repository state: Pre-existing code/documentation work retained; no implementation requested by this question.
- Work performed / changed files: Read current source, commit 3008546 and architecture/history references; added snapshot/to-do records and this history entry.
- Decisions: No authentication policy change. The historical rationale is protection against accepting a sign-in response from the wrong issuer. That rationale does not by itself establish that requiring this exact advertised feature for all upstream services was the right implementation or a separately selected user requirement.
- Checks and evidence: October29 evaluator history 2026-10-29_12-16-09-000005-evaluator-oauth-issuer-mixup-finding.md explicitly requested published issuer-response support; following coder entry records enforcing it. November3 commit 3008546 retained the guard while adopting rmcp, with a preserve-existing-protection comment. docs/architecture/native-server-storage.md records the limitation. Current sdk.rs rejects before beginning the library login session. No independent review or tests in this explanatory pass.
- Resulting state / acceptance: Origin and later retention established. Broad security goals and an implementation/reviewer-imposed compatibility requirement must not be conflated. Earlier wording about login restriction should not imply simply deleting one condition guarantees working Vercel authentication.
- Remaining work and handoff: Recommend replacing custom provider-login decisions with the chosen library's supported flow and making Fetch-owned failures visibly distinct; exact changes need bounded scope, not silently applied from an explanation request. Existing work remains unchanged.
