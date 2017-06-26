package ConnPool;

import java.sql.Connection;
import java.util.LinkedList;

/**
 * Created by wangyu on 2017/6/22.
 */
public class ConnectionPool {
    // LinkedList类是双向列表,列表中的每个节点都包含了对前一个和后一个元素的引用.
    private LinkedList<Connection> pool = new LinkedList<Connection>();

    public ConnectionPool(int initalSize){
        if (initalSize > 0) {
            for (int i = 0; i < initalSize; i++) {
                // 将改造之后的connection放到双向链表 pool中
                pool.addLast(ConnectionDriver.CreateConnection());
            }
        }
    }

    public void releaseConnection(Connection connection){
        if(connection != null){
            synchronized (pool){
                // 连接释放后需要通知，这样其他消费者能够感知到连接池中已经归还了一个连接。
                pool.addLast(connection);
                pool.notifyAll();
            }
        }
    }
    // 在mills 内无法获取到连接，将会返回null

    public Connection fetchConnection(long mills) throws InterruptedException{
        synchronized (pool){
            // 完全超时
            if(mills < 0){
                while (pool.isEmpty()){
                    pool.wait();
                }
                return pool.removeFirst();
            }else{
                long future = System.currentTimeMillis();
                long remaining  = mills;
                while (pool.isEmpty() && remaining >0){
                    pool.wait(remaining);
                    remaining = future - System.currentTimeMillis();

                }
                Connection result = null;
                if (!pool.isEmpty()){
                    result = pool.removeFirst();
                }
                return result;
            }
        }
    }
}
