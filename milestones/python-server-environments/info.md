# Python server environments

Goal: stop forcing Python package MCP servers onto Python 3.13.18. Use uv's own persistent tool installation, with separate dependencies per managed installation and existing private account/storage boundaries.

Scope: shared uvx import/direct parsing, optional explicit --python forwarding, private uv tool installation and executable-folder metadata. Preserve package pins, supported server arguments, resolved installed versions, old saved environments and existing process/account boundaries. No API/UI schema, login, credential migration or provider-specific behavior. Existing standalone local-script handling is outside this package-installation change.

Completion: focused actual import/preparation checks prove default/explicit Python requests, separate installs, installed metadata/command and legacy helper lookup; native build and independent Evaluator acceptance. Real Blender connection is separate and must not run against personal resources. Status: scoped implementation and cross-Mac setup documentation complete.32 focused checks and signed native build pass; independent Evaluator ACCEPT. Actual Blender installation/connection remains unverified because the restricted experiment stopped before execution. See [evidence](evidence.md) and [setup guide](../../docs/testing/PYTHON_MCP_SETUP.md).
