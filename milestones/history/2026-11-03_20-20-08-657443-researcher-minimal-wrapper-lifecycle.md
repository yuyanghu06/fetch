# Research server-owned behavior and minimal wrapper lifecycle

- Local timestamp / acting role / milestone: 2026-11-03T20:20:08.657443-05:00 / Researcher report serialized by Orchestrator / native-server-storage.
- Requested scope: full lifecycle; maximum generality/minimal Fetch code; investigate server-owned behavior and absolutely necessary Fetch work first.
- Starting repository state: paused incomplete refactor with Keychain and same-authenticate-route selections; manual credential/folder/native setup details open.
- Work performed / changed files: read-only code/official MCP/Rust SDK/mcp-remote source research; update architecture lifecycle/ownership/recommendations, context/notes and journal.
- Decisions: preference for wrapper/reuse confirmed; official OAuth-only SDK integration is recommended, not accepted. No new per-server code.
- Checks and evidence: official Rust auth/store API and automatic refresh; mcp-remote native token files/extra process/retry/message behavior; local auth outside HTTP MCP standard; no SDK installed or live behavior tested.
- Resulting state / acceptance: lifecycle plan with explicit server/library/Fetch ownership and necessity vs product choices; no implementation acceptance.
- Remaining work and handoff: user reviews lifecycle/SDK recommendation; settle exact native setup/folder/manual key and background request authority/browser behavior before coding; later isolated tests/evaluation.
