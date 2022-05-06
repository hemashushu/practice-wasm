// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 二元运算指令
//!
//! 从栈顶弹出 2 个操作数，经过运算后压入栈
//! 注意先弹出的作为 RHS，后弹出的作为 LHS。
//!
//! -- 栈顶。--
//! Right-hand-side value
//! Left-hand-size value
//! -- 栈底。--
//!
//! i32.add
//! i32.sub
//! i32.mul
//! i32.div_s
//! i32.div_u
//! i32.rem_s
//! i32.rem_u
//!
//! i32.and
//! i32.or
//! i32.xor
//! i32.shl         ;; 左移
//! i32.shr_s       ;; 算术右移
//! i32.shr_u       ;; 逻辑右移
//!                 ;; 移位操作可以先对第二个操作数进行一次求余运算，以防止错误
//! i32.rotl        ;; 左旋，即左移，然后将溢出的比特数拼接到低端
//!                 ;; 8'b1110_0000 左旋 1 的结果是 8'b1100_0001
//!                 ;; https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Left_rotate
//! i32.rotr        ;; 右旋
//!                 ;; 注意位移动和旋转的右手边操作数只能是 i32
//!
//! i64 有跟 i32 一样的二元运算指令
//!
//! f32.add
//! f32.sub
//! f32.mul
//! f32.div
//!
//! f32.min
//! f32.max
//! f32.copysign    ;; 应用位于栈顶的操作数（稍后会弹出）的符号到从栈顶开始位于第二位的操作数
//!
//! f64 有跟 f32 一样的二元运算指令

use std::{cell::RefCell, rc::Rc};

use anvm_parser::types::Value;

use crate::{instance::EngineError, vm_module::VMModule};

// i32

pub fn i32_add(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left + right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.add\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_sub(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left - right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.sub\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_mul(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left * right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.mul\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_div_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left / right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.div_s\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_div_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push(Value::I32(((left as u32) / (right as u32)) as i32));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.div_u\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_rem_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left % right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.rem_s\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_rem_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push(Value::I32(((left as u32) % (right as u32)) as i32));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.rem_u\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_and(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left & right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.and\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_or(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left | right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.or\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_xor(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left ^ right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.xor\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_shl(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left << right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.shl\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_shr_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push(Value::I32(left >> right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.shr_s\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_shr_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push(Value::I32(((left as u32) >> right) as i32));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.shr_u\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_rotl(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push(Value::I32(i32::rotate_left(left, right as u32)));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.rotl\" should be \"i32\""
                .to_string(),
        )),
    }
}

pub fn i32_rotr(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push(Value::I32(i32::rotate_right(left, right as u32)));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.rotr\" should be \"i32\""
                .to_string(),
        )),
    }
}

// i64

pub fn i64_add(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left + right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.add\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_sub(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left - right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.sub\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_mul(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left * right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.mul\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_div_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left / right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.div_s\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_div_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module
                .operand_stack
                .push(Value::I64(((left as u64) / (right as u64)) as i64));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.div_u\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_rem_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left % right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.rem_s\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_rem_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module
                .operand_stack
                .push(Value::I64(((left as u64) % (right as u64)) as i64));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.rem_u\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_and(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left & right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.and\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_or(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left | right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.or\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_xor(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push(Value::I64(left ^ right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.xor\" should be \"i64\""
                .to_string(),
        )),
    }
}

pub fn i64_shl(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            module.operand_stack.push(Value::I64(left << right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.shl\" should be \"i64\" and \"i32\""
                .to_string(),
        )),
    }
}

pub fn i64_shr_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            module.operand_stack.push(Value::I64(left >> right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.shr_s\" should be \"i64\" and \"i32\""
                .to_string(),
        )),
    }
}

pub fn i64_shr_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            module
                .operand_stack
                .push(Value::I64(((left as u64) >> right) as i64));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.shr_u\" should be \"i64\" and \"i32\""
                .to_string(),
        )),
    }
}

pub fn i64_rotl(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            module
                .operand_stack
                .push(Value::I64(i64::rotate_left(left, right as u32)));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.rotl\" should be \"i64\" and \"i32\""
                .to_string(),
        )),
    }
}

pub fn i64_rotr(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            module
                .operand_stack
                .push(Value::I64(i64::rotate_right(left, right as u32)));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.rotr\" should be \"i64\" and \"i32\""
                .to_string(),
        )),
    }
}

// f32

pub fn f32_add(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push(Value::F32(left + right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.add\" should be \"f32\""
                .to_string(),
        )),
    }
}

pub fn f32_sub(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push(Value::F32(left - right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.sub\" should be \"f32\""
                .to_string(),
        )),
    }
}

pub fn f32_mul(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push(Value::F32(left * right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.mul\" should be \"f32\""
                .to_string(),
        )),
    }
}

pub fn f32_div(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push(Value::F32(left / right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.div\" should be \"f32\""
                .to_string(),
        )),
    }
}

pub fn f32_min(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            if left < right {
                module.operand_stack.push(lhs)
            } else {
                module.operand_stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.min\" should be \"f32\""
                .to_string(),
        )),
    }
}

pub fn f32_max(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            if left > right {
                module.operand_stack.push(lhs)
            } else {
                module.operand_stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.max\" should be \"f32\""
                .to_string(),
        )),
    }
}

pub fn f32_copysign(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module
                .operand_stack
                .push(Value::F32(f32::copysign(right, left)));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.copysign\" should be \"f32\""
                .to_string(),
        )),
    }
}

// f64

pub fn f64_add(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push(Value::F64(left + right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.add\" should be \"f64\""
                .to_string(),
        )),
    }
}

pub fn f64_sub(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push(Value::F64(left - right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.sub\" should be \"f64\""
                .to_string(),
        )),
    }
}

pub fn f64_mul(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push(Value::F64(left * right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.mul\" should be \"f64\""
                .to_string(),
        )),
    }
}

pub fn f64_div(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push(Value::F64(left / right));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.div\" should be \"f64\""
                .to_string(),
        )),
    }
}

pub fn f64_min(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            if left < right {
                module.operand_stack.push(lhs)
            } else {
                module.operand_stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.min\" should be \"f64\""
                .to_string(),
        )),
    }
}

pub fn f64_max(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            if left > right {
                module.operand_stack.push(lhs)
            } else {
                module.operand_stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.max\" should be \"f64\""
                .to_string(),
        )),
    }
}

pub fn f64_copysign(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module
                .operand_stack
                .push(Value::F64(f64::copysign(right, left)));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.copysign\" should be \"f64\""
                .to_string(),
        )),
    }
}
