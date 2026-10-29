# Local MVP verification evidence

These records identify the commands, expected behavior, observed results and
limits of the October 29 D81 verification. Keep credentials, raw browser state
and private provider responses out of this folder. Fixture evidence is separate
from actual native/provider/client acceptance.

[Fixture checks](fixture-checks-2026-10-29.md) records independent Rust library,
in-process API/MCP, import, path, raw-data and disposable localhost TLS checks.
The first run includes a failing import-test assertion and a successful targeted
retest after its author's correction. A subsequent combined rerun after the
issuer-response fix passed all 68 selected cases. These fixture results remain
separate from operational-MVP proof and real provider/native/client evidence.

[Offline client preparation](offline-client-2026-10-29.md) records copied Codex
initialization with zero MCPs, disposable confinement canaries and the actual
administrator authentication handoff. It does not prove native Keychain isolation
or any real server/provider connection.
