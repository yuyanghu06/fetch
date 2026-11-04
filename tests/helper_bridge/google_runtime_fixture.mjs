// Synthetic stdio output checks byte preservation and acknowledged refresh use.
import { authorize } from './auth.js';
const client = await authorize();
await client.getAccessToken();
process.stdout.write('{ "jsonrpc":"2.0", "result":{"number":184467440737095516160000001,"content":[{"type":"image","mimeType":"image/png","data":"fixture-base64"}]} }\n');
