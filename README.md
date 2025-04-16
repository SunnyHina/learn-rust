# learn-rust
rust-book.cs.brown.edu
https://kaisery.github.io/trpl-zh-cn/title-page.html
https://rust.now.cc/ch03-03-how-functions-work.html

你还真别说,这所有权还真挺烧脑.


在 Linux 或 macOS 上安装 rustup
如果你使用 Linux 或 macOS，打开终端并输入如下命令：

`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
此命令下载一个脚本并开始安装 rustup 工具，这会安装最新稳定版 Rust。过程中可能会提示你输入密码。如果安装成功，将会出现如下内容：

`Rust is installed now. Great!`


另外，你还需要一个 链接器（linker），这是 Rust 用来将其编译的输出连接到一个文件中的程序。很可能你已经有一个了。如果你遇到了链接器错误，请尝试安装一个 C 编译器，它通常包括一个链接器。C 编译器也很有用，因为一些常见的 Rust 包依赖于 C 代码，因此需要安装一个 C 编译器。


Linux 用户通常需要根据发行版（distribution）文档安装 GCC 或 Clang。比如，如果你使用 Ubuntu，可以安装 build-essential 包。


1. 首先,更新您的包列表:


```bash
sudo apt update
```


2. 然后,安装build-essential包,它包含了gcc、g++和make等基本的编译工具:


```bash
sudo apt install build-essential
```