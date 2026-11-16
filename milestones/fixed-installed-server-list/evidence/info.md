# Fixed server list evidence

widget-tests.txt records five actual-widget tests: dedicated server-list scroll/expand behavior at 1248×900 and 760×520, existing per-server controls/key ownership, nested Tools scrolling, page specimens and fast navigation. fidelity-tests.txt records two existing first-frame/1x/2x rendering checks. signed-build.txt records the successful signed development build and launcher strict signature verification.

scrolled-normal.png and scrolled-minimum.png show later cards with navigation and Installed servers title/Refresh fixed. tools-normal.png shows the existing nested tool grid inside a server card, clipped to the server-list viewport as expected. These three actual-widget screenshots were visually inspected. Fixtures use synthetic data and execute no backend tasks; no actual app, credentials, listener or MCP was launched.
