package challenge;

/**
 * Created by wangyu on 2017/6/21.
 */
public class DeadLockDemo {
    private static String A="A";
    private static String B="B";

    // 运行入口
    public static void main(String[] args){
        new DeadLockDemo().deadLock();
    }

    private void deadLock(){
        Thread t1 = new Thread(new Runnable() {
            @Override
            public void run() {
                synchronized (A){

                    // 进行A的同步操作，这个时候进行Sleep操作，让出运行时间。
                    // 开始运行其他线程，等待其他线程结束。再回来运行当前的线程
                    // 但是因为同步锁不是放，所以无法运行其他线程。所以发生卡死等待
                    // 中断程序执行，抛出异常，执行打印
                    try {
                        Thread.currentThread().sleep(2000);
                    }catch (InterruptedException e){
                        e.printStackTrace();
                    }
                    synchronized (B){
                        System.out.println("1");
                    }
                }
            }
        });

        Thread t2 = new Thread(new Runnable() {
            @Override
            public void run() {
                System.out.println("thread t2 exe");
                synchronized (B){
                    synchronized (A){
                        System.out.println("2");
                    }
                }
            }
        });
        t1.start();
        t2.start();
    }
}
