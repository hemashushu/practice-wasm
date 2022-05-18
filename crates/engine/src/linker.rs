// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    error::EngineError,
    native_module::NativeModule,
    object::{FunctionItem, NamedAstModule},
    vm_global_variable::VMGlobalVariable,
    vm_memory::VMMemory,
    vm_table::VMTable,
};
use anvm_ast::ast::{self, ImportDescriptor, ImportItem, TypeItem};

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionLocation {
    External {
        type_index: usize,
        module_name: String,
        function_name: String,
    },
    Internal {
        internal_function_index: usize,
        type_index: usize,
        start_index: usize,
        end_index: usize, // 函数 `end 指令` 所在的位置
    },
}

/// 解决模块间的函数 "导出和导入" 的链接
///
/// 返回各个 AST Module 对应的函数信息列表。
pub fn link_functions(
    native_modules: &[NativeModule],
    named_ast_modules: &[NamedAstModule],
) -> Result<Vec<Vec<FunctionItem>>, EngineError> {
    // 第 1 步：
    // - 获取每个外部函数的模块名称和函数名称
    // - 获取每个内部函数指令序列的开始和结束位置
    // - 合并以上两项信息，得到每个模块的函数位置信息列表

    // function_locations_list 仅包含 AST 模块的函数位置信息，
    // 包括导入函数和模块内部函数。
    // 不包括本地函数（native function）模块的函数信息。
    let mut function_locations_list: Vec<Vec<FunctionLocation>> = vec![];

    for named_ast_module in named_ast_modules {
        // 注：内部函数的索引值并非总是从 0 开始，当一个模块有
        // 导入的函数时，索引值优先从导入函数开始计算，所以第一个内部函数的索引值
        // 等于导入函数的数量。
        let import_function_locations =
            get_ast_module_import_function_locations(&named_ast_module.module);
        let internal_function_locations =
            get_ast_module_internal_function_locations(&named_ast_module.module);

        let mut function_locations: Vec<FunctionLocation> = vec![];
        function_locations.extend_from_slice(&import_function_locations);
        function_locations.extend_from_slice(&internal_function_locations);

        function_locations_list.push(function_locations);
    }

    // 第 2 步：
    // 将 FunctionLocation 转换为 FunctionItem
    // 具体来说，因为一个模块里的导入函数（即对应的 FunctionLocation::External）
    // - 既有可能是另外一个模块的函数，
    // - 也有可能是本地模块的本地函数，
    // - 还有可能是另外一个模块的导入函数再次导出的函数。
    //
    // 这一个步骤主要就是为了解析 FunctionLocation::External 到最终的
    // FunctionItem::Native 和 FunctionItem::External。

    let module_names = get_module_names(native_modules, named_ast_modules);
    let native_module_count = native_modules.len();
    let mut function_items_list: Vec<Vec<FunctionItem>> = vec![];

    for (ast_module_index, function_locations) in function_locations_list.iter().enumerate() {
        let mut function_items: Vec<FunctionItem> = vec![];

        for function_location in function_locations {
            let function_item = match function_location {
                FunctionLocation::External {
                    type_index,
                    module_name,
                    function_name,
                } => {
                    let expected_type_item = &named_ast_modules[ast_module_index].module.type_items
                        [*type_index as usize];
                    let TypeItem::FunctionType(expected_function_type) = expected_type_item;

                    let mut target_module_name = module_name;
                    let mut target_function_name = function_name;

                    loop {
                        let target_module_index =
                            get_module_index_by_name(&module_names, target_module_name).ok_or(
                                EngineError::ObjectNotFound(format!(
                                    "cannot found the module: {}",
                                    target_module_name
                                )),
                            )?;

                        if target_module_index < native_module_count {
                            // 目标是本地函数模块的本地函数
                            let target_native_module_index = target_module_index;
                            let target_native_module = &native_modules[target_native_module_index];
                            let target_function_index = get_native_module_function_index_by_name(
                                target_native_module,
                                target_function_name,
                            )
                            .ok_or(EngineError::ObjectNotFound(format!(
                                "cannot found the native function: {} in native module: {}",
                                target_function_name, target_module_name
                            )))?;

                            // 检查函数的实际类型个导入时声明的类型是否匹配
                            let actual_function_type =
                                &target_native_module.function_types[*type_index as usize];

                            if expected_function_type != actual_function_type {
                                return Err(EngineError::InvalidOperation(
                                    "imported function type does not match".to_string(),
                                ));
                            }

                            let function_item = FunctionItem::Native {
                                type_index: *type_index,
                                native_module_index: target_native_module_index,
                                function_index: target_function_index,
                            };

                            break function_item;
                        } else {
                            // 目标是 AST 模块的函数

                            let target_ast_module_index = target_module_index - native_module_count;
                            let target_ast_module =
                                &named_ast_modules[target_ast_module_index].module;

                            let target_function_index = get_ast_module_function_index_by_name(
                                target_ast_module,
                                target_function_name,
                            )
                            .ok_or(EngineError::ObjectNotFound(format!(
                                "cannot found the exported function: {} in module: {}",
                                target_function_name, target_module_name
                            )))?;

                            let target_function_location = &function_locations_list
                                [target_ast_module_index][target_function_index];

                            match target_function_location {
                                FunctionLocation::External {
                                    type_index,
                                    module_name,
                                    function_name,
                                } => {
                                    // 目标函数是外部模块 "从外部导入然后再重新导出" 的函数，
                                    // 所需需要再解析一遍，直到目标函数是 "AST 模块的内部函数" 和 "本地函数模块的本地函数"
                                    // 这两者之中的一个为止。
                                    target_module_name = module_name;
                                    target_function_name = function_name;
                                }
                                FunctionLocation::Internal {
                                    internal_function_index,
                                    type_index,
                                    start_index,
                                    end_index,
                                } => {
                                    // 目标函数是外部模块的内部函数

                                    // 检查函数的实际类型个导入时声明的类型是否匹配
                                    let actual_type_item =
                                        &target_ast_module.type_items[*type_index];

                                    if expected_type_item != actual_type_item {
                                        return Err(EngineError::InvalidOperation(
                                            "imported function type does not match".to_string(),
                                        ));
                                    }

                                    let function_item = FunctionItem::External {
                                        type_index: *type_index,
                                        ast_module_index: target_ast_module_index,
                                        function_index: target_function_index,
                                        internal_function_index: *internal_function_index,
                                        start_index: *start_index,
                                        end_index: *end_index,
                                    };
                                    break function_item;
                                }
                            }
                        }
                    }
                }
                FunctionLocation::Internal {
                    internal_function_index,
                    type_index,
                    start_index,
                    end_index,
                } => FunctionItem::Internal {
                    type_index: *type_index,
                    internal_function_index: *internal_function_index,
                    start_index: *start_index,
                    end_index: *end_index,
                },
            };

            function_items.push(function_item);
        }

        function_items_list.push(function_items);
    }

    Ok(function_items_list)
}

