

class ProcessJson(object):

    
    def __init__(self, json_obj):
        self.json_obj = json_obj

        if isinstance(self.json_obj, str):
            self.dumps()
        else:
            self.loads()

    def loads(self):
        pass


    def dumps(self):
        pass

