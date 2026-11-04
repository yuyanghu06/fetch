"""Disposable server-owned sign-in, confined by confinement.rs before execution."""
import json
import os
from pathlib import Path
import signal
import sys
import time

home = Path(sys.argv[1])
home.mkdir(parents=True, exist_ok=True)
(home / "peer.pid").write_text(str(os.getpid()))
if "FIXTURE_LOGIN_INPUT" in os.environ:
    (home / "login-input.txt").write_text(os.environ["FIXTURE_LOGIN_INPUT"])
with (home / "starts.log").open("a") as starts:
    starts.write(str(os.getpid()) + "\n")

if "--requires-native-config" in sys.argv:
    if os.environ.get("FIXTURE_CLIENT_SECRET"):
        source = "assigned-environment"
    elif (home / "credentials.json").is_file():
        json.loads((home / "credentials.json").read_text())
        source = "native-file"
    else:
        (home / "self-reported-failure").write_text("the server itself rejected missing setup")
        message = "FATAL: this disposable server has no client setup"
        if "--diagnostic-details" in sys.argv:
            message += " assigned=" + os.environ.get("FIXTURE_STARTUP_SECRET", "unset")
            message += " source=https://login.fixture.example/authorize?state=diagnostic-private-state&client_secret=diagnostic-private-url-secret"
        print(message, file=sys.stderr, flush=True)
        sys.exit(3)
    (home / "configuration-source.txt").write_text(source)
    (home / "profile.txt").write_text(os.environ.get("FIXTURE_PROFILE", "default"))

if "--descendant" in sys.argv:
    descendant = os.fork()
    if descendant == 0:
        (home / "descendant.pid").write_text(str(os.getpid()))
        while True:
            time.sleep(0.05)
    else:
        def stop_owned_descendant(_signal, _frame):
            os.waitpid(descendant, 0)
            sys.exit(0)
        signal.signal(signal.SIGTERM, stop_owned_descendant)

if "--wait-for-link" in sys.argv:
    while not (home / "release-link").exists():
        time.sleep(0.02)

url = (
    "https://accounts.fixture.example/oauth/authorize?response_type=code"
    "&client_id=fixture-client&redirect_uri=http%3A%2F%2Flocalhost%3A9876"
    "&state=private-transient-fixture-state"
)
stream = sys.stdout if "--helper" in sys.argv else sys.stderr
print("private fixture credential=never-display-this-secret", file=stream, flush=True)
print("Diagnostic reference https://unrelated.fixture.example/docs", file=stream, flush=True)
print("Authorize this app by visiting this url: " + url, file=stream, flush=True)

while not (home / "login-complete").exists():
    time.sleep(0.02)
(home / "native-login.json").write_text('{"owner":"server","account":"disposable"}')
if "--helper" in sys.argv:
    sys.exit(0)

for line in sys.stdin:
    request = json.loads(line)
    method = request.get("method")
    if "id" not in request:
        continue
    if method == "initialize":
        result = {
            "protocolVersion": "2025-11-25",
            "capabilities": {"tools": {}},
            "serverInfo": {"name": "disposable-login-peer", "version": "1"},
        }
    elif method == "tools/list":
        result = {"tools": [{"name": "echo", "inputSchema": {"type": "object"}}]}
    elif method == "tools/call":
        (home / "last-request.json").write_text(line)
        if request.get("params", {}).get("arguments", {}).get("hold"):
            (home / "call-entered").write_text("owned tool call is running")
            while not (home / "release-call").exists():
                time.sleep(0.02)
        # Exact bytes exercise the raw result path without account verification.
        print('{"jsonrpc":"2.0","id":' + json.dumps(request["id"]) + ',"result":{ "content":[{"type":"text","text":"server-owned login fixture"}], "number":184467440737095516160000001 }}', flush=True)
        continue
    else:
        result = {}
    print(json.dumps({"jsonrpc": "2.0", "id": request["id"], "result": result}), flush=True)