fn get_ast_module_import_function_locations(ast_module: &ast::Module) -> Vec<FunctionLocation> {
    ast_module
        .import_items
        .iter()
        .filter_map(|item| {
            if let ast::ImportDescriptor::FunctionTypeIndex(type_index) = item.import_descriptor {
                let temp_item = FunctionLocation::External {
                    type_index: type_index as usize,
                    module_name: item.module_name.clone(),
                    function_name: item.item_name.clone(),
                };
                Some(temp_item)
            } else {
                None
            }
        })
        .collect::<Vec<FunctionLocation>>()
}

fn get_ast_module_internal_function_locations(ast_module: &ast::Module) -> Vec<FunctionLocation> {
    let mut function_addr_offset: usize = 0;
    let mut function_locations: Vec<FunctionLocation> = vec![];

    for (internal_function_index, type_index) in ast_module.function_list.iter().enumerate() {
        let instruction_count = ast_module.code_items[internal_function_index]
            .instruction_items
            .len();
        let function_location = FunctionLocation::Internal {
            internal_function_index,
            type_index: *type_index as usize,
            start_index: function_addr_offset,
            end_index: function_addr_offset + instruction_count - 1,
        };
        function_locations.push(function_location);

        // 递增函数开始位置的偏移值
        // 因为同一个模块里的所有内部函数的指令序列将会被合并
        function_addr_offset += instruction_count;
    }

    function_locations
}

fn get_module_names(
    native_modules: &[NativeModule],
    named_ast_modules: &[NamedAstModule],
) -> Vec<String> {
    let native_module_names = native_modules
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<String>>();
    let ast_module_names = named_ast_modules
        .iter()
        .map(|m| m.name.clone())
        .collect::<Vec<String>>();
    let mut module_names: Vec<String> = vec![];

    module_names.extend_from_slice(&native_module_names);
    module_names.extend_from_slice(&ast_module_names);
    module_names
}

fn get_module_index_by_name(module_names: &[String], name: &str) -> Option<usize> {
    module_names
        .iter()
        .enumerate()
        .find(|(_, module_name)| *module_name == name)
        .map(|(index, _)| index)
}

