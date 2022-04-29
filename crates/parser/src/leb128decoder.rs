// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// # leb128 整数编码
//
// 一种变长的整数编码格式
// i32 编码后有 1~5 字节长
// i64 编码后有 1~10 字节长
// https://en.wikipedia.org/wiki/LEB128
//
// ## 原理
//
// 每个字节如果最高位（第 7 位，从 0 开始数）为 1，则表示还有后续的内容。比如
//
// byte 0     byte 1     byte 2
// ---------  ---------  ---------
// 1000 0001, 1000 0010, 0000 0000
// ^          ^          ^
// |--有后续   |--有后续   |--无后续
//
// 每个字节的低 7 位拼接在一起得出整数：
//
// byte 0   byte 1   byte 2
// -------- -------- --------
// 000 0000 000 0010 000 0001
// -----/\--------/\--------/
// byte 2' byte 1'  byte 0'
//
// 对于无符号数，需要从最高位开始补上零个或多个 0 以形成 u32 或者 u64。
//
// ## 有符号整数
//
// 对于有符号整数，拼接后的最高位是符号位，比如对于一个 i32 数：
//
// byte 0     byte 1
// ---------  ---------
// 1000 0001, 0111 1010
// ^          ^^
// |          ||--- 符号位
// |--有后续   |---- 无后续
//
// 拼接后的整数：
//
// 111 1010 000 0001
// ------/\--------/
// byte 1   byte 0
//
// 最高位是 1，表示这个数是负数，需要从最高位开始补上零个或多个 1 以形成 i32 或者 i64
//
// 11 111001 00000001
// || -----/ \------/
// || byte 1  byte 0
// ||
// ^^---补上的 1（实际上前面还需要补上 16 个 1 才能形成 int32，这里省略了）
//
// 注意
// 对于 32 位整数，最多有效的位是：前 4 字节（4*7=28位）+ 第 5 个字节的低端 4 位（32-28=4），也就是说，
// 对于无符号数，第 5 个字节剩余高端必须是 `0000`；对于有符号数，剩余高端必须是 `0000`` 或者 `0111`
//
// 对于 64 位整数，最多有效的位是：前 9 字节（9*7=63位）+ 第 10 个字节的低端 1 位（64-63=1），也就是说，
// 对于无符号数，第 10 个字节剩余高端必须是 `0000 000`；对于有符号数，剩余高端必须是 `0000 000` 或者 `0111 111`

