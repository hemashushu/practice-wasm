// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use anvm_parser::{ast::FunctionType, instruction::Instruction};

/// # 控制栈
///
/// 因为 WebAssembly 的流程控制块的工作方式跟函数调用比较相近，所以
/// 当前的 `函数调用栈`（call stack）除了用于存储 `函数调用帧`（call frame），同时
/// 也用于存储包含了 `流程控制块的帧`（flow control frame）。
///
/// 当前使用 `控制栈` 和 `操作数栈` 合在一起以实现传统体系结构的 `栈` 和 `寄存器`，
/// 简单来说，`操作数栈` 用于数值运算，以及存储每个调用帧的参数、局部变量以及返回值等数据，
/// `控制栈` 负责记录每个调用帧的基本信息，比如当前帧的开始位置（frame pointer）。
///
/// 当前帧的开始位置同时也是上一个调用帧传递过来的实参当中第 0 个实参的位置，也就是说当前帧
/// 会从上一个帧的栈顶部分区域开始（即实参数据区域），这样可以避免复制实参。
///
/// 但注意的是，返回值的复制有时是无法避免的，因为：
/// 1. 当前函数有局部变量（即运算过程中所需的空槽）
/// 2. 有时当前函数可能因为逻辑错误导致当前操作数栈内有除了返回值之外的数据残留。
///
/// ## `逻辑栈帧` 示意图
///
/// 注意一般体系结构当中栈底一般是内存的最高位，然后栈往下（往低处）增长，
/// 但虚拟机里使用数组来实现栈，所以这里的栈底的索引为 0，栈顶的索引比栈底的大。
///
/// ```diagram
///                         当前栈帧。。--| ------- 栈顶。 -------- | <-- stack pointer
///                                     | 运算槽位。。             |
///                                     | ---------------------- | <-- operand pointer
///                                     | 局部变量占用的槽位        |
///        | ------- 栈顶。-------- |    | ---------------------- | <-\
///        | 传给下一个栈帧的实参。。  | -> | 来自上一个栈帧的实参。。   |   |
///        |                       | -> |                        |   |-- 上一帧的部分区域
///        | ..................... | -> | ------- 栈底。 -------- | <-- frame pointer
///        | 运算槽位。。            |
/// 上一个  | --------------------- |
/// 栈帧。--| 局部变量占用的槽位       |
///        | --------------------- |
///        | 来自上一个栈帧的实参。。  |
///        |                       |
///        | ------- 栈底。 ------- |
/// ```
pub struct ControlStack {
    // 这里的 `stack frame` 除了包括普通的 `call frame`，还包括
    // 函数内的诸如 block/loop/if 等控制块这种 `flow control frame`
    pub frames: Vec<StackFrame>,
}

// name: stackFrame
pub struct StackFrame {
    // 创建当前帧的指令
    // 对于函数调用，创建帧的指令是 call
    // 对于流程控制所产生的帧，创建的指令有 block/loop 等
    pub frame_type: FrameType,

    // 函数签名、以及块类型
    pub function_type: FunctionType,

    // 复制了一份当前过程的指令
    pub instructions: Rc<Vec<Instruction>>,
    // instructions []binary.Instruction

    // base pointer 一个栈帧的开始的开始地址，对于函数调用来说，它是第 0 个实参的地址
    pub frame_pointer: usize,
    //bp int // name: framePointer

    // operand pointer，用于表示（逻辑上）操作数栈的开始位置，
    // 这样便于调试时，快速列出局部变量和操作数这两种数据。
    pub operand_pointer: usize,

    // program counter 程序计数器，即当前指令的地址 **在当前函数的指令序列** 里的索引（位置），
    // 初始值为 0
    pub program_counter: usize,
    //pc int
}

#[derive(Debug, PartialEq, Clone)]
pub enum FrameType {
    Call,
    Block,
    Loop,
}

impl StackFrame {
    pub fn new(
        frame_type: FrameType,
        function_type: FunctionType,
        instructions: Rc<Vec<Instruction>>,
        frame_pointer: usize,
        local_variable_count: usize,
    ) -> Self {
        StackFrame {
            frame_type,
            function_type,
            instructions,
            frame_pointer,
            operand_pointer: frame_pointer + local_variable_count,
            program_counter: 0, // pc 的初始值为 0
        }
    }
}

impl ControlStack {
    pub fn push_frame(&mut self, stack_frame: StackFrame) {
        self.frames.push(stack_frame)
    }

    pub fn pop_frame(&mut self) -> StackFrame {
        let option_frame = self.frames.pop();
        if let Some(frame) = option_frame {
            frame
        } else {
            panic!("control stack is empty")
        }
    }

    pub fn peek_frame(&self) -> &StackFrame {
        let option_frame = self.frames.last();
        if let Some(frame) = option_frame {
            frame
        } else {
            panic!("control stack is empty")
        }
    }

    pub fn get_frame_count(&self) -> usize {
        self.frames.len()
    }

    // 获取最后的一个**调用帧**
    pub fn get_last_call_frame(&self) -> &StackFrame {
        let option_frame = self
            .frames
            .iter()
            .rev()
            .find(|f| f.frame_type == FrameType::Call);

        if let Some(frame) = option_frame {
            frame
        } else {
            panic!("call frame not found")
        }
    }

    /// 返回当前帧距离调用帧（即当前函数）的相对于函数的深度
    ///
    /// 因为 WebAssembly 的控制块（比如 block/loop/if）行为
    /// 跟函数类似，所以当前帧可能是控制块产生的。
    ///
    /// 如果当前帧是：
    /// - 调用帧本身（即在函数层里），则返回 0，
    /// - 在一层控制块里，则返回 1
    /// - 在两层控制块里，则返回 2
    pub fn get_relative_depth(&self) -> usize {
        let mut depth: usize = 0;
        for frame in self.frames.iter().rev() {
            if frame.frame_type == FrameType::Call {
                break;
            } else {
                depth += 1;
            }
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use anvm_parser::ast::FunctionType;

    use super::{ControlStack, FrameType, StackFrame};

    fn new_void_function_type() -> FunctionType {
        FunctionType {
            params: vec![],
            results: vec![],
        }
    }

    fn new_call_frame() -> StackFrame {
        StackFrame::new(
            FrameType::Call,
            new_void_function_type(),
            Rc::new(vec![]),
            0,
            0,
        )
    }

    fn new_block_frame() -> StackFrame {
        StackFrame::new(
            FrameType::Block,
            new_void_function_type(),
            Rc::new(vec![]),
            0,
            0,
        )
    }

    fn new_loop_frame() -> StackFrame {
        StackFrame::new(
            FrameType::Loop,
            new_void_function_type(),
            Rc::new(vec![]),
            0,
            0,
        )
    }

    #[test]
    fn test_push_pop_and_peek() {
        let mut s0 = ControlStack { frames: vec![] };

        // 测试 push
        s0.push_frame(new_call_frame());
        s0.push_frame(new_loop_frame());
        s0.push_frame(new_block_frame());

        // 测试 pop
        assert_eq!(s0.get_frame_count(), 3);
        assert_eq!(s0.pop_frame().frame_type, FrameType::Block);
        assert_eq!(s0.pop_frame().frame_type, FrameType::Loop);
        assert_eq!(s0.get_frame_count(), 1);

        // 测试 peek
        assert_eq!(s0.peek_frame().frame_type, FrameType::Call);
        assert_eq!(s0.get_frame_count(), 1);
    }
}
