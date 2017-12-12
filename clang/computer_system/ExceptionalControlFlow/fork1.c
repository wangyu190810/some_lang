#include "csapp.h"
int main()
{

pid_t pid;
int x = 1;

pid = Fork();
pid = Fork();
pid = Fork();
printf("hellon\n");
exit(0);
}
