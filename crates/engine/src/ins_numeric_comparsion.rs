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

use anvm_ast::types::Value;

use crate::{
    error::{make_invalid_operand_data_types_engine_error, EngineError},
    vm::VM,
};

// i32

pub fn i32_eq(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.eq", "i32")),
    }
}

pub fn i32_ne(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.ne", "i32")),
    }
}

pub fn i32_lt_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.lt_s", "i32")),
    }
}

pub fn i32_lt_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool((left as u32) < (right as u32));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.lt_u", "i32")),
    }
}

pub fn i32_gt_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.gt_s", "i32")),
    }
}

pub fn i32_gt_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool((left as u32) > (right as u32));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.gt_u", "i32")),
    }
}

pub fn i32_le_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.le_s", "i32")),
    }
}

pub fn i32_le_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool((left as u32) <= (right as u32));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.le_u", "i32")),
    }
}

pub fn i32_ge_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.ge_s", "i32")),
    }
}

pub fn i32_ge_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push_bool((left as u32) >= (right as u32));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i32.ge_u", "i32")),
    }
}

// i64

pub fn i64_eq(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.eq", "i64")),
    }
}

pub fn i64_ne(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.ne", "i64")),
    }
}

pub fn i64_lt_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.lt_s", "i64")),
    }
}

pub fn i64_lt_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool((left as u64) < (right as u64));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.lt_u", "i64")),
    }
}

pub fn i64_gt_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.gt_s", "i64")),
    }
}

pub fn i64_gt_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool((left as u64) > (right as u64));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.gt_u", "i64")),
    }
}

pub fn i64_le_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.le_s", "i64")),
    }
}

pub fn i64_le_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool((left as u64) <= (right as u64));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.le_u", "i64")),
    }
}

pub fn i64_ge_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.ge_s", "i64")),
    }
}

pub fn i64_ge_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push_bool((left as u64) >= (right as u64));
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("i64.ge_u", "i64")),
    }
}

// f32

pub fn f32_eq(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f32.eq", "f32")),
    }
}

pub fn f32_ne(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f32.ne", "f32")),
    }
}

pub fn f32_lt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f32.lt", "f32")),
    }
}

pub fn f32_gt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f32.gt", "f32")),
    }
}

pub fn f32_le(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f32.le", "f32")),
    }
}

pub fn f32_ge(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f32.ge", "f32")),
    }
}

// f64

pub fn f64_eq(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push_bool(left == right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f64.eq", "f64")),
    }
}

pub fn f64_ne(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push_bool(left != right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f64.ne", "f64")),
    }
}

pub fn f64_lt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push_bool(left < right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f64.lt", "f64")),
    }
}

pub fn f64_gt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push_bool(left > right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f64.gt", "f64")),
    }
}

pub fn f64_le(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push_bool(left <= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f64.le", "f64")),
    }
}

pub fn f64_ge(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push_bool(left >= right);
            Ok(())
        }
        _ => Err(make_invalid_operand_data_types_engine_error("f64.ge", "f64")),
    }
}
