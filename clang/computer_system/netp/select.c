#include "csapp.h"
#include "lib.h"

void echo(int connfd);
void command(void);


int main(int argc, char **argv){
    
    int listenfd, connfd, port;
    socklen_t clientlen = sizeof(struct sockaddr_in);
    struct sockaddr_in clientaddr;
    fd_set read_set, ready_set;
    if (argc != 2){
        fprintf(stderr,"usage: %s <port>\n", argv[0]);
        exit(0);
    }
    port = atoi(argv[1]);
    listenfd = Open_listenfd(port);
    FD_ZERO(&read_set);
    FD_SET(STDIN_FILENO,& ready_set);
    FD_SET(listenfd,&ready_set);
    while(1) {
        ready_set = read_set;
        Select(listenfd + 1,&read_set, NULL, NULL, NULL);
        if (FD_ISSET(STDIN_FILENO,&ready_set) )
            command();
        if (FD_ISSET(listenfd, &ready_set)){
            connfd = Accept(listenfd, (SA *)&clientaddr,&clientlen);
            echo(connfd);
            Close(connfd);
        }
    }


}
