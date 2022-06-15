// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 流程控制指令
//!
//! ## br 指令
//!
//! br 指令后面接着 `跳转目标` 的相对深度。
//! 对于 block/if 指令来说，跳转目标是指令的结尾处（即 end 指令），
//! 对于 loop 指令来说，跳转目标是指令的开始处（即 loop 指令）。
//!
//! (func
//!     (block
//!         (i32.const 100)
//!         (br 0)               ;; 跳转目标为 dest_a
//!         (i32.const 101)
//!     )                        ;; 此处是 dest_a
//!     (loop                    ;; 此处是 dest_b
//!         (i32.const 200)
//!         (br 0)               ;; 跳转目标为 dest_b
//!         (i32.const 201)
//!     )
//!     (if (i32.eqz (i32.const 300))
//!         (then (i32.const 400) (br 0) (i32.const 401))    ;; 跳转目标为 dest_b
//!         (else (i32.const 500) (br 0) (i32.const 501))    ;; 跳转目标为 dest_b
//!     )                                                    ;; 此处是 dest_c
//! )
//!
//! relative_depth 从 0 开始，函数本身一层，然后每个嵌套的 block/loop/if 一层，当 relative_depth 为 N 时，
//! 实际上一共有 N+1 层（包括函数本身一层）可以跳。
//!
//! ## br_if 指令
//!
//! br_if 指令先从操作数栈顶弹出一个有符号的整数（int32），非 0 则执行 br 操作，
//! 等于 0 则什么都不做（仅仅消耗掉栈顶的一个操作数）
//!
//! ## if 指令
//!
//! if 指令先从操作数栈顶弹出一个有符号的整数（int32），
//! 非 0 则执行 consequet_body 的指令
//! 等于 0 则执行 alternate_body 的指令
//!
//! ## br_table 指令
//!
//! br_table 指令先从操作数栈顶弹出一个 uint32 整数，这个数将作为
//! br_table 后面的整数列表的索引，获取跳转的目标。如果该索引超出了
//! 列表范围，则跳转目标的 br_table 指令的最末尾一个参数（即默认目标）

use anvm_ast::{
    instruction::BlockType,
    types::{check_value_types, Value, ValueTypeCheckError},
};

use crate::{
    error::{make_invalid_operand_data_type_engine_error, EngineError},
    ins_control::ControlResult,
    object::BranchTarget,
    vm::VM,
    vm_stack::INFO_SEGMENT_ITEM_COUNT,
};

/// 处理原 `block/loop 指令`
pub fn block(
    vm: &mut VM,
    block_type: &BlockType,
    block_index: usize,
    end_address: usize,
) -> Result<ControlResult, EngineError> {
    // 对于 block 结构块
    // 默认的返回地址（即在中间没有遇到 br/return 指令的情况下，一直执行到 end 指令）应该是
    // 结构块最后一条指令 `end 指令` 的下一个指令
    let return_address = end_address + 1;

    // 执行完 `block/loop 指令` 之后的下一个指令应该是当前指令的下一个指令
    let next_instruction_address = vm.status.address + 1;

    continue_process_block(
        vm,
        block_type,
        block_index,
        next_instruction_address,
        return_address,
    )
}

