#!-*-coding-utf8-*-
import socket



HOST, PORT = '', 5000

HOST = ''
PORT = 5000

"""
line proto test
"""


listen_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
listen_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
listen_socket.bind((HOST, PORT))
listen_socket.listen(100)

print 'Serving line proto on port %s ...' % PORT

while True:
    client_connection, client_address = listen_socket.accept()
    print "client_connection", client_connection, client_address
    content = ""
    while True:
        
        request = client_connection.recv(1024)
        print(type(request))
        print "request",request
        content += request
        
        if "\r\n" in content:
            data = content.split("\r\n")
            print(data)
            for row in data[:-1]:
                _resp_data = "{data}\n".format(data=row)
                print(_resp_data)
                client_connection.sendall(_resp_data)
            content = data[-1]
        
client_connection.close()