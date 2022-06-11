# XiaoXuan VM

A zero-dependency WebAssembly VM with a full-featured Web UI debugger implemented in Rust.

一个用 Rust 实现的零依赖 WebAssembly 虚拟机，带有全功能的 Web UI 调试界面。

Features

- [x] Run multi-module WASM applications
- [x] Disassemble WASM applications
- [ ] Supports WASI interface to run WASM applications compiled from C/C++ and Rust
- [ ] Web UI debugging interface, support step-by-step tracing, set breakpoints, and view memory and call stack data.

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [XiaoXuan VM](#xiaoxuan-vm)
  - [功能](#功能)
  - [获取程序](#获取程序)
    - [下载预编译可执行文件](#下载预编译可执行文件)
    - [从源码编译](#从源码编译)
      - [单元测试](#单元测试)
      - [编译及安装](#编译及安装)
  - [运行程序](#运行程序)
    - [指定入口函数及其参数](#指定入口函数及其参数)
    - [运行多个模块的程序](#运行多个模块的程序)
  - [反汇编](#反汇编)

<!-- /code_chunk_output -->

## 功能

- [x] 运行多模块 WASM 应用程序
- [x] 反汇编 WASM 应用程序
- [ ] 支持 WASI 接口，能运行由 C/C++ 和 Rust 编译而得的 WASM 程序
- [ ] Web UI 调试界面，支持逐步跟踪、设置断点，能直观地查看内存、调用栈的数据。

## 获取程序

你可以通过下载预编译的可执行文件而获得 XiaoXuan VM 程序，或者从源码编译获得。

### 下载预编译可执行文件

访问 XiaoXuan VM 项目的 [源代码仓库](https://github.com/hemashushu/xiaoxuan-vm)，进入 "[Release 页面](https://github.com/hemashushu/xiaoxuan-vm/releases)"，选择最新的版本，然后根据你的操作系统选择相应的可执行文件压缩包。

下载完毕之后将程序解压，并复制到目录 `~/.local/bin` 或者任意一个位于环境变量 `$PATH` 列表当中的目录。

### 从源码编译

todo::

#### 单元测试

切换到 XiaoXuan VM 项目的源码的首层目录，然后运行下面命令：

`$ cargo test`

如果所有单元测试均通过，则可以进行下一步。

#### 编译及安装

运行下面命令以编译 XiaoXuan VM 程序：

`$ cargo build`

编译完成之后将会在目录 `./target/debug/` 里面得到 `anvm` 和 `anvm-debugger` 两个程序。

也可以编译为发行版：

`$ cargo build --release`

编译后的程序将会存在目录 `./target/release` 里。

为了方便运行程序，建议将上面步骤得到的两个程序复制到目录 `~/.local/bin` 或者任意一个位于环境变量 `$PATH` 列表当中的目录。也可以使用下面命令将程序复制到目录 `~/.cargo/bin`：

```bash
$ cargo install --path ./crates/launcher
$ cargo install --path ./crates/debugger
```

## 运行程序

XiaoXuan VM 可以运行由 C/C++/Rust 编译而得的 WebAssembly 应用程序（以下简称 "WASM 应用程序"，一个 WASM 应用程序由一个或多个 `*.WASM` 模块文件组成）。XiaoXuan VM 支持 [WASI](https://github.com/WebAssembly/WASI) 接口，即支持基本的文件 I/O 等操作。

> XiaoXuan VM 项目自带几个测试用的 WASM 应用程序，位于 `./crate/launcher/resources` 目录之中，可用于下面的示例。

假设有一个 WASM 应用程序的模块文件名为 `app.wasm`，则运行这个应用程序的命令如下：

`$ anvm app.wasm`

如果模块文件不在当前目录，则需要写上模块文件的完整路径或者相对路径，比如：

`$ anvm ~/projects/xiaoxuan-vm/crate/launcher/resources/app.wasm`

### 指定入口函数及其参数

模块当中 `start` 段指向的函数或者名称为 `main` 的函数将会作为程序的入口。

`$ anvm app.wasm --function app::add 123 456`

上面的 `app::add` 为指定的函数的名称，其中 `app` 为模块的名称，即 `*.wasm` 的文件名，`add` 则为该模块所导出的函数的名称，`123 456` 为函数的参数，多个参数之间使用空格分隔。

如果想启动一个未导出的函数，也可以通过函数的索引来启动，假设 `app.wasm` 模块里有一个函数 `sub`，其索引值为 `2`，则启动的命令为：

`$ anvm app.wasm --function app::2 123 456`

上述的命令参数 `--function` 可以使用 `-f` 来代替，比如：

`$ anvm app.wasm -f app::add 123 456`

### 运行多个模块的程序

有时一个 WebAssembly 应用程序可能由多个模块组成，如果要运行这种应用程序，则只需将所有模块（的文件名）传入 XiaoXuan VM 即可：

`$ anvm core.wasm std.wasm app.wasm`

程序的入口将从右侧的模块开始搜索。

## 反汇编

XiaoXuam VM 也提供了反汇编的功能，用于将二进制的 WASM 反汇编为文本格式，命令如下：

`$ anvm --disassembly input.wasm output.wat`

每次只能反汇编一个模块，其中的命令参数 `--disassembly` 可以使用 `-d` 来代替，比如：

`$ anvm -d input.wasm output.wat`