/// 处理原 `if 指令`
pub fn block_and_jump_when_eq_zero(
    vm: &mut VM,
    block_type: &BlockType,
    block_index: usize,
    option_alternate_address: Option<usize>,
    end_address: usize,
) -> Result<ControlResult, EngineError> {
    // 对于 if 结构块
    // 默认的返回地址（即在中间没有遇到 else/br/return 指令的情况下，一直执行到 end 指令）应该是
    // 返回地址应该是结构块最后一条指令 `end 指令` 的下一个指令
    let return_address = end_address + 1;

    let testing = match vm.stack.pop_bool() {
        Ok(b) => b,
        Err(v) => {
            return Err(EngineError::InvalidOperation(
                "expected i32 for bool value".to_string(),
            ))
        }
    };

    // 执行完 `if 指令` 之后，如果刚才栈顶的数值是：
    //
    // - true，则下一个指令应该是 `if 指令` 的下一个指令；
    // - false，则下一个指令应该是：
    //   - 存在 `else 指令` 的话，则跳到 `else 指令` 的下一个指令
    //     注意不要跳到 `else 指令` 本身，因为 `else 指令` 已经被
    //     转换为 `jump 控制指令`，该指令的效果是直接跳到 if 结构块的结束位置
    //   - 不存在 `else 指令` 的话，则跳到结构块之外，即跳到if 结构块的
    //     最后一条 `end 指令` 的下一个指令
    let next_instruction_address = if testing {
        vm.status.address + 1
    } else {
        if let Some(alternate_address) = option_alternate_address {
            alternate_address + 1
        } else {
            end_address + 1
        }
    };

    continue_process_block(
        vm,
        block_type,
        block_index,
        next_instruction_address,
        return_address,
    )
}

fn continue_process_block(
    vm: &mut VM,
    block_type: &BlockType,
    block_index: usize,
    next_instruction_address: usize,
    return_address: usize,
) -> Result<ControlResult, EngineError> {
    let parameter_types = {
        match block_type {
            BlockType::ResultEmpty
            | BlockType::ResultI32
            | BlockType::ResultI64
            | BlockType::ResultF32
            | BlockType::ResultF64 => {
                vec![]
            }
            BlockType::TypeIndex(type_index) => {
                let vm_module_index = vm.status.vm_module_index;
                let vm_module = &vm.resource.vm_modules[vm_module_index];
                let function_type = &vm_module.function_types[*type_index as usize];
                function_type.params.to_owned()
            }
        }
    };

    let function_index = vm.status.function_index;
    let vm_module_index = vm.status.vm_module_index;

    // 判断操作数是否足够当前函数或结构块用于返回
    let parameter_count = parameter_types.len();
    let stack_size = vm.stack.get_size();
    let operands_count = stack_size - vm.status.base_pointer - INFO_SEGMENT_ITEM_COUNT;
    if operands_count < parameter_count {
        return Err(EngineError::InvalidOperation(format!(
            "failed to enter block {} (function {}, module {}), not enough operands, expected: {}, actual: {}",
            block_index, function_index, vm_module_index, parameter_count, operands_count
        )));
    }

    // 从栈弹出数据，作为结构块的参数
    // 跟普通的函数调用栈帧不同，结构块的栈帧没有自己的局部变量段，所以只能
    // 把实参先弹出来，在创建了控制帧之后，再压入栈。
    let arguments = vm.stack.pop_values(parameter_count);

    // 核对实参的数据类型和数量
    match check_value_types(&arguments, &parameter_types) {
        Err(ValueTypeCheckError::DataTypeMismatch(index)) => {
            return Err(EngineError::InvalidOperation(format!(
                "failed to enter block {} (function {}, module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                block_index, function_index, vm_module_index, index,
                parameter_types[index],
                arguments[index].get_type())));
        }
        _ => {
            // pass
        }
    }

    // 压入调用栈
    vm.push_control_frame(return_address);

    // 压入实参
    vm.stack.push_values(&arguments);

    // 返回新的状态信息，让调用者更新虚拟机状态
    let control_result = ControlResult::PushStackFrame {
        is_call_frame: false,
        vm_module_index,
        function_index,
        frame_type: block_type.to_owned(),
        address: next_instruction_address,
    };

    Ok(control_result)
}

pub fn jump(_vm: &mut VM, address: usize) -> Result<ControlResult, EngineError> {
    Ok(ControlResult::JumpWithinBlock { address })
}

