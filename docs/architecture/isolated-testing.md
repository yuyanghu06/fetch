> October 28 follow-up: the user selected importing normal global Codex definitions
> into development Fetch, then a fresh Codex profile connected only to Fetch. The
> main Codex may use that same Fetch instance concurrently. Current implemented
> defaults and portable paths are in [DEVELOPMENT.md](../testing/DEVELOPMENT.md).
> The earlier Fetch-Testing layout below remains historical/proposed; it is not
> the native debug default. Existing host/provider protection requirements remain.

# Isolated MCP testing environment

October 28, 2026. Status: researched options and proposed operating rules, awaiting the user's choice. No VM, software installation, credentials, client settings, trust changes or live MCP tests are authorized by this discussion. D46's isolation requirement and D74's source-build permission remain unchanged. This document expands the isolated-verification item in [the single implementation worklist](build-readiness.md#one-engineering-worklist--not-separate-product-approval-rounds); it is not another product approval backlog.

## Current batch: short MCP checks and proposed location

D76 records the user's immediate scope: standard MCP behavior, installation and authentication flows, without heavy or long-running workflows. A lighter separate-profile/install arrangement is proportionate for that batch, provided actual access controls protect the resources used. Test duration does not itself isolate files, credentials or apps. This batch does not replace the broader recorded release/client acceptance requirements.

Sequence requested by the user: define the location first, then select a representative MCP server list and record it in the to-do/milestone. The user subsequently explicitly requested curation: [the first roster](../../milestones/isolated-mcp-testing/servers.md) now selects **Everything, Filesystem, Time, Tavily and Linear** as test targets, all not run. The location/layout below remains a recommendation for review, not an already approved/provisioned environment.

Recommended root on this Mac:

```text
~/Library/Application Support/Fetch-Testing/
  clients/
    codex/                 # Separate test client configuration/state
    claude/                # Separate test client configuration/state
  fetch/                   # Test Fetch data directory
    packages/
      runtimes/            # Test-owned managed runtime installations
      servers/             # Replaceable test server packages
    servers/               # Durable test server config/encrypted credentials
  cache/
    npm/                   # Test-only package cache
    uv/                    # Test-only package cache
  workspaces/              # Disposable files/projects exposed to test tools
  browser/                 # Dedicated test browser profile/login state
  runs/                    # Private per-run logs and raw evidence
```

Why here: it is outside Git, outside the source checkout and outside normal `Application Support/Fetch`, `~/.codex` and `~/.claude` state. It avoids a second operating-system installation. Keep runtime files private to the test owner, and do not symlink personal config/cache/credential directories into it. Existing client executables can be reused; package/runtime isolation concerns the test state and managed installations, not a required duplicate CLI binary.

Supported now versus intended wiring:

- Existing `fetch dev --data-dir` can point to the `fetch/` directory. Use a separate loopback port when running alongside ordinary development; `53672` is a proposed test default, subject to availability, and conflicts must not stop another service. Existing `--port` supports this; no server was started or port reserved.
- Separate client profile controls are documented in the research below. Pin an explicit test-only MCP list and keep inherited personal plugins/hooks/configs out. Exact installed-version compatibility and launch settings must be checked when setting up.
- `fetch/packages/` and `fetch/servers/` mirror the selected replaceable-package versus durable-data separation under this alternate root. Current installation/authentication/MCP-sharing code is unfinished; this diagram is a target layout, not a claim these paths are already supported by an installer. No separate package-root API/flag is proposed.
- Route applicable npm/uv/browser/workspace/log state to the shown directories when supported; verify each selected server's actual storage behavior. No universal environment override forces arbitrary MCP software to obey this root. OS Keychain entries and provider-side tokens do not live entirely inside it and are not erased/revoked by deleting it.
- This pass only defines paths. Dedicated test credentials and actual client/Fetch/MCP process or account restrictions remain prerequisites before live tests. Native desktop-app control is outside this first batch. No VM or new Mac user account is required merely to define folders; no general Keychain/browser-isolation guarantee follows from them.

