# Estimate UTM memory and storage

- Local timestamp / acting role / milestone: October 28, 2026, 13:20:20.994314 America/New_York / Orchestrator / isolated-mcp-testing.
- Requested scope: How much RAM and disk storage UTM would require for the proposed Fetch tests.
- Starting repository state: main at 1fae5b6 with preserved uncommitted project-note and virtual-testing documentation; first developer foundation complete, isolated live tests deferred.
- Work performed / changed files: Read official UTM system/drive/guest guidance and Scaleway's primary UTM setup guide; checked host total RAM and workspace-volume free space using read-only sysctl/df. Updated isolated-testing.md, testing milestone info, snapshot/to-do and this history entry. No application edits or VM operations.
- Decisions: No new user selection. Estimate 8 GB guest RAM / 100 GB virtual disk starting point and 150–200 GB free host budget for base/test copies and growth. Clearly distinguish basic guide recommendations from workload estimates and measured hardware capacity.
- Checks and evidence: UTM documents APFS sparse image behavior; Scaleway recommends at least 4 GB RAM / 64 GB disk for its basic macOS VM. Current host returned 17,179,869,184 total RAM bytes and df reported 76 GiB available. Documentation diff/link existence and git diff --check checked by Orchestrator; no independent benchmark or runtime test.
- Resulting state / acceptance: Resource comparison ready to explain; disk headroom is the immediate constraint for the proposed complete workflow. No settings, credentials, files outside project docs, services or environments changed.
- Remaining work and handoff: Select compatible guest/host workflow, storage location and concrete isolation/reset/account/network/evidence plan before provisioning. Existing implementation/push permission and release/live-test restrictions remain unchanged.
