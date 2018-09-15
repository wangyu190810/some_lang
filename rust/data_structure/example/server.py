import socket

sock = socket.socket()
addr = ("0.0.0.0",8111)
sock.bind(addr)
sock.listen(10)
# sock.connect()
while True:
    connfd = sock.accept()
    sock.sendto()