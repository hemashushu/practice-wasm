// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod vm_operand_stack;
mod vm_stack_frame;
mod vm_control_stack;
mod vm_function;
mod vm_table;
pub mod vm_memory;
mod vm_global_variable;
mod vm_module;
mod vm_instruction;

mod ins_const;
mod ins_parametric;
mod ins_numeric_eqz;
mod ins_numeric_comparsion;
mod ins_numeric_unary;
mod ins_numeric_binary;
mod ins_numeric_convert;
mod ins_variable;
mod ins_memory;
mod ins_function;
mod ins_branch;

pub mod object;
pub mod instance;