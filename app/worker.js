import init, { encode, decode, stega_encode, stega_decode } from './pkg/chaotic_enc.js';

await init();
self.onmessage = async function(event) {
    const { type, buffer, message, secret, maxSide, outputAs } = event.data;

    console.log(
        'Worker received args:', 
        type, secret, maxSide, outputAs
    );

    try {
        if (type === 'encode') {
            const encoded = encode(buffer, secret, maxSide, outputAs);
            self.postMessage({
                type,
                buffer: encoded,
                format: outputAs 
            });
        }
        if (type === 'decode') {
            const decoded = decode(buffer, secret, maxSide, outputAs);
            self.postMessage({
                type, 
                buffer: decoded,
                format: outputAs 
            });
        }

        if (type === 'stega_encode') {
            const encoded = stega_encode(buffer, message, secret, maxSide);
            self.postMessage({
                type, 
                buffer: encoded,
                format: 'png', 
            });
        }

        if (type === 'stega_decode') {
            const decoded = stega_decode(buffer, secret, maxSide);
            self.postMessage({
                type, 
                message: decoded,
            });
        }

    }
    catch (error) {
        console.error('Worker error:', error);
        self.postMessage({ error });
    }

}