"""Synthetic startup output only; executed through confinement.rs."""
import json
import os
from pathlib import Path
import sys
import time

home = Path(sys.argv[1])
home.mkdir(parents=True, exist_ok=True)
(home / "peer.pid").write_text(str(os.getpid()))
mode = sys.argv[2]
for index in range(80):
    print("startup history line %03d" % index, file=sys.stderr, flush=True)
print("assigned=" + os.environ.get("FIXTURE_LOG_SECRET", "not-assigned"), file=sys.stderr, flush=True)
if mode == "helper":
    print("setup stdout final useful reason", flush=True)
    print("setup stderr final useful reason", file=sys.stderr, flush=True)
    sys.exit(9)
if mode == "fixture-invalid-stdout-mode":
    print("invalid startup stdout diagnostic", flush=True)
    time.sleep(0.05)
    sys.exit(6)

for line in sys.stdin:
    request = json.loads(line)
    if "id" not in request:
        continue
    method = request.get("method")
    if method == "initialize":
        print(json.dumps({"jsonrpc":"2.0","method":"notifications/message","params":{
            "level":"warning","logger":"fixture","data":{"reason":"service log before ready","access_token":"nested-private-token"}}}), flush=True)
        result = {"protocolVersion":"2025-11-25","capabilities":{"tools":{}},
                  "serverInfo":{"name":"startup-fixture","version":"1"},
                  "instructions":"Not logged in; run railway login (synthetic fixture)",
                  "successful_payload":"successful-frame-must-stay-private"}
    elif method == "tools/list":
        if mode == "fixture-tools-reject-mode":
            print(json.dumps({"jsonrpc":"2.0","id":request["id"],"error":{
                "code":-32001,"message":"Not logged in; run railway login",
                "data":{"reason":"original service rejection","cookie":"private-cookie-value"}}}), flush=True)
            continue
        result = {"tools":[{"name":"echo","inputSchema":{"type":"object"}}]}
    elif method == "tools/call":
        print("post-ready-output-must-stay-private", file=sys.stderr, flush=True)
        result = {"content":[{"type":"text","text":"successful-tool-result-must-stay-private"}]}
    else:
        result = {}
    print(json.dumps({"jsonrpc":"2.0","id":request["id"],"result":result}), flush=True)
