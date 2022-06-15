// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// `表段` 和 `元素段` 目前用于列出一组函数，然后在执行 `call_indirect` 指令时，根据栈顶
// 的操作数获取该列表中的其中一个函数，从而实现 "动态" 选择被调用的函数。
// 相对应高级语言里的 `函数指针`（比如数据类型为 `函数` 类型的参数）
//
// `表段` 用于说明元素项目数量的范围，
// `元素段` 用于存储 `表` 的初始化数据，也就是函数的索引。
//
// 指令 call_indirect 的操作步骤：
// 1. 从操作数栈弹出一个 u32 数，该数是表内项目的索引
// 2. 从表里获取指定的项目，也就是目标函数的索引值
// 3. 通过函数索引值获取目标函数
// 4. 调用目标函数
//
// | 操作数栈。。  |       | 表。。       |     | 函数列表。。 |
// | ----------- |       | ----------- |     | ---------- |
// | -- 栈顶。 -- |       | #0 func#2   |  /--> #3 sub     |
// | 1u32 -------- pop --> #1 func#3 ----/   | #4 mul     |
// | ...         |       | #2 func#5   |     | #5 div     |
// | -- 栈底。 -- |       | ...         |     | ...        |

use anvm_ast::ast::{Limit, TableType};

use crate::error::{EngineError, Overflow, OutOfRange};

pub struct VMTable {
    /// TableType 的信息包含表的类型（目前只有函数引用类型）以及限制值（范围值）
    table_type: TableType,

    /// 函数索引列表
    elements: Vec<Option<u32>>,
}

impl VMTable {
    pub fn new(table_type: TableType) -> Self {
        let min = table_type.limit.get_min();
        VMTable {
            table_type: table_type,

            // 预先分配好空槽，因为访问者会随机访问指定的地址，所以不能仅仅
            // 分配 Vec 的容量，而应该分配空槽。
            // 空槽的初始值都是 None
            elements: vec![None; min as usize],
        }
    }

    /// 创建指定项目数量（且不限最大值的）的表
    pub fn new_by_min(min: u32) -> Self {
        let table_type = TableType {
            limit: Limit::AtLeast(min),
        };

        VMTable::new(table_type)
    }

    /// min 和 max 的值都是 `包括的`（`included`）
    pub fn new_by_page_range(min_page: u32, max_page: u32) -> Self {
        let table_type = TableType {
            limit: Limit::Range(min_page, max_page),
        };

        VMTable::new(table_type)
    }

    pub fn get_size(&self) -> u32 {
        self.elements.len() as u32
    }

    /// 返回原先的大小
    pub fn increase_size(&mut self, increase_number: u32) -> Result<u32, EngineError> {
        let old_len = self.get_size();
        let new_len = self.get_size() + increase_number;

        // 如果 TableType 的 limit 成员不指定 max 值，则可以
        // 增长到 u32 的最大值
        if let Limit::Range(_, max) = self.table_type.limit {
            if new_len > max {
                return Err(EngineError::Overflow(Overflow::TableSizeExceed(
                    new_len, max,
                )));
            }
        }

        // 新增加的空槽的初始值都是 None
        self.elements.resize(new_len as usize, None);
        Ok(old_len)
    }

    pub fn get_element(&self, index: usize) -> Result<Option<u32>, EngineError> {
        if index >= self.elements.len() {
            return Err(EngineError::OutOfRange(
                OutOfRange::ElementIndexOutOfRange(index, self.elements.len())
            ));
        }

        Ok(self.elements[index])
    }

    pub fn set_element(&mut self, index: usize, function_index: u32) -> Result<(), EngineError> {
        if index >= self.elements.len() {
            return Err(EngineError::OutOfRange(
                OutOfRange::ElementIndexOutOfRange(index, self.elements.len())
            ));
        }

        self.elements[index] = Some(function_index);
        Ok(())
    }

    pub fn get_table_type(&self) -> &TableType {
        &self.table_type
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        error::{EngineError, OutOfRange, Overflow},
        vm_table::VMTable,
    };

    #[test]
    fn test_increase_size() {
        let mut t0 = VMTable::new_by_min(4);

        assert_eq!(t0.get_size(), 4);

        assert_eq!(t0.increase_size(2).unwrap(), 4);
        assert_eq!(t0.get_size(), 6);

        assert_eq!(t0.increase_size(3).unwrap(), 6);
        assert_eq!(t0.get_size(), 9);

        // 创建一个只有 min page 值的表格
        let mut m1 = VMTable::new_by_min(2);
        assert_eq!(m1.get_size(), 2);

        assert_eq!(m1.increase_size(4).unwrap(), 2);
        assert_eq!(m1.get_size(), 6);

        // 创建一个有 max page 值的表格
        let mut m2 = VMTable::new_by_page_range(2, 4);

        assert_eq!(m2.get_size(), 2);
        assert!(matches!(m2.increase_size(2), Ok(_)));
        assert!(matches!(
            m2.increase_size(1),
            Err(EngineError::Overflow(Overflow::TableSizeExceed(_, _)))
        ));
    }

    #[test]
    fn test_read_write_element() {
        let mut t0 = VMTable::new_by_min(10);

        t0.set_element(0, 10).unwrap();
        t0.set_element(1, 11).unwrap();
        t0.set_element(2, 12).unwrap();
        t0.set_element(3, 13).unwrap();

        assert_eq!(t0.get_element(0).unwrap(), Some(10));
        assert_eq!(t0.get_element(1).unwrap(), Some(11));
        assert_eq!(t0.get_element(2).unwrap(), Some(12));
        assert_eq!(t0.get_element(3).unwrap(), Some(13));

        assert_eq!(t0.get_element(4).unwrap(), None);
        assert_eq!(t0.get_element(9).unwrap(), None);

        assert!(matches!(
            t0.get_element(20),
            Err(EngineError::OutOfRange(OutOfRange::ElementIndexOutOfRange(
                _,
                _
            )))
        ));
    }
}
