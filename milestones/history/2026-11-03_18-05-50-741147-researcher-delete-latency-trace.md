# Trace deletion latency before a disposable demo

- Local timestamp / acting role / milestone: November 3, 2026 / Researcher; serialized by Orchestrator / delete-server.
- Requested scope: Install and demo deleting an MCP, measuring why deletion is slow.
- Starting repository state: Existing dirty source/UI/documentation changes preserved; actual protected native setup last recorded incomplete. Shared role files/workflow are absent on this machine; session role instructions used.
- Work performed / changed files: Read-only researcher traced manager, runner, storage, credentials and UI. Orchestrator records this pass in snapshot/to-do/history.
- Decisions: Measure a disposable actual package through real backend with test-only keys under a verified OS sandbox. Do not claim native/Keychain acceptance or change product deletion behavior without evidence.
- Checks and evidence: storage/mod.rs remove_entry calls sync_all after every file/directory removal. Manager awaits full package cleanup before credentials/config removal. Process close permits 4 seconds graceful + 1 second forced wait. Global installer lock remains held throughout deletion; UI awaits completion with 150-second HTTP timeout. No proven connection RwLock deadlock.
- Resulting state / acceptance: Concrete latency hypotheses and bounded measurement plan established; no runtime result yet.
- Remaining work and handoff: Verify sandbox canaries, install official disposable MCP, time deletion and preserve unrelated fixture/source/runtime files. Protected native window demo remains unprovisioned.
