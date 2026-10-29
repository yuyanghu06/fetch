# Add bound generic remote sign-in source

- Local timestamp / acting role / milestone: 2026-10-29T12:16:09.000002-04:00 / Coder B / local-working-mvp.
- Requested scope: D81 finish/build/test the selected local MVP with protected actual computer-use installation and separate Codex, excluding packaging/signing/DMG.
- Starting repository state: main at1276516 with prior readiness records and concurrent exclusively owned changes.
- Work performed / changed files: Added manager/remote_auth.rs and shared-manager flow integration; existing HTTPS callback, encrypted registration/grant/refresh provenance, pinned public DNS, no proxy/redirects; updated guide and Railway shared rendering. No dependencies changed.
- Decisions: Follow researched SDK designs without replacing raw runner. Generic verified-auth success and credential outcome changes stay pending. Local wrapper setup and provider client-approval restrictions remain explicit.
- Checks and evidence: Initial6 pure fixtures and first integrated locked build passed; later source review requested issuer mix-up correction. No real auth/provider/network/package action.
- Resulting state / acceptance: Generic OAuth source is implemented but not complete authenticated provider support or live acceptance.
- Remaining work and handoff: Keep actual native/provider/client acceptance, dedicated credentials/environment and pending contract choices unresolved until evidence/user decisions arrive. Save scoped accepted source; do not announce working-local completion or send completion alert.

