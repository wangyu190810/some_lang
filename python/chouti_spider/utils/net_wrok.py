# -*-coding:utf-8-*-
import http.cookiejar
import json
import pprint
import re
import time

import requests


headers = {'User-Agent':'Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/63.0.3239.108 Safari/537.36',
"Accept":"application/json, text/javascript, */*; q=0.01" }

# result = requests.get(url, data=data, headers=headers)
