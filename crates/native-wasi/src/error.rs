// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    any::Any,
    fmt::{Debug, Display},
};

use anvm_engine::error::InternalError;

/// WASI 模块程序自身的错误
///
/// 注意 WASI 各个 API/ABI 的业务逻辑错误不在此范围，
/// 比如大部分 WASI API 的错误是通过项向调用者返回一个 errno 数字
/// 以表示出错的类型，所以在实现这些 API 时，底层/宿主环境引起的 Result::Err
/// 应该转换为 errno，而不是 WASIError。
#[derive(Debug, PartialEq, Clone)]
pub struct WASIError {
    pub message: String,
}

impl InternalError for WASIError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for WASIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// impl WASIError {
//     pub fn new(message: &str) -> Self {
//         Self {
//             message: message.to_owned(),
//         }
//     }
//
//     pub fn to_native_error(self) -> NativeError {
//         NativeError {
//             internal_error: Box::new(self),
//             module_name: MODULE_NAME.to_owned(),
//         }
//     }
// }

/// 函数返回的错误类型
///
/// 对应着错误代码，原始错误代码的名称过于精简，这里对名称
/// 稍微进行补充。
///
/// errno: Enum(u16)
///
/// Error codes returned by functions. Not all of these error codes are returned by the functions
/// provided by this API;
/// some are used in higher-level library layers, and others are provided merely for alignment with POSIX.
///
/// https://github.com/WebAssembly/WASI/blob/snapshot-01/phases/snapshot/docs.md#variants-1
#[derive(Debug, PartialEq, Clone)]
pub enum Errno {
    Success,                    // success: No error occurred. System call completed successfully.
    TooBig,                     // 2big: Argument list too long.
    Access,                     // acces: Permission denied.
    AddressInUse,               // addrinuse: Address in use.
    AddressNotAvailable,        // addrnotavail: Address not available.
    AddressFamilyNotSupported,  // afnosupport: Address family not supported.
    Again,                      // again: Resource unavailable, or operation would block.
    Already,                    // already: Connection already in progress.
    BadFile,                    // badf: Bad file descriptor.
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
    FileBig,                    // fbig: File too large.
    HostUnreachable,            // hostunreach: Host is unreachable.
    IdRemoved,                  // idrm: Identifier removed.
    IllegalSequence,            // ilseq: Illegal byte sequence.
    InProgress,                 // inprogress: Operation in progress.
    Interrupted,                // intr: Interrupted function.
    Invalid,                    // inval: Invalid argument.
    Io,                         // io: I/O error.
    IsConnected,                // isconn: Socket is connected.
    IsDir,                      // isdir: Is a directory.
    Loop,                       // loop: Too many levels of symbolic links.
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

impl From<Errno> for u16 {
    fn from(errno: Errno) -> Self {
        match errno {
            Errno::Success => 0,
            Errno::TooBig => 1,
            Errno::Access => 2,
            Errno::AddressInUse => 3,
            Errno::AddressNotAvailable => 4,
            Errno::AddressFamilyNotSupported => 5,
            Errno::Again => 6,
            Errno::Already => 7,
            Errno::BadFile => 8,
            Errno::BadMessage => 9,
            Errno::Busy => 10,
            Errno::Canceled => 11,
            Errno::Child => 12,
            Errno::ConnectionAborted => 13,
            Errno::ConnectionRefused => 14,
            Errno::ConnectionReset => 15,
            Errno::DeadLock => 16,
            Errno::DestinationAddressRequired => 17,
            Errno::Dom => 18,
            Errno::Dquot => 19,
            Errno::Exist => 20,
            Errno::Fault => 21,
            Errno::FileBig => 22,
            Errno::HostUnreachable => 23,
            Errno::IdRemoved => 24,
            Errno::IllegalSequence => 25,
            Errno::InProgress => 26,
            Errno::Interrupted => 27,
            Errno::Invalid => 28,
            Errno::Io => 29,
            Errno::IsConnected => 30,
            Errno::IsDir => 31,
            Errno::Loop => 32,
            Errno::Mfile => 33,
            Errno::Mlink => 34,
            Errno::MessageSize => 35,
            Errno::Multihop => 36,
            Errno::NameTooLong => 37,
            Errno::NetworkDown => 38,
            Errno::NetworkReset => 39,
            Errno::NetworkUnreachable => 40,
            Errno::Nfile => 41,
            Errno::NoBufferSpace => 42,
            Errno::NoDevice => 43,
            Errno::NoEntry => 44,
            Errno::NoExecute => 45,
            Errno::NoLock => 46,
            Errno::Nolink => 47,
            Errno::NoMemory => 48,
            Errno::NoMessage => 49,
            Errno::NoProtocolOpt => 50,
            Errno::NoSpace => 51,
            Errno::Nosys => 52,
            Errno::NotConnect => 53,
            Errno::NotDir => 54,
            Errno::NotEmpty => 55,
            Errno::NotRecoverable => 56,
            Errno::NotSocket => 57,
            Errno::NotSupported => 58,
            Errno::NotTty => 59,
            Errno::Nxio => 60,
            Errno::Overflow => 61,
            Errno::OwnerDead => 62,
            Errno::Permitted => 63,
            Errno::Pipe => 64,
            Errno::Protocol => 65,
            Errno::ProtocolNotSupported => 66,
            Errno::ProtocolType => 67,
            Errno::Range => 68,
            Errno::ReadOnlyFileSystem => 69,
            Errno::Spipe => 70,
            Errno::Srch => 71,
            Errno::Stale => 72,
            Errno::Timedout => 73,
            Errno::TextBusy => 74,
            Errno::Xdev => 75,
            Errno::NotCapable => 76,
        }
    }
}

impl Display for Errno {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_name = match self {
            Errno::Success => "SUCCESS",
            Errno::TooBig => "E2BIG",
            Errno::Access => "EACCES",
            Errno::AddressInUse => "EADDRINUSE",
            Errno::AddressNotAvailable => "EADDRNOTAVAIL",
            Errno::AddressFamilyNotSupported => "EAFNOSUPPORT",
            Errno::Again => "EAGAIN",
            Errno::Already => "EALREADY",
            Errno::BadFile => "EBADF",
            Errno::BadMessage => "EBADMSG",
            Errno::Busy => "EBUSY",
            Errno::Canceled => "ECANCELED",
            Errno::Child => "ECHILD",
            Errno::ConnectionAborted => "ECONNABORTED",
            Errno::ConnectionRefused => "ECONNREFUSED",
            Errno::ConnectionReset => "ECONNRESET",
            Errno::DeadLock => "EDEADLK",
            Errno::DestinationAddressRequired => "EDESTADDRREQ",
            Errno::Dom => "EDOM",
            Errno::Dquot => "EDQUOT",
            Errno::Exist => "EEXIST",
            Errno::Fault => "EFAULT",
            Errno::FileBig => "EFBIG",
            Errno::HostUnreachable => "EHOSTUNREACH",
            Errno::IdRemoved => "EIDRM",
            Errno::IllegalSequence => "EILSEQ",
            Errno::InProgress => "EINPROGRESS",
            Errno::Interrupted => "EINTR",
            Errno::Invalid => "EINVAL",
            Errno::Io => "EIO",
            Errno::IsConnected => "EISCONN",
            Errno::IsDir => "EISDIR",
            Errno::Loop => "ELOOP",
            Errno::Mfile => "EMFILE",
            Errno::Mlink => "EMLINK",
            Errno::MessageSize => "EMSGSIZE",
            Errno::Multihop => "EMULTIHOP",
            Errno::NameTooLong => "ENAMETOOLONG",
            Errno::NetworkDown => "ENETDOWN",
            Errno::NetworkReset => "ENETRESET",
            Errno::NetworkUnreachable => "ENETUNREACH",
            Errno::Nfile => "ENFILE",
            Errno::NoBufferSpace => "ENOBUFS",
            Errno::NoDevice => "ENODEV",
            Errno::NoEntry => "ENOENT",
            Errno::NoExecute => "ENOEXEC",
            Errno::NoLock => "ENOLCK",
            Errno::Nolink => "ENOLINK",
            Errno::NoMemory => "ENOMEM",
            Errno::NoMessage => "ENOMSG",
            Errno::NoProtocolOpt => "ENOPROTOOPT",
            Errno::NoSpace => "ENOSPC",
            Errno::Nosys => "ENOSYS",
            Errno::NotConnect => "ENOTCONN",
            Errno::NotDir => "ENOTDIR",
            Errno::NotEmpty => "ENOTEMPTY",
            Errno::NotRecoverable => "ENOTRECOVERABLE",
            Errno::NotSocket => "ENOTSOCK",
            Errno::NotSupported => "ENOTSUP",
            Errno::NotTty => "ENOTTY",
            Errno::Nxio => "ENXIO",
            Errno::Overflow => "EOVERFLOW",
            Errno::OwnerDead => "EOWNERDEAD",
            Errno::Permitted => "EPERM",
            Errno::Pipe => "EPIPE",
            Errno::Protocol => "EPROTO",
            Errno::ProtocolNotSupported => "EPROTONOSUPPORT",
            Errno::ProtocolType => "EPROTOTYPE",
            Errno::Range => "ERANGE",
            Errno::ReadOnlyFileSystem => "EROFS",
            Errno::Spipe => "ESPIPE",
            Errno::Srch => "ESRCH",
            Errno::Stale => "ESTALE",
            Errno::Timedout => "ETIMEDOUT",
            Errno::TextBusy => "ETXTBSY",
            Errno::Xdev => "EXDEV",
            Errno::NotCapable => "ENOTCAPABLE",
        };

        write!(f, "{}", err_name)
    }
}
