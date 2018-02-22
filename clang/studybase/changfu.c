#include <stddef.h>
#include <unistd.h>
#include <pthread.h>
thread1()
{
    while (1)
    {
        printf("Iam the thread\n");
        sleep(1);
    }
}
main()
{
    int ret;
    pthread_t id;
    ret = pthread_create(&id, NULL, (void *)thread1, NULL);
    if (ret != 0)
    {
        printf("thread_send error\n");
        return (1);
    }
    while (1)
    {
        printf("I am the main-thread\n");
        sleep(1);
    }
}