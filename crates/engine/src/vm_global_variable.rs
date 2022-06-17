// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::{ast::GlobalType, types::Value};

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

    pub fn get_value(&self) -> Value {
        self.value
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), SetGlobalVariableError> {
        if !self.global_type.mutable {
            return Err(SetGlobalVariableError::Immutable);
        }

        if self.value.get_type() != value.get_type() {
            return Err(SetGlobalVariableError::TypeMismatch);
        }

        self.value = value;
        Ok(())
    }

    pub fn get_global_type(&self) -> &GlobalType {
        &self.global_type //.clone()
    }
}

pub enum SetGlobalVariableError {
    Immutable,
    TypeMismatch,
}

#[cfg(test)]
mod tests {
    use anvm_ast::{
        ast::GlobalType,
        types::{Value, ValueType},
    };

    use crate::vm_global_variable::SetGlobalVariableError;

    use super::VMGlobalVariable;

    #[test]
    fn test_get_set() {
        // 创建一个不可变的全局变量
        let mut g0 = VMGlobalVariable::new(
            GlobalType {
                value_type: ValueType::I32,
                mutable: false,
            },
            Value::I32(10),
        );

        assert_eq!(g0.get_value(), Value::I32(10));
        assert!(matches!(
            g0.set_value(Value::I32(20)), // 设置一个不可变的全局变量
            Err(SetGlobalVariableError::Immutable)
        ));

        // 创建一个可变的全局变量
        let mut g1 = VMGlobalVariable::new(
            GlobalType {
                value_type: ValueType::I32,
                mutable: true,
            },
            Value::I32(20),
        );

        assert_eq!(g1.get_value(), Value::I32(20));

        assert!(matches!(g1.set_value(Value::I32(30)), Ok(_)));
        assert_eq!(g1.get_value(), Value::I32(30));

        assert!(matches!(
            g1.set_value(Value::I64(40)), // 设置一个不同数据类型的值
            Err(SetGlobalVariableError::TypeMismatch)
        ))
    }
}
