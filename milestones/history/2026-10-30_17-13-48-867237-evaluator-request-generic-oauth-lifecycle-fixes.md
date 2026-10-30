# Request corrections to generic login classification

- Local timestamp / acting role / milestone: 2026-10-30T17:13:48.867237-04:00 / Evaluator / generic-authentication.
- Requested scope: Implement D82 shared imported inputs and reusable trusted-description verification while retaining D25/D31 and existing remote OAuth.
- Starting repository state: main36d0f47 plus preserved research/plan/docs and exclusively owned concurrent source/tests.
- Work performed / changed files: Independent source review found that every new built-in profile except Linear became Manual, disabling generic remote OAuth for newly described HTTP services. Also has_values fell back to all imported refs without a descriptor, so an ordinary saved workspace header caused failed verification instead of immediate generic login. Tester additionally noted remote manual-header success must not require an absent OAuth grant.
- Decisions: No public verification route/URL schema or saved-record migration; no readiness substitute or automatic unknown-provider proof. Every submitted auth value must participate in its current saved proof.
- Checks and evidence: Source paths inspected; no live browser/provider requests. Existing107 passing fixtures did not cover these production decision paths. Writer and Tester received concrete corrections/regression requirements.
- Resulting state / acceptance: REQUEST CHANGES. Separate trusted verification description from login mode, use only actual described authentication refs/OAuth for login evidence, and preserve manual-header-only authentication.
- Remaining work and handoff: Fix lifecycle findings, add production-path regression fixtures, rerun affected/full fixtures and native binary build, obtain final independent acceptance, refresh docs and deliver. Parent local working MVP remains incomplete.
