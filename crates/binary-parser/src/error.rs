// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Display;

use anvm_ast::instruction::Instruction;

use crate::types;

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// 规范里存在的但 VM 暂时没实现的，
    /// 或者
    /// 已经存在规范的草稿里，但目前尚未形成正式规范的功能。
    ///
    /// 比如读取索引值为非 0 的内存块或者表
    Unsupported(Unsupported),

    /// 语法错误
    ///
    /// 指明确不符合规范的数值。
    SyntaxError(SyntaxError),

    /// 不明的数据
    ///
    /// 比如在段和段之间本不应该存在的任何数据，
    /// 如果在解析时发现此类数据，则会抛出此异常
    UnexpectedData(
        /* section_name */ String,
        /* option_paragraph_name */ Option<String>,
    ),

    /// leb128 编码或者 UTF-8 编码错误
    DecodingError,

    /// 未预料的结束
    ///
    /// 解析一个函数或者常量表达式时，找不到 end 指令。
    UnexpectedEnd,

    /// 验证失败
    Invalid,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unsupported {
    UnsupportedFormat(/* magic_number */ u32),
    UnsupportedVersion(/* version_number */ u32),
    UnsupportedTypeTag(/* tag */ u8),
    UnsupportedValueTag(/* tag */ u8),
    UnsupportedImportTag(/* tag */ u8),
    UnsupportedTableTag(/* tag */ u8),

    UnsupportMultipleTable,
    UnsupportMultipleMemoryBlock,

    UnsupportedInstructionOpcode(/* opcode */ u8),
    UnsupportedInstructionExtensionCode(/* opcode */ u8, /* extension_code */ u32),

    UnsupportedExportTag(/* tag */ u8),
}

impl Display for Unsupported {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unsupported::UnsupportedFormat(magic_number) => {
                write!(
                    f,
                    "unknown binary file magic number, expected:{} actual: {}",
                    format!("0x{:08x}", types::MAGIC_NUMBER),
                    format!("0x{:08x}", magic_number)
                )
            }
            Unsupported::UnsupportedVersion(version_number) => {
                write!(
                    f,
                    "unsupported file version, expected: {}, actual: {}",
                    types::VERSION,
                    version_number
                )
            }

            Unsupported::UnsupportedTypeTag(tag) => {
                write!(
                    f,
                    "unsupported type tag, expected: {}, actual: {}",
                    types::FUNCTION_TYPE_TAG,
                    tag
                )
            }
            Unsupported::UnsupportedValueTag(tag) => {
                write!(f, "unsupported value tag: {}", tag)
            }
            Unsupported::UnsupportedImportTag(tag) => {
                write!(f, "unsupported import tag: {}", tag)
            }
            Unsupported::UnsupportedTableTag(tag) => {
                write!(f, "unsupported table tag: {}", tag)
            }
            Unsupported::UnsupportMultipleTable => {
                write!(f, "only one table is allowed in a module")
            }
            Unsupported::UnsupportMultipleMemoryBlock => {
                write!(f, "only one memory block is allowed in a module")
            }
            Unsupported::UnsupportedInstructionOpcode(opcode) => {
                write!(f, "unsupported instruction opcode: {}", opcode)
            }
            Unsupported::UnsupportedInstructionExtensionCode(opcode, extension_code) => write!(
                f,
                "unsupported extension instruction code {} for opcode {}",
                extension_code, opcode
            ),
            Unsupported::UnsupportedExportTag(tag) => {
                write!(f, "unsupported export tag: {}", tag)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SyntaxError {
    InvalidSectionId(u8),
    InvalidGlobalTag(u8),
    InvalidLimitTag(u8),
    InvalidCustomNameSectionTag(u8),
    InvalidBlockType(i32),
    InvalidConstantExpressionInstruction(Instruction),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::InvalidSectionId(id) => {
                write!(f, "invalid section id: {}", id)
            }
            SyntaxError::InvalidGlobalTag(tag) => {
                write!(f, "invalid global tag: {}", tag)
            }
            SyntaxError::InvalidLimitTag(tag) => {
                write!(f, "invalid limit tag: {}", tag)
            }
            SyntaxError::InvalidConstantExpressionInstruction(instruction) => {
                write!(
                    f,
                    "invalid constant expression instruction \"{:?}\"",
                    instruction
                )
            }
            SyntaxError::InvalidCustomNameSectionTag(tag) => {
                write!(f, "invalid custom section \"name\" tag: {}", tag)
            }
            SyntaxError::InvalidBlockType(value) => {
                write!(f, "invalid block type: {}", value)
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Unsupported(m) => write!(f, "{}", m),
            ParseError::SyntaxError(m) => write!(f, "{}", m),
            ParseError::UnexpectedData(section_name, option_paragraph_name) => {
                if let Some(paragraph_name) = option_paragraph_name {
                    write!(
                        f,
                        "unexpected data in section \"{}\" paragraph \"{}\"",
                        section_name, paragraph_name
                    )
                } else {
                    write!(f, "unexpected data after section \"{}\"", section_name)
                }
            }
            ParseError::DecodingError => write!(f, "{}", "decoding error"),
            ParseError::UnexpectedEnd => write!(f, "{}", "unexpected end"),
            ParseError::Invalid => write!(f, "{}", "verification failure"),
        }
    }
}
