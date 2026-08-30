#!/usr/bin/env python3
"""HTTP server for the tokweb xbox demo. Logs each request to stdout (flushed)."""
import sys
from http.server import BaseHTTPRequestHandler, HTTPServer

class H(BaseHTTPRequestHandler):
    def do_GET(self):
        body = b"hello from the host web server, xbox 360!\n"
        self.send_response(200)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)
        print(f"REQUEST from {self.client_address[0]}:{self.client_address[1]}  {self.command} {self.path}", flush=True)
    def log_message(self, *a):
        pass

if __name__ == "__main__":
    print("host_server: listening on 0.0.0.0:8080", flush=True)
    HTTPServer(("0.0.0.0", 8080), H).serve_forever()
