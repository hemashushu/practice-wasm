// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{cell::RefCell, rc::Rc};

use anvm_parser::instruction::Instruction;

use crate::{
    ins_branch, ins_const, ins_function, ins_memory, ins_numeric_binary, ins_numeric_comparsion,
    ins_numeric_convert, ins_numeric_eqz, ins_numeric_unary, ins_parametric, ins_variable,
    object::EngineError, vm_module::VMModule,
};

pub fn exec_instruction(
    vm_module: Rc<RefCell<VMModule>>,
    instruction: &Instruction,
) -> Result<(), EngineError> {
    match instruction {
        // 控制指令
        Instruction::Unreachable => Err(EngineError::InvalidOperation(
            "unreachable instruction is executed".to_string(),
        )),
        Instruction::Nop | Instruction::Else | Instruction::End => {
            // 无需任何操作
            Ok(())
        }

        // 分支指令
        Instruction::Block { block_type, body } => ins_branch::exec_block(vm_module, block_type, body),
        Instruction::Loop { block_type, body } => ins_branch::exec_loop(vm_module, block_type, body),
        Instruction::If {
            block_type,
            consequet_body,
            alternate_body,
        } => ins_branch::exec_if(vm_module, block_type, consequet_body, alternate_body),
        Instruction::Br(relative_depth) => ins_branch::br(vm_module, *relative_depth),
        Instruction::BrIf(relative_depth) => ins_branch::br_if(vm_module, *relative_depth),
        Instruction::BrTable {
            relative_depths,
            default_relative_depth,
        } => ins_branch::br_table(vm_module, relative_depths, default_relative_depth),
        Instruction::Return => ins_branch::exec_return(vm_module),

        // 函数指令
        Instruction::Call(function_index) => ins_function::call(vm_module, *function_index),
        Instruction::CallIndirect(function_type_index, table_index) => {
            ins_function::call_indirect(vm_module, *function_type_index, *table_index)
        }

        // 操作数（参数，parametric）指令
        Instruction::Drop => ins_parametric::drop(vm_module),
        Instruction::Select => ins_parametric::select(vm_module),

        // 变量指令
        Instruction::LocalGet(index) => ins_variable::local_get(vm_module, *index),
        Instruction::LocalSet(index) => ins_variable::local_set(vm_module, *index),
        Instruction::LocalTee(index) => ins_variable::local_tee(vm_module, *index),
        Instruction::GlobalGet(index) => ins_variable::global_get(vm_module, *index),
        Instruction::GlobalSet(index) => ins_variable::global_set(vm_module, *index),

        // 内存指令
        Instruction::MemorySize(memory_block_index) => {
            ins_memory::memory_size(vm_module, *memory_block_index)
        }

        Instruction::MemoryGrow(memory_block_index) => {
            ins_memory::memory_grow(vm_module, *memory_block_index)
        }

        Instruction::I32Load(memory_args) => ins_memory::i32_load(vm_module, memory_args),
        Instruction::I32Load16S(memory_args) => ins_memory::i32_load16_s(vm_module, memory_args),
        Instruction::I32Load16U(memory_args) => ins_memory::i32_load16_u(vm_module, memory_args),
        Instruction::I32Load8S(memory_args) => ins_memory::i32_load8_s(vm_module, memory_args),
        Instruction::I32Load8U(memory_args) => ins_memory::i32_load8_u(vm_module, memory_args),

        Instruction::I64Load(memory_args) => ins_memory::i64_load(vm_module, memory_args),
        Instruction::I64Load32S(memory_args) => ins_memory::i64_load32_s(vm_module, memory_args),
        Instruction::I64Load32U(memory_args) => ins_memory::i64_load32_u(vm_module, memory_args),
        Instruction::I64Load16S(memory_args) => ins_memory::i64_load16_s(vm_module, memory_args),
        Instruction::I64Load16U(memory_args) => ins_memory::i64_load16_u(vm_module, memory_args),
        Instruction::I64Load8S(memory_args) => ins_memory::i64_load8_s(vm_module, memory_args),
        Instruction::I64Load8U(memory_args) => ins_memory::i64_load8_u(vm_module, memory_args),

        Instruction::F32Load(memory_args) => ins_memory::f32_load(vm_module, memory_args),
        Instruction::F64Load(memory_args) => ins_memory::f64_load(vm_module, memory_args),

        Instruction::I32Store(memory_args) => ins_memory::i32_store(vm_module, memory_args),
        Instruction::I32Store16(memory_args) => ins_memory::i32_store_16(vm_module, memory_args),
        Instruction::I32Store8(memory_args) => ins_memory::i32_store_8(vm_module, memory_args),
        Instruction::I64Store(memory_args) => ins_memory::i64_store(vm_module, memory_args),
        Instruction::I64Store32(memory_args) => ins_memory::i64_store_32(vm_module, memory_args),
        Instruction::I64Store16(memory_args) => ins_memory::i64_store_16(vm_module, memory_args),
        Instruction::I64Store8(memory_args) => ins_memory::i64_store_8(vm_module, memory_args),

        Instruction::F32Store(memory_args) => ins_memory::f32_store(vm_module, memory_args),
        Instruction::F64Store(memory_args) => ins_memory::f64_store(vm_module, memory_args),

        // 常量指令
        Instruction::I32Const(value) => ins_const::i32_const(vm_module, *value),
        Instruction::I64Const(value) => ins_const::i64_const(vm_module, *value),
        Instruction::F32Const(value) => ins_const::f32_const(vm_module, *value),
        Instruction::F64Const(value) => ins_const::f64_const(vm_module, *value),

        // 零值测试指令
        Instruction::I32Eqz => ins_numeric_eqz::i32_eqz(vm_module),
        Instruction::I64Eqz => ins_numeric_eqz::i64_eqz(vm_module),

        // 数值比较指令
        Instruction::I32Eq => ins_numeric_comparsion::i32_eq(vm_module),
        Instruction::I32Ne => ins_numeric_comparsion::i32_ne(vm_module),
        Instruction::I32LtS => ins_numeric_comparsion::i32_lt_s(vm_module),
        Instruction::I32LtU => ins_numeric_comparsion::i32_lt_u(vm_module),
        Instruction::I32GtS => ins_numeric_comparsion::i32_gt_s(vm_module),
        Instruction::I32GtU => ins_numeric_comparsion::i32_gt_u(vm_module),
        Instruction::I32LeS => ins_numeric_comparsion::i32_le_s(vm_module),
        Instruction::I32LeU => ins_numeric_comparsion::i32_le_u(vm_module),
        Instruction::I32GeS => ins_numeric_comparsion::i32_ge_s(vm_module),
        Instruction::I32GeU => ins_numeric_comparsion::i32_ge_u(vm_module),

        Instruction::I64Eq => ins_numeric_comparsion::i64_eq(vm_module),
        Instruction::I64Ne => ins_numeric_comparsion::i64_ne(vm_module),
        Instruction::I64LtS => ins_numeric_comparsion::i64_lt_s(vm_module),
        Instruction::I64LtU => ins_numeric_comparsion::i64_lt_u(vm_module),
        Instruction::I64GtS => ins_numeric_comparsion::i64_gt_s(vm_module),
        Instruction::I64GtU => ins_numeric_comparsion::i64_gt_u(vm_module),
        Instruction::I64LeS => ins_numeric_comparsion::i64_le_s(vm_module),
        Instruction::I64LeU => ins_numeric_comparsion::i64_le_u(vm_module),
        Instruction::I64GeS => ins_numeric_comparsion::i64_ge_s(vm_module),
        Instruction::I64GeU => ins_numeric_comparsion::i64_ge_u(vm_module),

        Instruction::F32Eq => ins_numeric_comparsion::f32_eq(vm_module),
        Instruction::F32Ne => ins_numeric_comparsion::f32_ne(vm_module),
        Instruction::F32Lt => ins_numeric_comparsion::f32_lt(vm_module),
        Instruction::F32Gt => ins_numeric_comparsion::f32_gt(vm_module),
        Instruction::F32Le => ins_numeric_comparsion::f32_le(vm_module),
        Instruction::F32Ge => ins_numeric_comparsion::f32_ge(vm_module),

        Instruction::F64Eq => ins_numeric_comparsion::f64_eq(vm_module),
        Instruction::F64Ne => ins_numeric_comparsion::f64_ne(vm_module),
        Instruction::F64Lt => ins_numeric_comparsion::f64_lt(vm_module),
        Instruction::F64Gt => ins_numeric_comparsion::f64_gt(vm_module),
        Instruction::F64Le => ins_numeric_comparsion::f64_le(vm_module),
        Instruction::F64Ge => ins_numeric_comparsion::f64_ge(vm_module),

        // 一元运算
        Instruction::I32Clz => ins_numeric_unary::i32_clz(vm_module),
        Instruction::I32Ctz => ins_numeric_unary::i32_ctz(vm_module),
        Instruction::I32PopCnt => ins_numeric_unary::i32_popcnt(vm_module),

        Instruction::I64Clz => ins_numeric_unary::i64_clz(vm_module),
        Instruction::I64Ctz => ins_numeric_unary::i64_ctz(vm_module),
        Instruction::I64PopCnt => ins_numeric_unary::i64_popcnt(vm_module),

        Instruction::F32Abs => ins_numeric_unary::f32_abs(vm_module),
        Instruction::F32Neg => ins_numeric_unary::f32_neg(vm_module),
        Instruction::F32Ceil => ins_numeric_unary::f32_ceil(vm_module),
        Instruction::F32Floor => ins_numeric_unary::f32_floor(vm_module),
        Instruction::F32Trunc => ins_numeric_unary::f32_trunc(vm_module),
        Instruction::F32Nearest => ins_numeric_unary::f32_nearest(vm_module),
        Instruction::F32Sqrt => ins_numeric_unary::f32_sqrt(vm_module),

        Instruction::F64Abs => ins_numeric_unary::f64_abs(vm_module),
        Instruction::F64Neg => ins_numeric_unary::f64_neg(vm_module),
        Instruction::F64Ceil => ins_numeric_unary::f64_ceil(vm_module),
        Instruction::F64Floor => ins_numeric_unary::f64_floor(vm_module),
        Instruction::F64Trunc => ins_numeric_unary::f64_trunc(vm_module),
        Instruction::F64Nearest => ins_numeric_unary::f64_nearest(vm_module),
        Instruction::F64Sqrt => ins_numeric_unary::f64_sqrt(vm_module),

        // 二元运算
        Instruction::I32Add => ins_numeric_binary::i32_add(vm_module),
        Instruction::I32Sub => ins_numeric_binary::i32_sub(vm_module),
        Instruction::I32Mul => ins_numeric_binary::i32_mul(vm_module),
        Instruction::I32DivS => ins_numeric_binary::i32_div_s(vm_module),
        Instruction::I32DivU => ins_numeric_binary::i32_div_u(vm_module),
        Instruction::I32RemS => ins_numeric_binary::i32_rem_s(vm_module),
        Instruction::I32RemU => ins_numeric_binary::i32_rem_u(vm_module),
        Instruction::I32And => ins_numeric_binary::i32_and(vm_module),
        Instruction::I32Or => ins_numeric_binary::i32_or(vm_module),
        Instruction::I32Xor => ins_numeric_binary::i32_xor(vm_module),
        Instruction::I32Shl => ins_numeric_binary::i32_shl(vm_module),
        Instruction::I32ShrS => ins_numeric_binary::i32_shr_s(vm_module),
        Instruction::I32ShrU => ins_numeric_binary::i32_shr_u(vm_module),
        Instruction::I32Rotl => ins_numeric_binary::i32_rotl(vm_module),
        Instruction::I32Rotr => ins_numeric_binary::i32_rotr(vm_module),

        Instruction::I64Add => ins_numeric_binary::i64_add(vm_module),
        Instruction::I64Sub => ins_numeric_binary::i64_sub(vm_module),
        Instruction::I64Mul => ins_numeric_binary::i64_mul(vm_module),
        Instruction::I64DivS => ins_numeric_binary::i64_div_s(vm_module),
        Instruction::I64DivU => ins_numeric_binary::i64_div_u(vm_module),
        Instruction::I64RemS => ins_numeric_binary::i64_rem_s(vm_module),
        Instruction::I64RemU => ins_numeric_binary::i64_rem_u(vm_module),
        Instruction::I64And => ins_numeric_binary::i64_and(vm_module),
        Instruction::I64Or => ins_numeric_binary::i64_or(vm_module),
        Instruction::I64Xor => ins_numeric_binary::i64_xor(vm_module),
        Instruction::I64Shl => ins_numeric_binary::i64_shl(vm_module),
        Instruction::I64ShrS => ins_numeric_binary::i64_shr_s(vm_module),
        Instruction::I64ShrU => ins_numeric_binary::i64_shr_u(vm_module),
        Instruction::I64Rotl => ins_numeric_binary::i64_rotl(vm_module),
        Instruction::I64Rotr => ins_numeric_binary::i64_rotr(vm_module),

        Instruction::F32Add => ins_numeric_binary::f32_add(vm_module),
        Instruction::F32Sub => ins_numeric_binary::f32_sub(vm_module),
        Instruction::F32Mul => ins_numeric_binary::f32_mul(vm_module),
        Instruction::F32Div => ins_numeric_binary::f32_div(vm_module),
        Instruction::F32Min => ins_numeric_binary::f32_min(vm_module),
        Instruction::F32Max => ins_numeric_binary::f32_max(vm_module),
        Instruction::F32CopySign => ins_numeric_binary::f32_copysign(vm_module),

        Instruction::F64Add => ins_numeric_binary::f64_add(vm_module),
        Instruction::F64Sub => ins_numeric_binary::f64_sub(vm_module),
        Instruction::F64Mul => ins_numeric_binary::f64_mul(vm_module),
        Instruction::F64Div => ins_numeric_binary::f64_div(vm_module),
        Instruction::F64Min => ins_numeric_binary::f64_min(vm_module),
        Instruction::F64Max => ins_numeric_binary::f64_max(vm_module),
        Instruction::F64CopySign => ins_numeric_binary::f64_copysign(vm_module),

        // 类型转换指令
        Instruction::I32WrapI64 => ins_numeric_convert::i32_wrap_i64(vm_module),

        Instruction::I32Extend8S => ins_numeric_convert::i32_extend8_s(vm_module),
        Instruction::I32Extend16S => ins_numeric_convert::i32_extend16_s(vm_module),
        Instruction::I64ExtendI32S => ins_numeric_convert::i64_extend_i32_s(vm_module),
        Instruction::I64ExtendI32U => ins_numeric_convert::i64_extend_i32_u(vm_module),
        Instruction::I64Extend8S => ins_numeric_convert::i64_extend8_s(vm_module),
        Instruction::I64Extend16S => ins_numeric_convert::i64_extend16_s(vm_module),
        Instruction::I64Extend32S => ins_numeric_convert::i64_extend32_s(vm_module),

        Instruction::I32TruncF32S => ins_numeric_convert::i32_trunc_f32_s(vm_module),
        Instruction::I32TruncF32U => ins_numeric_convert::i32_trunc_f32_u(vm_module),
        Instruction::I64TruncF32S => ins_numeric_convert::i64_trunc_f32_s(vm_module),
        Instruction::I64TruncF32U => ins_numeric_convert::i64_trunc_f32_u(vm_module),
        Instruction::I32TruncF64S => ins_numeric_convert::i32_trunc_f64_s(vm_module),
        Instruction::I32TruncF64U => ins_numeric_convert::i32_trunc_f64_u(vm_module),
        Instruction::I64TruncF64S => ins_numeric_convert::i64_trunc_f64_s(vm_module),
        Instruction::I64TruncF64U => ins_numeric_convert::i64_trunc_f64_u(vm_module),

        Instruction::TruncSat(kind) => ins_numeric_convert::trunc_sat(vm_module, *kind),

        Instruction::F32ConvertI32S => ins_numeric_convert::f32_convert_i32_s(vm_module),
        Instruction::F32ConvertI32U => ins_numeric_convert::f32_convert_i32_u(vm_module),
        Instruction::F64ConvertI32S => ins_numeric_convert::f64_convert_i32_s(vm_module),
        Instruction::F64ConvertI32U => ins_numeric_convert::f64_convert_i32_u(vm_module),
        Instruction::F32ConvertI64S => ins_numeric_convert::f32_convert_i64_s(vm_module),
        Instruction::F32ConvertI64U => ins_numeric_convert::f32_convert_i64_u(vm_module),
        Instruction::F64ConvertI64S => ins_numeric_convert::f64_convert_i64_s(vm_module),
        Instruction::F64ConvertI64U => ins_numeric_convert::f64_convert_i64_u(vm_module),

        Instruction::F32DemoteF64 => ins_numeric_convert::f32_demote_f64_s(vm_module),
        Instruction::F64PromoteF32 => ins_numeric_convert::f64_promote_f32(vm_module),

        Instruction::I32ReinterpretF32 => ins_numeric_convert::i32_reinterpret_f32(vm_module),
        Instruction::I64ReinterpretF64 => ins_numeric_convert::i64_reinterpret_f64(vm_module),
        Instruction::F32ReinterpretI32 => ins_numeric_convert::f32_reinterpret_i32(vm_module),
        Instruction::F64ReinterpretI64 => ins_numeric_convert::f64_reinterpret_i64(vm_module),
    }
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
