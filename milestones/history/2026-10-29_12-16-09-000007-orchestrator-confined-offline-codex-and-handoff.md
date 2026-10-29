# Prepare separate Codex and native test handoff

- Local timestamp / acting role / milestone: 2026-10-29T12:16:09.000007-04:00 / Orchestrator / local-working-mvp.
- Requested scope: D81 finish/build/test the selected local MVP with protected actual computer-use installation and separate Codex, excluding packaging/signing/DMG.
- Starting repository state: main at1276516 with prior readiness records and concurrent exclusively owned changes.
- Work performed / changed files: Created private external local-mvp-proof files and copied Codex executable only. Added fresh file-based auth/config/state locations, offline policy/canaries, generated schema; initialized actual copied Codex app-server and queried MCP status. Opened Mac Users&Groups AddUser through CUA.
- Decisions: Do not copy everyday auth/config/plugins/hooks or alter installed binary/client state. Full native test setup requires real owner/admin authentication. Do not claim folder/profile confinement proves native Keychain isolation.
- Checks and evidence: Codex0.164.0 initializes with0 MCP servers, owned process stopped. Initial deny-default policy aborted both canaries; revised offline process policy permits disposable allowed canary and refuses denied canary. No real MCP/provider connections. SystemSettings now at Password/Unlock; no account/security changes. Clippy alltargets exit0 in19.82s with72 warnings; git diff --check passes.
- Resulting state / acceptance: Scoped source/fixture delivery independently ACCEPTed; actual UI install/tool call/native/provider proof and2 credential choices pending. No completion alert.
- Remaining work and handoff: Keep actual native/provider/client acceptance, dedicated credentials/environment and pending contract choices unresolved until evidence/user decisions arrive. Save scoped accepted source; do not announce working-local completion or send completion alert.

