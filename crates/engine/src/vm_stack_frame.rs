// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use anvm_ast::{ast::FunctionType, instruction::Instruction};

use crate::object::{FrameType, StackFrame};

#[derive(Debug, PartialEq, Clone)]
pub struct VMStackFrame {
    /// 创建当前帧的指令
    /// 对于函数调用，创建帧的指令是 call
    /// 对于流程控制所产生的帧，创建的指令有 block/loop 等
    pub frame_type: FrameType,

    /// 函数签名、以及块类型
    pub function_type: Rc<FunctionType>,

    /// 复制了一份当前过程的指令
    pub instructions: Rc<Vec<Instruction>>,

    /// 一个栈帧的在总的操作数栈当中的开始地址，
    /// 对于函数调用帧来说，它是第 0 个实参的地址
    pub frame_pointer: usize,

    /// 程序计数器，
    /// 注意它是一个相对地址，
    /// 即当前指令的地址 **在当前函数的指令序列** 里的索引（位置），
    /// 初始值为 0
    pub program_counter: usize,

    /// 记录当前操作数栈中，（逻辑上）操作数的开始位置，
    /// 以便于快速列出局部变量和操作数这两种数据。
    /// 用于提供调试功能
    operand_pointer: usize,

    /// 记录函数的索引值
    /// 用于提供调试功能
    function_index: Option<usize>,
}

impl StackFrame for VMStackFrame {
    fn get_frame_type(&self) -> FrameType {
        self.frame_type.clone()
    }

    fn get_function_type(&self) -> Rc<FunctionType> {
        Rc::clone(&self.function_type)
    }

    fn get_instructions(&self) -> Rc<Vec<Instruction>> {
        Rc::clone(&self.instructions)
    }

    fn get_frame_pointer(&self) -> usize {
        self.frame_pointer
    }

    fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    fn get_operand_pointer(&self) -> usize {
        self.operand_pointer
    }

    fn get_function_index(&self) -> Option<usize> {
        self.function_index.clone()
    }
}

impl VMStackFrame {
    pub fn new(
        frame_type: FrameType,
        function_type: Rc<FunctionType>,
        function_index: Option<usize>,
        instructions: Rc<Vec<Instruction>>,
        frame_pointer: usize,
        local_variable_count: usize,
    ) -> Self {
        VMStackFrame {
            frame_type,
            function_type,
            function_index,
            instructions,
            frame_pointer,
            operand_pointer: frame_pointer + local_variable_count,
            program_counter: 0, // pc 的初始值为 0
        }
    }
}
