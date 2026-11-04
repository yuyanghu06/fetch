# Review the real MCP deletion diagnostic

- Local timestamp / acting role / milestone: November 3, 2026 / independent Evaluator; serialized by Orchestrator / delete-server.
- Requested scope: Review the opt-in real install/echo/delete diagnostic before interpreting runtime evidence.
- Starting repository state: Diagnostic compiled; separate Tester checking confinement and actual execution. Existing product changes untouched by this pass.
- Work performed / changed files: Read-only review of tests/delete_latency_demo.rs and relevant shutdown/storage paths; Orchestrator updates journal.
- Decisions: ACCEPT diagnostic source, conditional on verified OS confinement and actual run. No product behavior change.
- Checks and evidence: Public Installer/Runner/Manager used; explicit ignored opt-in root/marker; test-only keys; checks echo then package/config/runner removal and preservation. The key event measures cumulative shutdown plus package cleanup; sampling proves progress but adds directory reads and cannot quantify fsync contribution. Timeout does not stop the owned deletion worker; external supervisor remains responsible.
- Resulting state / acceptance: Source accepted for backend investigation only. UI, HTTP, existing native process, human confirmation and Keychain timing are excluded.
- Remaining work and handoff: Collect actual timings, check owned-process exit, quantify disk-flush overhead with disposable comparison if needed, and remove temporary state/packages after retaining evidence.
