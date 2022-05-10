// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use crate::types::ValueType;

// # WebAssembly 指令的操作码
//
// 指令的操作码使用一个 byte 类型整数存储
//
// # 指令操作码列表
//
// 控制类指令
//
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
//
// 操作数类（参数，parametric）指令
//
pub const DROP: u8 = 0x1A;
pub const SELECT: u8 = 0x1B;
//
// 变量类指令
//
pub const LOCAL_GET: u8 = 0x20;
pub const LOCAL_SET: u8 = 0x21;
pub const LOCAL_TEE: u8 = 0x22;
pub const GLOBAL_GET: u8 = 0x23;
pub const GLOBAL_SET: u8 = 0x24;
//
// 内存类指令
//
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
//
// 数值类指令
//
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
//
// `饱和截断` 指令的次操作码
// `饱和截断` 指令由一个主操作码 `0xFC` 和一个次操作码（byte 类型）组成
//
pub const I32_TRUNC_SAT_F32_S: u8 = 0x0;
pub const I32_TRUNC_SAT_F32_U: u8 = 0x1;
pub const I32_TRUNC_SAT_F64_S: u8 = 0x2;
pub const I32_TRUNC_SAT_F64_U: u8 = 0x3;
pub const I64_TRUNC_SAT_F32_S: u8 = 0x4;
pub const I64_TRUNC_SAT_F32_U: u8 = 0x5;
pub const I64_TRUNC_SAT_F64_S: u8 = 0x6;
pub const I64_TRUNC_SAT_F64_U: u8 = 0x7;

/// # WebAssembly 指令列表
///
/// <https://webassembly.github.io/spec/core/syntax/instructions.html>
///
/// WebAssembly 指令存在于 `代码段`（`Code Section`），每个函数体一个段落，
/// 每个段落由连续的一个或多个指令组成且均由 `End` 指令表示结束。
///
/// 需注意的是，每个函数体的指令实际上是并排的，但在 WebAssembly 示例经常以文本格式
/// 书写，而文本格式因为允许折叠和内联，看起来就像是有结构的 S-Expression，
/// 诸如 `If`、`Block`、`Loop` 等指令的文本格式，往往以层次结构的形式书写。
/// 河马蜀黍认为这很容易误导阅读者，让阅读者认为 WebAssembly 的指令是有层次结构的，
/// 这样一来无论是写 parser 还是 engine ，如果写成层次结构型的，都会复杂化。
/// 另外 WebAssembly 的规范文档书写得实在一言难尽，真心希望学习研究 WebAssembly 的
/// 同学能先看到这段话。
///
/// WebAssembly 的指令有些需要参数，而参数的形式有以下几种：
///
/// - 部分指令直接在指令当中带有参数，比如 `Br` 指令带有一个
///   表示 `相对层次` 的参数
/// - 有些指令的参数则来自操作数栈，比如 `I32Add` 指令，两个参数均从操作数栈顶
///   弹出而获得。
/// - 有些指令则结合了上面的两种形式，比如 `MemoryGrow` 指令除了
///   直接带有的表示 `内存块索引` 的参数，还有一个表示 `数量` 的参数从栈顶获取。
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Unreachable,
    Nop,
    Block(BlockType), // params: (block type)
    Loop(BlockType),  // params: (block type)
    If(BlockType),    // params: (block type)
    Else,
    End,
    Br(u32),                // params: (relative deepth)
    BrIf(u32),              // params: (relative deepth)
    BrTable(Vec<u32>, u32), // params: (relative depths, default relative depth)
    Return,
    Call(u32),              // params: (function index)
    CallIndirect(u32, u32), // params: (function type index, table index)

    Drop,
    Select,

    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    I32Load(MemoryArgument),
    I64Load(MemoryArgument),
    F32Load(MemoryArgument),
    F64Load(MemoryArgument),
    I32Load8S(MemoryArgument),
    I32Load8U(MemoryArgument),
    I32Load16S(MemoryArgument),
    I32Load16U(MemoryArgument),
    I64Load8S(MemoryArgument),
    I64Load8U(MemoryArgument),
    I64Load16S(MemoryArgument),
    I64Load16U(MemoryArgument),
    I64Load32S(MemoryArgument),
    I64Load32U(MemoryArgument),
    I32Store(MemoryArgument),
    I64Store(MemoryArgument),
    F32Store(MemoryArgument),
    F64Store(MemoryArgument),
    I32Store8(MemoryArgument),
    I32Store16(MemoryArgument),
    I64Store8(MemoryArgument),
    I64Store16(MemoryArgument),
    I64Store32(MemoryArgument),
    MemorySize(u32), // params: (memory block index)
    MemoryGrow(u32), // params: (memory block index)

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

    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
}

