// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::{
    ast::{self, FunctionType, TypeItem},
    types::{Value, ValueType},
};

use crate::{
    decoder::{decode, decode_constant_expression},
    error::{
        make_invalid_memory_index_engine_error, make_invalid_table_index_engine_error, EngineError,
    },
    linker::{link_functions, link_global_variables, link_memorys, link_tables},
    native_module::NativeModule,
    object::NamedAstModule,
    vm::{Resource, Status, VM},
    vm_module::VMModule,
    vm_stack::VMStack,
};

pub fn create_instance(
    native_modules: Vec<NativeModule>,
    named_ast_modules: &[NamedAstModule],
) -> Result<VM, EngineError> {
    // 获取指令列表
    // 指令列表跟 AST 模块列表是一一对应的，所以无需映射表
    let mut function_items_list = link_functions(&native_modules, named_ast_modules)?;
    let mut instructions_list = decode(named_ast_modules, &function_items_list)?;

    // 获取 "表" 实例列表，以及 "AST 模块 - 表" 映射表
    let (tables, mut module_to_table_index_list) = link_tables(named_ast_modules)?;

    // 获取内存块实例列表，以及 "AST 模块 - 内存块" 映射表
    let (memory_blocks, mut module_to_memory_block_index_list) = link_memorys(named_ast_modules)?;

    // 获取全局变量实例列表，以及 "AST 模块 - 全局变量列表" 映射表
    let (global_variables, mut module_to_global_variables_list) =
        link_global_variables(named_ast_modules)?;

    let ast_module_count = named_ast_modules.len();
    let mut vm_modules: Vec<VMModule> = vec![];

    for reverse_index in 0..ast_module_count {
        let function_items = function_items_list.pop().unwrap();
        let instructions = instructions_list.pop().unwrap();
        let table_index = module_to_table_index_list.pop().unwrap();
        let memory_index = module_to_memory_block_index_list.pop().unwrap();
        let global_variable_indexes = module_to_global_variables_list.pop().unwrap();

        let ast_module_index = ast_module_count - reverse_index - 1;
        let named_ast_module = &named_ast_modules[ast_module_index];

        let name = named_ast_module.name.clone();
        let ast_module = &named_ast_module.module;

        // 复制一份函数类型表
        let function_types = ast_module
            .type_items
            .iter()
            .map(|item| match item {
                TypeItem::FunctionType(function_type) => function_type.to_owned(),
            })
            .collect::<Vec<FunctionType>>();

        let internal_function_local_variable_types_list = ast_module
            .code_items
            .iter()
            .map(|item| {
                item.local_groups
                    .iter()
                    .flat_map(|local_group| {
                        vec![local_group.value_type.clone(); local_group.variable_count as usize]
                    })
                    .collect::<Vec<ValueType>>()
            })
            .collect::<Vec<Vec<ValueType>>>();

        let vm_module = VMModule::new(
            name,
            table_index,
            memory_index,
            global_variable_indexes,
            function_types,
            internal_function_local_variable_types_list,
            function_items,
            instructions,
            vec![] // TODO::
        );

        vm_modules.push(vm_module);
    }

    // 因为 vm_modules 的元素是反序添加的，所以这里需要翻转一次
    vm_modules.reverse();

    // 构建 VM 实例

    let stack = VMStack::new();

    let resource = Resource::new(
        memory_blocks,
        tables,
        global_variables,
        native_modules,
        vm_modules,
    );

    let status = Status::new();

    let mut vm = VM {
        stack,
        status,
        resource,
    };

    // 填充 data 和 element 到 memory 和 table
    //
    // 因为 data 和 element 的常量表达式里可能存在引用数据，所以需要先构造了 vm 之后
    // 再对表达式进行求值。
    // https://webassembly.github.io/spec/core/valid/instructions.html#constant-expressions
    for ast_module_index in 0..ast_module_count {
        let named_ast_module = &named_ast_modules[ast_module_index];
        let ast_module = &named_ast_module.module;

        let instance_memory_index = vm.resource.vm_modules[ast_module_index].memory_index;
        let instance_table_index = vm.resource.vm_modules[ast_module_index].table_index;

        // 填充 data 到 memory
        for data_item in &ast_module.data_items {
            // 内存块索引，目前 WebAssembly 标准只支持 0
            if data_item.memory_block_index != 0 {
                return Err(make_invalid_memory_index_engine_error());
            }

            let offset_instruction_items = &data_item.offset_instruction_items;
            let constant_expression = decode_constant_expression(offset_instruction_items)?;
            let offset = vm.eval_constant_expression(&constant_expression)?;

            let address = match offset {
                Value::I32(v) => v as usize,
                _ => {
                    return Err(EngineError::InvalidOperation(
                        "memory data offset should be a i32 number".to_string(),
                    ));
                }
            };

            let data = &data_item.data;
            vm.resource.memory_blocks[instance_memory_index].write_bytes(address, data);
        }

        // 填充 element 到 table
        for element_item in &ast_module.element_items {
            // 表索引，目前 WebAssembly 标准只支持 0
            if element_item.table_index != 0 {
                return Err(make_invalid_table_index_engine_error());
            }

            let offset_instruction_items = &element_item.offset_instruction_items;
            let constant_expression = decode_constant_expression(offset_instruction_items)?;
            let offset_value = vm.eval_constant_expression(&constant_expression)?;

            let offset = match offset_value {
                Value::I32(v) => v as usize,
                _ => {
                    return Err(EngineError::InvalidOperation(
                        "table element offset should be a i32 number".to_string(),
                    ));
                }
            };

            for (index, function_index) in element_item.function_indices.iter().enumerate() {
                vm.resource.tables[instance_table_index]
                    .set_element(offset + index, *function_index)?;
            }
        }
    }

    Ok(vm)
}

