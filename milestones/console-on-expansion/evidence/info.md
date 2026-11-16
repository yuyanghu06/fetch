# Console expansion evidence

expanded-error-console.png is an offscreen actual-widget capture at1248×900 using disposable fixture data. Console is automatically selected; captured server output and the returned Fetch error code/message/details share its dark output body; Copy is present and duplicate card messages are absent. Root and independent Tester inspected it. No native application or real provider was launched.

Three focused fixture checks pass: server_expansion_opens_console_with_fetch_errors_and_server_output; neumorphic_server_panels_open_from_their_own_controls_and_keep_key_ownership; console_switch_collapse_and_page_changes_keep_screen_session_and_output_reads. Each ran via cargo test --locked --lib <name> -- --nocapture. First regression run had a test-only Copy text lookup failure, corrected to a real icon click; rerun passes.

./scripts/run-development.sh --build-only passes, including selected local development signature and strict verification. Existing unrelated compiler warnings remain. Native app reload and actual provider error reproduction were not run.
