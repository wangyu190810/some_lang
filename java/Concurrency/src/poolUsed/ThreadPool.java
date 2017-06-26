package poolUsed;

/**
 * Created by wangyu on 2017/6/23.
 */
public interface ThreadPool<Job extends Runnable> {
    // 执行一个job，这个job需要实现Runnable
    void execute(Job job);
    // 关闭线程池
    void shutdown();
    // 增加工作者
    void addWorkers(int nums);
    // 减少工作者线程
    void removeWorker(int nums);
    // 得到正在等候执行的任务量
    int getJobSize();

}
