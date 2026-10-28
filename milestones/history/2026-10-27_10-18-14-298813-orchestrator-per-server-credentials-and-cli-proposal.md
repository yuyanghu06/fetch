# Record per-server encrypted files and proposed placement

- Local timestamp / acting role / milestone: 2026-10-27T10:18:14.298813-04:00 / Orchestrator (serialized by Orchestrator) / mcp-mvp-architecture.
- Requested scope: Investigate proposed GitHub CLI support, record per-server encrypted credential files, and advise whether they belong inside package-install folders.
- Starting repository state: Documentation-only repository; D24/D25 auth ownership/store/readback/service-test required, general auth review underway, no implementation authorized.
- Work performed / changed files: Recorded D26 per-server encrypted credential files and D27 approval of previously shown general auth behavior. Added github-cli-proposal.md with bounded adapter/auth alternatives and a draft installation-only prompt; updated research/storage/API/current instructions/context/notes/index/readiness/guides and milestone records. Checked Apple Application Support guidance.
- Decisions: Encrypted-file-per-server organization confirmed; exact directory/names/format/key handling remain open. Outside-package durable server-state directory is a recommendation awaiting feedback. User GitHub CLI idea does not approve unshown tools, arbitrary commands or storage/refresh exceptions.
- Checks and evidence: Inspected actual docs and dedicated Researcher report; independently browsed official Apple/CLI sources. Final format/link/anchor/JSON/history/template checks and independent evaluation follow serialization. No runtime/source/credential changes or app tests.
- Resulting state / acceptance: Documentation update ready for independent review; package co-location and CLI details not silently selected.
- Remaining work and handoff: Obtain placement and GitHub CLI tool/API/install/fresh-auth/renewal decisions; continue remaining contracts. No package installation, login, credential migration or personal config changes authorized/performed.

Append this entry once under `milestones/history/` with a chronological,
collision-safe filename. Corrections belong in a new entry.
