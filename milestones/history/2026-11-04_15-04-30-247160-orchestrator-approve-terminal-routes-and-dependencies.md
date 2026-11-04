# Approve terminal API and prepare existing terminal dependencies

- Local timestamp / acting role / milestone: 2026-11-04T15:04:30.247160-05:00 / Orchestrator / server-console-terminal
- Requested scope: User approved exact route/persistence review and asks implementation with system shell.
- Starting repository state: Research/plan complete; pre-existing continuous-server-console work and unrelated notes preserved.
- Work performed / changed files: Updated canonical approval; assigned exclusive UI/vendor, terminal/backend and log-persistence source owners. Added iced_term path dependency and pinned portable-pty0.9.0 in Cargo.toml; cargo fetch updated Cargo.lock and downloaded dependencies without app/provider execution.
- Decisions: Use approved2-method resource/action contract rather than unapproved alternate5route sketch. Same MCP env/privateHOME; service credential ownership; no real login/globalfallback. Vendorverified0.8.0MIT with existing parser/renderer and externalIOseam, no fresh emulator implementation.
- Checks and evidence: cargo fetch succeeded, resolving134additional packages compatible with Rust1.89. portable-pty0.9.0MIT/alacritty_terminal0.25.1Apache2.0 metadata inspected. iced_term0.8archivechecksum verified by source writer; crateVCS e0e90be2c160bd68f1bf544a51bf772d75b98ffc. No compile/tests/nativePTY/providerstate actions yet.
- Resulting state / acceptance: Approval recorded, source/dependency preparation underway; tests/build/review pending.
- Remaining work and handoff: Finish3exclusive sources, independent API/PTY/persistence/UI checks and evaluator acceptance; no app launch/current user services disturbed.
