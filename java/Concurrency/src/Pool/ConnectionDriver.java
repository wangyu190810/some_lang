package Pool;

import java.lang.reflect.InvocationHandler;
import java.lang.reflect.Method;
import java.lang.reflect.Proxy;
import java.sql.Connection;
import java.util.concurrent.TimeUnit;

/**
 * Created by wangyu on 2017/6/22.
 */
public class ConnectionDriver {
    // 动态代理的一些知识。补充自己的java知识，就能看懂这段代码到底是什么意思。
    // http://www.cnblogs.com/xiaoluo501395377/p/3383130.html

    static class ConnectionHandler implements InvocationHandler{

        public Object invoke(Object proxy, Method method, Object[] args)  throws Throwable{
            if(method.getName().equals("commit")){
                TimeUnit.MILLISECONDS.sleep(100);
                System.out.print("commit");

            }
            return null;
        }
    }
    public static final Connection CreateConnection(){
        return (Connection) Proxy.newProxyInstance(ConnectionDriver.class.getClassLoader(),new Class<?>[]{Connection.class},new ConnectionHandler());
    }
}
