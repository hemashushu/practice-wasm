// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, rc::Rc};

use anvm_parser::instruction::Instruction;

use crate::{
    ins_const, ins_numeric_comparsion, ins_numeric_eqz, ins_parametric, instance::EngineError,
    vm_module::VMModule, ins_numeric_unary,
};

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

        // 零值测试指令
        Instruction::I32Eqz => ins_numeric_eqz::i32_eqz(vm_module)?,
        Instruction::I64Eqz => ins_numeric_eqz::i64_eqz(vm_module)?,

        // 数值比较指令
        Instruction::I32Eq => ins_numeric_comparsion::i32_eq(vm_module)?,
        Instruction::I32Ne => ins_numeric_comparsion::i32_ne(vm_module)?,
        Instruction::I32LtS => ins_numeric_comparsion::i32_lt_s(vm_module)?,
        Instruction::I32LtU => ins_numeric_comparsion::i32_lt_u(vm_module)?,
        Instruction::I32GtS => ins_numeric_comparsion::i32_gt_s(vm_module)?,
        Instruction::I32GtU => ins_numeric_comparsion::i32_gt_u(vm_module)?,
        Instruction::I32LeS => ins_numeric_comparsion::i32_le_s(vm_module)?,
        Instruction::I32LeU => ins_numeric_comparsion::i32_le_u(vm_module)?,
        Instruction::I32GeS => ins_numeric_comparsion::i32_ge_s(vm_module)?,
        Instruction::I32GeU => ins_numeric_comparsion::i32_ge_u(vm_module)?,

        Instruction::I64Eq => ins_numeric_comparsion::i64_eq(vm_module)?,
        Instruction::I64Ne => ins_numeric_comparsion::i64_ne(vm_module)?,
        Instruction::I64LtS => ins_numeric_comparsion::i64_lt_s(vm_module)?,
        Instruction::I64LtU => ins_numeric_comparsion::i64_lt_u(vm_module)?,
        Instruction::I64GtS => ins_numeric_comparsion::i64_gt_s(vm_module)?,
        Instruction::I64GtU => ins_numeric_comparsion::i64_gt_u(vm_module)?,
        Instruction::I64LeS => ins_numeric_comparsion::i64_le_s(vm_module)?,
        Instruction::I64LeU => ins_numeric_comparsion::i64_le_u(vm_module)?,
        Instruction::I64GeS => ins_numeric_comparsion::i64_ge_s(vm_module)?,
        Instruction::I64GeU => ins_numeric_comparsion::i64_ge_u(vm_module)?,

        Instruction::F32Eq => ins_numeric_comparsion::f32_eq(vm_module)?,
        Instruction::F32Ne => ins_numeric_comparsion::f32_ne(vm_module)?,
        Instruction::F32Lt => ins_numeric_comparsion::f32_lt(vm_module)?,
        Instruction::F32Gt => ins_numeric_comparsion::f32_gt(vm_module)?,
        Instruction::F32Le => ins_numeric_comparsion::f32_le(vm_module)?,
        Instruction::F32Ge => ins_numeric_comparsion::f32_ge(vm_module)?,

        Instruction::F64Eq => ins_numeric_comparsion::f64_eq(vm_module)?,
        Instruction::F64Ne => ins_numeric_comparsion::f64_ne(vm_module)?,
        Instruction::F64Lt => ins_numeric_comparsion::f64_lt(vm_module)?,
        Instruction::F64Gt => ins_numeric_comparsion::f64_gt(vm_module)?,
        Instruction::F64Le => ins_numeric_comparsion::f64_le(vm_module)?,
        Instruction::F64Ge => ins_numeric_comparsion::f64_ge(vm_module)?,

        // 一元运算
        Instruction::I32Clz => ins_numeric_unary::i32_clz(vm_module)?,
        Instruction::I32Ctz => ins_numeric_unary::i32_ctz(vm_module)?,
        Instruction::I32PopCnt => ins_numeric_unary::i32_popcnt(vm_module)?,

        Instruction::I64Clz => ins_numeric_unary::i64_clz(vm_module)?,
        Instruction::I64Ctz => ins_numeric_unary::i64_ctz(vm_module)?,
        Instruction::I64PopCnt => ins_numeric_unary::i64_popcnt(vm_module)?,

        Instruction::F32Abs => ins_numeric_unary::f32_abs(vm_module)?,
        Instruction::F32Neg => ins_numeric_unary::f32_neg(vm_module)?,
        Instruction::F32Ceil => ins_numeric_unary::f32_ceil(vm_module)?,
        Instruction::F32Floor => ins_numeric_unary::f32_floor(vm_module)?,
        Instruction::F32Trunc => ins_numeric_unary::f32_trunc(vm_module)?,
        Instruction::F32Nearest => ins_numeric_unary::f32_nearest(vm_module)?,
        Instruction::F32Sqrt => ins_numeric_unary::f32_sqrt(vm_module)?,

        Instruction::F64Abs => ins_numeric_unary::f64_abs(vm_module)?,
        Instruction::F64Neg => ins_numeric_unary::f64_neg(vm_module)?,
        Instruction::F64Ceil => ins_numeric_unary::f64_ceil(vm_module)?,
        Instruction::F64Floor => ins_numeric_unary::f64_floor(vm_module)?,
        Instruction::F64Trunc => ins_numeric_unary::f64_trunc(vm_module)?,
        Instruction::F64Nearest => ins_numeric_unary::f64_nearest(vm_module)?,
        Instruction::F64Sqrt => ins_numeric_unary::f64_sqrt(vm_module)?,

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
