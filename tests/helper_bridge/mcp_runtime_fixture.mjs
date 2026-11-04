// Disposable MCP protocol peer used only for ManagedStdio lifecycle verification.
import { authorize } from './auth.js';
import * as fs from 'node:fs/promises';
import path from 'node:path';
import readline from 'node:readline';
const client = await authorize();
await client.getAccessToken();
await fs.writeFile(path.join(process.env.HOME, 'managed-pid'), String(process.pid));
await fs.writeFile(path.join(process.env.HOME, 'managed-socket'), process.env.FETCH_HELPER_SOCKET);
const rawResult = '{ "number":184467440737095516160000001,"content":[{"type":"image","mimeType":"image/png","data":"fixture-base64"}],"future":{"keep":true} }';
for await (const line of readline.createInterface({input:process.stdin})) {
  const message = JSON.parse(line);
  if (message.id === undefined) continue;
  let result;
  if (message.method === 'initialize') {
    result = JSON.stringify({protocolVersion:'2025-11-25',capabilities:{tools:{}},serverInfo:{name:'disposable-helper-peer',version:'1'}});
  } else if (message.method === 'tools/list') {
    result = '{"tools":[{ "name":"echo","inputSchema":{"type":"object"},"future":{"number":184467440737095516160000001} }]}';
  } else if (message.method === 'tools/call') result = rawResult;
  else result = '{}';
  process.stdout.write(`{"jsonrpc":"2.0","id":${JSON.stringify(message.id)},"result":${result}}\n`);
}
