// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 流程控制指令
//!
//! ## br 指令
//!
//! br 指令后面接着 `跳转目标` 的相对深度。
//! 对于 block/if 指令来说，跳转目标是指令的结尾处（即 end 指令），
//! 对于 loop 指令来说，跳转目标是指令的开始处（即 loop 指令）。
//!
//! (func
//!     (block
//!         (i32.const 100)
//!         (br 0)               ;; 跳转目标为 dest_a
//!         (i32.const 101)
//!     )                        ;; 此处是 dest_a
//!     (loop                    ;; 此处是 dest_b
//!         (i32.const 200)
//!         (br 0)               ;; 跳转目标为 dest_b
//!         (i32.const 201)
//!     )
//!     (if (i32.eqz (i32.const 300))
//!         (then (i32.const 400) (br 0) (i32.const 401))    ;; 跳转目标为 dest_b
//!         (else (i32.const 500) (br 0) (i32.const 501))    ;; 跳转目标为 dest_b
//!     )                                                    ;; 此处是 dest_c
//! )
//!
//! relative_depth 从 0 开始，函数本身一层，然后每个嵌套的 block/loop/if 一层，当 relative_depth 为 N 时，
//! 实际上一共有 N+1 层（包括函数本身一层）可以跳。
//!
//! ## br_if 指令
//!
//! br_if 指令先从操作数栈顶弹出一个有符号的整数（int32），非 0 则执行 br 操作，
//! 等于 0 则什么都不做（仅仅消耗掉栈顶的一个操作数）
//!
//! ## if 指令
//!
//! if 指令先从操作数栈顶弹出一个有符号的整数（int32），
//! 非 0 则执行 consequet_body 的指令
//! 等于 0 则执行 alternate_body 的指令
//!
//! ## br_table 指令
//!
//! br_table 指令先从操作数栈顶弹出一个 uint32 整数，这个数将作为
//! br_table 后面的整数列表的索引，获取跳转的目标。如果该索引超出了
//! 列表范围，则跳转目标的 br_table 指令的最末尾一个参数（即默认目标）

use anvm_ast::{
    instruction::BlockType,
    types::{check_types, check_value_types, Value, ValueType, ValueTypeCheckError},
};

use crate::{
    error::{
        make_invalid_operand_data_type_engine_error,
        make_invalid_operand_data_types_2_engine_error,
        make_invalid_operand_data_types_engine_error, make_invalid_table_index_engine_error,
        make_mismatch_function_type_engine_error, EngineError,
    },
    object::FunctionItem,
    vm::VM,
    vm_stack::INFO_SEGMENT_ITEM_COUNT,
};
