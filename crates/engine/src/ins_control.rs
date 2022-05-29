// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 调用函数的过程
//!
//! 1. 调用函数之前的栈内容如下：
//!
//!         | ------- 栈顶。------- |
//! 当前。   |                      |
//! 函数的   | 当前函数用于运算的槽位。 |
//! 栈帧。-- | ------- 栈底。 ------ |
//!
//! 注意数据增加的方向是 "从栈底到栈顶"，即 "栈顶" 是最新的数据。
//!
//! 2. 为方便讲解，假设在逻辑上还有一个 `局部变量` 表，每次函数调用都会创建一个
//!    单独的 `局部变量表`。`局部变量表` 除了用来存储函数调用的实参，也用于函数运行过程中
//!    的所有局部变量。
//! 3. `调用者` 把实参准备好，存放在栈顶，第一个参数位于靠近栈底，后面的参数靠近栈顶。
//!
//!                        局部变量表。 -- | ---- index N ----- |
//!                                      | 第 N 个局部变量空槽。 |
//!                                      | 第 0 个局部变量空槽。 |
//!         | ------- 栈顶。------- |     |................... |
//! 当前。   | 第 N 个参数值。        | --> | 第 N 个参数值。      |
//! 函数的   | 第 0 个参数值。        | --> | 第 0 个参数值。      |
//! 栈帧。-- | ....... 原栈顶 ...... |     | ---- index 0 ----- |
//!         |                      |
//!         | 当前函数用于运算的槽位。 |
//!         | ------- 栈底。 ------ |
//!
//! 4. VM 从栈顶弹出 N 个参数，并存入局部变量表。
//! 5. VM 在局部变量表开辟 N 个局部变量空槽，初始值均为 0（WebAssembly 规范似乎没有要求初始化局部变量）。
//! 6. VM 在栈顶创建一个新的栈帧。
//! 7. `被调用者` 在新的栈帧上执行当前函数的运算指令。
//!
//!                        局部变量表。 -- | ---- index N ----- |
//!                                      | 第 N 个局部变量空槽。 |
//!                                      | 第 0 个局部变量空槽。 |
//!         | ------- 栈顶。 ------ |     |................... |
//! 当前函   |                      |     | 第 N 个参数值。      |
//! 数栈帧-- | 当前函数用于运算的槽位。 | <=> | 第 0 个参数值。      |
//!         | -------------------- |     | ---- index 0 ----- |
//!         |                      |
//! 调用者-- | 上一个函数运算的槽位。。 |
//! 栈帧。   | ------- 栈底。 ------ |
//!
//! 8. `被调用者` 执行完毕之后，VM 把返回值暂存起来。
//! 9. VM 移除刚创建的栈帧。
//! 10. VM 将返回值压入操作数栈，第一个返回值先压入，后面的返回值后压入。
//!
//!
//!         | ------- 栈顶。 ------ |
//!         | `被调用者` 退出后留下   | <-- 在调用函数之前，这个区域存储的是
//!         | 的遗产，即返回值。      |     传递给 `被调用者` 的实参数据，
//! 当前。   | 第 N 个返回值。        |     调用完毕之后，这个区域存储的是
//! 函数的   | 第 0 个返回值。        |     `被调用者` 返回的值。
//! 栈帧。-- | .................... | <-- 这个是调用函数前的栈顶。
//!         |                      |
//!         | 当前函数用于运算的槽位。 |
//!         | ------- 栈底。 ------ |
//!
//! 注意：
//! - 以上是函数调用的逻辑，具体的实现可能有所不同；
//! - 有时 `被调用者` 可能会残存一些操作数以及局部变量在栈上，所以在调用函数前需要
//!   记录新栈帧的起始位置（同时也是第 0 个实参的位置），以便调用完目标函数后
//!   清除整个新栈帧，这样目标函数在操作数栈上残留的数据也能被清除干净，
//!   让有用的数据（即返回值）刚好承接在调用前的栈顶。
//!
//! ## 调用栈 call stack
//!
//! 一连串调用帧（call frame）堆起来的栈
//!
//! - f3  <-- 栈顶（当前调用帧，当前函数）
//! - f2
//! - f1
//! - f0  <-- 栈底

//! # 原始控制指令
//!
//! ## call 指令
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
//! ## call_indirect 指令
//!
//! 间接函数调用，格式
//!
//! call_indirect type_idx:uint32 table_idx:uint32
//!
//! 其中 table_idx 的值目前只能是 0
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
//!
//! ## return 指令
//!
//! return 指令相当于 br 指令跳到函数本身一层，比如当 return 出现在一层 block 里，
//! 作用相当于 `br 1`。

//! # 转换指令
//!
//! 所有控制指令均被 transformer 转换为便于当前 vm 直接解析的 Instruction::Control 指令
//!
//! 转换关系是：
//!
//!
use anvm_ast::types::Value;

use crate::{
    error::{make_invalid_operand_data_types_engine_error, make_invalid_operand_data_types_2_engine_error, EngineError},
    vm::VM,
};

/// 处理 call 指令
///
/// 该函数将会创建了一个调用帧，并且更改 pc 值
pub fn call(vm: &mut VM, function_index: u32) -> Result<(), EngineError> {
    let (argument_count, result_count, local_variable_types) = {
        let vm_module = vm.context.vm_modules[vm_module_index];
        let function_type = &vm_module.function_types[type_index];
        let local_variable_types = &vm_module
            .internal_function_local_variable_types_list[internal_function_index];
        (
            function_type.params.len(),
            function_type.results.len(),
            local_variable_types.to_owned(),
        )
    };

    let vm_function = rc_function
        .as_ref()
        .as_any()
        .downcast_ref::<VMFunction>()
        .expect("should be a VMFunction object");

    let function_index = vm_function.function_index;
    let rc_function_type = Rc::clone(&vm_function.function_type);

    match &vm_function.function_kind {
        VMFunctionKind::External(external_function) => {
            call_external_func(vm_module, &rc_function_type, Rc::clone(external_function))
        }
        VMFunctionKind::Internal {
            local_groups,
            expression,
            vm_module: _,
        } => {
            call_internal_function(
                vm_module,
                &rc_function_type,
                function_index,
                local_groups,
                expression,
            );
            Ok(())
        }
    }
}

pub fn call_internal_function(
    vm: &mut VM,
    function_type: &Rc<FunctionType>,
    function_index: usize,
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
        FrameType::Call,
        function_type,
        Some(function_index),
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
    vm: &mut VM,
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

fn pop_arguments(vm: &mut VM, function_type: &Rc<FunctionType>) -> Vec<Value> {
    let parameter_count = function_type.params.len();
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .pop_values(parameter_count)
}

fn push_results(vm: &mut VM, results: &[Value]) {
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push_values(results)
}

pub fn call_indirect(
    vm: &mut VM,
    function_type_index: u32,
    table_index: u32,
) -> Result<(), EngineError> {
    if table_index != 0 {
        return Err(make_invalid_table_index_engine_error());
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

    let function_index = vm_function.function_index;
    let rc_function_type = Rc::clone(&vm_function.function_type);

    match &vm_function.function_kind {
        VMFunctionKind::External(external_function) => {
            call_external_func(vm_module, &rc_function_type, Rc::clone(external_function))
        }
        VMFunctionKind::Internal {
            local_groups,
            expression,
            vm_module: _,
        } => {
            call_internal_function(
                vm_module,
                &rc_function_type,
                function_index,
                local_groups,
                expression,
            );
            Ok(())
        }
    }
}
