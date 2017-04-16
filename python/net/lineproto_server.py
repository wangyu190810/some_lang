#!-*-coding-utf8-*-
import socket
import threading


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

def client_process(client_connection):
    content = ""
    while True:
        try:
            request = client_connection.recv(1024)
            if len(request) == 0:
                # 当收到长度为零的数据时，表示客户端已经断开连接。
                break
            print(type(request))
            print "request",request
            content += request
            if "\r\n" in content:
                data = content.split("\r\n")
                print(data)
                for row in data[:-1]:
                    _resp_data = "{data}\r\n".format(data=row)
                    print(_resp_data)
                    client_connection.sendall(_resp_data)
                content = data[-1]
        except Exception as e:
            print("e :%s" %e)
    client_connection.close()
           


while True:
    client_connection, client_address = listen_socket.accept()
    print "client_connection", client_connection, client_address
    client_thread = threading.Thread(target=client_process, args=(client_connection,))
    client_thread.setDaemon(True)
    client_thread.start()
