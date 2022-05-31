// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::instruction::Instruction;

use crate::{
    error::EngineError,
    ins_const,
    ins_control::{self, ControlResult},
    ins_memory, ins_numeric_binary, ins_numeric_comparsion, ins_numeric_convert, ins_numeric_eqz,
    ins_numeric_unary, ins_parametric, ins_variable,
    object::{self, Control},
    vm::VM,
};

pub fn exec_instruction(
    vm: &mut VM,
    instruction: &object::Instruction,
) -> Result<bool, EngineError> {
    match instruction {
        object::Instruction::Sequence(instruction) => {
            let sequence_result = match instruction {
                // 控制指令
                Instruction::Unreachable => Err(EngineError::InvalidOperation(
                    "unreachable instruction is executed".to_string(),
                )),
                Instruction::Nop => Ok(()), // 无需任何操作

                // 常量指令
                Instruction::I32Const(value) => ins_const::i32_const(vm, *value),
                Instruction::I64Const(value) => ins_const::i64_const(vm, *value),
                Instruction::F32Const(value) => ins_const::f32_const(vm, *value),
                Instruction::F64Const(value) => ins_const::f64_const(vm, *value),

                // 操作数（参数，parametric）指令
                Instruction::Drop => ins_parametric::drop(vm),
                Instruction::Select => ins_parametric::select(vm),

                // 零值测试指令
                Instruction::I32Eqz => ins_numeric_eqz::i32_eqz(vm),
                Instruction::I64Eqz => ins_numeric_eqz::i64_eqz(vm),

                // 数值比较指令
                Instruction::I32Eq => ins_numeric_comparsion::i32_eq(vm),
                Instruction::I32Ne => ins_numeric_comparsion::i32_ne(vm),
                Instruction::I32LtS => ins_numeric_comparsion::i32_lt_s(vm),
                Instruction::I32LtU => ins_numeric_comparsion::i32_lt_u(vm),
                Instruction::I32GtS => ins_numeric_comparsion::i32_gt_s(vm),
                Instruction::I32GtU => ins_numeric_comparsion::i32_gt_u(vm),
                Instruction::I32LeS => ins_numeric_comparsion::i32_le_s(vm),
                Instruction::I32LeU => ins_numeric_comparsion::i32_le_u(vm),
                Instruction::I32GeS => ins_numeric_comparsion::i32_ge_s(vm),
                Instruction::I32GeU => ins_numeric_comparsion::i32_ge_u(vm),

                Instruction::I64Eq => ins_numeric_comparsion::i64_eq(vm),
                Instruction::I64Ne => ins_numeric_comparsion::i64_ne(vm),
                Instruction::I64LtS => ins_numeric_comparsion::i64_lt_s(vm),
                Instruction::I64LtU => ins_numeric_comparsion::i64_lt_u(vm),
                Instruction::I64GtS => ins_numeric_comparsion::i64_gt_s(vm),
                Instruction::I64GtU => ins_numeric_comparsion::i64_gt_u(vm),
                Instruction::I64LeS => ins_numeric_comparsion::i64_le_s(vm),
                Instruction::I64LeU => ins_numeric_comparsion::i64_le_u(vm),
                Instruction::I64GeS => ins_numeric_comparsion::i64_ge_s(vm),
                Instruction::I64GeU => ins_numeric_comparsion::i64_ge_u(vm),

                Instruction::F32Eq => ins_numeric_comparsion::f32_eq(vm),
                Instruction::F32Ne => ins_numeric_comparsion::f32_ne(vm),
                Instruction::F32Lt => ins_numeric_comparsion::f32_lt(vm),
                Instruction::F32Gt => ins_numeric_comparsion::f32_gt(vm),
                Instruction::F32Le => ins_numeric_comparsion::f32_le(vm),
                Instruction::F32Ge => ins_numeric_comparsion::f32_ge(vm),

                Instruction::F64Eq => ins_numeric_comparsion::f64_eq(vm),
                Instruction::F64Ne => ins_numeric_comparsion::f64_ne(vm),
                Instruction::F64Lt => ins_numeric_comparsion::f64_lt(vm),
                Instruction::F64Gt => ins_numeric_comparsion::f64_gt(vm),
                Instruction::F64Le => ins_numeric_comparsion::f64_le(vm),
                Instruction::F64Ge => ins_numeric_comparsion::f64_ge(vm),

                // 一元运算
                Instruction::I32Clz => ins_numeric_unary::i32_clz(vm),
                Instruction::I32Ctz => ins_numeric_unary::i32_ctz(vm),
                Instruction::I32PopCnt => ins_numeric_unary::i32_popcnt(vm),

                Instruction::I64Clz => ins_numeric_unary::i64_clz(vm),
                Instruction::I64Ctz => ins_numeric_unary::i64_ctz(vm),
                Instruction::I64PopCnt => ins_numeric_unary::i64_popcnt(vm),

                Instruction::F32Abs => ins_numeric_unary::f32_abs(vm),
                Instruction::F32Neg => ins_numeric_unary::f32_neg(vm),
                Instruction::F32Ceil => ins_numeric_unary::f32_ceil(vm),
                Instruction::F32Floor => ins_numeric_unary::f32_floor(vm),
                Instruction::F32Trunc => ins_numeric_unary::f32_trunc(vm),
                Instruction::F32Nearest => ins_numeric_unary::f32_nearest(vm),
                Instruction::F32Sqrt => ins_numeric_unary::f32_sqrt(vm),

                Instruction::F64Abs => ins_numeric_unary::f64_abs(vm),
                Instruction::F64Neg => ins_numeric_unary::f64_neg(vm),
                Instruction::F64Ceil => ins_numeric_unary::f64_ceil(vm),
                Instruction::F64Floor => ins_numeric_unary::f64_floor(vm),
                Instruction::F64Trunc => ins_numeric_unary::f64_trunc(vm),
                Instruction::F64Nearest => ins_numeric_unary::f64_nearest(vm),
                Instruction::F64Sqrt => ins_numeric_unary::f64_sqrt(vm),

                // 二元运算
                Instruction::I32Add => ins_numeric_binary::i32_add(vm),
                Instruction::I32Sub => ins_numeric_binary::i32_sub(vm),
                Instruction::I32Mul => ins_numeric_binary::i32_mul(vm),
                Instruction::I32DivS => ins_numeric_binary::i32_div_s(vm),
                Instruction::I32DivU => ins_numeric_binary::i32_div_u(vm),
                Instruction::I32RemS => ins_numeric_binary::i32_rem_s(vm),
                Instruction::I32RemU => ins_numeric_binary::i32_rem_u(vm),
                Instruction::I32And => ins_numeric_binary::i32_and(vm),
                Instruction::I32Or => ins_numeric_binary::i32_or(vm),
                Instruction::I32Xor => ins_numeric_binary::i32_xor(vm),
                Instruction::I32Shl => ins_numeric_binary::i32_shl(vm),
                Instruction::I32ShrS => ins_numeric_binary::i32_shr_s(vm),
                Instruction::I32ShrU => ins_numeric_binary::i32_shr_u(vm),
                Instruction::I32Rotl => ins_numeric_binary::i32_rotl(vm),
                Instruction::I32Rotr => ins_numeric_binary::i32_rotr(vm),

                Instruction::I64Add => ins_numeric_binary::i64_add(vm),
                Instruction::I64Sub => ins_numeric_binary::i64_sub(vm),
                Instruction::I64Mul => ins_numeric_binary::i64_mul(vm),
                Instruction::I64DivS => ins_numeric_binary::i64_div_s(vm),
                Instruction::I64DivU => ins_numeric_binary::i64_div_u(vm),
                Instruction::I64RemS => ins_numeric_binary::i64_rem_s(vm),
                Instruction::I64RemU => ins_numeric_binary::i64_rem_u(vm),
                Instruction::I64And => ins_numeric_binary::i64_and(vm),
                Instruction::I64Or => ins_numeric_binary::i64_or(vm),
                Instruction::I64Xor => ins_numeric_binary::i64_xor(vm),
                Instruction::I64Shl => ins_numeric_binary::i64_shl(vm),
                Instruction::I64ShrS => ins_numeric_binary::i64_shr_s(vm),
                Instruction::I64ShrU => ins_numeric_binary::i64_shr_u(vm),
                Instruction::I64Rotl => ins_numeric_binary::i64_rotl(vm),
                Instruction::I64Rotr => ins_numeric_binary::i64_rotr(vm),

                Instruction::F32Add => ins_numeric_binary::f32_add(vm),
                Instruction::F32Sub => ins_numeric_binary::f32_sub(vm),
                Instruction::F32Mul => ins_numeric_binary::f32_mul(vm),
                Instruction::F32Div => ins_numeric_binary::f32_div(vm),
                Instruction::F32Min => ins_numeric_binary::f32_min(vm),
                Instruction::F32Max => ins_numeric_binary::f32_max(vm),
                Instruction::F32CopySign => ins_numeric_binary::f32_copysign(vm),

                Instruction::F64Add => ins_numeric_binary::f64_add(vm),
                Instruction::F64Sub => ins_numeric_binary::f64_sub(vm),
                Instruction::F64Mul => ins_numeric_binary::f64_mul(vm),
                Instruction::F64Div => ins_numeric_binary::f64_div(vm),
                Instruction::F64Min => ins_numeric_binary::f64_min(vm),
                Instruction::F64Max => ins_numeric_binary::f64_max(vm),
                Instruction::F64CopySign => ins_numeric_binary::f64_copysign(vm),

                // 类型转换指令
                Instruction::I32WrapI64 => ins_numeric_convert::i32_wrap_i64(vm),

                Instruction::I32Extend8S => ins_numeric_convert::i32_extend8_s(vm),
                Instruction::I32Extend16S => ins_numeric_convert::i32_extend16_s(vm),
                Instruction::I64ExtendI32S => ins_numeric_convert::i64_extend_i32_s(vm),
                Instruction::I64ExtendI32U => ins_numeric_convert::i64_extend_i32_u(vm),
                Instruction::I64Extend8S => ins_numeric_convert::i64_extend8_s(vm),
                Instruction::I64Extend16S => ins_numeric_convert::i64_extend16_s(vm),
                Instruction::I64Extend32S => ins_numeric_convert::i64_extend32_s(vm),

                Instruction::I32TruncF32S => ins_numeric_convert::i32_trunc_f32_s(vm),
                Instruction::I32TruncF32U => ins_numeric_convert::i32_trunc_f32_u(vm),
                Instruction::I64TruncF32S => ins_numeric_convert::i64_trunc_f32_s(vm),
                Instruction::I64TruncF32U => ins_numeric_convert::i64_trunc_f32_u(vm),
                Instruction::I32TruncF64S => ins_numeric_convert::i32_trunc_f64_s(vm),
                Instruction::I32TruncF64U => ins_numeric_convert::i32_trunc_f64_u(vm),
                Instruction::I64TruncF64S => ins_numeric_convert::i64_trunc_f64_s(vm),
                Instruction::I64TruncF64U => ins_numeric_convert::i64_trunc_f64_u(vm),

                Instruction::I32TruncSatF32S => todo!(),
                Instruction::I32TruncSatF32U => todo!(),
                Instruction::I32TruncSatF64S => todo!(),
                Instruction::I32TruncSatF64U => todo!(),

                Instruction::I64TruncSatF32S => todo!(),
                Instruction::I64TruncSatF32U => todo!(),
                Instruction::I64TruncSatF64S => todo!(),
                Instruction::I64TruncSatF64U => todo!(),

                Instruction::F32ConvertI32S => ins_numeric_convert::f32_convert_i32_s(vm),
                Instruction::F32ConvertI32U => ins_numeric_convert::f32_convert_i32_u(vm),
                Instruction::F64ConvertI32S => ins_numeric_convert::f64_convert_i32_s(vm),
                Instruction::F64ConvertI32U => ins_numeric_convert::f64_convert_i32_u(vm),
                Instruction::F32ConvertI64S => ins_numeric_convert::f32_convert_i64_s(vm),
                Instruction::F32ConvertI64U => ins_numeric_convert::f32_convert_i64_u(vm),
                Instruction::F64ConvertI64S => ins_numeric_convert::f64_convert_i64_s(vm),
                Instruction::F64ConvertI64U => ins_numeric_convert::f64_convert_i64_u(vm),

                Instruction::F32DemoteF64 => ins_numeric_convert::f32_demote_f64_s(vm),
                Instruction::F64PromoteF32 => ins_numeric_convert::f64_promote_f32(vm),

                Instruction::I32ReinterpretF32 => ins_numeric_convert::i32_reinterpret_f32(vm),
                Instruction::I64ReinterpretF64 => ins_numeric_convert::i64_reinterpret_f64(vm),
                Instruction::F32ReinterpretI32 => ins_numeric_convert::f32_reinterpret_i32(vm),
                Instruction::F64ReinterpretI64 => ins_numeric_convert::f64_reinterpret_i64(vm),

                // 变量指令
                Instruction::LocalGet(index) => ins_variable::local_get(vm, *index),
                Instruction::LocalSet(index) => ins_variable::local_set(vm, *index),
                Instruction::LocalTee(index) => ins_variable::local_tee(vm, *index),
                Instruction::GlobalGet(index) => ins_variable::global_get(vm, *index),
                Instruction::GlobalSet(index) => ins_variable::global_set(vm, *index),

                // 内存指令
                Instruction::MemorySize(memory_block_index) => {
                    ins_memory::memory_size(vm, *memory_block_index)
                }
                Instruction::MemoryGrow(memory_block_index) => {
                    ins_memory::memory_grow(vm, *memory_block_index)
                }

                Instruction::I32Load(memory_args) => ins_memory::i32_load(vm, memory_args),
                Instruction::I32Load16S(memory_args) => ins_memory::i32_load16_s(vm, memory_args),
                Instruction::I32Load16U(memory_args) => ins_memory::i32_load16_u(vm, memory_args),
                Instruction::I32Load8S(memory_args) => ins_memory::i32_load8_s(vm, memory_args),
                Instruction::I32Load8U(memory_args) => ins_memory::i32_load8_u(vm, memory_args),

                Instruction::I64Load(memory_args) => ins_memory::i64_load(vm, memory_args),
                Instruction::I64Load32S(memory_args) => ins_memory::i64_load32_s(vm, memory_args),
                Instruction::I64Load32U(memory_args) => ins_memory::i64_load32_u(vm, memory_args),
                Instruction::I64Load16S(memory_args) => ins_memory::i64_load16_s(vm, memory_args),
                Instruction::I64Load16U(memory_args) => ins_memory::i64_load16_u(vm, memory_args),
                Instruction::I64Load8S(memory_args) => ins_memory::i64_load8_s(vm, memory_args),
                Instruction::I64Load8U(memory_args) => ins_memory::i64_load8_u(vm, memory_args),

                Instruction::F32Load(memory_args) => ins_memory::f32_load(vm, memory_args),
                Instruction::F64Load(memory_args) => ins_memory::f64_load(vm, memory_args),

                Instruction::I32Store(memory_args) => ins_memory::i32_store(vm, memory_args),
                Instruction::I32Store16(memory_args) => ins_memory::i32_store_16(vm, memory_args),
                Instruction::I32Store8(memory_args) => ins_memory::i32_store_8(vm, memory_args),
                Instruction::I64Store(memory_args) => ins_memory::i64_store(vm, memory_args),
                Instruction::I64Store32(memory_args) => ins_memory::i64_store_32(vm, memory_args),
                Instruction::I64Store16(memory_args) => ins_memory::i64_store_16(vm, memory_args),
                Instruction::I64Store8(memory_args) => ins_memory::i64_store_8(vm, memory_args),

                Instruction::F32Store(memory_args) => ins_memory::f32_store(vm, memory_args),
                Instruction::F64Store(memory_args) => ins_memory::f64_store(vm, memory_args),

                // 其他指令已经被替换成 Instruction::Control，所以
                // 程序不应该来到这个分支
                _ => {
                    unreachable!()
                }
            };

            match sequence_result {
                Ok(_) => {
                    vm.status.address += 1;
                    Ok(false)
                }
                Err(e) => Err(e),
            }
        }
        object::Instruction::Control(control) => {
            let control_result = match control {
                Control::Return => ins_control::do_return(vm),
                Control::CallInternal {
                    type_index,
                    function_index,
                    internal_function_index,
                    address,
                } => {
                    let vm_module_index = vm.status.vm_module_index;
                    ins_control::do_call_module_function(
                        vm,
                        vm_module_index,
                        *type_index,
                        *function_index,
                        *internal_function_index,
                        *address,
                    )
                }
                Control::CallExternal {
                    vm_module_index,
                    type_index,
                    function_index,
                    internal_function_index,
                    address,
                } => ins_control::do_call_module_function(
                    vm,
                    *vm_module_index,
                    *type_index,
                    *function_index,
                    *internal_function_index,
                    *address,
                ),
                Control::CallNative {
                    native_module_index,
                    type_index,
                    function_index,
                } => ins_control::do_call_native_function(
                    vm,
                    *native_module_index,
                    *type_index,
                    *function_index,
                ),
                _ => {
                    todo!()
                }
            };

            match control_result {
                Ok(ControlResult::ProgramEnd) => Ok(true),
                Ok(ControlResult::Sequence) => {
                    vm.status.address += 1;
                    Ok(false)
                }
                Ok(ControlResult::FunctionIn {
                    is_function_call,
                    vm_module_index,
                    function_index,
                    frame_type,
                    address,
                }) => {
                    // 更新虚拟机的 pc 值
                    let status = &mut vm.status;

                    status.vm_module_index = vm_module_index;
                    status.function_index = function_index;
                    status.frame_type = frame_type;
                    status.address = address;

                    Ok(false)
                }
                Ok(ControlResult::FunctionOut {
                    is_function_call,
                    vm_module_index,
                    function_index,
                    frame_type,
                    address,
                }) => {
                    // 更新虚拟机的 pc 值
                    let status = &mut vm.status;

                    status.vm_module_index = vm_module_index;
                    status.function_index = function_index;
                    status.frame_type = frame_type;
                    status.address = address;

                    Ok(false)
                }
                Err(e) => Err(e),
            }
        }
    }
}
