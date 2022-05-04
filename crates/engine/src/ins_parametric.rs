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

use std::{cell::RefCell, rc::Rc};

use anvm_parser::types::Value;

use crate::{instance::EngineError, vm_module::VMModule};

/// # drop
///
/// 弹出栈顶的一个操作数并扔掉
pub fn drop(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    vm_module.as_ref().borrow_mut().operand_stack.pop();
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
/// - alternate   <-- 为 1 时压入这个
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
pub fn select(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let (testing, consequent, alternate) = (
        module.operand_stack.pop(),
        module.operand_stack.pop(),
        module.operand_stack.pop(),
    );

    if consequent.get_type() != alternate.get_type() {
        Err(EngineError::InvalidOperation(
            "the value type of the consequent and alternate for \"select\" instruction should be the same".to_string(),
        ))
    } else {
        if let Value::I32(value) = testing {
            if value == 0 {
                module.operand_stack.push(consequent);
            } else {
                module.operand_stack.push(alternate);
            }
            Ok(())
        } else {
            Err(EngineError::InvalidOperation(
                "the testing number for \"select\" instruction should be type \"i32\"".to_string(),
            ))
        }
    }
}
