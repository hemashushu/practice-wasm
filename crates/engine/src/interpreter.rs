// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    instance::{EngineError, Module},
    vm_module::VMModule,
};
use anvm_parser::ast;

pub fn new_modules(
    ast_modules: HashMap<&str, ast::Module>,
) -> Result<HashMap<String, Rc<RefCell<dyn Module>>>, EngineError> {
    let mut module_map = HashMap::<String, Rc<RefCell<dyn Module>>>::new();

    for (name, ast_module) in ast_modules {
        let vm_module = VMModule::new(name, ast_module, &module_map, None)?;

        module_map.insert(name.to_string(), vm_module);
    }

    Ok(module_map)
}
