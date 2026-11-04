# Confirm actual global GitHub CLI

- Local timestamp / acting role / milestone: 2026-11-04T17:54:37.382153-05:00 / orchestrator / normal-system-home
- Requested scope: User asks to check actual GitHub package/launcher and whether gh is global, then says “oh it isnt.”
- Starting repository state: Private HOME retained; HOME/Keychain matching unresolved, no new execution changes selected.
- Work performed / changed files: Read only matching development config noncredential metadata, launcher command lines and filesystem symlink targets. No credentials file or token command executed.
- Decisions: Confirm gh IS global in this exact entry: launcher hardcodes /opt/homebrew/bin/gh; symlink resolves /opt/homebrew/Cellar/gh/2.89.0/bin/gh. Fetch imports existing local-script launcher ~/.config/shared-mcp/bin/github-mcp and sibling github-mcp-server. No recorded private package folder/program dirs/native setup exists for this entry. Global executable still receives Fetch private HOME through existing transport.
- Checks and evidence: Matched saved development executable to launcher; checked recorded package folder is absent; inspected only gh invocation and exec line. Global gh and sibling MCP binary exist. No gh/MCP/Keychain/auth/process execution.
- Resulting state / acceptance: Executable identity confirmed; actual Keychain failure not repaired. No code/config/credential changes.
- Remaining work and handoff: Explain this concrete global-binary/private-environment finding. Any changed account-home/Keychain behavior still needs bounded agreed design; preserve locally installed Railway instance login.
