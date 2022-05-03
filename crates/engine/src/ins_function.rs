// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_parser::{
    ast::{FunctionType, LocalGroup},
    instruction::Instruction,
};

use crate::vm_module::VMModule;

pub fn call_internal_function(
    vm_module: &mut VMModule,
    function_type: &FunctionType,
    local_groups: &Vec<LocalGroup>,
    expression: &Vec<Instruction>,
) {
    todo!()
}
