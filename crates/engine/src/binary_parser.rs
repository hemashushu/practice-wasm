// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::vec;

use crate::{ast::Module, leb128decoder};

pub fn parse(source: &[u8]) -> Result<Module, ParseError> {
    parse_module(source)
}

fn parse_module(source: &[u8]) -> Result<Module, ParseError> {
    /// 二进制模块以一个 4 个字节的幻数 `0x00 0x61 0x73 0x6d` 开始。
    /// 转成 ascii 则是 `0x00` 和 `asm`
    const MAGIC_NUMBER: u32 = 0x6d736100;

    /// 二进制格式的版本号，占用了 4 个字节
    /// 当前解析器只支持版本 1（little endian）
    const VERSION: u32 = 0x00000001;

    let mut remains = source;
    let (mag, post_magic_number) = read_fixed_u32(remains)?;
    if mag != MAGIC_NUMBER {
        return Err(ParseError::Unsupported);
    }
    remains = post_magic_number;

    let (ver, post_version) = read_fixed_u32(remains)?;
    if ver != VERSION {
        return Err(ParseError::Unsupported);
    }
    remains = post_version;

    parse_sections(remains)
}

fn parse_sections(source: &[u8]) -> Result<Module, ParseError> {
    Ok(Module {
        code_items: vec![],
        custom_items: vec![],
        data_items: vec![],
        element_items: vec![],
        export_items: vec![],
        function_list: vec![],
        function_types: vec![],
        global_items: vec![],
        import_items: vec![],
        memory_blocks: vec![],
        start_function_index: None,
        tables: vec![],
    })
}

#[derive(Debug)]
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
        Ok(s) => Ok((s, remains)),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取一个字节并断言其值为 0
/// 用于读取 memory/data 和 table/element 等需要指定目标对象索引值，
/// 但该索引值只能是 0 的场合
fn consume_byte_zero(source: &[u8]) -> Result<&[u8], ParseError> {
    let (byte, remains) = read_byte(source)?;
    if byte != 0 {
        Err(ParseError::Unsupported)
    } else {
        Ok(remains)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::parse;

    // 辅助方法

    fn get_test_resource_file_binary(filename: &str) -> Vec<u8> {
        let mut path_buf = env::current_dir().expect("failed to get current directory");

        // 使用 `cargo test` 测试时，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm/crates/engine`；
        //
        // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm`。
        //
        // 这里需要处理这种情况。

        if path_buf.ends_with("engine") {
            path_buf.pop();
            path_buf.pop();
        }
        let fullname_buf = path_buf.join("test/resources/parser").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!("failed to read the specified file: {}", fullname))
    }

    #[test]
    fn test_parse_module() {
        let source = get_test_resource_file_binary("test-section-class.wasm");
        let mod0 = parse(&source).unwrap();
    }
}
