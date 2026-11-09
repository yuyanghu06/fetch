# Open server panels explicitly

Goal: show server tools/output when requested and make selected controls look pressed into the surface.

Scope: new neumorphic Figma app/component pages in official file OcgOlmfuoSTJMHTeN753TF. No Rust, backend, original reference or scrolling redesign.

Selected behavior: Inspect tools opens Available tools; Console stays hidden until Console is clicked; Terminal opens its own pane. Clicking the selected control closes the demonstrated panel. Selected Inspect tools, Settings, Console and Terminal use a shared inset state with no glow. Normal round restart replaces the recovery restart buttons; Shift-click means force restart in the later application implementation.

Implementation: closed expanded state 170:2698, existing Tools 78:1438/Console 78:1705/Terminal 78:1752 and 78:1767, plus state-preserving Console-open counterparts 170:2818/170:2918/170:3044. Shared selected Button 172:1910; review 172:3525. Both Terminal masters fill the 1072px card content width. Four new window specimens are added; existing source-reference windows remain separate.

Completion criteria: actual prototype clicks show only explicitly opened panels; selected styles have inner shadows and no glow; redundant recovery buttons are gone; terminal right edges align; independent review accepts.

Status: Figma implementation and independent real-click verification PASS. Selected label binding was corrected and retested. Independent final review ACCEPT. [Review](../history/2026-11-08_19-10-36-942953-evaluator-accept-explicit-server-panels.md). [Test evidence](../history/2026-11-08_18-58-56-541892-tester-pass-explicit-panel-clicks.md).

Limit: Figma's click trigger does not distinguish Shift-click. Component guidance records the requested force restart behavior; it is not executable or verified in this prototype. No Fetch source/runtime change is claimed.

Existing Settings prototype navigation remains illustrative (selected server: search); the shared Settings selected appearance is demonstrated in the component review.
