// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 类型转换指令
//!
//! ## 整数截断
//!
//! 将 64 位的整数直接截断为 32 位（即只保留低端信息）
//!
//! - i32.wrap_i64
//!
//! ## 整数提升
//!
//! 将位宽较窄的整数提升为位宽较广的整数，比如将 32 位整数提升为 64 位
//!
//! 源 i32，目标 i32
//! - i32.extend8_s
//! - i32.extend16_s
//!
//! 源 i32，目标 i64
//! - i64.extend_i32_s
//! - i64.extend_i32_u
//!
//! 源 i64，目标 i64
//! - i64.extend8_s
//! - i64.extend16_s
//! - i64.extend32_s
//!
//! ## 浮点数转整数（截断运算）
//!
//! 把浮点数截断为整数
//!
//! 源 f32，目标 i32
//!
//! - i32.trunc_f32_s
//! - i32.trunc_f32_u
//!
//! 源 f32，目标 i64
//!
//! - i64.trunc_f32_s
//! - i64.trunc_f32_u
//!
//! 源 f64，目标 i32
//!
//! - i32.trunc_f64_s
//! - i32.trunc_f64_u
//!
//! 源 f64，目标 i64
//!
//! - i64.trunc_f64_s
//! - i64.trunc_f64_u
//!
//! ## 饱和截断
//!
//! 用于将浮点数转换为整数
//!
//! - i32.trunc_sat_f32_s
//! - i32.trunc_sat_f32_u
//! - i32.trunc_sat_f64_s
//! - i32.trunc_sat_f64_u
//! - i64.trunc_sat_f32_s
//! - i64.trunc_sat_f32_u
//! - i64.trunc_sat_f64_s
//! - i64.trunc_sat_f64_u
//!
//! 跟一般截断不同的是：
//! - 将 NaN 转为 0
//! - 将正/负无穷转为整数最大/最小值
//! - 饱和截断不会抛出异常，原文：
//!   `saturating` meaning that their results are limited to the maximum or
//!   minimum possible value for the given destination type.
//!
//! ## 整数转浮点数（转换运算）
//!
//! 源 i32，目标 f32
//!
//! - f32.convert_i32_s
//! - f32.convert_i32_u
//!
//! 源 i32，目标 f64
//!
//! - f64.convert_i32_s
//! - f64.convert_i32_u
//!
//! 源 i64，目标 f32
//!
//! - f32.convert_i64_s
//! - f32.convert_i64_u
//!
//! 源 i64，目标 f64
//!
//! - f64.convert_i64_s
//! - f64.convert_i64_u
//!
//! ## 浮点数精度调整
//!
//! - f32.demote_f64_s
//! - f64.promote_f32
//!
//! ## 比特位重新解释
//!
//! 不改变操作数的比特位，仅重新解释成其他类型

use anvm_ast::types::{Value, ValueType};

use crate::{
    error::{make_operand_data_types_mismatch_engine_error, EngineError},
    vm::VM,
};

// 整数截断

pub fn i32_wrap_i64(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = value as i32;
        stack.push(Value::I32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.wrap_i64",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

// 整数提升

pub fn i32_extend8_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = (value as i8) as i32;
        stack.push(Value::I32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.extend8_s",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn i32_extend16_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = (value as i16) as i32;
        stack.push(Value::I32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.extend16_s",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn i64_extend_i32_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = value as i64;
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.extend_i32_s",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn i64_extend_i32_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = (value as u32) as i64;
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.extend_i32_u",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn i64_extend8_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = (value as i8) as i64;
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.extend8_s",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

pub fn i64_extend16_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = (value as i16) as i64;
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.extend16_s",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

pub fn i64_extend32_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = (value as i32) as i64;
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.extend32_s",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

// 浮点数转整数（截断运算）

pub fn i32_trunc_f32_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = value as i32;
        stack.push(Value::I32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.trunc_f32_s",
            vec![ValueType::F32],
            vec![&operand],
        ))
    }
}

pub fn i32_trunc_f32_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = value as u32;
        stack.push(Value::I32(result as i32));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.trunc_f32_u",
            vec![ValueType::F32],
            vec![&operand],
        ))
    }
}

