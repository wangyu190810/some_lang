#include "csapp.h"
int main(int argc,char **argv[],char **envp[]){
    char value;
    int i;
    const char *name = "test";
    
    *getenv(name);
    printf("name %s\n",name);
    for(i=0;i<3,i++;){
        printf("argv[%d] %s",i,argv[i]);
    }
//   printf("vaule %s\n",value);

}