pub fn process_break(
    vm: &mut VM,
    option_block_index: Option<usize>,
    relative_depth: usize,
    address: usize,
) -> Result<ControlResult, EngineError> {
    if relative_depth == 0 {
        // 本层的跳转，只需简单地改变下一个指令的地址为结构块的 `end 指令` 位置即可
        Ok(ControlResult::JumpWithinBlock { address })
    } else {
        // 跨层跳转，需要将当前栈帧的操作数作为目标层的返回值，具体步骤：
        // 1. 根据目标层返回值的数量，暂存当前栈帧相应数量的操作数；
        // 2. 弹出 `相对深度` 数量的栈帧；
        // 3. 压入第一步所保留的操作数，作为目标结构块返回值。

        let vm_module_index = vm.status.vm_module_index;
        let function_index = vm.status.function_index;

        let target_status = vm.get_status_by_relative_depth(relative_depth);
        let target_frame_type = &target_status.frame_type;

        // 获取目标帧的返回值的数量
        //
        // 注意
        // - 这里不检查目标帧返回值的数据类型，因为当执行到目标结构块的 `end 指令` 时，
        //   解析器还会进一步检查，所以这里只需获取返回值的数量即可。
        // - 实际上当执行到结构块的 `end 指令` 时，解析器不单会检查返回值的数据类型，
        //   还会再次检查作为返回值的操作数的数量，这个检查跟现在的检查是重复的，
        //   目前这种处理方式主要是为了简化程序的实现。
        let result_count = {
            match target_frame_type {
                BlockType::ResultEmpty => 0,
                BlockType::ResultI32
                | BlockType::ResultI64
                | BlockType::ResultF32
                | BlockType::ResultF64 => 1,
                BlockType::TypeIndex(type_index) => {
                    let vm_module = &vm.resource.vm_modules[vm_module_index];
                    let function_type = &vm_module.function_types[*type_index as usize];
                    function_type.results.len()
                }
            }
        };

        // 判断操作数是否足够当前函数或结构块用于返回
        let stack_size = vm.stack.get_size();
        let operands_count = stack_size - vm.status.base_pointer - INFO_SEGMENT_ITEM_COUNT;
        if operands_count < result_count {
            let message = if let Some(block_index) = option_block_index {
                format!(
                "failed to break block {} to relative depth {} (function {}, module {}), not enough operands, expected: {}, actual: {}",
                block_index, relative_depth, function_index, vm_module_index, result_count, operands_count)
            } else {
                format!(
                "failed to break function {} (module {}), not enough operands, expected: {}, actual: {}",
                function_index, vm_module_index, result_count, operands_count)
            };
            return Err(EngineError::InvalidOperation(message));
        }

        // 丢弃指定数量的栈帧
        vm.pop_frames(relative_depth, result_count);

        Ok(ControlResult::JumpWithinFunction {
            frame_type: target_status.frame_type,
            address,
        })
    }
}

/// 处理原 `br_if` 跳转到 `loop` 结构层的情况
pub fn process_break_when_not_eq_zero(
    vm: &mut VM,
    option_block_index: Option<usize>,
    relative_depth: usize,
    address: usize,
) -> Result<ControlResult, EngineError> {
    let testing = match vm.stack.pop_bool() {
        Ok(b) => b,
        Err(v) => {
            return Err(EngineError::InvalidOperation(
                "expected i32 for bool value".to_string(),
            ))
        }
    };

    if testing {
        process_break(vm, option_block_index, relative_depth, address)
    } else {
        Ok(ControlResult::Sequence)
    }
}

