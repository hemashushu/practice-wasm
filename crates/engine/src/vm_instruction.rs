// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_parser::instruction::Instruction;

use crate::{ins_const, instance::EngineError, vm_module::VMModule};

pub fn exec_instruction(
    vm_module: &mut VMModule,
    instruction: &Instruction,
) -> Result<(), EngineError> {
    match instruction {
        Instruction::I32Const(value) => ins_const::i32_const(vm_module, *value)?,
        Instruction::I64Const(value) => ins_const::i64_const(vm_module, *value)?,
        Instruction::F32Const(value) => ins_const::f32_const(vm_module, *value)?,
        Instruction::F64Const(value) => ins_const::f64_const(vm_module, *value)?,
        _ => {
            return Err(EngineError::InvalidOperation(
                "unsupported instruction".to_string(),
            ));
        }
    }

    Ok(())
}
