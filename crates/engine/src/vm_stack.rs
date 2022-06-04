// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # VMStack
//!
//! 栈式的 VM 在逻辑上存在：
//!
//! 1. 记录了字节码以及程序计数器（pc）等状态信息的结构；
//! 2. 栈，用于记录函数调用路径；
//! 3. 运算栈，用于操作数运算，相当于体系结构里的寄存器；
//! 4. 局部变量表；
//!
//! 由于后三者具有较高的相关性，所以当前的 VM 实现将后 3 者合并在一起，
//! 由一个 VMStack 来实现，具体来说：
//!
//! VMStack 由 `栈帧`（stack frame） 组成，一个 `栈帧` 由以下 3 个段组成：
//!
//! 1. 局部变量段
//! 2. 之前一帧（previous）的信息段，用于函数调用的返回
//! 3. 运算操作数段（其中包括要传给下一个函数的实参）
//!
//! 下面是一个函数调用所创建的 `栈帧` 示意图：
//!
//! ```diagram
//!
//!                              | ------------------ | <-- sp (stack pointer)
//!                              |                    |
//!                              | 运算操作数段         |
//!                              |                    |
//!                              | .................. |
//!                              |                    |
//!                              | 旧帧的 fp, bp 等信息 | <-- 栈帧信息段
//!                              |                    |
//!                              | ---- index N ----- | <-- 新 bp
//!                              | 第 N 个局部变量空槽。 |
//!                              | 第 0 个局部变量空槽。 | -|
//! | ------- 栈顶。------- |     | .................. |  |- 局部变量段
//! | 第 N 个参数值。        | --> | 第 N 个参数值。      | -|
//! | 第 0 个参数值。        | --> | 第 0 个参数值。      |
//! | ....... 原栈顶 ...... |     | ---- index 0 ----- | <-- 新 fp, lp
//! |                      |
//! | 运算操作数段           |
//! | -------------------- |
//! |                      |
//! | 栈帧信息段。           |
//! |                      |
//! | -------------------- | <-- 旧 bp
//! |                      |
//! | 局部变量段。           |
//! | .................... |
//! | 来自上一个栈帧的实参。。 |
//! |                      |
//! | ------- 栈底。 ------ | <-- 旧 fp, lp
//!
//! ```
//!
//! 注，
//! - 从上图可见，新栈帧并不总是从栈顶开始创建的，当有实参传递时，新栈帧是从第 0 个
//!   实参的位置开始创建。
//! - 一般体系结构当中栈底一般是内存的最高位，然后栈往下（往低处）增长，
//!   但虚拟机里使用数组来实现栈，所以这里的栈底的索引为 0，栈顶的索引比栈底的大。
//!
//! ## 控制帧
//!
//! 栈帧因函数调用而产生，所以栈帧信息段记录的自然是函数调用的信息，所以排除局部变量段
//! 以及运算操作数段之后，栈也可以称为 `函数调用栈`（`call stack`），栈帧也可以称为
//! `函数调用帧`（`call frame`），但因为 WebAssembly 的流程控制结构块的工作方式
//! 跟函数调用非常相近，所以当前的 VM 也会因为进入一个流程控制块而创建一个新的栈帧，当离开
//! 控制块时，也会弹出一个栈帧，因此当前的栈帧其实也包含了 `流程控制结构块的帧`（`flow control frame`），
//! 下面简称为 `控制帧`（`control frame`）。
//!
//! `控制帧` 跟 `调用帧` 几乎是一样的，除了：
//!
//! 1. 控制帧没有局部变量段，控制帧共享最近一次函数调用所创建的帧的局部变量；
//! 2. 返回地址是结构块的最后一条指令（即 `end` 指令）所处的位置，而并非调用指令的下一条指令的位置。
//!
//! 下面是进入一个流程控制结构块所创建的 `栈帧` 示意图：
//!
//! ```diagram
//!
//!                              | ------------------ | <-- sp (stack pointer)
//!                              |                    |
//!                              | 运算操作数段         |
//!                              |                    |
//!                              | .................. |
//!                              |                    |
//!                              | 旧帧的 fp, bp 等信息 | <-- 栈帧信息段
//!                              |                    |
//! | ------- 栈顶。------- |     | .................. | <-- 新 bp
//! | 第 N 个参数值。        | --> | 第 N 个参数值。      |
//! | 第 0 个参数值。        | --> | 第 0 个参数值。      |
//! | ....... 原栈顶 ...... |     | ---- index 0 ----- | <-- 新 fp
//! |                      |
//! | 运算操作数段           |
//! | -------------------- |
//! |                      |
//! | 栈帧信息段。           |
//! |                      |
//! | -------------------- | <-- 旧 bp
//! |                      |
//! | 局部变量段。           |
//! | .................... |
//! | 来自上一个栈帧的实参。。 |
//! |                      |
//! | ------- 栈底。 ------ | <-- 旧 fp, 新旧 lp 均在此
//!
//! ## 信息段的数据
//!
//! 信息段里存放着上一个栈帧及程序的状态信息，具体有：
//!
//! 0. previous_frame_pointer
//! 1. previous_local_pointer
//! 2. previous_base_pointer
//! 3. return_vm_module_index
//! 4. return_function_index
//! 5. frame_type_class
//! 6. frame_type_value
//! 7. return_address
//!
//! 其中第 5 和第 6 项合并起来表示上一帧的函数或者结构块的类型（即参数和返回值的情况）

