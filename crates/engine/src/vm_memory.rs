// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_parser::ast::{Limit, MemoryType};

use crate::instance::{EngineError, Memory};

/// 内存的容量单位是 `页`（`page`）
/// 一页内存为 65536 个字节
const PAGE_SIZE: u32 = 65536;

/// WebAssembly 约定内存块最大只能有 65536 个页面
const MAX_PAGES: u32 = 65536;

/// 当前使用字节数组来充当内存
pub struct VMMemory {
    memory_type: MemoryType,
    data: Vec<u8>,
}

impl VMMemory {
    pub fn new(memory_type: MemoryType) -> Self {
        let min = memory_type.limit.get_min();
        let byte_len = min * PAGE_SIZE;

        VMMemory {
            memory_type: memory_type,
            data: vec![0; byte_len as usize], //Vec::<u8>::with_capacity(byte_len as usize),
        }
    }

    /// 以给定的初始数据来创建 VMMemory 对象
    pub fn new_with_init_data(init_data: Vec<u8>) -> Self {
        let byte_len = init_data.len() as u32;
        let page_count = (byte_len - 1) / PAGE_SIZE + 1;

        // 对齐到整页
        let mut data = Vec::from(init_data);
        data.resize((page_count * PAGE_SIZE) as usize, 0);

        let memory_type = MemoryType {
            limit: Limit::new(page_count, Some(page_count)),
        };

        VMMemory {
            memory_type: memory_type,
            data: data,
        }
    }
}

impl Memory for VMMemory {
    fn get_page_count(&self) -> u32 {
        self.data.len() as u32 / PAGE_SIZE
    }

    fn incrase_page(&mut self, increase_number: u32) -> Result<u32, EngineError> {
        let old_page_count = self.get_page_count();
        let new_page_count = old_page_count + increase_number;

        // 如果 MemoryType 的 limit 成员不指定 max 值，则可以
        // 增长到 WebAssembly 内存块最大允许的页面数 MAX_PAGES
        if let Limit::Range(_, max) = self.memory_type.limit {
            if new_page_count > max {
                return Err(EngineError::Overflow(
                    "exceeded the maximum page of the memory".to_string(),
                ));
            }
        }

        if new_page_count >= MAX_PAGES {
            return Err(EngineError::Overflow(
                "exceeded the maximum number of pages allowed in memory".to_string(),
            ));
        }

        self.data.resize((new_page_count * PAGE_SIZE) as usize, 0u8);

        Ok(old_page_count)
    }

    fn get_memory_type(&self) -> MemoryType {
        self.memory_type.clone()
    }

    fn read_bytes(&self, address: usize, length: usize) -> &[u8] {
        &self.data[address..(address + length)]
    }

    fn write_bytes(&mut self, address: usize, data: &[u8]) {
        for index in 0..data.len() {
            self.data[address + index] = data[index]
        }
    }

    fn read_i8(&self, address: usize) -> i8 {
        let bytes = self.read_bytes(address, 1);
        bytes[0] as i8
    }

    // fn read_u8(&self, address: usize) -> u8 {
    //     let bytes = self.read_bytes(address, 1);
    //     bytes[0]
    // }

    fn read_i16(&self, address: usize) -> i16 {
        let bytes = self.read_bytes(address, 2);
        i16::from_le_bytes(bytes.try_into().unwrap())
    }

    // fn read_u16(&self, address: usize) -> u16 {
    //     let bytes = self.read_bytes(address, 2);
    //     u16::from_le_bytes(bytes.try_into().unwrap())
    // }

    fn read_i32(&self, address: usize) -> i32 {
        let bytes = self.read_bytes(address, 4);
        i32::from_le_bytes(bytes.try_into().unwrap())
    }

    // fn read_u32(&self, address: usize) -> u32 {
    //     let bytes = self.read_bytes(address, 4);
    //     u32::from_le_bytes(bytes.try_into().unwrap())
    // }

