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
//
// 注意，`表` 可以被导出导入，一张表的内容，即函数引用有可能来自多个不同的模块。

use std::rc::Rc;

use anvm_ast::ast::{Limit, TableType};

use crate::object::{EngineError, Function, Table};

pub struct VMTable {
    /// TableType 的信息包含表的类型（目前只有函数引用类型）以及限制值（范围值）
    table_type: TableType,

    elements: Vec<Option<Rc<dyn Function>>>,
}

impl VMTable {
    pub fn new(table_type: TableType) -> Self {
        let min = table_type.limit.get_min();
        VMTable {
            table_type: table_type,
            elements: vec![None; min as usize], //Vec::<Option<Rc<dyn Function>>>::with_capacity(min as usize),
        }
    }
}

impl Table for VMTable {
    fn get_size(&self) -> u32 {
        self.elements.len() as u32
    }

    fn increase_size(&mut self, increase_number: u32) -> Result<u32, EngineError> {
        let new_len = self.get_size() + increase_number;

        if let Limit::Range(_, max) = self.table_type.limit {
            if new_len > max {
                return Err(EngineError::Overflow(
                    "exceeded the maximum size of the table".to_string(),
                ));
            }
        }

        self.elements.resize(new_len as usize, None);
        Ok(new_len)
    }

    fn get_element(&self, index: usize) -> Result<Option<Rc<dyn Function>>, EngineError> {
        if index >= self.elements.len() {
            return Err(EngineError::OutOfIndex(
                "element index out of the range of the table".to_string(),
            ));
        }

        match &self.elements[index] {
            Some(r) => Ok(Some(Rc::clone(r))),
            None => Ok(None),
        }
    }

    fn set_element(&mut self, index: usize, func: Rc<dyn Function>) -> Result<(), EngineError> {
        if index >= self.elements.len() {
            return Err(EngineError::OutOfIndex(
                "element index out of the range of the table".to_string(),
            ));
        }

        self.elements[index] = Some(func);
        Ok(())
    }

    fn get_table_type(&self) -> TableType {
        self.table_type.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_increase_size() {
        // todo
    }
}
