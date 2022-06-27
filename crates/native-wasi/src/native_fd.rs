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

use std::io::{Read, Seek, SeekFrom, Write};

use anvm_engine::vm_memory::VMMemory;

use crate::{
    error::Errno,
    types::{CIOVec, FdStat, Filetype, Whence},
    wasi_module_context::WASIModuleContext,
};

use crate::filesystem_context::FileSource;

/// # fd_write(fd: fd, iovs: ciovec_array) -> (errno, size)
///
/// Write to a file descriptor. Note: This is similar to writev in POSIX.
///
/// Params
/// - fd: fd
/// - iovs: ciovec_array List of scatter/gather vectors from which to retrieve data.
///
/// Results
/// - error: errno
///   - Badf: if `fd` is invalid
///   - Fault: if `iovs` or `resultSize` contain an invalid offset due to the memory constraint
///   - Io: if an IO related error happens during the operation
/// - nwritten: size The number of bytes written.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-fd_writefd-fd-iovs-ciovec_array---errno-size
///
/// 把内存中的数据写入到指定的文件
///
/// - 其中数据来自内存，由 `CIOVec` 指定数据在内存当中的位置/地址，以及数据的长度。
/// - 支持一次写入多个来自不同位置及长度的数据，位置及长度由参数 `ciovecs` 指定，
///   多段数据写到文件之后都被合并形成一段连续的数据。
/// - 如果需要写到文件的不同位置，需要使用 fd_seek 来改变写入位置。
pub fn fd_write(
    memory_block: &mut VMMemory,
    module_context: &mut WASIModuleContext,
    fd: u32,
    ciovecs: &[CIOVec],
) -> Result<u32, Errno> {
    let option_file_entry = module_context.filesystem_context.get_file_mut(fd);
    if let Some(file_entry) = option_file_entry {
        match &mut file_entry.file_source {
            FileSource::File(file) => {
                let mut wrote_bytes: usize = 0;
                for ciovec in ciovecs {
                    let data = memory_block
                        .read_bytes(ciovec.buf_offset as usize, ciovec.buf_len as usize);
                    match file.write(data) {
                        Ok(n) => {
                            wrote_bytes += n;
                        }
                        Err(_) => {
                            return Err(Errno::Io);
                        }
                    }
                }

                Ok(wrote_bytes as u32)
            }
            FileSource::Write(w) => {
                let mut writer = w.as_ref().borrow_mut();
                let mut wrote_bytes: usize = 0;
                for ciovec in ciovecs {
                    let data = memory_block
                        .read_bytes(ciovec.buf_offset as usize, ciovec.buf_len as usize);
                    match writer.write(data) {
                        Ok(n) => {
                            wrote_bytes += n;
                        }
                        Err(_) => {
                            return Err(Errno::Io);
                        }
                    }
                }

                Ok(wrote_bytes as u32)
            }
            FileSource::Read(_) => {
                return Err(Errno::BadFile); // Read 不支持 write
            }
        }
    } else {
        Err(Errno::BadFile)
    }
}

/// # fd_fdstat_get(fd: fd) -> (errno, fdstat)
///
/// Get the attributes of a file descriptor.
/// Note: This returns similar flags to fsync(fd, F_GETFL) in POSIX, as well as additional fields.
///
/// Params
/// - fd: the file descriptor to get the fdstat attributes data
///
/// Results
/// - error: errno
///   - Badf: if `fd` is invalid
///   - Fault: if `resultFdstat` contains an invalid offset due to the memory constraint
/// - stat: fdstat The buffer where the file descriptor's attributes are stored.
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

/// # fd_seek(fd: fd, offset: filedelta, whence: whence) -> (errno, filesize)
///
/// Move the offset of a file descriptor. Note: This is similar to lseek in POSIX.
///
/// Params
/// - fd: fd
/// - offset: filedelta The number of bytes to move.
/// - whence: whence The base from which the offset is relative.
///
/// Results
/// - error: errno
///   - Badf: if `fd` is invalid
///   - Fault: if `resultNewoffset` is an invalid offset of the memory
///   - Inval: if `whence` is an invalid value
///   - Io: if other error happens during the operation of the underying file system
/// - newoffset: filesize The new offset of the file descriptor, relative to the start of the file.
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

