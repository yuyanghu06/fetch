# Raised dropdowns and readable session commands

Goal/scope: the user's follow-up asks for outward-raised project dropdowns, blue session buttons, code-block commands and missing assistant replies. Reuse shared native surfaces/buttons/code blocks and existing Workspace API chunk reads. Do not edit source session logs or personal-client state.

Implemented: closed project pickers use raised_with_radius at12px; entry_card permits primary session actions; session_history separates native command/output/note wrappers and marks tool data as code. Workspace reads past initial user/partial records until the next assistant reply or EOF, pauses for Read more, and stops follow-up reads when closed.

Status: source and focused fixture verification complete. Nine unique focused Rust checks pass: five Workspace, three history/parser and one heading/menu. PNG captures inspected, including raised dropdown, blue session row and command/User/Assistant popup. Final signed build passes18.33s. Live saved histories remain user observation after explicit Quit/reopen. Same-agent implementation/review, no independent evaluator for this direct UI pass.

Evidence: evidence/project-picker-test.txt, workspace-command-tests.txt, command-parser-tests.txt, signed-build-retry.txt and PNGs. Earlier signed-build.txt preserves a no-disk-space failure. Regenerable Fetch package archives/incremental caches were removed under Cargo lock; retry used CARGO_INCREMENTAL=0. Existing signed executables and user data were preserved. The old initial session-card test also remains as evidence, superseded by the realistic split-response case.
