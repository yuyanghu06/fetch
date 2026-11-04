# Verify Figma unsupported import and direct install

- Local timestamp / acting role / milestone: 2026-11-04T18:06:30.557549-05:00 / Tester / figma-remote-unsupported
- Requested scope: Independent focused feature checks for selected remote-only import restriction.
- Starting repository state: Coder source implemented; pre-existing edits preserved.
- Work performed / changed files: Tester owns tests/import_compatibility.rs and tests/direct_install.rs; two tests added. Root inspected actual fixtures and assertions.
- Decisions: Two realistic scenarios, no extra edge-case matrix or UI infrastructure. Source review establishes shared disabled selection and message display.
- Checks and evidence: cargo test --locked --test import_compatibility --test direct_install --message-format short PASS 19/19 (8 import,11 direct;2new). Mixed fixture checks remote URL/recognized explicit-transport bridge unsupported with reason, Desktop/other supported/capturable and original config unchanged. Actual Installer::submit URL path fails without saved records/packages. No pre-fix reproduction claimed because source already changed. Locked native build pending.
- Resulting state / acceptance: Feature checks pass; independent evaluation/build still required. No MCP/provider/app/Keychain/personal settings executed.
- Remaining work and handoff: Finish native build then independent Evaluator decision; actual native appearance unobserved.
