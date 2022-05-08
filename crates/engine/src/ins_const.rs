// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 常量指令
//!
//! 将指令当中的立即数压入栈
//!
//! - i32.const
//! - i64.const
//! - f32.const
//! - f64.const

use std::{cell::RefCell, rc::Rc};

use anvm_ast::types::Value;

use crate::{object::EngineError, vm_module::VMModule};

pub fn i32_const(vm_module: Rc<RefCell<VMModule>>, value: i32) -> Result<(), EngineError> {
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push(Value::I32(value));
    Ok(())
}

pub fn i64_const(vm_module: Rc<RefCell<VMModule>>, value: i64) -> Result<(), EngineError> {
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push(Value::I64(value));
    Ok(())
}

pub fn f32_const(vm_module: Rc<RefCell<VMModule>>, value: f32) -> Result<(), EngineError> {
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push(Value::F32(value));
    Ok(())
}

pub fn f64_const(vm_module: Rc<RefCell<VMModule>>, value: f64) -> Result<(), EngineError> {
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push(Value::F64(value));
    Ok(())
}
