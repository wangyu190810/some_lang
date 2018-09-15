#include "csapp.h"

void handler(int sig){
    static int beeps = 0;
    printf("BEEP\n");
    if (++beeps<5){
        Alarm(1);
    
    }else{
        printf("BOOM\n");
        exit(0);
    
    }
}

int main(){
    Signal(SIGALRM,handler);
    Alarm(1);
    while(1){
        ;
    
    }
    exit(0);
}
