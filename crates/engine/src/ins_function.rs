// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// # 调用函数的过程
//
// 1. 调用函数之前的栈
//
//         | ------- 栈顶。------- |
// 当前。   |                      |
// 函数的   | 当前函数用于运算的槽位。 |
// 栈帧。-- | ------- 栈底。 ------ |
//
// 2. 为方便讲解，假设在逻辑上还有一个 `局部变量` 表，每次函数调用都会创建一个
//    单独的局部变量表。`局部变量表` 除了用来存储函数调用的实参，也用于函数运行过程中
//    所有需要的局部变量。
// 3. `调用者` 把实参准备好，存放在操作数栈顶，第一个参数位于靠近栈底，后面的参数靠近栈顶。
//
//                        局部变量表。 -- | ---- index N ----- |
//                                      | 第 N 个局部变量空槽。 |
//                                      | 第 0 个局部变量空槽。 |
//         | ------- 栈顶。------- |     |................... |
// 当前。   | 第 N 个参数值。        | --> | 第 N 个参数值。      |
// 函数的   | 第 0 个参数值。        | --> | 第 0 个参数值。      |
// 栈帧。-- | ....... 原栈顶 ...... |     | ---- index 0 ----- |
//         |                      |
//         | 当前函数用于运算的槽位。 |
//         | ------- 栈底。 ------ |
//
// 4. `被调用者` 从操作数栈弹出 N 个参数，并存入局部变量表。
// 5. `被调用者` 在局部变量表开辟 N 个局部变量空槽，初始值均为 0。
// 6. `被调用者` 在操作数栈上执行当前函数的指令。
//
//                        局部变量表。 -- | ---- index N ----- |
//                                      | 第 N 个局部变量空槽。 |
//                                      | 第 0 个局部变量空槽。 |
//         | ------- 栈顶。 ------ |     |................... |
// 当前函   |                      |     | 第 N 个参数值。      |
// 数栈帧-- | 当前函数用于运算的槽位。 | <=> | 第 0 个参数值。      |
//         | -------------------- |     | ---- index 0 ----- |
//         |                      |
// 调用者-- | 上一个函数运算的槽位。。 |
// 栈帧。   | ------- 栈底。 ------ |
//
// 7. `被调用者` 将返回值压入操作数栈，第一个返回值先压入，后面的返回值后压入。
//
//
//         | ------- 栈顶。 ------ |
//         | `被调用者` 退出后留下   | <-- 在调用函数之前，这个区域存储的是
//         | 的遗产，即返回值。      |     传递给 `被调用者` 的实参数据，
// 当前。   | 第 N 个返回值。        |     调用完毕之后，这个区域存储的是
// 函数的   | 第 0 个返回值。        |     `被调用者` 返回的值。
// 栈帧。-- | .................... | <-- 这个是调用函数前的栈顶。
//         |                      |
//         | 当前函数用于运算的槽位。 |
//         | ------- 栈底。 ------ |
//
// 注意：
// - 以上是函数调用的逻辑，具体的实现可能有所不同
// - 有时 `被调用者` 可能会残存一些局部变量在操作数栈上，所以在调用函数前需要
//   记录新帧栈的起始位置（同时也是第 0 个实参的位置），以便调用完目标函数后
//   清除目标函数在操作数栈上残留的数据，让有用的数据（即返回值）刚好承接在
//   调用前的栈顶。
//
// ## 调用栈 call stack
//
// 一连串调用帧（call frame）堆起来的栈
//
// - f3  <-- 栈顶（当前调用帧，当前函数）
// - f2
// - f1
// - f0  <-- 栈底
//

//! # call 指令
//!
//! 格式
//!
//! call func_idx:uint32
//!
//! 注：
//! 函数索引值是包括 "导入的函数" 的，而且是先从导入函数开始排索引，
//! 然后才到 "当前模块定义的函数（即内部函数）"。
//! 比如一个模块有 3 个函数导入和 2 个内部函数，则第一个内部函数的索引值为 3。
//!
//! # call_indirect 指令
//!
//! 间接函数调用，格式
//!
//! call_indirect type_idx:uint32 table_idx:uint32
//!
//! 其中 table_idx 的值目前只能是 0
//!

use std::{cell::RefCell, rc::Rc};

use anvm_parser::{
    ast::{FunctionType, LocalGroup},
    instruction::Instruction,
    types::Value,
};

use crate::{
    instance::{EngineError, Function},
    vm_control_stack::VMFrameType,
    vm_function::{FunctionItem, VMFunction},
    vm_module::{enter_control_block, VMModule},
};

