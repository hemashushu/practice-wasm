# XiaoXuan VM

A WebAssembly VM with a full-featured Web UI debugger implemented in Rust.

一个用 Rust 实现的 WebAssembly 虚拟机，带有全功能的 Web UI 调试界面。

Features

- [x] Run multi-module WASM applications.
- [x] Disassemble WASM applications.
- [ ] Translate the WebAssembly text format to binary.
- [ ] Web UI debugging interface, support step-by-step tracing, set breakpoints, and view memory and call stack data.
- [x] Supports WASI interface, can run applications compiled from C/C++ and Rust.
- [ ] Support for loading application images, providing Docker-like container features.
- [ ] Support for state persistence, 0-time cold startup, providing Serverless and Function-as-a-Service (FaaS) services.

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [XiaoXuan VM](#xiaoxuan-vm)
  - [功能](#功能)
  - [获取 VM 程序](#获取-vm-程序)
    - [下载预编译的可执行文件](#下载预编译的可执行文件)
    - [从源代码编译](#从源代码编译)
      - [单元测试](#单元测试)
      - [编译及安装](#编译及安装)
  - [运行 WASM 应用程序](#运行-wasm-应用程序)
    - [指定起始函数及其参数](#指定起始函数及其参数)
    - [运行多个模块的程序](#运行多个模块的程序)
  - [反汇编](#反汇编)
  - [构建 WASM 应用程序](#构建-wasm-应用程序)
    - [直接手动书写](#直接手动书写)
    - [编译 C 语言程序](#编译-c-语言程序)
    - [编译 Rust 语言程序](#编译-rust-语言程序)
    - [编译 Rust 的 Cargo 项目](#编译-rust-的-cargo-项目)

<!-- /code_chunk_output -->

## 功能

- [x] 运行多模块 WASM 应用程序；
- [x] 反汇编 WASM 应用程序；
- [ ] 转译 WAT 文本格式到二进制格式；
- [ ] Web UI 调试界面，支持逐步跟踪、设置断点，能直观地查看内存、调用栈的数据；
- [x] 支持 WASI 接口，能运行 C/C++ 和 Rust 编译的程序；
- [ ] 支持加载应用程序映像，实现类似 Docker 的容器功能；
- [ ] 支持状态持久化，支持 0 时间冷启动，实现 Serverless 和 Function-as-a-Service (FaaS) 功能；

## 获取 VM 程序

你可以通过下载预编译的可执行文件来获取 XiaoXuan VM 程序，也可以从源代码开始编译而获得。

### 下载预编译的可执行文件

访问 XiaoXuan VM 项目的 [源代码仓库](https://github.com/hemashushu/xiaoxuan-vm)，进入 "[Release 页面](https://github.com/hemashushu/xiaoxuan-vm/releases)"，选择最新的版本，然后根据你的 CPU 架构和操作系统选择相应的可执行文件压缩包下载。（如果找不到合适的预编译可执行文件，则需要从源代码编译）

下载完毕之后解压压缩包，并复制里面的程序到目录 `~/.local/bin` 或者任意一个位于环境变量 `$PATH` 列表当中的目录即可。

### 从源代码编译

访问 XiaoXuan VM 项目的 [源代码仓库](https://github.com/hemashushu/xiaoxuan-vm)，下载源代码的压缩包然后解压到任意目录。

或者使用 Git 克隆项目的源代码：

`$ git clone https://github.com/hemashushu/xiaoxuan-vm.git`

#### 单元测试

切换到 XiaoXuan VM 项目的源代码的首层目录，然后运行下面命令：

`$ cargo test`

如果所有单元测试均通过，则可以进行下一步。

#### 编译及安装

运行下面命令以编译 XiaoXuan VM 程序：

`$ cargo build`

编译完成之后将会在目录 `./target/debug/` 里面得到 `anvm` 和 `anvm-debugger` 两个程序。

注意默认编译得到的是调试版的程序，调试版的程序运行速度可能非常慢，如果不需要调试程序，建议编译为发行版：

`$ cargo build --release`

编译完成之后将会在目录 `./target/release/` 里面得到 `anvm` 和 `anvm-debugger` 两个程序。

为了方便运行程序，建议将上面步骤得到的两个程序复制到目录 `~/.local/bin` 或者任意一个位于环境变量 `$PATH` 列表当中的目录。也可以使用下面命令将程序复制到目录 `~/.cargo/bin`：

```bash
$ cargo install --path ./crates/launcher
$ cargo install --path ./crates/debugger
```

## 运行 WASM 应用程序

XiaoXuan VM 可以运行由 C/C++/Rust 编译而得的 WebAssembly 应用程序（以下简称 "WASM 应用程序"），你也可以通过文本格式 `*.wat` 手动书写 WebAssembly 应用程序，然后使用 XiaoXuan VM 的编译程序（尚未完成）或者 [wasm-tools](https://github.com/bytecodealliance/wasm-tools) 将 `*.wat` 编译为 `*.wasm`。

> 注意区分 `XiaoXuan VM 程序` 和 `WASM 应用程序`，前者是一个虚拟机，同时也是一个引擎，也就是本项目的程序，是一个可执行的程序，用于加载和运行后者。后者是一个或多个 `*.wasm` 模块文件，通常由 C/C++/Rust 或者其他语言编译而得，无法直接运行。

XiaoXuan VM 支持 [WASI](https://github.com/WebAssembly/WASI) 接口，即支持基本的文件 I/O、Socket I/O 等操作（尚未完成）。

> XiaoXuan VM 项目自带几个测试用的 WASM 应用程序，位于 `./crate/launcher/resources` 目录之中，可用于下面的示例。

假设有一个 WASM 应用程序的模块文件名为 `fib.wasm`，则可以用如下命令运行这个应用程序：

`$ anvm fib.wasm`

如果模块文件不在当前目录，则需要指明模块文件的完整路径或者相对路径，比如：

`$ anvm ~/projects/xiaoxuan-vm/crates/launcher/resources/fib.wasm`

### 指定起始函数及其参数

WebAssembly 模块当中有一个 `start` 段，该段所指向的函数将会作为 WASM 应用程序的入口（即起始函数），如果模块不存在 `start` 段，则 XiaoXuan VM 会尝试寻找模块当中名称为 `_start` 的导出函数，如果找到则它将会作为程序的入口；如果找不到，则 XiaoXuan VM 会因为找不到入口而无法运行 WASM 应用程序。

你可以手动指定 WASM 应用程序的起始函数，假设有 WASM 应用程序 `lib.wasm`，其中有一个名称为 `pow` 的导出函数，如果想执行它，则可以运行如下命令：

`$ anvm lib.wasm --function lib::pow 2 10`

> 命令参数 `--function` 可以使用 `-f` 来代替。

上面的 `lib::pow` 为起始函数的完整名称，其中 `lib` 为模块的名称（即 `*.wasm` 的文件名当中排除了扩展名的部分），`pow` 则为该模块所导出的函数的名称，`2 10` 为函数的参数，多个参数之间使用空格分隔。

> 注意 "函数的参数" 跟我们平常通过命令行传递给应用程序的 "命令行参数" 不是同一个概念，"函数的参数" 仅用于手动指定起始函数这种场合，这相当于一种 hack 手段，将数据强行传递给应用程序里的某个片段（即函数）。

WebAssembly 1.0 只支持 4 种基本的数字的数据类型，分别是 `i32`、`i64`、`f32` 和 `f64`。在我们输入的命令行中的 "函数参数值" 如果其中存在小数点，则视为 `f32`，如果没有小数点，则视为 `i32`。当然可以通过在数字后面加上后缀 `i`、`l`、`f` 和 `d` 显式指定数据类型，示例：

| 数据类型 | 示例               |
| ------- | ----------------- |
| `i32`   | `123`、`123i`     |
| `i64`   | `3l`、`345l`      |
| `f32`   | `3.142`、`3.142f` |
| `f64`   | `2d`、`2.718d`    |

XiaoXuan VM 也能执行一个未导出的函数，只需指定函数的索引即可，假设 `lib.wasm` 模块里有一个函数 `pow`，其索引值为 `0`，则命令为：

`$ anvm lib.wasm --function lib::0 2 10`

### 运行多个模块的程序

有时一个 WASM 应用程序由多个模块组成，如果要运行这种应用程序，则需要将所有模块（的文件路径及名称）传入 XiaoXuan VM ，例如：

`$ anvm lib.wasm app.wasm`

XiaoXuan VM 将会从右侧的模块开始搜索应用程序的入口函数。当然你仍然可以手动指定起始函数，比如：

`$ anvm lib.wasm app.wasm -f lib::pow 2 20`

## 反汇编

XiaoXuam VM 也提供了反汇编的功能，用于将 WASM 应用程序的二进制的格式 `*.wasm` 反汇编为文本格式  `*.wat`，命令如下：

`$ anvm --disassembly input.wasm output.wat`

但每次只能反汇编一个模块，其中的命令参数 `--disassembly` 可以使用 `-i` 来代替，比如：

`$ anvm -i input.wasm output.wat`

## 构建 WASM 应用程序

一般的 WASM 应用程序是由 C/C++/Rust 或者其他语言编译而得，当然你也可以手动书写文本格式的 WebAssembly 应用程序，然后编译成二进制格式。

### 直接手动书写

WASM 的文本格式在外观上很像 Lisp 程序，比如下面是一个计算 `斐波那契数`（`fib`）的函数的源代码：

```scheme
(module
    ;; 计算 `斐波那契数`（`fib`）的函数
    ;;
    ;; 0、 1、 1、 2、 3、 5、 8、 13、 21、 34、 55、 89、 144、 233、 377、 610
    ;; ^   ^                                   ^
    ;; |   \-- 第 1 项                          \-- 第 10 项
    ;; \------ 第 0 项
    (func $fib (param $n i32) (result i32)
        (if (result i32)
            (i32.le_s (local.get $n) (i32.const 1))
            (then
                (local.get $n)
            )
            (else
                (i32.add
                    (i32.sub (local.get $n) (i32.const 1))
                    (call $fib)
                    (i32.sub (local.get $n) (i32.const 2))
                    (call $fib)
                )
            )
        )
    )

    (export "fib" (func $fib))
)
```

把上面的代码保存到一个文件（比如 `my.wat`），然后使用 XiaoXuan VM 的编译功能把它编译为二进制格式（尚未完成）：

`$ anvm -a my.wat my.wasm`

或者使用 [wasm-tools](https://github.com/bytecodealliance/wasm-tools) 来编译：

`$ wasm-tools parse my.wat -o my.wasm`

编译完成之后将会得到 WASM 应用程序模块文件 `my.wasm`，然后就可以使用 XiaoXuan VM 运行它了：

`$ anvm my.wasm -f my::fib 10`

### 编译 C 语言程序

todo::

### 编译 Rust 语言程序

todo::

### 编译 Rust 的 Cargo 项目

todo::
