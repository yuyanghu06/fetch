# Shared authentication router verification

November3,2026. Scoped router/refactor acceptance; Google is pending by user
choice. No live provider/helper login or native/MCP-client acceptance is claimed.

All Cargo commands used process-scoped
`DEVELOPER_DIR=/Library/Developer/CommandLineTools`, without changing Mac settings.

| Check | Actual result |
| --- | --- |
| `cargo test --locked --lib manager::` |53 passed, including11 new router cases; compile11.21s, tests0.61s. |
| Existing backend/development_https/environment_paths/import_compatibility targets |16+7+4+7 passed. |
| Initial local_mvp_acceptance |10 passed,1 failed; alias lookup fixture omitted configured routes required by existing deletion protection. Targeted rerun reproduced it; production guard identical in HEAD. |
| Corrected local_mvp_acceptance and raw_json |11+1 passed; compile2.17s, tests0.04s and0.00s. Only affected/unrun targets rerun. |
| `cargo build --locked --bin fetch --message-format short` |exit0,0.74s. Existing five native Security deprecations and unread MCP error-field warning remain. |

99 unique fixture checks passed. The alias repair changes test setup only:
configure stopped disposable routes after reloading Runner before resolving their
saved aliases. It does not run the fixture executable or change production.

Checks cover common dispatch/request forms and named compatibility, Google
pending/no mutation/no execution, same-package accounts using different stored
keys with independent success/failure, safe background errors without fallback,
issuer validation before token exchange, stale attempt rejection and existing
saved-access verification/normalization. No real account credentials are used.

Independent Evaluator ACCEPT; no material findings. Existing public APIs,
authorization, callback ownership, encrypted formats and raw MCP data remain.
Unknown services still need trusted saved-access proof; arbitrary local helper
commands are not guessed. Google integration, explicit duplicate-instance
creation from one source entry and full real-provider/native/client acceptance
remain separate unfinished work. No packaging or release operation occurred.
