#include "csapp.h"
int main()
{

int x = 1;

if(Fork()==0){
    printf("print1: x=%d\n", ++x);
}
    printf("print2: x=%d\n", --x);
exit(0);
}

/*
print2: x=0
user$ print1: x=2
print2: x=1

*/
