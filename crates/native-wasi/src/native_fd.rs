// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # Rust std 当中跟文件系统操作相关的结构体和函数
//!
//! - std::fs
//!   https://doc.rust-lang.org/std/fs/index.html
//! - std::io
//!   https://doc.rust-lang.org/std/io/index.html
//! - std::path
//!   https://doc.rust-lang.org/std/path/index.html
//!
//! - std::fs::File
//!   https://doc.rust-lang.org/std/fs/struct.File.html
//!
//! https://doc.rust-lang.org/std/io/trait.Read.html
//! https://doc.rust-lang.org/std/io/trait.Write.html
//! https://doc.rust-lang.org/std/io/fn.stdin.html
//! https://doc.rust-lang.org/std/io/struct.Empty.html
//! https://doc.rust-lang.org/std/io/struct.Sink.html

use std::io::{Seek, SeekFrom};

use crate::{
    error::Errno,
    types::{FdStat, Filetype, Whence},
    wasi_module_context::WASIModuleContext,
};

use crate::filesystem_context::FileSource;

/// fd_fdstat_get(fd: fd) -> (errno, fdstat)
///
/// Get the attributes of a file descriptor.
/// Note: This returns similar flags to fsync(fd, F_GETFL) in POSIX, as well as additional fields.
///
/// Params
///      fd: the file descriptor to get the fdstat attributes data
/// Results
///      error: errno
///      stat: fdstat The buffer where the file descriptor's attributes are stored.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-fd_fdstat_getfd-fd---errno-fdstat
pub fn fd_fdstat_get(module_context: &mut WASIModuleContext, fd: u32) -> Result<FdStat, Errno> {
    let option_file_entry = module_context.filesystem_context.get_file(fd);
    if let Some(_) = option_file_entry {
        // TODO::
        // 这里先不对 fd 作具体分析，仅判断是否存在
        let fd_stat = FdStat {
            fs_filetype: Filetype::RegularFile,
            fs_flags: 0,
            fs_rights_base: 0,
            fs_rights_inheriting: 0,
        };

        // 返回 FdStat 结构体实例
        Ok(fd_stat)
    } else {
        Err(Errno::BadFile)
    }
}

/// fd_seek(fd: fd, offset: filedelta, whence: whence) -> (errno, filesize)
/// Move the offset of a file descriptor. Note: This is similar to lseek in POSIX.
///
/// Params
/// fd: fd
/// offset: filedelta The number of bytes to move.
/// whence: whence The base from which the offset is relative.
///
/// Results
/// error: errno
///
/// newoffset: filesize The new offset of the file descriptor, relative to the start of the file.
///
/// https://github.com/WebAssembly/WASI/blob/main/phases/snapshot/docs.md#-fd_seekfd-fd-offset-filedelta-whence-whence---resultfilesize-errno
pub fn fd_seek(
    module_context: &mut WASIModuleContext,
    fd: u32,
    offset: i64,
    whence: Whence,
) -> Result<u64, Errno> {
    let option_file_entry = module_context.filesystem_context.get_file_mut(fd);
    if let Some(file_entry) = option_file_entry {
        match &mut file_entry.file_source {
            FileSource::File(file) => {
                let host_result = file.seek(match whence {
                    Whence::Set => SeekFrom::Start(offset as u64), // new offset == `offset`.
                    Whence::Current => SeekFrom::Current(offset), // new offset == current offset + `offset`.
                    Whence::End => SeekFrom::End(offset), // new offset == file size + `offset`，注意这时 offset 应该是负数
                });

                if let Ok(new_offset) = host_result {
                    Ok(new_offset)
                } else {
                    Err(Errno::Io)
                }
            }
            FileSource::Read(_) => Err(Errno::BadFile), // Read 不支持 seek
            FileSource::Write(_) => Err(Errno::BadFile), // Write 不支持 seek
        }
    } else {
        Err(Errno::BadFile)
    }
}
