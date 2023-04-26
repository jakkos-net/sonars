#!/usr/bin/env python3

# Taken from the wasm_bindgen audio worklet example: https://github.com/rustwasm/wasm-bindgen/tree/c5b073ae58cb3b6d44252108ea9862bf0d04f3b6/examples/wasm-audio-worklet

from http.server import HTTPServer, SimpleHTTPRequestHandler, test
import sys

class RequestHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        SimpleHTTPRequestHandler.end_headers(self)

if __name__ == '__main__':
    test(RequestHandler, HTTPServer, port=int(sys.argv[1]) if len(sys.argv) > 1 else 8000)