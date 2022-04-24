// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::ValueType;

/// # WebAssembly 指令
///
/// <https://webassembly.github.io/spec/core/syntax/instructions.html>
pub enum Instruction {
    // 控制指令
    Unreachable,
    Nop,
    Block {
        result: Option<ValueType>,
        body: Vec<Instruction>,
    },
    Loop {
        result: Option<ValueType>,
        body: Vec<Instruction>,
    },
    If {
        result: Option<ValueType>,
        then_body: Vec<Instruction>,
        else_body: Vec<Instruction>,
    },
    Br(u32),
    BrIf(u32),
    BrTable {
        labels: Vec<u32>,
        default_label: u32,
    },
    Return,
    Call(u32),
    CallIndirect(u32), // 参数 0：目标函数的类型索引；参数 1：表索引，因为只能是 0，所以省略了此参数

    // 参数（操作数，parametric）指令
    Drop,
    Select,

    // 变量指令
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // 内存指令
    I32Load(MemoryArg),
    I64Load(MemoryArg),
    F32Load(MemoryArg),
    F64Load(MemoryArg),
    I32Load8S(MemoryArg),
    I32Load8U(MemoryArg),
    I32Load16S(MemoryArg),
    I32Load16U(MemoryArg),
    I64Load8S(MemoryArg),
    I64Load8U(MemoryArg),
    I64Load16S(MemoryArg),
    I64Load16U(MemoryArg),
    I64Load32S(MemoryArg),
    I64Load32U(MemoryArg),
    I32Store(MemoryArg),
    I64Store(MemoryArg),
    F32Store(MemoryArg),
    F64Store(MemoryArg),
    I32Store8(MemoryArg),
    I32Store16(MemoryArg),
    I64Store8(MemoryArg),
    I64Store16(MemoryArg),
    I64Store32(MemoryArg),
    MemoryArgorySize, // 参数 0：目标内存块的索引，因为只能是 0，所以省略了此参数
    MemoryArgoryGrow, // 参数 0：目标内存块的索引，因为只能是 0，所以省略了此参数

    // 数值指令
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // 数值指令——比较 i32
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    // 数值指令——比较 i64
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    // 数值指令——比较 f32
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    // 数值指令——比较 f64
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    // 数值指令——一元运算 i32
    I32Clz,
    I32Ctz,
    I32Popcnt,

    // 数值指令——二元运算 i32
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    // 数值指令——一元运算 i64
    I64Clz,
    I64Ctz,
    I64Popcnt,

    // 数值指令——二元运算 i64
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    // 数值指令——一元运算 f32
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,

    // 数值指令——二元运算 f32
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    // 数值指令——一元运算 f64
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,

    // 数值指令——二元运算 f64
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    // 转换指令
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,

    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,

    TruncSat(u32), // 参数 0：饱和截断的类型
}

pub struct MemoryArg {
    pub align: u32,
    pub offset: u32,
}
