# Plan public client identity wiring

- Local timestamp / acting role / milestone: 2026-11-04T15:22:44.343477-05:00 / Planner, serialized by Orchestrator / client-identity.
- Requested scope: Minimal rmcp client metadata URL wiring after user approval; hosting not yet selected. User subsequently requires focused real-world feature tests, recorded in AGENTS.md.
- Starting repository state: No configured hosted identity document; SDK supports it already. Existing source and concurrent features remain preserved.
- Work performed / changed files: Planner read current source and pinned SDK; no source edits. Parent inspected report and SDK source and records this pass.
- Decisions: Prefer one fixed Fetch-owned public URL once chosen; no need introduce new environment settings/Manager configuration while URL undecided. Static JSON lists exact own client_id, Fetch name, current two callbacks, native application type and no client secret. SDK selects advertised support; do not write custom negotiation/fetching code.
- Checks and evidence: Pinned rmcp3.8 auth.rs3846-3897 selects public metadata identity when supported and keeps DCR otherwise. sdk.rs begin is the insertion point. No tests, live-provider/network/account use or deployment performed by Planner.
- Resulting state / acceptance: Bounded plan ready; actual public URL remains unresolved. Proposed test inventory is narrowed by latest user instruction: prove realistic metadata-identity sign-in and existing ordinary registration using current fakes, without speculative case matrices or broad new infrastructure.
- Remaining work and handoff: User asks what code is needed; show static JSON plus one SDK option. After hostname selection, implement only that scope, then focused test/build/evaluation and documented hosting outcome. No placeholder production identity or invented domain.
