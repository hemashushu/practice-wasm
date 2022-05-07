// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    object::{EngineError, Function, Module},
    vm_module::VMModule,
};
use anvm_parser::{ast, types::Value};

pub fn create_modules(
    ast_modules: HashMap<&str, ast::Module>,
) -> Result<HashMap<String, Rc<RefCell<dyn Module>>>, EngineError> {
    let mut module_map = HashMap::<String, Rc<RefCell<dyn Module>>>::new();

    for (name, ast_module) in ast_modules {
        let vm_module = VMModule::new(name, ast_module, &module_map, None)?;

        module_map.insert(name.to_string(), vm_module);
    }

    Ok(module_map)
}

/// 从 vm 外部（即宿主）或者其他模块调用函数
pub fn eval_function(
    vm_module: Rc<RefCell<VMModule>>,
    name: &str,
    args: &[Value],
) -> Result<Vec<Value>, EngineError> {
    let rc_function = vm_module.as_ref().borrow().get_export_function(name)?;
    rc_function.as_ref().eval(args)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_linker() {
        //
    }
}