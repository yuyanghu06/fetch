# modern subscription transport findings

- Local timestamp / acting role / milestone: 2026-11-12T16:23:52.303652-05:00 / researcher / connector-subscriptions
- Requested scope: Diagnose/fix continuing manual slow reconnect and tool-fetch closure.
- Starting repository state: Server-first34fixtures/signedbuild accepted; actual user retry still fails. Unrelated work retained.
- Work performed / changed files: Read-only Researcher verifies official specification, TypeScript client ack correlation and local maintained rmcp3.8.0 validation. Identifies missed reserved subscription references and mismatched stdio→HTTP cancellation. Crate Apache-2.0, existing dependency reused.
- Decisions: User reports slow reconnect and tools fetch connection closed even after server-first build. Read-only sampling of their existing connector shows Runtime::block_on and a stdin read, not runtime shutdown; backend public metadata responds200 in0.003289seconds. The connector later exits. No agent-launched coding client or credential access occurred. The original tool-count attribution remains unproven.

Research identifies two actual source bugs: outgoing subscription listen IDs are rewritten but protocol-owned subscriptionId metadata is not restored, so maintained SDK acknowledgement validation fails; modern stdio cancellations are incorrectly POSTed as notifications to HTTP, where they are rejected, instead of dropping only the matching stream. This provides a concrete possible chain (ack mismatch→timeout→cancel→fatal rejection), not a captured exact Claude trace.

Bounded repair restores only protocol subscription-ID references from this connector's active subscription map and adds separate per-request modern cancellation tokens; existing legacy dispatch receipt/POST, permissions, raw tool payloads, progress tokens and global connector shutdown remain. No new route/auth/policy or speculative runtime shutdown fix. Independent real-rmcp-client fixture precedes implementation; actual installed clients remain manual.
- Checks and evidence: Read-only source/primary sources and existing-process sample; no real client rerun by agent, no new service or credential/config/trust changes.
- Resulting state / acceptance: Research/plan accepted; actual SDK red fixture and code correction pending.
- Remaining work and handoff: Reproduce with maintained SDK, implement/test/review/build; user owns actual client/native acceptance.