Keep source, the server/check list and inspected redacted evidence in the repository. Future per-run summaries may go under `milestones/isolated-mcp-testing/evidence/<run-id>/`; raw logs, tokens, browser state and downloaded packages stay private under the external root. Create no evidence entry claiming checks ran before actual execution.

Short check scope, as applicable to each later-selected server: install/start; initialize and list tools; make one harmless bounded call; authenticate with a dedicated test login; check success and basic rejection/cancel behavior; verify saved test login reuse. Select concrete cases and supported mechanisms with the later server list. No load testing, endurance run, destructive provider operation or long agent workflow is included in this batch.

Current state: root checked read-only and does not exist; no directories created, package installed, client launched, authentication attempted or live MCP connection run. The requested five-server roster is recorded with source/version/auth/expected checks. Next: complete the location and concrete launch/access/auth prerequisites before authorized execution.

## Latest discussion: lighter client profiles and separate installs

The user finds the VM budget too heavy for the current Mac and proposes fresh Codex/Claude Code sessions with separate MCP installation paths. This is a feasibility question, not permission to launch clients or tests. The earlier VM-first recommendation below is no longer the only proposed next step. A lighter setup is feasible for bounded tests, but separate paths alone do not establish D46 isolation.

Verified controls and limits:

- Codex CLI can use a separate `CODEX_HOME` for config, logs/history and file-based state. Project/system configuration and OS credential storage need separate review; changing this path is not a separate OS account. [Official configuration docs](https://learn.chatgpt.com/docs/config-file/config-advanced).
- Claude Code supports `CLAUDE_CONFIG_DIR`, explicit `--mcp-config` plus `--strict-mcp-config`, and `--setting-sources`. These separate/select client state and MCP definitions. Claude login storage has documented profile behavior and exceptions; do not claim every credential store changes with this variable. [CLI reference](https://code.claude.com/docs/en/cli-reference), [authentication](https://code.claude.com/docs/en/authentication).
- Fresh chat history does not change process permissions. OpenAI explicitly states that local stdio MCP processes run outside the local command sandbox. Anthropic likewise distinguishes its built-in shell sandbox from protection of MCP servers, file tools and hooks. A sandbox on the agent's shell must not be assumed to protect Fetch or its children. [OpenAI security cookbook](https://learn.chatgpt.com/docs/enterprise/security-cookbook), [Claude sandbox environments](https://code.claude.com/docs/en/sandbox-environments).
- Anthropic's standalone sandbox runtime (`srt`, Apache-2.0) is a maintained, beta/research-preview candidate for restricting an entire process tree on macOS without another OS. It needs explicit filesystem read/write and network rules; default broad reads are not sufficient. Independently started Fetch must also be covered. Enabling Apple Events can permit unsandboxed app launches, so do not enable it and claim the same boundary still holds. [Official runtime repository](https://github.com/anthropics/sandbox-runtime).
- Same-user Keychain isolation is not established. [Open issue 755](https://github.com/anthropics/sandbox-runtime/issues/755) reports login-Keychain access despite credential-file restrictions in runtime 0.0.79; this was not reproduced here, and no fix is assumed. The specialist observed release 0.0.81 with Windows-only changes. A restricted test account is therefore preferable for credential/native-app tests, with actual permissions/services verified rather than presumed isolated.

### Proposed lightweight arrangement

1. Reuse installed client executables unless testing a specific separate version. Create fresh CLI profiles, explicit test-only MCP definitions, a disposable workspace, separate package/install/cache paths and dedicated test credentials. Avoid inheriting personal hooks, plugins, configs or tokens. Changing `HOME` alone is not a complete solution.
2. Run a separate Fetch instance with its own developer data directory and unused loopback port. The current developer CLI already supports `--data-dir` and `--port`; the runner/MCP-sharing features are still unfinished. Separate paths prevent state collisions but grant no additional access protection.
3. For bounded protocol/file/process tests, explicitly restrict the client and Fetch/MCP process trees. Verify access-denial behavior with harmless disposable fixtures before authorizing actual connection tests, including no unsandboxed fallback. Do not experiment on real secrets to check that a boundary works.
4. For Keychain, native confirmation and desktop-app workflows, prefer a dedicated **standard macOS test user** with its own home, login Keychain and logged-in desktop apps. This uses the existing OS and avoids a VM's second macOS installation. Check permissions, shared folders/processes, network and services; user separation is not complete machine isolation. Switching to that user's desktop is an added inconvenience. [Apple user-account guidance](https://support.apple.com/en-ie/guide/mac-help/mchl3e281fc9/mac).
5. Defer any integration whose files, credentials, live apps or services cannot be confined by the selected arrangement. Preserve VM/dedicated physical test Mac options for those gaps. Dedicated external provider accounts remain necessary; profile deletion does not undo remote actions.

Earlier recommendation given the user's hardware: pursue separate CLI profiles/Fetch state and explicit process restrictions, with a dedicated standard Mac test user for real credential/desktop workflows when needed. The short batch and proposed location above are now the active planning step. No process/account boundary is selected or verified. There is no second OS to budget RAM/disk for; usage comes from the clients, MCPs, caches and builds actually run. No numerical performance promise is made.

## What the different environments separate

| Kind | Representative software | What it separates / fit for Fetch |
| --- | --- | --- |
| Full virtual machine | UTM, Parallels Desktop, Tart | Runs a separate operating system with its own accounts, files, installed apps and credential stores. An option for broader Mac/client/MCP separation when the lighter plan cannot confine resources. Hardware-specific behavior still needs separate evidence. |
| Linux containers | Docker, Podman | Separate processes, files and networking according to their configuration. Useful for disposable test services and Linux-compatible MCP helpers. They cannot establish macOS Keychain, native prompt or Mac desktop-app behavior. Shared host folders or excessive privileges weaken their separation. |
| Python virtual environment | Python venv | Separates installed Python packages, while programs keep their normal operating-system access. Useful inside a test system, insufficient by itself for D46. |

Sources: [Python venv](https://docs.python.org/3/library/venv.html), [Docker security](https://docs.docker.com/engine/security/), [Docker host-folder access](https://docs.docker.com/engine/storage/bind-mounts/), [Podman machine](https://docs.podman.io/en/latest/markdown/podman-machine.1.html). These tools overlap: a Python environment or containers may run inside a VM; they are not all equivalent security boundaries.

## Mac VM options

- **UTM:** free downloadable, open-source desktop VM software. Its documented macOS-guest path requires an Apple Silicon Mac. Earlier hands-on recommendation; retained as an option while the lighter plan above is considered. Preserve a clean powered-off base and run separate test copies; confirm the selected release's clone/reset behavior. Current documentation makes Apple-backend snapshots/disposable mode depend on macOS 27 or later, so snapshot support must not be assumed for an older host. [Repository/license](https://github.com/utmapp/UTM), [macOS guests](https://docs.getutm.app/guest-support/macos/), [snapshots](https://docs.getutm.app/advanced/snapshots/), [disposable mode](https://docs.getutm.app/advanced/disposable/).
- **Parallels Desktop:** paid desktop alternative with a documented macOS-on-Apple-Silicon installation path. A reasonable choice if already owned or commercial support is preferred. Check macOS-specific feature limits rather than assuming its Windows features also work in macOS guests. Explicitly disable host sharing. [Product](https://www.parallels.com/products/desktop/), [macOS installation](https://kb.parallels.com/en/125561), [macOS guest limits](https://kb.parallels.com/en/128867).
- **Tart:** macOS/Linux VM tooling on Apple Silicon, designed around building, cloning and running repeatable VM images. Recommended alternative when automated repeated runs are the priority. It is source available; assess the current repository license before adopting it, rather than assuming UTM's license applies. [Overview](https://tart.run/), [quick start](https://tart.run/quick-start/), [current repository/license](https://github.com/openai/tart).

These are suitability judgments based on primary documentation, not runtime comparisons, measured performance or a claim that one vendor is universally standard. A rented remote Mac is a hosting choice for the same isolation design, not a prerequisite; no hosting provider or spending is selected.

## Earlier VM proposal and sizing, retained for comparison

### UTM sizing estimate — October 28 follow-up

The user asked about RAM and disk requirements. These are planning estimates for a macOS guest with Fetch, browser/agent clients and test MCPs, not measured Fetch benchmarks or universal UTM minimums:

| Workload | RAM assigned to one running VM | Virtual disk capacity |
| --- | --- | --- |
| Light, limited testing | 4–6 GB; likely constrained | 64 GB; little development headroom |
| Recommended starting point | 8 GB | 100 GB |
| More clients/servers or builds at once | 12–16 GB, subject to host capacity | 150 GB or more |

For context, [Scaleway's own UTM macOS setup guide](https://www.scaleway.com/en/docs/apple-silicon/how-to/setup-vm-with-utm/) recommends at least 4 GB RAM and 64 GB disk for its basic VM setup. Our higher estimates allow for tools, packages and build files. A host with 16 GB total RAM can support a modest 6–8 GB guest if other heavy applications are closed; 24–32 GB total is a more comfortable planning target. Guest RAM is needed while running, not for a powered-off base copy; virtualization also adds overhead.

Allow roughly 150–200 GB of free host storage for a 100 GB test disk plus a clean base, installer, logs and growth; actual needs depend on image usage/copy behavior. This is a budget, not a fixed allocation requirement. [UTM's drive documentation](https://docs.getutm.app/settings-apple/drive/) says images on APFS are sparse: unwritten space does not consume the full configured capacity immediately. Do not assume copies stay free or disk usage automatically shrinks after guest deletion.

A read-only check of the current Mac returned 17,179,869,184 RAM bytes (16 GiB) and 76 GiB free on the workspace volume. No current free-RAM/performance measurement was made. The proposed full guest-plus-base workflow therefore needs more disk headroom, an external SSD, or a smaller carefully managed plan; no storage purchase, cleanup or VM sizing is selected. The guest architecture/OS and exact footprint still need confirmation before provisioning.

### Proposed run sequence

1. Prepare a clean macOS guest with a dedicated test user. Fetch, the test browser, Claude Code/OpenCode/Codex and any desktop apps controlled by MCP all run inside that guest. Use the existing redacted server inventory rather than asking the user to relist it.
2. Keep everyday host folders, credential stores, client settings, clipboard and devices disconnected. Install a source copy/build inside the guest instead of sharing the user's home directory. Keep Fetch and clients together so `localhost` refers to the guest; any test certificate trust changes stay there.
3. Use disposable files, service workspaces and dedicated test credentials. Recreate source definitions without migrating personal secrets. A VM does not prevent an API credential from changing a real cloud account, and restoring its disk does not undo those remote changes.
4. Define network rules before connection tests: permit required package/provider destinations and local guest services; restrict access to everyday host/LAN services. Ordinary VM networking alone does not prove that isolation. Exact enforcement depends on the chosen tool and host.
5. Keep a clean base and run copies for tests. Record exact macOS, Fetch, client and MCP versions, expected outcomes, redacted logs and UI evidence. Export only inspected evidence, then discard/reset the test copy and clean up dedicated remote test data separately. Keep secrets out of reusable base images and exported evidence.

## Evidence limits and next choice

No environment has been provisioned. D76 selects the short first test batch, the requested roster is curated, and the location proposal above is ready for review. Concrete credential/file/app/service/network boundaries and launch verification remain prerequisites before live tests. UTM/Tart/Parallels remain alternatives if needed; a VM vendor is not the mandatory next choice. This is the existing isolated-verification work item, not new application behavior.

macOS guests do not prove every hardware-backed behavior. There is no verified macOS-guest Touch ID passthrough in this research. Keep biometric-specific acceptance open; a dedicated physical test Mac may be needed. Guest account/service support also has limits: [Apple's macOS VM service documentation](https://support.apple.com/en-ca/120468). Do not weaken the selected confirmation policy merely to make a VM test pass.

The first developer backend remains the only completed implementation increment; native authorization/lasting identity/Keychain, installation, desktop UI and MCP sharing remain incomplete. Environment research is not live compatibility evidence.
