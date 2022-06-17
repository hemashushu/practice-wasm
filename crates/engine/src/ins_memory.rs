// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 内存指令
//!
//! ## 页面指令
//!
//! - memory.size mem_block_idx:uint32
//!   返回指定内存块的页面数（uint32）
//!
//! - memory.grow mem_block_idx:uint32
//!   增加指定数量的页面数
//!   从操作数栈弹出 uint32 作为增加量
//!   成功则返回旧的页面数量
//!   失败（比如超出限制值的 max）则返回 -1:uint32
//!
//! 内存还有其他几个操作：
//!
//! - The `memory.fill` instruction sets all values in a region to a given byte.
//! - The `memory.copy` instruction copies data from a source memory region to
//!   a possibly overlapping destination region.
//! - The `memory.init` instruction copies data from a passive data segment into a memory.
//! - The `data.drop` instruction prevents further use of a passive data segment.
//!   This instruction is intended to be used as an optimization hint.
//!   After a data segment is dropped its data can no longer be retrieved,
//!   so the memory used by this segment may be freed.
//!
//! https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-instr-memory
//!
//! ## 加载指令
//!
//! 二进制格式
//!
//! i32.load align:uint32 offset:uint32
//!
//! 文本格式
//!
//! (i32.load offset=0 align=2)
//!
//! 对于文本格式，必须先写 offset 再写 align，且可以省略 `align` 值，
//! 对于 i32.load/i32.store，默认对齐 4 个字节
//! 对于 i64.load/i64.store，默认对齐 8 个字节
//!
//! 参数的作用
//!
//! - offset 偏移值
//!   加载（以及存储）指令都会从操作数栈弹出一个 i32 类型的整数，让它与指令的立即数 offset 相加，得到
//!   实际的内存地址，即：有效地址 = offset + popUint32()
//!
//! - align 地址对齐字节数量的对数，表示对齐一个 ”以 2 为底，以 align 为指数“ 的字节数，
//!   比如 align = 1 时，表示对齐 2^1 = 2 个字节
//!   比如 align = 2 时，表示对齐 2^2 = 4 个字节
//!   align 只起提示作用，用于帮助编译器优化机器代码，对实际执行没有影响（对于 wasm 解析器，可以忽略这个值）
//!   文本格式里 `align` 的值就是字节数，比如文本格式的 8 对应二进制格式的 3 (2^3)。
//!
//! 加载过程：
//!
//! 1. 从操作数栈弹出一个 uint32，作为目标地址（address）
//! 2. 计算有效地址
//! 3. 将指定地址内存的数值（多个字节使用小端格式 litte-endian 编码）压入操作数栈
//!
//! 指令列表
//!
//! - i32.load
//! - i32.load16_s
//! - i32.load16_u
//! - i32.load8_s
//! - i32.load8_u
//!
//! - i64.load
//! - i64.load32_s
//! - i64.load32_u
//! - i64.load16_s
//! - i64.load16_u
//! - i64.load8_s
//! - i64.load8_u
//!
//! - f32.load
//! - f64.load
//!
//! ## 存储指令
//!
//! 格式：
//!
//! i32.store align:uint32 offset:uint32
//!
//! 存储过程：
//!
//! 1. 从操作数栈弹出一个操作数，这个操作数将作为被存储的数据（data）
//! 2. 从操作数栈弹出一个 uint32，作为目标地址（addr）
//! 3. 计算有效地址
//! 4. 将 data 写入指定地址的内存
//!
//! 指令列表
//!
//! - i32.store
//! - i32.store_16
//! - i32.store_8
//!
//! - i64.store
//! - i64.store_32
//! - i64.store_16
//! - i64.store_8
//!
//! - f32.store
//! - f64.store

use anvm_ast::{instruction::MemoryArgument, types::Value, types::ValueType};

use crate::{
    error::{EngineError, TypeMismatch, Unsupported, make_operand_data_types_mismatch_engine_error},
    vm::VM,
    vm_memory::VMMemory,
    vm_stack::VMStack,
};

pub fn memory_size(vm: &mut VM, memory_block_index: u32) -> Result<(), EngineError> {
    if memory_block_index != 0 {
        return Err(EngineError::Unsupported(
            Unsupported::UnsupportedMultipleMemoryBlock,
        ));
    }

    let instance_memory_block_index =
        vm.resource.vm_modules[vm.status.vm_module_index].memory_index;
    let memory_block = &mut vm.resource.memory_blocks[instance_memory_block_index];
    let page_count = memory_block.get_page_count();

    let stack = &mut vm.stack;
    stack.push(Value::I32(page_count as i32));

    Ok(())
}

