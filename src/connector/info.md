# Local client connector

`ConnectionContext` carries the client and optional registered project through
CLI --project, native activation, pending proof and profile save/reconnect.
Project profiles use separate keys/locks; existing global keys remain unchanged.
All transports and refresh use the same exact saved resource. Returned binding
is checked before saving; no global credentials are copied into project profiles.

This folder implements Fetch's client-side stdio command. It forwards raw MCP
messages to the local HTTPS backend using that installation's public certificate.
When no approved connection is saved, startup opens or focuses Fetch and requests
its native permission popup. It never owns Fetch or shared-server shutdown.

`profile.rs` keeps connector-held approved client grants in a separate Keychain
namespace. Its file locks coordinate rotating refresh across processes. No
provider or existing coding-client credentials are imported. Explicit fixture
backends allow tests without reading the user's Keychain.

The same saved profile can contain a private pending request proof. It survives
coding-client startup timeout; only the native Fetch app claims the approved
credentials and saves them. Creation coalesces concurrent startup requests under
a short lock. No lock remains held during the person's approval wait. Matching
expiry, denial or missing backend requests clear only that pending proof while
preserving any previously saved grant.

`mod.rs` owns raw forwarding, session cleanup and the explicit setup helper. The
native Fetch window owns human permission approval through the common APIs.
The optional explicit setup command shares automatic startup's path; it never
claims tokens itself. Normal connector output is MCP only; human diagnostics use
standard error. Initialization is forwarded unchanged after approval, without a
fabricated handshake to conceal a client startup timeout.

Backend RPC errors with no/null request ID are reported with their actual sanitized code/message/details and HTTP/method context. They are not mistaken for another connection’s response. Unknown non-null response IDs still fail ownership checks; no unsupported notification is silently accepted.

Modern subscription notifications restore only the protocol-owned
`io.modelcontextprotocol/subscriptionId` in notification parameters and final
subscription result metadata. They use the active subscription request mapping;
acknowledgements keep it alive, and foreign references remain errors. Raw tool
results, structured content and other extension metadata are not rewritten.

Each outgoing request has its own cancellation token, separate from the legacy
dispatch-receipt signal. A modern stdio cancellation closes only that request's
HTTP stream and retires its mappings. It is never posted as an unsupported modern
HTTP notification, and it does not close the connector, replay calls or stop
other connections. Legacy cancellation keeps its existing receipt-before-POST
behavior.
