"""Disposable MCP peer. Owns its ordinary HOME/XDG state; no Fetch secret hooks."""
import json
import os
from pathlib import Path
import sys
import time

home = Path(os.environ['HOME'])
account = json.loads((home / 'account.json').read_text())['account']
(home / 'peer.pid').write_text(str(os.getpid()))
with (home / 'starts.log').open('a') as log:
    log.write(account + '\n')
for name in ('XDG_CONFIG_HOME', 'XDG_DATA_HOME', 'XDG_STATE_HOME'):
    folder = Path(os.environ[name])
    folder.mkdir(parents=True, exist_ok=True)
    (folder / 'native-state.json').write_text(json.dumps({'account': account}))

for line in sys.stdin:
    message = json.loads(line)
    if 'id' not in message:
        continue
    method = message['method']
    if method == 'initialize':
        result = json.dumps({'protocolVersion': '2025-11-25', 'capabilities': {'tools': {}},
                             'serverInfo': {'name': 'disposable-native-state', 'version': '1'}})
    elif method == 'tools/list':
        result = '{"tools":[{ "name":"echo", "inputSchema":{"type":"object"}, "future":{"number":184467440737095516160000001} }]}'
    elif method == 'tools/call':
        (home / 'last-request.json').write_text(line)
        if message.get('params', {}).get('arguments', {}).get('wait'):
            time.sleep(0.6)
        result = ('{ "account":' + json.dumps(account) + ', "number":184467440737095516160000001,'
                  ' "content":[{"type":"image","mimeType":"image/png","data":"fixture-base64"}],'
                  ' "future":{"keep":true} }')
    else:
        result = '{}'
    sys.stdout.write('{"jsonrpc":"2.0","id":' + json.dumps(message['id']) + ',"result":' + result + '}\n')
    sys.stdout.flush()
