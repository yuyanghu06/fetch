# Explain current instance IDs and installation reuse

- Local timestamp / acting role / milestone: 2026-11-03T17:06:53.160519-05:00 / Orchestrator / shared-mcp-authentication.
- Requested scope: Explain whether multiple instances already work and whether server IDs are names, counters or generated identifiers.
- Starting repository state: Shared router delivered as4f18905 plus delivery records154c154; pre-existing documentation edits preserved; Google integration pending.
- Work performed / changed files: Read installation generation/reuse, discovery matching, repository/credential storage and alias identity source. Updated current journal and existing milestone guide; no application changes.
- Decisions: No new design or implementation. Current IDs are server- plus32 lowercase hexadecimal digits from16 OS-random bytes, generated for each new instance and persisted. Names remain separate. Importing a different source path or entry name creates a distinct instance even for the same package; reimporting the same path/name reuses its existing ID. No incrementing counter or explicit duplicate-instance control.
- Checks and evidence: src/installation/mod.rs96-118 and263; src/discovery/mod.rs72-90; src/storage/mod.rs44-52,292-307; src/credentials/mod.rs97-140. Source inspection only; previous99-fixture/native-build evidence is unchanged. No tests/provider/helper/personal-state action.
- Resulting state / acceptance: Source-grounded explanation ready. Existing backend state is server-ID scoped, while external helper/global account state still depends on each integration. Two actual Google sign-ins are not claimed supported yet.
- Remaining work and handoff: Continue user walkthrough; Google bridge and any explicit duplicate-install contract remain pending.
