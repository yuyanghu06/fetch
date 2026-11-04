# Audit home selection boundaries

- Local timestamp / acting role / milestone: 2026-11-04T17:51:29.280201-05:00 / planner / normal-system-home
- Requested scope: User asks to match HOME to local/global CLI during command search while preserving instance logins.
- Starting repository state: Private HOME restored; no new startup/Keychain mechanism selected.
- Work performed / changed files: Read current native_state, runner transport, terminal and installation. Fetch controls env at initial MCP/setup/shell spawn; shell and children perform later lookups independently. npm uses shared Node plus private script args. Imported local scripts can spawn absolute Homebrew gh. Thin PATH wrappers would cover only commands going through them, requiring explicit new coverage/behavior design; current env assembly alone cannot provide requested universal matching. Orchestrator records report; specialist source/test activity limited as stated.
- Decisions: No automatic account switch solely from arbitrary executable location. Preserve local package/account ownership; no custom shell/interception/provider code selected. Private instance Keychain setup remains an untested research option, not implemented.
- Checks and evidence: Read-only source audit; no code or runtime work.
- Resulting state / acceptance: Research/source audit complete, application behavior unchanged. Requested general automatic matching cannot be delivered by current PATH environment code alone.
- Remaining work and handoff: Explain the constraint and agree application-level account/Keychain handling before implementation. No personal state, tokens, Keychain settings or applications touched.
