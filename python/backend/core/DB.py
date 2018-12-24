#!/usr/bin/env python
# -*-coding:utf-8-*-

from sqlalchemy import create_engine
from sqlalchemy.orm import scoped_session,sessionmaker
from config.cfg import Config

class DBmanger(object):

    @staticmethod
    def connection(pool_size=20):
        db_conf = Config.db
        engine_dict = {}
        for key,value in db_conf.items():
            # engine_w_name = key + value.get("w")
            engine_w_name = key + "_w"
            engine_w_create = create_engine(value.get("w"), echo=False,pool_size=pool_size, max_overflow=0)
            # engine_r_name = key + value.get("r")
            engine_r_name = key + "_r"
            engine_r_create = create_engine(value.get("r"), echo=False, pool_size=pool_size, max_overflow=0)
            r_DBSession = scoped_session(sessionmaker(bind=engine_r_create))
            w_DBSession = scoped_session(sessionmaker(bind=engine_w_create))
            engine_dict[engine_w_name] = w_DBSession
            engine_dict[engine_r_name] = r_DBSession
        return engine_dict
