// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::ast;
use anvm_ast::instruction::{self, BlockType};

#[derive(Debug, PartialEq, Clone)]
pub struct NamedAstModule {
    pub name: String,
    pub module: ast::Module,
}

impl NamedAstModule {
    pub fn new(name: &str, module: ast::Module) -> Self {
        Self {
            name: name.to_string(),
            module,
        }
    }
}

/// 一个模块里的所有类型函数的信息
///
/// 函数包括导入的函数（分为本地函数和外部模块普通函数）以及
/// 模块内部定义的普通函数。
#[derive(Debug, PartialEq, Clone)]
pub enum FunctionItem {
    Native {
        native_module_index: usize,
        type_index: usize, // 目标函数在目标模块当中的类型索引
        function_index: usize,
    },
    External {
        ast_module_index: usize,
        type_index: usize, // 目标函数在目标模块当中的类型索引
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
    /// 进入一个新的栈帧
    /// 对应 block/loop 指令
    Block(BlockType, /* end_addr */ usize),

    /// 进入一个新的栈帧，并当原栈顶的数值等于 0 时，跳转到指定的地址
    /// 对应 if 指令
    BlockJumpEqZero(
        BlockType,
        /* alternate_addr */ usize,
        /* end_addr */ usize,
    ),

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
        /* type_index */ usize, // 冗余信息，为了省去一次查询过程
        /* function_index */ usize,
        /* addr */ usize, // 冗余信息，为了省去一次查询过程
    ),

    /// 调用模块外的函数
    CallExternal(
        /* ast_module_index */ usize,
        /* type_index 在原模块当中的类型索引 */
        usize, // 冗余信息，为了省去一次查询过程
        /* function_index 在原模块当中的函数索引（索引包括导入的外部函数，也包括模块内部函数，此索引值为 call 指令参数所指定的值）*/
        usize,
        /* addr */
        usize, // 冗余信息，为了省去一次查询过程
    ),

    /// 调用本地函数（native function）模块的本地函数
    CallNative(
        /* native_module_index */ usize,
        /* type_index 在原模块当中的类型索引 */
        usize, // 冗余信息，为了省去一次查询过程
        /* function_index 在原模块当中的函数索引 */ usize,
    ),
}

/// 编译后的指令
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Original(instruction::Instruction),
    Control(Control),
}
