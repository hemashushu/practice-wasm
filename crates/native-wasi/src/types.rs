// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 数据类型
//!
//! WASI 的 API 规范在设计上跟 POSIX 较为接近，包括结构体的名称、标记（flag）及
//! 常量的名称等，但标记和常量的值跟 POSIX 不同，WASI 已重新编排了这些数据的值，
//! 比如 Errno 的值根据 API 规范文档所列出的顺序，从 0 开始编排。
//!
//! https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#types
//! https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#supertypes
//!
//! 简单来说，这些数据的值跟我们熟悉的 "/usr/include/error.h" 和 "/usr/include/fcntl.h" 并没有关系，
//! 仅部分名称相同而已。
//! 甚至有些常量是 WASI 独有（新增）的，比如 Errno::NotCapable，这个常量是 "error.h" 不具有的。
//!
//! C/C++/Rust 等编译器通过使用 WASI 的 [wasi-libc](https://github.com/WebAssembly/wasi-libc)
//! 实现这些数值的重定义，所以需注意，如果有（c/c++）程序使用了硬编码的数值，
//! 则编译后的程序大概率无法正确运行。

use std::io::Write;

/// 当前版本的 WASI 模块的名称
///
/// 正式版的名称将会是 `wasi`
pub const MODULE_NAME: &str = "wasi_snapshot_preview1";

pub trait Serialize {
    fn get_serialize_size(&self) -> usize;

    fn serialize(&self) -> Vec<u8> {
        let size = self.get_serialize_size();
        let mut buffer = Vec::<u8>::with_capacity(size);
        self.write(&mut buffer);
        buffer
    }

    fn write(&self, writer: &mut dyn Write);
}

pub trait Deserialize {
    fn get_deserialize_size() -> usize;
    fn deserialize(data: &[u8]) -> Self;
}

/// clockid: Enum(u32)
/// Identifiers for clocks.
pub enum ClockID {
    Realtime, // The clock measuring real time. Time value zero corresponds with 1970-01-01T00:00:00Z.
    Monotonic, // The store-wide monotonic clock, which is defined as a clock measuring real time, whose value cannot be adjusted and which cannot have negative clock jumps. The epoch of this clock is undefined. The absolute time value of this clock therefore has no meaning.
    ProcessCputimeId, // The CPU-time clock associated with the current process.
    ThreadCputimeId, // The CPU-time clock associated with the current thread.
}

impl From<ClockID> for u32 {
    fn from(clock_id: ClockID) -> Self {
        match clock_id {
            ClockID::Realtime => 0,
            ClockID::Monotonic => 1,
            ClockID::ProcessCputimeId => 2,
            ClockID::ThreadCputimeId => 3,
        }
    }
}

/// rights: Flags(u64)
/// File descriptor rights, determining which actions may be performed.
/// Size: 8
/// Alignment: 8
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#rights
pub mod rights {
    pub const FD_DATASYNC: u64 = 1 << 0; // The right to invoke fd_datasync. If path_open is set, includes the right to invoke path_open with fdflags::dsync.
    pub const FD_READ: u64 = 1 << 1; // The right to invoke fd_read and sock_recv. If rights::fd_seek is set, includes the right to invoke fd_pread.
    pub const FD_SEEK: u64 = 1 << 2; // The right to invoke fd_seek. This flag implies rights::fd_tell.
    pub const FD_FDSTAT_SET_FLAGS: u64 = 1 << 3; // The right to invoke fd_fdstat_set_flags.
    pub const FD_SYNC: u64 = 1 << 4; // The right to invoke fd_sync. If path_open is set, includes the right to invoke path_open with fdflags::rsync and fdflags::dsync.
    pub const FD_TELL: u64 = 1 << 5; // The right to invoke fd_seek in such a way that the file offset remains unaltered (i.e., whence::cur with offset zero), or to invoke fd_tell.
    pub const FD_WRITE: u64 = 1 << 6; // The right to invoke fd_write and sock_send. If rights::fd_seek is set, includes the right to invoke fd_pwrite.
    pub const FD_ADVISE: u64 = 1 << 7; // The right to invoke fd_advise.
    pub const FD_ALLOCATE: u64 = 1 << 8; // The right to invoke fd_allocate.
    pub const PATH_CREATE_DIRECTORY: u64 = 1 << 9; // The right to invoke path_create_directory.
    pub const PATH_CREATE_FILE: u64 = 1 << 10; // If path_open is set, the right to invoke path_open with oflags::creat.
    pub const PATH_LINK_SOURCE: u64 = 1 << 11; // The right to invoke path_link with the file descriptor as the source directory.
    pub const PATH_LINK_TARGET: u64 = 1 << 12; // The right to invoke path_link with the file descriptor as the target directory.
    pub const PATH_OPEN: u64 = 1 << 13; // The right to invoke path_open.
    pub const FD_READDIR: u64 = 1 << 14; // The right to invoke fd_readdir.
    pub const PATH_READLINK: u64 = 1 << 15; // The right to invoke path_readlink.
    pub const PATH_RENAME_SOURCE: u64 = 1 << 16; // The right to invoke path_rename with the file descriptor as the source directory.
    pub const PATH_RENAME_TARGET: u64 = 1 << 17; // The right to invoke path_rename with the file descriptor as the target directory.
    pub const PATH_FILESTAT_GET: u64 = 1 << 18; // The right to invoke path_filestat_get.
    pub const PATH_FILESTAT_SET_SIZE: u64 = 1 << 19; // The right to change a file's size (there is no path_filestat_set_size). If path_open is set, includes the right to invoke path_open with oflags::trunc.
    pub const PATH_FILESTAT_SET_TIMES: u64 = 1 << 20; // The right to invoke path_filestat_set_times.
    pub const FD_FILESTAT_GET: u64 = 1 << 21; // The right to invoke fd_filestat_get.
    pub const FD_FILESTAT_SET_SIZE: u64 = 1 << 22; // The right to invoke fd_filestat_set_size.
    pub const FD_FILESTAT_SET_TIMES: u64 = 1 << 23; // The right to invoke fd_filestat_set_times.
    pub const PATH_SYMLINK: u64 = 1 << 24; // The right to invoke path_symlink.
    pub const PATH_REMOVE_DIRECTORY: u64 = 1 << 25; // The right to invoke path_remove_directory.
    pub const PATH_UNLINK_FILE: u64 = 1 << 26; // The right to invoke path_unlink_file.
    pub const POLL_FD_READWRITE: u64 = 1 << 27; // If rights::fd_read is set, includes the right to invoke poll_oneoff to subscribe to eventtype::fd_read. If rights::fd_write is set, includes the right to invoke poll_oneoff to subscribe to eventtype::fd_write.
    pub const SOCK_SHUTDOWN: u64 = 1 << 28; // The right to invoke sock_shutdown.
    pub const SOCK_ACCEPT: u64 = 1 << 29; // The right to invoke sock_accept.
}

