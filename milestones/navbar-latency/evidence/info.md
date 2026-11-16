# Navigation timing and appearance checks

before-navigation.txt reproduces actual tab presses and Retina decoration draws before the fix. final-ui-tests.txt runs the same navigation check plus existing server-panel, nested-scrolling and normal/minimum-window render checks. fidelity-tests.txt verifies buttons at 1x/2x plus Tools/Settings first-frame rendering. build.txt is the native binary build; signed-build.txt is the signed development build, with strict signature verification performed by the launcher.

pixel-comparison.json compares first-visit 1248×900 screenshots with the original renderer. Servers differs by mean 0.000522 RGB units out of 255; Install differs by 0.006933. Connect includes different random temporary certificate-path text; its whole-page difference is not solely rendering. servers-after.png is the visually inspected resulting synthetic-data screen. Complete raw captures remain disposable under /tmp/fetch-navbar-* and are not product assets.

These are actual native widgets drawn without opening a window or running backend tasks. They do not establish live-app input-to-display timing. Existing running Fetch was preserved. No providers, MCP servers, personal credentials or client configurations were accessed.