    fn read_i64(&self, address: usize) -> i64 {
        let bytes = self.read_bytes(address, 8);
        i64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_f32(&self, address: usize) -> f32 {
        let bytes = self.read_bytes(address, 4);
        f32::from_le_bytes(bytes.try_into().unwrap())
    }

    fn read_f64(&self, address: usize) -> f64 {
        let bytes = self.read_bytes(address, 8);
        f64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn write_i8(&mut self, address: usize, value: i8) {
        let data = i8::to_le_bytes(value);
        self.write_bytes(address, &data);
    }

    fn write_i16(&mut self, address: usize, value: i16) {
        let data = i16::to_le_bytes(value);
        self.write_bytes(address, &data);
    }

    fn write_i32(&mut self, address: usize, value: i32) {
        let data = i32::to_le_bytes(value);
        self.write_bytes(address, &data);
    }

    fn write_i64(&mut self, address: usize, value: i64) {
        let data = i64::to_le_bytes(value);
        self.write_bytes(address, &data);
    }

    fn write_f32(&mut self, address: usize, value: f32) {
        let data = f32::to_le_bytes(value);
        self.write_bytes(address, &data);
    }

    fn write_f64(&mut self, address: usize, value: f64) {
        let data = f64::to_le_bytes(value);
        self.write_bytes(address, &data);
    }
}

#[cfg(test)]
mod tests {
    use anvm_parser::ast::{Limit, MemoryType};

    use crate::instance::{EngineError, Memory};

    use super::VMMemory;

    #[test]
    fn test_increase_page() {
        let mut m0 = VMMemory::new(MemoryType {
            limit: Limit::new(4, None),
        });

        assert_eq!(m0.get_page_count(), 4);

        let n0 = m0.incrase_page(2).unwrap();
        assert_eq!(n0, 4);
        assert_eq!(m0.get_page_count(), 6);

        let n1 = m0.incrase_page(3).unwrap();
        assert_eq!(n1, 6);
        assert_eq!(m0.get_page_count(), 9);

        // 创建一个有 max page 值的内存块
        let mut m1 = VMMemory::new(MemoryType {
            limit: Limit::new(2, Some(4)),
        });
        assert_eq!(m1.get_page_count(), 2);
        assert!(matches!(m1.incrase_page(2), Ok(_)));
        assert!(matches!(m1.incrase_page(1), Err(EngineError::Overflow(_))));
    }

    #[test]
    fn test_read_write_bytes() {
        let mut m0 = VMMemory::new(MemoryType {
            limit: Limit::new(1, None),
        });

        assert_eq!(m0.read_bytes(0, 8), vec![0, 0, 0, 0, 0, 0, 0, 0]);

        m0.write_bytes(2, &vec![12, 23]);
        assert_eq!(m0.read_bytes(0, 8), vec![0, 0, 12, 23, 0, 0, 0, 0]);

        m0.write_bytes(0, &vec![11, 22, 33, 44, 55, 66]);
        assert_eq!(m0.read_bytes(0, 8), vec![11, 22, 33, 44, 55, 66, 0, 0]);
    }

    #[test]
    fn test_read_write_numbers() {
        let mut m0 = VMMemory::new(MemoryType {
            limit: Limit::new(1, None),
        });

        m0.write_i8(0, 0x11);
        m0.write_i8(8 * 1, 0x99u8 as i8);

        m0.write_i16(8 * 2, 0x2233);
        m0.write_i16(8 * 3, 0xaabbu16 as i16);

        m0.write_i32(8 * 4, 0x44556677);
        m0.write_i32(8 * 5, 0xccddeeffu32 as i32);

        m0.write_i64(8 * 6, 0x1020304050607080);
        m0.write_i64(8 * 7, 0x8090a0b0c0d0e0f0u64 as i64);

        m0.write_f32(8 * 8, 3.142);
        m0.write_f64(8 * 9, 2.718);

        assert_eq!(m0.read_i8(0), 0x11);
        assert_eq!(m0.read_i8(0) as u8, 0x11);
        assert_eq!(m0.read_i8(8 * 1), 0x99u8 as i8);
        assert_eq!(m0.read_i8(8 * 1) as u8, 0x99);

        assert_eq!(m0.read_i16(8 * 2), 0x2233);
        assert_eq!(m0.read_i16(8 * 2) as u16, 0x2233);
        assert_eq!(m0.read_i16(8 * 3), 0xaabbu16 as i16);
        assert_eq!(m0.read_i16(8 * 3) as u16, 0xaabb);

        assert_eq!(m0.read_i32(8 * 4), 0x44556677);
        assert_eq!(m0.read_i32(8 * 4) as u32, 0x44556677);
        assert_eq!(m0.read_i32(8 * 5), 0xccddeeffu32 as i32);
        assert_eq!(m0.read_i32(8 * 5) as u32, 0xccddeeffu32);

        assert_eq!(m0.read_i64(8 * 6), 0x1020304050607080);
        assert_eq!(m0.read_i64(8 * 6) as u64, 0x1020304050607080);
        assert_eq!(m0.read_i64(8 * 7), 0x8090a0b0c0d0e0f0u64 as i64);
        assert_eq!(m0.read_i64(8 * 7) as u64, 0x8090a0b0c0d0e0f0u64);

        assert_eq!(m0.read_f32(8 * 8), 3.142);
        assert_eq!(m0.read_f64(8 * 9), 2.718);
    }

    #[test]
    fn test_create_with_init_data() {
        let mut m0 = VMMemory::new_with_init_data(vec![11, 22, 33, 44, 55, 66]);

        assert_eq!(m0.get_page_count(), 1);
        assert_eq!(m0.read_bytes(0, 8), vec![11, 22, 33, 44, 55, 66, 00, 00]);

        m0.write_bytes(2, &vec![77, 88]);
        assert_eq!(m0.read_bytes(0, 8), vec![11, 22, 77, 88, 55, 66, 00, 00]);
    }
}
