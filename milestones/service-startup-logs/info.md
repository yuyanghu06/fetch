# Startup service logs and rejected replies

Goal: Fetch’s existing console shows startup service output and the actual returned error/rejection, alongside the failed step.

Scope: one private in-memory attempt capture shared by ordinary local/remote startup, recorded setup and remote OAuth. Preserve the beginning up to1MiB/4096lines with explicit truncation notice. Capture local stderr, helper stdout/stderr, nonprotocol stdout as an error, MCP logging notifications and returned MCP/HTTP/OAuth errors. Redact credentials before retention. No successful protocol/tool/token payload dumps, new API route, file retention, provider login implementation or personal-state change. Hosted internal logs are unavailable unless the service sends them.

Completion: independent confined process/HTTP/OAuth fixtures demonstrate precise errors, full prefix retention, limits, isolation, secret removal, native-only visibility, raw forwarding and cancellation cleanup; existing regressions/native build pass; evaluator accepts.

Status: complete.184uniquechecks including15new, final31UIrecheck, locked nativebuild/whitespace and independentEvaluatorACCEPT. [Evidence](evidence.md). Real sign-in/native appearance remain untested.
