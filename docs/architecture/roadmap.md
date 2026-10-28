# Fetch first-release roadmap

October28, 2026. The current source of truth is [the single implementation checkpoint](build-readiness.md), selected by D73. Do not maintain another overlapping roadmap or approval backlog here.

The build order is Rust foundation/authority → server management → MCP runner/sharing → native API-backed interface/client setup → isolated acceptance and MIT-licensed drag-and-drop DMG release. Each API is verified through actual HTTP before its corresponding interface; source work/tests/release require their existing authorization.

Use [confirmed decisions](decisions.md) for locked behavior, [the checkpoint index](open-questions.md) for the one remaining product-scope question and [milestone snapshot](../../milestones/milestone.md) for actual progress/evidence. Prior roadmap proposals remain in immutable pass history; they do not add features or reopen settled choices. Shared memory is deferred from this first release, not removed from the future Fetch product.
