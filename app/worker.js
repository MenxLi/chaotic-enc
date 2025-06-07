import init, { encode, decode } from './pkg/chaotic_enc.js';

await init();
self.onmessage = async function(event) {
    const { type, buffer, secret } = event.data;

    if (type === 'encode') {
        const encoded = encode(buffer, secret);
        self.postMessage({ type: 'encoded', buffer: encoded });
    }

    if (type === 'decode') {
        const decoded = decode(buffer, secret);
        self.postMessage({ type: 'decoded', buffer: decoded });
    }
}