pub fn memory_grow(vm: &mut VM, memory_block_index: u32) -> Result<(), EngineError> {
    if memory_block_index != 0 {
        return Err(EngineError::Unsupported(
            Unsupported::UnsupportedMultipleMemoryBlock,
        ));
    }

    let stack = &mut vm.stack;
    let increase_number = stack.pop();

    let instance_memory_block_index =
        vm.resource.vm_modules[vm.status.vm_module_index].memory_index;
    let memory_block = &mut vm.resource.memory_blocks[instance_memory_block_index];

    if let Value::I32(value) = increase_number {
        let result = memory_block.increase_page(value as u32);
        match result {
            Ok(previous_page_count) => {
                stack.push(Value::I32(previous_page_count as i32));
            }
            _ => {
                stack.push(Value::I32(-1));
            }
        }

        Ok(())
    } else {
        Err(make_operand_data_types_mismatch_engine_error(
            "memory.grow",
            vec![ValueType::I32],
            vec![&increase_number],
        ))
    }
}

/// 计算有效内存地址，即内存读写指令最终所访问内存的实际地址。
///
/// 注意，
/// 因为指令中的 offset 立即数是 u32，而操作数栈弹出的值也是 i32（实际是 u32），
/// 所以有效地址是一个 33 位（u32 + u32）的无符号整数，实际的值有可能会超出了 u32 的范围。
fn get_effective_address(immediate_offset: u32, operand_address: i32) -> usize {
    (immediate_offset + operand_address as u32) as usize
}

fn get_memory_load_access_meterial<'a>(
    vm: &'a mut VM,
    memory_args: &MemoryArgument,
) -> Result<(&'a mut VMMemory, &'a mut VMStack, usize), Value> {
    let stack = &mut vm.stack;
    let instance_memory_block_index =
        vm.resource.vm_modules[vm.status.vm_module_index].memory_index;
    let memory_block = &mut vm.resource.memory_blocks[instance_memory_block_index];

    // MemoryArg 里头的 align 暂时无用
    let immediate_offset = memory_args.offset;
    let address_value = stack.pop();

    if let Value::I32(address) = address_value {
        let effective_address = get_effective_address(immediate_offset, address);
        Ok((memory_block, stack, effective_address))
    } else {
        // 参数错误时，返回参数值
        Err(address_value)
    }
}

fn get_memory_store_access_meterial<'a>(
    vm: &'a mut VM,
    memory_args: &MemoryArgument,
) -> Result<(&'a mut VMMemory, usize, Value), (Value, Value)> {
    let stack = &mut vm.stack;
    let instance_memory_block_index =
        vm.resource.vm_modules[vm.status.vm_module_index].memory_index;
    let memory_block = &mut vm.resource.memory_blocks[instance_memory_block_index];

    // 待储存的数据
    let data_value = stack.pop();

    // MemoryArg 里头的 align 暂时无用
    let immediate_offset = memory_args.offset;
    let address_value = stack.pop();

    if let Value::I32(address) = address_value {
        let effective_address = get_effective_address(immediate_offset, address);
        Ok((memory_block, effective_address, data_value))
    } else {
        // 参数错误时，返回参数值
        Err((data_value, address_value))
    }
}

fn make_memory_load_operand_data_type_mismatch_engine_error(
    instruction_name: &str,
    address_value: &Value,
) -> EngineError {
    EngineError::TypeMismatch(TypeMismatch::OperandDataTypeMismatch(
        instruction_name.to_owned(),
        vec![ValueType::I32],
        vec![address_value.get_type()],
    ))
}

fn make_memory_store_operand_data_type_mismatch_engine_error(
    instruction_name: &str,
    expected_data_type: ValueType,
    data_value: &Value,
) -> EngineError {
    EngineError::TypeMismatch(TypeMismatch::OperandDataTypeMismatch(
        instruction_name.to_owned(),
        vec![expected_data_type, ValueType::I32],
        vec![data_value.get_type(), ValueType::I32],
    ))
}

fn make_memory_store_operand_data_types_mismatch_engine_error(
    instruction_name: &str,
    expected_data_type: ValueType,
    data_value: &Value,
    address_value: &Value,
) -> EngineError {
    EngineError::TypeMismatch(TypeMismatch::OperandDataTypeMismatch(
        instruction_name.to_owned(),
        vec![expected_data_type, ValueType::I32],
        vec![data_value.get_type(), address_value.get_type()],
    ))
}

// i32 load

pub fn i32_load(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i32(address);
            stack.push(Value::I32(value));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i32_load",
            &address_value,
        )),
    }
}

pub fn i32_load16_s(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i16(address);
            stack.push(Value::I32(value as i32));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i32_load16_s",
            &address_value,
        )),
    }
}

pub fn i32_load16_u(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i16(address);
            stack.push(Value::I32((value as u16) as i32));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i32_load16_u",
            &address_value,
        )),
    }
}

