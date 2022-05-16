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
//! - `br 指令` 转换为 `jump 控制指令`，跳转目标由目标 block 类型所决定，即
//!   * 对于原 block/if 结构块，跳转目标为原始结构块的 `end 指令` 所在的位置，
//!   * 对于原 loop 结构块，跳转目标为原始结构块的开始位置；
//! - `br_if 指令` 转换为 `jump_not_eq_zero 控制指令`；
//! - `return 指令` 转换为 `jump 控制指令`，跳转目标为函数的最后一条指令（即 `end 指令`）所在的位置；
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
//! - jump (relative_depth, addr)
//! - jump_not_eq_zero (relative_depth, addr)
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
        internal_function_index: usize,
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
    native_modules: &[NativeModule],
    named_ast_modules: &[NamedAstModule],
) -> Result<Vec<Vec<Instruction>>, EngineError> {
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
    // 具体来说，因为一个模块里的导入函数（即对应的 FunctionLocation::External）即有可能
    // 是另外一个模块的函数，也可能是另外一个模块的导入函数再次导出的函数，
    // 还有可能是本地模块的本地函数。
    // 这一个步骤主要就是为了解析 FunctionLocation::External 到
    // FunctionItem::Native 和 FunctionItem::External。

    let module_names = get_module_names(native_modules, named_ast_modules);
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
                                    internal_function_index,
                                    type_index,
                                    start_index,
                                    end_index,
                                } => {
                                    // 目标函数是外部模块的内部函数
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
                        Instruction::Control(Control::Jump(0, function_addr_offset + end_index))
                    }
                    instruction::Instruction::Br(relative_depth) => {
                        // target_depth 为目标层的层级，函数本层的层级为 0，第一层 block 的层级为 1，比如
                        //
                        // function
                        //  |         <--- level 0
                        //  |-- block
                        //  |   |     <--- level 1
                        //  |-- end
                        //
                        let target_depth = block_index_stack.len() - *relative_depth as usize;

                        if (target_depth as isize) < 0 {
                            // 目标层级超出了范围
                            return Err(EngineError::OutOfIndex(
                                "target depth out of index for instruction \"br\"".to_string(),
                            ));
                        }

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

                        Instruction::Control(Control::Jump(
                            *relative_depth as usize,
                            function_addr_offset + target_addr,
                        ))
                    }
                    instruction::Instruction::BrIf(relative_depth) => {
                        let target_depth = block_index_stack.len() - *relative_depth as usize;

                        if (target_depth as isize) < 0 {
                            // 目标层级超出了范围
                            return Err(EngineError::OutOfIndex(
                                "target depth out of index for instruction \"br\"".to_string(),
                            ));
                        }

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

                        Instruction::Control(Control::JumpNotEqZero(
                            *relative_depth as usize,
                            function_addr_offset + target_addr,
                        ))
                    }
                    instruction::Instruction::Return => {
                        let relative_depth = block_index_stack.len();
                        let end_index = original_instructions.len() - 1;

                        Instruction::Control(Control::Jump(
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
                                internal_function_index,
                                start_index,
                                end_index,
                            } => Instruction::Control(Control::CallInternal(
                                *type_index,
                                *function_index as usize,
                                *internal_function_index,
                                *start_index,
                            )),
                            FunctionItem::External {
                                type_index,
                                ast_module_index: vm_module_index,
                                function_index,
                                internal_function_index,
                                start_index,
                                end_index,
                            } => Instruction::Control(Control::CallExternal(
                                *vm_module_index,
                                *type_index,
                                *function_index,
                                *internal_function_index,
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
                    instruction::Instruction::End => {
                        // 函数的指令序列最后一个指令，即 `end 指令` 不属于结构块，所以需要排除
                        // 结构块栈已经弹空的情况
                        if block_index_stack.len() > 0 {
                            block_index_stack.pop();
                        }

                        Instruction::Original(instruction::Instruction::End)
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
/// function
/// |
/// |  0--block-start
/// |  |  1--loop-start
/// |  |  |  2--block-start
/// |  |  |  2--block-end
/// |  |  1--loop-end
/// |  |  3--block-start
/// |  |  |  4--if-start
/// |  |  |  |  5--block-start
/// |  |  |  |  5--block-end
/// |  |  |  4--if-mid
/// |  |  |  4--if-end
/// |  |  3--block-end
/// |  0--block-end
/// |  6--block-start
/// |  6--block-end
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
                let indexed_block_location = &mut block_stack[stack_last_index];
                indexed_block_location.block_location.middle_index = addr;
            }
            instruction::Instruction::End => {
                // 函数的指令序列最后一个指令，即 `end 指令` 不属于结构块，所以需要排除
                // 结构块栈已经弹空的情况
                if block_stack.len() > 0 {
                    let stack_last_index = block_stack.len() - 1;
                    let indexex_block_location = &mut block_stack[stack_last_index];
                    indexex_block_location.block_location.end_index = addr;

                    // 弹出一项并移入 indexed_block_locations
                    let last_indexed_block_location = block_stack.pop().unwrap();
                    indexed_block_locations.push(last_indexed_block_location);
                }
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

#[cfg(test)]
mod tests {
    use super::{compile, NamedAstModule};
    use crate::{
        error::NativeError,
        native_module::{NativeFunction, NativeModule},
        object::{Control, Instruction},
    };
    use anvm_ast::{
        ast::{self, CodeItem, ExportItem, FunctionType, ImportItem, TypeItem},
        instruction::{self, BlockType},
        types::{Value, ValueType},
    };

    use pretty_assertions::assert_eq;

    /// 创建一个测试用的 AST Module
    /// 跟测试无关的成员均以空列表顶替。
    fn create_test_ast_module(
        name: &str,
        import_items: Vec<ImportItem>,
        function_list: Vec<u32>,
        export_items: Vec<ExportItem>,
        code_items: Vec<CodeItem>,
    ) -> NamedAstModule {
        NamedAstModule {
            name: name.to_string(),
            module: ast::Module {
                custom_items: vec![],
                type_items: vec![], // 目前 compiler 不检查函数的类型，所以可以传入一个空的类型列表
                import_items,
                function_list,
                tables: vec![],
                memory_blocks: vec![],
                global_items: vec![],
                export_items,
                start_function_index: None,
                element_items: vec![],
                code_items,
                data_items: vec![],
            },
        }
    }

    /// 创建最小化的 AST Module
    fn create_simple_test_ast_module(
        name: &str,
        function_list: Vec<u32>,
        code_items: Vec<CodeItem>,
    ) -> NamedAstModule {
        create_test_ast_module(name, vec![], function_list, vec![], code_items)
    }

    fn test_native_function_add(params: &[Value]) -> Result<Vec<Value>, NativeError> {
        todo!()
    }

    fn test_native_function_sub(params: &[Value]) -> Result<Vec<Value>, NativeError> {
        todo!()
    }

    fn create_test_native_module() -> NativeModule {
        let mut module = NativeModule::new("m0");

        module.add_function(
            "add",
            vec![ValueType::I32, ValueType::I32],
            vec!["left", "right"],
            vec![ValueType::I32],
            test_native_function_add,
        );

        module.add_function(
            "sub",
            vec![ValueType::I32, ValueType::I32],
            vec!["left", "right"],
            vec![ValueType::I32],
            test_native_function_sub,
        );

        module
    }

    #[test]
    fn test_instruction_combine() {
        let native_modules: Vec<NativeModule> = vec![];
        let named_ast_modules: Vec<NamedAstModule> = vec![
            create_simple_test_ast_module(
                "m0",
                vec![0, 0, 0],
                vec![
                    CodeItem {
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(1),
                            instruction::Instruction::End,
                        ],
                    },
                    CodeItem {
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(2),
                            instruction::Instruction::End,
                        ],
                    },
                    CodeItem {
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(3),
                            instruction::Instruction::End,
                        ],
                    },
                ],
            ),
            create_simple_test_ast_module(
                "m1",
                vec![0, 0],
                vec![
                    CodeItem {
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(1),
                            instruction::Instruction::End,
                        ],
                    },
                    CodeItem {
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(2),
                            instruction::Instruction::I32Const(3),
                            instruction::Instruction::I32Add,
                            instruction::Instruction::End,
                        ],
                    },
                ],
            ),
        ];

        let actual = compile(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![
            vec![
                Instruction::Original(instruction::Instruction::I32Const(1)),
                Instruction::Original(instruction::Instruction::End),
                Instruction::Original(instruction::Instruction::I32Const(2)),
                Instruction::Original(instruction::Instruction::End),
                Instruction::Original(instruction::Instruction::I32Const(3)),
                Instruction::Original(instruction::Instruction::End),
            ],
            vec![
                Instruction::Original(instruction::Instruction::I32Const(1)),
                Instruction::Original(instruction::Instruction::End),
                Instruction::Original(instruction::Instruction::I32Const(2)),
                Instruction::Original(instruction::Instruction::I32Const(3)),
                Instruction::Original(instruction::Instruction::I32Add),
                Instruction::Original(instruction::Instruction::End),
            ],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_blocks() {
        let native_modules: Vec<NativeModule> = vec![];
        let named_ast_modules: Vec<NamedAstModule> = vec![create_simple_test_ast_module(
            "m0",
            vec![0, 0],
            vec![
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(0), // #00
                        instruction::Instruction::End,         // #01
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        // 创建如下的结构块
                        //
                        // |  0--block-start
                        // |  |  1--loop-start
                        // |  |  |  2--block-start
                        // |  |  |  2--block-end
                        // |  |  1--loop-end
                        // |  0--block-end
                        instruction::Instruction::I32Const(0), // #02
                        instruction::Instruction::I32Const(1), // #03
                        instruction::Instruction::Block(BlockType::Builtin(None), 0), // #04 - block 0
                        instruction::Instruction::Br(0),                              // #05
                        instruction::Instruction::Br(1),                              // #06
                        instruction::Instruction::Return,                             // #07
                        instruction::Instruction::Loop(BlockType::Builtin(None), 1), // #08 - block 1 loop
                        instruction::Instruction::Br(0),                             // #09
                        instruction::Instruction::Br(1),                             // #10
                        instruction::Instruction::Br(2),                             // #11
                        instruction::Instruction::Return,                            // #12
                        instruction::Instruction::Block(BlockType::Builtin(None), 2), // #13 - block 2
                        instruction::Instruction::Br(0),                              // #14
                        instruction::Instruction::Br(1),                              // #15
                        instruction::Instruction::Br(2),                              // #16
                        instruction::Instruction::Br(3),                              // #17
                        instruction::Instruction::Return,                             // #18
                        instruction::Instruction::End, // #19 - block 2 end
                        instruction::Instruction::Br(0), // #20
                        instruction::Instruction::Br(1), // #21
                        instruction::Instruction::Br(2), // #22
                        instruction::Instruction::Return, // #23
                        instruction::Instruction::End, // #24 - block 1 end
                        instruction::Instruction::Br(0), // #25
                        instruction::Instruction::Br(1), // #26
                        instruction::Instruction::Return, // #27
                        instruction::Instruction::End, // #28 - block 0 end
                        instruction::Instruction::I32Const(2), // #29
                        instruction::Instruction::I32Const(3), // #30
                        instruction::Instruction::Br(0), // #31
                        instruction::Instruction::Return, // #32
                        instruction::Instruction::I32Const(4), // #33
                        instruction::Instruction::I32Const(5), // #34
                        instruction::Instruction::End, // #35
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        // 创建如下的结构块
                        //
                        // |  0--block-start
                        // |  |  1--loop-start
                        // |  |  |  2--block-start
                        // |  |  |  2--block-end
                        // |  |  1--loop-end
                        // |  |
                        // |  |  3--block-start
                        // |  |  |  4--if-start
                        // |  |  |  |  5--block-start
                        // |  |  |  |  5--block-end
                        // |  |  |  4--if-mid
                        // |  |  |  |  6--block-start
                        // |  |  |  |  6--block-end
                        // |  |  |  4--if-end
                        // |  |  3--block-end
                        // |  0--block-end
                        instruction::Instruction::I32Const(0), // #36
                        instruction::Instruction::I32Const(1), // #37
                        instruction::Instruction::Block(BlockType::Builtin(None), 0), // #38 - block 0
                        instruction::Instruction::Loop(BlockType::Builtin(None), 1), // #39 - block 1 loop
                        instruction::Instruction::Block(BlockType::Builtin(None), 2), // #40 - block 2
                        instruction::Instruction::End, // #41 - block 2 end
                        instruction::Instruction::End, // #42 - block 1 end
                        //
                        instruction::Instruction::Block(BlockType::Builtin(None), 3), // #43 - block 3
                        instruction::Instruction::Br(0),                              // #44
                        instruction::Instruction::Br(1),                              // #45
                        instruction::Instruction::Return,                             // #46
                        instruction::Instruction::If(BlockType::Builtin(None), 4), // #47 - block 4 if
                        instruction::Instruction::BrIf(0),                         // #48
                        instruction::Instruction::BrIf(1),                         // #49
                        instruction::Instruction::BrIf(2),                         // #50
                        instruction::Instruction::Return,                          // #51
                        instruction::Instruction::Block(BlockType::Builtin(None), 5), // #52 - block 5
                        instruction::Instruction::Br(0),                              // #53
                        instruction::Instruction::Br(1),                              // #54
                        instruction::Instruction::Br(2),                              // #55
                        instruction::Instruction::Br(3),                              // #56
                        instruction::Instruction::Br(4), // #57 - jump to function end
                        instruction::Instruction::Return, // #58
                        instruction::Instruction::End,   // #59 - block 5 end
                        instruction::Instruction::Else,  // #60 - else
                        instruction::Instruction::BrIf(0), // #61
                        instruction::Instruction::BrIf(1), // #62
                        instruction::Instruction::BrIf(2), // #63
                        instruction::Instruction::Return, // #64
                        instruction::Instruction::Block(BlockType::Builtin(None), 6), // #65 - block 6
                        instruction::Instruction::Br(0),                              // #66
                        instruction::Instruction::Br(1),                              // #67
                        instruction::Instruction::Br(2),                              // #68
                        instruction::Instruction::Br(3),                              // #69
                        instruction::Instruction::Br(4), // #70 - jump to function end
                        instruction::Instruction::Return, // #71
                        instruction::Instruction::End,   // #72 - block 6 end
                        instruction::Instruction::End,   // #73 // block 4 end
                        instruction::Instruction::Br(0), // #74
                        instruction::Instruction::Br(1), // #75
                        instruction::Instruction::Return, // #76
                        instruction::Instruction::End,   // #77 // block 3 end
                        //
                        instruction::Instruction::Br(0),       // #78
                        instruction::Instruction::Return,      // #79
                        instruction::Instruction::End,         // #80 // block 0 end
                        instruction::Instruction::I32Const(0), // #81
                        instruction::Instruction::I32Const(1), // #82
                        instruction::Instruction::Br(0),       // #83
                        instruction::Instruction::Return,      // #84
                        instruction::Instruction::End,         // #85
                    ],
                },
            ],
        )];

        let actual = compile(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function 0
            Instruction::Original(instruction::Instruction::I32Const(0)),
            Instruction::Original(instruction::Instruction::End),
            // function 1
            Instruction::Original(instruction::Instruction::I32Const(0)), // #02
            Instruction::Original(instruction::Instruction::I32Const(1)), // #03
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #04 - block 0
            Instruction::Control(Control::Jump(0, 28)),                   // #05
            Instruction::Control(Control::Jump(1, 35)),                   // #06
            Instruction::Control(Control::Jump(1, 35)),                   // #07
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #08 - block 1 - loop
            Instruction::Control(Control::Jump(0, 8)),                    // #09
            Instruction::Control(Control::Jump(1, 28)),                   // #10
            Instruction::Control(Control::Jump(2, 35)),                   // #11
            Instruction::Control(Control::Jump(2, 35)),                   // #12
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #13 - block 2
            Instruction::Control(Control::Jump(0, 19)),                   // #14
            Instruction::Control(Control::Jump(1, 8)),                    // #15
            Instruction::Control(Control::Jump(2, 28)),                   // #16
            Instruction::Control(Control::Jump(3, 35)),                   // #17
            Instruction::Control(Control::Jump(3, 35)),                   // #18
            Instruction::Original(instruction::Instruction::End),         // #19 - block 2 end
            Instruction::Control(Control::Jump(0, 8)),                    // #20
            Instruction::Control(Control::Jump(1, 28)),                   // #21
            Instruction::Control(Control::Jump(2, 35)),                   // #22
            Instruction::Control(Control::Jump(2, 35)),                   // #23
            Instruction::Original(instruction::Instruction::End),         // #24 - block 1 end
            Instruction::Control(Control::Jump(0, 28)),                   // #25
            Instruction::Control(Control::Jump(1, 35)),                   // #26
            Instruction::Control(Control::Jump(1, 35)),                   // #27
            Instruction::Original(instruction::Instruction::End),         // #28 - block 0 end
            Instruction::Original(instruction::Instruction::I32Const(2)), // #29
            Instruction::Original(instruction::Instruction::I32Const(3)), // #30
            Instruction::Control(Control::Jump(0, 35)),                   // #31
            Instruction::Control(Control::Jump(0, 35)),                   // #32
            Instruction::Original(instruction::Instruction::I32Const(4)), // #33
            Instruction::Original(instruction::Instruction::I32Const(5)), // #34
            Instruction::Original(instruction::Instruction::End),         // #35
            // function 3
            Instruction::Original(instruction::Instruction::I32Const(0)), // #36
            Instruction::Original(instruction::Instruction::I32Const(1)), // #37
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #38 - block 0
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #39 - block 1 loop
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #40 - block 2
            Instruction::Original(instruction::Instruction::End),         // #41 - block 2 end
            Instruction::Original(instruction::Instruction::End),         // #42 - block 1 end
            //
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #43 - block 3
            Instruction::Control(Control::Jump(0, 77)),                     // #44
            Instruction::Control(Control::Jump(1, 80)),                     // #45
            Instruction::Control(Control::Jump(2, 85)),                     // #46
            Instruction::Control(Control::BlockJumpEqZero(BlockType::Builtin(None), 60)), // #47 - block 4 if
            Instruction::Control(Control::JumpNotEqZero(0, 73)),                          // #48
            Instruction::Control(Control::JumpNotEqZero(1, 77)),                          // #49
            Instruction::Control(Control::JumpNotEqZero(2, 80)),                          // #50
            Instruction::Control(Control::Jump(3, 85)),                                   // #51
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #52 - block 5
            Instruction::Control(Control::Jump(0, 59)),                     // #53
            Instruction::Control(Control::Jump(1, 73)),                     // #54
            Instruction::Control(Control::Jump(2, 77)),                     // #55
            Instruction::Control(Control::Jump(3, 80)),                     // #56
            Instruction::Control(Control::Jump(4, 85)), // #57 - jump to function end
            Instruction::Control(Control::Jump(4, 85)), // #58
            Instruction::Original(instruction::Instruction::End), // #59 - block 5 end
            Instruction::Control(Control::Jump(0, 73)), // #60 - else
            Instruction::Control(Control::JumpNotEqZero(0, 73)), // #61
            Instruction::Control(Control::JumpNotEqZero(1, 77)), // #62
            Instruction::Control(Control::JumpNotEqZero(2, 80)), // #63
            Instruction::Control(Control::Jump(3, 85)), // #64
            Instruction::Control(Control::Block(BlockType::Builtin(None))), // #65 - block 6
            Instruction::Control(Control::Jump(0, 72)), // #66
            Instruction::Control(Control::Jump(1, 73)), // #67
            Instruction::Control(Control::Jump(2, 77)), // #68
            Instruction::Control(Control::Jump(3, 80)), // #69
            Instruction::Control(Control::Jump(4, 85)), // #70 - jump to function end
            Instruction::Control(Control::Jump(4, 85)), // #71
            Instruction::Original(instruction::Instruction::End), // #72 - block 6 end
            Instruction::Original(instruction::Instruction::End), // #73 // block 4 end
            Instruction::Control(Control::Jump(0, 77)), // #74
            Instruction::Control(Control::Jump(1, 80)), // #75
            Instruction::Control(Control::Jump(2, 85)), // #76
            Instruction::Original(instruction::Instruction::End), // #77 // block 3 end
            //
            Instruction::Control(Control::Jump(0, 80)), // #78
            Instruction::Control(Control::Jump(1, 85)), // #79
            Instruction::Original(instruction::Instruction::End), // #80 // block 0 end
            Instruction::Original(instruction::Instruction::I32Const(0)), // #81
            Instruction::Original(instruction::Instruction::I32Const(1)), // #82
            Instruction::Control(Control::Jump(0, 85)), // #83
            Instruction::Control(Control::Jump(0, 85)), // #84
            Instruction::Original(instruction::Instruction::End), // #85
        ]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_module_internal() {
        let native_modules: Vec<NativeModule> = vec![];
        let named_ast_modules: Vec<NamedAstModule> = vec![create_simple_test_ast_module(
            "m0",
            vec![0, 0, 0],
            vec![
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(0),  // #00
                        instruction::Instruction::I32Const(1),  // #01
                        instruction::Instruction::Call(1),      // #02
                        instruction::Instruction::I32Const(10), // #03
                        instruction::Instruction::Call(2),      // #04
                        instruction::Instruction::I32Const(11), // #05
                        instruction::Instruction::End,          // #06
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(2), // #07
                        instruction::Instruction::I32Const(3), // #08
                        instruction::Instruction::End,         // #09
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(4), // #10
                        instruction::Instruction::I32Const(5), // #11
                        instruction::Instruction::Call(1),     // #12
                        instruction::Instruction::End,         // #13
                    ],
                },
            ],
        )];

        let actual = compile(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function 0
            Instruction::Original(instruction::Instruction::I32Const(0)), // #00
            Instruction::Original(instruction::Instruction::I32Const(1)), // #01
            Instruction::Control(Control::CallInternal(0, 1, 1, 7)),      // #02
            Instruction::Original(instruction::Instruction::I32Const(10)), // #03
            Instruction::Control(Control::CallInternal(0, 2, 2, 10)),     // #04
            Instruction::Original(instruction::Instruction::I32Const(11)), // #05
            Instruction::Original(instruction::Instruction::End),         // #06
            // function 1
            Instruction::Original(instruction::Instruction::I32Const(2)), // #07
            Instruction::Original(instruction::Instruction::I32Const(3)), // #08
            Instruction::Original(instruction::Instruction::End),         // #09
            // function 2
            Instruction::Original(instruction::Instruction::I32Const(4)), // #10
            Instruction::Original(instruction::Instruction::I32Const(5)), // #11
            Instruction::Control(Control::CallInternal(0, 1, 1, 7)),      // #12
            Instruction::Original(instruction::Instruction::End),         // #13
        ]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_module_external() {
        let native_modules: Vec<NativeModule> = vec![];
        let named_ast_modules: Vec<NamedAstModule> = vec![
            create_test_ast_module(
                "m0",
                vec![],
                vec![0, 0],
                vec![
                    ExportItem {
                        name: "f0".to_string(),
                        export_descriptor: ast::ExportDescriptor::FunctionIndex(0),
                    },
                    ExportItem {
                        name: "f1".to_string(),
                        export_descriptor: ast::ExportDescriptor::FunctionIndex(1),
                    },
                ],
                vec![
                    CodeItem {
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(0), // #00
                            instruction::Instruction::I32Const(1), // #01
                            instruction::Instruction::End,         // #02
                        ],
                    },
                    CodeItem {
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(2), // #03
                            instruction::Instruction::I32Const(3), // #04
                            instruction::Instruction::Call(0),     // #05
                            instruction::Instruction::End,         // #06
                        ],
                    },
                ],
            ),
            create_test_ast_module(
                "m1",
                vec![
                    ImportItem {
                        module_name: "m0".to_string(),
                        item_name: "f0".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                    ImportItem {
                        module_name: "m0".to_string(),
                        item_name: "f1".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                ],
                vec![0, 0],
                vec![],
                vec![
                    CodeItem {
                        // function index 2
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(0),  // #00
                            instruction::Instruction::I32Const(1),  // #01
                            instruction::Instruction::Call(0),      // #02
                            instruction::Instruction::I32Const(10), // #03
                            instruction::Instruction::Call(1),      // #04
                            instruction::Instruction::I32Const(11), // #05
                            instruction::Instruction::Call(3),      // #06
                            instruction::Instruction::I32Const(12), // #07
                            instruction::Instruction::End,          // #08
                        ],
                    },
                    CodeItem {
                        // function index 3
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(2),  // #09
                            instruction::Instruction::I32Const(3),  // #10
                            instruction::Instruction::Call(0),      // #11
                            instruction::Instruction::I32Const(20), // #12
                            instruction::Instruction::Call(1),      // #13
                            instruction::Instruction::I32Const(21), // #14
                            instruction::Instruction::End,          // #15
                        ],
                    },
                ],
            ),
        ];

        let actual = compile(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![
            vec![
                // function 0
                Instruction::Original(instruction::Instruction::I32Const(0)), // #00
                Instruction::Original(instruction::Instruction::I32Const(1)), // #01
                Instruction::Original(instruction::Instruction::End),         // #02
                // function 1
                Instruction::Original(instruction::Instruction::I32Const(2)), // #03
                Instruction::Original(instruction::Instruction::I32Const(3)), // #04
                Instruction::Control(Control::CallInternal(0, 0, 0, 0)),      // #05
                Instruction::Original(instruction::Instruction::End),         // #06
            ],
            vec![
                // function index 2
                Instruction::Original(instruction::Instruction::I32Const(0)), // #00
                Instruction::Original(instruction::Instruction::I32Const(1)), // #01
                Instruction::Control(Control::CallExternal(0, 0, 0, 0, 0)),   // #02
                Instruction::Original(instruction::Instruction::I32Const(10)), // #03
                Instruction::Control(Control::CallExternal(0, 0, 1, 1, 3)),   // #04
                Instruction::Original(instruction::Instruction::I32Const(11)), // #05
                Instruction::Control(Control::CallInternal(0, 3, 1, 9)),      // #06
                Instruction::Original(instruction::Instruction::I32Const(12)), // #07
                Instruction::Original(instruction::Instruction::End),         // #08
                // function index 3
                Instruction::Original(instruction::Instruction::I32Const(2)), // #09
                Instruction::Original(instruction::Instruction::I32Const(3)), // #10
                Instruction::Control(Control::CallExternal(0, 0, 0, 0, 0)),   // #11
                Instruction::Original(instruction::Instruction::I32Const(20)), // #12
                Instruction::Control(Control::CallExternal(0, 0, 1, 1, 3)),   // #13
                Instruction::Original(instruction::Instruction::I32Const(21)), // #14
                Instruction::Original(instruction::Instruction::End),         // #15
            ],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_module_native() {
        let native_modules: Vec<NativeModule> = vec![create_test_native_module()];

        let named_ast_modules: Vec<NamedAstModule> = vec![create_test_ast_module(
            "m1",
            vec![
                ImportItem {
                    module_name: "m0".to_string(),
                    item_name: "add".to_string(),
                    import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                },
                ImportItem {
                    module_name: "m0".to_string(),
                    item_name: "sub".to_string(),
                    import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                },
            ],
            vec![0, 0],
            vec![],
            vec![
                CodeItem {
                    // function index 2
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(0),  // #00
                        instruction::Instruction::I32Const(1),  // #01
                        instruction::Instruction::Call(0),      // #02
                        instruction::Instruction::I32Const(10), // #03
                        instruction::Instruction::Call(1),      // #04
                        instruction::Instruction::I32Const(11), // #05
                        instruction::Instruction::Call(3),      // #06
                        instruction::Instruction::I32Const(12), // #07
                        instruction::Instruction::End,          // #08
                    ],
                },
                CodeItem {
                    // function index 3
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(2),  // #09
                        instruction::Instruction::I32Const(3),  // #10
                        instruction::Instruction::Call(0),      // #11
                        instruction::Instruction::I32Const(20), // #12
                        instruction::Instruction::Call(1),      // #13
                        instruction::Instruction::I32Const(21), // #14
                        instruction::Instruction::End,          // #15
                    ],
                },
            ],
        )];

        let actual = compile(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function index 2
            Instruction::Original(instruction::Instruction::I32Const(0)), // #00
            Instruction::Original(instruction::Instruction::I32Const(1)), // #01
            Instruction::Control(Control::CallNative(0, 0, 0)),           // #02
            Instruction::Original(instruction::Instruction::I32Const(10)), // #03
            Instruction::Control(Control::CallNative(0, 0, 1)),           // #04
            Instruction::Original(instruction::Instruction::I32Const(11)), // #05
            Instruction::Control(Control::CallInternal(0, 3, 1, 9)),      // #06
            Instruction::Original(instruction::Instruction::I32Const(12)), // #07
            Instruction::Original(instruction::Instruction::End),         // #08
            // function index 3
            Instruction::Original(instruction::Instruction::I32Const(2)), // #09
            Instruction::Original(instruction::Instruction::I32Const(3)), // #10
            Instruction::Control(Control::CallNative(0, 0, 0)),           // #11
            Instruction::Original(instruction::Instruction::I32Const(20)), // #12
            Instruction::Control(Control::CallNative(0, 0, 1)),           // #13
            Instruction::Original(instruction::Instruction::I32Const(21)), // #14
            Instruction::Original(instruction::Instruction::End),         // #15
        ]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_module_external_complex() {
        // 本测试一共 4 个模块：
        // - m0 为本地函数模块；
        // - m1 为普通模块，定义了函数 f0 和 bottom，并导出函数 bottom；
        // - m2 为普通模块，从 m0 导入了函数 add，从 m1 导入了函数 bottom，定义了个函数 f1 和 middle，
        //      然后导出了 add, bottom, middle
        // - m3 为普通模块，从 m2 导入了 3 个函数，定义了函数 test

        let native_modules: Vec<NativeModule> = vec![create_test_native_module()];

        let named_ast_modules: Vec<NamedAstModule> = vec![
            create_test_ast_module(
                "m1",
                vec![],
                vec![0, 0],
                vec![ExportItem {
                    name: "bottom".to_string(),
                    export_descriptor: ast::ExportDescriptor::FunctionIndex(1),
                }],
                vec![
                    CodeItem {
                        // function index 0
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(0), // #00
                            instruction::Instruction::End,         // #01
                        ],
                    },
                    CodeItem {
                        // function index 1 - bottom
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(1), // #02
                            instruction::Instruction::End,         // #03
                        ],
                    },
                ],
            ),
            create_test_ast_module(
                "m2",
                vec![
                    ImportItem {
                        module_name: "m0".to_string(),
                        item_name: "add".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                    ImportItem {
                        module_name: "m1".to_string(),
                        item_name: "bottom".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                ],
                vec![0, 0],
                vec![
                    ExportItem {
                        name: "f0".to_string(),
                        export_descriptor: ast::ExportDescriptor::FunctionIndex(0),
                    },
                    ExportItem {
                        name: "f1".to_string(),
                        export_descriptor: ast::ExportDescriptor::FunctionIndex(1),
                    },
                    ExportItem {
                        name: "f2".to_string(),
                        export_descriptor: ast::ExportDescriptor::FunctionIndex(3),
                    },
                ],
                vec![
                    CodeItem {
                        // function index 2, internal index 0
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(2), // #00
                            instruction::Instruction::Drop,        // #01
                            instruction::Instruction::End,         // #02
                        ],
                    },
                    CodeItem {
                        // function index 3, internal index 1
                        local_groups: vec![],
                        instruction_items: vec![
                            instruction::Instruction::I32Const(3), // #03
                            instruction::Instruction::End,         // #04
                        ],
                    },
                ],
            ),
            create_test_ast_module(
                "m3",
                vec![
                    ImportItem {
                        module_name: "m2".to_string(),
                        item_name: "f0".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                    ImportItem {
                        module_name: "m2".to_string(),
                        item_name: "f1".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                    ImportItem {
                        module_name: "m2".to_string(),
                        item_name: "f2".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                ],
                vec![0],
                vec![],
                vec![CodeItem {
                    // function index 3
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(0), // #00
                        instruction::Instruction::Call(0),     // #01
                        instruction::Instruction::I32Const(1), // #02
                        instruction::Instruction::Call(1),     // #03
                        instruction::Instruction::I32Const(2), // #04
                        instruction::Instruction::Call(2),     // #05
                        instruction::Instruction::End,         // #06
                    ],
                }],
            ),
        ];

        let actual = compile(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![
            vec![
                // function index 0
                Instruction::Original(instruction::Instruction::I32Const(0)), // #00
                Instruction::Original(instruction::Instruction::End),         // #01
                // function index 1
                Instruction::Original(instruction::Instruction::I32Const(1)), // #02
                Instruction::Original(instruction::Instruction::End),         // #03
            ],
            vec![
                // function index 2
                Instruction::Original(instruction::Instruction::I32Const(2)), // #00
                Instruction::Original(instruction::Instruction::Drop),        // #01
                Instruction::Original(instruction::Instruction::End),         // #02
                // function index 3
                Instruction::Original(instruction::Instruction::I32Const(3)), // #03
                Instruction::Original(instruction::Instruction::End),         // #04
            ],
            vec![
                // function index 3
                Instruction::Original(instruction::Instruction::I32Const(0)), // #00
                Instruction::Control(Control::CallNative(0, 0, 0)),           // #01
                Instruction::Original(instruction::Instruction::I32Const(1)), // #02
                Instruction::Control(Control::CallExternal(0, 0, 1, 1, 2)),   // #03
                Instruction::Original(instruction::Instruction::I32Const(2)), // #04
                Instruction::Control(Control::CallExternal(1, 0, 3, 1, 3)),   // #05
                Instruction::Original(instruction::Instruction::End),         // #06
            ],
        ];

        assert_eq!(actual, expected);
    }
}
