// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use crate::{
    object::{ControlStack, FrameType, StackFrame},
    vm_stack_frame::VMStackFrame,
};

/// # 控制栈
///
/// 因为 WebAssembly 的流程控制结构块的工作方式跟函数调用比较相近，所以
/// 当前的 `函数调用栈`（call stack）除了用于存储 `函数调用帧`（call frame），同时
/// 也用于存储包含了 `流程控制结构块的帧`（flow control frame）。
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
pub struct VMControlStack {
    // 这里的 `stack frame` 除了包括普通的 `call frame`，还包括
    // 函数内的诸如 block/loop/if 等流程控制结构块这种 `flow control frame`
    frames: Vec<VMStackFrame>,
}

impl ControlStack for VMControlStack {
    fn get_frames(&self) -> Vec<Rc<dyn StackFrame>> {
        let mut stack_frames: Vec<Rc<dyn StackFrame>> = vec![];
        for frame in &self.frames {
            stack_frames.push(Rc::new(frame.clone()));
        }
        stack_frames
    }

    fn get_frame_count(&self) -> usize {
        self.frames.len()
    }

    fn get_last_frame(&self) -> Rc<dyn StackFrame> {
        if let Some(frame) = self.frames.last() {
            let stack_frame = frame.clone();
            Rc::new(stack_frame)
        } else {
            panic!("control stack is empty")
        }
    }

    fn get_last_call_all_frames(&self) -> Vec<Rc<dyn StackFrame>> {
        let option_frame = self
            .frames
            .iter()
            .enumerate()
            .rev()
            .find(|(_, frame)| frame.frame_type == FrameType::Call);

        if let Some((start, _)) = option_frame {
            let mut stack_frames: Vec<Rc<dyn StackFrame>> = vec![];
            for index in start..self.frames.len() {
                stack_frames.push(Rc::new(self.frames[index].clone()));
            }
            stack_frames
        } else {
            panic!("call frame not found")
        }
    }
}

impl VMControlStack {
    pub fn new() -> Self {
        VMControlStack { frames: vec![] }
    }

    pub fn push_frame(&mut self, stack_frame: VMStackFrame) {
        self.frames.push(stack_frame)
    }

    pub fn pop_frame(&mut self) -> VMStackFrame {
        let option_frame = self.frames.pop();
        if let Some(frame) = option_frame {
            frame
        } else {
            panic!("control stack is empty")
        }
    }

    pub fn peek_frame(&self) -> &VMStackFrame {
        if let Some(frame) = self.frames.last() {
            frame
        } else {
            panic!("control stack is empty")
        }
    }

    pub fn reset_program_counter(&mut self) {
        if self.frames.len() == 0 {
            panic!("control stack is empty")
        }

        let last_index = self.frames.len() - 1;
        self.frames[last_index].program_counter = 0;
    }

    pub fn increase_program_counter(&mut self) {
        if self.frames.len() == 0 {
            panic!("control stack is empty")
        }

        let last_index = self.frames.len() - 1;
        let mut last_frame = &mut self.frames[last_index];

        last_frame.program_counter = last_frame.program_counter + 1;
    }

    // 获取最后的一个**调用帧**
    pub fn get_last_call_frame(&self) -> &VMStackFrame {
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
    /// 因为 WebAssembly 的流程控制结构块（比如 block/loop/if）行为
    /// 跟函数类似，所以当前帧可能是流程控制结构块产生的。
    ///
    /// 如果当前帧是：
    /// - 调用帧本身（即在函数层里），则返回 0，
    /// - 在一层流程控制结构块里，则返回 1
    /// - 在两层流程控制结构块里，则返回 2
    pub fn get_relative_depth(&self) -> u32 {
        let mut depth: u32 = 0;
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

    use anvm_ast::ast::FunctionType;

    use super::{ControlStack, FrameType, VMControlStack, VMStackFrame};

    fn new_void_function_type() -> Rc<FunctionType> {
        Rc::new(FunctionType {
            params: vec![],
            results: vec![],
        })
    }

    fn new_call_frame() -> VMStackFrame {
        VMStackFrame::new(
            FrameType::Call,
            new_void_function_type(),
            Some(0),
            Rc::new(vec![]),
            0,
            0,
        )
    }

    fn new_block_frame() -> VMStackFrame {
        VMStackFrame::new(
            FrameType::Block,
            new_void_function_type(),
            None,
            Rc::new(vec![]),
            0,
            0,
        )
    }

    fn new_loop_frame() -> VMStackFrame {
        VMStackFrame::new(
            FrameType::Loop,
            new_void_function_type(),
            None,
            Rc::new(vec![]),
            0,
            0,
        )
    }

    #[test]
    fn test_push_pop_and_peek() {
        let mut s0 = VMControlStack { frames: vec![] };

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
