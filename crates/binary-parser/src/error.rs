// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    /// 不支持的功能
    /// 比如读取索引值为非 0 的内存块或者表
    Unsupported(String),

    /// 语法错误
    /// 比如不符合规范的数值，
    /// 对于暂时无法解析或者不支持的数据一般情况下使用 ParseError::Unsupported，仅当非常明确
    /// 是语法的错误才使用 ParseError::SyntaxError
    SyntaxError(String),

    /// leb128 编码或者 UTF-8 编码错误
    DecodingError,

    /// 未预料的结束，即预期的内容不完整
    /// 比如解析一个函数时，尚未到达末尾源文件就已经到末尾了。
    UnexpectedEnd,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Unsupported(m) => write!(f, "{}", m),
            ParseError::SyntaxError(m) => write!(f, "{}", m),
            ParseError::DecodingError => write!(f, "{}", "decoding error"),
            ParseError::UnexpectedEnd => write!(f, "{}", "unexpected end"),
        }
    }
}
