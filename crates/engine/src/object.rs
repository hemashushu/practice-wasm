// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::instruction::{self, BlockType};

/// 函数信息
#[derive(Debug, PartialEq, Clone)]
pub enum FunctionItem {
    Native {
        type_index: usize,
        native_module_index: usize,
        function_index: usize,
    },
    External {
        type_index: usize,
        ast_module_index: usize,
        function_index: usize,
        internal_function_index: usize,
        start_index: usize,
        end_index: usize, // 函数 `end 指令` 所在的位置
    },
    Internal {
        type_index: usize,
        internal_function_index: usize,
        start_index: usize,
        end_index: usize, // 函数 `end 指令` 所在的位置
    },
}

/// 控制指令
#[derive(Debug, PartialEq, Clone)]
pub enum Control {
    /// 进入一个新的栈帧
    /// 对应 block/loop 指令
    Block(BlockType),

    /// 进入一个新的栈帧，并当原栈顶的数值等于 0 时，跳转到指定的地址
    /// 对应 if 指令
    BlockJumpEqZero(BlockType, /* alternate_addr */ usize),

    /// 跳转到指定的地址
    /// 其中 relative_depth 为当前栈帧距离目标栈帧的层次数量，当数量为 0 时，表示
    /// 跳转到当前栈帧层的其他地址，当数量 >0 时，表示需要弹出相应的栈帧数量。
    /// 对应 else/br/return 指令
    /// 显然对于 else 指令，relative_depth 的值为 0.
    Jump(/* relative_depth */ usize, /* addr */ usize),

    /// 跳转到指定的地址
    /// 跟 Jump 指令类似，但仅当原栈顶的数值不等于 0 时才跳转，否则什么事都不做
    /// 对应 br_if 指令
    JumpNotEqZero(/* relative_depth */ usize, /* addr */ usize),

    /// 调用模块内的函数
    CallInternal(
        /* type_index */ usize,
        /* function_index */ usize,
        /* internal_function_index */ usize,
        /* addr */ usize,
    ),

    /// 调用模块外的函数
    CallExternal(
        /* ast_module_index */ usize,
        /* type_index */ usize,
        /* function_index */ usize,
        /* internal_function_index */ usize,
        /* addr */ usize,
    ),

    /// 调用本地函数（native function）模块的本地函数
    CallNative(
        /* native_module_index */ usize,
        /* type_index */ usize,
        /* function_index */ usize,
    ),
}

/// 编译后的指令
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Original(instruction::Instruction),
    Control(Control),
}
