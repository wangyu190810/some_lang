#!/usr/bin/env python
# -*-coding:utf-8-*-



def resp_msg(**kwargs):
    """标准化处理"""
    if kwargs.get("code") is None:
        kwargs['code'] = 0
    if kwargs.get("msg") is None:
        kwargs['msg'] = ""
    if kwargs.get("data") is None:
        kwargs['data'] = 0
    return kwargs
