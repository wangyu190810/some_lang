#!/usr/bin/env python
# -*-coding:utf-8-*-
from flask import redirect,sessions
from application import app

@app.route('/login')
def login():
    # sessions['user_name']= "login"
    return "success"
