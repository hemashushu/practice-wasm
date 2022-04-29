// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use crate::types::ValueType;

/// # WebAssembly 指令的指令码

pub const UNREACHABLE: u8 = 0x00;
pub const NOP: u8 = 0x01;
pub const BLOCK: u8 = 0x02;
pub const LOOP: u8 = 0x03;
pub const IF: u8 = 0x04;
pub const ELSE: u8 = 0x05;
pub const END: u8 = 0x0B;
pub const BR: u8 = 0x0C;
pub const BR_IF: u8 = 0x0D;
pub const BR_TABLE: u8 = 0x0E;
pub const RETURN: u8 = 0x0F;
pub const CALL: u8 = 0x10;
pub const CALL_INDIRECT: u8 = 0x11;
pub const DROP: u8 = 0x1A;
pub const SELECT: u8 = 0x1B;
pub const LOCAL_GET: u8 = 0x20;
pub const LOCAL_SET: u8 = 0x21;
pub const LOCAL_TEE: u8 = 0x22;
pub const GLOBAL_GET: u8 = 0x23;
pub const GLOBAL_SET: u8 = 0x24;
pub const I32_LOAD: u8 = 0x28;
pub const I64_LOAD: u8 = 0x29;
pub const F32_LOAD: u8 = 0x2A;
pub const F64_LOAD: u8 = 0x2B;
pub const I32_LOAD8_S: u8 = 0x2C;
pub const I32_LOAD8_U: u8 = 0x2D;
pub const I32_LOAD16_S: u8 = 0x2E;
pub const I32_LOAD16_U: u8 = 0x2F;
pub const I64_LOAD8_S: u8 = 0x30;
pub const I64_LOAD8_U: u8 = 0x31;
pub const I64_LOAD16_S: u8 = 0x32;
pub const I64_LOAD16_U: u8 = 0x33;
pub const I64_LOAD32_S: u8 = 0x34;
pub const I64_LOAD32_U: u8 = 0x35;
pub const I32_STORE: u8 = 0x36;
pub const I64_STORE: u8 = 0x37;
pub const F32_STORE: u8 = 0x38;
pub const F64_STORE: u8 = 0x39;
pub const I32_STORE8: u8 = 0x3A;
pub const I32_STORE16: u8 = 0x3B;
pub const I64_STORE8: u8 = 0x3C;
pub const I64_STORE16: u8 = 0x3D;
pub const I64_STORE32: u8 = 0x3E;
pub const MEMORY_SIZE: u8 = 0x3F;
pub const MEMORY_GROW: u8 = 0x40;
pub const I32_CONST: u8 = 0x41;
pub const I64_CONST: u8 = 0x42;
pub const F32_CONST: u8 = 0x43;
pub const F64_CONST: u8 = 0x44;
pub const I32_EQZ: u8 = 0x45;
pub const I32_EQ: u8 = 0x46;
pub const I32_NE: u8 = 0x47;
pub const I32_LT_S: u8 = 0x48;
pub const I32_LT_U: u8 = 0x49;
pub const I32_GT_S: u8 = 0x4A;
pub const I32_GT_U: u8 = 0x4B;
pub const I32_LE_S: u8 = 0x4C;
pub const I32_LE_U: u8 = 0x4D;
pub const I32_GE_S: u8 = 0x4E;
pub const I32_GE_U: u8 = 0x4F;
pub const I64_EQZ: u8 = 0x50;
pub const I64_EQ: u8 = 0x51;
pub const I64_NE: u8 = 0x52;
pub const I64_LT_S: u8 = 0x53;
pub const I64_LT_U: u8 = 0x54;
pub const I64_GT_S: u8 = 0x55;
pub const I64_GT_U: u8 = 0x56;
pub const I64_LE_S: u8 = 0x57;
pub const I64_LE_U: u8 = 0x58;
pub const I64_GE_S: u8 = 0x59;
pub const I64_GE_U: u8 = 0x5A;
pub const F32_EQ: u8 = 0x5B;
pub const F32_NE: u8 = 0x5C;
pub const F32_LT: u8 = 0x5D;
pub const F32_GT: u8 = 0x5E;
pub const F32_LE: u8 = 0x5F;
pub const F32_GE: u8 = 0x60;
pub const F64_EQ: u8 = 0x61;
pub const F64_NE: u8 = 0x62;
pub const F64_LT: u8 = 0x63;
pub const F64_GT: u8 = 0x64;
pub const F64_LE: u8 = 0x65;
pub const F64_GE: u8 = 0x66;
pub const I32_CLZ: u8 = 0x67;
pub const I32_CTZ: u8 = 0x68;
pub const I32_POP_CNT: u8 = 0x69;
pub const I32_ADD: u8 = 0x6A;
pub const I32_SUB: u8 = 0x6B;
pub const I32_MUL: u8 = 0x6C;
pub const I32_DIV_S: u8 = 0x6D;
pub const I32_DIV_U: u8 = 0x6E;
pub const I32_REM_S: u8 = 0x6F;
pub const I32_REM_U: u8 = 0x70;
pub const I32_AND: u8 = 0x71;
pub const I32_OR: u8 = 0x72;
pub const I32_XOR: u8 = 0x73;
pub const I32_SHL: u8 = 0x74;
pub const I32_SHR_S: u8 = 0x75;
pub const I32_SHR_U: u8 = 0x76;
pub const I32_ROTL: u8 = 0x77;
pub const I32_ROTR: u8 = 0x78;
pub const I64_CLZ: u8 = 0x79;
pub const I64_CTZ: u8 = 0x7A;
pub const I64_POP_CNT: u8 = 0x7B;
pub const I64_ADD: u8 = 0x7C;
pub const I64_SUB: u8 = 0x7D;
pub const I64_MUL: u8 = 0x7E;
pub const I64_DIV_S: u8 = 0x7F;
pub const I64_DIV_U: u8 = 0x80;
pub const I64_REM_S: u8 = 0x81;
pub const I64_REM_U: u8 = 0x82;
pub const I64_AND: u8 = 0x83;
pub const I64_OR: u8 = 0x84;
pub const I64_XOR: u8 = 0x85;
pub const I64_SHL: u8 = 0x86;
pub const I64_SHR_S: u8 = 0x87;
pub const I64_SHR_U: u8 = 0x88;
pub const I64_ROTL: u8 = 0x89;
pub const I64_ROTR: u8 = 0x8A;
pub const F32_ABS: u8 = 0x8B;
pub const F32_NEG: u8 = 0x8C;
pub const F32_CEIL: u8 = 0x8D;
pub const F32_FLOOR: u8 = 0x8E;
pub const F32_TRUNC: u8 = 0x8F;
pub const F32_NEAREST: u8 = 0x90;
pub const F32_SQRT: u8 = 0x91;
pub const F32_ADD: u8 = 0x92;
pub const F32_SUB: u8 = 0x93;
pub const F32_MUL: u8 = 0x94;
pub const F32_DIV: u8 = 0x95;
pub const F32_MIN: u8 = 0x96;
pub const F32_MAX: u8 = 0x97;
pub const F32_COPY_SIGN: u8 = 0x98;
pub const F64_ABS: u8 = 0x99;
pub const F64_NEG: u8 = 0x9A;
pub const F64_CEIL: u8 = 0x9B;
pub const F64_FLOOR: u8 = 0x9C;
pub const F64_TRUNC: u8 = 0x9D;
pub const F64_NEAREST: u8 = 0x9E;
pub const F64_SQRT: u8 = 0x9F;
pub const F64_ADD: u8 = 0xA0;
pub const F64_SUB: u8 = 0xA1;
pub const F64_MUL: u8 = 0xA2;
pub const F64_DIV: u8 = 0xA3;
pub const F64_MIN: u8 = 0xA4;
pub const F64_MAX: u8 = 0xA5;
pub const F64_COPY_SIGN: u8 = 0xA6;
pub const I32_WRAP_I64: u8 = 0xA7;
pub const I32_TRUNC_F32_S: u8 = 0xA8;
pub const I32_TRUNC_F32_U: u8 = 0xA9;
pub const I32_TRUNC_F64_S: u8 = 0xAA;
pub const I32_TRUNC_F64_U: u8 = 0xAB;
pub const I64_EXTEND_I32_S: u8 = 0xAC;
pub const I64_EXTEND_I32_U: u8 = 0xAD;
pub const I64_TRUNC_F32_S: u8 = 0xAE;
pub const I64_TRUNC_F32_U: u8 = 0xAF;
pub const I64_TRUNC_F64_S: u8 = 0xB0;
pub const I64_TRUNC_F64_U: u8 = 0xB1;
pub const F32_CONVERT_I32_S: u8 = 0xB2;
pub const F32_CONVERT_I32_U: u8 = 0xB3;
pub const F32_CONVERT_I64_S: u8 = 0xB4;
pub const F32_CONVERT_I64_U: u8 = 0xB5;
pub const F32_DEMOTE_F64: u8 = 0xB6;
pub const F64_CONVERT_I32_S: u8 = 0xB7;
pub const F64_CONVERT_I32_U: u8 = 0xB8;
pub const F64_CONVERT_I64_S: u8 = 0xB9;
pub const F64_CONVERT_I64_U: u8 = 0xBA;
pub const F64_PROMOTE_F32: u8 = 0xBB;
pub const I32_REINTERPRET_F32: u8 = 0xBC;
pub const I64_REINTERPRET_F64: u8 = 0xBD;
pub const F32_REINTERPRET_I32: u8 = 0xBE;
pub const F64_REINTERPRET_I64: u8 = 0xBF;
pub const I32_EXTEND8_S: u8 = 0xC0;
pub const I32_EXTEND16_S: u8 = 0xC1;
pub const I64_EXTEND8_S: u8 = 0xC2;
pub const I64_EXTEND16_S: u8 = 0xC3;
pub const I64_EXTEND32_S: u8 = 0xC4;
pub const TRUNC_SAT: u8 = 0xFC;

