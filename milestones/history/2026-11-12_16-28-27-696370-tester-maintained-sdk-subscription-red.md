# maintained sdk subscription red

- Local timestamp / acting role / milestone: 2026-11-12T16:28:27.696370-05:00 / tester / connector-subscriptions
- Requested scope: Fix confirmed modern correlation/cancellation defects behind reproducible subscription wait.
- Starting repository state: Existing server-first source/fixtures accepted but user-manual Claude stillslow/closed.
- Work performed / changed files: Tester added3focused groups: actual official rmcp modern SDK session through productionstdio/pinnedHTTPS; equal original ID7 across two same-profile connectors with isolated cancellation; protocol-field-only metadata translation/raw nested data retention. Root first ran the maintainedSDK target before code change: compiled successfully then failed in3.03seconds with subscription acknowledgement timed out. Owned SDK/connector/fixture cleanup is bounded. No installed clients/personal state used.
- Decisions: Reuse existing rmcp3.8.0 client via test-only async-rw dev feature; no new runtime dependency/API/auth/notificationpolicy/timeouts/stdin-shutdown change.
- Checks and evidence: RED log /tmp/fetch-subscription-sdk-red.log; source review by root and independent Evaluator, who has no remaining source blocker. Final GREEN/19/build evidence pending.
- Resulting state / acceptance: Failing real-SDK regression established and bounded source fix integrated.
- Remaining work and handoff: Prove SDK GREEN/all19legacy+modern fixtures, signed build-only, final independent ACCEPT; actual coding clients remain manual.
