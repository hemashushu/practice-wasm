// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 数据类型
//!
//! ## 基本类型
//!
//! https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#types
//!
//! - size: u32
//! - filesize: u64
//!   Non-negative file size or length of a region within a file.
//! - timestamp: u64
//!   Timestamp in nanoseconds.
//! - clockid: Enum/u32
//!   Identifiers for clocks.
//! - errno: Enum(u16)
//!   Error codes returned by functions. Not all of these error codes
//!   are returned by the functions provided by this API;
//!   some are used in higher-level library layers, and others are provided
//!   merely for alignment with POSIX.
//! - rights: Flags(u64)
//!   File descriptor rights, determining which actions may be performed.
//! - fd: u32
//!   A file descriptor handle.
//!
//! ## 复合类型
//!
//! 详细见：
//! https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#supertypes

pub const MODULE_NAME: &str = "wasi_snapshot_preview1";

/// clockid: Enum(u32)
/// Identifiers for clocks.
pub enum ClockID {
    Realtime, // The clock measuring real time. Time value zero corresponds with 1970-01-01T00:00:00Z.
    Monotonic, // The store-wide monotonic clock, which is defined as a clock measuring real time, whose value cannot be adjusted and which cannot have negative clock jumps. The epoch of this clock is undefined. The absolute time value of this clock therefore has no meaning.
    ProcessCputimeId, // The CPU-time clock associated with the current process.
    ThreadCputimeId, // The CPU-time clock associated with the current thread.
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
}

/// filetype: Enum(u8)
/// The type of a file descriptor or file.
/// Size: 1
/// Alignment: 1
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#filetype
pub enum FileType {
    Unknown, // The type of the file descriptor or file is unknown or is different from any of the other types specified.
    BlockDevice, // The file descriptor or file refers to a block device inode.
    CharacterDevice, // The file descriptor or file refers to a character device inode.
    Directory, // The file descriptor or file refers to a directory inode.
    RegularFile, // The file descriptor or file refers to a regular file inode.
    SocketDgram, // The file descriptor or file refers to a datagram socket.
    SocketStream, // The file descriptor or file refers to a byte-stream socket.
    SymbolicLink, // The file refers to a symbolic link inode.
}

/// fdflags: Flags(u16)
/// File descriptor flags.
/// Size: 2
/// Alignment: 2
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#fdflags
pub mod fd_flags {
    pub const APPEND: u16 = 1 << 0; // Append mode: Data written to the file is always appended to the file's end.
    pub const DSYNC: u16 = 1 << 1; // Write according to synchronized I/O data integrity completion. Only the data stored in the file is synchronized.
    pub const NONBLOCK: u16 = 1 << 2; // Non-blocking mode.
    pub const RSYNC: u16 = 1 << 3; // Synchronized read I/O operations.
    pub const SYNC: u16 = 1 << 4; // Write according to synchronized I/O file integrity completion. In addition to synchronizing the data stored in the file, the implementation may also synchronously update the file's metadata.
}

/// fdstat: Struct
/// File descriptor attributes.
/// Size: 24
/// Alignment: 8
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#fdstat
pub struct FdStat {
    pub fs_filetype: FileType,
    pub fd_flags: u16,
    pub fs_rights_base: u64,
    pub fs_rights_inheriting: u64,
}

// ### iovec: Struct
//
// A region of memory for scatter/gather reads.
// Size: 8
// Alignment: 4
// Struct members
// - buf: Pointer<u8> The address of the buffer to be filled.
//   Offset: 0
// - buf_len: size The length of the buffer to be filled.
//   Offset: 4
//
// ### ciovec: Struct
//
// A region of memory for scatter/gather writes.
// Size: 8
// Alignment: 4
// Struct members
// - buf: ConstPointer<u8> The address of the buffer to be written.
//   Offset: 0
// - buf_len: size The length of the buffer to be written.
//   Offset: 4
//
// ### iovec_array: Array<iovec>
// Size: 8
// Alignment: 4
//
// ### ciovec_array: Array<ciovec>
// Size: 8
// Alignment: 4
//
// ### filedelta: s64
//
// Relative offset within a file.
// Size: 8
// Alignment: 8
//
// ### whence: Enum(u8)
//
// The position relative to which to set the offset of the file descriptor.
// Size: 1
// Alignment: 1
// Variants
// - set: Seek relative to start-of-file.
// - cur: Seek relative to current position.
// - end: Seek relative to end-of-file.
