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
///
pub struct VMMemory {
    pub memory_type: MemoryType,
    pub data: Vec<u8>,
    // page_count: u32,
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

    /// 扩充内存的页面数
    ///
    /// 参数 increase_number 是 需要增加的页面的数量，而不是 "增加到" 的页面的数量。
    /// 函数的返回值：
    /// - 成功时返回旧的页面数（u32）
    /// - 失败时返回 Result::Err（注意，WebAssembly 的 memory.grow 指令约定
    ///   失败时要把被转为 u32 的 -1 压入操作数栈）
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

    fn write_bytes(&mut self, address: usize, data: &[u8], length: usize) {
        for index in 0..length {
            self.data[address + index] = data[index]
        }
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

        m0.write_bytes(2, &vec![12, 23, 34, 45], 2);
        assert_eq!(m0.read_bytes(0, 8), vec![0, 0, 12, 23, 0, 0, 0, 0]);

        m0.write_bytes(0, &vec![11, 22, 33, 44, 55, 66], 6);
        assert_eq!(m0.read_bytes(0, 8), vec![11, 22, 33, 44, 55, 66, 0, 0]);
    }

    #[test]
    fn test_create_with_init_data() {
        let mut m0 = VMMemory::new_with_init_data(vec![11, 22, 33, 44, 55, 66]);

        assert_eq!(m0.get_page_count(), 1);
        assert_eq!(m0.read_bytes(0, 8), vec![11, 22, 33, 44, 55, 66, 00, 00]);

        m0.write_bytes(2, &vec![77, 88], 2);
        assert_eq!(m0.read_bytes(0, 8), vec![11, 22, 77, 88, 55, 66, 00, 00]);
    }
}
