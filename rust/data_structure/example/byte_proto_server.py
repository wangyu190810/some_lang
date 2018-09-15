#!/usr/bin/env python
# -*- coding: utf-8 -*-


import socket
import threading
from struct import unpack,pack
import time
import Utils
HOST = ''
PORT = 5000

"""
byte proto test


#### 协议设计
   消息长度(head) 消息结束标记(flag)    消息数据(body)
*----------------   -                   ---------
    16              1    
*----------------   -                   ---------


*设计概要*

消息头部放置本次消息的长度,消息发送结束，长度之后的字节表示消息体

*具体设计*

* 首字符16位字节表示本次消息长度，最大的表示长度是65535。
* 本次消息发送是否完成，也就是是否完成了一条消息的完整发送
* 消息体，当消息超过65535，需要将消息分割，分成多次发送。

具体代码实现服务端

"""


listen_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
listen_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
listen_socket.bind((HOST, PORT))
listen_socket.listen(100)

print('Serving line proto on port %s ...' % PORT)

    

def client_process(client_connection):
    content = ""
    while True:
        # try:
        if True:
            msg = "121231231asdfasdfasdfasdfasdfasdf2"  
            # client_connection.sendall(Utils.pack_msg(msg))
            client_connection.sendall(Utils.pack_head())
            time.sleep(10)
    client_connection.close()
           

def main():
    while True:
        client_connection, client_address = listen_socket.accept()
        print("client_connection", client_connection, client_address)
        client_thread = threading.Thread(target=client_process, args=(client_connection,))
        client_thread.setDaemon(True)
        client_thread.start()



if __name__ == '__main__':
    main()

