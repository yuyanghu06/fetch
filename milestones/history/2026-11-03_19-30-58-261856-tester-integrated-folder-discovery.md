# Integrated folder discovery verification

- Local timestamp / acting role / milestone: 2026-11-03T19:30:58.261856-05:00 / independent Tester / automatic-config-discovery.
- Requested scope: verify folder discovery together with newer installed-row/direct-install/auth/deletion source.
- Starting repository state: resolved staged merge of b8c6fc7 and origin/main ed78aac; initial 47-case acceptance predates integration.
- Work performed / changed files: added one backend installed-folder case and one Desktop installed-row selection case in tests/backend.rs and src/ui/auth_tests.rs. Existing source/progress installation behavior is retained.
- Decisions: .codex folder input must return the current installed mapping, and automatic discovery/Select all/explicit Select must never opt an already-installed row into a new installation.
- Checks and evidence: env DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --locked --test backend --test import_compatibility --test direct_install passes 24 backend + 7 import + 8 direct-install cases. cargo test --locked --lib ui::auth_tests passes 18 UI cases. cargo test --locked --lib discovery::tests passes 7 cases. Total 64 relevant distinct cases, including 12 new across this feature. Final cargo build --locked --bin fetch --message-format short succeeds exit 0 (0.31s). Existing compiler warnings remain.
- Resulting state / acceptance: integrated fixture and build PASS, no failures or production-code fixes.
- Remaining work and handoff: independent merged evaluation then normal merge commit/push. Actual native picker/window/Figma follow-up and broader protected-runtime acceptance remain separate; no personal configs, live providers, real MCP connections or Keychain were used.
