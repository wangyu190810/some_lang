#!/usr/bin/env python
# -*-coding:utf-8-*-
from sqlalchemy import create_engine
from sqlalchemy.orm import scoped_session, sessionmaker

SECRET_KEY = 'XXXXXXXXX'

class BaseConf(object):
    port = 7878


class Config(object):
    # db = r"sqlite:///tmp/test.db"
    db = dict(
        test={
            "w": "mysql+pymysql://root:1234@localhost/test?charset=utf8mb4",
            "r": "mysql+pymysql://root:1234@localhost/test?charset=utf8mb4"
        }
    )
    SUCCESS_KEY = "change me"
    allowed_extensions = "set(['txt', 'pdf', 'png', 'jpg', 'jpeg', 'gif'])"
    upload_folder = r".\static\upload"