#[derive(Debug, PartialEq, Clone)]
/// filetype: Enum(u8)
/// The type of a file descriptor or file.
/// Size: 1
/// Alignment: 1
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#filetype
pub enum Filetype {
    Unknown, // The type of the file descriptor or file is unknown or is different from any of the other types specified.
    BlockDevice, // The file descriptor or file refers to a block device inode.
    CharacterDevice, // The file descriptor or file refers to a character device inode.
    Directory, // The file descriptor or file refers to a directory inode.
    RegularFile, // The file descriptor or file refers to a regular file inode.
    SocketDgram, // The file descriptor or file refers to a datagram socket.
    SocketStream, // The file descriptor or file refers to a byte-stream socket.
    SymbolicLink, // The file refers to a symbolic link inode.
}

impl From<Filetype> for u8 {
    fn from(file_type: Filetype) -> Self {
        match file_type {
            Filetype::Unknown => 0,
            Filetype::BlockDevice => 1,
            Filetype::CharacterDevice => 2,
            Filetype::Directory => 3,
            Filetype::RegularFile => 4,
            Filetype::SocketDgram => 5,
            Filetype::SocketStream => 6,
            Filetype::SymbolicLink => 7,
        }
    }
}

impl Serialize for Filetype {
    fn get_serialize_size(&self) -> usize {
        1
    }

    fn write(&self, writer: &mut dyn Write) {
        let value = u8::from(self.to_owned());
        let data = u8::to_le_bytes(value);
        writer.write(&data).unwrap();
    }
}

/// fdstat: Struct
/// File descriptor attributes.
/// Size: 24
/// Alignment: 8
/// Struct members
/// - fs_filetype: filetype File type.
///   Offset: 0
/// - fs_flags: fdflags File descriptor flags.
///   Offset: 2
/// - fs_rights_base: rights Rights that apply to this file descriptor.
///   Offset: 8
/// - fs_rights_inheriting: rights Maximum set of rights that may be installed on new file descriptors that are created through this file descriptor, e.g., through path_open.
///   Offset: 16
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#fdstat
pub struct FdStat {
    pub fs_filetype: Filetype,
    pub fs_flags: u16,
    pub fs_rights_base: u64,
    pub fs_rights_inheriting: u64,
}

impl Serialize for FdStat {
    fn get_serialize_size(&self) -> usize {
        24
    }

    fn write(&self, writer: &mut dyn Write) {
        self.fs_filetype.write(writer); // 1 byte
        writer.write(&[0, 1]).unwrap(); // 1 byte (padding)
        writer.write(&u16::to_le_bytes(self.fs_flags)).unwrap(); // 2 bytes
        writer.write(&[0, 4]).unwrap(); // 4 bytes (padding)
        writer
            .write(&u64::to_le_bytes(self.fs_rights_base))
            .unwrap(); // 8 bytes
        writer
            .write(&u64::to_le_bytes(self.fs_rights_inheriting))
            .unwrap(); // 8 bytes
    }
}

/// ### whence: Enum(u8)
/// The position relative to which to set the offset of the file descriptor.
/// Size: 1
/// Alignment: 1
/// Variants
/// - set: Seek relative to start-of-file.
/// - cur: Seek relative to current position.
/// - end: Seek relative to end-of-file.
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-whence-enumu8
pub enum Whence {
    Set,
    Current,
    End,
}

impl From<Whence> for u8 {
    fn from(whence: Whence) -> Self {
        match whence {
            Whence::Set => 0,
            Whence::Current => 1,
            Whence::End => 2,
        }
    }
}

impl TryFrom<u8> for Whence {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Whence::Set),
            1 => Ok(Whence::Current),
            2 => Ok(Whence::End),
            _ => Err(()),
        }
    }
}

/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#-ciovec-struct
///
/// ciovec: Struct
/// A region of memory for scatter/gather writes.
///
/// Size: 8
/// Alignment: 4
/// Struct members
/// - buf: ConstPointer<u8> The address of the buffer to be written.
///   Offset: 0
/// - buf_len: size The length of the buffer to be written.
///   Offset: 4
pub struct CIOVec {
    /// 数据在内存中的开始位置（地址）
    pub buf_offset: u32,

    /// 有效数据的长度
    pub buf_len: u32,
}

impl Deserialize for CIOVec {
    fn get_deserialize_size() -> usize {
        8
    }

    fn deserialize(data: &[u8]) -> Self {
        let buf_offset = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let buf_len = u32::from_le_bytes(data[4..8].try_into().unwrap());
        Self {
            buf_offset,
            buf_len,
        }
    }
}
