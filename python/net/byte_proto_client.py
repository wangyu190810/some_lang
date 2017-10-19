#!/usr/bin/env python
# -*- coding: utf-8 -*-


import socket
import threading
from struct import unpack,pack
import Utils
HOST = '127.0.0.1'
PORT = 5000

"""
byte proto test


#### 协议设计
   消息长度(head)     消息数据(body)
*----------------    ---------
    8                  
*----------------    ---------

long long的最大值：9223372036854775807
long long的最小值：-9223372036854775808


*设计概要*

消息头部放置本次消息的长度,长度之后的字节表示消息体

*具体设计*


* 首字符8位字节表示本次消息长度，最大的表示长度是9223372036854775807。
* 消息体，应该不会有消息超过长度

具体代码实现服务端

"""



client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
client.connect((HOST,PORT))
print('Serving line proto on port %s ...'% PORT)


def client_process(client_connection):
    content = ""
    msg = b""
    flag = 1
    while True:
        try:
            data_len = Utils.body_len(client_connection)
            msg_body = Utils.body_data(client_connection,data_len)
            print(msg_body)
        except Exception as e:
            print(e)

def main():
    while True:
        client_process(client)

if __name__ == '__main__':
    main()
