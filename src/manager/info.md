# Managed servers

`mod.rs` owns current server state, configuration changes, provider authentication and named restart/reconnect operations. UI and management-tool adapters call these same backend handlers. It overlays live process/queue/authentication state onto saved non-secret records; status GETs do not probe providers or unlock secret storage.

Manual credentials require a native Settings capability bound to the exact batch and current config/credential state. They are encrypted, read back and checked through a supported provider endpoint before a successful response. Unknown integrations remain unknown; tool-list readiness is never substituted for provider authentication.

Source includes Tavily/GitHub authenticated reads and Linear's documented authenticated GraphQL viewer read, plus metadata-driven Linear OAuth with PKCE and owned encrypted refresh state. [Linear MCP](https://linear.app/docs/mcp), [Linear API](https://linear.app/developers/graphql) and the existing architecture/provider research inform these paths. No provider authentication or test was performed during implementation.

The required verification gate has no unchecked-success exception. A single new effective assigned manual credential can succeed only after its readback passes the provider check; unsupported, unreferenced, multi-entry, removal-only or OAuth-shadowed manual changes cannot borrow a previous credential's successful check. They can commit encrypted storage and still return `CREDENTIAL_VERIFICATION_FAILED`, with explicit saved-state guidance. This preserves the unresolved edge-case boundary rather than claiming broad integration support.
