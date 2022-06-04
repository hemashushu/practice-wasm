// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 函数调用相关指令
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
//! 指令 call_indirect 的操作步骤：
//! 1. 从操作数栈弹出一个 u32 数，该数是表内项目的索引
//! 2. 从表里获取指定的项目，也就是目标函数的索引值
//! 3. 通过函数索引值获取目标函数
//! 4. 调用目标函数
//!
//! ```diagram
//! | 操作数栈。。  |       | 表。。       |     | 函数列表。。 |
//! | ----------- |       | ----------- |     | ---------- |
//! | -- 栈顶。 -- |       | #0 func#2   |  /--> #3 sub     |
//! | 1u32 -------- pop --> #1 func#3 ----/   | #4 mul     |
//! | ...         |       | #2 func#5   |     | #5 div     |
//! | -- 栈底。 -- |       | ...         |     | ...        |
//! ```
//!
//! ## return 指令
//!
//! return 指令相当于 br 指令跳到函数本身一层，比如当 return 出现在一层 block 里，
//! 作用相当于 `br 1`。

//! # 调用函数的过程
//!
//! 1. 调用函数之前的栈内容如下：
//!
//! ```diagram
//!         | ------- 栈顶。------- |
//! 当前。   |                      |
//! 函数的   | 当前函数用于运算的槽位。 |
//! 栈帧。-- | ------- 栈底。 ------ |
//! ```
//!
//! 注意数据增加的方向是 "从栈底到栈顶"，即 "栈顶" 是最新的数据。
//!
//! 2. 为方便讲解，假设在逻辑上还有一个 `局部变量` 表，每次函数调用都会创建一个
//!    单独的 `局部变量表`。`局部变量表` 除了用来存储函数调用的实参，也用于函数运行过程中
//!    的所有局部变量。
//! 3. `调用者` 把实参准备好，存放在栈顶，第一个参数位于靠近栈底，后面的参数靠近栈顶。
//!
//! ```diagram
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
//! ```
//!
//! 4. VM 从栈顶弹出 N 个参数，并存入局部变量表。
//! 5. VM 在局部变量表开辟 N 个局部变量空槽，初始值均为 0（WebAssembly 规范似乎没有要求初始化局部变量）。
//! 6. VM 在栈顶创建一个新的栈帧。
//! 7. `被调用者` 在新的栈帧上执行当前函数的运算指令。
//!
//! ```diagram
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
//! ```
//!
//! 8. `被调用者` 执行完毕之后，VM 把返回值暂存起来。
//! 9. VM 移除刚创建的栈帧。
//! 10. VM 将返回值压入操作数栈，第一个返回值先压入，后面的返回值后压入。
//!
//! ```diagram
//!         | ------- 栈顶。 ------ |
//!         | `被调用者` 退出后留下   | <-- 在调用函数之前，这个区域存储的是
//!         | 的遗产，即返回值。      |     传递给 `被调用者` 的实参数据，
//! 当前。   | 第 N 个返回值。        |     调用完毕之后，这个区域存储的是
//! 函数的   | 第 0 个返回值。        |     `被调用者` 返回的值。
//! 栈帧。-- | .................... | <-- 这个是调用函数前的栈顶。
//!         |                      |
//!         | 当前函数用于运算的槽位。 |
//!         | ------- 栈底。 ------ |
//! ```
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

use anvm_ast::{
    instruction::BlockType,
    types::{check_types, check_value_types, Value, ValueTypeCheckError},
};

use crate::{
    error::{
        make_invalid_operand_data_type_engine_error, make_invalid_table_index_engine_error,
        make_mismatch_function_type_engine_error, EngineError,
    },
    ins_control::ControlResult,
    object::FunctionItem,
    vm::VM,
    vm_stack::INFO_SEGMENT_ITEM_COUNT,
};

pub fn call(
    vm: &mut VM,
    vm_module_index: usize,
    type_index: usize,
    function_index: usize,
    internal_function_index: usize,
    address: usize,
) -> Result<ControlResult, EngineError> {
    let (parameter_types, local_variable_types) = {
        let vm_module = &vm.resource.vm_modules[vm_module_index];
        let function_type = &vm_module.function_types[type_index];
        let local_variable_types =
            &vm_module.internal_function_local_variable_types_list[internal_function_index];
        (&function_type.params, local_variable_types.to_owned())
    };

    // 判断操作数是否足够当前函数或结构块用于返回
    let parameter_count = parameter_types.len();
    let stack_size = vm.stack.get_size();
    let operands_count = stack_size - vm.status.base_pointer - INFO_SEGMENT_ITEM_COUNT;
    if operands_count < parameter_count {
        return Err(EngineError::InvalidOperation(format!(
            "failed to call function {} (module {}), not enough operands, expected: {}, actual: {}",
            function_index, vm_module_index, parameter_count, operands_count
        )));
    }

    let arguments = vm.stack.peek_values(parameter_count);

    // 核对实参的数据类型和数量
    match check_value_types(arguments, parameter_types) {
        Err(ValueTypeCheckError::LengthMismatch) => unreachable!(),
        // {
        //     return Err(EngineError::InvalidOperation(format!(
        //         "failed to call function {} (module {}). The number of parameters does not match, expected: {}, actual: {}",
        //         function_index, vm_module_index, parameter_count, arguments.len())));
        // }
        Err(ValueTypeCheckError::DataTypeMismatch(index)) => {
            return Err(EngineError::InvalidOperation(format!(
                "failed to call function {} (module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                function_index, vm_module_index, index + 1,
                parameter_types[index],
                arguments[index].get_type())));
        }
        _ => {
            // pass
        }
    }

    // 压入调用栈
    // 返回地址应该是 `call 指令` 的下一个指令
    let return_address = vm.status.address + 1;
    vm.push_call_frame(parameter_count, &local_variable_types, return_address);

    // 返回新的状态信息，让调用者更新虚拟机状态
    let control_result = ControlResult::PushStackFrame {
        is_call_frame: true,
        vm_module_index,
        function_index,
        frame_type: BlockType::TypeIndex(type_index as u32),
        address,
    };

    Ok(control_result)
}