use anvm_ast::types::Value;

use crate::error::EngineError;

/// 信息段的项目（数值）的数量
pub const INFO_SEGMENT_ITEM_COUNT: usize = 8;

/// # VMStack 同时肩负作为运算栈（操作数栈）和记录栈帧信息的任务
///
/// 也就是说，除了用于记录栈帧信息，同时供给运算指令（诸如 i32.add 等）
/// 作为运算栈，实际上这两个栈可以分开来实现，之所以将它们合并，是为了
/// 简化程序（它们俩有很多相似的方法）。
///
/// 当前使用偷懒的方法 -- 数组来实现操作数栈，让 Rust 底层库
/// 自动管理栈的分配和容量。
pub struct VMStack {
    slots: Vec<Value>,
}

impl VMStack {
    pub fn new() -> Self {
        VMStack { slots: vec![] }
    }

    /// 获取栈的总大小
    ///
    /// 相当于体系结构当中的 `stack pointer`
    pub fn get_size(&self) -> usize {
        self.slots.len()
    }

    /// 压入
    pub fn push(&mut self, value: Value) {
        self.slots.push(value);
    }

    /// 弹出
    pub fn pop(&mut self) -> Value {
        self.slots.pop().expect("operand stack is empty")
    }

    /// 查看最后一个操作数
    pub fn peek(&self) -> Value {
        *self.slots.last().expect("operand stack is empty")
    }

    /// 对于压入 bool 值的约定：
    /// 使用 i32 0 表示 false，
    /// 使用 i32 1 表示 true。
    pub fn push_bool(&mut self, value: bool) {
        if value {
            self.slots.push(Value::I32(1));
        } else {
            self.slots.push(Value::I32(0));
        }
    }

    /// 对于弹出 bool 值的约定：
    /// 如果数值为 i32 0，则表示 false，
    /// 如果数值为 i32 非零，则表示 true。
    pub fn pop_bool(&mut self) -> Result<bool, EngineError> {
        match self.pop() {
            Value::I32(0) => Ok(false),
            Value::I64(_) | Value::F32(_) | Value::F64(_) => Err(EngineError::InvalidOperation(
                "expected i32 for bool value".to_string(),
            )),
            _ => Ok(true),
        }
    }

    /// 按索引来获取栈的操作数
    ///
    /// 用于读写局部变量（局部变量包括函数调用的实参）
    /// 以及栈信息
    pub fn get_value(&self, index: usize) -> Value {
        self.slots[index]
    }

