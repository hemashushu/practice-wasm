// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::ast::{FunctionType, TypeItem};

use crate::{
    error::EngineError,
    linker::{link_functions, link_global_variables, link_memorys, link_tables},
    native_module::NativeModule,
    object::NamedAstModule,
    transformer::transform,
    vm::{Context, Status, VM},
    vm_module::VMModule,
    vm_stack::VMStack,
};

pub fn create_instance(
    native_modules: Vec<NativeModule>,
    mut named_ast_modules: Vec<NamedAstModule>,
) -> Result<VM, EngineError> {
    // 获取指令列表
    // 指令列表跟 AST 模块列表是一一对应的，所以无需映射表
    let mut function_items_list = link_functions(&native_modules, &named_ast_modules)?;
    let mut instructions_list = transform(&named_ast_modules, &function_items_list)?;

    // 获取内存块列表，以及 "AST 模块 - 内存块" 映射表
    let (memory_blocks, mut module_to_memory_block_index_list) = link_memorys(&named_ast_modules)?;

    // 获取 "表" 的列表，以及 "AST 模块 - 表" 映射表
    let (tables, mut module_to_table_index_list) = link_tables(&named_ast_modules)?;

    // 获取全局变量列表，以及 "AST 模块 - 全局变量列表" 映射表
    let (global_variables, mut module_to_global_variables_list) =
        link_global_variables(&named_ast_modules)?;

    let module_count = named_ast_modules.len();
    let mut vm_modules: Vec<VMModule> = vec![];

    for _ in 0..module_count {
        let function_items = function_items_list.pop().unwrap();
        let instructions = instructions_list.pop().unwrap();
        let memory_index = module_to_memory_block_index_list.pop().unwrap();
        let table_index = module_to_table_index_list.pop().unwrap();
        let global_variable_indexes = module_to_global_variables_list.pop().unwrap();
        let named_ast_module = named_ast_modules.pop().unwrap();

        let name = named_ast_module.name.clone();
        // let ast_module = &named_ast_module.module;

        // let function_types = ast_module
        //     .type_items
        //     .iter()
        //     .map(|item| match item {
        //         TypeItem::FunctionType(function_type) => function_type.to_owned(),
        //     })
        //     .collect::<Vec<FunctionType>>();

        // let function_to_type_indexes = ast_module
        //     .internal_function_to_type_index_list
        //     .iter()
        //     .map(|item| *item as usize)
        //     .collect::<Vec<usize>>();

        let vm_module = VMModule::new(
            name,
            table_index,
            memory_index,
            global_variable_indexes,
            // function_types,
            // function_to_type_indexes,
            function_items,
            instructions,
            named_ast_module.module,
        );

        // todo:: 填充 data 到 memory

        // todo:: 填充 element 到 table

        vm_modules.push(vm_module);
    }

    // 因为 vm_modules 的元素是反序添加的，所以这里需要翻转一次
    vm_modules.reverse();

    let stack = VMStack::new();

    let context = Context::new(
        stack,
        memory_blocks,
        tables,
        global_variables,
        native_modules,
        vm_modules,
    );

    let status = Status::new();

    let vm = VM { context, status };
    Ok(vm)
}

// pub struct Instance {
//     // pub module_map: HashMap<String, Rc<RefCell<dyn Module>>>,
//     // pub vm_modules: Vec<VMModule>,
//     // pub modules: Vec<Module>,
//
//     pub status: Status,
//     pub context: Context,
// }

// impl Instance {
// pub fn new() -> Self {
// let module_map = HashMap::<String, Rc<RefCell<dyn Module>>>::new();
// Self { module_map }
// let vm_modules = Vec::<VMModule>::new();
// Self { vm_modules }
// }

//     pub fn add_ast_module(
//         &mut self,
//         name: &str,
//         ast_module: ast::Module,
//     ) -> Result<(), EngineError> {
//         let vm_module = VMModule::new(name, ast_module, &self.module_map, None)?;
//         self.add_module(vm_module)
//     }
//
//     pub fn add_module(&mut self, vm_module: VMModule) -> Result<(), EngineError> {
//         let name = vm_module.get_name().to_string();
//         // self.module_map.insert(name, module);
//         self.vm_modules.push(vm_module);
//         Ok(())
//     }

