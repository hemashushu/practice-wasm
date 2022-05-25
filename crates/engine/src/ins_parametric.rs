// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 操作数（参数 parametric）指令
//!
//! 用于修改操作数栈元素的指令
//!
//! - drop
//! - select

use anvm_ast::types::Value;

use crate::{
    error::{make_invalid_operand_data_type, EngineError},
    vm::VM,
};

/// # drop
///
/// 弹出栈顶的一个操作数并扔掉
pub fn drop(vm: &mut VM) -> Result<(), EngineError> {
    vm.context.stack.pop();
    Ok(())
}

/// # select
///
/// 从栈弹出 3 个操作数，根据栈顶操作数（int32）是否为零，
/// 来决定是压入第 2 个操作数（consequent）或者第 3 个操作数（alternate）
///
/// - 栈顶
/// - testing     <-- 测试值
/// - consequent  <-- 为 0 时压入这个
/// - alternate   <-- 非 0 时压入这个
/// - 栈底
///
/// 示例：
/// (i32.const 11)  <-- alternate
/// (i32.const 10)  <-- consequent
/// (i32.const 0)   <-- testing
/// (select)        <-- 最终压入 10
///
/// 其中：
/// - 栈顶元素（第一个操作数）必须是 int32，
/// - 第二个和第三个操作数的类型必须相同
pub fn select(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let (testing, consequent, alternate) = (stack.pop(), stack.pop(), stack.pop());

    if consequent.get_type() != alternate.get_type() {
        Err(EngineError::InvalidOperation(
            "the operand data type of the consequent and alternate for \"select\" instruction should be the same".to_string(),
        ))
    } else {
        if let Value::I32(value) = testing {
            if value == 0 {
                stack.push(consequent);
            } else {
                stack.push(alternate);
            }
            Ok(())
        } else {
            Err(make_invalid_operand_data_type("select", "i32"))
        }
    }
}
