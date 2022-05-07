// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 变量指令
//!
//! 读写局部变量
//!
//! - local.get local_idx:uint32    ;; 读取指定索引的局部变量的值，压入操作数栈
//! - local.set local_idx:uint32    ;; 从操作数栈弹出一个数，写入到指定索引的局部变量；弹出的数的类型必须跟局部变量的一致
//! - local.tee local_idx:uint32    ;; 读取栈顶的值，写入到指定索引的局部变量
//!
//! 读写全局变量
//!
//! - global.get global_idx:uint32  ;; 读取指定索引的全局变量的值，压入操作数栈
//! - global.set global_idx:uint32  ;; 从操作数栈弹出一个数，写入到指定索引的全局变量；弹出的数的类型必须跟全局变量的一致

use std::{cell::RefCell, rc::Rc};

use crate::{object::EngineError, vm_module::VMModule};

pub fn local_get(vm_module: Rc<RefCell<VMModule>>, index: u32) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let offset = module.current_call_frame_base_pointer + (index as usize);
    let value = module.operand_stack.get_value(offset);
    module.operand_stack.push(value);
    Ok(())
}

pub fn local_set(vm_module: Rc<RefCell<VMModule>>, index: u32) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let value = module.operand_stack.pop();
    let offset = module.current_call_frame_base_pointer + (index as usize);
    module.operand_stack.set_value(offset, value);
    Ok(())
}

pub fn local_tee(vm_module: Rc<RefCell<VMModule>>, index: u32) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let value = module.operand_stack.peek();
    let offset = module.current_call_frame_base_pointer + (index as usize);
    module.operand_stack.set_value(offset, value);
    Ok(())
}

pub fn global_get(vm_module: Rc<RefCell<VMModule>>, index: u32) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let value = module.global_variables[index as usize]
        .as_ref()
        .borrow()
        .get_value();
    module.operand_stack.push(value);
    Ok(())
}

pub fn global_set(vm_module: Rc<RefCell<VMModule>>, index: u32) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();
    let value = module.operand_stack.pop();
    module.global_variables[index as usize]
        .as_ref()
        .borrow_mut()
        .set_value(value)?;
    Ok(())
}
