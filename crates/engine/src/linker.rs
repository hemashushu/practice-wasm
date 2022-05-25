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
use anvm_ast::{
    ast::{self, ExportDescriptor, GlobalType, ImportDescriptor, TypeItem},
    types::Value,
};

/// AST 模块的函数的指令序列位置信息
#[derive(Debug, PartialEq, Clone)]
pub enum FunctionLocation {
    Import {
        type_index: usize, // 导入项所期望的函数类型
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
    // 具体来说，因为一个模块里的导入函数（即对应的 FunctionLocation::Import）
    // - 既有可能是另外一个模块的函数，
    // - 也有可能是本地模块的本地函数，
    // - 还有可能是另外一个模块的导入函数再次导出的函数。
    //
    // 这一个步骤主要就是为了解析 FunctionLocation::Import 到最终的
    // FunctionItem::Native 和 FunctionItem::External。

    let module_names = get_module_names(native_modules, named_ast_modules);
    let native_module_count = native_modules.len();
    let mut function_items_list: Vec<Vec<FunctionItem>> = vec![];

    for (ast_module_index, function_locations) in function_locations_list.iter().enumerate() {
        let mut function_items: Vec<FunctionItem> = vec![];

        for function_location in function_locations {
            let function_item = match function_location {
                FunctionLocation::Import {
                    type_index, // 导入项所期望的函数类型
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
                            let target_function_index =
                                get_native_module_function_index_by_export_name(
                                    target_native_module,
                                    target_function_name,
                                )
                                .ok_or(
                                    EngineError::ObjectNotFound(format!(
                                        "cannot found the native function: {} in native module: {}",
                                        target_function_name, target_module_name
                                    )),
                                )?;

                            // 检查函数的实际类型个导入时声明的类型是否匹配
                            let target_type_index = target_native_module
                                .function_to_type_index_list[target_function_index];
                            let actual_function_type =
                                &target_native_module.function_types[target_type_index];

                            if expected_function_type != actual_function_type {
                                return Err(EngineError::InvalidOperation(
                                    "imported function type does not match".to_string(),
                                ));
                            }

                            let function_item = FunctionItem::Native {
                                native_module_index: target_native_module_index,
                                type_index: target_type_index,
                                function_index: target_function_index,
                            };

                            break function_item;
                        } else {
                            // 目标是 AST 模块的函数

                            let target_ast_module_index = target_module_index - native_module_count;
                            let target_ast_module =
                                &named_ast_modules[target_ast_module_index].module;

                            let target_function_index =
                                get_ast_module_function_index_by_export_name(
                                    target_ast_module,
                                    target_function_name,
                                )
                                .ok_or(
                                    EngineError::ObjectNotFound(format!(
                                        "cannot found the exported function: {} in module: {}",
                                        target_function_name, target_module_name
                                    )),
                                )?;

                            let target_function_location = &function_locations_list
                                [target_ast_module_index][target_function_index];

                            match target_function_location {
                                FunctionLocation::Import {
                                    type_index: _,
                                    module_name: another_module_name,
                                    function_name: another_function_name,
                                } => {
                                    // 目标函数是外部模块 "从外部导入然后再重新导出" 的函数，
                                    // 所需需要再解析一遍，直到目标函数是 "AST 模块的内部函数" 和 "本地函数模块的本地函数"
                                    // 这两者之中的一个为止。
                                    target_module_name = another_module_name;
                                    target_function_name = another_function_name;
                                }
                                FunctionLocation::Internal {
                                    internal_function_index,
                                    type_index: type_index_target_module,
                                    start_index,
                                    end_index,
                                } => {
                                    // 目标函数是外部模块的内部函数

                                    // 检查函数的实际类型跟导入时声明的类型是否匹配
                                    let actual_type_item =
                                        &target_ast_module.type_items[*type_index_target_module];

                                    if expected_type_item != actual_type_item {
                                        return Err(EngineError::InvalidOperation(
                                            "imported function type does not match".to_string(),
                                        ));
                                    }

                                    let function_item = FunctionItem::External {
                                        type_index: *type_index_target_module,
                                        vm_module_index: target_ast_module_index,
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
                let temp_item = FunctionLocation::Import {
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

    for (internal_function_index, type_index) in ast_module
        .internal_function_to_type_index_list
        .iter()
        .enumerate()
    {
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

fn get_native_module_function_index_by_export_name(
    native_modules: &NativeModule,
    name: &str,
) -> Option<usize> {
    native_modules.find_function_index_by_exported_name(name)
}

fn get_ast_module_function_index_by_export_name(
    ast_modules: &ast::Module,
    name: &str,
) -> Option<usize> {
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
/// - Vec<VMTable> 是虚拟机当中所有实例表的列表
/// - Vec<usize> 是每个 AST Module 对应的实例表的索引列表，
///   注：目前 WebAssembly 限制一个 Module 只能有一张表；
///   存在多个 Module 对应同一张表的情况。
pub fn link_tables(
    named_ast_modules: &[NamedAstModule],
) -> Result<(Vec<VMTable>, Vec<usize>), EngineError> {
    // "AST 模块 - 表格实例的索引" 的临时映射表，
    // 将元素的初始值设置为 None，以表示该项尚未设置。
    let mut module_to_table_index_list: Vec<Option<usize>> = vec![None; named_ast_modules.len()];

    // 所有实例表
    let mut instance_tables: Vec<VMTable> = vec![];

    // 先创建非导入的表
    for (ast_module_index, ast_module) in named_ast_modules
        .iter()
        .map(|item| &item.module)
        .enumerate()
    {
        // 先检查是否存在导入表
        let option_import_table_item = ast_module
            .import_items
            .iter()
            .find(|item| matches!(item.import_descriptor, ImportDescriptor::TableType(_)));

        if option_import_table_item == None {
            // 无导入表，创建新表

            let instance_table = if let Some(first) = ast_module.tables.first() {
                // 根据定义创建新表
                VMTable::new(first.clone())
            } else {
                // 创建默认表（容量最小值为 0，不限最大值的表）
                VMTable::new_by_min(0)
            };

            let instance_table_index = instance_tables.len();
            instance_tables.push(instance_table);

            module_to_table_index_list[ast_module_index] = Some(instance_table_index);
        }
    }

    // 解决导入表格
    for ast_module_index in 0..named_ast_modules.len() {
        if module_to_table_index_list[ast_module_index] == None {
            resolve_ast_module_table(
                named_ast_modules,
                &instance_tables,
                &mut module_to_table_index_list,
                ast_module_index,
            )?;
        }
    }

    // 转换临时映射表
    let list = module_to_table_index_list
        .iter()
        .map(|item| item.unwrap())
        .collect::<Vec<usize>>();

    Ok((instance_tables, list))
}

fn resolve_ast_module_table(
    named_ast_modules: &[NamedAstModule],
    instance_tables: &Vec<VMTable>,
    module_table_map: &mut Vec<Option<usize>>,
    ast_module_index: usize,
) -> Result<usize, EngineError> {
    let ast_module = &named_ast_modules[ast_module_index].module;

    let (target_module_name, target_export_item_name, target_table_type) = ast_module
        .import_items
        .iter()
        .find_map(|item| {
            if let ImportDescriptor::TableType(table_type) = &item.import_descriptor {
                Some((&item.module_name, &item.item_name, table_type))
            } else {
                None
            }
        })
        .expect("unreachable"); // 仅当 AST Module 声明了一个导入表格才会来到这里，所以不存在找不到导入项的情况

    let (target_ast_module_index, target_ast_module) = named_ast_modules
        .iter()
        .enumerate()
        .find(|(_index, item)| &item.name == target_module_name)
        .map(|(index, item)| (index, &item.module))
        .ok_or(EngineError::ObjectNotFound(format!(
            "can not found the module: {}",
            target_module_name
        )))?;

    let target_table_index = target_ast_module
        .export_items
        .iter()
        .find_map(|item| match item.export_descriptor {
            ExportDescriptor::TableIndex(table_index) if &item.name == target_export_item_name => {
                Some(table_index)
            }
            _ => None,
        })
        .ok_or(EngineError::ObjectNotFound(format!(
            "can not found the export table: {}.{}",
            target_module_name, target_export_item_name
        )))?;

    if target_table_index != 0 {
        return Err(EngineError::InvalidOperation(
            "only one table is allowed for a module".to_string(),
        ));
    }

    let option_target_instance_table_index = module_table_map[target_ast_module_index];

    let target_instance_table_index = if let Some(index) = option_target_instance_table_index {
        index
    } else {
        // 目标表实例是模块导入再次导出的，
        // 需要再次解析一次，直到找到真正的表实例为止
        resolve_ast_module_table(
            named_ast_modules,
            instance_tables,
            module_table_map,
            target_ast_module_index,
        )?
    };

    // 检查表格类型
    let instance_table = &instance_tables[target_instance_table_index];

    if instance_table.get_table_type() != target_table_type {
        return Err(EngineError::InvalidOperation(
            "imported table type mismatch".to_string(),
        ));
    }

    // 更新映射表
    module_table_map[ast_module_index] = Some(target_instance_table_index);

    Ok(target_ast_module_index)
}

/// 解决模块间的内存块链接，并创建相应的内存块对象。
///
/// 注，对于没有指定内存信息的模块，将会创建一个
/// 最小值为 0 的内存块对象
///
/// 返回值当中
/// - Vec<VMMemory> 是虚拟机当中所有内存块实例的列表
/// - Vec<usize> 是每个 AST Module 对应的内存块实例的索引列表，
///   注：目前 WebAssembly 限制一个 Module 只能有一个内存块；
///   存在多个 Module 对应同一个内存块的情况。
pub fn link_memorys(
    named_ast_modules: &[NamedAstModule],
) -> Result<(Vec<VMMemory>, Vec<usize>), EngineError> {
    // "AST 模块 - 内存块实例的索引" 的临时映射表，
    // 将元素的初始值设置为 None，以表示该项尚未设置。
    let mut module_to_memory_block_index_list: Vec<Option<usize>> =
        vec![None; named_ast_modules.len()];

    // 所有实例表
    let mut instance_memory_blocks: Vec<VMMemory> = vec![];

    // 先创建非导入的内存块实例
    for (ast_module_index, ast_module) in named_ast_modules
        .iter()
        .map(|item| &item.module)
        .enumerate()
    {
        // 先检查是否存在导入内存块
        let option_import_memory_item = ast_module
            .import_items
            .iter()
            .find(|item| matches!(item.import_descriptor, ImportDescriptor::MemoryType(_)));

        if option_import_memory_item == None {
            // 无导入内存块，创建新内存块

            let instance_memory = if let Some(first) = ast_module.memory_blocks.first() {
                // 根据定义创建新内存块
                VMMemory::new(first.clone())
            } else {
                // 创建默认内存块（容量最小值为 0，不限最大值的内存块）
                VMMemory::new_by_min_page(0)
            };

            let instance_memory_block_index = instance_memory_blocks.len();
            instance_memory_blocks.push(instance_memory);

            module_to_memory_block_index_list[ast_module_index] = Some(instance_memory_block_index);
        }
    }

    // 解决导入内存块
    for ast_module_index in 0..named_ast_modules.len() {
        if module_to_memory_block_index_list[ast_module_index] == None {
            resolve_ast_module_memory_block(
                named_ast_modules,
                &instance_memory_blocks,
                &mut module_to_memory_block_index_list,
                ast_module_index,
            )?;
        }
    }

    // 转换临时映射表
    let list = module_to_memory_block_index_list
        .iter()
        .map(|item| item.unwrap())
        .collect::<Vec<usize>>();

    Ok((instance_memory_blocks, list))
}

fn resolve_ast_module_memory_block(
    named_ast_modules: &[NamedAstModule],
    instance_memory_blocks: &Vec<VMMemory>,
    module_memory_block_map: &mut Vec<Option<usize>>,
    ast_module_index: usize,
) -> Result<usize, EngineError> {
    let ast_module = &named_ast_modules[ast_module_index].module;

    let (target_module_name, target_export_item_name, target_memory_type) = ast_module
        .import_items
        .iter()
        .find_map(|item| {
            if let ImportDescriptor::MemoryType(memory_type) = &item.import_descriptor {
                Some((&item.module_name, &item.item_name, memory_type))
            } else {
                None
            }
        })
        .expect("unreachable"); // 仅当 AST Module 声明了一个导入内存块才会来到这里，所以不存在找不到导入项的情况

    let (target_ast_module_index, target_ast_module) = named_ast_modules
        .iter()
        .enumerate()
        .find(|(_index, item)| &item.name == target_module_name)
        .map(|(index, item)| (index, &item.module))
        .ok_or(EngineError::ObjectNotFound(format!(
            "can not found the module: {}",
            target_module_name
        )))?;

    let target_memory_block_index = target_ast_module
        .export_items
        .iter()
        .find_map(|item| match item.export_descriptor {
            ExportDescriptor::MemoryBlockIndex(memory_block_index)
                if &item.name == target_export_item_name =>
            {
                Some(memory_block_index)
            }
            _ => None,
        })
        .ok_or(EngineError::ObjectNotFound(format!(
            "can not found the export memory: {}.{}",
            target_module_name, target_export_item_name
        )))?;

    if target_memory_block_index != 0 {
        return Err(EngineError::InvalidOperation(
            "only one memory block is allowed for a module".to_string(),
        ));
    }

    let option_target_instance_memory_block_index =
        module_memory_block_map[target_ast_module_index];

    let target_instance_memory_block_index =
        if let Some(index) = option_target_instance_memory_block_index {
            index
        } else {
            // 目标内存块实例是模块导入再次导出的，
            // 需要再次解析一次，直到找到真正的内存块实例为止
            resolve_ast_module_memory_block(
                named_ast_modules,
                instance_memory_blocks,
                module_memory_block_map,
                target_ast_module_index,
            )?
        };

    // 检查表格类型
    let instance_memory_block = &instance_memory_blocks[target_instance_memory_block_index];

    if instance_memory_block.get_memory_type() != target_memory_type {
        return Err(EngineError::InvalidOperation(
            "imported memory type mismatch".to_string(),
        ));
    }

    // 更新映射表
    module_memory_block_map[ast_module_index] = Some(target_instance_memory_block_index);

    Ok(target_ast_module_index)
}

/// 解决模块间的全局变量链接
///
/// 返回值当中
/// - Vec<VMGlobalVariable> 是虚拟机当中所有全局变量实例的列表
/// - Vec<Vec<usize>> 是每个 AST Module 对应的全局变量实例的索引列表
///   注：一个 Module 可以有多个全局变量
pub fn link_global_variables(
    // native_modules: &[NativeModule],
    named_ast_modules: &[NamedAstModule],
    // interpreter: &Interpreter,
) -> Result<(Vec<VMGlobalVariable>, Vec<Vec<usize>>), EngineError> {
    // "AST 模块 - 全局变量实例的索引" 的临时映射表
    let mut module_to_global_variables_list: Vec<Vec<Option<usize>>> = vec![];

    // 所有实例表
    let mut instance_global_variables: Vec<VMGlobalVariable> = vec![];

    for ast_module in named_ast_modules.iter().map(|item| &item.module) {
        let mut module_global_variable_map_item: Vec<Option<usize>> = vec![];

        // 先以 None 为值，填充模块的导入全局变量
        let import_global_variable_count = ast_module
            .import_items
            .iter()
            .filter(|item| matches!(item.import_descriptor, ImportDescriptor::GlobalType(_)))
            .count();

        for _ in 0..import_global_variable_count {
            module_global_variable_map_item.push(None);
        }

        // 再创建模块内定义的所有全局变量
        for global_item in &ast_module.global_items {
            let global_type = global_item.global_type.clone();
            // todo 执行 global_item.initialize_instruction_items
            let value = Value::I32(0);
            let instance_global_variable = VMGlobalVariable::new(global_type, value);

            // 创建全局变量实例
            let instance_global_variable_index = instance_global_variables.len();
            instance_global_variables.push(instance_global_variable);

            module_global_variable_map_item.push(Some(instance_global_variable_index));
        }

        module_to_global_variables_list.push(module_global_variable_map_item);
    }

    // 解决导入全局变量
    for ast_module_index in 0..named_ast_modules.len() {
        let module_global_variable_count = {
            let module_global_variable_map_item =
                &module_to_global_variables_list[ast_module_index];
            module_global_variable_map_item.len()
        };

        for module_global_variable_index in 0..module_global_variable_count {
            let is_none = {
                let module_global_variable_map_item =
                    &module_to_global_variables_list[ast_module_index];
                module_global_variable_map_item[module_global_variable_index] == None
            };
            if is_none {
                resolve_ast_module_global_variable(
                    named_ast_modules,
                    &instance_global_variables,
                    &mut module_to_global_variables_list,
                    ast_module_index,
                    module_global_variable_index,
                )?;
            }
        }
    }

    // 转换临时映射表
    let list = module_to_global_variables_list
        .iter()
        .map(|item| {
            item.iter()
                .map(|sub_item| sub_item.unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    Ok((instance_global_variables, list))
}

fn resolve_ast_module_global_variable(
    named_ast_modules: &[NamedAstModule],
    instance_global_variables: &Vec<VMGlobalVariable>,
    module_global_variable_map: &mut Vec<Vec<Option<usize>>>,
    ast_module_index: usize,
    module_global_variable_index: usize,
) -> Result<usize, EngineError> {
    let ast_module = &named_ast_modules[ast_module_index].module;

    let (target_module_name, target_export_item_name, target_global_type) = ast_module
        .import_items
        .iter()
        .filter_map(|item| {
            if let ImportDescriptor::GlobalType(global_type) = &item.import_descriptor {
                Some((&item.module_name, &item.item_name, global_type))
            } else {
                None
            }
        })
        .collect::<Vec<(&String, &String, &GlobalType)>>()[module_global_variable_index];

    let (target_ast_module_index, target_ast_module) = named_ast_modules
        .iter()
        .enumerate()
        .find(|(_index, item)| &item.name == target_module_name)
        .map(|(index, item)| (index, &item.module))
        .ok_or(EngineError::ObjectNotFound(format!(
            "can not found the module: {}",
            target_module_name
        )))?;

    let target_module_global_variable_index = target_ast_module
        .export_items
        .iter()
        .find_map(|item| match item.export_descriptor {
            ExportDescriptor::GlobalItemIndex(global_variable_index)
                if &item.name == target_export_item_name =>
            {
                Some(global_variable_index as usize)
            }
            _ => None,
        })
        .ok_or(EngineError::ObjectNotFound(format!(
            "can not found the export global variable: {}.{}",
            target_module_name, target_export_item_name
        )))?;

    let option_target_instance_global_variable_index =
        module_global_variable_map[target_ast_module_index][target_module_global_variable_index];

    let target_instance_global_variable_index =
        if let Some(index) = option_target_instance_global_variable_index {
            index
        } else {
            // 目标全局变量实例是模块导入再次导出的，
            // 需要再次解析一次，直到找到真正的全局变量实例为止
            resolve_ast_module_global_variable(
                named_ast_modules,
                instance_global_variables,
                module_global_variable_map,
                target_ast_module_index,
                target_module_global_variable_index,
            )?
        };

    // 检查表格类型
    let instance_global_variable =
        &instance_global_variables[target_instance_global_variable_index];

    if instance_global_variable.get_global_type() != target_global_type {
        return Err(EngineError::InvalidOperation(
            "imported global variable type mismatch".to_string(),
        ));
    }

    // 更新映射表
    module_global_variable_map[ast_module_index][module_global_variable_index] =
        Some(target_instance_global_variable_index);

    Ok(target_ast_module_index)
}
