# Explain repeated launch Keychain approvals

- Local timestamp / acting role / milestone: 2026-11-09T17:57:01.370148-05:00 / Orchestrator, read-only research/explanation / keychain-prompts.
- Requested scope: user asks whether three launch prompts can become one; no storage redesign requested.
- Starting repository state: stable signing/successful-key cache already implemented; current UI/frameless build verified separately.
- Work performed / changed files: inspected existing keychain-prompts design, application host/provider constructor, cached key provider and direct remote Keychain backend. Checked Apple's current permission guidance; updated canonical docs and milestone scope.
- Decisions: recommend human Always Allow per Fetch item, explaining per-item initial approvals versus remembered future permission. Exact prompting reads remain untraced. Consolidated unlock keys/storage migration remain unselected.
- Checks and evidence: source has one MacKeychainKeys provider per host and two permitted own encryption-key IDs; remote stores have separate labels. User screenshot labels Fetch encrypted local state but doesn't identify all three accesses. Apple sources linked in canonical record distinguish Allow from Always Allow. No live Keychain query, secret access, prompt interaction or additional tests run for this explanation.
- Resulting state / acceptance: source-backed explanation complete; no application/authentication/storage change. Does not prove prompt reduction.
- Remaining work and handoff: user chooses permission buttons; if Always Allow still repeats with signed launcher, trace nonsecret item identities/public signing metadata in a separate scoped diagnostic. UI build remains ready for explicit Quit/reopen.
