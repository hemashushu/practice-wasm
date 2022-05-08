// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{any::Any, cell::RefCell, rc::Rc, fmt::Debug};

use anvm_ast::{
    ast::{FunctionType, GlobalType, MemoryType, TableType},
    instruction::Instruction,
    types::Value,
};

/// `模块` 对象的接口
///
/// 在运行 WebAssembly 应用程序时，一个 WebAssembly 二进制（或文本）文件即一个模块，
/// 对应一个 `模块`（`Module`）对象实例。
/// 模块之间的链接和调用的实现基于 Module/Function/Table/Memory/Global 等接口，
/// 注意本地函数（native function）也将会存储在一个拥有 Module 接口的模块当中。
pub trait Module {
    fn get_name(&self) -> &str;

    fn get_export_table(&self, name: &str) -> Result<Rc<RefCell<dyn Table>>, EngineError>;
    fn get_export_memory(&self, name: &str) -> Result<Rc<RefCell<dyn Memory>>, EngineError>;
    fn get_export_function(&self, name: &str) -> Result<Rc<dyn Function>, EngineError>;
    fn get_export_global_variable(
        &self,
        name: &str,
    ) -> Result<Rc<RefCell<dyn GlobalVariable>>, EngineError>;

    fn get_exports(&self) -> Vec<Export>;

    // 下列是用于提供 debug 功能的函数

    /// 获取内存快照
    fn dump_memory(&self, address: usize, length: usize) -> Vec<u8>;

    // 获取调用栈快照
    // fn dump_stacks(&self, count: usize) -> Vec<StackFrame>;

    fn get_function_by_index(&self, index: usize) -> Rc<dyn Function>;
}

pub enum Export {
    Table(Rc<RefCell<dyn Table>>),
    Memory(Rc<RefCell<dyn Memory>>),
    Function(Rc<dyn Function>),
    GlobalVariable(Rc<RefCell<dyn GlobalVariable>>),
}

pub trait Function {
    /// 从 vm 外部（即宿主）或者其他模块调用函数
    fn eval(&self, args: &[Value]) -> Result<Vec<Value>, EngineError>;
    fn get_function_type(&self) -> Rc<FunctionType>;

    /// 获取函数在模块中的索引值，
    /// 用于提供调试功能
    fn get_index(&self) -> usize;
    fn get_name(&self) -> Option<String>;
    fn as_any(&self) -> &dyn Any;
}

pub trait Table {
    fn get_size(&self) -> u32;
    fn increase_size(&mut self, increase_number: u32) -> Result<u32, EngineError>;

    fn get_element(&self, index: usize) -> Result<Option<Rc<dyn Function>>, EngineError>;
    fn set_element(&mut self, index: usize, func: Rc<dyn Function>) -> Result<(), EngineError>;
    fn get_table_type(&self) -> TableType;
}

pub trait Memory {
    fn get_page_count(&self) -> u32;

    /// 扩充内存的页面数
    ///
    /// 参数 increase_number 是 需要增加的页面的数量，而不是 "增加到" 的页面的数量。
    /// 函数的返回值：
    /// - 成功时返回旧的页面数（u32）
    /// - 失败时返回 Result::Err（注意，WebAssembly 的 memory.grow 指令约定
    ///   失败时要把被转为 u32 的 -1 压入操作数栈）
    fn incrase_page(&mut self, increase_number: u32) -> Result<u32, EngineError>;

    /// 读取指定地址的数据
    ///
    /// 注意，因为指令中的 offset 立即数是 u32，而操作数栈弹出的值也是 u32，
    /// 所以有效地址（u32 + u32）是一个 33 位的无符号整数。
    fn read_bytes(&self, address: usize, length: usize) -> &[u8]; // Vec<u8>;
    fn write_bytes(&mut self, address: usize, data: &[u8]);

    fn read_i8(&self, address: usize) -> i8;
    fn read_i16(&self, address: usize) -> i16;
    fn read_i32(&self, address: usize) -> i32;
    fn read_i64(&self, address: usize) -> i64;
    fn read_f32(&self, address: usize) -> f32;
    fn read_f64(&self, address: usize) -> f64;

    fn write_i8(&mut self, address: usize, value: i8);
    fn write_i16(&mut self, address: usize, value: i16);
    fn write_i32(&mut self, address: usize, value: i32);
    fn write_i64(&mut self, address: usize, value: i64);
    fn write_f32(&mut self, address: usize, value: f32);
    fn write_f64(&mut self, address: usize, value: f64);

    fn get_memory_type(&self) -> MemoryType;
}

pub trait GlobalVariable {
    fn get_value(&self) -> Value;
    fn set_value(&mut self, value: Value) -> Result<(), EngineError>;
    fn get_global_type(&self) -> GlobalType;
}

#[derive(Debug)]
pub enum EngineError {
    OutOfIndex(String),
    Overflow(String),
    ObjectNotFound(String),
    InvalidOperation(String),
    ModuleError(Box<dyn ModuleError>)
}

pub trait ModuleError:Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait OperandStack {
    fn get_operands(&self) -> Vec<Value>;
    fn get_operand_count(&self) -> usize;
}

pub trait StackFrame {
    fn get_frame_type(&self) -> FrameType;
    fn get_function_type(&self) -> Rc<FunctionType>;
    fn get_instructions(&self) -> Rc<Vec<Instruction>>;
    fn get_frame_pointer(&self) -> usize;
    fn get_program_counter(&self) -> usize;
    fn get_operand_pointer(&self) -> usize;
    fn get_function_index(&self) -> Option<usize>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum FrameType {
    Call,
    Block,
    Loop,
    If,
}

pub trait ControlStack {
    fn get_frames(&self) -> Vec<Rc<dyn StackFrame>>;
    fn get_frame_count(&self) -> usize;
    fn get_last_frame(&self) -> Rc<dyn StackFrame>;
    fn get_last_call_all_frames(&self) -> Vec<Rc<dyn StackFrame>>;
}
