// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_engine::vm_memory::VMMemory;

use crate::{error::Errno, wasi_module_context::WASIModuleContext};

/// # args_sizes_get() -> (errno, size, size)
///
/// Return command-line argument data sizes.
///
/// Params
///
/// Results
/// - error: errno
///   - Fault: if either are invalid due to memory constraints
/// - argc: size The number of arguments.
/// - argv_buf_size: size The size of the argument string data.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-args_sizes_get---errno-size-size
///
/// 获取命令行参数的个数和数据总长度
///
/// 假设有两个命令行参数 `foo` 和 `--list`，则 `argc` 的值为 2，
/// argv_buf_size 的值为 (3+1) + (6+1) = 11，而不是 3 + 6 = 9，因为
/// 每个参数值（字符串）后面还有一个 `\0` 字符。
///
/// 返回 (argrment_count, argument_size)
pub fn args_sizes_get(module_context: &mut WASIModuleContext) -> Result<(u32, u32), Errno> {
    let arguments = &module_context.arguments;
    let argrment_count = arguments.len();

    let mut argument_size: usize = 0;
    for argument in arguments {
        // 每个参数值后面需要多加一个 `\0` 字符，所以每个值的长度需要额外加 1.
        let value_length = argument.as_bytes().len() + 1;
        argument_size += value_length;
    }

    Ok((argrment_count as u32, argument_size as u32))
}

/// # args_get(argv: Pointer<Pointer<u8>>, argv_buf: Pointer<u8>) -> errno
///
/// Read command-line argument data. The size of the array should match that returned by args_sizes_get
///
/// Params
/// - argv: Pointer<Pointer<u8>>
/// - argv_buf: Pointer<u8>
///
/// Results
/// - error: errno
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-args_getargv-pointerpointeru8-argv_buf-pointeru8---errno
///
/// 写下列数据到内存：
/// 1. （每个参数值的起始位置）列表。
/// 2. 所有参数值（的拼接）。
pub fn args_get(
    memory_block: &mut VMMemory,
    module_context: &mut WASIModuleContext,
    argv_address_list_offset: usize,
    argv_buffer_offset: usize,
) -> Result<(), Errno> {
    let mut list_address = argv_address_list_offset;
    let mut content_address = argv_buffer_offset;

    let arguments = &module_context.arguments;
    for argument in arguments {
        // 先写入列表元素
        memory_block.write_i32(list_address, content_address as i32);
        list_address += 4; // 数值 i32 的长度是 4 bytes

        // 再写入参数值
        let content_bytes = argument.as_bytes();
        let content_length = content_bytes.len();
        memory_block.write_bytes(content_address, content_bytes);
        memory_block.write_i8(content_address + content_length, 0); // 紧接着参数值后面写入字符 `\0`
        content_address += content_length + 1;
    }

    // 命令行参数的示例：
    //
    // 假设通过如下命令启动程序：
    //
    // `$ app -d demo.wasm`
    //
    // 对于如下的标准的 C 程序
    //
    // `int main(int argc, char *argv[])`
    //
    // argc 的值为 3
    // argv 的值如下：
    //      argv -> |p0| -> "app\0"，程序名/路径
    //              |p1| -> "-o\0"，第一个参数
    //              |p2| -> "demo.wasm\0"，第二个参数
    //              |p3| -> NULL, 一个空指针
    //
    // 注意，对于 C 的函数 strlen，其获得的是内容的长度，该值不包括 `\0` 字符，例如：
    // `strlen("abc")` 的值是 3，
    // 但字符串字面量 "abc" 在编译时，会转为 "abc\0" 来储存，也就是说，在内存里，
    // 字符串字面量 "abc" 的实际长度（占用的空间）是 4。

    // 注：对于 C 的 argv，假设有 N 个参数（包括程序名），argv 实际上
    // 会有 "N+1" 个元素，其中有 N 个有效元素，
    // 最后一个元素的值为 0,代表指针 NULL，但 WASI 模块似乎不需要写
    // 第 N+1 个元素（即数值 0）到内存，暂时不知道其中的原理。

    Ok(())
}