/// 流程控制结构块（比如 if/block/loop）跟函数类似
/// 也可以有参数和返回值，除了可以跟函数一样共享 `类型段`（`Type Secion`）所
/// 定义的类型，还有内置的 5 种无参数的类型：
/// - () -> i32
/// - () -> i64
/// - () -> f32
/// - () -> f64
/// - () -> ()
#[derive(Debug, PartialEq, Clone)]
pub enum BlockType {
    ResultOnly(Option<ValueType>),
    FunctionTypeIndex(u32),
}

impl Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ResultOnly(option_value_type) => match option_value_type {
                Some(ValueType::I32) => write!(f, "(result i32)"),
                Some(ValueType::I64) => write!(f, "(result i64)"),
                Some(ValueType::F32) => write!(f, "(result f32)"),
                Some(ValueType::F64) => write!(f, "(result f64)"),
                None => write!(f, ""),
            },
            Self::FunctionTypeIndex(function_type_index) => {
                write!(f, "(type {})", function_type_index)
            }
        }
    }
}

/// 内存类指令的参数
///
/// 参数存在于 load 和 store 指令的立即数里
/// 二进制格式
///
/// i32.load align:uint32 offset:uint32
///
/// 文本格式
///
/// (i32.load offset=0 align=2)
///
/// 对于文本格式，必须先写 offset 再写 align，且可以省略 `align` 值，
/// 对于 i32.load/i32.store，默认对齐 4 个字节
/// 对于 i64.load/i64.store，默认对齐 8 个字节
///
/// 参数的作用
///
/// - offset 偏移值
///   加载（以及存储）指令都会从操作数栈弹出一个 i32 类型的整数，让它与指令的立即数 offset 相加，得到
///   实际的内存地址，即：有效地址 = offset + popUint32()
///
/// - align 地址对齐字节数量的对数，表示对齐一个 ”以 2 为底，以 align 为指数“ 的字节数，
///   比如 align = 1 时，表示对齐 2^1 = 2 个字节
///   比如 align = 2 时，表示对齐 2^2 = 4 个字节
///   align 只起提示作用，用于帮助编译器优化机器代码，对实际执行没有影响（对于 wasm 解析器，可以忽略这个值）
///   文本格式里 `align` 的值就是字节数，比如文本格式的 8 对应二进制格式的 3 (2^3)。
#[derive(Debug, PartialEq, Clone)]
pub struct MemoryArgument {
    pub align: u32, // 这里记录的是二进制格式里所存储的数值，也就是指数。
    pub offset: u32,
}

