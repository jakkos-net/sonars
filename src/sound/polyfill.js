// TextEncoder and TextDecoder are broken and cause an error but we don't need it, so replace them with empty methods.
// Taken from the wasm_bindgen audio worklet example: https://github.com/rustwasm/wasm-bindgen/tree/c5b073ae58cb3b6d44252108ea9862bf0d04f3b6/examples/wasm-audio-worklet

if (!globalThis.TextDecoder) {
    globalThis.TextDecoder = class TextDecoder {
        decode(arg) {
            if (typeof arg !== 'undefined') {
                throw Error('TextDecoder stub called');
            } else {
                return '';
            }
        }
    };
}

if (!globalThis.TextEncoder) {
    globalThis.TextEncoder = class TextEncoder {
        encode(arg) {
            if (typeof arg !== 'undefined') {
                throw Error('TextEncoder stub called');
            } else {
                return new Uint8Array(0);
            }
        }
    };
}