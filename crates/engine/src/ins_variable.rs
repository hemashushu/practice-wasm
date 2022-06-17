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

use crate::{
    error::{EngineError, InvalidOperation, TypeMismatch},
    vm::VM,
    vm_global_variable::SetGlobalVariableError,
};

pub fn local_get(vm: &mut VM, index: u32) -> Result<(), EngineError> {
    let offset = vm.status.local_pointer + (index as usize);

    let stack = &mut vm.stack;
    let value = stack.get_value(offset);
    stack.push(value);
    Ok(())
}

pub fn local_set(vm: &mut VM, index: u32) -> Result<(), EngineError> {
    let offset = vm.status.local_pointer + (index as usize);

    let stack = &mut vm.stack;
    let value = stack.pop();
    stack.set_value(offset, value);
    Ok(())
}

pub fn local_tee(vm: &mut VM, index: u32) -> Result<(), EngineError> {
    let offset = vm.status.local_pointer + (index as usize);

    let stack = &mut vm.stack;
    let value = stack.peek();
    stack.set_value(offset, value);
    Ok(())
}

pub fn global_get(vm: &mut VM, index: u32) -> Result<(), EngineError> {
    let instance_global_variable_index =
        vm.resource.vm_modules[vm.status.vm_module_index].global_variable_indexes[index as usize];
    let value = vm.resource.global_variables[instance_global_variable_index].get_value();

    let stack = &mut vm.stack;
    stack.push(value);
    Ok(())
}

pub fn global_set(vm: &mut VM, index: u32) -> Result<(), EngineError> {
    let stack = &mut vm.stack;
    let value = stack.pop();

    let vm_module_index = vm.status.vm_module_index;
    let instance_global_variable_index =
        vm.resource.vm_modules[vm_module_index].global_variable_indexes[index as usize];
    let vm_global_variable = &mut vm.resource.global_variables[instance_global_variable_index];

    match vm_global_variable.set_value(value) {
        Err(SetGlobalVariableError::Immutable) => Err(EngineError::InvalidOperation(
            InvalidOperation::ImmutableGlobalVariable(vm_module_index, index as usize),
        )),
        Err(SetGlobalVariableError::TypeMismatch) => Err(EngineError::TypeMismatch(
            TypeMismatch::SetGlobalVariableValueTypeMismatch(
                vm_module_index,
                index as usize,
                vm_global_variable.get_global_type().value_type.clone(),
                value.get_type(),
            ),
        )),
        _ => Ok(()),
    }
}
