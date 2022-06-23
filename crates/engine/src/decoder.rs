// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 指令 "解码器"
//!
//! 诸如 `call`、`br`、`if`、`else` 等
//! 流程控制类指令转换为当前的 VM 引擎可以直接解析执行的指令。
//!
//! ## 指令转换规则
//!
//! 大部分指令都不需要转换，仅对流程控制（分支）和函数调用等指令需要转换为 `控制指令`，其他的不会对
//! 程序执行顺序产生影响的指令（比如数值指令）则转换为 `顺序指令`。
//!
//! - `block 指令` 转换为 `block 控制指令`；
//! - `loop 指令` 转换为 `block 控制指令`；
//! - `if 指令` 转换为 `block_jump_eq_zero 控制指令`，跳转目标为原 `else 指令` 或者原 `end 指令` 所在的位置；
//!   * 原始的 `if 指令` 其实是一个 `block 控制指令` 和一个 `jump_eq_zero 控制指令` 的语法糖，
//!     不过为了让转换后的 `指令序列` 跟原始的的位置（即索引）一一对于（以便于追踪和调试），所以
//!     新增加一个专门跟 `if 指令` 对应的 `block_jump_eq_zero 控制指令`；
//!   * 有时 `if 指令` 的结构里不一定存在 `else 指令`，这时相当于在 `end 指令` 前有一个隐藏的 `else 指令`，
//!     为了简化起见，如果 `if 指令` 的结构里不存在 `else 指令`，则直接让跳转目标为 `end 指令` 所在的位置。
//! - `else 指令` 转换为 `jump 控制指令`，跳转目标为 if 结构块当中 `end 指令` 所在的位置；
//! - `br 指令`
//!   * 对于原 block/if 结构块，转换为 `jump 控制指令`，跳转目标为原始结构块的 `end 指令` 所在的位置，
//!   * 对于原 loop 结构块，转换为 `recur 控制指令`，跳转目标为原始结构块的开始位置；
//! - `br_if 指令` 转换为 `jump_not_eq_zero 控制指令`；
//! - `return 指令` 转换为 `jump 控制指令`，跳转目标为函数的最后一条指令（即 `end 指令`）所在的位置；
//! - `call 指令`：
//!   * 对于目标为模块内的函数，转为 `call_internal 控制指令`；
//!   * 对于目标为模块外的函数，转为 `call_external 控制指令`；
//!   * 对于目标为本地的函数（native function），转为 `call_native 控制指令`；
//! - `end 指令` 转为 `return 控制指令`；
//!
//! 控制指令列表
//!
//! - block (block_type, end_addr)
//! - block_jump_eq_zero (block_type, alternate_addr, end_addr)
//! - jump (relative_depth, address)
//! - jump_not_eq_zero (relative_depth, address)
//! - recur (relative_depth, address)
//! - recur_not_eq_zero (relative_depth, address)
//! - branch ([branch_target::jump(relative_depth, address)], branch_target::recur(relative_depth, address))
//! - call_internal (type_index, function_index, address)
//! - call_external (module_index, type_index, function_index, address)
//! - call_native (module_index, type_index, function_index)
//! - dynamic_call (type_index, table_index)
//! - return

use anvm_ast::{ast::ImportDescriptor, instruction};

