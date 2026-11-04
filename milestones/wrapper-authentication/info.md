# Remote authentication follows the MCP library

Goal: remove Fetch's custom remote sign-in compatibility gates so the official MCP library owns discovery and callback validation.

Scope: remove discovered-only metadata, mandatory issuer metadata, mandatory advertised callback-issuer support, eager validation of unused metadata endpoints, full saved metadata-document equality, and rejection of unrelated callback query fields. Keep request-time network checks, library validation, optional callback issuer forwarding, native consent, instance credential separation/storage and existing routes. Audit neighboring startup/authentication code for other override gates; do not add provider-specific code or new login methods.

Completion: bounded planner/audit, source removal, independent fake-HTTP/fake-Keychain regression tests proving non-advertised/missing callback issuer and library fallback can proceed, library-owned mismatch rejection preserved, existing auth regressions and locked native build pass, independent evaluation accepts. No real provider sign-in or app launch is claimed.

Status: complete for the scoped removal.43 independent auth/storage/authority checks pass; current-tree locked native build passes; final Evaluator ACCEPT. No real Vercel/native account acceptance claimed. A later repeat of library tests awaits separate console-owner test-module wiring, outside this acceptance. See [evidence](evidence.md). Existing concurrent console/terminal work is preserved.
