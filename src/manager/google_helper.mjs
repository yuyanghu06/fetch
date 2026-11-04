// Adapter for the inspected @a-bonus/google-docs-mcp 1.11.3 auth/storage interface.
// The upstream helper retains its callback, state validation and code exchange.
import helperFs from 'node:fs';
import helperPath from 'node:path';
import helperHttp from 'node:http';
import { createRequire as helperCreateRequire, syncBuiltinESMExports as helperSyncBuiltins } from 'node:module';
import { pathToFileURL as helperFileURL } from 'node:url';
import { createHash as helperHash } from 'node:crypto';

const [helperMode, helperEntry] = process.argv.slice(1);
const helperBridge = createBridgeClient();
const helperRequire = helperCreateRequire(helperEntry);
const helperPackageRoot = helperPath.dirname(helperPath.dirname(helperEntry));
const helperTokenDirectory = helperPath.join(process.env.XDG_CONFIG_HOME, 'google-docs-mcp', process.env.GOOGLE_MCP_PROFILE);
const helperTokenPath = helperPath.join(helperTokenDirectory, 'token.json');
const helperCredentialsPath = helperPath.join(helperPackageRoot, 'credentials.json');
const helperPromises = helperFs.promises;
const helperRead = helperPromises.readFile.bind(helperPromises);
const helperWrite = helperPromises.writeFile.bind(helperPromises);
const helperMkdir = helperPromises.mkdir.bind(helperPromises);
let helperOperations = Promise.resolve();
let helperLastFailure;
function helperSerial(operation) {
  const result = helperOperations.then(operation);
  helperOperations = result.catch((error) => { helperLastFailure = error; });
  return result;
}
function helperFailure() { return new Error('Fetch could not safely complete the Google credential operation.'); }
function helperPayload(raw) {
  if (!raw || typeof raw !== 'object' || Array.isArray(raw)) throw helperFailure();
  const value = {};
  for (const field of ['access_token', 'refresh_token', 'scope', 'token_type']) {
    if (raw[field] !== undefined && raw[field] !== null) {
      if (typeof raw[field] !== 'string' || !raw[field] || /[\r\n\0]/.test(raw[field])) throw helperFailure();
      value[field] = raw[field];
    }
  }
  if (raw.expiry_date !== undefined && raw.expiry_date !== null) {
    if (!Number.isFinite(raw.expiry_date) || raw.expiry_date <= 0) throw helperFailure();
    value.expiry_date = raw.expiry_date;
  }
  if (!value.refresh_token && !value.access_token) throw helperFailure();
  return value;
}
helperPromises.readFile = async function (file, ...options) {
  const target = helperPath.resolve(String(file));
  if (target === helperCredentialsPath) throw helperFailure();
  if (target !== helperTokenPath) return helperRead(file, ...options);
  const snapshot = await helperSerial(() => helperBridge.load());
  const value = JSON.stringify(helperPayload(snapshot.payload));
  const encoding = typeof options[0] === 'string' ? options[0] : options[0]?.encoding;
  return encoding ? value : Buffer.from(value);
};
helperPromises.writeFile = async function (file, data, ...options) {
  const target = helperPath.resolve(String(file));
  if (target === helperCredentialsPath) throw helperFailure();
  if (target !== helperTokenPath) return helperWrite(file, data, ...options);
  const payload = helperPayload(JSON.parse(String(data)));
  await helperSerial(async () => {
    const snapshot = await helperBridge.load();
    await helperBridge.update(snapshot.revision, payload);
  });
};
helperPromises.mkdir = async function (directory, ...options) {
  if (helperPath.resolve(String(directory)) === helperTokenDirectory) return undefined;
  return helperMkdir(directory, ...options);
};
// A missing/corrupt store must not make authorize() silently start its browser
// fallback during ordinary MCP startup or saved-access verification.
if (helperMode !== 'auth') helperHttp.createServer = () => { throw helperFailure(); };
helperSyncBuiltins();

