# 实用工具

<!-- @import "[TOC]" {cmd="toc" depthFrom=1 depthTo=6 orderedList=false} -->

<!-- code_chunk_output -->

- [实用工具](#实用工具)
  - [脚本](#脚本)
  - [wasm-tools](#wasm-tools)
    - [Wasm 的文本和二进制格式的相互转换](#wasm-的文本和二进制格式的相互转换)
    - [查看 Wasm 二进制的全部信息](#查看-wasm-二进制的全部信息)
    - [查看 Wasm 二进制的段信息](#查看-wasm-二进制的段信息)
  - [wabt](#wabt)
    - [Wasm 的文本和二进制格式的相互转换](#wasm-的文本和二进制格式的相互转换-1)
    - [查看 Wasm 二进制的信息](#查看-wasm-二进制的信息)
    - [运行 Wasm 的所有导出函数](#运行-wasm-的所有导出函数)
  - [编译 Rust 到 Wasm](#编译-rust-到-wasm)
    - [编译单独一个 Rust 源码文件](#编译单独一个-rust-源码文件)
    - [编译一个 cargo 项目](#编译一个-cargo-项目)
  - [编译 C 到 wasm](#编译-c-到-wasm)
  - [wasm-pack 和 wasm-bindgen](#wasm-pack-和-wasm-bindgen)

<!-- /code_chunk_output -->

## 脚本

本目录附带了几个脚本，用于方便地编译 C、Rust、Wat 到 WebAssembly 字节码：

- c2wasm 编译单独一个 C 源码文件到 Wasm
- rust2wasm 编译单独一个 Rust 源码文件到 Wasm
- wat2wasm 编译 Wat 到 Wasm

另外还有：

- wasm2wat 转换 Wasm 二进制格式到文本格式（可以理解为反编译）
- wasmdump 显示 Wasm 二进制格式内容（可以理解为反编译的同时，显示结果到屏幕）

直接运行它们即可阅读它们的使用方法。

## wasm-tools

程序的源码：

[https://github.com/bytecodealliance/wasm-tools](https://github.com/bytecodealliance/wasm-tools)

安装：

`$ cargo install wasm-tools`

### Wasm 的文本和二进制格式的相互转换

文本转二进制：

`$ wasm-tools parse hello.wat -o hello.wasm`

二进制转文本：

`wasm-tools print hello.wasm`

### 查看 Wasm 二进制的全部信息

显示二进制信息，及其对应的文本格式的内容，相当于一边反编译一边显示内容：

`$ wasm-tools dump hello.wasm`

### 查看 Wasm 二进制的段信息

`$ wasm-tools objdump hello.wasm`

## wabt

程序的源码：

[https://github.com/WebAssembly/wabt](https://github.com/WebAssembly/wabt)

### Wasm 的文本和二进制格式的相互转换

文本转二进制：

`$ wat2wasm hello.wat`

二进制转文本：

`$ wasm2wat hello.wasm`

### 查看 Wasm 二进制的信息

`$ wasm-objdump -h hello.wasm`

可选参数：

- `-h, --headers`
  显示头信息

- `-j, --section=SECTION`
  只显示指定段的信息

- `-s, --full-contents`
  显示原始的内容

- `-d, --disassemble`
  反编译函数的字节码

### 运行 Wasm 的所有导出函数

`$ wasm-interp test.wasm --run-all-exports`

## 编译 Rust 到 Wasm

先 [安装 Rust](https://www.rust-lang.org/tools/install)，然后添加 `target` 之 `wasm32-unknown-unknown`：

`$ rustup target add wasm32-unknown-unknown`

### 编译单独一个 Rust 源码文件

`$ rustc --target wasm32-unknown-unknown -C lto -O --crate-type=cdylib hello.rs -o hello.wasm`

### 编译一个 cargo 项目

确保在 `Cargo.toml` 里面指定了 crate 的类型为 `cdylib`：

```toml
[lib]
path = "src/lib.rs"
crate-type = ["cdylib"]
```

然后开始编译：

`$ cargo build --target wasm32-unknown-unknown --release`

## 编译 C 到 wasm

先安装 clang（LLVM），然后使用 `clang` 命令编译：

`$ clang --target=wasm32 -c --no-standard-libraries hello.c -o hello.wasm`

参考：

- [Compiling C to WebAssembly and Running It - without Emscripten](https://depth-first.com/articles/2019/10/16/compiling-c-to-webassembly-and-running-it-without-emscripten/)
- [Compiling C to WebAssembly without Emscripten](https://dassur.ma/things/c-to-webassembly/)
- [Compiling C to WebAssembly using clang/LLVM and WASI](https://00f.net/2019/04/07/compiling-to-webassembly-with-llvm-and-clang/)

## wasm-pack 和 wasm-bindgen

这两个工具用于构建可以跟 JavaScript 互动的 Wasm 程序，参考：[Compiling from Rust to WebAssembly](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)
