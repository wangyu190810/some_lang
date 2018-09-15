#/bin/bash
#编译命令

gcc hostinfo.c   csapp.c -o hostinfo -lpthread
gcc echoserverp.c   csapp.c lib.c -o echoserver -lpthread
gcc select.c csapp.c lib.c -o select -lpthread

