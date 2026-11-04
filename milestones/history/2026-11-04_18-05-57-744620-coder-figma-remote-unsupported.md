# Mark Figma Remote unsupported

- Local timestamp / acting role / milestone: 2026-11-04T18:05:57.744620-05:00 / Coder / figma-remote-unsupported
- Requested scope: User-selected import/install-screen unsupported status with reason.
- Starting repository state: Accepted bounded plan; extensive pre-existing edits preserved.
- Work performed / changed files: Coder changed src/discovery/mod.rs (narrow matcher/reason and two classify call sites) and src/ui/mod.rs (existing row message). Root inspected actual helper, call sites and rendering, serialized docs.
- Decisions: Keep supported_remote_endpoint and saved validation unchanged. Default mcp-remote Figma forms remain generically unsupported by existing transport parser; no parser expansion. Recognized explicit-transport bridge gets Figma reason. No routes or API fields added.
- Checks and evidence: Actual source inspection and Coder diff --check pass. Two independent feature tests added, running separately. No app/provider/account activity.
- Resulting state / acceptance: Implementation ready for tests/review; not final acceptance. Local desktop and saved records untouched by restricted classifier.
- Remaining work and handoff: Focused tests, locked native build, independent Evaluator; native appearance remains unobserved.
