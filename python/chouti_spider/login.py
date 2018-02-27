# -*-coding:utf-8-*-
import http.cookiejar
import json
import pprint
import re
import time

import requests

from config import Config
from db import insert_data,get_data_exits

headers = {'User-Agent':'Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.108 Safari/537.36',
"Accept":"application/json, text/javascript, */*; q=0.01" }



# 建立一个会话，可以把同一用户的不同请求联系起来；直到会话结束都会自动处理cookies
session = requests.Session()
# 建立LWPCookieJar实例，可以存Set-Cookie3类型的文件。 # 而MozillaCookieJar类是存为'/.txt'格式的文件
session.cookies = http.cookiejar.LWPCookieJar("cookie")
# 若本地有cookie则不用再post数据了
try:
    session.cookies.load(ignore_discard=True)
except IOError:
    print("Cookie未加载！")



def login(phone, password):
    """登录操作"""
    url = 'http://dig.chouti.com/login'
    data = {'phone':phone,'password':password,"oneMonth":1}
    result = session.post(url, data=data, headers=headers)
    print(result.text)
    session.cookies.save(ignore_discard=True, ignore_expires=True)
    isLogin

def isLogin():
    # 通过查看用户个人信息来判断是否已经登录
    timestamp = time.time() * 1000
    url = "http://dig.chouti.com/getTopTenLinksOrComments.json?_=%s" %timestamp
    # 禁止重定向，否则登录失败重定向到首页也是响应200
    response = session.get(url, headers=headers,
                             allow_redirects=False)
    resp_json = response.json()
    data = resp_json.get("result")
    data = data.get("data")
    # print(data[1])\
    chouti_ids = []
    for exits in data:
        chouti_id = exits.get("id")
        chouti_ids.append(chouti_id)
    exits_ids = get_data_exits(*chouti_ids)
    end_ids = list(set(chouti_ids).difference(set(exits_ids)))
    for row in data:
        # 抽屉id
        chouti_id = row.get("id")
        chouti_content = row
        if chouti_id not in end_ids:
            pprint.pprint(chouti_content)
            continue
        # 抽屉内容
        chouti_comments = comments(chouti_id)
        # 抽屉评论
        save_data = dict(
            chouti_id = chouti_id,
            chouti_content = chouti_content,
            chouti_comments = chouti_comments
        )
        insert_data(**save_data)
    login_code = response.status_code
    if login_code == 200:
        return True
    else:
        return False

def comments(link):
    url = Config.link_title

    print(url)
    data = {"linkId":link,"sortType":"score","id":0}
    print(data)

    response = session.post(url,data=data, headers=headers)
    # print(response.text)
    resp_json = response.json()
    data = resp_json.get("result")
    data = data.get("data")
    data_list = data.get("dataList")
    
    return data_list

if __name__ == '__main__':
    if isLogin():
        print('您已经登录')
    else:
        phone = input('输入账号：')
        password = input('输入密码：')
        if phone.startswith("86") is False:
            phone += "86"
        login(phone, password)
