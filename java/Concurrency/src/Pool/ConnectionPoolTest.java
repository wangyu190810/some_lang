package Pool;

import java.sql.Connection;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Created by wangyu on 2017/6/22.
 */

public class ConnectionPoolTest {
    //  连接池初始化10个
    static ConnectionPool pool = new ConnectionPool(10);
    // http://www.importnew.com/15731.html
    // 用来指示线程全部动作更加统
    // 在实时系统中的使用场景
//
//    让我们尝试罗列出在java实时系统中CountDownLatch都有哪些使用场景。我所罗列的都是我所能想到的。如果你有别的可能的使用方法，请在留言里列出来，这样会帮助到大家。
//
//    实现最大的并行性：有时我们想同时启动多个线程，实现最大程度的并行性。例如，我们想测试一个单例类。如果我们创建一个初始计数为1的CountDownLatch，并让所有线程都在这个锁上等待，那么我们可以很轻松地完成测试。我们只需调用 一次countDown()方法就可以让所有的等待线程同时恢复执行。
//    开始执行前等待n个线程完成各自任务：例如应用程序启动类要确保在处理用户请求前，所有N个外部系统已经启动和运行了。
//    死锁检测：一个非常方便的使用场景是，你可以使用n个线程访问共享资源，在每次测试阶段的线程数目是不同的，并尝试产生死锁
    static CountDownLatch start = new CountDownLatch(1);
    static CountDownLatch end;

    public static void main(String[] args) throws Exception{

        // 修改同时执行的线程数量，达到测试目的
        //  int threadCount = 20
        //  int threadCount = 30
        //  int threadCount = 40
        int threadCount = 10;


        end  = new CountDownLatch(threadCount);
        int count = 20;
        AtomicInteger got = new AtomicInteger();
        AtomicInteger notGot = new AtomicInteger();
        for(int i=0; i < threadCount; i++){
            Thread thread = new Thread(new ConnectionRuner(count,got,notGot),"ConnectionRunerThread");
            thread.start();
        }
        // 全部启动完成，在这里执行
        start.countDown();

        // 阻塞主进程，让其他进程完成自己的任务
        end.await();

        System.out.println("total invoke: " + (threadCount * count));
        System.out.println("got connection : " + got);
        System.out.println("not got connection : " + notGot);
    }

    static class ConnectionRuner implements Runnable{
        int count;
        // http://www.cnblogs.com/Gordon-YangYiBao/archive/2012/08/07/2626422.html
        // 关于 AtomicInteger 定义，主要是指这个东西是线程安全的增加和减少。
        AtomicInteger got;
        AtomicInteger notGot;

        public ConnectionRuner(int count, AtomicInteger got, AtomicInteger notGot){
            this.count = count;
            this.got = got;
            this.notGot  = notGot;
        }

        @Override
        public void run() {
            try {
                start.await();
                // 所有启动的线程，停在这里等待，
            }catch (Exception e){
                e.printStackTrace();
            }
            while (count > 0){

                try {
                    Connection connection = pool.fetchConnection(2000);
                    if (connection != null){
                        try {
                            connection.createStatement();
                            connection.commit();
                        }finally {
                            pool.releaseConnection(connection);
                            got.incrementAndGet();
                        }

                    }else {
                        notGot.incrementAndGet();
                    }
                }catch (Exception e){

                }finally {
                    count --;
                }
            }end.countDown();
        }
    }

}