pub fn i64_trunc_f32_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = value as i64;
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.trunc_f32_s",
            vec![ValueType::F32],
            vec![&operand],
        ))
    }
}

pub fn i64_trunc_f32_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = value as u64;
        stack.push(Value::I64(result as i64));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.trunc_f32_u",
            vec![ValueType::F32],
            vec![&operand],
        ))
    }
}

pub fn i32_trunc_f64_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = value as i32;
        stack.push(Value::I32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.trunc_f64_s",
            vec![ValueType::F64],
            vec![&operand],
        ))
    }
}

pub fn i32_trunc_f64_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = value as u32;
        stack.push(Value::I32(result as i32));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.trunc_f64_u",
            vec![ValueType::F64],
            vec![&operand],
        ))
    }
}

pub fn i64_trunc_f64_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = value as i64;
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.trunc_f64_s",
            vec![ValueType::F64],
            vec![&operand],
        ))
    }
}

pub fn i64_trunc_f64_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = value as u64;
        stack.push(Value::I64(result as i64));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.trunc_f64_u",
            vec![ValueType::F64],
            vec![&operand],
        ))
    }
}

// 饱和截断

// todo::

// 整数转浮点数（转换运算）

pub fn f32_convert_i32_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = value as f32;
        stack.push(Value::F32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f32.convert_i32_s",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn f32_convert_i32_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = (value as u32) as f32;
        stack.push(Value::F32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f32.convert_i32_u",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn f64_convert_i32_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = value as f64;
        stack.push(Value::F64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f64.convert_i32_s",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn f64_convert_i32_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = (value as u32) as f64;
        stack.push(Value::F64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f64.convert_i32_u",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn f32_convert_i64_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = value as f32;
        stack.push(Value::F32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f32.convert_i64_s",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

pub fn f32_convert_i64_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = (value as u64) as f32;
        stack.push(Value::F32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f32.convert_i64_u",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

pub fn f64_convert_i64_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = value as f64;
        stack.push(Value::F64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f64.convert_i64_s",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

pub fn f64_convert_i64_u(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = (value as u64) as f64;
        stack.push(Value::F64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f64.convert_i64_u",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}

// 浮点数精度调整

pub fn f32_demote_f64_s(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = value as f32;
        stack.push(Value::F32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f32.demote_f64_s",
            vec![ValueType::F64],
            vec![&operand],
        ))
    }
}

pub fn f64_promote_f32(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = value as f64;
        stack.push(Value::F64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f64.promote_f32",
            vec![ValueType::F32],
            vec![&operand],
        ))
    }
}

// 比特位重新解释

pub fn i32_reinterpret_f32(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F32(value) = operand {
        let result = i32::from_le_bytes(value.to_le_bytes());
        stack.push(Value::I32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i32.reinterpret_f32",
            vec![ValueType::F32],
            vec![&operand],
        ))
    }
}

pub fn i64_reinterpret_f64(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::F64(value) = operand {
        let result = i64::from_le_bytes(value.to_le_bytes());
        stack.push(Value::I64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "i64.reinterpret_f64",
            vec![ValueType::F64],
            vec![&operand],
        ))
    }
}

pub fn f32_reinterpret_i32(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I32(value) = operand {
        let result = f32::from_le_bytes(value.to_le_bytes());
        stack.push(Value::F32(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f32.reinterpret_i32",
            vec![ValueType::I32],
            vec![&operand],
        ))
    }
}

pub fn f64_reinterpret_i64(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let operand = stack.pop();

    if let Value::I64(value) = operand {
        let result = f64::from_le_bytes(value.to_le_bytes());
        stack.push(Value::F64(result));
        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "f64.reinterpret_i64",
            vec![ValueType::I64],
            vec![&operand],
        ))
    }
}
