// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # WASI 标准及说明
//!
//! WASI 标准的详细文档见：
//! https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md
//!
//! 注意上面的链接是版本 snapshot-01
//! 跟最新版本的文档是不同的：
//! https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md

//! # ABI 实现的顺序
//!
//! 为了方便调试，本模块从最简单的应用程序所需要的 ABI 开始实现，具体是：
//!
//! 最简单的 C Hello World 程序（即写数据到文件 stdout），所需要的 ABI 有：
//!
//! - fd_fdstat_get
//! - fd_seek
//! - fd_write
//! - fd_close
//!
//! 在此基础上，要读取程序的 args，则需要 ABI：
//!
//! - args_get
//! - args_sizes_get
//!
//! 要读取环境变量，则需要 ABI：
//!
//! - environ_get
//! - environ_sizes_get
//!
//! 通常 C 程序的 main 函数会返回一个整数，作为程序退出的代码（exit code），
//! 通常 0 表示正常退出，非 0 表示非正常退出，为实现该功能，则需要 ABI：
//!
//! - proc_exit
//!
//! 涉及读文件时，需要 ABI：
//!
//! - fd_read
//!
//! 涉及打开文件系统的文件时，需要 ABI：
//!
//! - path_open
//! - fd_close
//! - fd_prestat_get
//! - fd_prestat_dir_name
//!
//! 注意，WASM 应用程序运行在一个沙盘中，在此环境里看到的文件系统是 host 文件系统
//! 的一个映射，即在 WASI 应用程序里，所有的文件路径都是相对于 host 某一个（或几个）目录，
//! 无法直接通过一个绝对文件路径来打开一个存在 host 文件系统同样路径的文件。
//!
//! 其他 ABI 还有：
//!
//! - clock_res_get
//! - clock_time_get
//!
//! - fd_advise
//! - fd_allocate
//! - fd_datasync
//! - fd_fdstat_set_flags
//! - fd_fdstat_set_rights
//! - fd_filestat_get
//! - fd_filestat_set_size
//! - fd_filestat_set_times
//! - fd_pread
//! - fd_pwrite
//! - fd_readdir
//! - fd_renumber
//! - fd_sync
//! - fd_tell
//!
//! - path_create_directory
//! - path_filestat_get
//! - path_filestat_set_times
//! - path_link
//! - path_readlink
//! - path_remove_directory
//! - path_rename
//! - path_symlink
//! - path_unlink_file
//! - poll_oneoff
//!
//! - proc_raise
//! - sched_yield
//! - random_get
//!
//! - sock_recv
//! - sock_send
//! - sock_shutdown

use std::io;

use anvm_ast::types::{Value, ValueType};
use anvm_engine::{error::NativeError, native_module::NativeModule, vm::VM};

use crate::{
    error::Errno,
    native_fd,
    types::{Whence, MODULE_NAME},
    wasi_module_context::WASIModuleContext,
};

pub fn new_wasi_module(module_context: WASIModuleContext) -> NativeModule {
    // NOTE::
    // ===============================================================
    // 测试用的 module context ---------------------------------------\\
    let module_context = WASIModuleContext::new(
        "demo",
        vec!["-l".to_string(), "123".to_string()],
        vec![
            ("USER".to_string(), "YANG".to_string()),
            ("EDITOR".to_string(), "vim".to_string()),
        ],
        Box::new(io::stdin()),
        Box::new(io::stdout()),
        Box::new(io::stderr()),
    );
    // 测试用的 module context ---------------------------------------//
    // ===============================================================

    let mut native_module = NativeModule::new(MODULE_NAME, Box::new(module_context));

    native_module.add_native_function(
        "fd_fdstat_get",
        vec![ValueType::I32, ValueType::I32],
        vec!["fd", "result.fdstat"],
        vec![ValueType::I32],
        fd_fdstat_get,
    );

    native_module.add_native_function(
        "fd_seek",
        vec![
            ValueType::I32,
            ValueType::I64,
            ValueType::I32,
            ValueType::I32,
        ],
        vec!["fd", "offset", "whence", "result.newoffset"],
        vec![ValueType::I32],
        fd_seek,
    );

    native_module
}

/// fd_fdstat_get
/// https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md#-fd_fdstat_getfd-fd---resultfdstat-errno
///
/// (func $wasi.fd_fdstat_get (param $fd i32) (param $result.fdstat i32) (result (;errno;) i32)))
///
/// `$result.fdstat` 是指函数正确运行之后得到的结果 `fdstat` 储存在内存的位置（地址）
fn fd_fdstat_get(
    vm: &mut VM,
    native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeError> {
    // 这里不需要检查参数的数量和数据类型，因为 engine 在调用本地函数时已经检查过，
    // 下面的本地函数均相同。

    let fd = if let Value::I32(fd) = args[0] {
        fd as u32
    } else {
        unreachable!()
    };

    let result_fdstat_offset = if let Value::I32(result_fdstat_offset) = args[1] {
        result_fdstat_offset
    } else {
        unreachable!()
    };

    match native_fd::fd_fdstat_get(get_wasi_module_context(vm, native_module_index), fd) {
        Ok(fd_stat) => {
            // todo::
            // 写 fdstat 到内存指定位置 `result_fdstat_offset`
            //

            make_success_result()
        }
        Err(errno) => make_error_result(errno),
    }
}

/// fd_seek
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-fd_seekfd-fd-offset-filedelta-whence-whence---errno-filesize
///
/// `(func $wasi.fd_seek (param $fd i32) (param $offset i64) (param $whence i32) (param $result.newoffset i32) (result (;errno;) i32)))`
///
/// `$result.newoffset` 是指函数正确运行之后得到的结果 `newoffset` 储存在内存的位置（地址）
fn fd_seek(
    vm: &mut VM,
    native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeError> {
    let fd = if let Value::I32(fd) = args[0] {
        fd as u32
    } else {
        unreachable!()
    };

    let offset = if let Value::I64(offset) = args[1] {
        offset
    } else {
        unreachable!()
    };

    let whence_i32 = if let Value::I32(whence) = args[2] {
        whence
    } else {
        unreachable!()
    };

    let result_newoffset_offset = if let Value::I32(result_newoffset_offset) = args[1] {
        result_newoffset_offset
    } else {
        unreachable!()
    };

    let result_whence = Whence::try_from(whence_i32 as u8);

    if let Ok(whence) = result_whence {
        match native_fd::fd_seek(
            get_wasi_module_context(vm, native_module_index),
            fd,
            offset,
            whence,
        ) {
            Ok(newoffset) => {
                let mem = &mut vm.resource.memory_blocks[0];
                mem.write_i64(result_newoffset_offset as usize, newoffset as i64);

                make_success_result()
            }
            Err(errno) => make_error_result(errno),
        }
    } else {
        make_error_result(Errno::Invalid)
    }
}

fn get_wasi_module_context(vm: &mut VM, native_module_index: usize) -> &mut WASIModuleContext {
    let any_module_context = &mut vm.resource.native_modules[native_module_index].module_context;
    any_module_context
        .as_any()
        .downcast_mut::<WASIModuleContext>()
        .unwrap()
}

fn make_success_result() -> Result<Vec<Value>, NativeError> {
    Ok(vec![Value::I32(u16::from(Errno::Success) as i32)])
}

fn make_error_result(errno: Errno) -> Result<Vec<Value>, NativeError> {
    Ok(vec![Value::I32(u16::from(errno) as i32)])
}
