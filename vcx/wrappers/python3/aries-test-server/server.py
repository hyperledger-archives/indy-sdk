import json
from http.server import BaseHTTPRequestHandler
import socketserver

from command_handler import CommandHandler


command_handler = CommandHandler()


class RequestHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        body = self.rfile.read(int(self.headers['Content-Length']))
        print("command received: >>> " + body.decode())
        response = command_handler.handle_command(json.loads(body.decode()))
        self.reply(response)


    def reply(self, content):
        self.send_response(200)
        self.end_headers()
        self.wfile.write(bytes(content.encode()))


def run():
    with socketserver.TCPServer(("", 8083), RequestHandler) as httpd:
        print('Starting server on 127.0.0.1 ')
        httpd.serve_forever()
        print('Server started')

if __name__ == '__main__':
    run()
