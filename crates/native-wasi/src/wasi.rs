// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # WASI API 标准
//!
//! WASI API 标准的详细文档见：
//! https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md
//!
//! 注意上面的链接是版本 snapshot-01，跟当前最新版本的文档是不同的：
//! https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md

//! # ABI
//!
//! WASI API 标准文档里列出的是 API 的详细情况，至于 ABI 在哪里定义，hemashushu 暂时还没找到。
//! 有关 ABI 的资料这里主要参考自项目 [wazero](https://github.com/tetratelabs/wazero)。
//!
//! 注：WASM VM 是栈式的虚拟机，这里说的 ABI 是指，当一个 WASM 应用程序调用一个 WASI 函数时，
//! 应该传入（压入）怎样的参数（参数的个数和数据类型），应该传出（弹出）怎样的数据。

//! # API 实现的顺序
//!
//! 为了方便调试，本模块从最简单的应用程序所需要的 API 开始实现，具体是：
//!
//! 最简单的 C Hello World 程序（即写数据到文件 stdout），所需要的 API 有：
//!
//! - fd_write
//!
//! 编译程序会同时导入下面 3 个 API：
//!
//! - fd_fdstat_get
//! - fd_seek
//! - fd_close
//!
//! 通常 C 程序的 main 函数会返回一个整数，作为程序退出的代码（exit code），
//! 通常 0 表示正常退出，非 0 表示非正常退出，为实现该功能，需要 API：
//!
//! - proc_exit
//!
//! 涉及读文件时，需要 API：
//!
//! - fd_read
//!
//! 要读取程序的 args，需要 API：
//!
//! - args_get
//! - args_sizes_get
//!
//! 要读取环境变量，需要 API：
//!
//! - environ_get
//! - environ_sizes_get
//!
//! 涉及打开文件系统的文件时，需要 API：
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
//! 其他 API 还有：
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

use anvm_ast::types::{Value, ValueType};
use anvm_engine::{
    error::{NativeError, NativeTerminate},
    native_module::{ModuleContext, NativeModule},
    vm::VM,
};

use crate::{
    error::Errno,
    native_fd,
    types::{CIOVec, Deserialize, Serialize, Whence, MODULE_NAME},
    wasi_module_context::WASIModuleContext,
};

pub fn new_wasi_module(module_context: WASIModuleContext) -> NativeModule {
    let mut native_module = NativeModule::new(MODULE_NAME, Box::new(module_context));

    native_module.add_native_function(
        "fd_write",
        vec![
            ValueType::I32,
            ValueType::I32,
            ValueType::I32,
            ValueType::I32,
        ],
        vec!["fd", "iovs", "iovs_len", "result.size"],
        vec![ValueType::I32],
        fd_write,
    );

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

    native_module.add_native_function(
        "fd_close",
        vec![ValueType::I32],
        vec!["fd"],
        vec![ValueType::I32],
        fd_close,
    );

    native_module.add_native_function(
        "proc_exit",
        vec![ValueType::I32],
        vec!["exit_code"],
        vec![],
        proc_exit,
    );

    native_module.add_native_function(
        "fd_read",
        vec![
            ValueType::I32,
            ValueType::I32,
            ValueType::I32,
            ValueType::I32,
        ],
        vec!["fd", "iovs", "iovs_len", "result.size"],
        vec![ValueType::I32],
        fd_read,
    );

    native_module
}

