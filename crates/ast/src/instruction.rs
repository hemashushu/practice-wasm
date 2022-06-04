// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::ValueType;

/// # WebAssembly 指令列表
///
/// <https://webassembly.github.io/spec/core/syntax/instructions.html>
///
/// WebAssembly 指令存在于：
/// - `代码段`（`Code Section`）里，每个函数体一个段落，
///   每个段落由连续的一个或多个指令组成且均由 `end` 指令表示结束。
/// - `global`、`element`、`data` 等条目的初始值和偏移值表达式里，每条表达式
///   通常由一个 `const` 指令和一个 `end` 指令构成。
///
/// 需注意的是，多个指令指令之间实际上是并排的关系，而不是层次型的。
/// WebAssembly 示例经常以文本格式书写，而文本格式因为允许折叠和内联，
/// 看起来就像是有结构的 S-Expression，
/// 诸如 `if`、`block`、`loop` 等指令的文本格式，往往以层次结构的形式书写。
/// 河马蜀黍认为这很容易误导阅读者，让阅读者认为 WebAssembly 的指令是有层次结构的，
/// 而事实上就跟多数的虚拟机指令一样，指令之间是并排的关系。
/// 另外 WebAssembly 的规范文档书写得实在一言难尽，真心希望学习、研究 WebAssembly 的
/// 同学能先看到这段话。
///
/// WebAssembly 的有些指令需要参数，而参数的形式有以下几种：
///
/// - 直接在指令当中的立即数，比如 `br` 指令带有一个表示 `相对层次` 的参数；
/// - 来自操作数栈，比如 `i32.add` 指令，两个参数均从操作数栈顶弹出而获得；
/// - 结合了上面的两种形式，比如 `br_if` 指令，除了表示 `相对层次` 的参数的直接数，
///   还有一个表示 `条件` 的参数从操作数栈顶获得。
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Unreachable,
    Nop,

    // 注意，block label index 是结构块的索引，即结构块出现的次序，比如第一个出现的
    // 结构块其索引值为 0，第三个出现的结构块其索引值为 2。索引值仅跟出现次序有关，跟结构块
    // 的层次结构关系无关。
    // 结构块的索引值是由解析器自动产生的，不属于指令的一部分，
    // 之所以记录这个索引值，是为了方便跟结构块的 block label name 关联起来。
    Block(BlockType, u32), // params: (block_type, block_label_index)
    Loop(BlockType, u32),  // params: (block_type, block_label_index)
    If(BlockType, u32),    // params: (block_type, block_label_index)
    Else,
    End,

    Br(u32),                // params: (relative_deepth)
    BrIf(u32),              // params: (relative_deepth)
    BrTable(Vec<u32>, u32), // params: (relative_depths, default_relative_depth)
    Return,
    Call(u32),              // params: (function_index)
    CallIndirect(u32, u32), // params: (type_index, table_index)

    Drop,
    Select,

    LocalGet(u32),  // params: (local_variable_index)
    LocalSet(u32),  // params: (local_variable_index)
    LocalTee(u32),  // params: (global_variable_index)
    GlobalGet(u32), // params: (global_variable_index)
    GlobalSet(u32), // params: (global_variable_index)

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
    MemorySize(u32), // params: (memory_block_index)
    MemoryGrow(u32), // params: (memory_block_index)

    I32Const(i32), // params: (immediate_number_value)
    I64Const(i64), // params: (immediate_number_value)
    F32Const(f32), // params: (immediate_number_value)
    F64Const(f64), // params: (immediate_number_value)

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
    ResultI32, //
    ResultI64, //
    ResultF32, //
    ResultF64, //
    ResultEmpty,
    TypeIndex(u32),
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
    pub align: u32, // 这里记录的是跟二进制格式里所存储的一致的数值，也就是指数。
    pub offset: u32,
}
