// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// 关于命令行参数
//
// 假设通过如下命令启动程序：
//
// `$ app -d demo.wasm`
//
// 对于如下的标准的 C 程序
//
// `int main(int argc, char *argv[])`
//
// argc 的值为 3
// argv 的值如下：
// 0 -> "app\0"，程序名/路径
// 1 -> "-o\0"，第一个参数
// 2 -> "demo.wasm\0"，第二个参数
// 3 -> NULL, 一个空指针
//
// 注意，对于 C 的函数 strlen，其获得的是内容的长度，该值不包括 `\0` 字符，例如：
// `strlen("abc")` 的值是 3，
// 尽管字面量 "abc" 在编译时，会转为 "abc\0" 来储存，但函数 strlen 只统计 `\0` 之前的字符。

use std::io::{self, Read, Write};

use anvm_engine::native_module::ModuleContext;

use crate::filesystem_context::FileSystemContext;

pub struct WASIModuleContext {
    pub arguments: Vec<String>,
    pub environments: Vec<(String, String)>,

    // walltime,
    // nanotime,
    // rand_source
    pub filesystem_context: FileSystemContext,
}

impl ModuleContext for WASIModuleContext {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl WASIModuleContext {
    pub fn new(
        app_path_name: &str,
        app_arguments: Vec<String>,
        environments: Vec<(String, String)>,
        // walltime,
        // nanotime,
        // rand_source,
        stdin: Box<dyn Read>,
        stdout: Box<dyn Write>,
        stderr: Box<dyn Write>,
    ) -> Self {
        // 合并 arguments 到 app_path_name
        let mut arguments: Vec<String> = vec![];
        arguments.push(app_path_name.to_owned());
        arguments.extend(app_arguments);

        let filesystem_context = FileSystemContext::new(stdin, stdout, stderr);

        Self {
            arguments,
            environments,
            filesystem_context,
        }
    }

    pub fn new_minimal() -> Self {
        let filesystem_context = FileSystemContext::new(
            Box::new(io::empty()),
            Box::new(io::sink()),
            Box::new(io::sink()),
        );

        Self {
            arguments: vec![],
            environments: vec![],
            filesystem_context,
        }
    }
}