/// 处理 call 指令
///
/// 该函数仅仅创建了一个调用帧，并不会自动开始
/// 执行函数当中的指令
pub fn call(vm_module: Rc<RefCell<VMModule>>, function_index: u32) -> Result<(), EngineError> {
    let rc_function = {
        let module = vm_module.as_ref().borrow();
        Rc::clone(&module.functions[function_index as usize])
    };

    let vm_function = rc_function
        .as_ref()
        .as_any()
        .downcast_ref::<VMFunction>()
        .expect("should be a VMFunction object");

    let rc_function_type = Rc::clone(&vm_function.function_type);

    match &vm_function.function_item {
        FunctionItem::External(external_function) => {
            call_external_func(vm_module, &rc_function_type, Rc::clone(external_function))
        }
        FunctionItem::Internal {
            local_groups,
            expression,
            vm_module: _,
        } => {
            call_internal_function(vm_module, &rc_function_type, local_groups, expression);
            Ok(())
        }
    }
}

pub fn call_internal_function(
    vm_module: Rc<RefCell<VMModule>>,
    function_type: &Rc<FunctionType>,
    local_groups: &Vec<LocalGroup>,
    instructions: &Rc<Vec<Instruction>>,
) {
    let local_variable_count = local_groups
        .iter()
        .fold(0, |acc, local_group| acc + local_group.variable_count)
        as usize;

    // 创建被进入新的调用帧
    enter_control_block(
        Rc::clone(&vm_module),
        VMFrameType::Call,
        function_type,
        instructions,
        local_variable_count,
    );

    // 分配局部变量空槽
    // 局部变量的空槽初始值为 0:i32
    let placehold_values = vec![Value::I32(0); local_variable_count];
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push_values(&placehold_values)
}

/// 从模块内部函数调用外部函数的过程。
///
/// 先弹出的参数放在参数列表的右边（大索引端），
/// 对于返回值，左边（小索引端）的数值先压入。
///
/// 示例：
///
/// |-----------------------------|
/// |     external function       |
/// |                             |
/// | (a,b,c)     -->  (x,y)      |
/// |  ^ ^ ^            | |       |
/// |  | | |            V V       |
/// |                             |
/// |--- 栈顶。---|   |--- 栈顶。---|
/// | - c        |   |            |
/// | - b        |   | - y        |
/// | - a        |   | - x        |
/// | - ...      |   | - ...      |
/// |--- 栈底。---|   |--- 栈顶。---|
/// |  ^                |         |
/// |  |                V         |
/// |  START            END       |
///
fn call_external_func(
    vm_module: Rc<RefCell<VMModule>>,
    function_type: &Rc<FunctionType>,
    function: Rc<dyn Function>,
) -> Result<(), EngineError> {
    let arguments = pop_arguments(Rc::clone(&vm_module), function_type);
    let results = function.eval(&arguments)?;

    if arguments.len() != function_type.params.len() {
        return Err(EngineError::InvalidOperation(
            "the number of arguments and parameters do not match".to_string(),
        ));
    }

    push_results(Rc::clone(&vm_module), &results);
    Ok(())
}

fn pop_arguments(vm_module: Rc<RefCell<VMModule>>, function_type: &Rc<FunctionType>) -> Vec<Value> {
    let parameter_count = function_type.params.len();
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .pop_values(parameter_count)
}

fn push_results(vm_module: Rc<RefCell<VMModule>>, results: &[Value]) {
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push_values(results)
}

pub fn call_indirect(
    vm_module: Rc<RefCell<VMModule>>,
    function_type_index: u32,
    table_index: u32,
) -> Result<(), EngineError> {
    if table_index != 0 {
        return Err(EngineError::InvalidOperation(
            "only table with index value of 0 is allowed".to_string(),
        ));
    }

    let rc_function = {
        let mut module = vm_module.as_ref().borrow_mut();
        let element_index_value = module.operand_stack.pop();

        let element_index = match element_index_value {
            Value::I32(i) => i,
            _ => return Err(EngineError::InvalidOperation(
                "the value type of the operand for instruction \"call_indirect\" should be \"i32\""
                    .to_string(),
            )),
        };

        let rc_function = match module.table.borrow().get_element(element_index as usize)? {
            Some(f) => f,
            _ => {
                return Err(EngineError::ObjectNotFound(format!(
                    "can not found the element(function) by the index: {}",
                    element_index
                )))
            }
        };

        // 这里检查函数类型（函数的签名）是否匹配
        if module.ast_module.function_types[function_type_index as usize].as_ref()
            != rc_function.as_ref().get_function_type().as_ref()
        {
            return Err(EngineError::InvalidOperation(
                "function type mismatch in indirect call".to_string(),
            ));
        }

        rc_function
    };

    let vm_function = rc_function
        .as_ref()
        .as_any()
        .downcast_ref::<VMFunction>()
        .expect("should be a VMFunction object");

    let rc_function_type = Rc::clone(&vm_function.function_type);

    match &vm_function.function_item {
        FunctionItem::External(external_function) => {
            call_external_func(vm_module, &rc_function_type, Rc::clone(external_function))
        }
        FunctionItem::Internal {
            local_groups,
            expression,
            vm_module: _,
        } => {
            call_internal_function(vm_module, &rc_function_type, local_groups, expression);
            Ok(())
        }
    }
}
