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

use anvm_ast::types::{Value, ValueType};

use crate::{
    error::{make_operand_data_types_mismatch_engine_error, EngineError},
    vm::VM,
};

// i32

pub fn i32_add(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left + right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.add",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_sub(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left - right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.sub",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_mul(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left * right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.mul",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_div_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left / right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.div_s",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_div_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(((left as u32) / (right as u32)) as i32));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.div_u",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_rem_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left % right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.rem_s",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_rem_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(((left as u32) % (right as u32)) as i32));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.rem_u",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_and(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left & right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.and",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_or(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left | right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.or",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_xor(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left ^ right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.xor",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_shl(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left << right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.shl",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_shr_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(left >> right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.shr_s",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_shr_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(((left as u32) >> right) as i32));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.shr_u",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_rotl(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(i32::rotate_left(left, right as u32)));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.rotl",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i32_rotr(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I32(left), Value::I32(right)) => {
            stack.push(Value::I32(i32::rotate_right(left, right as u32)));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i32.rotr",
            vec![ValueType::I32, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

// i64

pub fn i64_add(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left + right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.add",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_sub(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left - right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.sub",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_mul(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left * right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.mul",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_div_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left / right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.div_s",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_div_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(((left as u64) / (right as u64)) as i64));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.div_u",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_rem_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left % right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.rem_s",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_rem_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(((left as u64) % (right as u64)) as i64));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.rem_u",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_and(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left & right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.and",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_or(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left | right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.or",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_xor(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I64(right)) => {
            stack.push(Value::I64(left ^ right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.xor",
            vec![ValueType::I64, ValueType::I64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_shl(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            stack.push(Value::I64(left << right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.shl",
            vec![ValueType::I64, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_shr_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            stack.push(Value::I64(left >> right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.shr_s",
            vec![ValueType::I64, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_shr_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            stack.push(Value::I64(((left as u64) >> right) as i64));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.shr_u",
            vec![ValueType::I64, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_rotl(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            stack.push(Value::I64(i64::rotate_left(left, right as u32)));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.rotl",
            vec![ValueType::I64, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn i64_rotr(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::I64(left), Value::I32(right)) => {
            // RHS 的类型必须是 i32
            stack.push(Value::I64(i64::rotate_right(left, right as u32)));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "i64.rotr",
            vec![ValueType::I64, ValueType::I32],
            vec![&lhs, &rhs],
        )),
    }
}

// f32

pub fn f32_add(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push(Value::F32(left + right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f32.add",
            vec![ValueType::F32, ValueType::F32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f32_sub(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push(Value::F32(left - right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f32.sub",
            vec![ValueType::F32, ValueType::F32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f32_mul(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push(Value::F32(left * right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f32.mul",
            vec![ValueType::F32, ValueType::F32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f32_div(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push(Value::F32(left / right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f32.div",
            vec![ValueType::F32, ValueType::F32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f32_min(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            if left < right {
                stack.push(lhs)
            } else {
                stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f32.min",
            vec![ValueType::F32, ValueType::F32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f32_max(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            if left > right {
                stack.push(lhs)
            } else {
                stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f32.max",
            vec![ValueType::F32, ValueType::F32],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f32_copysign(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F32(left), Value::F32(right)) => {
            stack.push(Value::F32(f32::copysign(right, left)));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f32.copysign",
            vec![ValueType::F32, ValueType::F32],
            vec![&lhs, &rhs],
        )),
    }
}

// f64

pub fn f64_add(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push(Value::F64(left + right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f64.add",
            vec![ValueType::F64, ValueType::F64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f64_sub(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push(Value::F64(left - right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f64.sub",
            vec![ValueType::F64, ValueType::F64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f64_mul(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push(Value::F64(left * right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f64.mul",
            vec![ValueType::F64, ValueType::F64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f64_div(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push(Value::F64(left / right));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f64.div",
            vec![ValueType::F64, ValueType::F64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f64_min(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            if left < right {
                stack.push(lhs)
            } else {
                stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f64.min",
            vec![ValueType::F64, ValueType::F64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f64_max(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            if left > right {
                stack.push(lhs)
            } else {
                stack.push(rhs)
            }
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f64.max",
            vec![ValueType::F64, ValueType::F64],
            vec![&lhs, &rhs],
        )),
    }
}

pub fn f64_copysign(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let (rhs, lhs) = (stack.pop(), stack.pop());

    match (lhs, rhs) {
        (Value::F64(left), Value::F64(right)) => {
            stack.push(Value::F64(f64::copysign(right, left)));
            Ok(())
        }
        _ => Err(make_operand_data_types_mismatch_engine_error(
            "f64.copysign",
            vec![ValueType::F64, ValueType::F64],
            vec![&lhs, &rhs],
        )),
    }
}
