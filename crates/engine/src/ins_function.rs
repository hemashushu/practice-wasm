// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use anvm_parser::{
    ast::{FunctionType, LocalGroup},
    instruction::Instruction, types::Value,
};

use crate::{vm_control_stack::VMFrameType, vm_module::VMModule};

pub fn call_internal_function(
    vm_module: &mut VMModule,
    function_type: &Rc<FunctionType>,
    local_groups: &Vec<LocalGroup>,
    instructions: &Rc<Vec<Instruction>>,
) {
    let local_variable_count = local_groups
        .iter()
        .fold(0, |acc, local_group| acc + local_group.variable_count) as usize;

    // 创建被进入新的调用帧
    vm_module.enter_control_block(
        VMFrameType::Call,
        function_type,
        instructions,
        local_variable_count,
    );

    // 分配局部变量空槽
    // 局部变量的空槽初始值为 0:i32
    let placehold_values = vec![Value::I32(0); local_variable_count];
    vm_module.operand_stack.push_values(&placehold_values)
}
