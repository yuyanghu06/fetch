# Python package environment evidence — November 4, 2026

## Delivered behavior

New supported uvx package installs use `uv tool install --managed-python` with separate private tool/environment/export directories. uv owns package resolution and compatible Python selection. Explicit supported --python requests, package pins and server arguments survive shared import/direct parsing. Fetch records installed METADATA version and validated executable, and makes that environment's helper commands available. No package update occurs on saved-record reuse. Existing legacy venvs, standalone script provisioning, accounts, schemas and startup/login rules remain unchanged.

Cross-Mac setup is documented in [PYTHON_MCP_SETUP.md](../../docs/testing/PYTHON_MCP_SETUP.md), linked from AGENTS.md, README.md, DEVELOPMENT.md and docs/testing/info.md. It describes local tooling/signing prerequisites, automatic uv/Python provisioning, optional Python requests, per-Mac paths/fresh setup and verification limits. No separate global Python/uv installation is required for this supported package path.

## Checks

- Independent Tester: tests/python_installation.rs **1/1 PASS**. Submits normal Blender and explicit-Python3.12 Time installs through the actual Installer with a verified deny-default uv fixture. Checks separate tool/HOME folders, omitted default Python, forwarded explicit request/package pin/timezone, actual saved metadata/executable/helper folders, untouched source/outside account and no MCP Connect. Final run: 3.06s compilation/0.44s test. Log: /private/tmp/fetch-python-installer-fixture-check.log.
- Independent Tester: import_compatibility **8/8**, direct_install **11/11**, installation::program_paths_tests **6/6 PASS**.
- Coder: `cargo test --locked --offline --lib installation::direct_tests:: -- --nocapture` **6/6 PASS**.
- Total **32 unique focused checks**, including one new realistic two-server preparation scenario. No speculative matrix or broad unrelated suite.
- Orchestrator: `./scripts/run-development.sh --build-only` **PASS**, native build in 13.26s, signed separate executable and strict signature verification. Existing warnings only. Read-only prerequisites confirmed macOS, Rust/cargo1.94.1 (project requires1.89), developer tools/clang/codesign, valid selected local fingerprint and matching development signing identity/private key. No trust/key/credential changes and no app launch.
- Orchestrator: `git diff --check` **PASS**.
- Independent Evaluator: **ACCEPT** source, fixture evidence and cross-Mac guide, no blocking findings.

## Actual-package and runtime limits

The actual Blender package experiment did **not** run. macOS rejected destination-IP-specific network sandbox rules before uv executed. Tester stopped without broadening access and removed every experiment directory/copied runtime. No actual Blender package installation, MCP connection, desktop control, account authentication or other-Mac execution is claimed. Fixture and build acceptance establish this scoped source change only.

The existing generic installer error still discards child output. Preserving actual installation errors remains separately tracked. Standalone local Python script imports retain their existing fixed-version provisioning; this package change does not claim to fix that path. Personal client/configuration/credential state and the running Fetch instance were preserved.

To use the rebuilt app, explicitly Quit Fetch, then run `./scripts/run-development.sh`. Follow the setup guide on each additional Mac; do not copy environments, saved credentials or signing keys as an automatic setup step.
