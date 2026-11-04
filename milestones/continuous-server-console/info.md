# Permanent expanded server console

Every expanded server shows the shared console and retains service output from startup through authentication during the current Fetch run. Display is passive; history survives Authenticate, manual save, collapse and window reopen.

Only sanitized lines and observed exit facts carry into fresh capture handles. Existing memory limits and native-only visibility remain. A new request error stays visible beside older logs until new service output arrives.

Status: complete. Independent 150 relevant tests, including eight new checks, native build and whitespace pass; Evaluator ACCEPT. See [evidence](evidence.md). Authentication, issuer checks, account folders and credential storage are unchanged. No Codex code imported.

Native appearance and real sign-in remain untested. A separate failed explicit Restart → Authenticate SERVER_BUSY observation remains open.
