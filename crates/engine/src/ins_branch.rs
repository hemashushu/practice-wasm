// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 流程控制指令
//!
//! - block
//! - loop
//! - if
//! - else
//! - end
//!
//! # 分支/跳转指令
//!
//! - br
//! - br_if
//! - br_table
//! - return
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

use std::{cell::RefCell, rc::Rc};

use anvm_parser::{
    ast::FunctionType,
    instruction::{BlockType, Instruction},
    types::{Value, ValueType},
};

use crate::{
    instance::EngineError,
    vm_control_stack::VMFrameType,
    vm_module::{enter_control_block, leave_control_block, repeat_control_block, VMModule},
};

pub fn exec_block(
    vm_module: Rc<RefCell<VMModule>>,
    block_type: &BlockType,
    body: &Rc<Vec<Instruction>>,
) -> Result<(), EngineError> {
    let function_type = get_block_function_type(&vm_module, block_type);
    enter_control_block(
        vm_module,
        VMFrameType::Block,
        &function_type,
        body,
        0, // `block 流程控制结构块` 没有自己的局部变量空槽
    );
    Ok(())
}

pub fn exec_loop(
    vm_module: Rc<RefCell<VMModule>>,
    block_type: &BlockType,
    body: &Rc<Vec<Instruction>>,
) -> Result<(), EngineError> {
    let function_type = get_block_function_type(&vm_module, block_type);
    enter_control_block(
        vm_module,
        VMFrameType::Loop,
        &function_type,
        body,
        0, // `loop 流程控制结构块` 没有自己的局部变量空槽
    );
    Ok(())
}

pub fn exec_if(
    vm_module: Rc<RefCell<VMModule>>,
    block_type: &BlockType,
    consequet_body: &Rc<Vec<Instruction>>,
    alternate_body: &Rc<Vec<Instruction>>,
) -> Result<(), EngineError> {
    // if 指令先从操作数栈顶弹出一个有符号的整数（int32），
    // 非 0 则执行 consequet_body 的指令
    // 等于 0 则执行 alternate_body 的指令
    let testing = {
        let mut module = vm_module.as_ref().borrow_mut();
        module.operand_stack.pop()
    };

    if let Value::I32(value) = testing {
        // if 结构的两个分支共用同一个 block type
        let function_type = get_block_function_type(&vm_module, block_type);

        if value != 0 {
            enter_control_block(
                vm_module,
                VMFrameType::If,
                &function_type,
                consequet_body,
                0, // `if 流程控制结构块` 没有自己的局部变量空槽
            );
        } else {
            enter_control_block(
                vm_module,
                VMFrameType::If,
                &function_type,
                alternate_body,
                0, // `if 流程控制结构块` 没有自己的局部变量空槽
            );
        }

        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"if\" should be \"i32\"".to_string(),
        ))
    }
}

fn get_block_function_type(
    vm_module: &Rc<RefCell<VMModule>>,
    block_type: &BlockType,
) -> Rc<FunctionType> {
    match block_type {
        BlockType::ResultOnly(result) => match result {
            Some(ValueType::I32) => Rc::new(FunctionType {
                params: vec![],
                results: vec![ValueType::I32],
            }),
            Some(ValueType::I64) => Rc::new(FunctionType {
                params: vec![],
                results: vec![ValueType::I64],
            }),
            Some(ValueType::F32) => Rc::new(FunctionType {
                params: vec![],
                results: vec![ValueType::F32],
            }),
            Some(ValueType::F64) => Rc::new(FunctionType {
                params: vec![],
                results: vec![ValueType::F64],
            }),
            None => Rc::new(FunctionType {
                params: vec![],
                results: vec![],
            }),
        },
        BlockType::FunctionTypeIndex(function_type_index) => {
            let rc_function_type = &vm_module.as_ref().borrow().ast_module.function_types
                [*function_type_index as usize];
            Rc::clone(rc_function_type)
        }
    }
}

/// 处理 br 指令
///
/// 在 `流程控制结构块` 里， `br 指令` 的作用跟在 `函数` 里的 `return 指令` 类似
/// 所以这两个指令可以使用同样的处理方式
pub fn br(vm_module: Rc<RefCell<VMModule>>, relative_depth: u32) -> Result<(), EngineError> {
    // 先放心弹出（并抛弃） relative_depth 个控制帧
    // 注：
    // 该数值从 0 开始，当 relative_depth 为 N 时，实际上有 N+1 层
    // 流程控制结构块，先直接弹出前 N 层（包括第 N 层），最后一层则需判断是 block/if 还是 loop
    // 指令，前者则跳到结构末尾，后者则跳到结构开头。

    let frame_type = {
        let mut module = vm_module.as_ref().borrow_mut();

        for _ in 0..relative_depth {
            module.control_stack.pop_frame(); // 直接抛弃
        }

        let stack_frame = module.control_stack.peek_frame();
        stack_frame.frame_type.clone()
    };

    if frame_type == VMFrameType::Loop {
        repeat_control_block(vm_module);
    } else {
        leave_control_block(vm_module);
    }

    Ok(())
}

pub fn br_if(vm_module: Rc<RefCell<VMModule>>, relative_depth: u32) -> Result<(), EngineError> {
    // br_if 指令先从操作数栈顶弹出一个有符号的整数（int32），非 0 则执行 br 操作，
    // 等于 0 则什么都不做（仅仅消耗掉栈顶的一个操作数）
    let testing = {
        let mut module = vm_module.as_ref().borrow_mut();
        module.operand_stack.pop()
    };

    if let Value::I32(value) = testing {
        if value != 0 {
            br(vm_module, relative_depth)
        } else {
            Ok(())
        }
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"br_if\" should be \"i32\"".to_string(),
        ))
    }
}

pub fn br_table(
    vm_module: Rc<RefCell<VMModule>>,
    relative_depths: &[u32],
    default_relative_depth: &u32,
) -> Result<(), EngineError> {
    // br_table 指令先从操作数栈顶弹出一个 uint32 整数，这个数将作为
    // br_table 后面的整数列表的索引，获取跳转的目标。如果该索引超出了
    // 列表范围，则跳转目标的 br_table 指令的最末尾一个参数（即默认目标）

    let br_index = {
        let mut module = vm_module.as_ref().borrow_mut();
        module.operand_stack.pop()
    };

    if let Value::I32(index) = br_index {
        let uindex = index as usize;
        if uindex < relative_depths.len() {
            br(vm_module, relative_depths[uindex])
        } else {
            br(vm_module, *default_relative_depth)
        }
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"br_table\" should be \"i32\"".to_string(),
        ))
    }
}

pub fn exec_return(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let relative_depth = {
        vm_module
            .as_ref()
            .borrow()
            .control_stack
            .get_relative_depth()
    };

    br(vm_module, relative_depth)
}
