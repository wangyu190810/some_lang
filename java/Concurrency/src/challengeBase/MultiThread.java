package challengeBase;

import java.lang.management.ManagementFactory;
import java.lang.management.ThreadInfo;
import java.lang.management.ThreadMXBean;

/**
 * Created by wangyu on 2017/6/21.
 */
public class MultiThread {
    public static void main(String[] args){
        ThreadMXBean threadMXBean = ManagementFactory.getThreadMXBean();
        ThreadInfo[] threadInfos = threadMXBean.dumpAllThreads(false,false);
        for(ThreadInfo threadInfos1: threadInfos){
            System.out.println("[" + threadInfos1.getThreadId() +"] " + threadInfos1.getThreadName());
        }
    }
}
