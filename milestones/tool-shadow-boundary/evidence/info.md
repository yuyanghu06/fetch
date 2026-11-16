# Tool edge and frameless rendering evidence

before-shadow-regression.txt shows the existing Tools interaction test failing because the rightmost-card shadow is cut off: the sampled padding pixel is plain [234,230,223]. widget-tests.txt shows all five widget checks passing after the fix. The same test checks three columns, real inner scrolling and preserved outer position. The navigation test now verifies transparent corner pixels, top-bar drag events and tab-click capture without drag. Other existing panel/fixed-section tests remain.

fidelity-tests.txt records two 1x/2x button and full Tools/Settings first-frame checks. signed-build.txt records the successful signed development build/strict verification. The three RGBA screenshots were visually inspected, including normal/scrolled Tools right edges and minimum-window rounded corners. Corrected pixels beyond the right card around its lower edge now fade [220,216,208] → [223,219,211] → [226,221,214] instead of jumping to flat padding.

Offscreen screenshots are not a native compositor/window test. No app launch or Keychain interaction occurred during these checks; the user controls any open password dialog. Running Fetch was preserved.
