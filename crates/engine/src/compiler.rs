// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! 将一个模块的所有函数的指令序列合并成一个序列，
//! 并且将诸如 `call`、`br`、`if`、`else` 等指令转换为相应的跳转指令。
//!
//! # 转换规则
//!
//! 大部分指令都不需要转换，仅对流程控制（分支）和函数调用等指令需要转换为 `控制指令`：
//!
//! - `block 指令` 转换为 `block 控制指令`；
//! - `loop 指令` 转换为 `block 控制指令`；
//! - `if 指令` 转换为 `block_jump_eq_zero 控制指令`，跳转目标为原 `else` 指令所在的位置；
//!   注，原始的 `if 指令` 其实是一个 `block 控制指令` 和一个 `jump_eqz 控制指令` 的语法糖，
//!   不过为了让转换后的 `指令序列` 跟原始的的位置（即索引）一一对于（以便于追踪和调试），所以
//!   新增加一个专门跟 `if 指令` 对应的 `block_jump_eq_zero 控制指令`；
//! - `else 指令` 转换为 `jump 控制指令`，跳转目标为 if 结构块当中 `end 指令` 所在的位置；
//! - `br 指令` 转换为 `jump_out 控制指令`，跳转目标由目标 block 类型所决定，即
//!   * 对于原 block/if 结构块，跳转目标为原始结构块的 `end 指令` 所在的位置，
//!   * 对于原 loop 结构块，跳转目标为原始结构块的开始位置；
//! - `br_if 指令` 转换为 `jump_out_not_eq_zero 控制指令`；
//! - `return 指令` 转换为 `jump_out 控制指令`，跳转目标为函数的最后一条指令（即 `end 指令`）所在的位置；
//! - `call 指令`：
//!   * 对于目标为模块内的函数，转为 `call_internal 控制指令`；
//!   * 对于目标为模块外的函数，转为 `call_external 控制指令`；
//!   * 对于目标为本地的函数（native function），转为 `call_native 控制指令`；
//!
//! 注：当前假设 br, br_if, return 等指令不允许出现在函数首层
//!
//! # 控制指令列表
//!
//! - block (block_type)
//! - block_jump_eq_zero (block_type, alternate_addr)
//! - jump (addr)
//! - jump_out (relative_depth, addr)
//! - jump_out_not_eq_zero (relative_depth, addr)
//! - call_internal (type_index, function_index, addr)
//! - call_external (module_index, type_index, function_index, addr)
//! - call_native (module_index, type_index, function_index)

use anvm_ast::{
    ast::{self, CodeItem},
    instruction,
};

use crate::{
    error::EngineError,
    native_module::NativeModule,
    object::{Control, FunctionItem, Instruction},
};