/// leb128 解码的错误
#[derive(Debug)]
pub enum DecodeError {
    Incorrect(&'static str),
    Overflow,
}

const CONTINUE_BIT: u8 = 0b1000_0000;
const SIGN_BIT: u8 = 0b0100_0000;

const INT64_LAST_BYTE_INDEX: usize = 9;
const INT32_LAST_BYTE_INDEX: usize = 4;

/// 解码无符号 64 位整型
/// 返回解码后的 u64 数值以及该数值在 leb128 编码中实际有效的字节数
///
/// # 示例
///
/// ```
/// use anvm_parser::leb128decoder::decode_u64;
/// let data: [u8; 4] = [0b1010_1111, 0b0101_1010, 0b1100_0011, 0b0011_1100];
/// match decode_u64(&data) {
///     Ok((value, length)) => {
///         assert_eq!(value, 0b101_1010_010_1111);
///         assert_eq!(length, 2);
///     }
///     _ => panic!("decode u64 failed"),
/// }
/// ```
pub fn decode_u64(data: &[u8]) -> Result<(u64, usize), DecodeError> {
    let mut result: u64 = 0;
    let mut index: usize = 0;
    let mut remains = data;

    loop {
        remains = if let Some((byte, rest)) = remains.split_first() {
            result |= ((byte & 0b0111_1111) as u64) << (index * 7);

            if index == INT64_LAST_BYTE_INDEX {
                // 已到达最后一个字节

                if byte & CONTINUE_BIT != 0 {
                    // 最后一个字节的索引 7 位置的比特值应该为 0
                    return Err(DecodeError::Overflow);
                }

                if byte & 0b0111_1110 != 0 {
                    // 超出 64 位部分的比特值应该都为 0，否则视为溢出
                    // 最后一个字节有 64 - (9*7) = 1 位有效位
                    return Err(DecodeError::Overflow);
                }
            }

            if byte & CONTINUE_BIT == 0 {
                // 当前数值的字节序列已完结，返回结果
                return Ok((result, index + 1));
            }

            index += 1;
            rest
        } else {
            // 字节序列不完整
            return Err(DecodeError::Incorrect("incomplete leb128 byte sequence"));
        }
    }
}

/// 解码有符号 64 位整型
/// 返回解码后的 i64 数值以及该数值在 leb128 编码中实际有效的字节数
///
/// # 示例
///
/// ```
/// use anvm_parser::leb128decoder::decode_i64;
/// let d1: [u8; 4] = [0b1010_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
/// match decode_i64(&d1) {
///     Ok((value, length)) => {
///         assert_eq!(value, 0xffffffffffffed2fu64 as i64);
///         assert_eq!(length, 2);
///     }
///     _ => panic!("decode i64 failed"),
/// }
/// ```
pub fn decode_i64(data: &[u8]) -> Result<(i64, usize), DecodeError> {
    let mut result: i64 = 0;
    let mut index: usize = 0;
    let mut remains = data;

    loop {
        remains = if let Some((byte, rest)) = remains.split_first() {
            result |= ((byte & 0b0111_1111) as i64) << (index * 7);

            if index == INT64_LAST_BYTE_INDEX {
                // 否已到达最后一个字节

                if byte & CONTINUE_BIT != 0 {
                    // 最后一个字节的索引 7 位置的比特值应该为 0
                    return Err(DecodeError::Overflow);
                }

                // 检查符号位
                if byte & SIGN_BIT == 0 {
                    // 当前为正数，则超出 64 部分的比特值应该都为 0，否则视为溢出
                    // 最后一个字节有 64 - (9*7) = 1 位有效位
                    if byte & 0b0011_1110 != 0 {
                        return Err(DecodeError::Overflow);
                    }
                } else {
                    // 当前为负数，则超出 64 部分的比特值应该都为 1，否则视为错误
                    // 最后一个字节有 64 - (9*7) = 1 位有效位
                    if byte & 0b0011_1110 != 0b0011_1110 {
                        return Err(DecodeError::Incorrect("invalid negative"));
                    }
                }
            }

            if byte & CONTINUE_BIT == 0 {
                // 当前数值的字节序列已完结
                // 检查最后一个字节的索引 6 位置的比特值（即符号位）是否为 1（即负数），
                // 是的话还需要把高端所有位都补上 1 以形成 i64。
                if byte & SIGN_BIT != 0 {
                    result |= (-1 as i64) << (index + 1) * 7
                }

                // 返回结果
                return Ok((result, index + 1));
            }

            index += 1;
            rest
        } else {
            // 字节序列不完整
            return Err(DecodeError::Incorrect("incomplete leb128 byte sequence"));
        }
    }
}

/// 解码无符号 32 位整型
/// 返回解码后的 u32 数值以及该数值在 leb128 编码中实际有效的字节数
pub fn decode_u32(data: &[u8]) -> Result<(u32, usize), DecodeError> {
    let mut result: u32 = 0;
    let mut index: usize = 0;
    let mut remains = data;

    loop {
        remains = if let Some((byte, rest)) = remains.split_first() {
            result |= ((byte & 0b0111_1111) as u32) << (index * 7);

            if index == INT32_LAST_BYTE_INDEX {
                // 已到达最后一个字节

                if byte & CONTINUE_BIT != 0 {
                    // 最后一个字节的索引 7 位置的比特值应该为 0
                    return Err(DecodeError::Overflow);
                }

                if byte & 0b0111_0000 != 0 {
                    // 超出 32 位部分的比特值应该都为 0，否则视为溢出
                    // 最后一个字节有 32 - (4*7) = 4 位有效位
                    return Err(DecodeError::Overflow);
                }
            }

            if byte & CONTINUE_BIT == 0 {
                // 当前数值的字节序列已完结，返回结果
                return Ok((result, index + 1));
            }

            index += 1;
            rest
        } else {
            // 字节序列不完整
            return Err(DecodeError::Incorrect("incomplete leb128 byte sequence"));
        }
    }
}

/// 解码有符号 32 位整型
/// 返回解码后的 i32 数值以及该数值在 leb128 编码中实际有效的字节数
pub fn decode_i32(data: &[u8]) -> Result<(i32, usize), DecodeError> {
    let mut result: i32 = 0;
    let mut index: usize = 0;
    let mut remains = data;

    loop {
        remains = if let Some((byte, rest)) = remains.split_first() {
            result |= ((byte & 0b0111_1111) as i32) << (index * 7);

            if index == INT32_LAST_BYTE_INDEX {
                // 否已到达最后一个字节

                if byte & CONTINUE_BIT != 0 {
                    // 最后一个字节的索引 7 位置的比特值应该为 0
                    return Err(DecodeError::Overflow);
                }

                // 检查符号位
                if byte & SIGN_BIT == 0 {
                    // 当前为正数，则超出 32 部分的比特值应该都为 0，否则视为溢出
                    // 最后一个字节有 32 - (4*7) = 4 位有效位
                    if byte & 0b0011_0000 != 0 {
                        return Err(DecodeError::Overflow);
                    }
                } else {
                    // 当前为负数，则超出 32 部分的比特值应该都为 1，否则视为错误
                    // 最后一个字节有 32 - (4*7) = 4 位有效位
                    if byte & 0b0011_0000 != 0b0011_0000 {
                        return Err(DecodeError::Incorrect("invalid negative"));
                    }
                }
            }

            if byte & CONTINUE_BIT == 0 {
                // 当前数值的字节序列已完结
                // 检查最后一个字节的索引 6 位置的比特值（即符号位）是否为 1（即负数），
                // 是的话还需要把高端所有位都补上 1 以形成 i32。
                if byte & SIGN_BIT != 0 {
                    result |= (-1 as i32) << (index + 1) * 7
                }

                // 返回结果
                return Ok((result, index + 1));
            }

            index += 1;
            rest
        } else {
            // 字节序列不完整
            return Err(DecodeError::Incorrect("incomplete leb128 byte sequence"));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::leb128decoder::DecodeError;

    use super::{decode_i32, decode_i64, decode_u32, decode_u64};

    #[test]
    fn test_decode_u64() {
        // 测试 1 个字节长度
        let d0: [u8; 4] = [0b0000_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_u64(&d0) {
            Ok((value, length)) => {
                assert_eq!(value, 0b0000_1111);
                assert_eq!(length, 1);
            }
            _ => panic!("decode u64 failed"),
        }

        // 测试 2 个字节长度
        let d1: [u8; 4] = [0b1010_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_u64(&d1) {
            Ok((value, length)) => {
                assert_eq!(value, 0b101_1010_010_1111);
                assert_eq!(length, 2);
            }
            _ => panic!("decode u64 failed"),
        }

        // 测试 3 个字节长度
        let d2: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b0100_0011, 0b0011_1100];
        match decode_u64(&d2) {
            Ok((value, length)) => {
                assert_eq!(value, 0b100_0011_101_1010_010_1111);
                assert_eq!(length, 3);
            }
            _ => panic!("decode u64 failed"),
        }

        // 测试 4 个字节长度
        let d3: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b0011_1100];
        match decode_u64(&d3) {
            Ok((value, length)) => {
                assert_eq!(value, 0b111100_1000011_1011010_0101111);
                assert_eq!(length, 4);
            }
            _ => panic!("decode u64 failed"),
        }

        // 测试所有比特位皆为 1，即 u64 最大值
        let d4: [u8; 10] = [
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0b0000_0001,
        ];
        match decode_u64(&d4) {
            Ok((value, length)) => {
                assert_eq!(value, u64::MAX);
                assert_eq!(length, 10);
            }
            _ => panic!("decode u64 failed"),
        }

        // 测试字节序列不完整的情况
        let d5: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b1111_1100];
        match decode_u64(&d5) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Incorrect(_)));
            }
            _ => {
                panic!("decode u64 failed")
            }
        }

        // 测试最后一个字节仍未结束的情况
        let d6: [u8; 10] = [0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9];
        match decode_u64(&d6) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode u64 failed")
            }
        }

        // 测试最后一个字节有溢出比特的情况
        let d7: [u8; 10] = [
            0xf0,
            0xf1,
            0xf2,
            0xf3,
            0xf4,
            0xf5,
            0xf6,
            0xf7,
            0xf8,
            0b0000_0010,
        ];
        match decode_u64(&d7) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode u64 failed")
            }
        }
    }

    #[test]
    fn test_decode_i64() {
        // 测试 1 个字节长度
        let d0: [u8; 4] = [0b0000_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_i64(&d0) {
            Ok((value, length)) => {
                assert_eq!(value, 0b0000_1111);
                assert_eq!(length, 1);
            }
            _ => panic!("decode i64 failed"),
        }

        // 测试 2 个字节长度（这个是负数）
        let d1: [u8; 4] = [0b1010_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_i64(&d1) {
            Ok((value, length)) => {
                assert_eq!(value, 0xffffffffffffed2fu64 as i64);
                // 前面补上 64-(2*7=14)=50个1
                // 0b11111111_11111111_11111111_11111111_11111111_11111111_11____101_1010_010_1111
                assert_eq!(length, 2);
            }
            _ => panic!("decode i64 failed"),
        }

        // 测试 3 个字节长度（这个是负数）
        let d2: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b0100_0011, 0b0011_1100];
        match decode_i64(&d2) {
            Ok((value, length)) => {
                assert_eq!(value, 0xfffffffffff0ed2fu64 as i64);
                // 前面补上 64-(3*7=21)=43个1
                // 0b11111111_11111111_11111111_11111111_11111111_111____100_0011_101_1010_010_1111
                assert_eq!(length, 3);
            }
            _ => panic!("decode i64 failed"),
        }

        // 测试 4 个字节长度
        let d3: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b0011_1100];
        match decode_i64(&d3) {
            Ok((value, length)) => {
                assert_eq!(value, 0b111100_1000011_1011010_0101111);
                assert_eq!(length, 4);
            }
            _ => panic!("decode i64 failed"),
        }

        // 测试所有比特位皆为 1，即 i64 的 -1
        let d4: [u8; 10] = [
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0xff,
            0b0000_0001,
        ];
        match decode_i64(&d4) {
            Ok((value, length)) => {
                assert_eq!(value, -1);
                assert_eq!(length, 10);
            }
            _ => panic!("decode i64 failed"),
        }

        // 测试字节序列不完整的情况
        let d5: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b1111_1100];
        match decode_i64(&d5) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Incorrect(_)));
            }
            _ => {
                panic!("decode i64 failed")
            }
        }

        // 测试最后一个字节仍未结束的情况
        let d6: [u8; 10] = [0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9];
        match decode_i64(&d6) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode i64 failed")
            }
        }

        // 测试最后一个字节有溢出比特的情况
        let d6: [u8; 10] = [
            0xf0,
            0xf1,
            0xf2,
            0xf3,
            0xf4,
            0xf5,
            0xf6,
            0xf7,
            0xf8,
            0b0000_0010,
        ];
        match decode_i64(&d6) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode i64 failed")
            }
        }

        // 测试最后一个字节的符号位为 1 但其他超出 64 位的高端位不为 1 的情况
        let d7: [u8; 10] = [
            0xf0,
            0xf1,
            0xf2,
            0xf3,
            0xf4,
            0xf5,
            0xf6,
            0xf7,
            0xf8,
            0b0100_0010,
        ];
        match decode_i64(&d7) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Incorrect(_)));
            }
            _ => {
                panic!("decode i64 failed")
            }
        }
    }

    #[test]
    fn test_decode_u32() {
        // 测试 1 个字节长度
        let d0: [u8; 4] = [0b0000_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_u32(&d0) {
            Ok((value, length)) => {
                assert_eq!(value, 0b0000_1111);
                assert_eq!(length, 1);
            }
            _ => panic!("decode u32 failed"),
        }

        // 测试 2 个字节长度
        let d1: [u8; 4] = [0b1010_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_u32(&d1) {
            Ok((value, length)) => {
                assert_eq!(value, 0b101_1010_010_1111);
                assert_eq!(length, 2);
            }
            _ => panic!("decode u32 failed"),
        }

        // 测试 3 个字节长度
        let d2: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b0100_0011, 0b0011_1100];
        match decode_u32(&d2) {
            Ok((value, length)) => {
                assert_eq!(value, 0b100_0011_101_1010_010_1111);
                assert_eq!(length, 3);
            }
            _ => panic!("decode u32 failed"),
        }

        // 测试 4 个字节长度
        let d3: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b0011_1100];
        match decode_u32(&d3) {
            Ok((value, length)) => {
                assert_eq!(value, 0b111100_1000011_1011010_0101111);
                assert_eq!(length, 4);
            }
            _ => panic!("decode u32 failed"),
        }

        // 测试所有比特位皆为 1，即 u32 最大值
        let d4: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0b0000_1111];
        match decode_u32(&d4) {
            Ok((value, length)) => {
                assert_eq!(value, u32::MAX);
                assert_eq!(length, 5);
            }
            _ => panic!("decode u32 failed"),
        }

        // 测试字节序列不完整的情况
        let d5: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b1111_1100];
        match decode_u32(&d5) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Incorrect(_)));
            }
            _ => {
                panic!("decode u32 failed")
            }
        }

        // 测试最后一个字节仍未结束的情况
        let d6: [u8; 5] = [0xf0, 0xf1, 0xf2, 0xf3, 0xf4];
        match decode_u32(&d6) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode u32 failed")
            }
        }

        // 测试最后一个字节有溢出比特的情况
        let d7: [u8; 5] = [0xf0, 0xf1, 0xf2, 0xf3, 0b0001_0000];
        match decode_u32(&d7) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode u32 failed")
            }
        }
    }

    #[test]
    fn test_decode_i32() {
        // 测试 1 个字节长度
        let d0: [u8; 4] = [0b0000_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_i32(&d0) {
            Ok((value, length)) => {
                assert_eq!(value, 0b0000_1111);
                assert_eq!(length, 1);
            }
            _ => panic!("decode i32 failed"),
        }

        // 测试 2 个字节长度（这个是负数）
        let d1: [u8; 4] = [0b1010_1111, 0b0101_1010, 0b0100_0011, 0b0011_1100];
        match decode_i32(&d1) {
            Ok((value, length)) => {
                assert_eq!(value, 0xffffed2fu32 as i32);
                // 前面补上 32-(2*7=14)=18个1
                // 0b11111111_11111111_11____101_1010_010_1111
                assert_eq!(length, 2);
            }
            _ => panic!("decode i32 failed"),
        }

        // 测试 3 个字节长度（这个是负数）
        let d2: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b0100_0011, 0b0011_1100];
        match decode_i32(&d2) {
            Ok((value, length)) => {
                assert_eq!(value, 0xfff0ed2fu32 as i32);
                // 前面补上 32-(3*7=21)=11个1
                // 0b11111111_111____100_0011_101_1010_010_1111
                assert_eq!(length, 3);
            }
            _ => panic!("decode i32 failed"),
        }

        // 测试 4 个字节长度
        let d3: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b0011_1100];
        match decode_i32(&d3) {
            Ok((value, length)) => {
                assert_eq!(value, 0b111100_1000011_1011010_0101111);
                assert_eq!(length, 4);
            }
            _ => panic!("decode i32 failed"),
        }

        // 测试所有比特位皆为 1，即 i32 的 -1
        let d4: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0b0000_1111];
        match decode_i32(&d4) {
            Ok((value, length)) => {
                assert_eq!(value, -1);
                assert_eq!(length, 5);
            }
            _ => panic!("decode i32 failed"),
        }

        // 测试字节序列不完整的情况
        let d5: [u8; 4] = [0b1010_1111, 0b1101_1010, 0b1100_0011, 0b1111_1100];
        match decode_i32(&d5) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Incorrect(_)));
            }
            _ => {
                panic!("decode i32 failed")
            }
        }

        // 测试最后一个字节仍未结束的情况
        let d6: [u8; 5] = [0xf0, 0xf1, 0xf2, 0xf3, 0xf4];
        match decode_i32(&d6) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode i32 failed")
            }
        }

        // 测试最后一个字节有溢出比特的情况
        let d6: [u8; 5] = [0xf0, 0xf1, 0xf2, 0xf3, 0b0001_0000];
        match decode_i32(&d6) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Overflow));
            }
            _ => {
                panic!("decode i32 failed")
            }
        }

        // 测试最后一个字节的符号位为 1 但其他超出 32 位的高端位不为 1 的情况
        let d7: [u8; 5] = [0xf0, 0xf1, 0xf2, 0xf3, 0b0101_0000];
        match decode_i32(&d7) {
            Err(e) => {
                assert!(matches!(e, DecodeError::Incorrect(_)));
            }
            _ => {
                panic!("decode i32 failed")
            }
        }
    }
}
