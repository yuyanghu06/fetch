# Workspace native formats and paging

- Local timestamp / acting role / milestone: 2026-11-12T15:37:46.280108-05:00 / researcher / Workspace.
- Requested scope: Build the selected Workspace feature with a frontend wrapping backend APIs.
- Starting repository state: Substantial unrelated tracked/untracked work; project MCP URL remains proposed, current /mcp unbound. Existing native Settings capability available.
- Work performed / changed files: Verified official model initialization protocols and native formats. Claude SDK message pagination reads the full file before slicing; bounded raw JSONL reads avoid that cost. Selected maintained MIT/Apache serde-saphyr2.1.1 for YAML parsing. No real model/CLI metadata process or personal state access. Root records the selected behavior and proposed private routes in docs/architecture/workspace.md and milestones/workspace/info.md; preserves all existing records.
- Decisions: User selects native-only instruction/agent APIs, existing-file editing, converted paste drafts, dynamic model choices and read-only chunked sessions. Exact new HTTP contract review and MCP project connection scope are pending through asynchronous questions.
- Checks and evidence: Read-only source/docs inspection and primary-source research. No builds, tests, personal file edits, credential use, runtime clients, MCP connections, app relaunch or release.
- Resulting state / acceptance: Plan/research complete; backend core implementation in progress. No independent implementation acceptance yet.
- Remaining work and handoff: Finish core, receive route decisions, wire native UI/API, verify disposable fixtures and independently evaluate. No histories on unbound MCP.