/// 从 named_ast_modules 的最后一个元素开始，寻找 ast module 当中
/// `start` 段指定的函数或者导出名称为 `main` 的函数的索引。
pub fn get_entry_module_and_function_index(
    named_ast_modules: &[NamedAstModule],
) -> Option<(usize, usize)> {
    let mut option_mod_and_func_index: Option<(usize, usize)> = None;

    for (module_index, named_ast_module) in named_ast_modules.iter().enumerate().rev() {
        let ast_module = &named_ast_module.module;
        if let Some(i) = ast_module.start_function_index {
            option_mod_and_func_index = Some((module_index, i as usize));
            break;
        } else if let Some(i) = find_ast_module_export_function(ast_module, "main") {
            // 查找导出函数当中，名字为 `main` 的函数的索引
            option_mod_and_func_index = Some((module_index, i as usize));
            break;
        }
    }

    option_mod_and_func_index
}

/// 在指定的模块当中查找指定名字的导出函数的索引
pub fn find_ast_module_export_function(ast_module: &ast::Module, export_name: &str) -> Option<u32> {
    ast_module
        .export_items
        .iter()
        .find_map(|item| match &item.export_descriptor {
            ast::ExportDescriptor::FunctionIndex(i) if item.name == export_name => Some(*i),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use anvm_ast::{
        ast,
        types::{Value, ValueType},
    };
    use anvm_binary_parser::parser;

    use pretty_assertions::assert_eq;

    use crate::{
        error::{EngineError, NativeError},
        native_module::NativeModule,
        object::NamedAstModule,
    };

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

    fn native_function_add_i32(params: &[Value]) -> Result<Vec<Value>, NativeError> {
        match (params[0], params[1]) {
            (Value::I32(left), Value::I32(right)) => Ok(vec![Value::I32(left + right)]),
            _ => panic!("incorrect data type of the native function arguments"),
        }
    }

    fn native_function_sub_i32(params: &[Value]) -> Result<Vec<Value>, NativeError> {
        match (params[0], params[1]) {
            (Value::I32(left), Value::I32(right)) => Ok(vec![Value::I32(left - right)]),
            _ => panic!("incorrect data type of the native function arguments"),
        }
    }

    fn get_test_native_module() -> NativeModule {
        let mut native_module = NativeModule::new("math");

        native_module.add_native_function(
            "add",
            vec![ValueType::I32, ValueType::I32],
            vec!["left", "right"],
            vec![ValueType::I32],
            native_function_add_i32,
        );

        native_module.add_native_function(
            "sub",
            vec![ValueType::I32, ValueType::I32],
            vec!["left", "right"],
            vec![ValueType::I32],
            native_function_sub_i32,
        );

        native_module
    }

    fn eval(
        filename: &str,
        function_index: usize,
        args: &[Value],
    ) -> Result<Vec<Value>, EngineError> {
        let ast_module = get_test_ast_module(filename);
        let named_ast_module = NamedAstModule::new("test", ast_module);
        let mut vm = create_instance(vec![], &vec![named_ast_module])?;

        vm.eval_function_by_index(0, function_index, args)
    }

    fn eval_with_initial_memory_data(
        filename: &str,
        function_index: usize,
        args: &[Value],
        initial_memory_data: &[u8],
    ) -> Result<Vec<Value>, EngineError> {
        let ast_module = get_test_ast_module(filename);
        let named_ast_module = NamedAstModule::new("test", ast_module);
        let mut vm = create_instance(vec![], &vec![named_ast_module])?;

        vm.resource.memory_blocks[0].write_bytes(0, initial_memory_data);
        vm.eval_function_by_index(0, function_index, args)
    }

    fn eval_and_dump_memory_data(
        filename: &str,
        function_index: usize,
        args: &[Value],
        address: usize,
        length: usize,
    ) -> Result<(Vec<Value>, Vec<u8>), EngineError> {
        let ast_module = get_test_ast_module(filename);
        let named_ast_module = NamedAstModule::new("test", ast_module);
        let mut vm = create_instance(vec![], &vec![named_ast_module])?;

        let result = vm.eval_function_by_index(0, function_index, args)?;
        let data = vm.resource.memory_blocks[0]
            .read_bytes(address, length)
            .to_vec();
        Ok((result, data))
    }

    fn eval_with_multiple_modules(
        function_index: usize,
        args: &[Value],
    ) -> Result<Vec<Value>, EngineError> {
        let native_module = get_test_native_module();

        let named_ast_module_callee = NamedAstModule::new(
            "callee",
            get_test_ast_module("test-function-call-callee.wasm"),
        );

        let named_ast_module_callee_intermediate = NamedAstModule::new(
            "intermediate",
            get_test_ast_module("test-function-call-callee-intermediate.wasm"),
        );

        let named_ast_module_caller = NamedAstModule::new(
            "caller",
            get_test_ast_module("test-function-call-caller.wasm"),
        );

        let mut vm = create_instance(
            vec![native_module],
            &vec![
                named_ast_module_callee,
                named_ast_module_callee_intermediate,
                named_ast_module_caller,
            ],
        )?;

        vm.eval_function_by_index(2, function_index, args)
    }

    fn convert_i32_list(values: &[i32]) -> Vec<Value> {
        values
            .iter()
            .map(|v| Value::I32(*v))
            .collect::<Vec<Value>>()
    }

    #[test]
    fn test_instruction_const() {
        let module_name = "test-const.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(123)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![]).unwrap(),
            vec![Value::I32(123), Value::I32(456)]
        );
    }

    #[test]
    fn test_inst_parametric() {
        let module_name = "test-parametric.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(100), Value::I32(123)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![]).unwrap(),
            vec![Value::I32(100), Value::I32(456)]
        );
        assert_eq!(
            eval(module_name, 2, &vec![]).unwrap(),
            vec![Value::I32(123)]
        );
        assert_eq!(eval(module_name, 3, &vec![]).unwrap(), vec![]);
        assert_eq!(
            eval(module_name, 4, &vec![]).unwrap(),
            vec![Value::I32(100), Value::I32(123)]
        );
    }

    #[test]
    fn test_inst_numeric_eqz() {
        let module_name = "test-numeric-eqz.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(module_name, 2, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
    }

    #[test]
    fn test_numeric_comparsion() {
        let module_name = "test-numeric-comparsion.wasm";

        // i32

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 2, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 3, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );

        assert_eq!(
            eval(module_name, 4, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(module_name, 5, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 6, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 7, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );

        assert_eq!(
            eval(module_name, 8, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );
        assert_eq!(
            eval(module_name, 9, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 10, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 11, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(0)]
        );

        assert_eq!(
            eval(module_name, 12, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 13, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 14, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 15, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(1)]
        );

        // f32

        assert_eq!(
            eval(module_name, 16, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );
        assert_eq!(
            eval(module_name, 17, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 18, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 19, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );

        assert_eq!(
            eval(module_name, 20, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 21, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );
        assert_eq!(
            eval(module_name, 22, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0)]
        );
        assert_eq!(
            eval(module_name, 23, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );

        assert_eq!(
            eval(module_name, 24, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 25, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 26, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
        assert_eq!(
            eval(module_name, 27, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(1)]
        );
    }

    #[test]
    fn test_numeric_unary() {
        let module_name = "test-numeric-unary.wasm";

        // i32

        assert_eq!(eval(module_name, 0, &vec![]).unwrap(), vec![Value::I32(27)]);
        assert_eq!(eval(module_name, 1, &vec![]).unwrap(), vec![Value::I32(2)]);
        assert_eq!(eval(module_name, 2, &vec![]).unwrap(), vec![Value::I32(3)]);

        // f32
        assert_eq!(
            eval(module_name, 3, &vec![]).unwrap(),
            vec![Value::F32(2.718)]
        );
        assert_eq!(
            eval(module_name, 4, &vec![]).unwrap(),
            vec![Value::F32(2.718)]
        );
        assert_eq!(
            eval(module_name, 5, &vec![]).unwrap(),
            vec![Value::F32(-2.718)]
        );
        assert_eq!(
            eval(module_name, 6, &vec![]).unwrap(),
            vec![Value::F32(3.0)]
        );
        assert_eq!(
            eval(module_name, 7, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );
        assert_eq!(
            eval(module_name, 8, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );

        // 就近取整（4 舍 6 入，5 奇进偶不进）
        assert_eq!(
            eval(module_name, 9, &vec![]).unwrap(),
            vec![Value::F32(1.0)]
        );
        assert_eq!(
            eval(module_name, 10, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );
        assert_eq!(
            eval(module_name, 11, &vec![]).unwrap(),
            vec![Value::F32(2.0)]
        );
        assert_eq!(
            eval(module_name, 12, &vec![]).unwrap(),
            vec![Value::F32(4.0)]
        );

        // sqrt
        assert_eq!(
            eval(module_name, 13, &vec![]).unwrap(),
            vec![Value::F32(5.0)]
        );
    }

    #[test]
    fn test_numeric_binary() {
        let module_name = "test-numeric-binary.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(55)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(-11)]
        );
        assert_eq!(
            eval(module_name, 2, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(726)]
        );
        assert_eq!(
            eval(module_name, 3, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(-4)]
        );
        assert_eq!(
            eval(module_name, 4, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b01111111111111111111111111111100)
            ]
        );
        assert_eq!(
            eval(module_name, 5, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(-2)]
        );
        assert_eq!(
            eval(module_name, 6, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(2)]
        );

        assert_eq!(
            eval(module_name, 7, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0b11000)]
        );
        assert_eq!(
            eval(module_name, 8, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0b1111_1001)]
        );
        assert_eq!(
            eval(module_name, 9, &vec![]).unwrap(),
            vec![Value::I32(11), Value::I32(0b1110_0001)]
        );

        assert_eq!(
            eval(module_name, 10, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b11111111_11111111_11111111_1111_0000u32 as i32)
            ]
        );

        assert_eq!(
            eval(module_name, 11, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b11111111_11111111_11111111_1111_1111u32 as i32)
            ]
        );
        assert_eq!(
            eval(module_name, 12, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b00001111_11111111_11111111_1111_1111)
            ]
        );

        assert_eq!(
            eval(module_name, 13, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b11111111_11111111_11111111_1110_0011u32 as i32)
            ]
        );
        assert_eq!(
            eval(module_name, 14, &vec![]).unwrap(),
            vec![
                Value::I32(11),
                Value::I32(0b00_11111111_11111111_11111111_111110)
            ]
        );
    }

    #[test]
    fn test_numeric_convert() {
        let module_name = "test-numeric-convert.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(123)]
        );
        assert_eq!(eval(module_name, 1, &vec![]).unwrap(), vec![Value::I64(8)]);
        assert_eq!(eval(module_name, 2, &vec![]).unwrap(), vec![Value::I64(8)]);
        assert_eq!(eval(module_name, 3, &vec![]).unwrap(), vec![Value::I64(-8)]);
        assert_eq!(
            eval(module_name, 4, &vec![]).unwrap(),
            vec![Value::I64(0x00_00_00_00_ff_ff_ff_f8)]
        );

        assert_eq!(eval(module_name, 5, &vec![]).unwrap(), vec![Value::I32(3)]);
        assert_eq!(eval(module_name, 6, &vec![]).unwrap(), vec![Value::I32(3)]);

        assert_eq!(
            eval(module_name, 7, &vec![]).unwrap(),
            vec![Value::F32(66.0)]
        );
        assert_eq!(
            eval(module_name, 8, &vec![]).unwrap(),
            vec![Value::F32(66.0)]
        );

        // todo:: 这里仅测试了部分指令
    }

    #[test]
    fn test_local_variable() {
        let module_name = "test-local-variable.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![Value::I32(10), Value::I32(20)]).unwrap(),
            vec![Value::I32(10), Value::I32(20)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![Value::I32(33), Value::I32(44)]).unwrap(),
            vec![Value::I32(-11)]
        );
        assert_eq!(
            eval(module_name, 2, &vec![Value::I32(33), Value::I32(22)]).unwrap(),
            vec![Value::I32(10),]
        );
    }

    #[test]
    fn test_global_variable() {
        let module_name = "test-global-variable.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(55), Value::I32(66)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![Value::I32(3), Value::I32(4)]).unwrap(),
            vec![Value::I32(-12)]
        );
        assert_eq!(
            eval(module_name, 2, &vec![Value::I32(7)]).unwrap(),
            vec![Value::I32(114),]
        );
    }

    #[test]
    fn test_memory_page() {
        let module_name = "test-memory-page.wasm";

        assert_eq!(
            eval(module_name, 0, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(2)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![]).unwrap(),
            vec![Value::I32(10), Value::I32(2), Value::I32(4), Value::I32(7)]
        );
    }

    #[test]
    fn test_memory_load() {
        let initial_memory_data: Vec<u8> = vec![
            /* addr: 0      */ 0x11, // 17
            /* addr: 1      */ 0xf1, // uint8'241 == int8'-15 (-15=241-256)
            /* addr: 2,3    */ 0x55, 0x66, // 0x6655
            /* addr: 4,5    */ 0x80, 0x90, // 0x9080
            /* addr: 6..13  */ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            /* addr: 14..21 */ 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0,
        ];

        let module_name = "test-memory-load.wasm";

        assert_eq!(
            eval_with_initial_memory_data(module_name, 0, &vec![], &initial_memory_data).unwrap(),
            vec![Value::I32(0x11)]
        );
        assert_eq!(
            eval_with_initial_memory_data(module_name, 1, &vec![], &initial_memory_data).unwrap(),
            vec![
                Value::I32(0x11),
                Value::I32(0xf1),
                Value::I32(0x55),
                Value::I32(0x66)
            ]
        );
        assert_eq!(
            eval_with_initial_memory_data(module_name, 2, &vec![], &initial_memory_data).unwrap(),
            vec![
                Value::I32(0x11),
                Value::I32(0xf1),
                Value::I32(0x55),
                Value::I32(0x66)
            ]
        );

        // 测试符号
        assert_eq!(
            eval_with_initial_memory_data(module_name, 3, &vec![], &initial_memory_data).unwrap(),
            vec![
                Value::I32(17),
                Value::I32(17),
                Value::I32(241),
                Value::I32(-15)
            ]
        );

        // 测试 16 位和 32 位整数
        assert_eq!(
            eval_with_initial_memory_data(module_name, 4, &vec![], &initial_memory_data).unwrap(),
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
            eval_with_initial_memory_data(module_name, 5, &vec![], &initial_memory_data).unwrap(),
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
        let module_name = "test-memory-store.wasm";

        // 测试 i32.load
        let e0: Vec<u8> = vec![
            0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x33, 0x22, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x77, 0x66, 0x55, 0x44, 0x00, 0x00, 0x00, 0x00, 0x80, 0x90, 0xa0, 0xb0,
            0xc0, 0xd0, 0xe0, 0xf0, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x00, 0x00, 0xE4, 0xB8,
            0xAD, 0xE6, 0x96, 0x87, 0x00, 0x00,
        ];
        let (r0, d0) = eval_and_dump_memory_data(module_name, 0, &vec![], 0, e0.len()).unwrap();
        assert_eq!(
            r0,
            vec![Value::I32(0x11), Value::I32(0x2233), Value::I32(0x44556677)]
        );
        assert_eq!(d0, e0);

        // 测试 i64.load
        let r1 = eval(module_name, 1, &vec![]).unwrap();
        assert_eq!(
            r1,
            vec![
                Value::I64(0xf0e0d0c0b0a09080u64 as i64),
                Value::I64(0x68),
                Value::I64(0xe4)
            ]
        );

        // 测试 i32.store8
        let e2: Vec<u8> = vec![0xaa, 0xbb, 0xcc, 0xdd, 0x00, 0x00, 0x00, 0x00];
        let (r2, d2) = eval_and_dump_memory_data(module_name, 2, &vec![], 0, e2.len()).unwrap();
        assert_eq!(r2, vec![Value::I32(0xddccbbaau32 as i32)]);
        assert_eq!(e2, d2);

        // 测试 i32 和 i64 的各种类型 store 指令
        let e3: Vec<u8> = vec![
            0xaa, 0xbb, 0xcc, 0xdd, 0x02, 0x01, 0x00, 0x00, 0xa3, 0xa2, 0xa1, 0xa0, 0xb0, 0x00,
            0xc1, 0xc0, 0xd3, 0xd2, 0xd1, 0xd0, 0xe7, 0xe6, 0xe5, 0xe4, 0xe3, 0xe2, 0xe1, 0xe0,
        ];
        let (r3, d3) = eval_and_dump_memory_data(module_name, 3, &vec![], 0, e3.len()).unwrap();
        assert_eq!(r3, vec![]);
        assert_eq!(d3, d3);

        // 测试 memory.grow 指令之后，访问原有的内存数据
        let r4 = eval(module_name, 4, &vec![]).unwrap();
        assert_eq!(
            r4,
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
        let module_name = "test-function-call.wasm";

        // test stack call frame
        assert_eq!(eval(module_name, 0, &vec![]).unwrap(), vec![Value::I32(2)]);

        // call max
        assert_eq!(
            eval(module_name, 1, &vec![Value::I32(55), Value::I32(66)]).unwrap(),
            vec![Value::I32(66)]
        );
        assert_eq!(
            eval(module_name, 1, &vec![Value::I32(66), Value::I32(55)]).unwrap(),
            vec![Value::I32(66)]
        );

        // call abs
        assert_eq!(
            eval(module_name, 2, &vec![Value::I32(123)]).unwrap(),
            vec![Value::I32(123)]
        );
        assert_eq!(
            eval(module_name, 2, &vec![Value::I32(-123)]).unwrap(),
            vec![Value::I32(123)]
        );

        // call abs_max
        assert_eq!(
            eval(module_name, 3, &vec![Value::I32(-55), Value::I32(66)]).unwrap(),
            vec![Value::I32(66)]
        );
        assert_eq!(
            eval(module_name, 3, &vec![Value::I32(55), Value::I32(-66)]).unwrap(),
            vec![Value::I32(-66)]
        );
        assert_eq!(
            eval(module_name, 3, &vec![Value::I32(55), Value::I32(-44)]).unwrap(),
            vec![Value::I32(55)]
        );
        assert_eq!(
            eval(module_name, 3, &vec![Value::I32(-55), Value::I32(44)]).unwrap(),
            vec![Value::I32(-55)]
        );
    }

    #[test]
    fn test_function_call_native() {
        // 测试 $na_add
        assert_eq!(
            eval_with_multiple_modules(0, &vec![Value::I32(55), Value::I32(66)]).unwrap(),
            vec![Value::I32(121)]
        );
        assert_eq!(
            eval_with_multiple_modules(0, &vec![Value::I32(-44), Value::I32(-33)]).unwrap(),
            vec![Value::I32(-77)]
        );
    }

    #[test]
    fn test_function_call_external() {
        // 测试 $ex_mul
        assert_eq!(
            eval_with_multiple_modules(1, &vec![Value::I32(2), Value::I32(3)]).unwrap(),
            vec![Value::I32(6)]
        );
        assert_eq!(
            eval_with_multiple_modules(1, &vec![Value::I32(4), Value::I32(-5)]).unwrap(),
            vec![Value::I32(-20)]
        );

        // 测试 $ex_div
        assert_eq!(
            eval_with_multiple_modules(2, &vec![Value::I32(8), Value::I32(2)]).unwrap(),
            vec![Value::I32(4)]
        );
        assert_eq!(
            eval_with_multiple_modules(2, &vec![Value::I32(-55), Value::I32(5)]).unwrap(),
            vec![Value::I32(-11)]
        );
    }

    #[test]
    fn test_function_call_re_export() {
        // 测试 $re_sub
        assert_eq!(
            eval_with_multiple_modules(3, &vec![Value::I32(8), Value::I32(6)]).unwrap(),
            vec![Value::I32(2)]
        );
        assert_eq!(
            eval_with_multiple_modules(3, &vec![Value::I32(6), Value::I32(-5)]).unwrap(),
            vec![Value::I32(11)]
        );

        // 测试 $re_mul
        assert_eq!(
            eval_with_multiple_modules(4, &vec![Value::I32(2), Value::I32(3)]).unwrap(),
            vec![Value::I32(6)]
        );
        assert_eq!(
            eval_with_multiple_modules(4, &vec![Value::I32(4), Value::I32(-5)]).unwrap(),
            vec![Value::I32(-20)]
        );

        // 测试 $re_div
        assert_eq!(
            eval_with_multiple_modules(5, &vec![Value::I32(8), Value::I32(2)]).unwrap(),
            vec![Value::I32(4)]
        );
        assert_eq!(
            eval_with_multiple_modules(5, &vec![Value::I32(-55), Value::I32(5)]).unwrap(),
            vec![Value::I32(-11)]
        );
    }

    #[test]
    fn test_function_call_indirect() {
        let module_name = "test-function-call-indirect.wasm";

        // 测试 add
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![0, 40, 5])).unwrap(),
            vec![Value::I32(45)]
        );
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![0, 5, 40])).unwrap(),
            vec![Value::I32(45)]
        );

        // 测试 sub
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![1, 40, 5])).unwrap(),
            vec![Value::I32(35)]
        );
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![1, 5, 40])).unwrap(),
            vec![Value::I32(-35)]
        );

        // 测试 mul
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![2, 40, 5])).unwrap(),
            vec![Value::I32(200)]
        );
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![2, 5, 40])).unwrap(),
            vec![Value::I32(200)]
        );

        // 测试 div
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![3, 40, 5])).unwrap(),
            vec![Value::I32(8)]
        );
        assert_eq!(
            eval(module_name, 0, &convert_i32_list(&vec![3, 5, 40])).unwrap(),
            vec![Value::I32(0)]
        );
    }

    #[test]
    fn test_block() {
        let module_name = "test-block.wasm";

        // 测试 return
        assert_eq!(eval(module_name, 0, &vec![]).unwrap(), vec![Value::I32(1)]);
        assert_eq!(eval(module_name, 1, &vec![]).unwrap(), vec![Value::I32(2)]);
        assert_eq!(eval(module_name, 2, &vec![]).unwrap(), vec![Value::I32(3)]);

        // 测试 br
        assert_eq!(eval(module_name, 3, &vec![]).unwrap(), vec![Value::I32(4)]);
        assert_eq!(eval(module_name, 4, &vec![]).unwrap(), vec![Value::I32(2)]);
        assert_eq!(eval(module_name, 5, &vec![]).unwrap(), vec![Value::I32(11)]);
        assert_eq!(eval(module_name, 6, &vec![]).unwrap(), vec![Value::I32(12)]);
        assert_eq!(eval(module_name, 7, &vec![]).unwrap(), vec![Value::I32(13)]);

        // 测试 br_if
        assert_eq!(eval(module_name, 8, &vec![]).unwrap(), vec![Value::I32(55)]);

        // 测试在结构块里访问函数的局部变量，以及返回值的类型等
        assert_eq!(
            eval(module_name, 9, &vec![Value::I32(55), Value::I32(66)]).unwrap(),
            vec![Value::I32(77)]
        );
    }

    #[test]
    fn test_block_if() {
        let module_name = "test-block-if.wasm";

        // 测试 if
        assert_eq!(eval(module_name, 0, &vec![]).unwrap(), vec![Value::I32(2)]);
        assert_eq!(eval(module_name, 1, &vec![]).unwrap(), vec![Value::I32(1)]);
    }

    #[test]
    fn test_block_branch_table() {
        let module_name = "test-block-branch-table.wasm";

        // 测试 br_table
        assert_eq!(
            eval(module_name, 0, &vec![Value::I32(0)]).unwrap(),
            vec![Value::I32(22)]
        );
        assert_eq!(
            eval(module_name, 0, &vec![Value::I32(1)]).unwrap(),
            vec![Value::I32(33)]
        );
        assert_eq!(
            eval(module_name, 0, &vec![Value::I32(2)]).unwrap(),
            vec![Value::I32(44)]
        );

        // 超出 br_table 范围
        assert_eq!(
            eval(module_name, 0, &vec![Value::I32(3)]).unwrap(),
            vec![Value::I32(55)]
        );
        assert_eq!(
            eval(module_name, 0, &vec![Value::I32(4)]).unwrap(),
            vec![Value::I32(55)]
        );
        assert_eq!(
            eval(module_name, 0, &vec![Value::I32(5)]).unwrap(),
            vec![Value::I32(55)]
        );
    }
}
