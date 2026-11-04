# Research ip loopback callback

- Local timestamp / acting role / milestone: 2026-11-04T17:06:10.724169-05:00 / researcher, serialized by Orchestrator / client-identity.
- Requested scope: Publish identity on user-selected personal site and configure rmcp; user adds127.0.0.1 callback and explicitly selects recommended temporaryHTTP receiver.
- Starting repository state: Accepted earlier auth-wrapper removals, substantial concurrent source/docs preserved. Site dev checkout dirty; isolatedproductionworktreeaf22b2c prepared.
- Work performed / changed files: Inspected certificate/boundary and pinned SDK; SDK has no production listener. OfficialRFC8252§7.3/currentCIMD§4.2 support ephemeralHTTPIPloopback and portless registered path; Vercel docs permitHTTPloopback.
- Decisions: Fixed public identity https://www.yuyang-hu.org/oauth-client.json; publiccallback http://127.0.0.1/oauth/provider/callback with standardportexception; actualcallback ephemeralport. No localhostcertmigration or mainAPIchange, newproviderlogic or secrets in JSON. User requires focusedrealfeaturetests.
- Checks and evidence: No writes/live tests by Researcher. Exact HTTPS127 substitution would fail existinglocalhostcert/Hostchecks; user explicitly selected temporaryHTTP instead. Official primaryrefs: https://www.rfc-editor.org/rfc/rfc8252.html#section-7.3 , https://datatracker.ietf.org/doc/html/draft-ietf-oauth-client-id-metadata-document#section-4.2 , https://vercel.com/docs/agent-resources/vercel-mcp#client-identification-with-cimd .
- Resulting state / acceptance: Website candidate ready for independent review; loopbacksource/test implementation underway. No liveVercellogin or Fetchlaunch.
- Remaining work and handoff: Focused featurecheck/build/evaluation, site authorizedproductionpublication/publicGETcheck, finalcurrentrecords. Preserve other owners’ code and personalfiles/settings.