// pub fn get_module(&self, name: &str) -> Option<&Rc<RefCell<dyn Module>>> {
//     self.module_map.get(name)
// }
// }

// #[cfg(test)]
// mod tests {
//     use super::Instance;
//     use anvm_ast::{ast, types::Value};
//     use anvm_binary_parser::parser;
//     use std::{env, fs};
//
//     fn get_test_binary_resource(filename: &str) -> Vec<u8> {
//         let mut path_buf = env::current_dir().unwrap();
//
//         // 使用 `cargo test` 测试时，
//         // `env::current_dir()` 函数获得的当前目录为
//         // `./xiaoxuan-vm/crates/engine`；
//         //
//         // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
//         // `env::current_dir()` 函数获得的当前目录为
//         // `./xiaoxuan-vm`。
//         //
//         // 下面语句用于处理这种情况。
//
//         if !path_buf.ends_with("engine") {
//             path_buf.push("crates");
//             path_buf.push("engine");
//         }
//         let fullname_buf = path_buf.join("resources").join(filename);
//         let fullname = fullname_buf.to_str().unwrap();
//         fs::read(fullname).expect(&format!("failed to read the specified binary file: {}", fullname))
//     }
//
//     fn get_test_ast_module(filename: &str) -> ast::Module {
//         let bytes = get_test_binary_resource(filename);
//         parser::parse(&bytes).unwrap()
//     }
//
//     #[test]
//     fn test_module_link() {
//         let mut instance = Instance::new();
//         instance
//             .add_ast_module("lib", get_test_ast_module("test-module-link-lib.wasm"))
//             .unwrap();
//         instance
//             .add_ast_module("app", get_test_ast_module("test-module-link-app.wasm"))
//             .unwrap();
//
//         let rc_app_vm_module = instance.get_module("app").unwrap();
//
//         let f0 = rc_app_vm_module
//             .as_ref()
//             .borrow()
//             .get_export_function("test_add")
//             .unwrap();
//         assert_eq!(f0.as_ref().eval(&vec![]).unwrap(), vec![Value::I32(77)]);
//
//         let f1 = rc_app_vm_module
//             .as_ref()
//             .borrow()
//             .get_export_function("test_sub")
//             .unwrap();
//         assert_eq!(f1.as_ref().eval(&vec![]).unwrap(), vec![Value::I32(33)]);
//     }
// }

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use anvm_ast::{ast, types::Value};
    use anvm_binary_parser::parser;

    use pretty_assertions::assert_eq;

    use crate::{error::EngineError, object::NamedAstModule};

    use super::create_instance;

    // 辅助方法
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

        if !path_buf.ends_with("engine") {
            path_buf.push("crates");
            path_buf.push("engine");
        }
        let fullname_buf = path_buf.join("resources").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!(
            "failed to read the specified binary file: {}",
            fullname
        ))
    }

    fn get_test_ast_module(filename: &str) -> ast::Module {
        let bytes = get_test_binary_resource(filename);
        parser::parse(&bytes).unwrap()
    }

    //     fn get_test_vm_module(filename: &str) -> Rc<RefCell<VMModule>> {
    //         let ast_module = get_test_ast_module(filename);
    //         let module_map = HashMap::<String, Rc<RefCell<dyn Module>>>::new();
    //         let vm_module = VMModule::new("test", ast_module, &module_map, None).unwrap();
    //         vm_module
    //     }
    //
    //     fn get_test_vm_module_with_init_memory_data(
    //         filename: &str,
    //         init_memory_data: Vec<u8>,
    //     ) -> Rc<RefCell<VMModule>> {
    //         let ast_module = get_test_ast_module(filename);
    //         let module_map = HashMap::<String, Rc<RefCell<dyn Module>>>::new();
    //         let vm_module =
    //             VMModule::new("test", ast_module, &module_map, Some(init_memory_data)).unwrap();
    //         vm_moduleeval_function_by_index
    //     }

    pub fn eval(filename: &str, index: usize, args: &[Value]) -> Result<Vec<Value>, EngineError> {
        let ast_module = get_test_ast_module(filename);
        let named_ast_module = NamedAstModule::new("test", ast_module);
        let mut vm = create_instance(vec![], vec![named_ast_module])?;

        vm.eval_function_by_index(0, index, args)
    }

    #[test]
    fn test_instruction_const() {
        let module_name = "test-const.wasm";

        // assert_eq!(
        //     eval(module_name, 0, &vec![]).unwrap(),
        //     vec![Value::I32(123)]
        // );
        // assert_eq!(
        //     eval(module_name, 1, &vec![]).unwrap(),
        //     vec![Value::I32(123), Value::I32(456)]
        // );
    }
}

