# Managed servers

`mod.rs` owns current server state, configuration changes, provider authentication and named restart/reconnect operations. UI and management-tool adapters call these same backend handlers. It overlays live process/queue/authentication state onto saved non-secret records; status GETs do not probe providers or unlock secret storage.

Manual credentials require a native Settings capability bound to the exact batch and current config/credential state. They are encrypted, read back and checked through a supported provider endpoint before a successful response. Unknown integrations remain unknown; tool-list readiness is never substituted for provider authentication.

Source includes Tavily/GitHub authenticated reads and Linear's documented authenticated GraphQL viewer read, plus metadata-driven Linear OAuth with PKCE and owned encrypted refresh state. [Linear MCP](https://linear.app/docs/mcp), [Linear API](https://linear.app/developers/graphql) and the existing architecture/provider research inform these paths. No provider authentication or test was performed during implementation.

The required verification gate has no unchecked-success exception. A single new effective assigned manual credential can succeed only after its readback passes the provider check; unsupported, unreferenced, multi-entry, removal-only or OAuth-shadowed manual changes cannot borrow a previous credential's successful check. They can commit encrypted storage and still return `CREDENTIAL_VERIFICATION_FAILED`, with explicit saved-state guidance. This preserves the unresolved edge-case boundary rather than claiming broad integration support.

Stdio servers without an explicit saved working directory now use their private
server home as both the default HOME and working directory, keeping relative
writes out of the launch checkout. Explicit imported working directories are
preserved. The home follows the selected development/release application root.
This fallback change is source-reviewed, not runtime-verified.

Railway support is implemented in railway.rs, provider_callback.rs and the shared OAuth handlers:
Fetch-owned DCR/public PKCE, encrypted registration and rotating grant ownership,
stored-token /oauth/me verification, and D80's temporary numeric-loopback native
callback. The same Authenticate operation is used by UI and MCP; safe outcome
logging is in the shared handler. Legacy HTTPS callback remains for Linear.

Railway stdio gets only Fetch's verified access token as RAILWAY_API_TOKEN, an
authoritative private HOME and private CLI/Node PATH. Automatic CLI/skill updates
and telemetry are disabled. No personal Railway login/config is read or written.
An existing child needs explicit Restart after token refresh; the server exposes
restart_required rather than claiming a live environment update.

## October 29 generic remote authentication source

`remote_auth.rs` implements the published MCP protected-resource/issuer discovery,
public client registration, PKCE browser flow and encrypted renewal context using
the existing HTTP library. It follows the official [Rust SDK authentication
design](https://github.com/modelcontextprotocol/rust-sdk/blob/main/docs/OAUTH_SUPPORT.md)
and [MCP authorization contract](https://modelcontextprotocol.io/specification/2025-11-25/basic/authorization);
no SDK protocol models replace the raw runner and no dependency was added.

Each metadata, registration and token request resolves its destination, rejects
private/special addresses, pins the checked answers on the request client, uses
normal HTTPS verification, ignores inherited proxies and refuses redirects.
Published metadata must match its advertised issuer and the imported resource.
Refresh must match the encrypted registration, issuer, resource, client and
token endpoint; interrupted generic rotating refresh is not replayed. Browser
authorization uses the existing HTTPS callback. A provider that rejects that
redirect remains unsupported; the Railway-only numeric callback is not expanded.

Generic issuers must publish `authorization_response_iss_parameter_supported: true`.
Their exact published issuer is kept in the encrypted metadata, and the callback's
`iss` must equal it before Fetch sends an authorization code or PKCE verifier to
the token endpoint. Missing/wrong issuer responses fail without an exchange; there
is no downgrade for providers lacking that mix-up protection. Existing named
Linear/Railway callback policy is unchanged.

The [official SDK security advisory](https://github.com/modelcontextprotocol/rust-sdk/security/advisories/GHSA-5p52-q7pq-9fq2)
informed DNS pinning and redirect refusal. The implementation does not invent
legacy authorization/token/registration URLs when metadata is absent. Seven pure
fixtures cover metadata provenance, resource/query binding, public-address policy,
well-known URLs, challenge parsing and saved refresh context. Root coordinates
their execution separately; writing these fixtures is not a passing result.

Generic grant storage is distinct from authentication proof. Under the current
D25/D31 contract it still cannot report authenticated without an approved provider
verification adapter. The two pending user decisions about saved-state success
and protected-MCP proof have not been implemented. Existing credential batches,
removal and unknown local setup limitations therefore remain explicit.

## Recorded inventory support, without personal configuration access

These rows describe source coverage from the previously redacted inventory, not
successful installation, sign-in or operation. No everyday credentials were read
or copied for this implementation.

| Recorded servers | Current setup/ownership boundary |
| --- | --- |
| Everything, Filesystem, Time test references | Explicit no-provider-auth profiles; isolated installation/tool checks still required. |
| Tavily, GitHub | Manual API credential assignment plus documented authenticated service read; fresh encrypted-save verification still required. |
| Linear, Railway | Fetch-owned browser login, encrypted tokens and provider verification; Railway gets its private CLI context. Actual provider consent and reuse remain unrun. |
| PostHog, Calendly, Miro, Hugging Face, v0, hosted Supabase | General remote metadata/registration/login/refresh source is available where published metadata and Fetch's HTTPS callback are supported. No universal account-check endpoint is assumed; unknown verification remains an explicit blocker under D25. |
| Vercel | Import/general remote source does not establish Vercel's required client approval. Fetch must not impersonate an approved client. |
| Apple Calendar/Notes/Reminders, iMessage, Blender, Premiere | Native app/OS permission and wrapper setup must be verified in an isolated desktop. Current unknown local classification is not proof of no authentication or working native access. |
| Four Google accounts, Slack, GHL, Pinecone | Each local wrapper's credential ownership, fresh setup, private handoff and verification still need a documented integration. No universal export of wrapper-held tokens is available. |
| Photopea, rea, ccmux, node_repl and other custom/local Supabase entry | Existing script/bridge/process setup is source-specific; no Fetch-owned provider adapter or verified no-auth claim has been invented. |

Read [the remaining-work record](../../docs/testing/MVP_GAPS.md) for operational
acceptance and follow-ups. The Railway callback now uses the shared first-party
HTML message helper while preserving its response texts, statuses and headers.
