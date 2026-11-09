# Native neumorphic UI verification — November 9, 2026

Independent Tester changed only the UI fixture/tests and this evidence folder. Product fixes were made by the implementation writers and Orchestrator.

## Results

- `FETCH_UI_EVIDENCE_DIR=/Users/yuyang/Documents/fetch/milestones/neumorphic-dev-sync/evidence cargo test --locked --lib ui:: -- --nocapture`: **52 passed, 0 failed**, 123 filtered out; final execution 0.61 seconds. Exact test output is in [ui-tests.txt](ui-tests.txt).
- After the final narrow-field wrapping fix, `cargo test --locked --lib ui::neumorphic_tests:: -- --nocapture`: **3 passed, 0 failed**, 172 filtered out, 0.74 seconds. These three affected real-widget scenarios were rerun; the other 49 unchanged checks were not repeated. Exact output is in [final-affected-tests.txt](final-affected-tests.txt).
- `cargo build --locked --bin fetch`: **PASS**, final build after wrapping 6.01 seconds. Existing native Apple API deprecations and unused-code warnings remain; no compile error.
- Sixteen offscreen PNG captures cover Servers, tools, inline Settings, Console, Install, installation popup and Connect at 1248×900 and 760×520, plus actual nested-tools scrolling and minimum-width outer-page scrolling.

## Actual interactions checked

The actual Iced `Desktop::view` widget tree receives pointer presses/releases and wheel events. API tasks returned by `Desktop::update` are dropped without polling or executing them.

- Inspect tools opens only the owning server's tools. The read-only card can reveal its complete original JSON and return to its details.
- Settings opens inside Servers, loads the chosen server's distinct limits, offers only that server's assigned key, and clears the previous key value when switching servers. The second server's key control is reached by actual outer-page scrolling.
- Three tool cards share the first row and the fourth begins a second row. Wheel movement over a tool card advances the main tools viewport while the outer page remains still; rendered pixels change.
- The header's actual round restart control emits `RestartClicked` with normal and Shift modifier state retained. The existing force-restart request body is a source-review check; no restart request was executed.
- A URL-only remote server cannot open a terminal. Console remains hidden until explicitly toggled, and toggles closed again.
- Copy passes the displayed Fetch address into the existing clipboard message. Actual system clipboard writes are not executed.
- Closing an installation popup preserves an active installation; the actual View installation progress control reopens it at both window sizes.
- Existing authentication ownership, credential-value clearing, terminal sessions/output, terminal pointer/selection and refresh tests also pass.

## Corrections found during testing

The first run found that remote-terminal selection happened before its remote guard and that each tool card's extra scrollable consumed the tools-area wheel gesture. Writers fixed both. The final run passes the original checks. Search's initially offscreen key was reached by scrolling, without changing product behavior.

Offscreen buttons keep their current drawing status on each widget instance. The exporter sends `RedrawRequested` to the same instance immediately before drawing it. Earlier exploratory images with all controls appearing disabled are superseded by the current PNGs. Inter's 400/600/700 static faces and Roboto Mono are loaded from the same bundled files as the app; earlier fallback-font images are likewise superseded.

## Limits

No native app window, listener, managed server, provider sign-in, Keychain access, Finder action, shell or real client connection was started. All data comes from disposable fixtures with synthetic keys. Build and offscreen evidence do not establish native operating-system acceptance. The screenshot set is representative, not every possible server value or state combination.

At minimum width, the three-column tool layout retains tall readable cards and reachable Raw definition controls. The observed narrow-value overflow was corrected with wrapping. The final `tools-minimum-scrolled-760x520.png` shows `list_directory` and `file_information` fully inside their colored value faces over two lines.
