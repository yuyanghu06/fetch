# Portable development state

Goal: development-owned packages, runtime/cache files, server config/credentials
and app identity must default to a different current-home-relative folder from
the final build, with a distinct native port. Existing explicit overrides and
read-only global discovery stay available. A fresh Codex profile and main Codex
can share one development Fetch instance with separate client authorizations.

Source changes and four unexecuted regression tests are integrated locally after
independent source review. Source acceptance is complete; runtime acceptance is
pending. No old state migrated, profile created, login/provider connection run,
personal client config edited or certificate trust changed by this work.

Completion requires actual path/default/override/cwd checks and selected native,
client-login and provider checks with redacted evidence. Follow the current
snapshot/to-do and docs/testing/DEVELOPMENT.md. Root serializes journal updates;
history entries remain immutable. No release is selected.

Follow-up binary-only compilation succeeded (exit0,9.74seconds). The binary and
regression targets were not executed; native/client/provider acceptance remains pending.
