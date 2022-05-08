// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// # 数据类型
///
/// <https://webassembly.github.io/spec/core/syntax/types.html>
///
/// WebAssembly 只支持 4 种基本数据类型
/// i32, i64, f32, f64
#[derive(Debug, PartialEq, Clone)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

/// # 虚拟机内部运算使用的基本数据类型
///
/// 部分指令明确表明需要将整数解析为无符号整数（unsigned int）进行运算，
/// 比如 `lt_u` 和 `gt_u` 等，而 `Value` 仅包含了有符号的整数，
/// 所以进行无符号运算时，需要先转换在运算。
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Value {
    pub fn get_type(&self) -> ValueType {
        match self {
            Self::I32(_) => ValueType::I32,
            Self::I64(_) => ValueType::I64,
            Self::F32(_) => ValueType::F32,
            Self::F64(_) => ValueType::F64,
        }
    }
}

/// WebAssembly 的索引使用 u32 类型。
/// https://webassembly.github.io/spec/core/syntax/modules.html#indices
///
/// 这里为了简单起见就不单独定义它们了，而直接使用 u32。
type _Index = u32; // 无用的语句，仅为了书写文档注释
