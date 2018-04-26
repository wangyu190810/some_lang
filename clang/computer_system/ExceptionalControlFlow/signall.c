#include "csapp.h"

void handler1(int sig){

    pid_t pid;
    if ((pid= waitpid(-1,NULL,0))< 0)
        unix_error("waitpid error");
    
    printf("Handler resped child %d\n", (int)pid);
    Sleep(1);
    return ;

}

int main()
{

    int i, n;
    char buf[MAXBUF];
    if (signal(SIGCHLD,handler1)== SIG_ERR)
        unix_error("signal error");
    /*Parent create child  */

    for(i = 0; i< 3; i++){
        if (Fork() ==0){
            printf("hello form child %d\n", (int)getpid());
            Sleep(1);
            exit(0);
        }
    }

    /* Parent waits for terminal input and then process it */
    if ((n = read(STDERR_FILENO,buf,sizeof(buf))) <0 ){
        unix_error("read");
    }
    printf("Parent processing input\n");
    while (1)
        ;
    exit(0);

}