use crate::{
    error::{EngineError, OutOfRange, Unsupported},
    object::{BlockItem, BranchTarget, Control, FunctionItem, Instruction, NamedAstModule},
};

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
///
/// function_items_list 来自 linker 对所有函数进行链接之后的结果
pub fn decode(
    named_ast_modules: &[NamedAstModule],
    function_items_list: &Vec<Vec<FunctionItem>>,
) -> Result<Vec<Vec<Instruction>>, EngineError> {
    // 第 3 步：
    // - 将 AST 的 Instruction 转换为虚拟机可直接解析运行的 Instruction
    // - 合并一个模块里的所有内部函数的指令序列

    let mut instructions_list: Vec<Vec<Instruction>> = vec![];

    // 转换一个模块
    for (ast_module_index, named_ast_module) in named_ast_modules.iter().enumerate() {
        let mut instructions: Vec<Instruction> = vec![];

        // 获取模块第一个内部函数的索引值

        // 统计导入的函数的数量
        let internal_function_index_offset = named_ast_module
            .module
            .import_items
            .iter()
            .filter(|item| match item.import_descriptor {
                ImportDescriptor::FunctionTypeIndex(_) => true,
                _ => false,
            })
            .count();

        let function_items = &function_items_list[ast_module_index];

        // 转换一个函数
        for (internal_function_index, code_item) in
            named_ast_module.module.code_items.iter().enumerate()
        {
            let function_index = internal_function_index_offset + internal_function_index;
            let function_item = &function_items[function_index];

            let (block_items, function_start_address, function_end_address) = match function_item {
                FunctionItem::Normal {
                    vm_module_index: _,
                    type_index: _,
                    function_index: _,
                    internal_function_index: _,
                    start_address,
                    end_address,
                    block_items,
                } => (block_items, *start_address, *end_address),
                _ => unreachable!("should be normal \"function\" item"),
            };

            let mut block_index_stack: Vec<usize> = vec![];

            let function_original_instructions = &code_item.instruction_items;

            for function_original_instruction in function_original_instructions {
                let instruction = match function_original_instruction {
                    instruction::Instruction::Block(block_type, block_index) => {
                        let block_index_usize = *block_index as usize;
                        block_index_stack.push(block_index_usize);

                        // 获取 block 结构块当中的 `end 指令` 所在的位置
                        let block_item = &block_items[block_index_usize];
                        let end_address = if let BlockItem::Block {
                            block_type: _,
                            start_address: _,
                            end_address,
                        } = block_item
                        {
                            end_address
                        } else {
                            unreachable!("should be \"block\" structure")
                        };
                        Instruction::Control(Control::Block {
                            block_type: block_type.to_owned(),
                            block_index: block_index_usize,
                            end_address: function_start_address + end_address,
                        })
                    }
                    instruction::Instruction::Loop(block_type, block_index) => {
                        let block_index_usize = *block_index as usize;
                        block_index_stack.push(block_index_usize);

                        // 获取 loop 结构块当中的 `end 指令` 所在的位置
                        let block_item = &block_items[block_index_usize];
                        let end_address = if let BlockItem::Loop {
                            block_type: _,
                            start_address: _,
                            end_address,
                        } = block_item
                        {
                            end_address
                        } else {
                            unreachable!("should be \"loop\" structure")
                        };
                        Instruction::Control(Control::Block {
                            block_type: block_type.to_owned(),
                            block_index: block_index_usize,
                            end_address: function_start_address + end_address,
                        })
                    }
                    instruction::Instruction::If(block_type, block_index) => {
                        let block_index_usize = *block_index as usize;
                        block_index_stack.push(block_index_usize);

                        // 获取 if 结构块当中的 `else 指令` 所在的位置
                        let block_item = &block_items[block_index_usize];
                        let (end_address, option_alternate_address) = if let BlockItem::If {
                            block_type: _,
                            start_address: _,
                            end_address,
                            alternate_address,
                        } = block_item
                        {
                            (end_address, alternate_address)
                        } else {
                            unreachable!("should be \"if\" structure")
                        };

                        let map_alternate_address = option_alternate_address
                            .map(|address| function_start_address + address);

                        Instruction::Control(Control::BlockAndJumpWhenEqZero {
                            block_type: block_type.to_owned(),
                            block_index: block_index_usize,
                            option_alternate_address: map_alternate_address,
                            end_address: function_start_address + end_address,
                        })
                    }
                    instruction::Instruction::Else => {
                        let block_index = block_index_stack.last().unwrap();

                        // 获取 if 结构块当中的 `end 指令` 所在的位置
                        let block_index_usize = *block_index as usize;
                        let block_item = &block_items[block_index_usize];

                        let end_address = if let BlockItem::If {
                            block_type: _,
                            start_address: _,
                            end_address,
                            alternate_address: _,
                        } = block_item
                        {
                            end_address
                        } else {
                            unreachable!("should be \"if\" structure")
                        };
                        Instruction::Control(Control::JumpWithinBlock(function_start_address + end_address))
                    }
                    instruction::Instruction::Br(relative_depth) => {
                        let result = get_branch_target(
                            function_start_address,
                            function_end_address,
                            &block_items,
                            &block_index_stack,
                            *relative_depth as usize,
                        )?;

                        match result {
                            BranchTarget::Break(relative_depth, address) => {
                                let option_block_index = block_index_stack.last().map(|v| *v);
                                Instruction::Control(Control::Break {
                                    option_block_index,
                                    relative_depth,
                                    address,
                                })
                            }
                            BranchTarget::Recur(relative_depth, address) => {
                                let block_index = block_index_stack.last().map(|v| *v).unwrap();
                                Instruction::Control(Control::Recur {
                                    block_index,
                                    relative_depth,
                                    address,
                                })
                            }
                        }
                    }
                    instruction::Instruction::BrIf(relative_depth) => {
                        let result = get_branch_target(
                            function_start_address,
                            function_end_address,
                            &block_items,
                            &block_index_stack,
                            *relative_depth as usize,
                        )?;

                        match result {
                            BranchTarget::Break(relative_depth, address) => {
                                let option_block_index = block_index_stack.last().map(|v| *v);
                                Instruction::Control(Control::BreakWhenNotEqZero {
                                    option_block_index,
                                    relative_depth,
                                    address,
                                })
                            }
                            BranchTarget::Recur(relative_depth, address) => {
                                let block_index = block_index_stack.last().map(|v| *v).unwrap();
                                Instruction::Control(Control::RecurWhenNotEqZero {
                                    block_index,
                                    relative_depth,
                                    address,
                                })
                            }
                        }
                    }
                    instruction::Instruction::BrTable(relative_depths, default_relative_depth) => {
                        let mut depths = relative_depths.to_owned();
                        depths.push(*default_relative_depth);

                        let mut targets: Vec<BranchTarget> = vec![];
                        for depth in depths {
                            let target = get_branch_target(
                                function_start_address,
                                function_end_address,
                                &block_items,
                                &block_index_stack,
                                depth as usize,
                            )?;
                            targets.push(target)
                        }

                        let option_block_index = block_index_stack.last().map(|v| *v);
                        let default_branch_target = targets.last().unwrap();
                        let branch_targets = &targets[0..targets.len() - 1];

                        Instruction::Control(Control::Branch {
                            option_block_index,
                            branch_targets: branch_targets.to_owned(),
                            default_branch_target: default_branch_target.to_owned(),
                        })
                    }
                    instruction::Instruction::Return => {
                        let option_block_index = block_index_stack.last().map(|v| *v);
                        let relative_depth = block_index_stack.len();
                        let end_address = function_original_instructions.len() - 1;
                        Instruction::Control(Control::Break {
                            option_block_index,
                            relative_depth,
                            address: function_start_address + end_address,
                        })
                    }
                    instruction::Instruction::Call(function_index) => {
                        // 获取函数的位置信息
                        let function_item =
                            &function_items_list[ast_module_index][*function_index as usize];

                        match function_item {
                            FunctionItem::Normal {
                                vm_module_index,
                                type_index,
                                function_index,
                                internal_function_index,
                                start_address,
                                end_address: _,
                                block_items: _,
                            } => Instruction::Control(Control::Call {
                                vm_module_index: *vm_module_index,
                                type_index: *type_index,
                                function_index: *function_index,
                                internal_function_index: *internal_function_index,
                                address: *start_address,
                            }),
                            FunctionItem::Native {
                                native_module_index,
                                type_index,
                                function_index,
                            } => Instruction::Control(Control::CallNative {
                                native_module_index: *native_module_index,
                                type_index: *type_index,
                                function_index: *function_index,
                            }),
                        }
                    }
                    instruction::Instruction::CallIndirect(type_index, table_index) => {
                        Instruction::Control(Control::CallIndirect {
                            type_index: *type_index as usize,
                            table_index: *table_index as usize,
                        })
                    }
                    instruction::Instruction::End => {
                        // 函数的指令序列最后一个指令，即 `end 指令` 不属于结构块，所以需要排除
                        // 结构块栈已经弹空的情况
                        let option_block_index = block_index_stack.pop();
                        Instruction::Control(Control::End(option_block_index))
                    }
                    instruction::Instruction::Unreachable => {
                        Instruction::Control(Control::Unreachable)
                    }
                    instruction::Instruction::Nop => Instruction::Control(Control::Nop),
                    _ => Instruction::Sequence(function_original_instruction.to_owned()), // 其他指令归类为 `顺序指令`，
                };

                instructions.push(instruction);
            }
        }

        instructions_list.push(instructions);
    }

    Ok(instructions_list)
}

