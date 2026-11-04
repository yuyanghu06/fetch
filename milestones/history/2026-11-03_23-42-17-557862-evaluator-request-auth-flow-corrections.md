# Request corrections before authentication acceptance

- Local timestamp / acting role / milestone: 2026-11-03T23:42:17.557862-05:00 / Evaluator / standard-authenticate.
- Requested scope: Independent source review of generic login/start-route implementation before tests.
- Starting repository state: Runner source ready; Manager, root UI/API and independent fixtures integrated in progress.
- Work performed / changed files: Read-only review of source/fixtures; no Evaluator edits or runtime commands. Root recorded report and assigned fixes.
- Decisions: REQUEST CHANGES. Restore pending host consent from backend status after window reopen; atomic active/queued/uncertain-work guard for both startup and helper; recognize SERVER_INPUT_REQUIRED in ordinary connection outcomes; demonstrate/await owned helper cleanup on Quit.
- Checks and evidence: UI clears local flows on close but fallback lacked consent; Scheduler begin_restart fences rather than rejects active work; Manager connection_failed omitted new input code; helper task not owned by Runner shutdown. Root fixed reopen code and added fixture; Coders/Tester own other fixes.
- Resulting state / acceptance: Not accepted; no runtime success claimed. Core privacy/route/interactive-wait direction otherwise matched selected scope.
- Remaining work and handoff: Finish corrections, meaningful tests, final read-only review and exact evidence.