/*
    #[test]
    fn test_inst_parametric() {
        let module = get_test_vm_module("test-parametric.wasm");

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(100), Value::I32(123)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(100), Value::I32(456)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(123)]
        );
        assert_eq!(eval(&module, 3, &vec![]).unwrap(), vec![]);
        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![Value::I32(100), Value::I32(123)]
        );
    }

    #[test]
    fn test_inst_numeric_eqz() {
        let module = get_test_vm_module("test-numeric-eqz.wasm");

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
    }

    #[test]
    fn test_numeric_comparsion() {
        let module = get_test_vm_module("test-numeric-comparsion.wasm");

        // i32

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 3, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );

        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(&module, 5, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 6, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 7, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );

        assert_eq!(
            eval(&module, 8, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(&module, 9, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 10, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 11, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );

        assert_eq!(
            eval(&module, 12, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 13, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 14, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 15, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );

        // f32

        assert_eq!(
            eval(&module, 16, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );
        assert_eq!(
            eval(&module, 17, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 18, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 19, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );

        assert_eq!(
            eval(&module, 20, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 21, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );
        assert_eq!(
            eval(&module, 22, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );
        assert_eq!(
            eval(&module, 23, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );

        assert_eq!(
            eval(&module, 24, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 25, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 26, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 27, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
    }

    #[test]
    fn test_numeric_unary() {
        let module = get_test_vm_module("test-numeric-unary.wasm");

        // i32

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(27)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(2)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(3)]
        );

        // f32
        assert_eq!(
            eval(&module, 3, &vec![]).unwrap(),
            vec![Value::F32(2.718)]
        );
        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![Value::F32(2.718)]
        );
        assert_eq!(
            eval(&module, 5, &vec![]).unwrap(),
            vec![Value::F32(-2.718)]
        );
        assert_eq!(
            eval(&module, 6, &vec![]).unwrap(),
            vec![Value::F32(3.0)]
        );
        assert_eq!(
            eval(&module, 7, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );
        assert_eq!(
            eval(&module, 8, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );

        // 就近取整（4 舍 6 入，5 奇进偶不进）
        assert_eq!(
            eval(&module, 9, &vec![]).unwrap(),
            vec![Value::F32(1.0)]
        );
        assert_eq!(
            eval(&module, 10, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );
        assert_eq!(
            eval(&module, 11, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );
        assert_eq!(
            eval(&module, 12, &vec![]).unwrap(),
            vec![Value::F32(4.0)]
        );

        // sqrt
        assert_eq!(
            eval(&module, 13, &vec![]).unwrap(),
            vec![Value::F32(5.0)]
        );
    }

    #[test]
    fn test_numeric_binary() {
        let module = get_test_vm_module("test-numeric-binary.wasm");

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(55)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(-11)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(726)]
        );
        assert_eq!(
            eval(&module, 3, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(-4)]
        );
        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b01111111111111111111111111111100)
            ]
        );
        assert_eq!(
            eval(&module, 5, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(-2)]
        );
        assert_eq!(
            eval(&module, 6, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(2)]
        );

        assert_eq!(
            eval(&module, 7, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0b11000)]
        );
        assert_eq!(
            eval(&module, 8, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0b1111_1001)]
        );
        assert_eq!(
            eval(&module, 9, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0b1110_0001)]
        );

        assert_eq!(
            eval(&module, 10, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b11111111_11111111_11111111_1111_0000u32 as i32)
            ]
        );

        assert_eq!(
            eval(&module, 11, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b11111111_11111111_11111111_1111_1111u32 as i32)
            ]
        );
        assert_eq!(
            eval(&module, 12, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b00001111_11111111_11111111_1111_1111)
            ]
        );

        assert_eq!(
            eval(&module, 13, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b11111111_11111111_11111111_1110_0011u32 as i32)
            ]
        );
        assert_eq!(
            eval(&module, 14, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b00_11111111_11111111_11111111_111110)
            ]
        );
    }

    #[test]
    fn test_numeric_convert() {
        let module = get_test_vm_module("test-numeric-convert.wasm");

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(123)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I64(8)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I64(8)]
        );
        assert_eq!(
            eval(&module, 3, &vec![]).unwrap(),
            vec![Value::I64(-8)]
        );
        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![Value::I64(0x00_00_00_00_ff_ff_ff_f8)]
        );

        assert_eq!(
            eval(&module, 5, &vec![]).unwrap(),
            vec![Value::I32(3)]
        );
        assert_eq!(
            eval(&module, 6, &vec![]).unwrap(),
            vec![Value::I32(3)]
        );

        assert_eq!(
            eval(&module, 7, &vec![]).unwrap(),
            vec![Value::F32(66.0)]
        );
        assert_eq!(
            eval(&module, 8, &vec![]).unwrap(),
            vec![Value::F32(66.0)]
        );

        // todo:: 这里仅测试了部分指令
    }

    #[test]
    fn test_variable() {
        let module = get_test_vm_module("test-variable.wasm");

        assert_eq!(
            eval(&module, 0, &vec![Value::I32(11), Value::I32(22)]).unwrap(),
            vec![Value::I32(22), Value::I32(11)]
        );
        assert_eq!(
            eval(&module, 1, &vec![Value::I32(11), Value::I32(22)]).unwrap(),
            vec![Value::I32(33)]
        );
        assert_eq!(
            eval(&module, 2, &vec![Value::I32(55), Value::I32(66)]).unwrap(),
            vec![
                Value::I32(55),
                Value::I32(66),
                Value::I32(20),
                Value::I32(10),
            ]
        );
    }

    #[test]
    fn test_memory_page() {
        let module = get_test_vm_module("test-memory-page.wasm");

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(2)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(2), Value::I32(4), Value::I32(7)]
        );
    }

    #[test]
    fn test_memory_load() {
        let init_memory_data: Vec<u8> = vec![
            /* addr: 0      */ 0x11, // 17
            /* addr: 1      */ 0xf1, // uint8'241 == int8'-15 (-15=241-256)
            /* addr: 2,3    */ 0x55, 0x66, // 0x6655
            /* addr: 4,5    */ 0x80, 0x90, // 0x9080
            /* addr: 6..13  */ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            /* addr: 14..21 */ 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0,
        ];

        let module =
            get_test_vm_module_with_init_memory_data("test-memory-load.wasm", init_memory_data);

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(0x11)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![
                Value::I32(0x11),
                Value::I32(0xf1),
                Value::I32(0x55),
                Value::I32(0x66)
            ]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![
                Value::I32(0x11),
                Value::I32(0xf1),
                Value::I32(0x55),
                Value::I32(0x66)
            ]
        );

        // 测试符号
        assert_eq!(
            eval(&module, 3, &vec![]).unwrap(),
            vec![
                Value::I32(17),
                Value::I32(17),
                Value::I32(241),
                Value::I32(-15)
            ]
        );

        // 测试 16 位和 32 位整数
        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![
                Value::I32(0x6655),
                Value::I32(0x6655),
                Value::I32(0x9080),
                Value::I32(0xffff9080u32 as i32),
                Value::I32(0x03020100)
            ]
        );

        // 测试 64 位整数
        assert_eq!(
            eval(&module, 5, &vec![]).unwrap(),
            vec![
                Value::I64(0x03020100),
                Value::I64(0x03020100),
                Value::I64(0xb0a09080),
                Value::I64(0xffffffffb0a09080u64 as i64),
                Value::I64(0x0706050403020100),
                Value::I64(0xf0e0d0c0b0a09080u64 as i64)
            ]
        );
    }

    #[test]
    fn test_memory_store() {
        let module = get_test_vm_module("test-memory-store.wasm");

        // 检查内存 （经过 data 段）初始化之后的内容
        let d0: Vec<u8> = vec![
            0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x22, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x77, 0x66, 0x55, 0x44, 0x00, 0x00, 0x00, 0x00, 0x80, 0x90, 0xa0, 0xb0,
            0xc0, 0xd0, 0xe0, 0xf0, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x00, 0x00, 0xE4, 0xB8,
            0xAD, 0xE6, 0x96, 0x87, 0x00, 0x00,
        ];
        assert_eq!(module.as_ref().borrow().dump_memory(0, d0.len()), d0);

        // 测试读取数据
        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(0x11), Value::I32(0x2233), Value::I32(0x44556677)]
        );

        // 测试读取数据
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![
                Value::I64(0xf0e0d0c0b0a09080u64 as i64),
                Value::I64(0x68),
                Value::I64(0xe4)
            ]
        );

        // 测试 i32.store8
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(0xddccbbaau32 as i32)]
        );
        let d2: Vec<u8> = vec![0xaa, 0xbb, 0xcc, 0xdd, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(module.as_ref().borrow().dump_memory(0, d2.len()), d2);

        // 测试 i32 和 i64 的各种类型 store 指令
        assert_eq!(eval(&module, 3, &vec![]).unwrap(), vec![]);
        let d3: Vec<u8> = vec![
            0xaa, 0xbb, 0xcc, 0xdd, 0x02, 0x01, 0x00, 0x00, 0xa3, 0xa2, 0xa1, 0xa0, 0xb0, 0x00,
            0xc1, 0xc0, 0xd3, 0xd2, 0xd1, 0xd0, 0xe7, 0xe6, 0xe5, 0xe4, 0xe3, 0xe2, 0xe1, 0xe0,
        ];
        assert_eq!(module.as_ref().borrow().dump_memory(0, d3.len()), d3);

        // 测试 memory.grow 指令之后，访问原有的内存数据
        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![
                Value::I32(1),
                Value::I32(2),
                Value::I32(0xaabbccddu32 as i32),
                Value::I32(0x10012002),
            ]
        );
    }

    #[test]
    fn test_function_call() {
        let module = get_test_vm_module("test-function-call.wasm");

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(3)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(-5)]
        );
    }

    #[test]
    fn test_function_indirect_call() {
        let module = get_test_vm_module("test-function-indirect-call.wasm");

        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(12)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(8)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(20)]
        );
        assert_eq!(
            eval(&module, 3, &vec![]).unwrap(),
            vec![Value::I32(5)]
        );
    }

    #[test]
    fn test_branch() {
        let module = get_test_vm_module("test-branch.wasm");

        // 测试 return
        assert_eq!(
            eval(&module, 0, &vec![]).unwrap(),
            vec![Value::I32(1)]
        );
        assert_eq!(
            eval(&module, 1, &vec![]).unwrap(),
            vec![Value::I32(2)]
        );
        assert_eq!(
            eval(&module, 2, &vec![]).unwrap(),
            vec![Value::I32(3)]
        );

        // 测试 br
        assert_eq!(
            eval(&module, 3, &vec![]).unwrap(),
            vec![Value::I32(4)]
        );
        assert_eq!(
            eval(&module, 4, &vec![]).unwrap(),
            vec![Value::I32(2)]
        );
        assert_eq!(
            eval(&module, 5, &vec![]).unwrap(),
            vec![Value::I32(11)]
        );
        assert_eq!(
            eval(&module, 6, &vec![]).unwrap(),
            vec![Value::I32(12)]
        );
        assert_eq!(
            eval(&module, 7, &vec![]).unwrap(),
            vec![Value::I32(13)]
        );

        // 测试 br_if
        assert_eq!(
            eval(&module, 8, &vec![]).unwrap(),
            vec![Value::I32(55)]
        );

        // 测试 br_table
        // todo::

        // 测试 if
        assert_eq!(
            eval(&module, 9, &vec![]).unwrap(),
            vec![Value::I32(2)]
        );
        assert_eq!(
            eval(&module, 10, &vec![]).unwrap(),
            vec![Value::I32(1)]
        );
    }

*/