/// # fd_close(fd: fd) -> errno
///
/// Close a file descriptor. Note: This is similar to close in POSIX.
///
/// Params
/// - fd: fd
///
/// Results
/// - error: errno
///   - Badf: if `fd` is invalid
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-fd_closefd-fd---errno
pub fn fd_close(module_context: &mut WASIModuleContext, fd: u32) -> Result<(), Errno> {
    let option_file_entry = module_context.filesystem_context.get_file_mut(fd);
    if let Some(file_entry) = option_file_entry {
        match &mut file_entry.file_source {
            FileSource::File(_) => {
                // 从 opened_files 当中移除目标文件
                // 文件在引用移除之后应该自动关闭
                module_context.filesystem_context.remove_opened_file(fd);

                Ok(())
            }
            FileSource::Read(_) => {
                println!("try to close a read stream: {}", fd);
                Err(Errno::BadFile) // Read 不支持 close
            }
            FileSource::Write(_) => {
                println!("try to close a write stream: {}", fd);
                Err(Errno::BadFile) // Write 不支持 close
            }
        }
    } else {
        Err(Errno::BadFile)
    }
}

/// # fd_read(fd: fd, iovs: iovec_array) -> (errno, size)
///
/// Read from a file descriptor. Note: This is similar to readv in POSIX.
///
/// Params
/// - fd: fd
/// - iovs: iovec_array List of scatter/gather vectors to which to store data.
///
/// Results
/// - error: errno
///   - Badf: if `fd` is invalid
///   - Fault: if `iovs` or `resultSize` contain an invalid offset due to the memory constraint
///   - Io: if an IO related error happens during the operation
/// - nread: size The number of bytes read.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-fd_readfd-fd-iovs-iovec_array---errno-size
///
/// 读取指定文件的数据到内存
///
/// - 数据来自文件，由 `CIOVec` 指定数据写入到内存当中的位置/地址，以及数据的长度。
/// - 支持连续读取几次数据然后写入到内存的不同位置，位置及长度由参数 `ciovecs` 指定，
///   注意多次读取文件的数据，它们在文件里原本是连续的。
/// - 如果需要读取文件的不同位置，需要使用 fd_seek 来改变读取位置。
pub fn fd_read(
    memory_block: &mut VMMemory,
    module_context: &mut WASIModuleContext,
    fd: u32,
    ciovecs: &[CIOVec],
) -> Result<u32, Errno> {
    let option_file_entry = module_context.filesystem_context.get_file_mut(fd);
    if let Some(file_entry) = option_file_entry {
        match &mut file_entry.file_source {
            FileSource::File(file) => {
                let mut read_bytes: usize = 0;
                for ciovec in ciovecs {
                    let buffer = memory_block
                        .get_bytes_mut(ciovec.buf_offset as usize, ciovec.buf_len as usize);
                    match file.read(buffer) {
                        Ok(n) => {
                            read_bytes += n;
                        }
                        Err(_) => {
                            return Err(Errno::Io);
                        }
                    }
                }

                Ok(read_bytes as u32)
            }
            FileSource::Write(_) => {
                return Err(Errno::BadFile); // Write 不支持 read
            }
            FileSource::Read(r) => {
                let mut reader = r.as_ref().borrow_mut();
                let mut read_bytes: usize = 0;
                for ciovec in ciovecs {
                    let buffer = memory_block
                        .get_bytes_mut(ciovec.buf_offset as usize, ciovec.buf_len as usize);
                    match reader.read(buffer) {
                        Ok(n) => {
                            read_bytes += n;
                        }
                        Err(_) => {
                            return Err(Errno::Io);
                        }
                    }
                }

                Ok(read_bytes as u32)
            }
        }
    } else {
        Err(Errno::BadFile)
    }
}
