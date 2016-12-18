# 关于rust的一些学习总结

## 更改源

因为墙的存在所以，经常出现资源无法下载

在用户目录下.cargo下创建文件config
文件内容如下:

    [registry]
    index = "http://crates.mirrors.ustc.edu.cn/index"

更换源，目前很多人使用中科大的源
然后就可以进行编译
