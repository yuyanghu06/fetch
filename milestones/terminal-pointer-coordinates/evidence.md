# Scrolled terminal pointer — evidence

November4,2026. Independent Evaluator ACCEPT.11focused terminalUI tests (1new) and native build6.90s/whitespace pass.

## Cause and missed coverage

The previous selection tests placed TerminalView directly in a layout. They checked release outside/re-entry, scrolling and rendered placement, but did not click through an actual scrolled parent page. Iced's Scrollable translates the child Cursor position by its page offset while forwarding the raw CursorMoved event unchanged. Terminal press used that adjusted cursor; movement used raw event position. A tiny movement could therefore jump selection several rows. Primary source inspected: installed iced_widget0.14.2 scrollable.rs790–825.

## Reproduction and correction

One new actual outer Scrollable+TerminalView scenario includes a600pxspacer and500pxpage scroll. Before the fix, a1pxvisible movement emitted selection y=-474 rather than26. The focused original-source regression fails; root/Evaluator inspected /private/tmp/fetch-terminal-scrolled-pointer-before.log. To reproduce that exact case, run `cargo test --locked --lib terminal_click_and_drag_stay_on_the_clicked_row_after_outer_page_scroll -- --nocapture` against the original source.

Only two production lines change in vendor/iced_term/src/view.rs: pattern CursorMoved{..}, then pass &cursor_position to the existing helper. No new selection algorithm, threshold, cursor-style or underlying parser change. Prior release-outside/pixel-wheel corrections stay in place.

## Final verification

- `cargo test --locked --lib ui::server_terminal::tests -- --nocapture`:11passed,0failed (one new actual-parent scenario),0.65stest/7.89scompile. The new test proves tinyclick movement does not highlight other rows; a deliberate shortdrag visibly selects only the intended row; a plainclick clears it. Actual rendered RGBA pixels verify all three results. Existing release/trackpad/history/empty-poll/embedded-render/state/replay cases pass.
- `cargo build --locked --bin fetch`:PASS6.90s; existing warnings remain.
- Scoped/root `git diff --check`:PASS.

Original checked logs: /private/tmp/fetch-terminal-scrolled-pointer-before.log, fetch-terminal-scrolled-pointer-after.log, fetch-terminal-scrolled-pointer-build.log. Root/Evaluator inspected them. No actual native app, shell, provider, credentials or test fixture processes/folders involved; rendering is headless GPU with generic fixture text. User screenshot login/account/device-code contents were not copied.

## Remaining observation

The actual user's rebuilt native window still needs checking after explicitly quitting Fetch and running ./scripts/run-development.sh. This scoped acceptance establishes the reproduced page-scroll click correction, not universal native behavior or provider login acceptance.
