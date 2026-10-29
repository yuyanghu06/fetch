# Documentation guide

[FIGMA.md](FIGMA.md) records Fetch’s official, user-selected Figma project,
the 17 corrected current-UI copies, screenshot reference, shared components and
prototype coverage. Earlier styled proposals are archived as rejected; current
copies remain separate from runtime acceptance and have documented font limits.

[SHARED_UI_COMPONENTS.md](SHARED_UI_COMPONENTS.md) defines the seven shared
component families and the user's requirement to reuse their equivalents in code.

[UI_STATE_COVERAGE.md](UI_STATE_COVERAGE.md) indexes 153 current UI states and
specimens, including authentication, individual Settings, setup/browser errors,
control interactions and explicitly unobserved native contracts. The shared
credential editor and inline-auth families extend the initial component registry.

[architecture/README.md](architecture/README.md) is the design index. [The single implementation checkpoint](architecture/build-readiness.md) contains locked scope, five build stages, one engineering worklist and the custom GitHub CLI scope question. [Decisions](architecture/decisions.md) remain canonical; older drafts/history do not reopen selected subjects.

[templates/](templates/info.md) contains the pass-history template. Actual pass logs live under the project's milestones directory. This folder holds design/evidence, not runtime source, credentials or proof that the app works.

The testing/ folder holds the explicit D77 source handoff. Its commands are instructions for later user-started work; none was executed by this implementation pass.