#[derive(Debug, PartialEq, Clone)]
pub enum FunctionLocation {
    External {
        type_index: usize,
        module_name: String,
        function_name: String,
    },
    Internal {
        type_index: usize,
        start_index: usize,
        end_index: usize, // 函数 `end 指令` 所在的位置
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamedAstModule {
    name: String,
    module: ast::Module,
}

impl NamedAstModule {
    pub fn new(name: &str, module: ast::Module) -> Self {
        Self {
            name: name.to_string(),
            module,
        }
    }
}

/// 将 AST 模块当中的函数指令序列编译为虚拟机能直接解析运行的指令
///
/// 具体来说，原始指令（即 WebAssembly 指令）当中部分指令包含有一定的流程控制逻辑，
/// 比如 `br 指令` 会根据目标结构块的类型来决定跳转目标（位置），又比如 `if 指令`
/// 实际上是两个跳转指令共同完成。
/// 这些工作均可以在开始执行程序之前进行分析和计算，减少虚拟机在执行过程中的逻辑
/// 分析和计算，以提高运行的效率。
///
/// 实际上还可以将所有模块的所有函数的指令序列编译为一个超大的指令序列，这样可以
/// 提高模块间调用的处理效率。
/// 不过为了便于追踪和调试，当前编译模块仅对一个模块里的所有函数指令序列进行合并，
/// 而不会合并所有模块。
pub fn compile(
    native_modules: Vec<NativeModule>,
    named_ast_modules: Vec<NamedAstModule>,
) -> Result<Vec<Vec<Instruction>>, EngineError> {
    // 第 1 步：
    // - 获取每个外部函数的模块名称和函数名称
    // - 获取每个内部函数指令序列的开始和结束位置
    // - 合并以上两项信息，得到每个模块的函数位置信息列表

    // function_locations_list 仅包含 AST 模块的函数位置信息，
    // 包括导入函数和模块内部函数。
    // 不包括本地函数（native function）模块的函数信息。
    let mut function_locations_list: Vec<Vec<FunctionLocation>> = vec![];

    for named_ast_module in &named_ast_modules {
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
    // 具体来说，因为一个模块里的导入函数（即对应的 FunctionLocation::External）即有可能
    // 是另外一个模块的函数，也可能是另外一个模块的导入函数再次导出的函数，
    // 还有可能是本地模块的本地函数。
    // 这一个步骤主要就是为了解析 FunctionLocation::External 到
    // FunctionItem::Native 和 FunctionItem::External。

    let module_names = get_module_names(&native_modules, &named_ast_modules);
    let native_module_count = native_modules.len();
    let mut function_items_list: Vec<Vec<FunctionItem>> = vec![];

    for function_locations in &function_locations_list {
        let mut function_items: Vec<FunctionItem> = vec![];

        for function_location in function_locations {
            let function_item = match function_location {
                FunctionLocation::External {
                    type_index,
                    module_name,
                    function_name,
                } => {
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
                                    type_index,
                                    start_index,
                                    end_index,
                                } => {
                                    // 目标函数是外部模块的内部函数
                                    let function_item = FunctionItem::External {
                                        type_index: *type_index,
                                        ast_module_index: target_ast_module_index,
                                        function_index: target_function_index,
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
                    type_index,
                    start_index,
                    end_index,
                } => FunctionItem::Internal {
                    type_index: *type_index,
                    start_index: *start_index,
                    end_index: *end_index,
                },
            };

            function_items.push(function_item);
        }

        function_items_list.push(function_items);
    }

    // 第 3 步：
    // - 将 AST 的 Instruction 转换为虚拟机可直接解析运行的 Instruction
    // - 合并一个模块里的所有内部函数的指令序列

    let mut instructions_list: Vec<Vec<Instruction>> = vec![];

    for (ast_module_index, ast_module) in named_ast_modules.iter().enumerate() {
        // 转换一个模块

        let mut instructions: Vec<Instruction> = vec![];
        let mut function_addr_offset: usize = 0;

        for code_item in &ast_module.module.code_items {
            // 转换一个函数

            // 先对一个函数的指令序列扫描一次，以获取内部结构块信息（主要是为了获取 else 和 end 指令的位置）
            let block_locations = get_function_block_locations(code_item);
            let mut block_index_stack: Vec<usize> = vec![];

            let original_instructions = &code_item.instruction_items;
            for original_instruction in original_instructions {
                let instruction = match original_instruction {
                    instruction::Instruction::Block(block_type, block_index) => {
                        block_index_stack.push(*block_index as usize);
                        Instruction::Control(Control::Block(block_type.to_owned()))
                    }
                    instruction::Instruction::Loop(block_type, block_index) => {
                        block_index_stack.push(*block_index as usize);
                        Instruction::Control(Control::Block(block_type.to_owned()))
                    }
                    instruction::Instruction::If(block_type, block_index) => {
                        block_index_stack.push(*block_index as usize);

                        // 获取 if 结构块当中的 `else 指令` 所在的位置
                        let else_index = block_locations[*block_index as usize].middle_index;
                        Instruction::Control(Control::BlockJumpEqZero(
                            block_type.to_owned(),
                            function_addr_offset + else_index,
                        ))
                    }
                    instruction::Instruction::Else => {
                        let block_index = block_index_stack.last().unwrap();
                        // 获取 if 结构块当中的 `end 指令` 所在的位置
                        let end_index = block_locations[*block_index as usize].end_index;
                        Instruction::Control(Control::Jump(function_addr_offset + end_index))
                    }
                    instruction::Instruction::Br(relative_depth) => {
                        // 注：当前假设 br 指令不允许出现在函数首层

                        // target_depth 为目标层的层级，函数本层的层级为 0，第一层 block 的层级为 1，比如
                        //
                        // function
                        //  |         <--- level 0
                        //  |-- block
                        //  |   |     <--- level 1
                        //  |-- end
                        //
                        let target_depth = block_index_stack.len() - *relative_depth as usize;

                        let target_addr = if target_depth == 0 {
                            // 跳到函数本层了
                            // 目标位置应该是函数的最后一个指令，即 `end 指令` 所在的位置
                            original_instructions.len() - 1
                        } else {
                            let target_block_index = block_index_stack[target_depth - 1];

                            // 获取目标层的位置信息
                            let block_location = &block_locations[target_block_index];

                            if block_location.block_structure_type == BlockStructureType::Loop {
                                block_location.start_index
                            } else {
                                block_location.end_index
                            }
                        };

                        Instruction::Control(Control::JumpOut(
                            *relative_depth as usize,
                            function_addr_offset + target_addr,
                        ))
                    }
                    instruction::Instruction::BrIf(relative_depth) => {
                        // 注：当前假设 br_if 指令不允许出现在函数首层
                        let target_depth = block_index_stack.len() - *relative_depth as usize;

                        let target_addr = if target_depth == 0 {
                            // 跳到函数本层了
                            // 目标位置应该是函数的最后一个指令，即 `end 指令` 所在的位置
                            original_instructions.len() - 1
                        } else {
                            let target_block_index = block_index_stack[target_depth - 1];

                            // 获取目标层的位置信息
                            let block_location = &block_locations[target_block_index];

                            if block_location.block_structure_type == BlockStructureType::Loop {
                                block_location.start_index
                            } else {
                                block_location.end_index
                            }
                        };

                        Instruction::Control(Control::JumpOutNotEqZero(
                            *relative_depth as usize,
                            function_addr_offset + target_addr,
                        ))
                    }
                    instruction::Instruction::Return => {
                        // 注：当前假设 return 指令不允许出现在函数首层
                        let relative_depth = block_index_stack.len() - 1;
                        let end_index = original_instructions.len() - 1;

                        Instruction::Control(Control::JumpOut(
                            relative_depth,
                            function_addr_offset + end_index,
                        ))
                    }
                    instruction::Instruction::Call(function_index) => {
                        // 获取函数的位置信息
                        let function_item =
                            &function_items_list[ast_module_index][*function_index as usize];

                        match function_item {
                            FunctionItem::Internal {
                                type_index,
                                start_index,
                                end_index,
                            } => Instruction::Control(Control::CallInternal(
                                *type_index,
                                *function_index as usize,
                                *start_index,
                            )),
                            FunctionItem::External {
                                type_index,
                                ast_module_index: vm_module_index,
                                function_index,
                                start_index,
                                end_index,
                            } => Instruction::Control(Control::CallExternal(
                                *vm_module_index,
                                *type_index,
                                *function_index,
                                *start_index,
                            )),
                            FunctionItem::Native {
                                type_index,
                                native_module_index,
                                function_index,
                            } => Instruction::Control(Control::CallNative(
                                *native_module_index,
                                *type_index,
                                *function_index,
                            )),
                        }
                    }
                    _ => Instruction::Original(original_instruction.to_owned()), // 其他指令不用转换
                };

                instructions.push(instruction);
            }

            // 递增函数的开始地址偏移值
            function_addr_offset += original_instructions.len();
        }

        instructions_list.push(instructions);
    }

    Ok(instructions_list)
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

    for (index, type_index) in ast_module.function_list.iter().enumerate() {
        let instruction_count = ast_module.code_items[index].instruction_items.len();
        let function_location = FunctionLocation::Internal {
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

#[derive(Debug, PartialEq, Clone)]
enum BlockStructureType {
    Block,
    Loop,
    If,
}

#[derive(Debug, PartialEq, Clone)]
struct BlockLocation {
    block_structure_type: BlockStructureType,
    start_index: usize,
    end_index: usize,
    middle_index: usize, // 仅 if 结构块才有
}

#[derive(Debug, PartialEq, Clone)]
struct IndexedBlockLocation {
    block_index: usize,
    block_location: BlockLocation,
}

impl IndexedBlockLocation {
    fn new(
        block_structure_type: BlockStructureType,
        block_index: usize,
        start_index: usize,
    ) -> Self {
        Self {
            block_index,
            block_location: BlockLocation {
                block_structure_type,
                start_index,
                end_index: 0,
                middle_index: 0,
            },
        }
    }
}

/// 对一个函数的指令序列当中的块结构生成位置信息列表
/// 主要是为了获取 else 和 end 指令的位置
///
/// 示例：
/// 0--block-start
/// |  1--loop-start
/// |  1--loop-end
/// |  2--block-start
/// |  |  3--if-start
/// |  |  3--if-mid
/// |  |  |  4--block-start
/// |  |  |  4--block-end
/// |  |  3--if-end
/// |  2--block-end
/// 0--block-end
fn get_function_block_locations(code_item: &CodeItem) -> Vec<BlockLocation> {
    let mut indexed_block_locations: Vec<IndexedBlockLocation> = vec![]; // 未排序的
    let mut block_stack: Vec<IndexedBlockLocation> = vec![];

    for (addr, instruction) in code_item.instruction_items.iter().enumerate() {
        match instruction {
            instruction::Instruction::Block(_, block_index) => {
                block_stack.push(IndexedBlockLocation::new(
                    BlockStructureType::Block,
                    *block_index as usize,
                    addr,
                ));
            }
            instruction::Instruction::Loop(_, block_index) => {
                block_stack.push(IndexedBlockLocation::new(
                    BlockStructureType::Loop,
                    *block_index as usize,
                    addr,
                ));
            }
            instruction::Instruction::If(_, block_index) => {
                block_stack.push(IndexedBlockLocation::new(
                    BlockStructureType::If,
                    *block_index as usize,
                    addr,
                ));
            }
            instruction::Instruction::Else => {
                let stack_last_index = block_stack.len() - 1;
                let index_block_location = &mut block_stack[stack_last_index];
                index_block_location.block_location.middle_index = addr;
            }
            instruction::Instruction::End => {
                let stack_last_index = block_stack.len() - 1;
                let index_block_location = &mut block_stack[stack_last_index];
                index_block_location.block_location.end_index = addr;

                // 弹出一项
                let item = block_stack.pop().unwrap();
                indexed_block_locations.push(item);
            }
            _ => {
                //
            }
        }
    }

    // 对 indexed_block_locations 按照 block index 进行排序
    indexed_block_locations.sort_by_key(|item| item.block_index);

    // 提取结构位置信息列表
    indexed_block_locations
        .iter()
        .map(|item| &item.block_location)
        .map(|item| item.to_owned())
        .collect::<Vec<BlockLocation>>()
}
