# Terminal selection and scrolling evidence

November4,2026. Independent Evaluator ACCEPT.10focused terminalUI tests pass, including2new realistic interaction/render scenarios. Native build8.30s and whitespace pass.

## Original reproduction

`cargo test --locked --lib ui::server_terminal::tests::terminal_ -- --nocapture` on original widget:3selected,1existingpass/2newfail. Outside release/re-entry still emitted SelectUpdate; fractional trackpad wheel was not captured. Root/Evaluator inspected /private/tmp/fetch-terminal-selection-scroll-before.log. This is deliberate before-fix evidence, not an unresolved final failure.

Primary local source confirms pixel sign: Icedwinit conversion preserves line/pixel y; Iced Scrollable uses -y for top-origin page offset, Alacritty positive Scroll::Delta reveals older history. Original widget subtracted pixel y while line deltas retained the sign.

## Narrow correction

Only vendor/iced_term/src/view.rs changes production behavior: active left-button release outside visible pane ends drag; existing mouse-mode release is relayed without opening links outside. Visible hit region intersects viewport. Pixel delta accumulates positive y, matching line wheel; wheel events over pane are captured even before enough pixels accumulate for one terminal line. Intentional selection and white text cursor appearance remain.

Two existing vendored cfg(test) cases also have their obsolete inverted sign/name/comment expectations corrected. No additional cases, source parser, process/API/authentication/PATH or dependencies changed.

## Final checks

- `cargo test --locked --lib ui::server_terminal::tests -- --nocapture`:10passed,0failed (2new). Actual TerminalView mouse events feed existing external Terminal/parser/renderer. Deliberate drag visibly selects; outside release/re-entry produces no SelectUpdate and exactly unchanged selected pixels. Fractional trackpad gesture stays captured with unchanged screen until a full line; positive movement reveals older red output, empty output polling leaves those pixels stable, negative movement returns toward latest output. Existing embedded prompt/cursor/movement/clipping and session/state tests pass.
- `cargo build --locked --bin fetch`:PASS8.30s; existing warnings remain. The later edit only aligns vendored cfg(test) assertions and does not alter the built production behavior; no extra rebuild/suite claimed for that cleanup.
- Scoped/final `git diff --check`:PASS.

Root/Evaluator inspected /private/tmp/fetch-terminal-selection-scroll-after.log and fetch-terminal-selection-scroll-build.log. Headless GPU rendering only: no native window, shell, MCP/provider, personal credentials or task fixture processes/folders involved. No real user app killed/relaunched.

## Remaining native observation

Explicitly Quit Fetch, run ./scripts/run-development.sh and check the new terminal window. Actual user's rebuilt native interaction is not observed by these tests. Normal deliberate selection continues to highlight text; the correction prevents it continuing to grow after mouse release. No new cursor appearance is selected.
