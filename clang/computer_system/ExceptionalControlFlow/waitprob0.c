
#include "csapp.h"

int main(){

if (Fork()==0){
	printf("a\n");
}

else{
	
	printf("b\n");
	waitpid(-1,NULL,0);

}
printf("c\n");
exit(0);

}

/*
b
a
c
c

*/

/*
父进程执行else 打印b，
子进程执行 if 打印a,
父进程和子进程同时执行c

*/
