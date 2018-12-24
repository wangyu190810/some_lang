#!/usr/bin/env python
# -*-coding:utf-8-*-
# encoding: utf-8
import pika, time
try:
    from config import rabbitmq
except ImportError:
    rabbitmq = {
        "host": 'localhost',
        "port": '5672',
        "user_name": "rabbitmq",
        "password": "rabbitmq"

    }
'''创建rabbitmq连接'''
credentials = pika.PlainCredentials(rabbitmq.get("user_name"), rabbitmq.get("password"))
connection = pika.BlockingConnection(pika.ConnectionParameters(
    host=rabbitmq.get("host"),port=5672, credentials=credentials,virtual_host="/"))


delay_channel = connection.channel()
'''发送消息确认'''
delay_channel.confirm_delivery()

'''消息延迟时间(ms)'''
headers = {
    "x-delay": 10000 * 10
}
delay_channel.basic_publish(exchange='delay',
                      routing_key='delay',
                      body=str( time.strftime("%Y-%m-%d %H:%M:%S",time.localtime(time.time()))),
                      properties=pika.BasicProperties(headers=headers, delivery_mode=2))

print( " [x] Sent")
