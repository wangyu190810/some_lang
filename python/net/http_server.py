#!-*-coding-utf8-*-
import socket

HOST, PORT = '', 5000

HOST = ''
PORT = 5000

"""http proto test"""

listen_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
listen_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
listen_socket.bind((HOST, PORT))
listen_socket.listen(100)


print 'Serving HTTP on port %s ...' % PORT

while True:
    client_connection, client_address = listen_socket.accept()
    print "client_connection", client_connection, client_address

    request = client_connection.recv(1024)
    print "request",request

    http_response = """
HTTP/1.1 200 OK

Hello, World!asdfasdf
"""
    client_connection.sendall(http_response)
    client_connection.close()
