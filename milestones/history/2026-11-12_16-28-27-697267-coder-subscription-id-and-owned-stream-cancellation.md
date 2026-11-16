# subscription id and owned stream cancellation

- Local timestamp / acting role / milestone: 2026-11-12T16:28:27.697267-05:00 / coder / connector-subscriptions
- Requested scope: Fix confirmed modern correlation/cancellation defects behind reproducible subscription wait.
- Starting repository state: Existing server-first source/fixtures accepted but user-manual Claude stillslow/closed.
- Work performed / changed files: Coder changed only src/connector/mod.rs and guide. New per-request cancellation tokens are separate from dispatch receipts/globalclose. Modern stdio cancel closes only matching owned HTTPsend/stream without POST. SubscriptionId references in protocol notification params metadata and registeredsubscription terminalresult metadata restore original client IDs; ACK retains mapping. Strictforeign checks and raw tool/userdata preservation remain. Source inspection/whitespace pass; SDK GREEN and final19checks pending.
- Decisions: Reuse existing rmcp3.8.0 client via test-only async-rw dev feature; no new runtime dependency/API/auth/notificationpolicy/timeouts/stdin-shutdown change.
- Checks and evidence: RED log /tmp/fetch-subscription-sdk-red.log; source review by root and independent Evaluator, who has no remaining source blocker. Final GREEN/19/build evidence pending.
- Resulting state / acceptance: Failing real-SDK regression established and bounded source fix integrated.
- Remaining work and handoff: Prove SDK GREEN/all19legacy+modern fixtures, signed build-only, final independent ACCEPT; actual coding clients remain manual.
