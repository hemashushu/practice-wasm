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
    types::{check_types, check_value_types, Value, ValueType, ValueTypeCheckError},
};

use crate::{
    error::{
        make_invalid_operand_data_type_engine_error,
        make_invalid_operand_data_types_2_engine_error,
        make_invalid_operand_data_types_engine_error, make_invalid_table_index_engine_error,
        make_mismatch_function_type_engine_error, EngineError,
    },
    ins_control::ControlResult,
    ins_function::pop_arguments,
    object::FunctionItem,
    vm::VM,
    vm_stack::INFO_SEGMENT_ITEM_COUNT,
};

/// 处理原 `block 指令` 和 `loop 指令`
pub fn block(
    vm: &mut VM,
    block_type: BlockType,
    block_index: usize,
    end_address: usize,
) -> Result<ControlResult, EngineError> {
    // 返回地址应该是结构块最后一条指令 `end 指令` 的下一个指令
    let return_address = end_address + 1;

    // 下一个指令应该是 `block 指令` 的下一个指令
    let next_instruction_address = vm.status.address + 1;

    call_block(
        vm,
        block_type,
        block_index,
        next_instruction_address,
        return_address,
    )
}

/// 处理原 `if 指令`
pub fn block_jump_eq_zero(
    vm: &mut VM,
    block_type: BlockType,
    block_index: usize,
    alternate_address: usize,
    end_address: usize,
) -> Result<ControlResult, EngineError> {
    // 返回地址应该是结构块最后一条指令 `end 指令` 的下一个指令
    let return_address = end_address + 1;

    let testing = vm.stack.pop_bool()?;

    // 下一个指令应该是 `block 指令` 的下一个指令
    let next_instruction_address = if testing {
        vm.status.address + 1
    } else {
        alternate_address
    };

    call_block(
        vm,
        block_type,
        block_index,
        next_instruction_address,
        return_address,
    )
}

fn call_block(
    vm: &mut VM,
    block_type: BlockType,
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
                let function_type = &vm_module.function_types[type_index as usize];
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

    let arguments = pop_arguments(vm, parameter_count);

    // 核对实参的数据类型和数量
    match check_value_types(&arguments, &parameter_types) {
        Err(ValueTypeCheckError::DataTypeMismatch(index)) => {
            return Err(EngineError::InvalidOperation(format!(
            "failed to enter block {} (function {}, module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
            block_index, function_index, vm_module_index, index + 1,
            parameter_types[index],
            arguments[index].get_type())));
        }
        _ => {
            // pass
        }
    }

    // 压入调用栈
    vm.push_control_frame(parameter_count, return_address);

    // 返回新的状态信息，让调用者更新虚拟机状态
    let control_result = ControlResult::FunctionIn {
        is_function_call: false,
        vm_module_index,
        function_index,
        frame_type: block_type,
        address: next_instruction_address,
    };

    Ok(control_result)
}
