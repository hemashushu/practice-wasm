// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{ast::Module, leb128decoder};

pub fn parse(source: &[u8]) -> Result<Module, ParseError> {
    todo!()
}

pub enum ParseError {
    Something(&'static str),

    /// 不支持的操作
    /// 比如读取索引值为非 0 的内存块或者表
    Unsupported,

    /// leb128 编码或者 UTF-8 编码错误
    DecodingError,

    /// 未预料的结束，即预期的内容不完整
    /// 比如解析一个函数时，尚未到达末尾源文件就已经到末尾了。
    UnexpectedEnd,
}

/// 读取一个字节的内容
/// 返回一个 u8 整数
fn read_byte(source: &[u8]) -> Result<(u8, &[u8]), ParseError> {
    match source.split_first() {
        Some((first, rest)) => Ok((*first, rest)),
        None => Err(ParseError::UnexpectedEnd),
    }
}

fn read_n_bytes(source: &[u8], length: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if length > source.len() {
        Err(ParseError::UnexpectedEnd)
    } else {
        Ok(source.split_at(length))
    }
}

fn read_4_bytes(source: &[u8]) -> Result<([u8; 4], &[u8]), ParseError> {
    let (bytes, remains) = read_n_bytes(source, 4)?;

    let mut buf: [u8; 4] = [0; 4];
    bytes
        .iter()
        .enumerate()
        .for_each(|(index, value)| buf[index] = *value);
    Ok((buf, remains))
}

fn read_8_bytes(source: &[u8]) -> Result<([u8; 8], &[u8]), ParseError> {
    let (bytes, remains) = read_n_bytes(source, 8)?;

    let mut buf: [u8; 8] = [0; 8];
    bytes
        .iter()
        .enumerate()
        .for_each(|(index, value)| buf[index] = *value);
    Ok((buf, remains))
}

/// 读取固定长度（4 字节）u32
fn read_fixed_u32(source: &[u8]) -> Result<(u32, &[u8]), ParseError> {
    let (bytes, remains) = read_4_bytes(source)?;
    Ok((u32::from_le_bytes(bytes), remains))
}

/// 读取变长（leb128 编码的）u32
fn read_u32(source: &[u8]) -> Result<(u32, &[u8]), ParseError> {
    match leb128decoder::decode_u32(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取变长（leb128 编码的）i32
fn read_i32(source: &[u8]) -> Result<(i32, &[u8]), ParseError> {
    match leb128decoder::decode_i32(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取变长（leb128 编码的）i64
fn read_i64(source: &[u8]) -> Result<(i64, &[u8]), ParseError> {
    match leb128decoder::decode_i64(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取固定长度的 f32
fn read_f32(source: &[u8]) -> Result<(f32, &[u8]), ParseError> {
    let (bytes, remains) = read_4_bytes(source)?;
    Ok((f32::from_le_bytes(bytes), remains))
}

/// 读取固定长度的 f64
fn read_f64(source: &[u8]) -> Result<(f64, &[u8]), ParseError> {
    let (bytes, remains) = read_8_bytes(source)?;
    Ok((f64::from_le_bytes(bytes), remains))
}

/// 读取如下结构的 u32 数组
/// `length:u32 + u32{*}`
fn read_u32_vec(source: &[u8]) -> Result<(Vec<u32>, &[u8]), ParseError> {
    let mut remains = source;
    let (length, remains) = read_u32(remains)?;

    let mut list: Vec<u32> = vec![];
    for _ in [0..length] {
        let (value, remains) = read_u32(remains)?;
        list.push(value);
    }

    Ok((list, remains))
}

/// 读取如下结构的 byte 数组
/// `length:u32 + u8{*}`
fn read_byte_vec(source: &[u8]) -> Result<(&[u8], &[u8]), ParseError> {
    let mut remains = source;
    let (length, remains) = read_u32(remains)?;
    read_n_bytes(source, length as usize)
}

/// 读取如下结构的 byte 数组并以 UTF-8 编码转换为 String
/// `length:u32 + u8{*}`
fn read_string(source: &[u8]) -> Result<(String, &[u8]), ParseError> {
    let mut remains = source;
    let (bytes, remains) = read_byte_vec(remains)?;
    match String::from_utf8(bytes.into()) {
        Ok(s) => {
            Ok((s, remains))
        },
        _=>{
            Err(ParseError::DecodingError)
        }
    }
}

/// 读取一个字节并断言其值为 0
/// 用于读取 memory/data 和 table/element 等需要指定目标对象索引值，
/// 但该索引值只能是 0 的场合
fn consume_byte_zero(source: &[u8]) -> Result<&[u8], ParseError> {
    let (byte, remains) = read_byte(source)?;
    if (byte != 0) {
        Err(ParseError::Unsupported)
    }else {
        Ok(remains)
    }
}
