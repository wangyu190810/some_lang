# -*-coding:utf-8-*-
# email:190810401@qq.com
__author__ = 'wangyu'

from sqlalchemy.orm import sessionmaker
from sqlalchemy import create_engine,text
import json

from config import Config


def connection(database):
    engine = create_engine(database)
    Session = sessionmaker(engine)
    session = Session()
    return session

conn = connection(Config.db)


def insert_data(**data):
    sql =text("insert into chouti(chouti_id,content,comments) "
              "VALUES (:chouti_id,:content,:comments)")
    sql = sql.bindparams(chouti_id=data.get("chouti_id"),
                         content=json.dumps(data.get("chouti_content")),
                         comments=json.dumps(data.get("chouti_comments"))
                         )
    conn.execute(sql)
    conn.commit()

def get_data_exits(*ids):
    sql = text("select chouti_id from  chouti  where chouti_id in :ids")
    sql = sql.bindparams(ids=ids)
    data = conn.execute(sql)
    resps = data.fetchall()
    resp_ids = []
    for resp in resps:
        resp_ids.append(resp.chouti_id)
    return resp_ids
