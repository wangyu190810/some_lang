#!/usr/bin/env python
# -*-coding:utf-8-*-
from flask import request,g,Response,current_app,session,redirect
from application import app
from functools import wraps

# @app.before_request
# def before_request():
#     g.db = current_app.DBSession()
#
#     ip = request.remote_addr
#     url = request.url
#     print(ip)
#     print(url)
#
# @app.after_request
# def after_request(response):
#     # Response.
#     current_app.DBSession.remove()
#     g.db.close()
#     print(response.get_data())
#     return response
#     # print(Response)

def login_required(func):
    @wraps(func)
    def decorated_view(*args, **kwargs):
        return func(*args, **kwargs)
        # pass
        # if 'username' in session:
        #     return func(*args, **kwargs)
        # else:
        #     return redirect("/login")
    return decorated_view



@app.before_request
def before_request():
    db = {}
    for db_name,DBsession in current_app.DBSession.items():
        db[db_name] = DBsession()
    g.db = db
    ip = request.remote_addr
    url = request.url
    print(ip)
    print(url)


@app.after_request
def after_request(response):
    # Response.
    for db_name, DBsession in current_app.DBSession.items():
        DBsession.remove()
    for val in g.db.values():
        val.close()
    print(response.get_data())
    return response


def request_data():
    if request.method == 'POST':
        form_data = request.form
    else:
        form_data = request.values
    return form_data
