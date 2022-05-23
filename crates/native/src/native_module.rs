// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use anvm_ast::{ast::FunctionType, types::ValueType};
use anvm_engine::object::{EngineError, Export, Function, GlobalVariable, Memory, Module, Table};

use crate::native_function::{InternalFunction, NativeFunction};

pub const MODULE_NAME: &str = "native";
pub const MEMORY_BLOCK_NAME: &str = "memory";

struct NativeModule {
    pub name: String,
    pub memory: Rc<RefCell<dyn Memory>>,
    pub functions: Vec<Rc<dyn Function>>,

    function_types: Vec<Rc<FunctionType>>,
    function_name_map: HashMap<String, usize>,
}

impl Module for NativeModule {
    fn get_name(&self) -> &str {
        "native"
    }

    fn get_export_table(&self, name: &str) -> Result<Rc<RefCell<dyn Table>>, EngineError> {
        Err(EngineError::ObjectNotFound(format!(
            "no export table for module \"{}\"",
            MODULE_NAME
        )))
    }

    fn get_export_memory(&self, name: &str) -> Result<Rc<RefCell<dyn Memory>>, EngineError> {
        // 约定导出一个名为 "memory" 的内存块
        if name == MEMORY_BLOCK_NAME {
            Ok(Rc::clone(&self.memory))
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot find the specified memory \"{}.{}\"",
                self.name, name
            )))
        }
    }

    fn get_export_function(&self, name: &str) -> Result<Rc<dyn Function>, EngineError> {
        let option_function_index = self.function_name_map.get(name);
        if let Some(function_index) = option_function_index {
            Ok(self.get_function_by_index(*function_index))
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot find the specified function \"{}.{}\"",
                self.name, name
            )))
        }
    }

    fn get_export_global_variable(
        &self,
        name: &str,
    ) -> Result<Rc<RefCell<dyn GlobalVariable>>, EngineError> {
        Err(EngineError::ObjectNotFound(format!(
            "no export table for module \"{}\"",
            self.name
        )))
    }

    fn get_exports(&self) -> Vec<Export> {
        todo!()
    }

    fn dump_memory(&self, address: usize, length: usize) -> Vec<u8> {
        let memory = self.memory.as_ref().borrow();
        let data = memory.read_bytes(address, length);
        data.to_vec()
    }

    fn get_function_by_index(&self, index: usize) -> Rc<dyn Function> {
        Rc::clone(&self.functions[index])
    }
}

impl NativeModule {
    pub fn new(memory: Rc<RefCell<dyn Memory>>) -> Self {
        Self {
            name: MODULE_NAME.to_string(),
            memory,
            functions: vec![],
            function_types: vec![],
            function_name_map: HashMap::<String, usize>::new(),
        }
    }

    /// 添加 FunctionType 到内部列表，
    /// 如果相同的 FunctionType 已经存在，则返回已存在的。
    fn add_function_type(
        &mut self,
        params: Vec<ValueType>,
        results: Vec<ValueType>,
    ) -> Rc<FunctionType> {
        let function_type = FunctionType { params, results };

        let option_function_type = self
            .function_types
            .iter()
            .find(|t| t.as_ref() == &function_type);

        if let Some(exist_rc_function_type) = option_function_type {
            Rc::clone(exist_rc_function_type)
        } else {
            let rc_function_type = Rc::new(function_type);
            let clone_rc_function_type = Rc::clone(&rc_function_type);
            self.function_types.push(rc_function_type);
            clone_rc_function_type
        }
    }

    pub fn add_function(
        &mut self,
        name: &str,
        params: Vec<ValueType>,
        results: Vec<ValueType>,
        internal_function: InternalFunction,
    ) {
        let rc_function_type = self.add_function_type(params, results);

        let index = self.functions.len();

        let native_function =
            NativeFunction::new(rc_function_type, index, internal_function, name.to_string());

        self.functions.push(Rc::new(native_function));
        self.function_name_map.insert(name.to_string(), index);
    }
}

#[cfg(test)]
mod tests {
    use anvm_ast::{
        ast,
        types::{Value, ValueType},
    };
    use anvm_binary_parser::parser;
    use anvm_engine::{self, instance::Instance, object::Module, vm_memory::VMMemory};
    use std::{cell::RefCell, env, fs, rc::Rc};

    use crate::interface::{InternalError, NativeError};

    use super::NativeModule;

    #[derive(Debug)]
    struct TestInternalError {
        //
    }

    impl InternalError for TestInternalError {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    fn get_test_binary_resource(filename: &str) -> Vec<u8> {
        let mut path_buf = env::current_dir().unwrap();

        // 使用 `cargo test` 测试时，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm/crates/engine`；
        //
        // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm`。
        //
        // 下面语句用于处理这种情况。

        if !path_buf.ends_with("native") {
            path_buf.push("crates");
            path_buf.push("native");
        }
        let fullname_buf = path_buf.join("resources").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!("failed to read the specified binary file: {}", fullname))
    }

    fn get_test_ast_module(filename: &str) -> ast::Module {
        let bytes = get_test_binary_resource(filename);
        parser::parse(&bytes).unwrap()
    }

    fn test_internal_function_add_i32(params: &[Value]) -> Result<Vec<Value>, NativeError> {
        match (params[0], params[1]) {
            (Value::I32(left), Value::I32(right)) => Ok(vec![Value::I32(left + right)]),
            _ => Err(NativeError::new(
                Box::new(TestInternalError {}),
                "add_i32 require two i32 parameters",
            )),
        }
    }

    fn get_test_native_module() -> NativeModule {
        let memory = VMMemory::new_with_min_page(1);
        let mut module = NativeModule::new(Rc::new(RefCell::new(memory)));

        module.add_function(
            "add_i32",
            vec![ValueType::I32, ValueType::I32],
            vec![ValueType::I32],
            test_internal_function_add_i32,
        );

        module
    }

    #[test]
    fn test_native_function_add_i32() {
        let mut instance = Instance::new();

        let native_module = get_test_native_module();
        let rc_native_module = Rc::new(RefCell::new(native_module));

        instance.add_module(rc_native_module).unwrap();

        instance
            .add_ast_module("app", get_test_ast_module("test-module-native.wasm"))
            .unwrap();

        let rc_app_vm_module = instance.get_module("app").unwrap();

        let f0 = rc_app_vm_module
            .as_ref()
            .borrow()
            .get_export_function("test_add")
            .unwrap();
        assert_eq!(f0.as_ref().eval(&vec![]).unwrap(), vec![Value::I32(33)]);
    }
}
