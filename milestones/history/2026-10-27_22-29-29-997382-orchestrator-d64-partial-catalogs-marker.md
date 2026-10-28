# Record marker approval and explicitly incomplete tool lists

- Local timestamp / acting role / milestone: 2026-10-27T22:29:29.997382-04:00 / orchestrator / mcp-mvp-architecture
- Requested scope: Record user approval of first marker example and request to return19 valid tools with incomplete status after one rejected tool.
- Starting repository state: D63 accepted; protocol bundle proposed; whole-origin/native-catalog failure never approved.
- Work performed / changed files: Added D64; rewrote publication guide from whole-failure to explicit partial availability; aligned marker presentation/API/tool/context/notes/instructions and milestone guides/tracking.
- Decisions: Approve native-only required legacy marker; preserve canonical HTTPR. Select raw valid tools/incomplete/rejected reasons, not wholefailure. Exact wire/status/native metadata mapping and separate defaults stay open.
- Checks and evidence: Inspected exact user partial approval and current native ListToolsResult schema _meta semantics. Documentation checker follows; no runtime/config/credential work.
- Resulting state / acceptance: Selected partial-availability direction documented without claiming standard incomplete flag or universal model visibility.
- Remaining work and handoff: Independent approval-boundary review; explain partial behavior and native metadata limitation briefly.
