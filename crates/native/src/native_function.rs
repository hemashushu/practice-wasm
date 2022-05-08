// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use anvm_ast::{ast::FunctionType, types::Value};
use anvm_engine::object::{EngineError, Function};

use crate::interface::NativeError;

pub struct NativeFunction {
    function_type: Rc<FunctionType>,
    function_index: usize,
    internal_function: InternalFunction,
    name: String,
}

pub type InternalFunction = fn(&[Value]) -> Result<Vec<Value>, NativeError>;

impl Function for NativeFunction {
    fn eval(&self, args: &[Value]) -> Result<Vec<Value>, EngineError> {
        match (self.internal_function)(args) {
            Ok(result) => Ok(result),
            Err(err) => Err(EngineError::ModuleError(Box::new(err))),
        }
    }

    fn get_function_type(&self) -> Rc<FunctionType> {
        Rc::clone(&self.function_type)
    }

    fn get_index(&self) -> usize {
        self.function_index
    }

    fn get_name(&self) -> Option<String> {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl NativeFunction {
    pub fn new(
        function_type: Rc<FunctionType>,
        function_index: usize,
        internal_function: InternalFunction,
        name: String,
    ) -> Self {
        Self {
            function_type,
            function_index,
            internal_function,
            name,
        }
    }
}
