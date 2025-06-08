import init, { encode, decode } from './pkg/chaotic_enc.js';

await init();
self.onmessage = async function(event) {
    const { type, buffer, secret, maxSide } = event.data;

    console.log('Worker received message:', event.data);

    if (type === 'encode') {
        const encoded = encode(buffer, secret, maxSide);
        self.postMessage({ type: 'encoded', buffer: encoded });
    }

    if (type === 'decode') {
        const decoded = decode(buffer, secret, maxSide);
        self.postMessage({ type: 'decoded', buffer: decoded });
    }
}