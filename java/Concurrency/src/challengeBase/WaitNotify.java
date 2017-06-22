package challengeBase;

import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.concurrent.TimeUnit;
import static java.util.concurrent.TimeUnit.SECONDS;
/**
 * Created by wangyu on 2017/6/22.
 */
public class WaitNotify {
    static  boolean flag = true;
    static Object lock = new Object();

    public static void main(String[] args) throws Exception{
        Thread waitThread = new Thread(new Wait(), "WaitThread");
        waitThread.start();
        SECONDS.sleep(1);
        Thread notifyThread = new Thread(new Notify(),"notifyThread");
        notifyThread.start();
    }

    static class Wait  implements Runnable {
        public void run(){
            // 加锁
            synchronized (lock){
                while (flag){
                    try {
                        System.out.println(Thread.currentThread() + " flag is ture, wait @ " + new SimpleDateFormat("HH:mm:ss").format(new Date()));
                        // 释放锁
                        lock.wait();
                    }catch (Exception e){

                    }
                }
                System.out.println( Thread.currentThread() + "flag is false. runing @" + new SimpleDateFormat("HH:mm:ss").format(new Date()));
            }
        }
    }
    static class Notify  implements Runnable{
        public void run(){
            // 加锁
            synchronized (lock){
                System.out.println( Thread.currentThread() + "hlod lock. runing @" + new SimpleDateFormat("HH:mm:ss").format(new Date()));
                // 通知其他等待锁的成员。 并不是释放锁，而是告诉其他线程，你们可以开始抢锁了。
                // 我释放时，当我执行结束时候也就是 synchronized 结构结束时候，你们就可以拿着lock开始执行了。
                lock.notifyAll();
                flag = false;
                // SleepUtils.second(5) not find, replace this.
                try {
                    SECONDS.sleep(5);
                }catch (InterruptedException e){
                    e.printStackTrace();

                }
                System.out.println( Thread.currentThread() + "end. runing @" + new SimpleDateFormat("HH:mm:ss").format(new Date()));
            }

            synchronized (lock){
                System.out.println( Thread.currentThread() + "hlod lock. runing @" + new SimpleDateFormat("HH:mm:ss").format(new Date()));
                // SleepUtils.second(5) not find, replace this.
                try {
                    SECONDS.sleep(5);
                }catch (InterruptedException e){
                    e.printStackTrace();

                }
            }
        }
    }
}
