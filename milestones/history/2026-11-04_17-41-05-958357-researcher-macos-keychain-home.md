# macOS Keychain home research

- Local timestamp / acting role / milestone: 2026-11-04T17:41:05.958357-05:00 / researcher / normal-system-home
- Requested scope: Find supported generic startup-context fix for gh auth login Keychain failure in Fetch terminal; retain service-owned credential handling.
- Starting repository state: Actual Codex builder integrated; shared transport still overrides HOME/XDG to private per-server folders. Normal inherited PATH change independently integrated.
- Work performed / changed files: Researcher read primary Apple Security/CF and GitHub CLI source/docs; no writes/experiments. Orchestrator inspected current shared transport and records result/proposal.
- Decisions: Recommend removing Fetch HOME override while retaining XDG/private packages/helper PATH/cwd. This is proposed pending explicit user choice because normal HOME allows shared service logins/files. CFFIXED_USER_HOME does not fix direct Security getenv HOME calls. No source copying or new dependency needed.
- Checks and evidence: Apple pinned db15acbe6a7f257a859ad9a3bb86097bfe0679d9 OSX/libsecurity_keychain/lib/DLDBListCFPref.cpp#L615 and StorageManager.cpp#L2055 establish direct HOME path lookup. gh pinned eeb44bc432c0107de002dadd34a61b4e2bd15882 internal/config/config.go#L392/#L725 establishes Keychain token namespace independent of config dir. cli.github.com/manual/gh_help_environment documents XDG config support. Apple APSL2 source unarchived/pushed June18,2026; gh MIT source unarchived/pushed Nov3,2026. No code imported.
- Resulting state / acceptance: Research complete; mechanism established, user's exact failure not reproduced. Implementation not started. Shared-home/account-access choice is pending.
- Remaining work and handoff: Conditional Planner identifies source/test scope and pre-sandbox HOME-write fixture risk. Await choice; never run that fixture unchanged. Then implementation, isolated verification and independent evaluation.
