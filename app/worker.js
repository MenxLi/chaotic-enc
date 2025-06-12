import init, { encode, decode } from './pkg/chaotic_enc.js';

await init();
self.onmessage = async function(event) {
    const { type, buffer, secret, maxSide, outputAs } = event.data;

    console.log(
        'Worker received args:', 
        type, secret, maxSide, outputAs
    );

    try {
        if (type === 'encode') {
            const encoded = encode(buffer, secret, maxSide, outputAs);
            self.postMessage({
                type: 'encoded',
                buffer: encoded,
                format: outputAs 
            });
        }
        if (type === 'decode') {
            const decoded = decode(buffer, secret, maxSide, outputAs);
            self.postMessage({
                type: 'decoded',
                buffer: decoded,
                format: outputAs 
            });
        }
    }
    catch (error) {
        console.error('Worker error:', error);
        self.postMessage({ error });
    }

}