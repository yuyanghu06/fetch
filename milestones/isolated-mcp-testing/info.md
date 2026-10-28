# Isolated MCP testing

## Goal and scope

Define and later verify a test system that protects everyday files, credentials, client settings, apps and services while exercising Fetch with the recorded server inventory and real supported clients. [The environment comparison](../../docs/architecture/isolated-testing.md) expands the existing engineering worklist.

Immediate D76 batch: short standard MCP behavior, installation and authentication checks, without heavy/long workflows or desktop-app control. The location is defined as a proposal, and the user's subsequent server-curation request is now fulfilled below. Broader MVP/client acceptance remains separate and unchanged.

The user subsequently explicitly requested curation. [The first roster](servers.md) is Everything, Filesystem, Time, Tavily and Linear. It covers Node/Python installation, small protocol/file/time calls, API-key auth and browser OAuth, with optional HTTP/SSE runs of the same fixtures. All checks are not run; test credentials/accounts and actual access/auth mechanisms are still prerequisites. Choosing targets does not install or prove them.

Roster/current-tracking documentation independently accepted after a stale snapshot checkpoint was corrected. This completes curation, not the testing milestone or any runtime compatibility claim; history is linked from the rolling snapshot.

## Completion criteria

Choose a compatible Mac environment and concrete isolation/reset/account/network plan; obtain operational authorization before provisioning or live connections; independently verify the selected real-client flows and record redacted evidence and unresolved gaps. A passing build, Python virtual environment or Linux container alone does not establish full Mac compatibility.

## Current status

Planning only. The short first-batch scope is selected and the requested five-server curation is recorded. Proposed private root: `/Users/yuyang/Library/Application Support/Fetch-Testing/`, with separate `clients/`, `fetch/packages/`, durable `fetch/servers/`, `cache/`, `workspaces/`, `browser/` and `runs/`. No directory was created. Keep only reviewed/redacted reports in project milestone evidence. The precise layout and live-access/auth mechanisms remain unselected/unverified. Profile paths and normal shell sandboxes alone do not isolate MCP servers; OS/provider-held credentials need explicit handling. VM research remains a comparison, not a current requirement. Application implementation permission and the completed developer foundation are unchanged.

Earlier VM sizing, retained only for comparison: an 8 GB RAM / 100 GB virtual disk starting point and roughly 150–200 GB host free-space budget for a clean base plus test copy/installers/growth were estimates, not benchmarks. The read-only host check found 16 GiB total RAM and 76 GiB free on the workspace volume. The current lightweight proposal has no second OS installation or selected VM resource allocation.

## Next action

Review/complete the location and actual launch/file/credential/network/auth prerequisites for the [curated roster](servers.md), then prepare/run only when authorized and supported by the implemented backend. Per-server package/source/transport/auth/check/status are recorded in the roster and to-do; all execution is pending. The snapshot links serialized research/review and Orchestrator history.
