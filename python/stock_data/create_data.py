from sqlalchemy import create_engine
import tushare as ts
import redis
import json
import pymysql
import time

#获取2014年第3季度的盈利能力数据
date = {}
date[2015] = [row for row in range(1,3+1)]
date[2016] = [row for row in range(1,3+1)]
date[2017] = [row for row in range(1,3+1)]
# df = ts.get_profit_data(2016,3)

engine = create_engine('mysql+pymysql://root:2016@127.0.0.1/test?charset=utf8')
conn = engine.connect()
rdb = redis.Redis(host='192.168.1.11',port=6379,db=2)
db =  pymysql.connect(host='127.0.0.1',
                    port=3306,
                    user='root',
                    password='2016',
                    db='test',
                    charset='utf8mb4',
                    cursorclass=pymysql.cursors.DictCursor)


#存入数据库
#df.to_sql("profit_data",engine)

#追加数据到现有表
#df.to_sql('profit_data',engine,if_exists='append')
update_date = "update profit_data set date= %s where profit_data.index in (%s) "
repalce_date = "update profit_data as a inner join profit_data as b on  a.date = b.date and a.date != 0 set a.date = %s"
select_date = "select pd.index from profit_data as pd where date = 0"
str_date = "201501"
repalce_date = repalce_date%str_date

# ids = conn.execute(select_date).fetchall()
# ids = ",".join([str(index_id['index']) for index_id in ids])
# update_date = update_date % (str_date, ids)
# print(update_date)
# stmt = conn.execute(update_date, {"date":str_date})
# #stmt.commit()
# conn.commit()

# conn.execute(repalce_date)
# ids = conn.execute(select_date).fetchall()
# ids = [index_id['index'] for index_id in ids]
# print(ids)

#conn.commit()

# df = ts.get_profit_data(2016,3)
# data = df.to_json()
# fb_file = open("test.txt","w")
# fb_file.write(data)
# fb_file.close()

# rdb.hset("test_profit_data",201603, data)

def add_data(key, value,update_date=update_date,select_date=select_date):
    # for value in values:
    if True:
        conn = engine.connect()
        df = ts.get_profit_data(key,value)
        df.to_sql('profit_data',engine,if_exists='append',index=False)
        str_date = str(key) + (str(value) if len(str(value)) == 2 else "0" + str(value))
        # conn.execute(repalce_date, {"date":int(date)})
        # repalce_date = repalce_date%str_date
        # print(repalce_date)
        try:
            with db.cursor() as cursor:
                cursor.execute(select_date)
                ids = cursor.fetchall()
                ids = ",".join([ str(index_id['index']) for index_id in ids])
                update_date = update_date % (str_date, ids)
                print(update_date)
                stmt = cursor.execute(update_date)
            db.commit()

        except Exception as e:
            print(e)
        # finally:
        #     db.close()
        # break

# add_data(2015,  1)
# print(2015, 1)
# time.sleep(10)

# add_data(2015,  2)
# print(2015, 2)
# time.sleep(10)

def add_profit():

    add_data(2015,  3)
    print(2015, 3)
    time.sleep(10)

    add_data(2015,  4)
    print(2015, 4)
    time.sleep(10)

    # add_data(2016,  1)
    # print(2016, 1)
    # time.sleep(10)

    add_data(2016,  2)
    print(2016, 2)
    time.sleep(10)

    add_data(2016,  3)
    print(2016, 3)
    time.sleep(10)

    add_data(2016,  4)
    print(2016, 4)
    time.sleep(10)

    add_data(2017,  1)
    print(2017, 1)
    time.sleep(10)

    # break

    # break

def get_stock_basics():
    df = ts.get_stock_basics()
    df.to_sql('stock_basics',engine,if_exists='append',index=False)

get_stock_basics()
