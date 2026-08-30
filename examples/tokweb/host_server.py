#!/usr/bin/env python3
"""Tiny HTTP server for the tokweb xbox demo. Run on the host: python3 host_server.py"""
from http.server import BaseHTTPRequestHandler, HTTPServer

class H(BaseHTTPRequestHandler):
    def do_GET(self):
        body = b"hello from the host web server, xbox 360!\n"
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)
    def log_message(self, *a):
        print("host_server:", self.address_string(), a[0] % a[1:])

if __name__ == "__main__":
    HTTPServer(("0.0.0.0", 8080), H).serve_forever()
