// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;
use std::io::prelude::*;

// 使用 wasmtime 运行该程序：
// `$ wasmtime --mapdir /var::/home/yang/temp crates/native/resources/hello-rust.wasm`
//
// `--mapdir` 用于映射虚拟机中的程序所访问的路径到宿主机。
// 其中 `/var` 是虚拟机中的程序看到的路径，而 `/home/yang/temp` 是该路径对应的宿主机的路径

fn main() {
    println!("Hello world!");

    let mut file = fs::File::create("/var/test.txt").unwrap();
    write!(file, "Hello world!\n").unwrap();
}
