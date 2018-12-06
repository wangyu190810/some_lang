#!/usr/bin/env python
# -*-coding:utf-8-*-

from application import app
from config.cfg import BaseConf
if __name__ == '__main__':
    app.run(debug=True,port=BaseConf.port)

