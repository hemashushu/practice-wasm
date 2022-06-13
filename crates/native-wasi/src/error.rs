// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    any::Any,
    fmt::{Debug, Display},
};

use anvm_ast::types::{Value, ValueType};
use anvm_engine::error::{InternalError, NativeError};

use crate::types::MODULE_NAME;

#[derive(Debug, PartialEq, Clone)]
pub struct WASIError {
    pub err_no: ErrNo,
    pub message: String,
}

impl InternalError for WASIError {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Display for WASIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.err_no, self.message)
    }
}

pub fn make_wasi_native_error(err_no: ErrNo, message: String) -> NativeError {
    let wasi_error = WASIError { err_no, message };

    NativeError {
        internal_error: Box::new(wasi_error),
        module_name: MODULE_NAME.to_owned(),
    }
}

/// INVALID_ARGUMENT_DATA_TYPE
pub fn make_invalid_argument_data_type_native_error(
    function_name: &str,
    argument_index: usize,
    expected_type: ValueType,
    actual_type: ValueType,
) -> NativeError {
    make_wasi_native_error(
        ErrNo::Invalid,
        format!(
            "invalid data type for the argument {} of function \"{}\", expected: {}, actual: {}",
            argument_index, function_name, expected_type, actual_type
        ),
    )
}

/// INVALID_ARGUMENT_SIZE
pub fn make_invalid_argument_size_native_error(
    function_name: &str,
    expected_size: usize,
    actual_size: usize,
) -> NativeError {
    make_wasi_native_error(
        ErrNo::Invalid,
        format!(
            "invalid number of arguments of function \"{}\", expected: {}, actual: {}",
            function_name, expected_size, actual_size
        ),
    )
}

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
pub enum ErrNo {
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

impl Display for ErrNo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_name = match self {
            ErrNo::TooBig => "E2BIG",
            ErrNo::AccessDenied => "EACCES",
            ErrNo::AddressInUse => "EADDRINUSE",
            ErrNo::AddressNotAvailable => "EADDRNOTAVAIL",
            ErrNo::AddressFamilyNotSupported => "EAFNOSUPPORT",
            ErrNo::Again => "EAGAIN",
            ErrNo::Already => "EALREADY",
            ErrNo::BadFileDescriptor => "EBADF",
            ErrNo::BadMessage => "EBADMSG",
            ErrNo::Busy => "EBUSY",
            ErrNo::Canceled => "ECANCELED",
            ErrNo::Child => "ECHILD",
            ErrNo::ConnectionAborted => "ECONNABORTED",
            ErrNo::ConnectionRefused => "ECONNREFUSED",
            ErrNo::ConnectionReset => "ECONNRESET",
            ErrNo::DeadLock => "EDEADLK",
            ErrNo::DestinationAddressRequired => "EDESTADDRREQ",
            ErrNo::Dom => "EDOM",
            ErrNo::Dquot => "EDQUOT",
            ErrNo::Exist => "EEXIST",
            ErrNo::Fault => "EFAULT",
            ErrNo::FileTooBig => "EFBIG",
            ErrNo::HostUnreachable => "EHOSTUNREACH",
            ErrNo::IdRemoved => "EIDRM",
            ErrNo::IllegalSequence => "EILSEQ",
            ErrNo::InProgress => "EINPROGRESS",
            ErrNo::Interrupted => "EINTR",
            ErrNo::Invalid => "EINVAL",
            ErrNo::Io => "EIO",
            ErrNo::IsConnected => "EISCONN",
            ErrNo::IsDir => "EISDIR",
            ErrNo::LoopLink => "ELOOP",
            ErrNo::Mfile => "EMFILE",
            ErrNo::Mlink => "EMLINK",
            ErrNo::MessageSize => "EMSGSIZE",
            ErrNo::Multihop => "EMULTIHOP",
            ErrNo::NameTooLong => "ENAMETOOLONG",
            ErrNo::NetworkDown => "ENETDOWN",
            ErrNo::NetworkReset => "ENETRESET",
            ErrNo::NetworkUnreachable => "ENETUNREACH",
            ErrNo::Nfile => "ENFILE",
            ErrNo::NoBufferSpace => "ENOBUFS",
            ErrNo::NoDevice => "ENODEV",
            ErrNo::NoEntry => "ENOENT",
            ErrNo::NoExecute => "ENOEXEC",
            ErrNo::NoLock => "ENOLCK",
            ErrNo::Nolink => "ENOLINK",
            ErrNo::NoMemory => "ENOMEM",
            ErrNo::NoMessage => "ENOMSG",
            ErrNo::NoProtocolOpt => "ENOPROTOOPT",
            ErrNo::NoSpace => "ENOSPC",
            ErrNo::Nosys => "ENOSYS",
            ErrNo::NotConnect => "ENOTCONN",
            ErrNo::NotDir => "ENOTDIR",
            ErrNo::NotEmpty => "ENOTEMPTY",
            ErrNo::NotRecoverable => "ENOTRECOVERABLE",
            ErrNo::NotSocket => "ENOTSOCK",
            ErrNo::NotSupported => "ENOTSUP",
            ErrNo::NotTty => "ENOTTY",
            ErrNo::Nxio => "ENXIO",
            ErrNo::Overflow => "EOVERFLOW",
            ErrNo::OwnerDead => "EOWNERDEAD",
            ErrNo::Permitted => "EPERM",
            ErrNo::Pipe => "EPIPE",
            ErrNo::Protocol => "EPROTO",
            ErrNo::ProtocolNotSupported => "EPROTONOSUPPORT",
            ErrNo::ProtocolType => "EPROTOTYPE",
            ErrNo::Range => "ERANGE",
            ErrNo::ReadOnlyFileSystem => "EROFS",
            ErrNo::Spipe => "ESPIPE",
            ErrNo::Srch => "ESRCH",
            ErrNo::Stale => "ESTALE",
            ErrNo::Timedout => "ETIMEDOUT",
            ErrNo::TextBusy => "ETXTBSY",
            ErrNo::Xdev => "EXDEV",
            ErrNo::NotCapable => "ENOTCAPABLE",
        };

        write!(f, "{}", err_name)
    }
}
