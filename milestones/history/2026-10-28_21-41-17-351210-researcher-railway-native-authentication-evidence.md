# Railway native authentication evidence

- Local timestamp / acting role / milestone: 2026-10-28T21:41:17.351210-04:00; researcher; inline-server-authentication / MVP completion.
- Requested scope: determine a real fresh Railway sign-in and private runtime handoff from official documentation/source.
- Starting repository state: Railway imported npm definitions fall through unsupported auth; private PATH lacks the Railway CLI required by its shim.
- Work performed / changed files: read-only official web research; no source edits by specialist. Root serializes this report.
- Decisions/evidence: Railway supports Fetch-owned public dynamic client registration, S256 PKCE, refresh and authenticated /oauth/me sub verification. CLI token handoff uses RAILWAY_API_TOKEN. Native registration documentation requires numeric HTTP loopback or custom scheme, so existing HTTPS localhost callback is not proven usable. User choice for a temporary callback listener requested; do not claim working before resolution.
- Checks: official docs https://docs.railway.com/integrations/oauth/quickstart and https://docs.railway.com/integrations/oauth/creating-an-app; official CLI v5.71.0/package source. No live provider registration/login, MCP, runtime/package or credentials actions.
- Resulting state / acceptance: concrete implementation evidence; public registration fields not demonstrated live. User architecture decision on callback remains pending.
- Remaining work: implement accepted callback, source-review/compiler check and later actual isolated sign-in proof. CLI env token refresh requires explicit process restart until selected further mechanism.
