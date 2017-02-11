# 关于rust的一些学习总结

因为墙的存在所以，经常出现资源无法下载

## 安装rustup

*当前在ubuntu16.04下执行*

[rust](https://www.rust-lang.org/en-US/)官网安装时，会让你下载rustup初始化文件

按照要就执行：
    
    curl https://sh.rustup.rs -sSf | sh

不过因为墙的存在，直接执行上面的命令会失败,所以仅仅执行如下命令：

    curl https://sh.rustup.rs -sSf     

会生成一个 *rustup-init.sh* 文件，打开当前文件：

会看到变量:
    
    RUSTUP_UPDATE_ROOT="https://static.rust-lang.org/rustup/dist"

将这个变量后边的地址进行替换，这个地址为中国科学技术大学官网：

    RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup/dist"

然后执行:

    bash rustup-init.sh

等待执行完成，输入如下命令：

    rustup
        
    rustup 1.0.0 (17b6d21 2016-12-15)
    The Rust toolchain installer

    USAGE:
        rustup [FLAGS] [SUBCOMMAND]

    FLAGS:
        -v, --verbose    Enable verbose output
        -h, --help       Prints help information
        -V, --version    Prints version information
    ...

安装成功

也有可能无法找到rustup命令，执行下面命令刷新环境变量：

    cd ~/.cargo/
    source env

当然最后是将上述命令添加到用户环境变量中,所以仅仅执行如下命令：

    vim ~/.bashrc

下面这句写入文件：
    
    export PATH="$HOME/.cargo/bin:$PATH"

rust开发环境搭建完成

十分感谢 [中国科学技术大学开源软件镜像](https://lug.ustc.edu.cn/wiki/server/mirrors/start)
    