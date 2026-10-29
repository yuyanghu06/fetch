# Bind remote callback issuer before exchanging credentials

- Local timestamp / acting role / milestone: 2026-10-29T12:16:09.000006-04:00 / Coder B / Orchestrator API integration / local-working-mvp.
- Requested scope: D81 finish/build/test the selected local MVP with protected actual computer-use installation and separate Codex, excluding packaging/signing/DMG.
- Starting repository state: main at1276516 with prior readiness records and concurrent exclusively owned changes.
- Work performed / changed files: Generic metadata now requires/persists issuer response support and exact published issuer; Manager rejects absent/wrong issuer before code/token fields, saved refresh metadata retains support. Product API retains single iss and rejects duplicates; owned Railway caller updated. Added seventh security fixture.
- Decisions: Existing API route and named-provider behavior preserved; no issuer downgrade/new callback route.
- Checks and evidence: Independent source re-review ACCEPT. Final combined68 fixtures pass; final locked binary build exit0 in0.46s after correction. Tests cover missing/wrong/slash/case issuer and absent/false metadata support; no real OAuth flow.
- Resulting state / acceptance: Reviewed source issue resolved; proof/outcome amendments and live-provider limitations remain pending.
- Remaining work and handoff: Keep actual native/provider/client acceptance, dedicated credentials/environment and pending contract choices unresolved until evidence/user decisions arrive. Save scoped accepted source; do not announce working-local completion or send completion alert.

