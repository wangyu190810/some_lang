class Mysql(object):

    def __init__(self):
        print("init mysql ")

    def __del__(self):
        print("mysql data")

    def foo(self):
        print("mysql foo")

class Redis(object):

    def __init__(self):
        print("init redis ")

    def __del__(self):
        print("redis data")

    def foo(self):
        print("redis foo")

class Handler(Mysql,Redis):
    def __init__(self):
        print("use ")

    def foo(self):
        '''
        '''
        super(Handler,self).foo()






def main():
    handler = Handler()
    handler.foo()

'''
执行结果：
use
mysql foo
mysql data

结果说明：
多继承，继承顺序按照排列顺序进行
'''




if __name__ == '__main__':
    main()