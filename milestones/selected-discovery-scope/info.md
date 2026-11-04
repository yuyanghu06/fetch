# Selected discovery folder

Goal: choosing `.codex` reads only its `config.toml` and lists only that source's servers.

Scope: remove the parent-folder expansion through the existing shared discovery reader. Preserve omitted-path automatic discovery, explicit home-folder scans, explicit single files, read limits and grouped failures. No installation, authentication or personal configuration changes.

Completion: focused HTTP discovery fixtures demonstrate the selected folder excludes neighboring Claude/OpenCode sources and retains installed flags; native build and independent review pass.

Status: Planner accepted; independent Tester reproduced the original HTTP bug (three groups returned, one expected). Existing tests/backend.rs fixtures now express the requested behavior. Source fix and current descriptions are implemented; whitespace check passes. Independent final HTTP checks pass 10/10 and native build passes (7.55s). Independent Evaluator ACCEPT with no findings; scoped implementation/checks complete. No native app was launched or signed in this pass. Next user action: explicitly Quit Fetch, run `./scripts/run-development.sh`, then refresh discovery. Native window observation remains separate.
