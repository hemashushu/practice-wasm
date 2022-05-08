// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    object::{EngineError, Module},
    vm_module::VMModule,
};
use anvm_ast::ast;

pub struct Instance {
    pub module_map: HashMap<String, Rc<RefCell<dyn Module>>>,
}

impl Instance {
    pub fn new() -> Self {
        let module_map = HashMap::<String, Rc<RefCell<dyn Module>>>::new();
        Self { module_map }
    }

    pub fn add_ast_module(
        &mut self,
        name: &str,
        ast_module: ast::Module,
    ) -> Result<(), EngineError> {
        let vm_module = VMModule::new(name, ast_module, &self.module_map, None)?;
        self.add_module(vm_module)
    }

    pub fn add_module(&mut self, module: Rc<RefCell<dyn Module>>) -> Result<(), EngineError> {
        let name = module.as_ref().borrow().get_name().to_string();
        self.module_map.insert(name, module);
        Ok(())
    }

    pub fn get_module(&self, name: &str) -> Option<&Rc<RefCell<dyn Module>>> {
        self.module_map.get(name)
    }
}

// pub fn create_modules(
//     ast_modules: HashMap<&str, ast::Module>,
// ) -> Result<HashMap<String, Rc<RefCell<dyn Module>>>, EngineError> {
//     let mut module_map = HashMap::<String, Rc<RefCell<dyn Module>>>::new();
//
//     for (name, ast_module) in ast_modules {
//         let vm_module = VMModule::new(name, ast_module, &module_map, None)?;
//
//         module_map.insert(name.to_string(), vm_module);
//     }
//
//     Ok(module_map)
// }

// /// 从 vm 外部（即宿主）或者其他模块调用函数
// pub fn eval_function(
//     vm_module: Rc<RefCell<VMModule>>,
//     name: &str,
//     args: &[Value],
// ) -> Result<Vec<Value>, EngineError> {
//     let rc_function = vm_module.as_ref().borrow().get_export_function(name)?;
//     rc_function.as_ref().eval(args)
// }

// fn get_entry_function(vm_module: Rc<RefCell<VMModule>>) -> Option<Rc<dyn Function>> {
// 返回 `start` 段指定的函数，或者当不存在 `start` 段时，
// 寻找导出的函数当中是否存在名字为 `main` 的函数
// }

#[cfg(test)]
mod tests {
    use super::Instance;
    use anvm_ast::{ast, types::Value};
    use anvm_binary_parser::parser;
    use std::{env, fs};

    fn get_test_binary_resource(filename: &str) -> Vec<u8> {
        let mut path_buf = env::current_dir().expect("failed to get current directory");

        // 使用 `cargo test` 测试时，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm/crates/engine`；
        //
        // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm`。
        //
        // 这里需要处理这种情况。

        if !path_buf.ends_with("engine") {
            path_buf.push("crates");
            path_buf.push("engine");
        }
        let fullname_buf = path_buf.join("resources").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!("failed to read the specified file: {}", fullname))
    }

    fn get_test_ast_module(filename: &str) -> ast::Module {
        let bytes = get_test_binary_resource(filename);
        parser::parse(&bytes).unwrap()
    }

    #[test]
    fn test_module_link() {
        let mut instance = Instance::new();
        instance
            .add_ast_module("lib", get_test_ast_module("test-module-link-lib.wasm"))
            .unwrap();
        instance
            .add_ast_module("app", get_test_ast_module("test-module-link-app.wasm"))
            .unwrap();

        let rc_app_vm_module = instance.get_module("app").unwrap();

        let f0 = rc_app_vm_module
            .as_ref()
            .borrow()
            .get_export_function("test_add")
            .unwrap();
        assert_eq!(f0.as_ref().eval(&vec![]).unwrap(), vec![Value::I32(77)]);

        let f1 = rc_app_vm_module
            .as_ref()
            .borrow()
            .get_export_function("test_sub")
            .unwrap();
        assert_eq!(f1.as_ref().eval(&vec![]).unwrap(), vec![Value::I32(33)]);
    }
}