try {
  if (!['auth', 'prepare', 'check', 'runtime'].includes(helperMode)
      || !process.env.GOOGLE_CLIENT_ID || !process.env.GOOGLE_CLIENT_SECRET
      || process.env.SERVICE_ACCOUNT_PATH || process.env.GOOGLE_IMPERSONATE_USER) throw helperFailure();
  const helperGoogleapis = helperRequire('googleapis');
  const helperGoogleRequire = helperCreateRequire(helperRequire.resolve('googleapis'));
  const helperCommonRequire = helperCreateRequire(helperGoogleRequire.resolve('googleapis-common'));
  const helperOAuthPath = helperCommonRequire.resolve('google-auth-library/build/src/auth/oauth2client.js');
  const helperOAuthSource = await helperRead(helperOAuthPath);
  if (helperHash('sha256').update(helperOAuthSource).digest('hex') !== 'af1aeb78dcd8ac899f157686f07c7b492131605fbf940909890fe9a4eeb90fe1') throw helperFailure();
  // Patch the constructor actually used by googleapis, whose nested auth library
  // differs from the package's top-level JWT dependency.
  const helperOAuthPrototype = helperGoogleapis.google.auth.OAuth2.prototype;
  const helperRefresh = helperOAuthPrototype.refreshTokenNoCache;
  const helperGenerate = helperOAuthPrototype.generateAuthUrl;
  helperOAuthPrototype.generateAuthUrl = function (...args) {
    if (helperMode !== 'auth') throw helperFailure();
    const url = helperGenerate.apply(this, args);
    process.stdout.write(`${JSON.stringify({ event: 'authorization_url', url })}\n`);
    return url;
  };
  helperOAuthPrototype.refreshTokenNoCache = async function (...args) {
    return helperSerial(async () => {
      const before = await helperBridge.load();
      const previous = helperPayload(before.payload);
      if (args[0] !== previous.refresh_token) throw helperFailure();
      const response = await helperRefresh.apply(this, args);
      const merged = helperPayload({ ...previous, ...helperPayload(response.tokens) });
      let saved = await helperBridge.update(before.revision, merged);
      if (helperMode !== 'runtime') saved = await helperBridge.verify(saved.revision);
      // The library's callers copy this refresh token back into their result.
      // Install the acknowledged replacement before those callers resume.
      this.credentials = { ...this.credentials, ...saved.payload };
      response.tokens = { ...response.tokens, ...saved.payload };
      return response;
    });
  };
  const helperAuth = await import(helperFileURL(helperPath.join(helperPackageRoot, 'dist/auth.js')).href);
  if (helperMode === 'auth') {
    await helperAuth.runAuthFlow();
  } else if (helperMode === 'runtime') {
    const saved = await helperBridge.load();
    if (!saved.verified) throw helperFailure();
    helperPayload(saved.payload);
    process.argv = [process.argv[0], helperEntry];
    await import(helperFileURL(helperEntry).href);
  } else {
    const before = await helperBridge.load();
    const client = await helperAuth.authorize();
    if (helperMode === 'check' || !before.verified) client.credentials.expiry_date = 1;
    await client.getAccessToken();
    const saved = await helperBridge.load();
    if (!saved.payload?.access_token || saved.payload.access_token !== client.credentials.access_token) throw helperFailure();
    await helperBridge.verify(saved.revision);
  }
} catch (error) {
  // Never print provider errors, token payloads, paths or callback query values.
  if (helperMode !== 'runtime') {
    const code = helperLastFailure?.code || error?.code;
    process.stdout.write(`${JSON.stringify({ event: 'failure', code: ['storage', 'stale', 'revoked', 'verification', 'unavailable'].includes(code) ? code : 'unavailable' })}\n`);
  }
  process.stderr.write('Fetch Google authentication failed.\n');
  process.exitCode = 1;
}
