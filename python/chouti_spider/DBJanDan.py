# -*-coding:utf-8-*-
# email:info@22too.com
__author__ = 'info'

from sqlalchemy.orm import sessionmaker
from sqlalchemy import create_engine,text
import json
from db import connection
from config import Config


conn = connection(Config.db)

