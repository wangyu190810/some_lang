package poolUsed;


import java.util.ArrayList;
import java.util.Collections;
import java.util.LinkedList;
import java.util.List;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * Created by wangyu on 2017/6/23.
 */
public class DefaultThreadPool <Job extends Runnable> implements ThreadPool<Job>{
    // 线程池最大限制数量
    private static final int MAX_WORKER_NUMBERS = 10;
    // 线程池默认的数量
    private static final int DEFAULT_WORKER_NUMBERS = 5;
    // 线程池最小限制数量
    private static final int MIN_WORKER_NUMBERS = 1;
    // 这是一个工作列表，将会向里边插入工作
    private final LinkedList<Job> jobs =  new LinkedList<Job>();
    // 工作者列表
    private final List<Worker>  workers = Collections.synchronizedList(new ArrayList<Worker>());
    // 工作者的线程数
    private int workerNum = DEFAULT_WORKER_NUMBERS;
    // 线程编号生成
    private AtomicInteger threadNum = new AtomicInteger();
    public DefaultThreadPool(){
        initializWorkers(DEFAULT_WORKER_NUMBERS);
    }

    public DefaultThreadPool(int num){
        workerNum = num > MAX_WORKER_NUMBERS?MAX_WORKER_NUMBERS:num < MIN_WORKER_NUMBERS?MIN_WORKER_NUMBERS:num;
        initializWorkers(workerNum);
    }

    public void execute(Job job){

        if(job != null){
            synchronized (jobs){
                jobs.addLast(job);
                jobs.notify();
            }
        }
    }


    public void shutdown(){
        for(Worker worker:workers){
            worker.shutdown();
        }
    }
    public void addWorkers(int num){
        synchronized (jobs){
            if(num + this.workerNum > MAX_WORKER_NUMBERS){
                num = MAX_WORKER_NUMBERS - this.workerNum;
            }
            initializWorkers(num);
            this.workerNum += num;
        }
    }
    public void removeWorker(int num){
        synchronized (jobs){
            if(num > this.workerNum){
                throw  new IllegalAccessError("beyond workNum");
            }
            int count = 0;
            while (count < num){
                Worker worker =workers.get(count);
                if(workers.remove(worker)){
                    worker.shutdown();
                    count ++;
                }
            }
            this.workerNum -= count;
        }
    }
    public int getJobSize(){
        return jobs.size();
    }
    public void initializWorkers(int num){
        for (int i = 0; i< num; i++){
            Worker worker = new Worker();
            workers.add(worker);
            Thread thread = new Thread(worker,"ThreadPool-work-" + threadNum.incrementAndGet());
            thread.start();
        }
    }

    // 工作者，负责消费任务
    class Worker implements Runnable{
        // 是否工作
        private volatile boolean running = true;
        @Override
        public void run() {
            while (running){
                Job job = null;
                synchronized (jobs){
                    // 如果工作者列表是空的，那么就执行wait
                    while (jobs.isEmpty()){
                        try {
                            jobs.wait();
                        }catch (InterruptedException e){
                            // 感知到对外部 WorkerThread 的中断操作，返回
                            Thread.currentThread().interrupt();
                            return;
                        }
                    }
                    // 取出一个Job
                    job  = jobs.getFirst();
                }
                if(job != null){
                    try {
                        job.run();
                    }catch (Exception e){

                    }
                }
            }

        }
        public void shutdown(){
            running = false;
        }
    }

}
