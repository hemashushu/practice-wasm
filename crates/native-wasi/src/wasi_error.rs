// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    any::Any,
    fmt::{Debug, Display},
};

use anvm_engine::error::{InternalError, NativeError};

pub const MODULE_NAME: &str = "wasi_snapshot_preview1";

#[derive(Debug)]
pub struct WASIError {
    pub code: i32,
}

impl InternalError for WASIError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for WASIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}", self.code)
    }
}

pub fn make_wasi_native_error(code: i32) -> NativeError {
    let wasi_error = WASIError { code };

    NativeError {
        internal_error: Box::new(wasi_error),
        module_name: MODULE_NAME.to_owned(),
    }
}

/// 函数返回的错误类型
///
/// 对应着错误代码，原始错误代码的名称过于精简，这里对名称
/// 稍微进行补充。
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#variants-1
/// errno: Enum(u16)
pub enum ErrorNumber {
    TooBig,                     // 2big: Argument list too long.
    AccessDenied,               // acces: Permission denied.
    AddressInUse,               // addrinuse: Address in use.
    AddressNotAvailable,        // addrnotavail: Address not available.
    AddressFamilyNotSupported,  // afnosupport: Address family not supported.
    Again,                      // again: Resource unavailable, or operation would block.
    Already,                    // already: Connection already in progress.
    BadFileDescriptor,          // badf: Bad file descriptor.
    BadMessage,                 // badmsg: Bad message.
    Busy,                       // busy: Device or resource busy.
    Canceled,                   // canceled: Operation canceled.
    Child,                      // child: No child processes.
    ConnectionAborted,          // connaborted: Connection aborted.
    ConnectionRefused,          // connrefused: Connection refused.
    ConnectionReset,            // connreset: Connection reset.
    DeadLock,                   // deadlk: Resource deadlock would occur.
    DestinationAddressRequired, // destaddrreq: Destination address required.
    Dom,                        // dom: Mathematics argument out of domain of function.
    Dquot,                      // dquot: Reserved.
    Exist,                      // exist: File exists.
    Fault,                      // fault: Bad address.
    FileTooBig,                 // fbig: File too large.
    HostUnreachable,            // hostunreach: Host is unreachable.
    IdRemoved,                  // idrm: Identifier removed.
    IllegalSequence,            // ilseq: Illegal byte sequence.
    InProgress,                 // inprogress: Operation in progress.
    Interrupted,                // intr: Interrupted function.
    Invalid,                    // inval: Invalid argument.
    Io,                         // io: I/O error.
    IsConnected,                // isconn: Socket is connected.
    IsDir,                      // isdir: Is a directory.
    LoopLink,                   // loop: Too many levels of symbolic links.
    Mfile,                      // mfile: File descriptor value too large.
    Mlink,                      // mlink: Too many links.
    MessageSize,                // msgsize: Message too large.
    Multihop,                   // multihop: Reserved.
    NameTooLong,                // nametoolong: Filename too long.
    NetworkDown,                // netdown: Network is down.
    NetworkReset,               // netreset: Connection aborted by network.
    NetworkUnreachable,         // netunreach: Network unreachable.
    Nfile,                      // nfile: Too many files open in system.
    NoBufferSpace,              // nobufs: No buffer space available.
    NoDevice,                   // nodev: No such device.
    NoEntry,                    // noent: No such file or directory.
    NoExecute,                  // noexec: Executable file format error.
    NoLock,                     // nolck: No locks available.
    Nolink,                     // nolink: Reserved.
    NoMemory,                   // nomem: Not enough space.
    NoMessage,                  // nomsg: No message of the desired type.
    NoProtocolOpt,              // noprotoopt: Protocol not available.
    NoSpace,                    // nospc: No space left on device.
    Nosys,                      // nosys: Function not supported.
    NotConnect,                 // notconn: The socket is not connected.
    NotDir,                     // notdir: Not a directory or a symbolic link to a directory.
    NotEmpty,                   // notempty: Directory not empty.
    NotRecoverable,             // notrecoverable: State not recoverable.
    NotSocket,                  // notsock: Not a socket.
    NotSupported,               // notsup: Not supported, or operation not supported on socket.
    NotTty,                     // notty: Inappropriate I/O control operation.
    Nxio,                       // nxio: No such device or address.
    Overflow,                   // overflow: Value too large to be stored in data type.
    OwnerDead,                  // ownerdead: Previous owner died.
    Permitted,                  // perm: Operation not permitted.
    Pipe,                       // pipe: Broken pipe.
    Protocol,                   // proto: Protocol error.
    ProtocolNotSupported,       // protonosupport: Protocol not supported.
    ProtocolType,               // prototype: Protocol wrong type for socket.
    Range,                      // range: Result too large.
    ReadOnlyFileSystem,         // rofs: Read-only file system.
    Spipe,                      // spipe: Invalid seek.
    Srch,                       // srch: No such process.
    Stale,                      // stale: Reserved.
    Timedout,                   // timedout: Connection timed out.
    TextBusy,                   // txtbsy: Text file busy.
    Xdev,                       // xdev: Cross-device link.
    NotCapable,                 // notcapable: Extension: Capabilities insufficient.
}

