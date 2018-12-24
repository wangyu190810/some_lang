#!/usr/bin/env python
# -*-coding:utf-8-*-
__author__ = '22too'

from sqlalchemy.schema import MetaData
from sqlalchemy.ext.declarative import declarative_base
# from sqlalchemy.sql import MetaData


metadata = MetaData()
Base = declarative_base()