fn get_native_module_function_index_by_name(
    native_modules: &NativeModule,
    name: &str,
) -> Option<usize> {
    native_modules
        .function_items
        .iter()
        .enumerate()
        .find(|(_, item)| item.name == name)
        .map(|(index, _)| index)
}

fn get_ast_module_function_index_by_name(ast_modules: &ast::Module, name: &str) -> Option<usize> {
    ast_modules.export_items.iter().find_map(|item| {
        if item.name == name {
            if let ast::ExportDescriptor::FunctionIndex(function_index) = item.export_descriptor {
                Some(function_index as usize)
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// 解决模块间的表链接，并创建相应的表对象。
///
/// 注，对于没有指定表信息的模块，将会创建一个
/// 最小值为 0 的表对象
///
/// 返回值当中
/// - Vec<VMTable> 是虚拟机实例当中所有表的列表
/// - Vec<usize> 是每个 AST Module 对应的表的索引列表，
///   注：目前 WebAssembly 限制一个 Module 只能有一张表；
///   存在多个 Module 对应同一张表的情况。
pub fn link_tables(
    native_modules: &[NativeModule],
    named_ast_modules: &[NamedAstModule],
    // interpreter: &Interpreter,
) -> Result<(Vec<usize>, Vec<VMTable>), EngineError> {
    let module_count = named_ast_modules.len();

    // 将列表元素的初始值设置为 usize::MAX
    // 以表示该项尚未设置
    let mut module_table_map: Vec<usize> = vec![usize::MAX, module_count];

    let mut tables: Vec<VMTable> = vec![];

    // 先创建非导入的表
    for (ast_module_index, ast_module) in named_ast_modules
        .iter()
        .map(|item| &item.module)
        .enumerate()
    {
        // 先检查是否存在导入项
        let option_import_item = ast_module
            .import_items
            .iter()
            .find(|item| matches!(item.import_descriptor, ImportDescriptor::TableType(_)));

        if option_import_item == None {
            // 无表类型的导入项，创建新表

            let table = if let Some(first) = ast_module.tables.first() {
                // 根据定义创建新表
                VMTable::new(first.clone())
            } else {
                // 创建默认表（容量最小值为 0，不限最大值的表）
                VMTable::new_by_min(0)
            };

            let table_index = tables.len();
            tables.push(table);

            module_table_map[ast_module_index] = table_index;
        }
    }

    // 解决导入表格
    for (ast_module_index, ast_module) in named_ast_modules
        .iter()
        .map(|item| &item.module)
        .enumerate()
    {
        if module_table_map[ast_module_index] != usize::MAX {
            continue;
        }

        let (import_module_name, import_item_name, import_table_type) = ast_module
            .import_items
            .iter()
            .find_map(|item| {
                if let ImportDescriptor::TableType(table_type) = &item.import_descriptor {
                    Some((&item.module_name, &item.item_name, table_type))
                } else {
                    None
                }
            })
            .unwrap();

        let mut target_module_name = import_module_name;
        let mut target_item_name = import_item_name;
        let mut target_table_type = import_table_type;
    }
    todo!()
}

/// 解决模块间的内存块链接，并创建相应的内存块对象。
///
/// 注，对于没有指定内存信息的模块，将会创建一个
/// 最小值为 0 的内存块对象
///
/// 返回值当中
/// - Vec<VMMemory> 是虚拟机实例当中所有内存块的列表
/// - Vec<usize> 是每个 AST Module 对应的内存块的索引列表，
///   注：目前 WebAssembly 限制一个 Module 只能有一个内存块；
///   存在多个 Module 对应同一个内存块的情况。
pub fn link_memorys(
    native_modules: &[NativeModule],
    named_ast_modules: &[NamedAstModule],
    // interpreter: &Interpreter,
) -> Result<(Vec<usize>, Vec<VMMemory>), EngineError> {
    todo!()
}

/// 解决模块间的全局变量链接
///
/// 返回值当中
/// - Vec<VMGlobalVariable> 是虚拟机实例当中所有全局变量的列表
/// - Vec<Vec<usize>> 是每个 AST Module 对应的全局变量的索引列表
pub fn link_global_variables(
    native_modules: &[NativeModule],
    named_ast_modules: &[NamedAstModule],
    // interpreter: &Interpreter,
) -> Result<(Vec<Vec<usize>>, Vec<VMGlobalVariable>), EngineError> {
    todo!()
}
