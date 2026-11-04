# Propose command-to-definition installation input

- Local timestamp / acting role / milestone: 2026-11-03T17:56:44.860492-05:00 / Orchestrator / direct-install-inputs.
- Requested scope: User clarifies users should supply a command which becomes an internal definition to prepare/run; then asks to continue.
- Starting repository state: Config-only install API; prior direct-input scope question; unrelated concurrent UI/startup/documentation edits preserved. Google integration remains pending.
- Work performed / changed files: Read current install validation, supported launch classifier and active scope records. Recorded complete proposed extension in canonical API contract, open questions, notes, milestone guide/snapshot/to-do and architecture guide. No application changes.
- Decisions: Command translation direction is user-requested; exact command/url fields, server_name requirement, direct identity/reuse, quote parsing/non-shell restrictions and response/error clauses are proposed, not approved. Current source imports remain available. No README interpreter, shell execution or credential-write expansion selected.
- Checks and evidence: Source inspection only. Prior source supports literal npx/uvx/local forms and strict package-argument checks; proposal reuses them without pretending all arbitrary commands work. No build/test/parser/provider/helper execution or personal-state changes.
- Resulting state / acceptance: Complete reviewable API amendment ready for user decision; dependent implementation not started.
- Remaining work and handoff: Obtain exact contract decision, then plan/implement/test/evaluate within selected scope and preserve other work.
