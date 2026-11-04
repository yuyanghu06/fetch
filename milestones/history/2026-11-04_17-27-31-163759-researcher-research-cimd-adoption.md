# Research cimd adoption

- Local timestamp / acting role / milestone: 2026-11-04T17:27:31.163759-05:00 / researcher, serialized by Orchestrator / client-identity research.
- Requested scope: Research whether public client identity JSON is common across MCP services.
- Starting repository state: Public Fetch identity and temporary IP callback are implemented and accepted; other concurrent terminal work remains separate. Preserve all existing source and history.
- Work performed / changed files: Dedicated read-only Researcher compared current MCP specification, hosted-provider documentation and SDK support. Verified Vercel, monday.com and Sumo Logic support; Linear documents older registration. No evidence supports a claim that most servers already require CIMD.
- Decisions: Distinguish a recommended standard, SDK implementation support and provider adoption. Public app information is separate from account credentials and service consent. Do not claim an adoption percentage or interpret missing documentation as proof of no support.
- Checks and evidence: Current primary sources linked in the research document. Parent corroborated the core provider/spec/SDK findings. No tests, builds, app launches, MCP connections, account access or configuration changes were needed.
- Resulting state / acceptance: Research complete; no files, settings, accounts or implementation changed by the specialist.
- Remaining work and handoff: Explain the qualified conclusion in plain English with sources. Actual provider acceptance still needs its own appropriately isolated runtime evidence; this research does not establish it.
