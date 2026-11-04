# Preserve normal command-search folders

Goal: commands such as gh work by name in managed MCP startup and its system-shell terminal. User approved keeping the normal command-search folders after server-private program folders on November4.

Scope: use the existing inherited PATH from Fetch's environment, retaining private executable folders first, existing executable-parent lookup, explicit assigned PATH and system fallback when inherited PATH is absent. Normal startup, recorded setup and terminal share the same transport builder. Preserve private HOME/XDG/account folders and service-owned credentials. No login-shell/profile execution, guessed command, provider change, new API/dependency or credential access.

Completion: bounded plan/source change, a few realistic confined startup/terminal/helper checks, relevant existing regressions, native build and independent review. No app relaunch or personal gh/login execution. Root owns journal/docs; specialists have exclusive source/test ownership.

Status: complete scoped source/verification.23focused checks1new, native build8.40s and independent EvaluatorACCEPT. See evidence.md. Rebuilt native terminal and real login remain unobserved; close old terminal/relaunch Fetch before checking.
