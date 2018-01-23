#include "csapp.h"
void handler(int sig){
    printf("Caught SIGINT\n");
    exit(0);

}
int main(){
    if (signal(SIGINT, handler)== SIG_ERR ){
        unix_error("signal error");

    }
    pause();
    exit(0);

}


