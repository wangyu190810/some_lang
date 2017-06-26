# java connect pool

ConnectionDriver 实现了，在commit时候sleep。

ConnnectionPoll 实现了一个线程池，初始化创建链接线程，当需要连接时候，从list中借出连接。
当断开连接时候，将连接归还。

ConnnectionPoolTest 设置相应的测试用例，用来测试数据。