impl Display for MemoryArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset={} align={}", self.offset, 2i32.pow(self.align))
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreachable => write!(f, "unreachable"),
            Self::Nop => write!(f, "nop"),
            Self::Block(block_type) => {
                let arg = block_type.to_string();
                if arg == "" {
                    write!(f, "block")
                } else {
                    write!(f, "block {}", arg)
                }
            }
            Self::Loop(block_type) => {
                let arg = block_type.to_string();
                if arg == "" {
                    write!(f, "loop")
                } else {
                    write!(f, "loop {}", arg)
                }
            }
            Self::If(block_type) => {
                let arg = block_type.to_string();
                if arg == "" {
                    write!(f, "if")
                } else {
                    write!(f, "if {}", arg)
                }
            }
            Self::Else => write!(f, "else"),
            Self::End => write!(f, "end"),
            Self::Br(relative_deepth) => {
                if *relative_deepth == 0 {
                    write!(f, "br")
                } else {
                    write!(f, "br {}", relative_deepth)
                }
            }
            Self::BrIf(relative_deepth) => {
                if *relative_deepth == 0 {
                    write!(f, "br_if")
                } else {
                    write!(f, "br_if {}", relative_deepth)
                }
            }
            Self::BrTable(relative_depths, default_relative_depth) => {
                let depths = relative_depths
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "br_table {} {}", depths, default_relative_depth)
            }
            Self::Return => write!(f, "return"),
            Self::Call(function_index) => write!(f, "call {}", function_index),
            Self::CallIndirect(function_type_index, table_index) => {
                write!(f, "call_indirect (type {})", function_type_index) // table_index 暂时用不上
            }

            Self::Drop => write!(f, "drop"),
            Self::Select => write!(f, "select"),

            Self::LocalGet(local_variable_index) => write!(f, "local.get {}", local_variable_index),
            Self::LocalSet(local_variable_index) => write!(f, "local.set {}", local_variable_index),
            Self::LocalTee(local_variable_index) => write!(f, "local.tee {}", local_variable_index),
            Self::GlobalGet(global_variable_index) => {
                write!(f, "global.get {}", global_variable_index)
            }
            Self::GlobalSet(global_variable_index) => {
                write!(f, "global.set {}", global_variable_index)
            }

            Self::I32Load(memory_argument) => write!(f, "i32.load {}", memory_argument),
            Self::I64Load(memory_argument) => write!(f, "i64.load {}", memory_argument),
            Self::F32Load(memory_argument) => write!(f, "f32.load {}", memory_argument),
            Self::F64Load(memory_argument) => write!(f, "f64.load {}", memory_argument),
            Self::I32Load8S(memory_argument) => write!(f, "i32.load8_s {}", memory_argument),
            Self::I32Load8U(memory_argument) => write!(f, "i32.load8_u {}", memory_argument),
            Self::I32Load16S(memory_argument) => write!(f, "i32.load16_s {}", memory_argument),
            Self::I32Load16U(memory_argument) => write!(f, "i32.load16_u {}", memory_argument),
            Self::I64Load8S(memory_argument) => write!(f, "i64.load8_s {}", memory_argument),
            Self::I64Load8U(memory_argument) => write!(f, "i64.load8_u {}", memory_argument),
            Self::I64Load16S(memory_argument) => write!(f, "i64.load16_s {}", memory_argument),
            Self::I64Load16U(memory_argument) => write!(f, "i64.load16_u {}", memory_argument),
            Self::I64Load32S(memory_argument) => write!(f, "i64.load32_s {}", memory_argument),
            Self::I64Load32U(memory_argument) => write!(f, "i64.load32_u {}", memory_argument),
            Self::I32Store(memory_argument) => write!(f, "i32.store {}", memory_argument),
            Self::I64Store(memory_argument) => write!(f, "i64.store {}", memory_argument),
            Self::F32Store(memory_argument) => write!(f, "f32.store {}", memory_argument),
            Self::F64Store(memory_argument) => write!(f, "f64.store {}", memory_argument),
            Self::I32Store8(memory_argument) => write!(f, "i32.store8 {}", memory_argument),
            Self::I32Store16(memory_argument) => write!(f, "i32.store16 {}", memory_argument),
            Self::I64Store8(memory_argument) => write!(f, "i64.store8 {}", memory_argument),
            Self::I64Store16(memory_argument) => write!(f, "i64.store16 {}", memory_argument),
            Self::I64Store32(memory_argument) => write!(f, "i64.store32 {}", memory_argument),
            Self::MemorySize(memory_block_index) => write!(f, "memory.size"), // memory_block_index 暂时用不上
            Self::MemoryGrow(memory_block_index) => write!(f, "memory.grow"), // memory_block_index 暂时用不上

            Self::I32Const(immediate_number) => write!(f, "i32.const {}", immediate_number),
            Self::I64Const(immediate_number) => write!(f, "i64.const {}", immediate_number),
            Self::F32Const(immediate_number) => write!(f, "f32.const {}", immediate_number),
            Self::F64Const(immediate_number) => write!(f, "f64.const {}", immediate_number),

            Self::I32Eqz => write!(f, "i32.eqz"),
            Self::I32Eq => write!(f, "i32.eq"),
            Self::I32Ne => write!(f, "i32.ne"),
            Self::I32LtS => write!(f, "i32.lt_s"),
            Self::I32LtU => write!(f, "i32.lt_u"),
            Self::I32GtS => write!(f, "i32.gt_s"),
            Self::I32GtU => write!(f, "i32.gt_u"),
            Self::I32LeS => write!(f, "i32.le_s"),
            Self::I32LeU => write!(f, "i32.le_u"),
            Self::I32GeS => write!(f, "i32.ge_s"),
            Self::I32GeU => write!(f, "i32.ge_u"),

            Self::I64Eqz => write!(f, "i64.eqz"),
            Self::I64Eq => write!(f, "i64.eq"),
            Self::I64Ne => write!(f, "i64.ne"),
            Self::I64LtS => write!(f, "i64.lt_s"),
            Self::I64LtU => write!(f, "i64.lt_u"),
            Self::I64GtS => write!(f, "i64.gt_s"),
            Self::I64GtU => write!(f, "i64.gt_u"),
            Self::I64LeS => write!(f, "i64.le_s"),
            Self::I64LeU => write!(f, "i64.le_u"),
            Self::I64GeS => write!(f, "i64.ge_s"),
            Self::I64GeU => write!(f, "i64.ge_u"),

            Self::F32Eq => write!(f, "f32.eq"),
            Self::F32Ne => write!(f, "f32.ne"),
            Self::F32Lt => write!(f, "f32.lt"),
            Self::F32Gt => write!(f, "f32.gt"),
            Self::F32Le => write!(f, "f32.le"),
            Self::F32Ge => write!(f, "f32.ge"),

            Self::F64Eq => write!(f, "f64.eq"),
            Self::F64Ne => write!(f, "f64.ne"),
            Self::F64Lt => write!(f, "f64.lt"),
            Self::F64Gt => write!(f, "f64.gt"),
            Self::F64Le => write!(f, "f64.le"),
            Self::F64Ge => write!(f, "f64.ge"),

            Self::I32Clz => write!(f, "i32.clz"),
            Self::I32Ctz => write!(f, "i32.ctz"),
            Self::I32PopCnt => write!(f, "i32.popcnt"),

            Self::I32Add => write!(f, "i32.add"),
            Self::I32Sub => write!(f, "i32.sub"),
            Self::I32Mul => write!(f, "i32.mul"),
            Self::I32DivS => write!(f, "i32.div_s"),
            Self::I32DivU => write!(f, "i32.div_u"),
            Self::I32RemS => write!(f, "i32.rem_s"),
            Self::I32RemU => write!(f, "i32.rem_u"),
            Self::I32And => write!(f, "i32.and"),
            Self::I32Or => write!(f, "i32.or"),
            Self::I32Xor => write!(f, "i32.xor"),
            Self::I32Shl => write!(f, "i32.shl"),
            Self::I32ShrS => write!(f, "i32.shr_s"),
            Self::I32ShrU => write!(f, "i32.shr_u"),
            Self::I32Rotl => write!(f, "i32.rotl"),
            Self::I32Rotr => write!(f, "i32.rotr"),

            Self::I64Clz => write!(f, "i64.clz"),
            Self::I64Ctz => write!(f, "i64.ctz"),
            Self::I64PopCnt => write!(f, "i64.popcnt"),

            Self::I64Add => write!(f, "i64.add"),
            Self::I64Sub => write!(f, "i64.sub"),
            Self::I64Mul => write!(f, "i64.mul"),
            Self::I64DivS => write!(f, "i64.div_s"),
            Self::I64DivU => write!(f, "i64.div_u"),
            Self::I64RemS => write!(f, "i64.rem_s"),
            Self::I64RemU => write!(f, "i64.rem_u"),
            Self::I64And => write!(f, "i64.and"),
            Self::I64Or => write!(f, "i64.or"),
            Self::I64Xor => write!(f, "i64.xor"),
            Self::I64Shl => write!(f, "i64.shl"),
            Self::I64ShrS => write!(f, "i64.shr_s"),
            Self::I64ShrU => write!(f, "i64.shr_u"),
            Self::I64Rotl => write!(f, "i64.rotl"),
            Self::I64Rotr => write!(f, "i64.rotr"),

            Self::F32Abs => write!(f, "f32.abs"),
            Self::F32Neg => write!(f, "f32.neg"),
            Self::F32Ceil => write!(f, "f32.ceil"),
            Self::F32Floor => write!(f, "f32.floor"),
            Self::F32Trunc => write!(f, "f32.trunc"),
            Self::F32Nearest => write!(f, "f32.nearest"),
            Self::F32Sqrt => write!(f, "f32.sqrt"),
            Self::F32Add => write!(f, "f32.add"),
            Self::F32Sub => write!(f, "f32.sub"),
            Self::F32Mul => write!(f, "f32.mul"),
            Self::F32Div => write!(f, "f32.div"),
            Self::F32Min => write!(f, "f32.min"),
            Self::F32Max => write!(f, "f32.max"),
            Self::F32CopySign => write!(f, "f32.copysign"),

            Self::F64Abs => write!(f, "f64.abs"),
            Self::F64Neg => write!(f, "f64.neg"),
            Self::F64Ceil => write!(f, "f64.ceil"),
            Self::F64Floor => write!(f, "f64.floor"),
            Self::F64Trunc => write!(f, "f64.trunc"),
            Self::F64Nearest => write!(f, "f64.nearest"),
            Self::F64Sqrt => write!(f, "f64.sqrt"),
            Self::F64Add => write!(f, "f64.add"),
            Self::F64Sub => write!(f, "f64.sub"),
            Self::F64Mul => write!(f, "f64.mul"),
            Self::F64Div => write!(f, "f64.div"),
            Self::F64Min => write!(f, "f64.min"),
            Self::F64Max => write!(f, "f64.max"),
            Self::F64CopySign => write!(f, "f64.copysign"),

            Self::I32WrapI64 => write!(f, "i32.wrap_i64"),
            Self::I32TruncF32S => write!(f, "i32.trunc_f32_s"),
            Self::I32TruncF32U => write!(f, "i32.trunc_f32_u"),
            Self::I32TruncF64S => write!(f, "i32.trunc_f64_s"),
            Self::I32TruncF64U => write!(f, "i32.trunc_f64_u"),
            Self::I64ExtendI32S => write!(f, "i64.extend_i32_s"),
            Self::I64ExtendI32U => write!(f, "i64.extend_i32_u"),
            Self::I64TruncF32S => write!(f, "i64.trunc_f32_s"),
            Self::I64TruncF32U => write!(f, "i64.trunc_f32_u"),
            Self::I64TruncF64S => write!(f, "i64.trunc_f64_s"),
            Self::I64TruncF64U => write!(f, "i64.trunc_f64_u"),
            Self::F32ConvertI32S => write!(f, "f32.convert_i32_s"),
            Self::F32ConvertI32U => write!(f, "f32.convert_i32_u"),
            Self::F32ConvertI64S => write!(f, "f32.convert_i64_s"),
            Self::F32ConvertI64U => write!(f, "f32.convert_i64_u"),
            Self::F32DemoteF64 => write!(f, "f32.demote_f64"),
            Self::F64ConvertI32S => write!(f, "f64.convert_i32_s"),
            Self::F64ConvertI32U => write!(f, "f64.convert_i32_u"),
            Self::F64ConvertI64S => write!(f, "f64.convert_i64_s"),
            Self::F64ConvertI64U => write!(f, "f64.convert_i64_u"),
            Self::F64PromoteF32 => write!(f, "f64.promote_f32"),
            Self::I32ReinterpretF32 => write!(f, "i32.reinterpret_f32"),
            Self::I64ReinterpretF64 => write!(f, "i64.reinterpret_f64"),
            Self::F32ReinterpretI32 => write!(f, "i32.reinterpret_i32"),
            Self::F64ReinterpretI64 => write!(f, "i64.reinterpret_i64"),

            Self::I32Extend8S => write!(f, "i32.extend8_s"),
            Self::I32Extend16S => write!(f, "i32.extend16_s"),
            Self::I64Extend8S => write!(f, "i64.extend8_s"),
            Self::I64Extend16S => write!(f, "i64.extend16_s"),
            Self::I64Extend32S => write!(f, "i64.extend32_s"),

            Self::I32TruncSatF32S => write!(f, "i32.trunc_sat_f32_s"),
            Self::I32TruncSatF32U => write!(f, "i32.trunc_sat_f32_u"),
            Self::I32TruncSatF64S => write!(f, "i32.trunc_sat_f64_s"),
            Self::I32TruncSatF64U => write!(f, "i32.trunc_sat_f64_u"),
            Self::I64TruncSatF32S => write!(f, "i64.trunc_sat_f32_s"),
            Self::I64TruncSatF32U => write!(f, "i64.trunc_sat_f32_u"),
            Self::I64TruncSatF64S => write!(f, "i64.trunc_sat_64_s"),
            Self::I64TruncSatF64U => write!(f, "i64.trunc_sat_64_u"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{instruction::MemoryArgument, types::ValueType};

    use super::{BlockType, Instruction};

    use pretty_assertions::assert_eq;

    #[test]
    fn test_instruction_display() {
        // 这里只测试部分带有直接操作数的指令

        let instructions: Vec<Instruction> = vec![
            Instruction::Unreachable,
            Instruction::Block(BlockType::ResultOnly(Some(ValueType::I32))),
            Instruction::Block(BlockType::ResultOnly(None)),
            Instruction::Block(BlockType::FunctionTypeIndex(1)),
            Instruction::Loop(BlockType::ResultOnly(Some(ValueType::I64))),
            Instruction::Loop(BlockType::ResultOnly(None)),
            Instruction::Loop(BlockType::FunctionTypeIndex(2)),
            Instruction::If(BlockType::ResultOnly(Some(ValueType::F32))),
            Instruction::If(BlockType::ResultOnly(None)),
            Instruction::If(BlockType::FunctionTypeIndex(3)),
            Instruction::Br(0),
            Instruction::Br(1),
            Instruction::BrIf(0),
            Instruction::BrIf(2),
            Instruction::BrTable(vec![0], 1),
            Instruction::BrTable(vec![0, 1, 2], 3),
            Instruction::Call(1),
            Instruction::CallIndirect(1, 0),
            Instruction::LocalGet(1),
            Instruction::LocalSet(2),
            Instruction::LocalTee(3),
            Instruction::GlobalGet(1),
            Instruction::GlobalSet(2),
            Instruction::I32Load(MemoryArgument {
                offset: 100,
                align: 2,
            }),
            Instruction::I64Load(MemoryArgument {
                offset: 200,
                align: 3,
            }),
            Instruction::MemorySize(0),
            Instruction::MemoryGrow(0),
            Instruction::I32Const(100),
            Instruction::I64Const(200),
            Instruction::F32Const(2.414),
            Instruction::F64Const(1.618),
        ];

        let text: Vec<&str> = vec![
            "unreachable",
            "block (result i32)",
            "block",
            "block (type 1)",
            "loop (result i64)",
            "loop",
            "loop (type 2)",
            "if (result f32)",
            "if",
            "if (type 3)",
            "br",
            "br 1",
            "br_if",
            "br_if 2",
            "br_table 0 1",
            "br_table 0 1 2 3",
            "call 1",
            "call_indirect (type 1)",
            "local.get 1",
            "local.set 2",
            "local.tee 3",
            "global.get 1",
            "global.set 2",
            "i32.load offset=100 align=4",
            "i64.load offset=200 align=8",
            "memory.size",
            "memory.grow",
            "i32.const 100",
            "i64.const 200",
            "f32.const 2.414",
            "f64.const 1.618",
        ];

        assert_eq!(
            instructions
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>(),
            text
        );
    }
}
