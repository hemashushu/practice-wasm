// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! # 等零测试指令
//!
//! 从栈顶弹出一个操作数，判断是否为 0，
//! 如果为 0 则压入 1（int32）， 否则压入 0（int32）
//!
//! i32.eqz
//! i64.eqz

use anvm_ast::types::Value;

use crate::{
    error::{make_invalid_operand_data_types_engine_error, EngineError},
    vm::VM,
};

pub fn i32_eqz(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let testing = stack.pop();

    if let Value::I32(value) = testing {
        stack.push_bool(value == 0);
        Ok(())
    } else {
        Err(make_invalid_operand_data_types_engine_error("i32.eqz", "i32"))
    }
}

pub fn i64_eqz(vm: &mut VM) -> Result<(), EngineError> {
    let stack = &mut vm.context.stack;
    let testing = stack.pop();

    if let Value::I64(value) = testing {
        stack.push_bool(value == 0);
        Ok(())
    } else {
        Err(make_invalid_operand_data_types_engine_error("i64.eqz", "i64"))
    }
}