fn get_branch_target(
    function_start_address: usize,
    function_end_address: usize,
    block_items: &[BlockItem],
    block_index_stack: &[usize],
    relative_depth: usize,
) -> Result<BranchTarget, EngineError> {
    // target_level 为目标层的层级，函数本层的层级为 0，第一层 block 的层级为 1，比如
    //
    // function
    //  |         <--- level 0
    //  |-- block
    //  |   |     <--- level 1
    //  |-- end
    //
    let target_level = block_index_stack.len() - relative_depth;

    if (target_level as isize) < 0 {
        // 目标层级超出了范围
        return Err(EngineError::OutOfRange(
            OutOfRange::BlockRelativeDepthOutOfRange(relative_depth, block_index_stack.len()),
        ));
    }

    if target_level == 0 {
        // 跳到函数本层了
        // 目标位置应该是函数的最后一个指令，即 `end 指令` 所在的位置

        Ok(BranchTarget::Break(relative_depth, function_end_address))
    } else {
        let target_block_index = block_index_stack[target_level - 1];

        // 获取目标层的位置信息
        let block_item = &block_items[target_block_index];

        match block_item {
            BlockItem::Loop {
                block_type: _,
                start_address,
                end_address: _,
            } => Ok(BranchTarget::Recur(
                relative_depth,
                function_start_address + *start_address,
            )),
            BlockItem::Block {
                block_type: _,
                start_address: _,
                end_address,
            } => Ok(BranchTarget::Break(
                relative_depth,
                function_start_address + *end_address,
            )),
            BlockItem::If {
                block_type: _,
                start_address: _,
                end_address,
                alternate_address: _,
            } => Ok(BranchTarget::Break(
                relative_depth,
                function_start_address + *end_address,
            )),
        }
    }
}

