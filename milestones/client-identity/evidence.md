# Public identity and local callback evidence — November 4, 2026

The identity file is live at https://www.yuyang-hu.org/oauth-client.json . A request with redirects disabled returned HTTP 200, application/json and exactly the authored document, including its own client_id and http://127.0.0.1/oauth/provider/callback registration. The existing homepage returned HTTP 200 with its Yuyang title.

The personal-site change started from the verified production commit af22b2c in a separate worktree. Source commit 6c99ed7 added the static JSON and required documentation. Final documentation commit 7891229 is deployed to production as dpl_8nTR5VudGyoWrYtmnyjNs1EKfpWn, READY. Application code, dependencies, DNS and Vercel settings were unchanged. The original dirty dev checkout was preserved.

The default Next.js production build passed, including TypeScript and all 18 static pages. An initial dependency symlink was unsuitable for Turbopack; copying the existing dependency directory into the worktree resolved that build setup without source or dependency-version changes. No environment or credential files were copied. The final JSON-only callback correction passed a direct content check without repeating the entire build.

Fetch passes the public identity URL through rmcp's existing configuration. New interactive sign-in binds 127.0.0.1 on a temporary port and sends the actual HTTP callback URL to the library. The callback-only router uses the existing parser and Manager flow, checks the pending state, and closes gracefully after completion. Source review also confirms closure on browser failure, flow removal, expiry and Quit. The existing HTTPS API and certificate remain unchanged.

## Focused independent verification

Only one new feature test was written. It drives Manager sign-in against a fake OAuth provider and memory credential store, captures the actual browser URL, sends a real HTTP callback to the temporary IP address, verifies the saved grant and authenticated status, and checks that the listener closes. The fake provider rejects registration, proving the library used the public client identity.

- cargo test --locked --offline --lib manager::remote_callback_tests::remote_login_uses_public_identity_and_temporary_loopback_callback — 1/1 pass.
- cargo test --locked --offline --lib remote_oauth::tests::sdk_legacy_endpoint_fallback_can_finish_without_issuer_metadata — 1/1 pass; existing test.
- cargo test --locked --offline --lib manager::native_storage_tests::revoked_saved_access_requires_quiet_handoff_and_explicit_login_recovers — 1/1 pass; existing test.
- cargo build --locked --offline --bin fetch — pass, 16.53 seconds.
- Scoped whitespace checks — pass.

The independent evaluator reports ACCEPT with no material findings. These checks establish the Fetch flow and public file delivery. They do not establish actual Vercel account acceptance. No provider account, native Keychain, personal client settings, app launch or signing was used for verification.

Next: explicitly Quit Fetch, reopen through ./scripts/run-development.sh and retry Vercel. Moving the identity URL to another domain later may require new authorization. Separate MVP and native/provider acceptance work remains unchanged.
