# Explain gh auth login Keychain prompt

- Local timestamp / acting role / milestone: 2026-11-04T17:35:15.506420-05:00 / orchestrator / codex-startup-environment
- Requested scope: Explain user screenshot showing macOS Keychain Not Found during gh auth login.
- Starting repository state: Environment helper implementation accepted; concurrent terminal/logging work remains separate.
- Work performed / changed files: Read-only inspection of native transport/terminal env_clear and protected HOME/XDG settings; consulted official gh auth login manual, cli/cli internal/keyring and go-keyring macOS implementation. No application code or personal state changes.
- Decisions: The screenshot establishes failed Keychain storage discovery for the process, not the underlying cause. Private HOME is a hypothesis only; no root cause confirmed. Do not reset/create Keychains or change startup behavior from this evidence.
- Checks and evidence: https://cli.github.com/manual/gh_auth_login documents default system-store use and plaintext fallback on failure. https://github.com/cli/cli/blob/trunk/internal/keyring/keyring.go wraps go-keyring. https://github.com/zalando/go-keyring/blob/master/keyring_darwin.go invokes /usr/bin/security for storage. Local code confirms terminal receives per-server environment/private HOME. No token files/Keychain entries read; no login or experiment run.
- Resulting state / acceptance: Explanation complete; no repair implemented or successful credential save claimed.
- Remaining work and handoff: If investigating further, establish whether command ran inside Fetch and compare read-only Keychain metadata in the relevant process context under appropriate authorization. Keep cause unconfirmed until tested.
