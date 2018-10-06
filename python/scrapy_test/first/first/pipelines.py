# -*- coding: utf-8 -*-

# Define your item pipelines here
#
# Don't forget to add your pipeline to the ITEM_PIPELINES setting
# See: https://doc.scrapy.org/en/latest/topics/item-pipeline.html
from scrapy.xlib.pydispatch import dispatcher
from scrapy import signals
import logging

class FirstPipeline(object):

    def process_item(self, item, spider):
        return item

    def __init__(self):
        dispatcher.connect(self.spider_stopped, signals.engine_stopped)  ##建立信号和槽，在爬虫结束时调用
        dispatcher.connect(self.spider_closed, signals.spider_closed)  ##建立信号和槽，在爬虫关闭时调用

    # 爬虫关闭时 调用本方法
    def spider_closed(self):
        logging.info("i close")
        print("close ")

    # 爬虫结束时 调用本方法
    def spider_stopped(self):
        logging.info("i done")
        print("done")

