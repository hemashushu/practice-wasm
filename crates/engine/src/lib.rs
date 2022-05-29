// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod error;

pub mod object;
pub mod native_module;
pub mod vm_stack;
pub mod vm_memory;
pub mod vm_table;
pub mod vm_global_variable;
pub mod linker;
pub mod transformer;
pub mod vm_module;
pub mod interpreter;
pub mod vm;

mod ins_const;
mod ins_parametric;
mod ins_numeric_eqz;
mod ins_numeric_comparsion;
mod ins_numeric_unary;
mod ins_numeric_binary;
mod ins_numeric_convert;
mod ins_variable;
mod ins_memory;
// mod ins_control;

pub mod instance;