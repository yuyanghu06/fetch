# gh auth debug capture guidance

- Local timestamp / acting role / milestone: 2026-11-04T17:36:49.255671-05:00 / orchestrator / codex-startup-environment
- Requested scope: User asks how to capture gh auth login logs for the reported Keychain error.
- Starting repository state: Cause unconfirmed; no service login or personal Keychain experiment authorized/run by agent.
- Work performed / changed files: Read official gh environment/login documentation and keyring source plus local macOS script(1) manual. Checked current Fetch terminal persistence policy: terminal recording is not saved as service Console logs. No code changes.
- Decisions: Recommend user-run GH_DEBUG=1 browser flow under macOS script with a private mktemp output file, preserving interactive prompts. Capture gh version, selected HOME/XDG/GH_CONFIG_DIR and read-only default/list Keychain path metadata in the same process context. Do not capture full env, token files, show-token or HTTP body debug. Review/redact one-time codes or tokens before sharing.
- Checks and evidence: https://cli.github.com/manual/gh_help_environment confirms verbose stderr with GH_DEBUG=1 and additional HTTP tracing only with api. Local script(1) confirms file/command argv, append/quiet and terminal recording. Only documentation/metadata-source reads performed; diagnostic command was not executed.
- Resulting state / acceptance: Capture instructions prepared; diagnosis awaits user output. No root cause or working repair claimed.
- Remaining work and handoff: Analyze returned log and process-context paths. Keep Keychain unchanged until cause established.
