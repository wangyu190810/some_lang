#! -*- coding:utf-8 -*-
import pymysql
import datetime

values  = dict(
    title = u"测试",
    guideread = u"测试",
    content = u"测试",
    pubtime = datetime.datetime.now(),
    status = 1 ,
    usertype = 199909,
    type= 4,
    sendmsg = u"测试"
)

STOCK =  {
        'host': '192.168.1.10',
        'port': 3306,
        'user': 'root',
        'passwd': '1234',
        'db': 'db',
       'cursorclass': pymysql.cursors.DictCursor,
       'charset':'utf8'
}


conn = pymysql.connect(**STOCK)
cur= conn.cursor()

sql = u"insert into tb_news_notice (title,guideread,content,status,usertype,type,sendmsg) \
values ('{title}','{guideread}','{content}',{status},{usertype},{type},'{sendmsg}')".format(**values)
print(sql)

cur.execute(sql)
conn.commit()