/// 从模块内部函数调用本地函数的过程。
///
/// 示例：
///
/// |-----------------------------|
/// |       native function       |
/// |                             |
/// | (a,b,c)     -->  (x,y)      |
/// |  ^ ^ ^            | | push  |
/// |  | | | pop        V V       |
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
/// 注：
/// 先弹出的参数放在参数列表的右边（大索引端），
/// 对于返回值，左边（小索引端）的数值先压入。
pub fn call_native(
    vm: &mut VM,
    native_module_index: usize,
    type_index: usize,
    function_index: usize,
) -> Result<ControlResult, EngineError> {
    let (parameter_types, native_function) = {
        let native_module = &vm.resource.native_modules[native_module_index];
        let function_type = &native_module.function_types[type_index];
        let native_function = native_module.native_functions[function_index];

        (function_type.params.to_owned(), native_function)
    };

    // 判断操作数是否足够当前函数或结构块用于返回
    let parameter_count = parameter_types.len();
    let stack_size = vm.stack.get_size();
    let operands_count = stack_size - vm.status.base_pointer - INFO_SEGMENT_ITEM_COUNT;
    if operands_count < parameter_count {
        return Err(EngineError::InvalidOperation(format!(
            "failed to call function {} (native module {}), not enough operands, expected: {}, actual: {}",
            function_index, native_module_index, parameter_count, operands_count
        )));
    }

    // 从栈弹出数据，作为函数调用的参数
    let arguments = vm.stack.pop_values(parameter_count);

    // 核对实参的数据类型和数量
    match check_value_types(&arguments, &parameter_types) {
        Err(ValueTypeCheckError::DataTypeMismatch(index)) => {
            return Err(EngineError::InvalidOperation(format!(
                "failed to call function {} (native module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                function_index, native_module_index, index + 1,
                parameter_types[index],
                arguments[index].get_type())));
        }
        _ => {
            // pass
        }
    }

    let result = native_function(&arguments);

    match result {
        Ok(result_values) => {
            // 将数据压入栈，作为函数调用的返回值
            vm.stack.push_values(&result_values);

            // 本地函数的调用并不会进入到函数体，所以调用完毕之后只需继续
            // 执行下一个指令即可。
            Ok(ControlResult::Sequence)
        }
        Err(e) => Err(EngineError::NativeError(e)),
    }
}

pub fn call_indirect(
    vm: &mut VM,
    type_index: usize,
    table_index: usize,
) -> Result<ControlResult, EngineError> {
    if table_index != 0 {
        return Err(make_invalid_table_index_engine_error());
    }

    let element_index = {
        let element_index_value = vm.stack.pop();
        match element_index_value {
            Value::I32(index) => index as usize,
            _ => {
                return Err(make_invalid_operand_data_type_engine_error(
                    "call_indirect",
                    "i32",
                ));
            }
        }
    };

    let (vm_module_index, function_index, function_item, expected_function_type) = {
        let vm_module_index = vm.status.vm_module_index;
        let vm_module = &vm.resource.vm_modules[vm_module_index];
        let table = &vm.resource.tables[vm_module.table_index];

        let element_item = table.get_element(element_index)?;
        let function_index = match element_item {
            Some(index) => index as usize,
            _ => {
                return Err(EngineError::ObjectNotFound(format!(
                    "can not found table element {} (module {})",
                    element_index, vm_module_index
                )))
            }
        };

        let function_item = &vm_module.function_items[function_index];
        let expected_function_type = &vm_module.function_types[type_index];

        (
            vm_module_index,
            function_index,
            function_item.to_owned(),
            expected_function_type.to_owned(),
        )
    };

    // 核对函数的签名
    let actual_function_type = match &function_item {
        FunctionItem::Normal {
            vm_module_index,
            type_index,
            function_index: _,
            internal_function_index: _,
            start_address: _,
            end_address: _,
            block_items: _,
        } => {
            let vm_module = &vm.resource.vm_modules[*vm_module_index];
            let function_type = &vm_module.function_types[*type_index];
            function_type
        }
        FunctionItem::Native {
            native_module_index,
            type_index,
            function_index: _,
        } => {
            let native_module = &vm.resource.native_modules[*native_module_index];
            let function_type = &native_module.function_types[*type_index];
            function_type
        }
    };

    if let Err(_) = check_types(&expected_function_type.params, &actual_function_type.params) {
        return Err(make_mismatch_function_type_engine_error(
            function_index,
            vm_module_index,
        ));
    }

    if let Err(_) = check_types(
        &expected_function_type.results,
        &actual_function_type.results,
    ) {
        return Err(make_mismatch_function_type_engine_error(
            function_index,
            vm_module_index,
        ));
    }

    // 调用处理函数
    match &function_item {
        FunctionItem::Normal {
            vm_module_index,
            type_index,
            function_index,
            internal_function_index,
            start_address,
            end_address: _,
            block_items: _,
        } => call(
            vm,
            *vm_module_index,
            *type_index,
            *function_index,
            *internal_function_index,
            *start_address,
        ),
        FunctionItem::Native {
            native_module_index,
            type_index,
            function_index,
        } => call_native(vm, *native_module_index, *type_index, *function_index),
    }
}
