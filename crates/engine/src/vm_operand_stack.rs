// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_parser::types::Value;

/// # 操作数栈（运算栈）
///
/// 当前使用数组来实现操作数栈，
/// 操作数栈除了用于实现数值运算（类似寄存器的任务），同时
/// 也用于实现 `函数调用帧`（call frame）。
pub struct VMOperandStack {
    slots: Vec<Value>,
}

impl VMOperandStack {
    pub fn new() -> Self {
        VMOperandStack { slots: vec![] }
    }

    /// 压入
    pub fn push(&mut self, value: Value) {
        self.slots.push(value);
    }

    /// WebAssembly 对于 bool 值的约定：
    /// 使用 i32 0 表示 false，
    /// 使用 i32 1 表示 true。
    pub fn push_bool(&mut self, value: bool) {
        if value {
            self.slots.push(Value::I32(1));
        } else {
            self.slots.push(Value::I32(0));
        }
    }

    /// 弹出
    ///
    /// 对于 bool 值的约定：
    /// 使用 i32 或者 i64 的 0 表示 false，
    /// 使用 1 表示 true。
    pub fn pop(&mut self) -> Value {
        let option_value = self.slots.pop();
        if let Some(value) = option_value {
            value
        } else {
            panic!("operand stack is empty")
        }
    }

    /// 查看最后一个操作数
    pub fn peek(&self) -> Value {
        let option_value = self.slots.last();
        if let Some(value) = option_value {
            *value
        } else {
            panic!("operand stack is empty")
        }
    }

    /// 获取栈的总大小
    ///
    /// 相当于体系结构当中的 `stack pointer`
    pub fn get_stack_size(&self) -> usize {
        self.slots.len()
    }

    /// 按索引来获取栈的操作数
    ///
    /// 用于读写函数调用的实参以及局部变量
    pub fn get_value(&self, index: usize) -> Value {
        self.slots[index]
    }

    /// 按索引来设置栈的操作数
    ///
    /// 用于读写函数调用的实参以及局部变量
    pub fn set_value(&mut self, index: usize, value: Value) {
        self.slots[index] = value;
    }

    /// 将一组数值原样压入栈
    /// 小索引端的数据先压入，即靠近栈底
    /// 大索引端的数据后压入，即靠近栈顶
    ///
    /// ```diagram
    ///                  |栈顶。|
    /// [0, 1, 2] ---->  | 2   |
    ///                  | 1   |
    ///                  | 0   |
    ///                  |栈底。|
    /// ```
    pub fn push_values(&mut self, values: &[Value]) {
        self.slots.extend_from_slice(values)
    }

    /// 将一组数值原样弹出栈
    /// 靠近栈底的数据会放置在结果的小索引端
    /// 靠近栈顶的数据会放置在结果的大索引端
    ///
    /// ```diagram
    /// |栈顶。|
    /// | 2   | ----> [0, 1, 2]
    /// | 1   |
    /// | 0   |
    /// |栈底。|
    /// ```
    pub fn pop_values(&mut self, count: usize) -> Vec<Value> {
        let index = self.slots.len() - count;
        let values: Vec<Value> = self.slots.drain(index..).collect();
        values
    }
}

#[cfg(test)]
mod tests {
    use anvm_parser::types::Value;

    use super::VMOperandStack;

    #[test]
    fn test_push_pop_and_peek() {
        let mut s0 = VMOperandStack { slots: vec![] };

        // 测试 push
        s0.push(Value::I32(1));
        s0.push(Value::I32(2));
        assert_eq!(s0.get_stack_size(), 2);

        // 测试 pop
        assert_eq!(s0.pop(), Value::I32(2));
        assert_eq!(s0.get_stack_size(), 1);

        // 再次 push
        s0.push(Value::F32(3.0));
        s0.push(Value::F32(4.0));
        assert_eq!(s0.get_stack_size(), 3);

        // 测试 peek 和 pop
        assert_eq!(s0.peek(), Value::F32(4.0));
        assert_eq!(s0.get_stack_size(), 3); // peek 不会改变 slots 的内容
        assert_eq!(s0.pop(), Value::F32(4.0));
        assert_eq!(s0.get_stack_size(), 2);
        assert_eq!(s0.peek(), Value::F32(3.0));
    }

    #[test]
    fn test_get_and_set() {
        let mut s0 = VMOperandStack { slots: vec![] };

        s0.push(Value::I32(1));
        s0.push(Value::I32(2));
        s0.push(Value::I32(3));

        assert_eq!(s0.get_stack_size(), 3);
        assert_eq!(s0.get_value(0), Value::I32(1));
        assert_eq!(s0.get_value(1), Value::I32(2));
        assert_eq!(s0.get_value(2), Value::I32(3));

        s0.set_value(0, Value::I64(11));
        s0.set_value(2, Value::F64(3.3));

        assert_eq!(s0.get_stack_size(), 3);
        assert_eq!(s0.get_value(0), Value::I64(11));
        assert_eq!(s0.get_value(1), Value::I32(2));
        assert_eq!(s0.get_value(2), Value::F64(3.3));

        assert_eq!(s0.pop(), Value::F64(3.3));
        assert_eq!(s0.pop(), Value::I32(2));
        assert_eq!(s0.pop(), Value::I64(11));
        assert_eq!(s0.get_stack_size(), 0);
    }

    #[test]
    fn test_push_and_pop_values() {
        let mut s0 = VMOperandStack { slots: vec![] };

        s0.push(Value::I32(1));
        s0.push(Value::I32(2));
        s0.push(Value::I32(3));
        assert_eq!(s0.get_stack_size(), 3);

        // 测试 push_values
        s0.push_values(&vec![Value::I32(11), Value::I32(22)]);
        assert_eq!(s0.get_stack_size(), 5);
        assert_eq!(s0.get_value(0), Value::I32(1));
        assert_eq!(s0.get_value(1), Value::I32(2));
        assert_eq!(s0.get_value(2), Value::I32(3));
        assert_eq!(s0.get_value(3), Value::I32(11));
        assert_eq!(s0.get_value(4), Value::I32(22));

        // 测试 pop_values
        assert_eq!(
            s0.pop_values(3),
            vec![Value::I32(3), Value::I32(11), Value::I32(22),]
        );

        // 再次测试
        s0.push_values(&vec![Value::F32(1.1)]);
        assert_eq!(
            s0.pop_values(3),
            vec![Value::I32(1), Value::I32(2), Value::F32(1.1),]
        );
    }
}
