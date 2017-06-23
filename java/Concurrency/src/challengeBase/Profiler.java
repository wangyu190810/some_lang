package challengeBase;

import java.util.concurrent.TimeUnit;

/**
 * Created by wangyu on 2017/6/22.
 */
public class Profiler {
    private static final ThreadLocal<Long> TIME_THREADLOCAL = new ThreadLocal<Long>(){
        // ThreadLocal 实现方法中有 initiaValue ，这里我们重写了这个方法，用来给final 初始化
        // 关于final 定义查看 地址 ： http://www.importnew.com/7553.html
        protected Long initiaValue(){
            return System.currentTimeMillis();
        }
    };

    public static final void begin(){
        TIME_THREADLOCAL.set(System.currentTimeMillis());
    }

    public static final long end(){
        return System.currentTimeMillis() - TIME_THREADLOCAL.get();
    }

    public static void main(String[] args) throws Exception{
        Profiler.begin();
        TimeUnit.SECONDS.sleep(1);
        System.out.println("Cost: " + Profiler.end() + "mills");
    }

}