// `饱和截断` 次操作码
// `饱和截断` 指令由一个主操作码 `0xFC` 和一个次操作码（byte 类型）组成

pub const I32_TRUNC_SAT_F32_S: u8 = 0;
pub const I32_TRUNC_SAT_F32_U: u8 = 1;
pub const I32_TRUNC_SAT_F64_S: u8 = 2;
pub const I32_TRUNC_SAT_F64_U: u8 = 3;
pub const I64_TRUNC_SAT_F32_S: u8 = 4;
pub const I64_TRUNC_SAT_F32_U: u8 = 5;
pub const I64_TRUNC_SAT_F64_S: u8 = 6;
pub const I64_TRUNC_SAT_F64_U: u8 = 7;

/// # WebAssembly 指令
///
/// <https://webassembly.github.io/spec/core/syntax/instructions.html>
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Unreachable,
    Nop,
    Block {
        result: Option<ValueType>,
        body: Rc<Vec<Instruction>>,
    },
    Loop {
        result: Option<ValueType>,
        body: Rc<Vec<Instruction>>,
    },
    If {
        result: Option<ValueType>,
        consequet_body: Rc<Vec<Instruction>>,
        alternate_body: Rc<Vec<Instruction>>,
    },
    Else,
    End,
    Br(u32),        // relative deepth
    BrIf(u32),      // relative deepth
    BrTable {
        relative_depths: Vec<u32>,
        default_relative_depth: u32,
    },
    Return,
    Call(u32),          // function index
    CallIndirect(u32),  // function index

    Drop,
    Select,

    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

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
    MemorySize(u32), // memory block index
    MemoryGrow(u32), // memory block index

    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

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

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Clz,
    I32Ctz,
    I32PopCnt,

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

    I64Clz,
    I64Ctz,
    I64PopCnt,

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

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,

    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32CopySign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,

    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64CopySign,

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

    TruncSat(u8),
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemoryArg {
    pub align: u32,
    pub offset: u32,
}
