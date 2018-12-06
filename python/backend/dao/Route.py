#!/usr/bin/env python
# -*-coding:utf-8-*-

__author__ = '22too'

import hashlib

from sqlalchemy import Column, String, Integer,desc
import time
from dao.Base import Base
import traceback
import logging
from utils.constant import BackendStatus

class Route(Base):
    __tablename__ = "backend_route"
    id = Column(Integer, primary_key=True, autoincrement=True)
    url = Column(String(80))
    name = Column(String(80))
    icon = Column(String(80))
    level = Column(Integer)
    create_time = Column(Integer)

    @staticmethod
    def add_route(connection,url,level,create_time=None,name="",icon=""):
        if create_time is None:
            create_time = int(time.time())
        route = Route(url=url,level=level,create_time=create_time,name=name,icon=icon)
        try:
            connection.add(route)
            connection.commit()
            return True
        except Exception as e:
            logging.error( traceback.print_exc())
            logging.error("add_route error %s" %e)
            return False

    @staticmethod
    def get_route(connection,id=None,url=None,level=None,start=0,offset=BackendStatus.PAGE_LIMIT,one=False):
        try:
            query =connection.query(Route)
            if id:
                query = query.filter_by(id=id)
                return query.first()
            if url is not None:
                query = query.filter_by(url=url)
            if level is not None:
                query = query.filter_by(level=level)
            if one:
                return query.first()
            total = query.count()
            query = query.order_by(desc(Route.create_time)).slice(start,start + offset).all()
            return query,total
        except Exception as e:
            logging.error(traceback.print_exc())
            logging.error("add_route error %s" % e)
            return [], 0




