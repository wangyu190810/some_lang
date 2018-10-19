from django.shortcuts import render
from django.http import HttpResponse
from threading import Thread
import os
# Create your views here.
def index(request):
    async = AsyncThread()
    async.start()
    return HttpResponse("Hello, world. You're at the polls index.")

class AsyncThread(Thread):

    def run(self):
        return1 = os.system('ping  %s' % "www.haotougu.com")
        print(return1)