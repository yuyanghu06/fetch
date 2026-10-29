# Local working MVP run

October 29, 2026. D81 authorizes finishing the selected local MVP, builds/tests,
actual computer-use MCP installation and a separate test Codex instance. No DMG,
packaging, signing/notarization or release work. This file records this run's
actual results; earlier evidence remains in its original records.

## Current work

Precise public import forms, native shared helpers/theme and browser shared HTML
rendering are implemented. General remote metadata/registration/PKCE/renewal
source is added, including callback issuer binding before code/verifier exchange.
Stored-credential success and protected-MCP verification choices remain pending;
preserve D25/D31 until explicitly amended. Required local wrapper/provider setup
is not universally implemented. No real UI installation or native/provider/MCP
acceptance is claimed.

Actual evidence so far:

- Locked native binary build exit0 (first integrated15.49s; final post-fix0.46s).
- Independent selected suite68/68 PASS together: lib22, backend16, HTTPS7,
  paths4, imports7, local-MVP11 and raw JSON1. The first import assertion failed
  because its /mcp substring also matched a hostname; its author corrected the
  assertion, and independent targeted/full reruns pass. See
  [fixture evidence](../../milestones/local-working-mvp/evidence/fixture-checks-2026-10-29.md).
- Independent source evaluation ACCEPT after correcting OAuth issuer mix-up.
  Actual callback-to-token integration still lacks a fixture seam; pure issuer
  guard tests plus source ordering are bounded evidence, not live login proof.
- Clippy all-targets exit0 in19.82s with72 style/dead-field warnings and existing
  native bridge deprecations. This is not strict warning-free lint acceptance.
- Copied Codex0.164.0 to fresh private test-owned files, without copying auth,
  config, hooks or plugins. A confined offline instance initialized and returned
  zero MCP servers, then only that owned process was stopped. See
  [offline client evidence](../../milestones/local-working-mvp/evidence/offline-client-2026-10-29.md).

## Protection and client setup

Accepted scoped source commits:56dbc13 (shared interface),e612017 (safe imports),
7196071 (bound remote OAuth),08c8225 (independent fixtures). Current status/evidence
documentation is saved separately. These commits do not claim full local readiness.

Implementation research: [official Rust auth design](https://github.com/modelcontextprotocol/rust-sdk/blob/main/docs/OAUTH_SUPPORT.md),
[TypeScript auth design](https://github.com/modelcontextprotocol/typescript-sdk/blob/main/docs/clients/oauth.md),
[mcp-remote source](https://github.com/punkpeye/mcp-remote),
[official metadata/redirect security advisory](https://github.com/modelcontextprotocol/rust-sdk/security/advisories/GHSA-5p52-q7pq-9fq2).
The implementation uses existing reqwest/Tokio dependencies and proven designs;
it adds no rmcp or bridge runtime dependency. Exact provider/client restrictions
and real authentication evidence remain separate.

Use disposable files, fresh test credentials and an explicit test-only server
list. Never copy existing Codex auth/config/hooks/plugins or provider credentials.
CODEX_HOME and CODEX_SQLITE_HOME must point to fresh test-owned locations;
cli_auth_credentials_store and mcp_oauth_credentials_store must be file-based.
Use process-scoped CODEX_CA_CERTIFICATE for the test Fetch certificate; do not
disable certificate verification or edit system trust. Reuse of an installed
executable does not permit writes to its installation or everyday state.

Official references: [Codex locations and scoped certificate trust](https://learn.chatgpt.com/docs/config-file/environment-variables),
[credential store settings](https://learn.chatgpt.com/docs/config-file/config-reference).

Read-only facility checks found sandbox-exec and no inspected standard VM/container
tool installation. Current native Fetch always uses MacKeychainKeys/current login
Keychain; root-based namespaces separate item names but do not isolate OS-store
access. Native owner confirmation requires real human authentication. A fresh
Codex profile/data folder alone cannot establish the existing required boundary.
No dedicated OS account or VM exists among inspected users/tools; sudo requires
a password. Do not fake a production native pass with fixture authority.

Computer interaction opened Users & Groups and Add User. macOS is now at its
administrator Password/Unlock prompt. No password was supplied, account created,
security setting changed, native Fetch started or live MCP connected. Human
authentication and an actual protected native session are still required. The
offline client's path/confinement checks do not prove full native/Keychain isolation.

## Verification order

1. Source completion and meaningful independent fixture checks.
2. Locked native binary build and relevant static checks.
3. Establish and verify the concrete protected process/OS environment before live
   MCP or credential tests, using disposable positive/negative canaries.
4. Use computer interaction to install the real reference MCP through Fetch.
5. Launch the explicit test Codex instance connected only to Fetch, make a real
   call and repeat after restart; preserve the existing Codex session/state.
6. Exercise selected raw results, permissions, queue/cancellation/restart and
   authenticated/native flows with dedicated credentials and real confirmation.
7. Fix actual failures, rerun affected checks and obtain independent acceptance.

Computer-use or native authentication handoffs, unavailable fresh provider accounts,
provider client-approval restrictions and any unfinished scope must be reported
explicitly. No success/completion notification before the requested outcome exists.
