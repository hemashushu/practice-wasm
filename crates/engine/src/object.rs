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
        start_index: usize,
        end_index: usize, // 函数 `end 指令` 所在的位置
    },
    Internal {
        type_index: usize,
        start_index: usize,
        end_index: usize, // 函数 `end 指令` 所在的位置
    },
}

/// 控制指令
#[derive(Debug, PartialEq, Clone)]
pub enum Control {
    Block(BlockType),
    BlockJumpEqZero(BlockType, /* alternate_addr */ usize),
    Jump(/* addr */ usize),
    JumpOut(/* relative_depth */ usize, /* addr */ usize),
    JumpOutNotEqZero(/* relative_depth */ usize, /* addr */ usize),
    CallInternal(
        /* type_index */ usize,
        /* function_index */ usize,
        /* addr */ usize,
    ),
    CallExternal(
        /* module_index */ usize,
        /* type_index */ usize,
        /* function_index */ usize,
        /* addr */ usize,
    ),
    CallNative(
        /* module_index */ usize,
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
