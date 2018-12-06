#!/usr/bin/env python
# -*-coding:utf-8-*-

from dao.Base import Base,metadata
from sqlalchemy import create_engine
from config.cfg import Config
from dao.Route import Route
from dao.User import User

url = Config.db.get("test").get("w")
print(url)
engine_w_create = create_engine(Config.db.get("test").get("w"))
Base.metadata.create_all(engine_w_create)
# metadata.create_all(engine_w_create,tables=User)
