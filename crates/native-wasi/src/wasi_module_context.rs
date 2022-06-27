// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    cell::RefCell,
    io::{self, Read, Write},
    rc::Rc,
};

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
        stdin: Rc<RefCell<dyn Read>>,
        stdout: Rc<RefCell<dyn Write>>,
        stderr: Rc<RefCell<dyn Write>>,
    ) -> Self {
        // 合并 app_path_name 和 app_arguments 到 arguments
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
            Rc::new(RefCell::new(io::empty())),
            Rc::new(RefCell::new(io::sink())),
            Rc::new(RefCell::new(io::sink())),
        );

        Self {
            arguments: vec![],
            environments: vec![],
            filesystem_context,
        }
    }
}
