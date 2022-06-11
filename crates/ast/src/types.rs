// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// # 数值的数据类型
///
/// <https://webassembly.github.io/spec/core/syntax/types.html>
///
/// WebAssembly 的数值只支持 4 种基本数据类型
/// i32, i64, f32, f64
#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::I32 => write!(f, "i32"),
            ValueType::I64 => write!(f, "i64"),
            ValueType::F32 => write!(f, "f32"),
            ValueType::F64 => write!(f, "f64"),
        }
    }
}

/// # 数值
///
/// 部分指令会明确表明需要将整数解析为无符号整数（unsigned integer）进行运算，
/// 比如 `lt_u` 和 `gt_u` 等，而 `Value` 仅包含了有符号的整数，
/// 所以进行无符号运算时，需要先转换再运算。
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Value {
    /// 获取数值的数据类型
    pub fn get_type(&self) -> ValueType {
        match self {
            Self::I32(_) => ValueType::I32,
            Self::I64(_) => ValueType::I64,
            Self::F32(_) => ValueType::F32,
            Self::F64(_) => ValueType::F64,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::I32(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v),
            Value::F32(v) => write!(f, "{}", v),
            Value::F64(v) => write!(f, "{}", v),
        }
    }
}

impl Into<usize> for Value {
    fn into(self) -> usize {
        if let Self::I64(v) = self {
            v as usize
        } else {
            panic!("failed to convert anvm_ast::types::Value into usize");
        }
    }
}

impl Into<Value> for usize {
    fn into(self) -> Value {
        Value::I64(self as i64)
    }
}

/// 检查一组值的数据类型
///
/// 返回 LengthMismatch: 参数和类型的数量长度不一致
/// 返回 DataTypeMismatch(usize): 其中一个参数的类型不一致
pub fn check_value_types(values: &[Value], types: &[ValueType]) -> Result<(), ValueTypeCheckError> {
    let length = values.len();
    if length != types.len() {
        // 参数和类型的数量长度不一致
        Err(ValueTypeCheckError::LengthMismatch)
    } else if length == 0 {
        Ok(())
    } else {
        for i in 0..length {
            if values[i].get_type() != types[i] {
                // 其中一个参数的类型不一致
                return Err(ValueTypeCheckError::DataTypeMismatch(i));
            }
        }
        Ok(())
    }
}

/// 检查两组类型是否匹配
///
/// 返回 LengthMismatch: 参数和类型的数量长度不一致
/// 返回 DataTypeMismatch(usize): 其中一个参数的类型不一致
pub fn check_types(
    expected: &[ValueType],
    actual: &[ValueType],
) -> Result<(), ValueTypeCheckError> {
    let length = expected.len();
    if length != actual.len() {
        // 参数和类型的数量长度不一致
        Err(ValueTypeCheckError::LengthMismatch)
    } else if length == 0 {
        Ok(())
    } else {
        for i in 0..length {
            if expected[i] != actual[i] {
                // 其中一个参数的类型不一致
                return Err(ValueTypeCheckError::DataTypeMismatch(i));
            }
        }
        Ok(())
    }
}

pub enum ValueTypeCheckError {
    LengthMismatch,
    DataTypeMismatch(/* index */ usize),
}

// WebAssembly 的索引值，比如类型索引、函数索引、内存块索引、表索引、标签索引等，使用 u32 类型。
// https://webassembly.github.io/spec/core/syntax/modules.html#indices
//
// 为了简单起见，XiaoXuan VM 里不单独定义它们

use std::fmt::Display;
