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
//! 格式
//!
//! i32.load align:uint32 offset:uint32
//!
//! load 指令有两个立即数：
//!
//! - align 地址对齐字节数量的对数，表示对齐一个 ”以 2 为底，以 align 为指数“ 的字节数，
//!   比如 align = 1 时，表示对齐 2^1 = 2 个字节
//!   比如 align = 2 时，表示对齐 2^2 = 4 个字节
//!   align 只起提示作用，用于帮助编译器优化机器代码，对实际执行没有影响（对于 wasm 解析器，可以忽略这个值）
//!
//! - offset 偏移值
//!   加载（以及存储）指令都会从操作数栈弹出一个 i32 类型的整数，让它与指令的立即数 offset 相加，得到
//!   实际的内存地址，即：有效地址 = offset + popUint32()
//!
//! 加载过程：
//!
//! 1. 从操作数栈弹出一个 uint32，作为目标地址（addr）
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

use std::{cell::RefCell, rc::Rc};

use anvm_ast::{instruction::MemoryArg, types::Value};

use crate::{object::EngineError, vm_module::VMModule};

pub fn memory_size(
    vm_module: Rc<RefCell<VMModule>>,
    memory_block_index: u32,
) -> Result<(), EngineError> {
    if memory_block_index != 0 {
        return Err(EngineError::InvalidOperation(
            "only memory with index value of 0 is allowed".to_string(),
        ));
    }

    let mut module = vm_module.as_ref().borrow_mut();
    let page_count = module.memory.as_ref().borrow().get_page_count();
    module.operand_stack.push(Value::I32(page_count as i32));
    Ok(())
}

pub fn memory_grow(
    vm_module: Rc<RefCell<VMModule>>,
    memory_block_index: u32,
) -> Result<(), EngineError> {
    if memory_block_index != 0 {
        return Err(EngineError::InvalidOperation(
            "only memory with index value of 0 is allowed".to_string(),
        ));
    }

    let mut module = vm_module.as_ref().borrow_mut();
    let increase_number = module.operand_stack.pop();

    if let Value::I32(value) = increase_number {
        // 限制 mut memory 的作用范围
        let result = {
            let mut memory = module.memory.as_ref().borrow_mut();
            memory.incrase_page(value as u32)
        };

        match result {
            Ok(previous_page_count) => {
                module
                    .operand_stack
                    .push(Value::I32(previous_page_count as i32));
            }
            _ => {
                module.operand_stack.push(Value::I32(-1));
            }
        }

        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"memory.grow\" should be \"i32\""
                .to_string(),
        ))
    }
}

/// 注意，因为指令中的 offset 立即数是 uint32，而操作数栈弹出的值也是 uint32，
/// 所以有效地址（uint32 + uint32）是一个 33 位的无符号整数，实际的值有可能会超出了 uint32 的范围。
fn get_effective_address(
    module: &mut VMModule,
    memory_args: &MemoryArg,
) -> Result<usize, EngineError> {
    // MemoryArg 里头的 align 暂时无用
    let offset = memory_args.offset;
    let address = module.operand_stack.pop();

    if let Value::I32(value) = address {
        Ok((offset + value as u32) as usize)
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for memory access instruction should be \"i32\"".to_string(),
        ))
    }
}

// i32 load

pub fn i32_load(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i32(address);
    module.operand_stack.push(Value::I32(value));
    Ok(())
}

pub fn i32_load16_s(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i16(address);
    module.operand_stack.push(Value::I32(value as i32));
    Ok(())
}

pub fn i32_load16_u(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i16(address);
    module.operand_stack.push(Value::I32((value as u16) as i32));
    Ok(())
}

pub fn i32_load8_s(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i8(address);
    module.operand_stack.push(Value::I32(value as i32));
    Ok(())
}

pub fn i32_load8_u(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i8(address);
    module.operand_stack.push(Value::I32((value as u8) as i32));
    Ok(())
}

// i64 load

pub fn i64_load(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i64(address);
    module.operand_stack.push(Value::I64(value));
    Ok(())
}

pub fn i64_load32_s(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i32(address);
    module.operand_stack.push(Value::I64(value as i64));
    Ok(())
}

pub fn i64_load32_u(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i32(address);
    module.operand_stack.push(Value::I64((value as u32) as i64));
    Ok(())
}

pub fn i64_load16_s(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i16(address);
    module.operand_stack.push(Value::I64(value as i64));
    Ok(())
}

pub fn i64_load16_u(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i16(address);
    module.operand_stack.push(Value::I64((value as u16) as i64));
    Ok(())
}

pub fn i64_load8_s(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i8(address);
    module.operand_stack.push(Value::I64(value as i64));
    Ok(())
}

pub fn i64_load8_u(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_i8(address);
    module.operand_stack.push(Value::I64((value as u8) as i64));
    Ok(())
}

// float load

pub fn f32_load(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_f32(address);
    module.operand_stack.push(Value::F32(value));
    Ok(())
}

pub fn f64_load(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let address = get_effective_address(&mut module, memory_args)?;
    let value = module.memory.as_ref().borrow().read_f64(address);
    module.operand_stack.push(Value::F64(value));
    Ok(())
}

// i32 store

pub fn i32_store(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::I32(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_i32(address, value);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"i32.store\" should be \"i32\"".to_string(),
        ))
    }
}

pub fn i32_store_16(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::I32(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_i16(address, value as i16);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"i32.store_16\" should be \"i32\""
                .to_string(),
        ))
    }
}

pub fn i32_store_8(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::I32(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_i8(address, value as i8);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"i32.store_8\" should be \"i32\""
                .to_string(),
        ))
    }
}

pub fn i64_store(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::I64(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_i64(address, value);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"i64.store\" should be \"i64\"".to_string(),
        ))
    }
}

pub fn i64_store_32(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::I64(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_i32(address, value as i32);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"i64.store_32\" should be \"i64\""
                .to_string(),
        ))
    }
}

pub fn i64_store_16(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::I64(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_i16(address, value as i16);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"i64.store_16\" should be \"i64\""
                .to_string(),
        ))
    }
}

pub fn i64_store_8(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::I64(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_i8(address, value as i8);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"i64.store_8\" should be \"i64\""
                .to_string(),
        ))
    }
}

// float store

pub fn f32_store(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::F32(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_f32(address, value);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"f32.store\" should be \"f32\"".to_string(),
        ))
    }
}

pub fn f64_store(
    vm_module: Rc<RefCell<VMModule>>,
    memory_args: &MemoryArg,
) -> Result<(), EngineError> {
    let mut module = vm_module.as_ref().borrow_mut();

    let operand = module.operand_stack.pop();
    let address = get_effective_address(&mut module, memory_args)?;
    if let Value::F64(value) = operand {
        module
            .memory
            .as_ref()
            .borrow_mut()
            .write_f64(address, value);
        Ok(())
    } else {
        Err(EngineError::InvalidOperation(
            "the value type of operand for instruction \"f64.store\" should be \"f64\"".to_string(),
        ))
    }
}