    /// 按索引来设置栈的操作数
    ///
    /// 用于读写局部变量（局部变量包括函数调用的实参）
    /// 以及栈信息
    pub fn set_value(&mut self, index: usize, value: Value) {
        self.slots[index] = value;
    }

    /// 将一组数值原样压入栈
    /// 此方法用于批量读写函数调用的实参以及返回值
    ///
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
    /// 此方法用于批量读写函数调用的实参以及返回值
    ///
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

    pub fn drop_values_at(&mut self, index: usize) {
        self.slots.drain(index..);
    }

    pub fn read_slots(&self, start: usize, end: usize) -> &[Value] {
        &self.slots[start..end]
    }

    pub fn peek_values(&self, count: usize) -> &[Value] {
        let index = self.slots.len() - count;
        &self.slots[index..]
    }
}

#[cfg(test)]
mod tests {
    use anvm_ast::types::Value;

    use crate::error::EngineError;

    use super::VMStack;

    #[test]
    fn test_push_pop_and_peek() {
        let mut s0 = VMStack::new();

        // 测试 push
        s0.push(Value::I32(1));
        s0.push(Value::I32(2));
        assert_eq!(s0.get_size(), 2);

        // 测试 pop
        assert_eq!(s0.pop(), Value::I32(2));
        assert_eq!(s0.get_size(), 1);

        // 再次 push
        s0.push(Value::F32(3.0));
        s0.push(Value::F32(4.0));
        assert_eq!(s0.get_size(), 3);

        // 测试 peek 和 pop
        assert_eq!(s0.peek(), Value::F32(4.0));
        assert_eq!(s0.get_size(), 3); // peek 不会改变 slots 的内容
        assert_eq!(s0.pop(), Value::F32(4.0));
        assert_eq!(s0.get_size(), 2);
        assert_eq!(s0.peek(), Value::F32(3.0));
    }

    #[test]
    fn test_push_pop_bool() {
        let mut s0 = VMStack::new();

        // 测试 push
        s0.push(Value::F32(3.14));
        s0.push(Value::F32(0.0));
        s0.push_bool(true);
        s0.push_bool(false);
        s0.push_bool(true);

        assert_eq!(s0.pop_bool().unwrap(), true);
        assert_eq!(s0.pop_bool().unwrap(), false);
        assert_eq!(s0.pop_bool().unwrap(), true);

        assert!(matches!(
            s0.pop_bool(),
            Err(EngineError::InvalidOperation(_))
        ));

        assert!(matches!(
            s0.pop_bool(),
            Err(EngineError::InvalidOperation(_))
        ));
    }

    #[test]
    fn test_get_and_set() {
        let mut s0 = VMStack::new();

        s0.push(Value::I32(1));
        s0.push(Value::I32(2));
        s0.push(Value::I32(3));

        assert_eq!(s0.get_size(), 3);
        assert_eq!(s0.get_value(0), Value::I32(1));
        assert_eq!(s0.get_value(1), Value::I32(2));
        assert_eq!(s0.get_value(2), Value::I32(3));

        s0.set_value(0, Value::I64(11));
        s0.set_value(2, Value::F64(3.3));

        assert_eq!(s0.get_size(), 3);
        assert_eq!(s0.get_value(0), Value::I64(11));
        assert_eq!(s0.get_value(1), Value::I32(2));
        assert_eq!(s0.get_value(2), Value::F64(3.3));

        assert_eq!(s0.pop(), Value::F64(3.3));
        assert_eq!(s0.pop(), Value::I32(2));
        assert_eq!(s0.pop(), Value::I64(11));
        assert_eq!(s0.get_size(), 0);
    }

    #[test]
    fn test_push_and_pop_values() {
        let mut s0 = VMStack::new();

        s0.push(Value::I32(1));
        s0.push(Value::I32(2));
        s0.push(Value::I32(3));
        assert_eq!(s0.get_size(), 3);

        // 测试 push_values
        s0.push_values(&vec![Value::I32(11), Value::I32(22)]);
        assert_eq!(s0.get_size(), 5);
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
