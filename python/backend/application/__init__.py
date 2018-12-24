#!/usr/bin/env python
# -*-coding:utf-8-*-
from datetime import timedelta

from flask import Flask

from config.cfg import SECRET_KEY
from core.DB import DBmanger

app = Flask(__name__)
app.config['SECRET_KEY'] = SECRET_KEY
app.permanent_session_lifetime = timedelta(minutes=60)
app.DBSession = DBmanger.connection(20)
from application import init

@app.route('/')
def hello_world():
    return 'Hello World!'
