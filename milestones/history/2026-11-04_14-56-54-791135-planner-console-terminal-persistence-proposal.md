# Propose persistent console and matching system-shell terminal

- Local timestamp / acting role / milestone: 2026-11-04T14:56:54.791135-05:00 / Planner+Researcher (recorded by Orchestrator) / server-console-terminal
- Requested scope: Latest user explicitly requests Console/Terminal buttons, system shell, and persistent logs. Earlier memory-only permanentconsole scope superseded. User asks global-versus-private availability; no automatic global fallback selected.
- Starting repository state: Prior service-log implementation accepted; new request interrupted before source changes; unrelated LivingBrain note preserved.
- Work performed / changed files: Read current UI/capture/storage/native-authority code; dedicated primary-source terminal library comparison; exact route/storage proposal and milestone docs.
- Decisions: Recommend mature MIT iced_term externaltransport seam/backendportable-pty; exact MCP env/cwd/accountfolder, service owns credentials. New terminal routes require user review perAGENTS; persistent retention policy proposed, not inferred from silence. No auto login/global credentialcopy.
- Checks and evidence: iced_term0.8 supportsIced0.14 but localPTYcoupled; new iced_terminal0.2.1 separated but young/env-overlay issue; xterm+Wry larger frameworkintegration. No dependencyinstall/runtime/code by this pass.
- Resulting state / acceptance: Concrete reviewable proposal ready; no terminal/API implementation before route decision.
- Remaining work and handoff: Obtain exact route/persistence input, then scoped coding/testing/evaluation. Source startupreuse discussion remains research-only.
