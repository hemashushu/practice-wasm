// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_parser::{ast::GlobalType, types::Value};

use crate::instance::{EngineError, GlobalVariable};

pub struct VMGlobalVariable {
    /// GlobalType 记录变量的 `数据类型` 以及 `可变性`
    global_type: GlobalType,

    value: Value,
}

impl VMGlobalVariable {
    pub fn new(global_type: GlobalType, value: Value) -> Self {
        VMGlobalVariable {
            global_type: global_type,
            value: value,
        }
    }
}

impl GlobalVariable for VMGlobalVariable {
    fn get_value(&self) -> Value {
        self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), EngineError> {
        if !self.global_type.mutable {
            return Err(EngineError::InvalidOperation(
                "the specified global variable is immutable".to_string(),
            ));
        }

        if self.value.get_type() != value.get_type() {
            return Err(EngineError::InvalidOperation(
                "the type of the new value does not match the specified global variable"
                    .to_string(),
            ));
        }

        self.value = value;
        Ok(())
    }

    fn get_global_type(&self) -> GlobalType {
        self.global_type.clone()
    }
}
