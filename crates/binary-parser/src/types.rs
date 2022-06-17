// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


/// 二进制模块以一个 4 个字节的幻数 `0x00 0x61 0x73 0x6d` 开始。
/// 转成 ascii 则是 `0x00` 和 `asm`
pub const MAGIC_NUMBER: u32 = 0x6d736100;

/// 二进制格式的版本号，占用了 4 个字节
/// 当前解析器只支持版本 1（little endian）
pub const VERSION: u32 = 0x00000001;

/// 二进制各个段（section）的 id
pub const SECTION_CUSTOM_ID: u8 = 0; // 0
pub const SECTION_TYPE_ID: u8 = 1; // 1
pub const SECTION_IMPORT_ID: u8 = 2; // 2
pub const SECTION_FUNCTION_ID: u8 = 3; // 3
pub const SECTION_TABLE_ID: u8 = 4; // 4
pub const SECTION_MEMORY_ID: u8 = 5; // 5
pub const SECTION_GLOBAL_ID: u8 = 6; // 6
pub const SECTION_EXPORT_ID: u8 = 7; // 7
pub const SECTION_START_ID: u8 = 8; // 8
pub const SECTION_ELEMENT_ID: u8 = 9; // 9
pub const SECTION_CODE_ID: u8 = 10; // 10
pub const SECTION_DATA_ID: u8 = 11; // 11

/// 在 `函数类型段` 里的 `类型项` 的 tag 值目前只能是 `0x60`
pub const FUNCTION_TYPE_TAG: u8 = 0x60;

/// 值类型的 tag
pub const VALUE_TYPE_TAG_I32: u8 = 0x7f; // i32
pub const VALUE_TYPE_TAG_I64: u8 = 0x7E; // i64
pub const VALUE_TYPE_TAG_F32: u8 = 0x7D; // f32
pub const VALUE_TYPE_TAG_F64: u8 = 0x7C; // f64

/// 导入项描述 tag
pub const IMPORT_TAG_FUNCTION: u8 = 0;
pub const IMPORT_TAG_TABLE: u8 = 1;
pub const IMPORT_TAG_MEMORY: u8 = 2;
pub const IMPORT_TAG_GLOBAL: u8 = 3;

/// 表项的 tag，目前只支持 func_ref
pub const TABLE_TYPE_TAG_FUNC_REF: u8 = 0x70;

/// 全局变量的可变性 tag，0 == 常量
pub const GLOBAL_VARIABLE_TAG_IMMUTABLE: u8 = 0;

/// 全局变量的可变性 tag，1 == 变量
pub const GLOBAL_VARIABLE_TAG_MUTABLE: u8 = 1;

/// 块结构的返回值类型
/// 块结构（比如 `block`，`loop`，`if` 等）能够返回一个值，
/// 可以理解为一种内嵌的（无自己局部变量的）函数。
pub const BLOCK_TYPE_I32: i32 = -1; // 返回 i32
pub const BLOCK_TYPE_I64: i32 = -2; // 返回 i64
pub const BLOCK_TYPE_F32: i32 = -3; // 返回 f32
pub const BLOCK_TYPE_F64: i32 = -4; // 返回 f64
pub const BLOCK_TYPE_EMPTY: i32 = -64; // 无返回

pub const EXPORT_TAG_FUNCTION: u8 = 0;
pub const EXPORT_TAG_TABLE: u8 = 1;
pub const EXPORT_TAG_MEM: u8 = 2;
pub const EXPORT_TAG_GLOBAL: u8 = 3;

pub const NAME_COLLECTION_KIND_FUNCTION_NAMES: u8 = 0x01;
pub const NAME_COLLECTION_KIND_FUNCTION_LOCAL_VARIABLE_NAMES: u8 = 0x02;
pub const NAME_COLLECTION_KIND_FUNCTION_BLOCK_LABELS: u8 = 0x03;
pub const NAME_COLLECTION_KIND_TYPE_NAMES: u8 = 0x04;
pub const NAME_COLLECTION_KIND_TABLE_NAMES: u8 = 0x05;
pub const NAME_COLLECTION_KIND_MEMORY_BLOCK_NAMES: u8 = 0x06;
pub const NAME_COLLECTION_KIND_GLOBAL_VARIABLE_NAMES: u8 = 0x07;
pub const NAME_COLLECTION_KIND_ELEMENT_NAMES: u8 = 0x08;
pub const NAME_COLLECTION_KIND_DATA_NAMES: u8 = 0x09;
