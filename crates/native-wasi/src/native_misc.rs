// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_engine::vm_memory::VMMemory;

use crate::{error::Errno, types::ClockId, wasi_module_context::WASIModuleContext};

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
/// 获取命令行参数（包括程序名）的个数和数据总长度
///
/// 假设程序 `foo` 有一个命令行参数 `--list`，则 `argc` 的值为 2，
/// 参数值分别为：
///
/// - "foo\0"
/// - "--list\0"
///
/// argv_buf_size 的值为 (3+1) + (6+1) = 11，而不是 3 + 6 = 9，
/// 因为每个参数值（字符串）后面还会被额外添加一个 `\0` 字符。
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
/// 1. 每个参数值的起始位置的列表。
/// 2. 所有参数值。
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
    //
    // argv -> |p0| -> "app\0"，程序名/路径
    //         |p1| -> "-o\0"，第一个参数
    //         |p2| -> "demo.wasm\0"，第二个参数
    //         |p3| -> NULL, 一个空指针
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

/// # environ_sizes_get() -> (errno, size, size)
///
/// Return environment variable data sizes.
///
/// Params
///
/// Results
/// - error: errno
/// - environc: size The number of environment variable arguments.
/// - environ_buf_size: size The size of the environment variable data.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-environ_sizes_get---errno-size-size
///
/// 获取环境变量的个数和数据总长度
///
/// 假设有如下环境变量：
/// - key: `USER`, value: `yang`
/// - key: `HOME`, value: `/home/yang`
///
/// 则环境变量个数为 2，
///
/// 环境变量的值写入到内存时（或者说在内存里的布局）是以 `key=value\0` 的形式存在的，
/// 所以上述环境变量在内存的数据如下：
///
/// - `USER=yang\0`
/// - `HOME=/home/yang\0`
///
/// 数据总长度为 (4+1+4+1) + (4+1+10+1) = 36，
/// 注意 每个环境变量值（字符串）后面会被额外添加一个 `\0` 字符。
///
/// 返回 (environ_count, environ_size)
pub fn environ_sizes_get(module_context: &mut WASIModuleContext) -> Result<(u32, u32), Errno> {
    let environs = &module_context.environments;
    let environ_count = environs.len();

    let mut environ_size: usize = 0;
    for environ in environs {
        // 每个 key 后面需要多添加一个 `=` 字符，
        // 每个 value 后面需要多添加一个 `\0` 字符。
        // 所以每个 environ 的长度需要额外添加 2.
        let key_length = environ.0.as_bytes().len();
        let value_length = environ.1.as_bytes().len();
        environ_size += key_length + value_length + 2;
    }

    Ok((environ_count as u32, environ_size as u32))
}

/// # environ_get(environ: Pointer<Pointer<u8>>, environ_buf: Pointer<u8>) -> errno
///
/// Read environment variable data. The sizes of the buffers should match that returned by environ_sizes_get.
///
/// Params
/// - environ: Pointer<Pointer<u8>>
/// - environ_buf: Pointer<u8>
///
/// Results
/// - error: errno
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-environ_getenviron-pointerpointeru8-environ_buf-pointeru8---errno
pub fn environ_get(
    memory_block: &mut VMMemory,
    module_context: &mut WASIModuleContext,
    environ_address_list_offset: usize,
    environ_buffer_offset: usize,
) -> Result<(), Errno> {
    let mut list_address = environ_address_list_offset;
    let mut content_address = environ_buffer_offset;

    let environs = &module_context.environments;
    for environ in environs {
        // 先写入列表元素
        memory_block.write_i32(list_address, content_address as i32);
        list_address += 4; // 数值 i32 的长度是 4 bytes

        // 再写入参数值
        let content = format!("{}={}", environ.0, environ.1);
        let content_bytes = content.as_bytes();
        let content_length = content_bytes.len();
        memory_block.write_bytes(content_address, content_bytes);
        memory_block.write_i8(content_address + content_length, 0); // 紧接着参数值后面写入字符 `\0`
        content_address += content_length + 1;
    }

    Ok(())
}

/// # clock_time_get(id: clockid, precision: timestamp) -> (errno, timestamp)
///
/// Return the time value of a clock. Note: This is similar to clock_gettime in POSIX.
///
/// Params
/// - id: clockid The clock for which to return the time.
/// - precision: timestamp The maximum lag (exclusive) that the returned time value may have, compared to its actual value.
///
/// Results
/// - error: errno
/// - time: timestamp The time value of the clock.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-clock_time_getid-clockid-precision-timestamp---errno-timestamp
pub fn clock_time_get(
    module_context: &mut WASIModuleContext,
    clock_id: ClockId,
    _precision: u64,
) -> Result<u64, Errno> {
    // `precision`（精度）目前用不着

    // 返回纳秒
    match clock_id {
        ClockId::Realtime => {
            let (seconds, nanoseconds) = module_context.realtime_clock.get_clock();
            let ns: u64 = seconds * 1000 * 1000 * 1000 + nanoseconds as u64;
            Ok(ns)
        }
        ClockId::Monotonic => Err(Errno::Nosys),
        ClockId::ProcessCputimeId => Err(Errno::Nosys),
        ClockId::ThreadCputimeId => Err(Errno::Nosys),
    }
}

/// # clock_res_get(id: clockid) -> (errno, timestamp)
///
/// Return the resolution of a clock. Implementations are required to provide a non-zero value for supported clocks. For unsupported clocks, return errno::inval. Note: This is similar to clock_getres in POSIX.
///
/// Params
/// - id: clockid The clock for which to return the resolution.
///
/// Results
/// - error: errno
/// - resolution: timestamp The resolution of the clock.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-clock_res_getid-clockid---errno-timestamp
pub fn clock_res_get(
    module_context: &mut WASIModuleContext,
    clock_id: ClockId,
) -> Result<u64, Errno> {
    // 返回纳秒
    match clock_id {
        ClockId::Realtime => Ok(module_context.realtime_clock.get_resolution()),
        ClockId::Monotonic => Err(Errno::Nosys),
        ClockId::ProcessCputimeId => Err(Errno::Nosys),
        ClockId::ThreadCputimeId => Err(Errno::Nosys),
    }
}

/// # random_get(buf: Pointer<u8>, buf_len: size) -> errno
///
/// Write high-quality random data into a buffer. This function blocks when the implementation is unable to immediately provide sufficient high-quality random data. This function may execute slowly, so when large mounts of random data are required, it's advisable to use this function to seed a pseudo-random number generator, rather than to provide the random data directly.
///
/// Params
/// - buf: Pointer<u8> The buffer to fill with random data.
/// - buf_len: size
///
/// Results
/// - error: errno
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-random_getbuf-pointeru8-buf_len-size---errno
pub fn random_get(
    module_context: &mut WASIModuleContext,
    data: &mut [u8],
) -> Result<usize, Errno> {
    // let mut data: Vec<u8> = vec![0; size as usize];
    module_context.random_source.read(data).unwrap();
    Ok(data.len())
}
