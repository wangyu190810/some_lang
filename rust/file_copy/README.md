# 文件拷贝服务器

主要实现文件的差异自动备份

## 最初需要解决问题如下

### 实现思路

- 客户端和服务器确定路径下的文件
- 文件变更发现
- 如何实现文件增量传输

### 客户端和服务端协议制定

- 使用http协议操作

### 客户端和服务器端安装方式

- 服务器启动时，指定备份存放目录

- 客户端启动时，指定需要备份的文件目录