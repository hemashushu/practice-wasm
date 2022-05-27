// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 一元运算指令
//!
//! 从栈顶弹出一个操作数，经过运算后压入栈
//!
//! 整数的一元运算
//! 返回值是整数（i32 或 i64，视操作数类型而定）
//!
//! - i32.clz       : count leading zeros 统计前缀（高端位）比特值为 0 的数量，opcode: 0x67
//! - i32.ctz       : count trailing zeros 统计后缀（低端位）比特值为 0 的数量，opcode: 0x68
//! - i32.popcnt    : population count 统计所有位当中，比特值为 1 的数量，opcode: 0x69
//!
//! 注，i64 有跟 i32 一样的一元运算指令，这里省略列出
//!
//! 如对于操作数 8'b00001100, clz => 4, ctz => 2, popcnt => 2
//!
//! 浮点数的一元运算
//! 返回值是浮点数（f32 或 f64，视操作数类型而定）
//!
//! - f32.abs       : 绝对值
//! - f32.neg       : 取反
//! - f32.ceil      : 向上取整（x 数轴向右方向取整）
//! - f32.floor     : 向下取整（x 数轴向左方向取整）
//! - f32.trunc     : 直接裁掉小数部分，取整
//! - f32.nearest   : 就近取整（4 舍 6 入，5 奇进偶不进）
//! - f32.sqrt      : 平方根
//!
//! 注，f64 有跟 f32 一样的一元运算指令，这里省略列出
//!
//! nearest 指令实际上是 "Round half to even"
//! https://en.wikipedia.org/wiki/Rounding#Round_half_to_even
//! https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Nearest
//!
//! Rust 的 f32::round() 函数是 4 舍 5 入，并不一样。

use anvm_ast::types::Value;

use crate::{
    error::{make_invalid_operand_data_type_engine_error, EngineError},
    vm::VM,
};

// i32

pub fn i32_clz(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = i32::leading_zeros(value);
        stack.push(Value::I32(result as i32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("i32.clz", "i32"))
    }
}

pub fn i32_ctz(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = i32::trailing_zeros(value);
        stack.push(Value::I32(result as i32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("i32.ctz", "i32"))
    }
}

pub fn i32_popcnt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = i32::count_ones(value);
        stack.push(Value::I32(result as i32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("i32.popcnt", "i32"))
    }
}

// i64

pub fn i64_clz(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = i64::leading_zeros(value);
        stack.push(Value::I64(result as i64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("i64.clz", "i64"))
    }
}

pub fn i64_ctz(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = i64::trailing_zeros(value);
        stack.push(Value::I64(result as i64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("i64.ctz", "i64"))
    }
}

pub fn i64_popcnt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = i64::count_ones(value);
        stack.push(Value::I64(result as i64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("i64.popcnt", "i64"))
    }
}

// f32

pub fn f32_abs(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = f32::abs(value);
        stack.push(Value::F32(result as f32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f32.abs", "f32"))
    }
}

pub fn f32_neg(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        stack.push(Value::F32(-value));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f32.neg", "f32"))
    }
}

pub fn f32_ceil(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = f32::ceil(value);
        stack.push(Value::F32(result as f32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f32.ceil", "f32"))
    }
}

pub fn f32_floor(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = f32::floor(value);
        stack.push(Value::F32(result as f32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f32.floor", "f32"))
    }
}

pub fn f32_trunc(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = f32::trunc(value);
        stack.push(Value::F32(result as f32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f32.trunc", "f32"))
    }
}

/// nearest 指令实现就近取整（4 舍 6 入，5 奇进偶不进）
/// 实际上是 "Round half to even"
/// https://en.wikipedia.org/wiki/Rounding#Round_half_to_even
/// https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Nearest
pub fn f32_nearest(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = round_half_to_even_f32(value);
        stack.push(Value::F32(result));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f32.nearest", "f32"))
    }
}

fn round_half_to_even_f32(value: f32) -> f32 {
    let round_half_away_from_zero = f32::round(value); // 4 舍 5 入，即 "Round half away from zero"
    let first_digit_of_fractional = ((value - f32::trunc(value)) * 10.0) as i32; // 获取第 1 位小数

    if first_digit_of_fractional != 5 && first_digit_of_fractional != -5 {
        // 第 1 位小数不是 5，结果跟 4 舍 5 入 一样
        round_half_away_from_zero
    } else {
        if (value as i32) % 2 == 0 {
            // 整数部分是偶数，减少降低 0.1
            if value > 0.0 {
                f32::round(value - 0.1)
            } else {
                f32::round(value + 0.1)
            }
        } else {
            // 整数部分是奇数，结果跟 4 舍 5 入 一样
            round_half_away_from_zero
        }
    }
}

pub fn f32_sqrt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = f32::sqrt(value);
        stack.push(Value::F32(result as f32));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f32.sqrt", "f32"))
    }
}

// f64

pub fn f64_abs(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = f64::abs(value);
        stack.push(Value::F64(result as f64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f64.abs", "f64"))
    }
}

pub fn f64_neg(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        stack.push(Value::F64(-value));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f64.neg", "f64"))
    }
}

pub fn f64_ceil(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = f64::ceil(value);
        stack.push(Value::F64(result as f64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f64.ceil", "f64"))
    }
}

pub fn f64_floor(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = f64::floor(value);
        stack.push(Value::F64(result as f64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f64.floor", "f64"))
    }
}

pub fn f64_trunc(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = f64::trunc(value);
        stack.push(Value::F64(result as f64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f64.trunc", "f64"))
    }
}

/// nearest 指令实现就近取整（4 舍 6 入，5 奇进偶不进）
/// 实际上是 "Round half to even"
/// https://en.wikipedia.org/wiki/Rounding#Round_half_to_even
/// https://developer.mozilla.org/en-US/docs/WebAssembly/Reference/Numeric/Nearest
pub fn f64_nearest(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = round_half_to_even_f64(value);
        stack.push(Value::F64(result));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f64.nearest", "f64"))
    }
}

fn round_half_to_even_f64(value: f64) -> f64 {
    let round_half_away_from_zero = f64::round(value); // 4 舍 5 入，即 "Round half away from zero"
    let first_digit_of_fractional = ((value - f64::trunc(value)) * 10.0) as i32; // 获取第 1 位小数

    if first_digit_of_fractional != 5 && first_digit_of_fractional != -5 {
        // 第 1 位小数不是 5，结果跟 4 舍 5 入 一样
        round_half_away_from_zero
    } else {
        if (value as i32) % 2 == 0 {
            // 整数部分是偶数，减少降低 0.1
            if value > 0.0 {
                f64::round(value - 0.1)
            } else {
                f64::round(value + 0.1)
            }
        } else {
            // 整数部分是奇数，结果跟 4 舍 5 入 一样
            round_half_away_from_zero
        }
    }
}

pub fn f64_sqrt(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = f64::sqrt(value);
        stack.push(Value::F64(result as f64));
        Ok(())
    } else {
        Err(make_invalid_operand_data_type_engine_error("f64.sqrt", "f64"))
    }
}
