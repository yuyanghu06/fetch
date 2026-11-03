# Already-installed discovery rows

Goal: show a previously imported MCP definition as checked and gray during later
discovery, with no option to submit it for installation again from the native UI.

Scope: reuse the installer's exact source configuration path/name association;
backend discovery supplies installed state through the existing HTTP and MCP
surfaces. Keep compatibility status separate, preserve distinct accounts and
existing idempotent installation behavior. Newly successful rows update from
installation progress; failed rows remain retryable. No broader import support.

Completion criteria: source implementation, independent source verification and
evaluation, updated API/UI guides and state coverage, accurate journal. Real
visual/build/runtime acceptance is separately pending under existing restrictions.

Status: source implemented; independent Tester and Evaluator source/documentation
ACCEPT with no material findings. Rebased onto a7a6e0d, re-reviewed ACCEPT and
pushed as9eddd00. The new
installed-row Figma specimen remains unmapped and runtime appearance unobserved.
No builds, tests, formatters,
linters, validation scripts, apps/providers/MCP connections or personal-state
operations are authorized by this UI request. Shared role files are unavailable;
specialists receive explicit roles and file ownership directly.
