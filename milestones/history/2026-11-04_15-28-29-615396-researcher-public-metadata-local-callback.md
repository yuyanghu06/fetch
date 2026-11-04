# Public client metadata can use a local callback

- Local timestamp / acting role / milestone: 2026-11-04T15:28:29.615396-05:00 / Orchestrator research clarification / server-console-terminal handoff.
- Requested scope: user asks whether passing app JSON through a URL still permits localhost callback. Specific JSON/URL absent from this message; answer assumes public client-registration metadata document.
- Starting repository state: Console/Terminal scoped implementation/checks/build/review accepted; no new authentication implementation requested here.
- Work performed / changed files: checked official MCP Ruby SDK authorization documentation and RFC8252 sections7.3/8.3. Added research note; no application/config/deployment changes.
- Decisions: explanation only. Public metadata URL identifies/describes the app; registered allowed redirect can return browser to local native app. Provider must support the registration method and callback. Prefer loopback IP127.0.0.1 per RFC rather than assuming literal localhost accepted everywhere. No new route/hosting/callback choice approved by this question.
- Checks and evidence: https://ruby.sdk.modelcontextprotocol.io/client/authorization/ documents HTTPS metadata URL and redirect_uris; https://www.rfc-editor.org/rfc/rfc8252.html documents HTTP loopback native callbacks and IP recommendation. MCP2026 direct spec page was inaccessible through browser tool; no claim it was read.
- Resulting state / acceptance: conditional factual answer verified from primary sources; no provider-specific acceptance tested.
- Remaining work and handoff: specific JSON/URL/provider needed for a concrete configuration review. Existing terminal next action remains explicit Quit/signed development relaunch for rebuilt native observation.
