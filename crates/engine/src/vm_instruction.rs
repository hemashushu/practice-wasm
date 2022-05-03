// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, rc::Rc};

use anvm_parser::instruction::Instruction;

use crate::{ins_const, ins_parametric, instance::EngineError, vm_module::VMModule};

pub fn exec_instruction(
    vm_module: Rc<RefCell<VMModule>>,
    instruction: &Instruction,
) -> Result<(), EngineError> {
    match instruction {
        // 控制指令
        Instruction::Nop | Instruction::Else | Instruction::End => {
            // 无需任何操作
        }

        // 操作数（参数，parametric）指令
        Instruction::Drop => ins_parametric::drop(vm_module)?,
        Instruction::Select => ins_parametric::select(vm_module)?,

        // 常量指令
        Instruction::I32Const(value) => ins_const::i32_const(vm_module, *value)?,
        Instruction::I64Const(value) => ins_const::i64_const(vm_module, *value)?,
        Instruction::F32Const(value) => ins_const::f32_const(vm_module, *value)?,
        Instruction::F64Const(value) => ins_const::f64_const(vm_module, *value)?,

        _ => {
            return Err(EngineError::InvalidOperation(format!(
                "unsupported instruction: {:?}",
                instruction
            )));
        }
    }

    Ok(())
}

pub fn exec_instructions(
    vm_module: Rc<RefCell<VMModule>>,
    instructions: &[Instruction],
) -> Result<(), EngineError> {
    for instruction in instructions {
        exec_instruction(Rc::clone(&vm_module), instruction)?
    }
    Ok(())
}
