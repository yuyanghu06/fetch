# Fix login selection independently of verification profiles

- Local timestamp / acting role / milestone: 2026-10-30T17:17:32.765976-04:00 / Coder / generic-authentication.
- Requested scope: Complete D82 generic imported inputs/trusted request descriptions without weakening stored-credential proof or remote OAuth.
- Starting repository state: main36d0f47 plus scoped source/docs/tests; preliminary107 fixtures passed but evaluator withheld acceptance for login-selection regressions.
- Work performed / changed files: Added trusted login-mode metadata with standard remote OAuth default; known GitHub/Tavily manual and Linear compatibility remain explicit data, Railway lifecycle stays intact. Shared Manager auth_provider/provider_for_profile drives production and fixture classification. has_stored_authentication now only considers actual descriptor-covered refs/OAuth. Manual-header-only remote success does not invent or require OAuth.
- Decisions: No new public descriptor URL/API schema, no saved-record migration, no guessed bearer treatment for ordinary inputs and no automatic unknown-provider login. Preserve D25/D31/native authority/per-server ownership.
- Checks and evidence: Evaluator source findings corrected; three new independent regression cases added. Request preflight/JSON overlap and original D25/D31 gates remain.
- Resulting state / acceptance: Final independent targeted35/35, combined110/110 and native build passed; review to confirm final docs.
- Remaining work and handoff: Final independent evaluation and delivery documentation/commit/push. Broader provider descriptions/wrapper setup, deletion/non-auth outcomes and actual protected native/client/restart proof remain unfinished.
