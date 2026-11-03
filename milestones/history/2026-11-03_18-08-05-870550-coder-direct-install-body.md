# Implement direct command and URL install bodies

- Local timestamp / acting role / milestone: 2026-11-03T18:08:05.870550-05:00 / Coder, inspected by Orchestrator / direct-install-inputs.
- Requested scope: Add approved command/url alternatives to existing POST body; no UI.
- Starting repository state: Approved complete amendment; existing config-only installation; unrelated UI/startup and documentation edits preserved; Google pending.
- Work performed / changed files: Cargo.toml/lock adds already-locked exact shlex2.0.1; discovery/mod.rs extracts shared capture_definition without changing import logic; new installation/direct.rs parses restricted quoted commands/URLs and hashes normalized name+definition; installation/mod.rs supports input alternatives, reuse and sanitized progress; storage/mod.rs accepts direct association variant; mcp/management.rs exposes matching schema. Orchestrator inspected complete source/diff and refreshed guides/journal.
- Decisions: Save only direct identity digest with source_name, separate from original source-path variant and current edited settings. Direct inputs captured once; unsupported syntax/URLs fail items; malformed shapes/exact duplicates400 before job. Reject command shell operations/comments/expansions and credential-bearing bridge headers; never invoke a shell. Existing sequential preparation/Connect, errors, auth and source capture/reuse remain.
- Checks and evidence: Coder/root source inspection only. Lock dependency entry updated without resolution/runtime commands. Coder ran no builds/tests/install/providers. Tester's focused fixtures and independent evaluation are in progress.
- Resulting state / acceptance: Backend source integrated for verification; no UI, manager, package/environment or personal-state changes by this pass.
- Remaining work and handoff: Independently test/evaluate, compile native binary, resolve material findings and deliver scoped code/docs. Preserve unrelated changes and Google deferral.
