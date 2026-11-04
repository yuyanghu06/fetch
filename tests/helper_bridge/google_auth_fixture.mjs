// Synthetic helper interface only. No Google SDK, callback listener or network.
import * as fs from 'node:fs/promises';
import path from 'node:path';
import http from 'node:http';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const { google } = require('googleapis');
const directory = path.join(process.env.XDG_CONFIG_HOME, 'google-docs-mcp', process.env.GOOGLE_MCP_PROFILE);
const tokenPath = path.join(directory, 'token.json');

export async function runAuthFlow() {
  if (process.env.GOOGLE_CLIENT_ID === 'page-only') {
    process.stderr.write('Disposable browser success page shown before credential save.\n');
    return;
  }
  if (process.env.GOOGLE_CLIENT_ID === 'wait') {
    await fs.writeFile(path.join(process.env.HOME, 'waiting-pid'), String(process.pid));
    await new Promise(() => { setInterval(() => {}, 1000); });
  }
  await fs.mkdir(directory, { recursive:true });
  // The installed1.11.3 helper also persists only its refresh token.
  await fs.writeFile(tokenPath, JSON.stringify({refresh_token:process.env.GOOGLE_CLIENT_ID + '-fresh-refresh'}));
}

export async function authorize() {
  try {
    const saved = JSON.parse(await fs.readFile(tokenPath, 'utf8'));
    const client = new google.auth.OAuth2(process.env.GOOGLE_CLIENT_ID, process.env.GOOGLE_CLIENT_SECRET);
    client.setCredentials(saved);
    return client;
  } catch {
    // Both attempted fallback operations must be blocked in noninteractive mode.
    await fs.readFile(path.resolve(import.meta.dirname, '../credentials.json'), 'utf8');
    http.createServer(() => {}).listen(0);
    throw new Error('The fixture must not silently start fallback login.');
  }
}
