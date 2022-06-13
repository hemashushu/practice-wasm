// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// hello world 所需要的 API
//
// - fd_fdstat_get
// - fd_seek
// - fd_write
// - fd_close

// 列 args 和 envs 所需要的 API
//
// - args_get
// - args_sizes_get
// - environ_get
// - environ_sizes_get
// - proc_exit
// - fd_fdstat_get (*)
// - fd_seek (*)
// - fd_write (*)
// - fd_close (*)