pub fn i32_load8_s(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i8(address);
            stack.push(Value::I32(value as i32));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i32_load8_s",
            &address_value,
        )),
    }
}

pub fn i32_load8_u(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i8(address);
            stack.push(Value::I32((value as u8) as i32));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i32_load8_u",
            &address_value,
        )),
    }
}

// i64 load

pub fn i64_load(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i64(address);
            stack.push(Value::I64(value));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i64_load",
            &address_value,
        )),
    }
}

pub fn i64_load32_s(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i32(address);
            stack.push(Value::I64(value as i64));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i64_load32_s",
            &address_value,
        )),
    }
}

pub fn i64_load32_u(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i32(address);
            stack.push(Value::I64((value as u32) as i64));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i64_load32_u",
            &address_value,
        )),
    }
}

pub fn i64_load16_s(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i16(address);
            stack.push(Value::I64(value as i64));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i64_load16_s",
            &address_value,
        )),
    }
}

pub fn i64_load16_u(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i16(address);
            stack.push(Value::I64((value as u16) as i64));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i64_load16_u",
            &address_value,
        )),
    }
}

pub fn i64_load8_s(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i8(address);
            stack.push(Value::I64(value as i64));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i64_load8_s",
            &address_value,
        )),
    }
}

pub fn i64_load8_u(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_i8(address);
            stack.push(Value::I64((value as u8) as i64));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "i64_load8_u",
            &address_value,
        )),
    }
}

// float load

pub fn f32_load(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_f32(address);
            stack.push(Value::F32(value));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "f32_load",
            &address_value,
        )),
    }
}

pub fn f64_load(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_load_access_meterial(vm, memory_args) {
        Ok((memory_block, stack, address)) => {
            let value = memory_block.read_f64(address);
            stack.push(Value::F64(value));
            Ok(())
        }
        Err(address_value) => Err(make_memory_load_operand_data_type_mismatch_engine_error(
            "f64_load",
            &address_value,
        )),
    }
}

// i32 store

pub fn i32_store(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::I32(value) = data_value {
                memory_block.write_i32(address, value);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "i32.store",
                    ValueType::I32,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "i32.store",
                ValueType::I32,
                &data_value,
                &address_value,
            ))
        }
    }
}

pub fn i32_store_16(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::I32(value) = data_value {
                memory_block.write_i16(address, value as i16);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "i32.store_16",
                    ValueType::I32,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "i32.store_16",
                ValueType::I32,
                &data_value,
                &address_value,
            ))
        }
    }
}

pub fn i32_store_8(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::I32(value) = data_value {
                memory_block.write_i8(address, value as i8);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "i32.store_8",
                    ValueType::I32,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "i32.store_8",
                ValueType::I32,
                &data_value,
                &address_value,
            ))
        }
    }
}

pub fn i64_store(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::I64(value) = data_value {
                memory_block.write_i64(address, value);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "i64.store",
                    ValueType::I64,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "i64.store",
                ValueType::I64,
                &data_value,
                &address_value,
            ))
        }
    }
}

pub fn i64_store_32(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::I64(value) = data_value {
                memory_block.write_i32(address, value as i32);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "i64.store_32",
                    ValueType::I64,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "i64.store_32",
                ValueType::I64,
                &data_value,
                &address_value,
            ))
        }
    }
}

pub fn i64_store_16(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::I64(value) = data_value {
                memory_block.write_i16(address, value as i16);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "i64.store_16",
                    ValueType::I64,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "i64.store_16",
                ValueType::I64,
                &data_value,
                &address_value,
            ))
        }
    }
}

pub fn i64_store_8(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::I64(value) = data_value {
                memory_block.write_i8(address, value as i8);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "i64.store_8",
                    ValueType::I64,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "i64.store_8",
                ValueType::I64,
                &data_value,
                &address_value,
            ))
        }
    }
}

// float store

pub fn f32_store(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::F32(value) = data_value {
                memory_block.write_f32(address, value);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "f32.store",
                    ValueType::F32,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "f32.store",
                ValueType::F32,
                &data_value,
                &address_value,
            ))
        }
    }
}

pub fn f64_store(vm: &mut VM, memory_args: &MemoryArgument) -> Result<(), EngineError> {
    match get_memory_store_access_meterial(vm, memory_args) {
        Ok((memory_block, address, data_value)) => {
            if let Value::F64(value) = data_value {
                memory_block.write_f64(address, value);
                Ok(())
            } else {
                Err(make_memory_store_operand_data_type_mismatch_engine_error(
                    "f64.store",
                    ValueType::F64,
                    &data_value,
                ))
            }
        }
        Err((data_value, address_value)) => {
            Err(make_memory_store_operand_data_types_mismatch_engine_error(
                "f64.store",
                ValueType::F64,
                &data_value,
                &address_value,
            ))
        }
    }
}
