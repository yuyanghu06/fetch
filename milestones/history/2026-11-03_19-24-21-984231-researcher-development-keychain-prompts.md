# Investigate repeated development Keychain prompts

- Local timestamp / acting role / milestone: November 3, 2026 / dedicated Researcher plus Orchestrator; serialized by Orchestrator / keychain-prompts.
- Requested scope: User screenshot reports about five same-label login-Keychain access prompts; investigate why and whether one approval can suffice.
- Starting repository state: Deletion speed fix delivered; unrelated helper/auth/UI/docs changes remain active. No prompt or credential API operated in this pass.
- Work performed / changed files: Read source startup/key provider/native bridge; inspected public binary signing metadata and counted encrypted record existence only. Researcher compared primary Apple docs and maintained licensed keyring implementation. Added canonical keychain-prompts.md, milestone guide and related index/notes/journal updates.
- Decisions: Recommendations only. Stable dev signing first; optional process key cache; literal single-item unlock needs wrapping-key/migration architecture decision. Preserve current per-server separation and no personal signing/Keychain/trust/secret changes.
- Checks and evidence: Current binary Signatureadhoc/noTeam. One app and one server encrypted record by metadata; no encrypted bytes/values read. Separate keys share display label and are repeatedly fetched without cache. Apple per-item auth and code-identity guidance support diagnosis; exact five dialogs untraced. Traditional-store background UI suppression remains unresolved; no assumption LAContext or bulk query fixes it. Details/links in canonical research doc.
- Resulting state / acceptance: Evidence-backed explanation/options documented; source/native behavior unchanged. No native prompt-count or proposed fix acceptance claimed.
- Remaining work and handoff: Independent review of claims, explain options plainly, obtain user decision before any signing/cache/migration implementation and establish protected native verification for chosen fix.
