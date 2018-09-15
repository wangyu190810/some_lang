import functools
def cost_time(asdf):
    def date(func):
        @functools.wraps(func)
        def app(*args,**kwarsg):
            "app doc"
            print("adaf")
            func(*args,**kwarsg)
            print("adaf")
        return app
    return date

@cost_time
def app():
    "fasdfasdf"
    pass

# rint(app.__doc__)

class classname(object):
    def __new__(cls):
        pass


class A(object):
    def __init__(self):
        print "init"

    def __new__(cls,*args, **kwargs):
        print "new %s"%cls

        return object.__new__(cls, *args, **kwargs)

A()