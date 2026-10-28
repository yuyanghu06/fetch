# Architecture folder

Updated October28, 2026 after D73. Read [README](README.md), [decisions](decisions.md), [existing API contracts](api-contract.md) and the **[single implementation checkpoint](build-readiness.md)**. [Open questions](open-questions.md) is its short index, not a second backlog.

One agent credential/token and `/mcp` connection, user-completed provider login reused on calls, Rust/execution/raw protocol/auth/Settings direction, no memory in this release, MIT and drag-and-drop DMG are selected. Unchosen engineering details are gathered in one worklist; new meaningful behavior is reviewed completely once, not as another broad subject.

The custom GitHub CLI adapter is the one remaining product-scope question. [The verification amendment](credential-verification.md) remains an unaccepted historical draft withdrawn from active review; no storage-only200 exception is selected. Settings mechanism candidates and actual client/provider compatibility remain honest engineering/evidence work.

No application/runtime/Keychain/client/certificate/credential/environment/test/release operation follows from this documentation. Preserve D46 isolation, D25/D31 success gate, raw values and API-first backend authority. [Milestone snapshot](../../milestones/milestone.md) links actual checks and append-only history.

The pre-build MCP source-session audit restores existing requirements and records source coverage; it adds no product scope or implementation permission. Refer to the audit through the architecture index/current snapshot.

The original-session MCP audit is independently accepted. Protected existing content and prior journal entries passed integrity checks; the snapshot records final documentation checks and audit pass links.

D74 authorizes implementation/push without release. developer-foundation.md records the first code increment and its exact limits; selected architecture behavior is not all implemented yet.
