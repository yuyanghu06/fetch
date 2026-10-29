# Current app pages in Figma

Latest expansion: 153 current states/specimens across nine boards, indexed in
[UI_STATE_COVERAGE.md](../../docs/UI_STATE_COVERAGE.md). Covers every inventoried
finite native/browser rendering branch and control class with representative
data, including all nine auth phases and the shared Settings/inline editor.
OS/provider-owned appearance is represented as an explicit source contract.
This does not claim all permutations, exhaustive pixel review or runtime success.

Independent source and representative Figma evaluation accepted after fixes.
The design-authoring scope is complete pending user feedback. Source changes
require future state/index updates; no application changes were made by this pass.
[Latest pass](../history/2026-10-28_21-57-19-222233-orchestrator-complete-current-ui-state-library.md). Older baseline details below are history.

Latest correction, October 28: the user rejected the first styled designs and
requested literal current-app UI. Those original boards are archived. Current
delivery is 13 native and four browser HTML states, using current shared widgets,
actual layout/scrolling and sample data. A captured existing empty Servers window
is the direct visual reference. Other states are reconstructed from source.

Current boards have 59 native prototype links. Long pages intentionally scroll;
they are not redesigned to make all content fit one viewport. The 1080×760
native client area is represented with an additional 32-pixel title bar.
Native SF Pro is unavailable in Figma: calibrated editable Arimo and retained
source-size metadata are documented. No pixel-identical font claim is made.
See [current evidence](../../docs/FIGMA.md) and
[correction record](../history/2026-10-28_21-17-10-000001-orchestrator-literal-current-ui-correction.md).
Older status below describes the superseded first pass.

Goal: build Figma versions of every current Fetch page in the official file,
including populated in-use examples and shared components for repeated parts.

Scope: native Servers, Install MCPs, Connect apps and Settings plus browser
authorization/provider callback pages. Source is src/ui/mod.rs, src/oauth/mod.rs
and src/api/runtime.rs. No new product API or application implementation.

Completion criteria: editable frames for each current page, important populated,
empty/busy/error states, repeated elements as linked component instances, useful
native navigation links and screenshot inspection for layout problems.

Status: design-authoring criteria complete. Seventeen frames exist across native
and browser boards; a component page holds seven families. All native content
bounds fit 1080×760 frames, with intentional internal Install-list scrolling.
Screenshots were inspected; this was one agent's design inspection, not an
independent evaluation or app test. User visual review remains open.

Follow-up requirement: commonly reused elements must also be shared components
in code. The seven families have responsibility/input definitions in
[the registry](../../docs/SHARED_UI_COMPONENTS.md) and descriptions in Figma.
AGENTS.md records the rule for future agents. Shared code extraction remains
pending; the follow-up changed documentation and Figma descriptions only.

See [inventory and links](../../docs/FIGMA.md) and
[pass evidence](../history/2026-10-28_20-46-58-000001-orchestrator-complete-figma-pages.md).
