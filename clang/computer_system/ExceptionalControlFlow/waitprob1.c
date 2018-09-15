#include "csapp.h"

int main(){
    int status;
    pid_t pid;
    printf("Hello\n");
    pid = Fork();
    printf("%d\n",!pid);
    if(pid != 0){
        if((pid=waitpid(-1,&status,0))>0){
            if(WIFEXITED(status))
            printf("child %d terminated normally with exit status = %d\n",
                    pid,WEXITSTATUS(status)
                    );
  
    
        }
    }
        printf("Bye\n");
        exit(2);

}