pub fn recur(
    vm: &mut VM,
    block_index: usize,
    relative_depth: usize,
    address: usize,
) -> Result<ControlResult, EngineError> {
    if relative_depth == 0 {
        // 本层的跳转，只需简单地改变下一个指令的地址为结构块的 `loop 指令` 的下一个指令位置即可
        Ok(ControlResult::JumpWithinBlock {
            address: address + 1,
        })
    } else {
        // 跨层跳转，需要将当前栈帧的操作数作为目标层的参数，具体步骤：
        // 1. 根据目标层参数的数量，暂存当前栈帧相应数量的操作数；
        // 2. 弹出 `相对深度` 数量的栈帧；
        // 3. 压入第一步所保留的操作数，作为目标结构块的参数。

        let vm_module_index = vm.status.vm_module_index;
        let function_index = vm.status.function_index;

        let target_status = vm.get_status_by_relative_depth(relative_depth);
        let target_frame_type = &target_status.frame_type;

        // 获取目标帧的参数信息
        let parameter_types = {
            match target_frame_type {
                BlockType::ResultEmpty
                | BlockType::ResultI32
                | BlockType::ResultI64
                | BlockType::ResultF32
                | BlockType::ResultF64 => vec![],
                BlockType::TypeIndex(type_index) => {
                    let vm_module = &vm.resource.vm_modules[vm_module_index];
                    let function_type = &vm_module.function_types[*type_index as usize];
                    function_type.params.to_owned()
                }
            }
        };

        // 判断操作数是否足够当前函数或结构块用于返回
        let parameter_count = parameter_types.len();
        let stack_size = vm.stack.get_size();
        let operands_count = stack_size - vm.status.base_pointer - INFO_SEGMENT_ITEM_COUNT;
        if operands_count < parameter_count {
            return Err(EngineError::InvalidOperation(
                format!(
                    "failed to recur block {} to relative depth {} (function {}, module {}), not enough operands, expected: {}, actual: {}",
                    block_index, relative_depth, function_index, vm_module_index, parameter_count, operands_count)
            ));
        }

        let arguments = vm.stack.peek_values(parameter_count);

        // 核对实参的数据类型和数量
        match check_value_types(arguments, &parameter_types) {
            Err(ValueTypeCheckError::LengthMismatch) => unreachable!(),
            // {
            //     return Err(EngineError::InvalidOperation(format!(
            //         "failed to call function {} (module {}). The number of parameters does not match, expected: {}, actual: {}",
            //         function_index, vm_module_index, parameter_count, arguments.len())));
            // }
            Err(ValueTypeCheckError::DataTypeMismatch(index)) => {
                return Err(EngineError::InvalidOperation(format!(
                    "failed to recur block {} to relative depth {} (function {}, module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                    block_index,relative_depth,  function_index, vm_module_index, index,
                    parameter_types[index],
                    arguments[index].get_type())));
            }
            _ => {
                // pass
            }
        }

        vm.pop_frames(relative_depth, parameter_count);

        Ok(ControlResult::JumpWithinFunction {
            frame_type: target_status.frame_type,
            address: address + 1,
        })
    }
}

/// 处理原 `br_if` 跳转到 `loop` 结构层的情况
pub fn recur_when_not_eq_zero(
    vm: &mut VM,
    block_index: usize,
    relative_depth: usize,
    address: usize,
) -> Result<ControlResult, EngineError> {
    let testing = match vm.stack.pop_bool() {
        Ok(b) => b,
        Err(v) => {
            return Err(EngineError::InvalidOperation(
                "expected i32 for bool value".to_string(),
            ))
        }
    };

    if testing {
        recur(vm, block_index, relative_depth, address)
    } else {
        Ok(ControlResult::Sequence)
    }
}

pub fn branch(
    vm: &mut VM,
    option_block_index: Option<usize>,
    branch_targets: &[BranchTarget],
    default_branch_target: &BranchTarget,
) -> Result<ControlResult, EngineError> {
    let branch_index_value = vm.stack.pop();

    let branch_index = if let Value::I32(branch_index) = branch_index_value {
        branch_index as usize
    } else {
        return Err(make_invalid_operand_data_type_engine_error(
            "br_table", "i32",
        ));
    };

    let actual_branch_target = if branch_index < branch_targets.len() {
        branch_targets[branch_index].clone()
    } else {
        default_branch_target.to_owned()
    };

    match actual_branch_target {
        BranchTarget::Break(relative_depth, address) => {
            process_break(vm, option_block_index, relative_depth, address)
        }
        BranchTarget::Recur(relative_depth, address) => {
            let block_index = option_block_index.unwrap();
            recur(vm, block_index, relative_depth, address)
        }
    }
}
