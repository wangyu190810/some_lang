#!/usr/bin/env python
# -*-coding:utf-8-*-
from flask import jsonify,request,g
from application import app
from core.request_process import request_data
from dao.Route import Route
from utils.utils import resp_msg
from core.request_process import login_required



@app.route('/route_list', methods=['GET', 'POST'])
@login_required
def route_list():
    if request.method == 'POST':
        form_data = request.form
    else:
        form_data = request.values
    page = form_data.get("page")
    print(g.db)
    data,total = Route.get_route(g.db.get("test_r"))
    resp_data = []
    for row in data:
        _stmt_ = dict()
        _stmt_['url'] = row.url
        _stmt_['name'] = row.name
        _stmt_['level'] = row.level
        resp_data.append(_stmt_)
    return jsonify(data=resp_data)

@app.route('/route_add', methods=['GET', 'POST'])
@login_required
def route_add():
    form_data = request_data()
    page = form_data.get("page")
    url = form_data.get("url")
    level = form_data.get("level")
    name = form_data.get("name")
    data = Route.add_route(g.db.get("test_w"),url=url,level=level,name=name)

    if data:
        data = resp_msg()
    else:
        data =resp_msg(code="1",data="add route error")
    return jsonify(data)


@app.route('/route_edit', methods=['GET', 'POST'])
@login_required
def route_edit():
    form_data = request_data()
    r_id = form_data.get("r_id")
    url = form_data.get("url")
    level = form_data.get("level")
    name = form_data.get("name")
    data = Route.edit_route(g.db.get("test_w"),r_id=r_id,url=url,level=level,name=name)

    if data:
        data = resp_msg()
    else:
        data =resp_msg(code="1",data="edit route error")
    return jsonify(data)

