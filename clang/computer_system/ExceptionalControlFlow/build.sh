#/bin/bash
#编译命令

gcc  fork.c  csapp.c -o fork -lpthread
gcc  fork1.c  csapp.c -o fork1 -lpthread
gcc  waitprob0.c  csapp.c -o waitprob0 -lpthread
gcc  forkprob0.c  csapp.c -o forkprob0 -lpthread
gcc  waitpid1.c  csapp.c -o waitpid1 -lpthread
gcc  waitpid2.c  csapp.c -o waitpid2 -lpthread
