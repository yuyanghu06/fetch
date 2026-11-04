# Home by command lookup limits

- Local timestamp / acting role / milestone: 2026-11-04T17:51:29.280200-05:00 / researcher / normal-system-home
- Requested scope: User asks to match HOME to local/global CLI during command search while preserving instance logins.
- Starting repository state: Private HOME restored; no new startup/Keychain mechanism selected.
- Work performed / changed files: Consulted primary Zsh/Bash command/environment docs and active MIT mise/direnv wrapper designs. PATH selects an executable but has no standard per-exec HOME callback. A wrapper chooses environment for one top-level app and descendants; absolute/unwrapped launches bypass it. Global Node/Python can execute a private app; shared /usr/bin/security serves both global/private apps. Binary location cannot safely define account context. Orchestrator records report; specialist source/test activity limited as stated.
- Decisions: No automatic account switch solely from arbitrary executable location. Preserve local package/account ownership; no custom shell/interception/provider code selected. Private instance Keychain setup remains an untested research option, not implemented.
- Checks and evidence: https://zsh.sourceforge.io/Doc/Release/Command-Execution.html ; https://www.gnu.org/s/bash/manual/html_node/Command-Execution-Environment.html ; https://mise.jdx.dev/dev-tools/shims.html ; https://github.com/direnv/direnv/blob/master/man/direnv.1.md
- Resulting state / acceptance: Research/source audit complete, application behavior unchanged. Requested general automatic matching cannot be delivered by current PATH environment code alone.
- Remaining work and handoff: Explain the constraint and agree application-level account/Keychain handling before implementation. No personal state, tokens, Keychain settings or applications touched.
