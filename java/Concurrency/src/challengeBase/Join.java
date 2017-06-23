package challengeBase;

import static java.util.concurrent.TimeUnit.SECONDS;

/**
 * Created by wangyu on 2017/6/22.
 */

public class Join {
    public static void main(String[] args) throws Exception{
        Thread previous = Thread.currentThread();
        for(int i = 0; i< 10; i++){
            Thread thread = new Thread(new Domino(previous),String.valueOf(i));
            thread.start();
            previous = thread;
        }
        SECONDS.sleep(5);
        System.out.println(Thread.currentThread().getName() + "  terminate.");
    }

    static class Domino implements Runnable{
        private Thread thread;
        // 将创建的线程初始化到当前线程
        public Domino(Thread thread){
            this.thread = thread;
        }

        @Override
        public void run() {

//      当注释掉  thread.join()
//            0  terminate.
//            2  terminate.
//            4  terminate.
//            3  terminate.
//            1  terminate.
//            8  terminate.
//            6  terminate.
//            7  terminate.
//            9  terminate.
//            5  terminate.
//            main  terminate.


            try {
                thread.join();
            }catch (InterruptedException e){

            }
//            当运行 thread.join();
//            main  terminate.
//            0  terminate.
//            1  terminate.
//            2  terminate.
//            3  terminate.
//            4  terminate.
//            5  terminate.
//            6  terminate.
//            7  terminate.
//            8  terminate.
//            9  terminate.

            System.out.println(Thread.currentThread().getName() + "  terminate.");
        }
    }

}
