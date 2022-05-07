// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 等零测试指令
//!
//! 从栈顶弹出一个操作数，判断是否为 0，
//! 如果为 0 则压入 1（int32）， 否则压入 0（int32）
//!
//! i32.eqz
//! i64.eqz

use std::{cell::RefCell, rc::Rc};

use anvm_parser::types::Value;

use crate::{object::EngineError, vm_module::VMModule};

pub fn i32_eqz(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let testing = module.operand_stack.pop();

    if let Value::I32(value) = testing {
        module.operand_stack.push_bool(value == 0);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of the operand for instruction \"i32.eqz\" should be \"i32\"".to_string(),
        ))
    }
}

pub fn i64_eqz(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let testing = module.operand_stack.pop();

    if let Value::I64(value) = testing {
        module.operand_stack.push_bool(value == 0);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of the operand for instruction \"i64.eqz\" should be \"i64\"".to_string(),
        ))
    }
}
