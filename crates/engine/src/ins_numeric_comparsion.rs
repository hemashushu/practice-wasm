// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 数值比较测试指令
//!
//! 比较测试包括 `相等测试` 以及 `大小比较测试`
//!
//! 从栈顶弹出 2 个操作数，然后把比较结果（int32，相当于 boolean）压入栈
//! 注意先弹出的作为 RHS，后弹出的作为 LHS。
//!
//! -- 栈顶。--
//! Right-hand-side value
//! Left-hand-size value
//! -- 栈底。--
//!
//! - i32.eq
//! - i32.ne
//! - i32.lt_s
//! - i32.lt_u
//! - i32.gt_s
//! - i32.gt_u
//! - i32.le_s
//! - i32.le_u
//! - i32.ge_s
//! - i32.ge_u
//!
//! 注，i64 有跟 i32 一样的比较指令，这里省略列出
//!
//! - f32.eq
//! - f32.ne
//! - f32.lt
//! - f32.gt
//! - f32.le
//! - f32.ge
//!
//! 注，f64 有跟 f32 一样的比较指令，这里省略列出

use std::{cell::RefCell, rc::Rc};

use anvm_ast::types::Value;

use crate::{object::EngineError, vm_module::VMModule};

// i32

pub fn i32_eq(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.eq\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_ne(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.ne\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_lt_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.lt_s\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_lt_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push_bool((left as u32) < (right as u32));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.lt_u\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_gt_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.gt_s\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_gt_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push_bool((left as u32) > (right as u32));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.gt_u\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_le_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.le_s\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_le_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push_bool((left as u32) <= (right as u32));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.le_u\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_ge_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module.operand_stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.ge_s\" should be \"i32\"".to_string(),
        )),
    }
}

pub fn i32_ge_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            module
                .operand_stack
                .push_bool((left as u32) >= (right as u32));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i32.ge_u\" should be \"i32\"".to_string(),
        )),
    }
}

// i64

pub fn i64_eq(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.eq\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_ne(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.ne\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_lt_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.lt_s\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_lt_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module
                .operand_stack
                .push_bool((left as u64) < (right as u64));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.lt_u\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_gt_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.gt_s\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_gt_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module
                .operand_stack
                .push_bool((left as u64) > (right as u64));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.gt_u\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_le_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.le_s\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_le_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module
                .operand_stack
                .push_bool((left as u64) <= (right as u64));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.le_u\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_ge_s(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module.operand_stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.ge_s\" should be \"i64\"".to_string(),
        )),
    }
}

pub fn i64_ge_u(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            module
                .operand_stack
                .push_bool((left as u64) >= (right as u64));
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"i64.ge_u\" should be \"i64\"".to_string(),
        )),
    }
}

// f32

pub fn f32_eq(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.eq\" should be \"f32\"".to_string(),
        )),
    }
}

pub fn f32_ne(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.ne\" should be \"f32\"".to_string(),
        )),
    }
}

pub fn f32_lt(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.lt\" should be \"f32\"".to_string(),
        )),
    }
}

pub fn f32_gt(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.gt\" should be \"f32\"".to_string(),
        )),
    }
}

pub fn f32_le(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.le\" should be \"f32\"".to_string(),
        )),
    }
}

pub fn f32_ge(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            module.operand_stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f32.ge\" should be \"f32\"".to_string(),
        )),
    }
}

// f64

pub fn f64_eq(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.eq\" should be \"f64\"".to_string(),
        )),
    }
}

pub fn f64_ne(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.ne\" should be \"f64\"".to_string(),
        )),
    }
}

pub fn f64_lt(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.lt\" should be \"f64\"".to_string(),
        )),
    }
}

pub fn f64_gt(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.gt\" should be \"f64\"".to_string(),
        )),
    }
}

pub fn f64_le(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.le\" should be \"f64\"".to_string(),
        )),
    }
}

pub fn f64_ge(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let (rhs, lhs) = (module.operand_stack.pop(), module.operand_stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            module.operand_stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(EngineError::InvalidOperation(
            "the value type of two operands for instruction \"f64.ge\" should be \"f64\"".to_string(),
        )),
    }
}
