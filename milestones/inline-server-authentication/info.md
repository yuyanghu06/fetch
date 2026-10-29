# Inline authentication, Railway sign-in and request diagnostics

Goal: Authenticate performs real supported setup and displays progress, manual
credential entry, completion or a precise failure on its server card.

Implemented source: dedicated guarded inline workflow; shared masked Settings
editor; Railway Fetch-owned DCR/public PKCE; D80 numeric-loopback callback with
original ten-minute deadline/one-use/cleanup; encrypted stored-token /oauth/me
verification; private CLI5.71.0 setup/repair and token/HOME/PATH handoff; safe
first-party HTTP request/outcome/error logs; independent pending-operation reads.
The official HTTPS root at mcp.railway.com is a supported exact import profile.
Existing APIs and raw provider data contracts remain unchanged.

Status: scoped source/build accepted by separate Tester and Evaluator. Final
locked binary builds exit0 in4.66s (preceding integrated build19.64s). No tests,
app/provider/MCP/native/client flow or actual runtime/package setup run. Runtime
completion therefore remains pending. Local token refresh needs explicit Restart;
no live child environment replacement or full generic OAuth support is claimed.

The user rejected the earlier UI-only approach; required operational MVP gaps
are recorded in docs/testing/MVP_GAPS.md rather than called approved deferrals.
See the rolling snapshot/history for exact implementation/build evidence.

Scoped code pushed as9def537. Later runtime acceptance remains pending; see the
rolling snapshot for delivery evidence. Unrelated Figma work remains separate.
