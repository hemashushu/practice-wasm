// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{error::WASIError, types::FdStat};

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
pub fn fd_fdstat_get(fd: u32) -> Result<FdStat, WASIError> {
    //
    todo!()
}
