#include "csapp.h"
#include "lib.h"

void echo(int connfd){
    size_t n;
    char buf[MAXLINE];
    rio_t rio;
    Rio_readinitb(&rio,connfd);
    while ((n = Rio_readlineb(&rio,buf,MAXLINE)) != 0){
        printf("server received %d types\n",n);
        Rio_writen(connfd,buf,n);
    }

}

void command(void){
    char buf[MAXLINE];
    if (!Fgets(buf,MAXLINE,stdin))
        exit(0);
    printf("%s",buf);

}
