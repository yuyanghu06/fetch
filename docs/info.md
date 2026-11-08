## November 7 visual follow-up

FIGMA.md, SHARED_UI_COMPONENTS.md and UI_STATE_COVERAGE.md describe the current control refinements, five Install popup states, manual-install Console design and tested Rust header-only change. The new popup/manual workflow is a design, not implemented app behavior.

# Documentation guide

[FIGMA.md](FIGMA.md) also links the three separate neumorphic design pages:24native states, copied components and29browser states. The literal current-app references remain separate; Rust is unchanged.

[design-inspiratinos/](design-inspiratinos/info.md) keeps the user’s original neumorphic UI references and selected light-blue accent #95D4F1 for the planned app overhaul. These are design inputs, not implemented screens.


[LOCAL_MVP_RUN.md](testing/LOCAL_MVP_RUN.md) records D81 source delivery,
68 passing fixtures/final binary build, scoped source acceptance and the actual
offline-Codex/native-test-user handoff. Real server installation/provider/client
proof and2 credential choices remain pending. No packaging/signing/DMG work.

[MVP_GAPS.md](testing/MVP_GAPS.md) is the current remaining-work list: five
implementation areas, isolated runtime acceptance and distribution preparation.
It distinguishes code gaps, unrun evidence and explicitly deferred features.

[FIGMA.md](FIGMA.md) records Fetch’s official, user-selected Figma project,
24current native specimens, earlier browser references, shared components and
prototype links. Earlier styled proposals are archived as rejected; current
copies remain separate from runtime acceptance and have documented font limits.

[SHARED_UI_COMPONENTS.md](SHARED_UI_COMPONENTS.md) defines the seven shared
component families and the user's requirement to reuse their equivalents in code.

[UI_STATE_COVERAGE.md](UI_STATE_COVERAGE.md) indexes24current native specimens and preserves the historical153 UI states and
specimens, including authentication, individual Settings, setup/browser errors,
control interactions and explicitly unobserved native contracts. The shared
credential editor and inline-auth families extend the initial component registry.

[architecture/README.md](architecture/README.md) is the design index. [The single implementation checkpoint](architecture/build-readiness.md) contains locked scope, five build stages, one engineering worklist and the custom GitHub CLI scope question. [Decisions](architecture/decisions.md) remain canonical; older drafts/history do not reopen selected subjects.

[templates/](templates/info.md) contains the pass-history template. Actual pass logs live under the project's milestones directory. This folder holds design/evidence, not runtime source, credentials or proof that the app works.

The testing/ folder holds the explicit D77 source handoff. Its commands are instructions for later user-started work; none was executed by this implementation pass.
