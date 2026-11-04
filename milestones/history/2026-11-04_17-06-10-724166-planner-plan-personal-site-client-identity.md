# Plan personal site client identity

- Local timestamp / acting role / milestone: 2026-11-04T17:06:10.724166-05:00 / planner, serialized by Orchestrator / client-identity.
- Requested scope: Publish identity on user-selected personal site and configure rmcp; user adds127.0.0.1 callback and explicitly selects recommended temporaryHTTP receiver.
- Starting repository state: Accepted earlier auth-wrapper removals, substantial concurrent source/docs preserved. Site dev checkout dirty; isolatedproductionworktreeaf22b2c prepared.
- Work performed / changed files: Read site instructions/production and plan static public JSON deployment from isolated production branch, not dirty dev. Canonicalwww avoids308 redirect.
- Decisions: Fixed public identity https://www.yuyang-hu.org/oauth-client.json; publiccallback http://127.0.0.1/oauth/provider/callback with standardportexception; actualcallback ephemeralport. No localhostcertmigration or mainAPIchange, newproviderlogic or secrets in JSON. User requires focusedrealfeaturetests.
- Checks and evidence: No source edits by Planner; accepted bounded website plan. Official primaryrefs: https://www.rfc-editor.org/rfc/rfc8252.html#section-7.3 , https://datatracker.ietf.org/doc/html/draft-ietf-oauth-client-id-metadata-document#section-4.2 , https://vercel.com/docs/agent-resources/vercel-mcp#client-identification-with-cimd .
- Resulting state / acceptance: Website candidate ready for independent review; loopbacksource/test implementation underway. No liveVercellogin or Fetchlaunch.
- Remaining work and handoff: Focused featurecheck/build/evaluation, site authorizedproductionpublication/publicGETcheck, finalcurrentrecords. Preserve other owners’ code and personalfiles/settings.
