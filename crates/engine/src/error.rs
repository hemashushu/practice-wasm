// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{any::Any, fmt::{Debug, Display}};

/// INVALID_OPERAND_DATA_TYPE
pub fn make_invalid_operand_data_type_engine_error(
    instruction_name: &str,
    data_type_name: &str,
) -> EngineError {
    EngineError::InvalidOperation(format!(
        "operand data type for instruction \"{}\" should be \"{}\"",
        instruction_name, data_type_name
    ))
}

/// INVALID_OPERAND_DATA_TYPES
pub fn make_invalid_operand_data_types_engine_error(
    instruction_name: &str,
    data_type_name: &str,
) -> EngineError {
    EngineError::InvalidOperation(format!(
        "operands data type for instruction \"{}\" should be \"{}\"",
        instruction_name, data_type_name
    ))
}

/// INVALID_OPERAND_DATA_TYPES_2
pub fn make_invalid_operand_data_types_2_engine_error(
    instruction_name: &str,
    data_type_name1: &str,
    data_type_name2: &str,
) -> EngineError {
    EngineError::InvalidOperation(format!(
        "the data type of the two operands of the instruction \"{}\" should be \"{}\" and \"{}\"",
        instruction_name, data_type_name1, data_type_name2
    ))
}

/// INVALID_TABLE_INDEX
pub fn make_invalid_table_index_engine_error() -> EngineError {
    EngineError::InvalidOperation("only table index 0 is supported".to_string())
}

/// INVALID_MEMORY_INDEX
pub fn make_invalid_memory_index_engine_error() -> EngineError {
    EngineError::InvalidOperation("only memory index 0 is supported".to_string())
}

/// MISMATCH_FUNCTION_TYPE
pub fn make_mismatch_function_type_engine_error(
    function_index: usize,
    vm_module_index: usize,
) -> EngineError {
    EngineError::InvalidOperation(format!(
        "failed to call function {} (module {}), the function type does not match",
        function_index, vm_module_index
    ))
}

#[derive(Debug)]
pub enum EngineError {
    OutOfIndex(String),
    Overflow(String),
    ObjectNotFound(String),
    InvalidOperation(String),
    NativeError(NativeError),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::OutOfIndex(s) => write!(f, "out of index: {}", s),
            EngineError::Overflow(s) => write!(f, "overflow: {}", s),
            EngineError::ObjectNotFound(s) => write!(f, "object not found: {}", s),
            EngineError::InvalidOperation(s) => write!(f, "invalid operation: {}", s),
            EngineError::NativeError(e) => write!(f, "{}", e.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct NativeError {
    pub internal_error: Box<dyn InternalError>,
    pub message: String,
}

pub trait InternalError: Debug + Display {
    fn as_any(&self) -> &dyn Any;
}

impl NativeError {
    pub fn new(internal_error: Box<dyn InternalError>, message: &str) -> Self {
        NativeError {
            internal_error: internal_error,
            message: message.to_string(),
        }
    }
}

impl Display for NativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native module error: {}", self.internal_error.to_string())
    }
}