/// 转换（校验）常量表达式里的指令
///
/// WebAssembly 里的
///
/// - 内存数据初始化指令当中的位置偏移值
/// - 表的元素的位置偏移值
/// - 全局变量的初始值
///
/// 都使用一条指令序列来表达，此序列也称为 `常量表达式`，表达式里的
/// 指令一般是一个 `t.const 指令` 和一个 `end 指令`，规范里允许的指令有：
///
/// - t.const
/// - ref.null
/// - ref.func x
/// - global.get x （目标只允许是导入的 global item）
///
/// 详细见：
/// https://webassembly.github.io/spec/core/valid/instructions.html#constant-expressions
///
/// 目前这里只支持 `t.const 指令`
pub fn decode_constant_expression(
    original_instructions: &[instruction::Instruction],
) -> Result<Vec<instruction::Instruction>, EngineError> {
    let mut instructions: Vec<instruction::Instruction> = vec![];

    for inst in original_instructions {
        let instruction = match inst {
            instruction::Instruction::I32Const(_)
            | instruction::Instruction::I64Const(_)
            | instruction::Instruction::F32Const(_)
            | instruction::Instruction::F64Const(_)
            | instruction::Instruction::End => inst.to_owned(),
            _ => {
                return Err(EngineError::Unsupported(
                    Unsupported::UnsupportedConstantExpressionInstruction(inst.to_owned()),
                ));
            }
        };

        instructions.push(instruction);
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::{decode, NamedAstModule};
    use crate::{
        error::{EngineError, NativeTerminate},
        linker,
        native_module::{EmptyModuleContext, NativeModule},
        object::{BranchTarget, Control, FunctionItem, Instruction},
        vm::VM,
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
        type_items: Vec<TypeItem>,
        function_list: Vec<u32>,
        import_items: Vec<ImportItem>,
        export_items: Vec<ExportItem>,
        code_items: Vec<CodeItem>,
    ) -> NamedAstModule {
        NamedAstModule {
            name: name.to_string(),
            module: ast::Module {
                custom_items: vec![],
                type_items,
                import_items,
                internal_function_to_type_index_list: function_list,
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
        type_items: Vec<TypeItem>,
        function_list: Vec<u32>,
        code_items: Vec<CodeItem>,
    ) -> NamedAstModule {
        create_test_ast_module(name, type_items, function_list, vec![], vec![], code_items)
    }

    fn test_native_function_add(
        _vm: &mut VM,
        _native_module_index: usize,
        _params: &[Value],
    ) -> Result<Vec<Value>, NativeTerminate> {
        // 返回值不是单元测试的检测项目，所以随便返回一个常量
        Ok(vec![Value::I32(10)])
    }

    fn test_native_function_sub(
        _vm: &mut VM,
        _native_module_index: usize,
        _params: &[Value],
    ) -> Result<Vec<Value>, NativeTerminate> {
        // 返回值不是单元测试的检测项目，所以随便返回一个常量
        Ok(vec![Value::I32(10)])
    }

    fn create_test_native_module() -> NativeModule {
        let empty_module_context = EmptyModuleContext::new();
        let mut module = NativeModule::new("m0", Box::new(empty_module_context));

        module.add_native_function(
            "add",
            vec![ValueType::I32, ValueType::I32],
            vec!["left", "right"],
            vec![ValueType::I32],
            test_native_function_add,
        );

        module.add_native_function(
            "sub",
            vec![ValueType::I32, ValueType::I32],
            vec!["left", "right"],
            vec![ValueType::I32],
            test_native_function_sub,
        );

        module
    }

    fn link_and_decode_function_instructions(
        native_modules: &[NativeModule],
        named_ast_modules: &[NamedAstModule],
    ) -> Result<Vec<Vec<Instruction>>, EngineError> {
        let function_items_list: Vec<Vec<FunctionItem>> =
            linker::link_functions(native_modules, named_ast_modules)?;
        decode(named_ast_modules, &function_items_list)
    }

    #[test]
    fn test_instruction_link() {
        let named_ast_modules: Vec<NamedAstModule> = vec![
            create_simple_test_ast_module(
                "m0",
                vec![TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![ValueType::I32],
                })],
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
                vec![TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![ValueType::I32],
                })],
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

        let actual = link_and_decode_function_instructions(&vec![], &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![
            vec![
                Instruction::Sequence(instruction::Instruction::I32Const(1)),
                Instruction::Control(Control::End(None)),
                Instruction::Sequence(instruction::Instruction::I32Const(2)),
                Instruction::Control(Control::End(None)),
                Instruction::Sequence(instruction::Instruction::I32Const(3)),
                Instruction::Control(Control::End(None)),
            ],
            vec![
                Instruction::Sequence(instruction::Instruction::I32Const(1)),
                Instruction::Control(Control::End(None)),
                Instruction::Sequence(instruction::Instruction::I32Const(2)),
                Instruction::Sequence(instruction::Instruction::I32Const(3)),
                Instruction::Sequence(instruction::Instruction::I32Add),
                Instruction::Control(Control::End(None)),
            ],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_blocks() {
        let named_ast_modules: Vec<NamedAstModule> = vec![create_simple_test_ast_module(
            "m0",
            vec![TypeItem::FunctionType(FunctionType {
                params: vec![ValueType::I32],
                results: vec![ValueType::I32],
            })],
            vec![0, 0, 0, 0],
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
                        // 测试 `block 结构块` 和 `loop 结构块`
                        //
                        // |  0--block-start
                        // |  |  1--loop-start
                        // |  |  |  2--block-start
                        // |  |  |  2--block-end
                        // |  |  1--loop-end
                        // |  0--block-end
                        instruction::Instruction::I32Const(0), // #02
                        instruction::Instruction::I32Const(1), // #03
                        instruction::Instruction::Block(BlockType::ResultEmpty, 0), // #04 - block 0
                        instruction::Instruction::Br(0),       // #05
                        instruction::Instruction::Br(1),       // #06
                        instruction::Instruction::Return,      // #07
                        instruction::Instruction::Loop(BlockType::ResultEmpty, 1), // #08 - block 1 loop
                        instruction::Instruction::Br(0),                           // #09
                        instruction::Instruction::Br(1),                           // #10
                        instruction::Instruction::Br(2),                           // #11
                        instruction::Instruction::Return,                          // #12
                        instruction::Instruction::Block(BlockType::ResultEmpty, 2), // #13 - block 2
                        instruction::Instruction::Br(0),                           // #14
                        instruction::Instruction::Br(1),                           // #15
                        instruction::Instruction::Br(2),                           // #16
                        instruction::Instruction::Br(3),                           // #17
                        instruction::Instruction::Return,                          // #18
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
                        // 测试 `if 结构块`
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
                        instruction::Instruction::Block(BlockType::ResultEmpty, 0), // #38 - block 0
                        instruction::Instruction::Loop(BlockType::ResultEmpty, 1), // #39 - block 1 loop
                        instruction::Instruction::Block(BlockType::ResultEmpty, 2), // #40 - block 2
                        instruction::Instruction::End, // #41 - block 2 end
                        instruction::Instruction::End, // #42 - block 1 end
                        //
                        instruction::Instruction::Block(BlockType::ResultEmpty, 3), // #43 - block 3 loop
                        instruction::Instruction::Br(0),                            // #44
                        instruction::Instruction::Br(1),                            // #45
                        instruction::Instruction::Return,                           // #46
                        instruction::Instruction::If(BlockType::ResultEmpty, 4), // #47 - block 4 if
                        instruction::Instruction::Br(0),                         // #48
                        instruction::Instruction::Br(1),                         // #49
                        instruction::Instruction::Br(2),                         // #50
                        instruction::Instruction::Return,                        // #51
                        instruction::Instruction::Block(BlockType::ResultEmpty, 5), // #52 - block 5
                        instruction::Instruction::Br(0),                         // #53
                        instruction::Instruction::Br(1),                         // #54
                        instruction::Instruction::Br(2),                         // #55
                        instruction::Instruction::Br(3),                         // #56
                        instruction::Instruction::Br(4), // #57 - jump to function end
                        instruction::Instruction::Return, // #58
                        instruction::Instruction::End,   // #59 - block 5 end
                        instruction::Instruction::Else,  // #60 - else
                        instruction::Instruction::Br(0), // #61
                        instruction::Instruction::Br(1), // #62
                        instruction::Instruction::Br(2), // #63
                        instruction::Instruction::Return, // #64
                        instruction::Instruction::Block(BlockType::ResultEmpty, 6), // #65 - block 6
                        instruction::Instruction::Br(0), // #66
                        instruction::Instruction::Br(1), // #67
                        instruction::Instruction::Br(2), // #68
                        instruction::Instruction::Br(3), // #69
                        instruction::Instruction::Br(4), // #70 - jump to function end
                        instruction::Instruction::Return, // #71
                        instruction::Instruction::End,   // #72 - block 6 end
                        instruction::Instruction::End,   // #73 - block 4 end
                        instruction::Instruction::Br(0), // #74
                        instruction::Instruction::Br(1), // #75
                        instruction::Instruction::Return, // #76
                        instruction::Instruction::End,   // #77 - block 3 end
                        //
                        instruction::Instruction::Br(0),       // #78
                        instruction::Instruction::Return,      // #79
                        instruction::Instruction::End,         // #80 - block 0 end
                        instruction::Instruction::I32Const(0), // #81
                        instruction::Instruction::I32Const(1), // #82
                        instruction::Instruction::Br(0),       // #83
                        instruction::Instruction::Return,      // #84
                        instruction::Instruction::End,         // #85
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        // 创建如下的结构块
                        // 测试缺少了 `else 指令` 的 `if 结构块`
                        //
                        // |  0--if-start
                        // |  |  1--block-start
                        // |  |  1--block-end
                        // |  0--if-end
                        instruction::Instruction::I32Const(0), // #86
                        instruction::Instruction::I32Const(1), // #87
                        instruction::Instruction::If(BlockType::ResultEmpty, 0), // #88 - block 0
                        instruction::Instruction::Br(0),       // #89
                        instruction::Instruction::Br(1),       // #90
                        instruction::Instruction::Return,      // #91
                        instruction::Instruction::Block(BlockType::ResultEmpty, 1), // #92 - block 1
                        instruction::Instruction::Br(0),       // #93
                        instruction::Instruction::Br(1),       // #94
                        instruction::Instruction::Br(2),       // #95
                        instruction::Instruction::Return,      // #96
                        instruction::Instruction::End,         // #97 - block 1 end
                        instruction::Instruction::Br(0),       // #98
                        instruction::Instruction::Br(1),       // #99
                        instruction::Instruction::Return,      // #100
                        instruction::Instruction::End,         // #101 - block 0 end
                        instruction::Instruction::I32Const(2), // #102
                        instruction::Instruction::I32Const(3), // #103
                        instruction::Instruction::End,         // #104
                    ],
                },
            ],
        )];

        let actual = link_and_decode_function_instructions(&vec![], &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function 0
            Instruction::Sequence(instruction::Instruction::I32Const(0)),
            Instruction::Control(Control::End(None)),
            // function 1
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #02
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #03
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 0,
                end_address: 28,
            }), // #04 - block 0
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 0,
                address: 28,
            }), // #05
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 35,
            }), // #06
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 35,
            }), // #07
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 1,
                end_address: 24,
            }), // #08 - block 1 - loop
            Instruction::Control(Control::Recur {
                block_index: 1,
                relative_depth: 0,
                address: 8,
            }), // #09
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 1,
                address: 28,
            }), // #10
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 2,
                address: 35,
            }), // #11
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 2,
                address: 35,
            }), // #12
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 2,
                end_address: 19,
            }), // #13 - block 2
            Instruction::Control(Control::Break {
                option_block_index: Some(2),
                relative_depth: 0,
                address: 19,
            }), // #14
            Instruction::Control(Control::Recur {
                block_index: 2,
                relative_depth: 1,
                address: 8,
            }), // #15
            Instruction::Control(Control::Break {
                option_block_index: Some(2),
                relative_depth: 2,
                address: 28,
            }), // #16
            Instruction::Control(Control::Break {
                option_block_index: Some(2),
                relative_depth: 3,
                address: 35,
            }), // #17
            Instruction::Control(Control::Break {
                option_block_index: Some(2),
                relative_depth: 3,
                address: 35,
            }), // #18
            Instruction::Control(Control::End(Some(2))),                  // #19 - block 2 end
            Instruction::Control(Control::Recur {
                block_index: 1,
                relative_depth: 0,
                address: 8,
            }), // #20
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 1,
                address: 28,
            }), // #21
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 2,
                address: 35,
            }), // #22
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 2,
                address: 35,
            }), // #23
            Instruction::Control(Control::End(Some(1))),                  // #24 - block 1 end
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 0,
                address: 28,
            }), // #25
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 35,
            }), // #26
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 35,
            }), // #27
            Instruction::Control(Control::End(Some(0))),                  // #28 - block 0 end
            Instruction::Sequence(instruction::Instruction::I32Const(2)), // #29
            Instruction::Sequence(instruction::Instruction::I32Const(3)), // #30
            Instruction::Control(Control::Break {
                option_block_index: None,
                relative_depth: 0,
                address: 35,
            }), // #31
            Instruction::Control(Control::Break {
                option_block_index: None,
                relative_depth: 0,
                address: 35,
            }), // #32
            Instruction::Sequence(instruction::Instruction::I32Const(4)), // #33
            Instruction::Sequence(instruction::Instruction::I32Const(5)), // #34
            Instruction::Control(Control::End(None)),                     // #35
            // function 3
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #36
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #37
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 0,
                end_address: 80,
            }), // #38 - block 0
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 1,
                end_address: 42,
            }), // #39 - block 1 loop
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 2,
                end_address: 41,
            }), // #40 - block 2
            Instruction::Control(Control::End(Some(2))),                  // #41 - block 2 end
            Instruction::Control(Control::End(Some(1))),                  // #42 - block 1 end
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 3,
                end_address: 77,
            }), // #43 - block 3
            Instruction::Control(Control::Break {
                option_block_index: Some(3),
                relative_depth: 0,
                address: 77,
            }), // #44
            Instruction::Control(Control::Break {
                option_block_index: Some(3),
                relative_depth: 1,
                address: 80,
            }), // #45
            Instruction::Control(Control::Break {
                option_block_index: Some(3),
                relative_depth: 2,
                address: 85,
            }), // #46
            Instruction::Control(Control::BlockAndJumpWhenEqZero {
                block_type: BlockType::ResultEmpty,
                block_index: 4,
                option_alternate_address: Some(60),
                end_address: 73,
            }), // #47 - block 4 if
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 0,
                address: 73,
            }), // #48
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 1,
                address: 77,
            }), // #49
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 2,
                address: 80,
            }), // #50
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 3,
                address: 85,
            }), // #51
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 5,
                end_address: 59,
            }), // #52 - block 5
            Instruction::Control(Control::Break {
                option_block_index: Some(5),
                relative_depth: 0,
                address: 59,
            }), // #53
            Instruction::Control(Control::Break {
                option_block_index: Some(5),
                relative_depth: 1,
                address: 73,
            }), // #54
            Instruction::Control(Control::Break {
                option_block_index: Some(5),
                relative_depth: 2,
                address: 77,
            }), // #55
            Instruction::Control(Control::Break {
                option_block_index: Some(5),
                relative_depth: 3,
                address: 80,
            }), // #56
            Instruction::Control(Control::Break {
                option_block_index: Some(5),
                relative_depth: 4,
                address: 85,
            }), // #57 - jump to function end
            Instruction::Control(Control::Break {
                option_block_index: Some(5),
                relative_depth: 4,
                address: 85,
            }), // #58
            Instruction::Control(Control::End(Some(5))),                  // #59 - block 5 end
            Instruction::Control(Control::JumpWithinBlock(73)),                      // #60 - else
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 0,
                address: 73,
            }), // #61
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 1,
                address: 77,
            }), // #62
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 2,
                address: 80,
            }), // #63
            Instruction::Control(Control::Break {
                option_block_index: Some(4),
                relative_depth: 3,
                address: 85,
            }), // #64
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 6,
                end_address: 72,
            }), // #65 - block 6
            Instruction::Control(Control::Break {
                option_block_index: Some(6),
                relative_depth: 0,
                address: 72,
            }), // #66
            Instruction::Control(Control::Break {
                option_block_index: Some(6),
                relative_depth: 1,
                address: 73,
            }), // #67
            Instruction::Control(Control::Break {
                option_block_index: Some(6),
                relative_depth: 2,
                address: 77,
            }), // #68
            Instruction::Control(Control::Break {
                option_block_index: Some(6),
                relative_depth: 3,
                address: 80,
            }), // #69
            Instruction::Control(Control::Break {
                option_block_index: Some(6),
                relative_depth: 4,
                address: 85,
            }), // #70 - jump to function end
            Instruction::Control(Control::Break {
                option_block_index: Some(6),
                relative_depth: 4,
                address: 85,
            }), // #71
            Instruction::Control(Control::End(Some(6))),                  // #72 - block 6 end
            Instruction::Control(Control::End(Some(4))),                  // #73 // block 4 end
            Instruction::Control(Control::Break {
                option_block_index: Some(3),
                relative_depth: 0,
                address: 77,
            }), // #74
            Instruction::Control(Control::Break {
                option_block_index: Some(3),
                relative_depth: 1,
                address: 80,
            }), // #75
            Instruction::Control(Control::Break {
                option_block_index: Some(3),
                relative_depth: 2,
                address: 85,
            }), // #76
            Instruction::Control(Control::End(Some(3))),                  // #77 // block 3 end
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 0,
                address: 80,
            }), // #78
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 85,
            }), // #79
            Instruction::Control(Control::End(Some(0))),                  // #80 // block 0 end
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #81
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #82
            Instruction::Control(Control::Break {
                option_block_index: None,
                relative_depth: 0,
                address: 85,
            }), // #83
            Instruction::Control(Control::Break {
                option_block_index: None,
                relative_depth: 0,
                address: 85,
            }), // #84
            Instruction::Control(Control::End(None)),                     // #85
            // function 4
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #86
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #87
            Instruction::Control(Control::BlockAndJumpWhenEqZero {
                block_type: BlockType::ResultEmpty,
                block_index: 0,
                option_alternate_address: None,
                end_address: 101,
            }), // #88 - block 0
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 0,
                address: 101,
            }), // #89
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 104,
            }), // #90
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 104,
            }), // #91
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 1,
                end_address: 97,
            }), // #92 - block 1
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 0,
                address: 97,
            }), // #93
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 1,
                address: 101,
            }), // #94
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 2,
                address: 104,
            }), // #95
            Instruction::Control(Control::Break {
                option_block_index: Some(1),
                relative_depth: 2,
                address: 104,
            }), // #96
            Instruction::Control(Control::End(Some(1))),                  // #97 - block 1 end
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 0,
                address: 101,
            }), // #98
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 104,
            }), // #99
            Instruction::Control(Control::Break {
                option_block_index: Some(0),
                relative_depth: 1,
                address: 104,
            }), // #100
            Instruction::Control(Control::End(Some(0))),                  // #101 - block 0 end
            Instruction::Sequence(instruction::Instruction::I32Const(2)), // #102
            Instruction::Sequence(instruction::Instruction::I32Const(3)), // #103
            Instruction::Control(Control::End(None)),                     // #104
        ]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_block_branch_if() {
        let named_ast_modules: Vec<NamedAstModule> = vec![create_simple_test_ast_module(
            "m0",
            vec![TypeItem::FunctionType(FunctionType {
                params: vec![ValueType::I32],
                results: vec![ValueType::I32],
            })],
            vec![0],
            vec![CodeItem {
                local_groups: vec![],
                instruction_items: vec![
                    // 创建如下的结构块
                    // 测试 `block 结构块` 和 `loop 结构块`
                    //
                    // |  0--block-start
                    // |  |  1--loop-start
                    // |  |  |  2--block-start
                    // |  |  |  2--block-end
                    // |  |  1--loop-end
                    // |  0--block-end
                    instruction::Instruction::I32Const(0), // #00
                    instruction::Instruction::Block(BlockType::ResultEmpty, 0), // #01 - block 0
                    instruction::Instruction::I32Const(1), // #02
                    instruction::Instruction::Loop(BlockType::ResultEmpty, 1), // #03 - block 1 loop
                    instruction::Instruction::I32Const(2), // #04
                    instruction::Instruction::Block(BlockType::ResultEmpty, 2), // #05 - block 2
                    instruction::Instruction::I32Const(3), // #06
                    instruction::Instruction::BrIf(0),     // #07 jump to `block 2 end`
                    instruction::Instruction::BrIf(1),     // #08 jump to `block 1 loop`
                    instruction::Instruction::BrIf(2),     // #09 jump to `block 0 end`
                    instruction::Instruction::BrIf(3),     // #10 jump to `function end`
                    instruction::Instruction::End,         // #11 - block 2 end
                    instruction::Instruction::End,         // #12 - block 1 end
                    instruction::Instruction::End,         // #13 - block 0 end
                    instruction::Instruction::End,         // #14
                ],
            }],
        )];

        let actual = link_and_decode_function_instructions(&vec![], &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function 0
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 0,
                end_address: 13,
            }), // #01 - block 0
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #02
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 1,
                end_address: 12,
            }), // #03 - block 1 - loop
            Instruction::Sequence(instruction::Instruction::I32Const(2)), // #04
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 2,
                end_address: 11,
            }), // #05 - block 2
            Instruction::Sequence(instruction::Instruction::I32Const(3)), // #06
            Instruction::Control(Control::BreakWhenNotEqZero {
                option_block_index: Some(2),
                relative_depth: 0,
                address: 11,
            }), // #07 jump to `block 2 end`
            Instruction::Control(Control::RecurWhenNotEqZero {
                block_index: 2,
                relative_depth: 1,
                address: 3,
            }), // #08 jump to `block 1 loop`
            Instruction::Control(Control::BreakWhenNotEqZero {
                option_block_index: Some(2),
                relative_depth: 2,
                address: 13,
            }), // #09 jump to `block 0 end`
            Instruction::Control(Control::BreakWhenNotEqZero {
                option_block_index: Some(2),
                relative_depth: 3,
                address: 14,
            }), // #10 jump to `function end`
            Instruction::Control(Control::End(Some(2))),                  // #11 - block 2 end
            Instruction::Control(Control::End(Some(1))),                  // #12 - block 1 end
            Instruction::Control(Control::End(Some(0))),                  // #13 - block 0 end
            Instruction::Control(Control::End(None)),                     // #14
        ]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_block_branch() {
        let named_ast_modules: Vec<NamedAstModule> = vec![create_simple_test_ast_module(
            "m0",
            vec![TypeItem::FunctionType(FunctionType {
                params: vec![ValueType::I32],
                results: vec![ValueType::I32],
            })],
            vec![0],
            vec![CodeItem {
                local_groups: vec![],
                instruction_items: vec![
                    // 创建如下的结构块
                    // 测试 `block 结构块` 和 `loop 结构块`
                    //
                    // |  0--block-start
                    // |  |  1--loop-start
                    // |  |  |  2--block-start
                    // |  |  |  2--block-end
                    // |  |  1--loop-end
                    // |  0--block-end
                    instruction::Instruction::I32Const(0), // #00
                    instruction::Instruction::Block(BlockType::ResultEmpty, 0), // #01 - block 0
                    instruction::Instruction::I32Const(1), // #02
                    instruction::Instruction::Loop(BlockType::ResultEmpty, 1), // #03 - block 1 loop
                    instruction::Instruction::I32Const(2), // #04
                    instruction::Instruction::Block(BlockType::ResultEmpty, 2), // #05 - block 2
                    instruction::Instruction::I32Const(3), // #06
                    // jump to [`block 0 end`, `block 2 end`, `loop 1 start`], `function end`
                    instruction::Instruction::BrTable(vec![2, 0, 1], 3), // # 07
                    instruction::Instruction::End,                       // #08 - block 2 end
                    instruction::Instruction::End,                       // #09 - block 1 end
                    instruction::Instruction::End,                       // #10 - block 0 end
                    instruction::Instruction::End,                       // #11
                ],
            }],
        )];

        let actual = link_and_decode_function_instructions(&vec![], &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function 0
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 0,
                end_address: 10,
            }), // #01 - block 0
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #02
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 1,
                end_address: 9,
            }), // #03 - block 1 - loop
            Instruction::Sequence(instruction::Instruction::I32Const(2)), // #04
            Instruction::Control(Control::Block {
                block_type: BlockType::ResultEmpty,
                block_index: 2,
                end_address: 8,
            }), // #05 - block 2
            Instruction::Sequence(instruction::Instruction::I32Const(3)), // #06
            // jump to [`block 0 end`, `block 2 end`, `loop 1 start`], `function end`
            Instruction::Control(Control::Branch {
                option_block_index: Some(2),
                branch_targets: vec![
                    BranchTarget::Break(2, 10),
                    BranchTarget::Break(0, 8),
                    BranchTarget::Recur(1, 3),
                ],
                default_branch_target: BranchTarget::Break(3, 11),
            }), // #07
            Instruction::Control(Control::End(Some(2))), // #08 - block 2 end
            Instruction::Control(Control::End(Some(1))), // #09 - block 1 end
            Instruction::Control(Control::End(Some(0))), // #10 - block 0 end
            Instruction::Control(Control::End(None)),    // #11
        ]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_module_internal() {
        let named_ast_modules: Vec<NamedAstModule> = vec![create_simple_test_ast_module(
            "m0",
            vec![TypeItem::FunctionType(FunctionType {
                params: vec![ValueType::I32],
                results: vec![ValueType::I32],
            })],
            vec![0, 0, 0, 0],
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
                        instruction::Instruction::End,         // #12
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        instruction::Instruction::I32Const(2),        // #13
                        instruction::Instruction::CallIndirect(1, 0), // #14
                        instruction::Instruction::End,                // #15
                    ],
                },
            ],
        )];

        let actual = link_and_decode_function_instructions(&vec![], &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function 0
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #01
            Instruction::Control(Control::Call {
                vm_module_index: 0,
                type_index: 0,
                function_index: 1,
                internal_function_index: 1,
                address: 7,
            }), // #02
            Instruction::Sequence(instruction::Instruction::I32Const(10)), // #03
            Instruction::Control(Control::Call {
                vm_module_index: 0,
                type_index: 0,
                function_index: 2,
                internal_function_index: 2,
                address: 10,
            }), // #04
            Instruction::Sequence(instruction::Instruction::I32Const(11)), // #05
            Instruction::Control(Control::End(None)),                     // #06
            // function 1
            Instruction::Sequence(instruction::Instruction::I32Const(2)), // #07
            Instruction::Sequence(instruction::Instruction::I32Const(3)), // #08
            Instruction::Control(Control::End(None)),                     // #09
            // function 2
            Instruction::Sequence(instruction::Instruction::I32Const(4)), // #10
            Instruction::Sequence(instruction::Instruction::I32Const(5)), // #11
            Instruction::Control(Control::End(None)),                     // #12
            // function 3
            Instruction::Sequence(instruction::Instruction::I32Const(2)), // #13
            Instruction::Control(Control::CallIndirect {
                type_index: 1,
                table_index: 0,
            }), // #14
            Instruction::Control(Control::End(None)),                     // #15
        ]];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_module_external() {
        let named_ast_modules: Vec<NamedAstModule> = vec![
            create_test_ast_module(
                "m0",
                vec![TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![ValueType::I32],
                })],
                vec![0, 0],
                vec![],
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
                vec![TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![ValueType::I32],
                })],
                vec![0, 0],
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

        let actual = link_and_decode_function_instructions(&vec![], &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![
            vec![
                // function 0
                Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
                Instruction::Sequence(instruction::Instruction::I32Const(1)), // #01
                Instruction::Control(Control::End(None)),                     // #02
                // function 1
                Instruction::Sequence(instruction::Instruction::I32Const(2)), // #03
                Instruction::Sequence(instruction::Instruction::I32Const(3)), // #04
                Instruction::Control(Control::Call {
                    vm_module_index: 0,
                    type_index: 0,
                    function_index: 0,
                    internal_function_index: 0,
                    address: 0,
                }), // #05
                Instruction::Control(Control::End(None)),                     // #06
            ],
            vec![
                // function index 2
                Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
                Instruction::Sequence(instruction::Instruction::I32Const(1)), // #01
                Instruction::Control(Control::Call {
                    vm_module_index: 0,
                    type_index: 0,
                    function_index: 0,
                    internal_function_index: 0,
                    address: 0,
                }), // #02
                Instruction::Sequence(instruction::Instruction::I32Const(10)), // #03
                Instruction::Control(Control::Call {
                    vm_module_index: 0,
                    type_index: 0,
                    function_index: 1,
                    internal_function_index: 1,
                    address: 3,
                }), // #04
                Instruction::Sequence(instruction::Instruction::I32Const(11)), // #05
                Instruction::Control(Control::Call {
                    vm_module_index: 1,
                    type_index: 0,
                    function_index: 3,
                    internal_function_index: 1,
                    address: 9,
                }), // #06
                Instruction::Sequence(instruction::Instruction::I32Const(12)), // #07
                Instruction::Control(Control::End(None)),                     // #08
                // function index 3
                Instruction::Sequence(instruction::Instruction::I32Const(2)), // #09
                Instruction::Sequence(instruction::Instruction::I32Const(3)), // #10
                Instruction::Control(Control::Call {
                    vm_module_index: 0,
                    type_index: 0,
                    function_index: 0,
                    internal_function_index: 0,
                    address: 0,
                }), // #11
                Instruction::Sequence(instruction::Instruction::I32Const(20)), // #12
                Instruction::Control(Control::Call {
                    vm_module_index: 0,
                    type_index: 0,
                    function_index: 1,
                    internal_function_index: 1,
                    address: 3,
                }), // #13
                Instruction::Sequence(instruction::Instruction::I32Const(21)), // #14
                Instruction::Control(Control::End(None)),                     // #15
            ],
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_call_module_native() {
        let native_modules = vec![create_test_native_module()];

        let named_ast_modules: Vec<NamedAstModule> = vec![create_test_ast_module(
            "m1",
            vec![TypeItem::FunctionType(FunctionType {
                params: vec![ValueType::I32, ValueType::I32],
                results: vec![ValueType::I32],
            })],
            vec![0, 0],
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

        let actual =
            link_and_decode_function_instructions(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![vec![
            // function index 2
            Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
            Instruction::Sequence(instruction::Instruction::I32Const(1)), // #01
            Instruction::Control(Control::CallNative {
                native_module_index: 0,
                type_index: 0,
                function_index: 0,
            }), // #02
            Instruction::Sequence(instruction::Instruction::I32Const(10)), // #03
            Instruction::Control(Control::CallNative {
                native_module_index: 0,
                type_index: 0,
                function_index: 1,
            }), // #04
            Instruction::Sequence(instruction::Instruction::I32Const(11)), // #05
            Instruction::Control(Control::Call {
                vm_module_index: 0,
                type_index: 0,
                function_index: 3,
                internal_function_index: 1,
                address: 9,
            }), // #06
            Instruction::Sequence(instruction::Instruction::I32Const(12)), // #07
            Instruction::Control(Control::End(None)),                     // #08
            // function index 3
            Instruction::Sequence(instruction::Instruction::I32Const(2)), // #09
            Instruction::Sequence(instruction::Instruction::I32Const(3)), // #10
            Instruction::Control(Control::CallNative {
                native_module_index: 0,
                type_index: 0,
                function_index: 0,
            }), // #11
            Instruction::Sequence(instruction::Instruction::I32Const(20)), // #12
            Instruction::Control(Control::CallNative {
                native_module_index: 0,
                type_index: 0,
                function_index: 1,
            }), // #13
            Instruction::Sequence(instruction::Instruction::I32Const(21)), // #14
            Instruction::Control(Control::End(None)),                     // #15
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

        let native_modules = vec![create_test_native_module()];

        let named_ast_modules: Vec<NamedAstModule> = vec![
            create_test_ast_module(
                "m1",
                vec![TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![],
                })],
                vec![0, 0],
                vec![],
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
                    TypeItem::FunctionType(FunctionType {
                        params: vec![ValueType::I32, ValueType::I32],
                        results: vec![ValueType::I32],
                    }),
                    TypeItem::FunctionType(FunctionType {
                        params: vec![],
                        results: vec![],
                    }),
                    TypeItem::FunctionType(FunctionType {
                        params: vec![ValueType::I32],
                        results: vec![ValueType::I32],
                    }),
                ],
                vec![2, 2],
                vec![
                    ImportItem {
                        // native 0, func 0,
                        module_name: "m0".to_string(),
                        item_name: "add".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                    ImportItem {
                        // ast 0, func 1, inter 1
                        module_name: "m1".to_string(),
                        item_name: "bottom".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(1),
                    },
                ],
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
                        // ast 1, func 3, inter 1
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
                    // type for native function 'add'/'f0'
                    TypeItem::FunctionType(FunctionType {
                        params: vec![ValueType::I32, ValueType::I32],
                        results: vec![ValueType::I32],
                    }),
                    // type for function 'bottom'/'f1'
                    TypeItem::FunctionType(FunctionType {
                        params: vec![],
                        results: vec![],
                    }),
                    // type for function 'f2'
                    TypeItem::FunctionType(FunctionType {
                        params: vec![ValueType::I32],
                        results: vec![ValueType::I32],
                    }),
                    // type for internal function index 3
                    TypeItem::FunctionType(FunctionType {
                        params: vec![ValueType::I32],
                        results: vec![],
                    }),
                ],
                vec![3],
                vec![
                    ImportItem {
                        // native 0, func 0,
                        module_name: "m2".to_string(),
                        item_name: "f0".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(0),
                    },
                    ImportItem {
                        // ast 0, func 1, inter 1
                        module_name: "m2".to_string(),
                        item_name: "f1".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(1),
                    },
                    ImportItem {
                        // ast 1, func 3, inter 1
                        module_name: "m2".to_string(),
                        item_name: "f2".to_string(),
                        import_descriptor: ast::ImportDescriptor::FunctionTypeIndex(2),
                    },
                ],
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
                        instruction::Instruction::Call(3),     // #07
                        instruction::Instruction::End,         // #08
                    ],
                }],
            ),
        ];

        let actual =
            link_and_decode_function_instructions(&native_modules, &named_ast_modules).unwrap();
        let expected: Vec<Vec<Instruction>> = vec![
            vec![
                // function index 0
                Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
                Instruction::Control(Control::End(None)),                     // #01
                // function index 1
                Instruction::Sequence(instruction::Instruction::I32Const(1)), // #02
                Instruction::Control(Control::End(None)),                     // #03
            ],
            vec![
                // function index 2
                Instruction::Sequence(instruction::Instruction::I32Const(2)), // #00
                Instruction::Sequence(instruction::Instruction::Drop),        // #01
                Instruction::Control(Control::End(None)),                     // #02
                // function index 3
                Instruction::Sequence(instruction::Instruction::I32Const(3)), // #03
                Instruction::Control(Control::End(None)),                     // #04
            ],
            vec![
                // function index 3
                Instruction::Sequence(instruction::Instruction::I32Const(0)), // #00
                Instruction::Control(Control::CallNative {
                    native_module_index: 0,
                    type_index: 0,
                    function_index: 0,
                }), // #01
                Instruction::Sequence(instruction::Instruction::I32Const(1)), // #02
                Instruction::Control(Control::Call {
                    vm_module_index: 0,
                    type_index: 0,
                    function_index: 1,
                    internal_function_index: 1,
                    address: 2,
                }), // #03
                Instruction::Sequence(instruction::Instruction::I32Const(2)), // #04
                Instruction::Control(Control::Call {
                    vm_module_index: 1,
                    type_index: 2,
                    function_index: 3,
                    internal_function_index: 1,
                    address: 3,
                }), // #05
                Instruction::Control(Control::End(None)),                     // #06
                Instruction::Control(Control::Call {
                    vm_module_index: 2,
                    type_index: 3,
                    function_index: 3,
                    internal_function_index: 0,
                    address: 0,
                }), // #07
                Instruction::Control(Control::End(None)),                     // #08
            ],
        ];

        assert_eq!(actual, expected);
    }
}
