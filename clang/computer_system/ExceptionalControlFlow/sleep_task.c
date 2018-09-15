#include "csapp.h"
int main(){
    pid_t pid;
    pid = Fork();
    printf("%d\n",pid);
    soonze(3);
    exit(0);
}

