# XiaoXuan VM

A simple zero-dependency WebAssembly virtual machine.

Comes with an interactive REPL environment to easily decompile and debug programs, view stack and memory contents, and execute instructions on-the-fly, etc. The source code has very detailed comments for learning how WebAssembly works.

一个简单的零依赖的 WebAssembly 虚拟机。

附带一个 REPL 交互式环境，可以方便地反编译和调试程序，可以查看栈和内存的内容，以及即时执行指令等。源码有非常详细的注释，用于学习 WebAssembly 的工作原理。

## 测试

`$ cargo test`

## 编译

`$ cargo build`

编译完成之后将会在 `./target/debug/` 里面得到 `anvm` 和 `anvm-debug` 两个程序。也可以编译为发行版：

`$ cargo build --release`

编译后的程序将会在 `./target/release` 里。

## 示例

### 运行程序

`$ anvm app.wasm`

其中 `app.wasm` 为被运行的 WebAssembly 应用程序（模块），模块当中 `start` 段指向的函数或者名称为 `main` 的函数将会作为程序的入口。

如果模块文件不在当前目录，则需要写上模块文件的完整路径（也可以写相对路径），比如：

`$ anvm ~/myproject/first/app.wasm`

如果你不想编译就直接运行 XiaoXuan VM，也可以通过命令 `cargo run` 来启动 XiaoXuan VM，例如：

`$ cargo run --bin anvm -- app.wasm`

这跟 `$ anvm app.wasm` 的效果是一样的，也就是说，下面的例子当中的 `$ anvm ...` 都可以替换成 `$ cargo run --bin anvm -- ...` 。

### 运行多个模块的程序

有时一个 WebAssembly 应用程序可能由多个模块组成，如果要运行这种应用程序，则只需将所有模块（的文件名）传入 XiaoXuan VM 即可：

`$ anvm core.wasm std.wasm app.wasm`

程序的入口将从右侧的模块开始搜索。

### 指定入口函数及其参数

`$ anvm app.wasm --function app::add 123 456`

上面的 `app::add` 为指定的函数的名称，其中 `app` 为模块的名称，即 `*.wasm` 的文件名，`add` 则为该模块所导出的函数的名称，`123 456` 为函数的参数，多个参数之间使用空格分隔。

如果想启动一个未导出的函数，也可以通过函数的索引来启动，假设 `app.wasm` 模块里有一个函数 `sub`，其索引值为 `2`，则启动的命令为：

`$ anvm app.wasm --function app::2 123 456`

上述的命令参数 `--function` 可以使用 `-f` 来代替，比如：

`$ anvm app.wasm -f app::add 123 456`

### 反汇编

XiaoXuam VM 也提供了反汇编的功能，用于将二进制的 WASM 反汇编为文本格式，命令如下：

`$ anvm --disassembly input.wasm output.wat`

每次只能反汇编一个模块，其中的命令参数 `--disassembly` 可以使用 `-d` 来代替，比如：

`$ anvm -d input.wasm output.wat`
