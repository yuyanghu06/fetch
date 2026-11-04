// Private helper storage client. It never writes to stdout or retries a failed write.
import { createConnection as fetchBridgeConnect } from 'node:net';

export function createBridgeClient() {
  const socketPath = process.env.FETCH_HELPER_SOCKET;
  const capability = process.env.FETCH_HELPER_CAPABILITY;
  if (!socketPath || !capability) throw new Error('Helper storage is unavailable.');
  const maxFrame = 1024 * 1024;
  let previous = Promise.resolve();
  function request(fields) {
    return new Promise((resolve, reject) => {
      const socket = fetchBridgeConnect(socketPath);
      let data = Buffer.alloc(0);
      let finished = false;
      const fail = (code = 'unavailable') => {
        if (finished) return;
        finished = true;
        socket.destroy();
        const error = new Error('Helper storage operation failed.');
        error.code = code;
        reject(error);
      };
      socket.setTimeout(30000, () => fail());
      socket.on('error', () => fail());
      socket.on('end', () => { if (!finished) fail(); });
      socket.on('connect', () => {
        const bytes = Buffer.from(JSON.stringify({ capability, ...fields }) + '\n');
        if (bytes.length > maxFrame) return fail('invalid');
        socket.write(bytes);
      });
      socket.on('data', (chunk) => {
        if (finished) return;
        if (data.length + chunk.length > maxFrame) return fail('invalid');
        data = Buffer.concat([data, chunk]);
        const newline = data.indexOf(10);
        if (newline < 0) return;
        let response;
        try { response = JSON.parse(data.subarray(0, newline).toString('utf8')); }
        catch { return fail('invalid'); }
        if (response.ok !== true) return fail(['invalid', 'storage', 'stale', 'revoked', 'verification', 'unavailable'].includes(response.error) ? response.error : 'unavailable');
        if (!Number.isSafeInteger(response.revision) || response.revision < 0 || typeof response.verified !== 'boolean'
            || !(response.payload === null || typeof response.payload === 'object' && !Array.isArray(response.payload))) return fail('invalid');
        finished = true;
        socket.destroy();
        resolve({ revision: response.revision, payload: response.payload, verified: response.verified });
      });
    });
  }
  function ordered(fields) {
    const result = previous.then(() => request(fields));
    previous = result.catch(() => {});
    return result;
  }
  return {
    load: () => ordered({ op: 'load' }),
    update: (expected_revision, payload) => ordered({ op: 'update', expected_revision, payload }),
    verify: (expected_revision) => ordered({ op: 'verify', expected_revision }),
  };
}
