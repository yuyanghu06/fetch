# Research remote sign-in and concrete local isolation

- Local timestamp / acting role / milestone: 2026-10-29T12:01:40.000002-04:00 / Researcher / local-working-mvp.
- Requested scope: D81 autonomous local MVP completion and protected verification; exclude packaging/signing/DMG.
- Starting repository state: main at1276516 with prior readiness docs pending; concurrent writers own separate source files.
- Work performed / changed files: Compared current official Rust SDK3.8.0, TypeScript OAuth and MIT mcp-remote designs; checked documented HF/Miro/Vercel/v0 profiles and local isolation tool availability.
- Decisions: Reuse proven OAuth flow/design without changing raw runner. Require public-DNS pinning/no redirects/issuer-resource binding; earlier rmcp<=3.6.0 advisory and metadata provenance matter. No universal provider identity API exists. Vercel client review may externally block Fetch. Fresh profiles do not isolate login Keychain.
- Checks and evidence: Primary sources opened and local facilities inspected read-only. sandbox-exec found; no inspected VM/container tool. No software/account/profile/install/provider or native action. Source links and limitations are in agent report and current run.
- Resulting state / acceptance: Bounded pass integrated into active local MVP work; no full operational acceptance.
- Remaining work and handoff: Coordinated build/independent fixture checks, pending credential choices, protected environment and actual UI installation/test Codex calls. Orchestrator serializes journals; preserve unfinished work.

