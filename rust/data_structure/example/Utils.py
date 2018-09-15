#!/usr/bin/env python
# -*- coding: utf-8 -*-

from struct import unpack,pack

"""
python 二进制转换可以参看这张网络博客
http://blog.csdn.net/ithomer/article/details/5974029/

"""

def pack_head(head_type=40,size=4):
    # pack()
    format_str = "HH"
    head_type = head_type
    size = size
    pack_data = pack(format_str,head_type,size) 
    print(pack_data)
    return pack_data

def pack_price():
    size = 12
    head_type = 40
    
    12, 40, 700, 474000

def pack_msg(msg):
    """
    msg:需要打包的消息
    return 打包后的数据
    """
    msg_len = len(msg)
    format_str ="q%ss"%(msg_len) 
    print(format_str)
    pack_data = pack(format_str,msg_len,msg)
    print(pack_data)
    print(len(pack_data))
    return pack_data
 

def unpack_head(msg):
    """"
    msg:需要解包的消息
    return:
        
         返回消息包大小
    """
    format_str ="q" 
    msg_len = unpack(format_str,msg)
    return msg_len

def unpack_body(msg,msg_len):
    """
    msg: 需要解包的消息体
    """
    format_str = "%ss"%msg_len
    print(format_str,msg_len,str(msg))
    body =  unpack(format_str,msg)
    print(body)
    return body

def body_len(connect):
    data = connect.recv(8)
    msg_len = unpack_head(bytes(data))
    return msg_len[0]

def body_data(connect,msg_len):
    data = connect.recv(msg_len)
    return unpack_body(bytes(data),msg_len)
    

def main():
    pack_msg("123123")

if __name__ == '__main__':
    main()