/// # fd_write
///
/// `(func $wasi.fd_write (param $fd i32) (param $iovs i32) (param $iovs_len i32) (param $result.size i32) (result (;errno;) i32)))`
///
/// - $fd：文件描述符
/// - $iovs：IOVecs 结构体实例在内存中的开始位置
/// - $iovs_len：IOVecs 实例的数量
/// - $result.size：函数的结果，即 `size`，储存在内存的位置
fn fd_write(
    vm: &mut VM,
    native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeTerminate> {
    let fd = if let Value::I32(fd) = args[0] {
        fd as u32
    } else {
        unreachable!()
    };

    let iovecs_offset = if let Value::I32(iovecs_offset) = args[1] {
        iovecs_offset as usize
    } else {
        unreachable!()
    };

    let iovecs_len = if let Value::I32(iovecs_len) = args[2] {
        iovecs_len as usize
    } else {
        unreachable!()
    };

    let result_size_offset = if let Value::I32(result_size_offset) = args[3] {
        result_size_offset as usize
    } else {
        unreachable!()
    };

    let mut ciovecs: Vec<CIOVec> = vec![];
    let ciovec_data_size = CIOVec::get_deserialize_size();

    {
        let memory_block = &vm.resource.memory_blocks[0];
        for idx in 0..iovecs_len {
            let data = memory_block.read_bytes(
                (idx * ciovec_data_size + iovecs_offset) as usize,
                ciovec_data_size,
            );

            let ciovec = CIOVec::deserialize(data);
            ciovecs.push(ciovec);
        }
    }

    let any_module_context = &mut vm.resource.native_modules[native_module_index].module_context;
    let memory_block = &mut vm.resource.memory_blocks[0];

    match native_fd::fd_write(
        memory_block,
        get_wasi_module_context(any_module_context),
        fd,
        &ciovecs,
    ) {
        Ok(wrote_bytes) => {
            memory_block.write_i32(result_size_offset, wrote_bytes as i32);
            make_success_result()
        }
        Err(errno) => make_error_result(errno),
    }
}

/// # fd_fdstat_get
///
/// (func $wasi.fd_fdstat_get (param $fd i32) (param $result.fdstat i32) (result (;errno;) i32)))
///
/// - $fd：文件描述符（所谓文件描述符、文件句柄，可以理解为一个代表着某个已打开的文件的数字）
/// - $result.fdstat：函数正确运行之后得到的结果，即 `fdstat`，储存在内存的位置（地址）
///
/// 注意，函数的运行结果，即 `fdstat` 在内存中所储存的位置，是由调用者指定的，而不是
/// 由被调用的函数 “分配内存——写入数据——返回地址”，也就是说，WASI 的函数风格跟传统的 libc 函数类似，
/// 由调用者负责分配内存（及指定存放结果的位置）。
fn fd_fdstat_get(
    vm: &mut VM,
    native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeTerminate> {
    // 这里不需要检查参数的数量和数据类型，因为 engine 在调用本地函数时已经检查过，
    // 下面的本地函数均相同。

    let fd = if let Value::I32(fd) = args[0] {
        fd as u32
    } else {
        unreachable!()
    };

    let result_fdstat_offset = if let Value::I32(result_fdstat_offset) = args[1] {
        result_fdstat_offset as usize
    } else {
        unreachable!()
    };

    let any_module_context = &mut vm.resource.native_modules[native_module_index].module_context;

    match native_fd::fd_fdstat_get(get_wasi_module_context(any_module_context), fd) {
        Ok(fd_stat) => {
            let mut data = Vec::<u8>::with_capacity(fd_stat.get_serialize_size());
            fd_stat.write(&mut data);

            // 写 fdstat 到内存指定位置 `result_fdstat_offset`
            let memory_block = &mut vm.resource.memory_blocks[0];
            memory_block.write_bytes(result_fdstat_offset, &data);

            make_success_result()
        }
        Err(errno) => make_error_result(errno),
    }
}

/// # fd_seek
///
/// `(func $wasi.fd_seek (param $fd i32) (param $offset i64) (param $whence i32) (param $result.newoffset i32) (result (;errno;) i32)))`
///
/// - $fd：文件描述符
/// - $offset：偏移量
/// - $whence：偏移类型
/// - $result.newoffset：函数正确运行之后得到的结果，即 `newoffset`，储存在内存的位置（地址）
fn fd_seek(
    vm: &mut VM,
    native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeTerminate> {
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

    let result_newoffset_offset = if let Value::I32(result_newoffset_offset) = args[3] {
        result_newoffset_offset as usize
    } else {
        unreachable!()
    };

    if let Ok(whence) = Whence::try_from(whence_i32 as u8) {
        let any_module_context =
            &mut vm.resource.native_modules[native_module_index].module_context;

        match native_fd::fd_seek(
            get_wasi_module_context(any_module_context),
            fd,
            offset,
            whence,
        ) {
            Ok(newoffset) => {
                let memory_block = &mut vm.resource.memory_blocks[0];
                memory_block.write_i64(result_newoffset_offset, newoffset as i64);

                make_success_result()
            }
            Err(errno) => make_error_result(errno),
        }
    } else {
        make_error_result(Errno::Invalid)
    }
}

/// # fd_close
///
/// `(func $wasi.fd_close (param $fd i32) (result (;errno;) i32)))`
///
/// - $fd：文件描述符
fn fd_close(
    vm: &mut VM,
    native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeTerminate> {
    let fd = if let Value::I32(fd) = args[0] {
        fd as u32
    } else {
        unreachable!()
    };

    let any_module_context = &mut vm.resource.native_modules[native_module_index].module_context;
    match native_fd::fd_close(get_wasi_module_context(any_module_context), fd) {
        Ok(_) => make_success_result(),
        Err(errno) => make_error_result(errno),
    }
}

/// # proc_exit
///
/// `(func $wasi.proc_exit (param $exit_code i32)))`
///
/// - exit_code: 程序的退出码
///   linux shell 得到的是 (exit_code % 256)，比如 exit(456)，实际得到的返回码是：456 % 256 = 200。
///   https://doc.rust-lang.org/stable/std/process/fn.exit.html
fn proc_exit(
    _vm: &mut VM,
    _native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeTerminate> {
    let exit_code = if let Value::I32(exit_code) = args[0] {
        exit_code
    } else {
        unreachable!()
    };

    // todo
    // 关闭所有已打开的文件

    Err(NativeTerminate {
        module_name: MODULE_NAME.to_owned(),
        native_error: NativeError::Exit(exit_code),
    })
}

/// # fd_read
///
/// `(func $wasi.fd_read (param $fd i32) (param $iovs i32) (param $iovs_len i32) (param $result.size i32) (result (;errno;) i32)))`

/// - $fd：文件描述符
/// - $iovs：IOVecs 结构体实例在内存中的开始位置
/// - $iovs_len：IOVecs 实例的数量
/// - $result.size：函数的结果，即 `size`，储存在内存的位置
fn fd_read(
    vm: &mut VM,
    native_module_index: usize,
    args: &[Value],
) -> Result<Vec<Value>, NativeTerminate> {
    let fd = if let Value::I32(fd) = args[0] {
        fd as u32
    } else {
        unreachable!()
    };

    let iovecs_offset = if let Value::I32(iovecs_offset) = args[1] {
        iovecs_offset as usize
    } else {
        unreachable!()
    };

    let iovecs_len = if let Value::I32(iovecs_len) = args[2] {
        iovecs_len as usize
    } else {
        unreachable!()
    };

    let result_size_offset = if let Value::I32(result_size_offset) = args[3] {
        result_size_offset as usize
    } else {
        unreachable!()
    };

    let mut ciovecs: Vec<CIOVec> = vec![];
    let ciovec_data_size = CIOVec::get_deserialize_size();

    {
        let memory_block = &vm.resource.memory_blocks[0];
        for idx in 0..iovecs_len {
            let data = memory_block.read_bytes(
                (idx * ciovec_data_size + iovecs_offset) as usize,
                ciovec_data_size,
            );

            let ciovec = CIOVec::deserialize(data);
            ciovecs.push(ciovec);
        }
    }

    let any_module_context = &mut vm.resource.native_modules[native_module_index].module_context;
    let memory_block = &mut vm.resource.memory_blocks[0];

    match native_fd::fd_read(
        memory_block,
        get_wasi_module_context(any_module_context),
        fd,
        &ciovecs,
    ) {
        Ok(read_bytes) => {
            memory_block.write_i32(result_size_offset, read_bytes as i32);
            make_success_result()
        }
        Err(errno) => make_error_result(errno),
    }
}

fn get_wasi_module_context(
    any_module_context: &mut Box<dyn ModuleContext>,
) -> &mut WASIModuleContext {
    any_module_context
        .as_any()
        .downcast_mut::<WASIModuleContext>()
        .unwrap()
}

fn make_success_result() -> Result<Vec<Value>, NativeTerminate> {
    Ok(vec![Value::I32(u16::from(Errno::Success) as i32)])
}

fn make_error_result(errno: Errno) -> Result<Vec<Value>, NativeTerminate> {
    Ok(vec![Value::I32(u16::from(errno) as i32)])
}

#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        env, fs,
        io::{self, Read, Write},
        rc::Rc,
    };

    use anvm_ast::{ast, types::Value};
    use anvm_binary_parser::parser;
    use anvm_engine::{
        error::{EngineError, NativeError, NativeTerminate},
        instance::{create_instance, find_ast_module_export_function},
        native_module::NativeModule,
        object::NamedAstModule,
    };

    use crate::{types::MODULE_NAME, wasi_module_context::WASIModuleContext};

    use super::new_wasi_module;
    use pretty_assertions::assert_eq;

    // 辅助方法
    fn get_test_binary_resource(filename: &str) -> Vec<u8> {
        let mut path_buf = env::current_dir().unwrap();

        // 使用 `cargo test` 测试时，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm/crates/native-wasi`；
        //
        // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm`。
        //
        // 下面语句用于处理这种情况。

        if !path_buf.ends_with("native-wasi") {
            path_buf.push("crates");
            path_buf.push("native-wasi");
        }
        let fullname_buf = path_buf.join("resources").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!(
            "failed to read the specified binary file: {}",
            fullname
        ))
    }

    fn get_test_ast_module(filename: &str) -> ast::Module {
        let bytes = get_test_binary_resource(filename);
        parser::parse(&bytes).unwrap()
    }

    fn get_test_wasi_module_context(
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
        stderr: Rc<RefCell<dyn Write>>,
    ) -> WASIModuleContext {
        WASIModuleContext::new(
            "demo",
            vec!["-l".to_string(), "123".to_string()],
            vec![
                ("USER".to_string(), "YANG".to_string()),
                ("EDITOR".to_string(), "vim".to_string()),
            ],
            stdin,
            stdout,
            stderr,
        )
    }

    fn get_test_native_module(
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
        stderr: Rc<RefCell<dyn Write>>,
    ) -> NativeModule {
        let wasi_module_context = get_test_wasi_module_context(stdin, stdout, stderr);
        new_wasi_module(wasi_module_context)
    }

    fn eval(
        filename: &str,
        export_function_name: &str,
        args: &[Value],
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
        stderr: Rc<RefCell<dyn Write>>,
    ) -> Result<Vec<Value>, EngineError> {
        let ast_module = get_test_ast_module(filename);

        let function_index = find_ast_module_export_function(&ast_module, export_function_name)
            .expect(&format!("function {} not found", export_function_name));

        let named_ast_module = NamedAstModule::new("test", ast_module);
        let wasi_native_module = get_test_native_module(stdin, stdout, stderr);
        let mut vm = create_instance(vec![wasi_native_module], &vec![named_ast_module])?;
        vm.eval_function_by_index(0, function_index as usize, args)
    }

    #[test]
    fn test_stdout_write() {
        let module_name = "test-stdout-write.wasm";

        // 测试函数 `write_string`
        let stdout1 = Rc::new(RefCell::new(Vec::<u8>::new()));
        let clone_stdout1 = Rc::clone(&stdout1);
        let result1 = eval(
            module_name,
            "write_string",
            &vec![],
            Rc::new(RefCell::new(io::empty())),
            stdout1,
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();
        let output_data1 = &clone_stdout1.as_ref().borrow()[..];
        let expected_data1 = "hello world".as_bytes();
        assert_eq!(output_data1, expected_data1);
        assert_eq!(result1, vec![Value::I32(0), Value::I32(11)]);

        // 测试函数 `write_utf8`
        let stdout2 = Rc::new(RefCell::new(Vec::<u8>::new()));
        let clone_stdout2 = Rc::clone(&stdout2);
        let result2 = eval(
            module_name,
            "write_utf8",
            &vec![],
            Rc::new(RefCell::new(io::empty())),
            stdout2,
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();
        let output_data2 = &clone_stdout2.as_ref().borrow()[..];
        let expected_data2 = "你好，世界".as_bytes();
        assert_eq!(output_data2, expected_data2);
        assert_eq!(result2, vec![Value::I32(0), Value::I32(15)]);

        // 测试函数 `write_multiple_parts`
        let stdout3 = Rc::new(RefCell::new(Vec::<u8>::new()));
        let clone_stdout3 = Rc::clone(&stdout3);
        let result3 = eval(
            module_name,
            "write_multiple_parts",
            &vec![],
            Rc::new(RefCell::new(io::empty())),
            stdout3,
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();
        let output_data3 = &clone_stdout3.as_ref().borrow()[..];
        let expected_data3 = "part1\npart2\n".as_bytes();
        assert_eq!(output_data3, expected_data3);
        assert_eq!(result3, vec![Value::I32(0), Value::I32(12)]);
    }

    #[test]
    fn test_stdout_c() {
        // 该模块是由 C 语言程序编译而来
        let module_name = "test-stdout-c.wasm";

        // 测试函数 `write_string`
        let stdout1 = Rc::new(RefCell::new(Vec::<u8>::new()));
        let clone_stdout1 = Rc::clone(&stdout1);
        let result1 = eval(
            module_name,
            "_start",
            &vec![],
            Rc::new(RefCell::new(io::empty())),
            stdout1,
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();
        let output_data1 = &clone_stdout1.as_ref().borrow()[..];
        let output_str1 = std::str::from_utf8(output_data1).unwrap();

        // 注意，如果使用 C 的 `puts` 函数在输出时会在末尾添加 '\n'
        // 使用 `fputs` 函数则不会。
        let expected1 = "Hello world!";
        assert_eq!(output_str1, expected1);
        assert_eq!(result1, vec![]);
    }

    #[test]
    fn test_stderr_c() {
        // 该模块是由 C 语言程序编译而来
        let module_name = "test-stderr-c.wasm";

        // 测试函数 `write_string`
        let stdout1 = Rc::new(RefCell::new(Vec::<u8>::new()));
        let stderr1 = Rc::new(RefCell::new(Vec::<u8>::new()));

        let clone_stdout1 = Rc::clone(&stdout1);
        let clone_stderr1 = Rc::clone(&stderr1);

        let result1 = eval(
            module_name,
            "_start",
            &vec![],
            Rc::new(RefCell::new(io::empty())),
            stdout1,
            stderr1,
        );

        let output_data1 = &clone_stdout1.as_ref().borrow()[..];
        let output_str1 = std::str::from_utf8(output_data1).unwrap();

        let error_data1 = &clone_stderr1.as_ref().borrow()[..];
        let error_str1 = std::str::from_utf8(error_data1).unwrap();

        assert_eq!(output_str1, "number: 123, string: foo\nend of stdout");
        assert_eq!(error_str1, "number: 456, string: bar\nend of stderr");

        assert!(matches!(
            result1,
            Err(EngineError::NativeTerminate(NativeTerminate {
                module_name: _,
                native_error: NativeError::Exit(66)
            }))
        ));
    }

    #[test]
    fn test_stdin_read() {
        let module_name = "test-stdin-read.wasm";

        // 测试函数 `read_string`
        let stdin1 = Rc::new(RefCell::new("0123456789".as_bytes()));

        let result1 = eval(
            module_name,
            "read_string",
            &vec![],
            stdin1,
            Rc::new(RefCell::new(io::sink())),
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();

        assert_eq!(
            result1,
            vec![
                Value::I32(0),
                Value::I32(4),
                Value::I32('0' as i32),
                Value::I32('1' as i32),
                Value::I32('2' as i32),
                Value::I32('3' as i32),
            ]
        );

        // 测试函数 `read_multiple_parts`
        let stdin2 = Rc::new(RefCell::new("0123456789".as_bytes()));

        let result2 = eval(
            module_name,
            "read_multiple_parts",
            &vec![],
            stdin2,
            Rc::new(RefCell::new(io::sink())),
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();

        assert_eq!(result2[0..2], vec![Value::I32(0), Value::I32(3)]);
        assert_eq!(result2[2], Value::I32('0' as i32));
        assert_eq!(
            result2[4..6],
            vec![Value::I32('1' as i32), Value::I32('2' as i32),]
        );
    }

    #[test]
    fn test_stdin_c() {
        // 该模块是由 C 语言程序编译而来
        let module_name = "test-stdin-c.wasm";

        // 测试函数 `echo_by_std`
        let stdin1 = Rc::new(RefCell::new("hello\nworld".as_bytes()));
        let stdout1 = Rc::new(RefCell::new(Vec::<u8>::new()));
        let clone_stdout1 = Rc::clone(&stdout1);

        let result1 = eval(
            module_name,
            "echo_by_std",
            &vec![],
            stdin1,
            stdout1,
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();

        let output_data1 = &clone_stdout1.as_ref().borrow()[..];
        let output_str1 = std::str::from_utf8(output_data1).unwrap();

        assert_eq!(output_str1, "hello\nworld");
        assert_eq!(result1, vec![Value::I32(0)]);

        // 测试函数 `echo_by_syscall`
        let stdin2 = Rc::new(RefCell::new("foo\nbar\n".as_bytes()));
        let stdout2 = Rc::new(RefCell::new(Vec::<u8>::new()));
        let clone_stdout2 = Rc::clone(&stdout2);

        let result2 = eval(
            module_name,
            "echo_by_syscall",
            &vec![],
            stdin2,
            stdout2,
            Rc::new(RefCell::new(io::sink())),
        )
        .unwrap();

        let output_data2 = &clone_stdout2.as_ref().borrow()[..];
        let output_str2 = std::str::from_utf8(output_data2).unwrap();

        assert_eq!(output_str2, "foo\nbar\n");
        assert_eq!(result2, vec![Value::I32(0)]);
    }

    fn test_args() {
        //
    }

    fn test_envs() {
        //
    }
}
