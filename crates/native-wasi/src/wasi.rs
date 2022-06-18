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
use anvm_engine::{error::NativeError, native_module::NativeModule};

use crate::{
    filesystem_context::FileSystemContext, types::MODULE_NAME,
    wasi_module_context::WASIModuleContext,
};

pub fn new_wasi_module(module_context: WASIModuleContext) -> NativeModule {
    let filesystem_context = FileSystemContext::new();

    // NOTE::
    // 测试用的 module context
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
        filesystem_context,
    );

    let mut native_module = NativeModule::new(MODULE_NAME, Box::new(module_context));

    native_module.add_native_function(
        "fd_fdstat_get",
        vec![ValueType::I32, ValueType::I32],
        vec!["fd", "result_offset"],
        vec![ValueType::I32],
        fd_fdstat_get,
    );

    native_module
}

/// fd_fdstat_get
/// (func $fd_fdstat_get (param $fd i32) (param $result_offset i32) (result (;errno;) i32)))
fn fd_fdstat_get(
    _native_module: &mut NativeModule,
    args: &[Value],
) -> Result<Vec<Value>, NativeError> {
    // 这里不需要检查参数的数量和数据类型，因为 engine 在调用本地函数时已经检查过，
    // 下面的本地函数均相同。
    todo!()
}
