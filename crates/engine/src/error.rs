// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    any::Any,
    fmt::{format, Debug, Display},
};

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

/// MISMATCH_DYNAMIC_FUNCTION_TYPE
pub fn make_mismatch_dynamic_function_type_engine_error(
    function_index: usize,
    vm_module_index: usize,
) -> EngineError {
    // TODO::
    // 尝试获取函数的名称
    EngineError::InvalidOperation(format!(
        "failed to call dynamic function {} (module {}), the type of function does not match",
        function_index, vm_module_index
    ))
}

#[derive(Debug)]
pub enum EngineError {
    OutOfRange(OutOfRange),
    Overflow(Overflow),
    ObjectNotFound(ObjectNotFound),
    InvalidOperation(String),
    NativeError(NativeError),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::OutOfRange(s) => write!(f, "{}", s),
            EngineError::Overflow(s) => write!(f, "{}", s),
            EngineError::ObjectNotFound(s) => write!(f, "{}", s),
            EngineError::InvalidOperation(s) => write!(f, "invalid operation: {}", s),
            EngineError::NativeError(e) => write!(f, "{}", e.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjectNotFound {
    // 以下几个异常都是在链接模块时触发的
    ModuleNotFound(/* module name */ String),
    FunctionNotFound(
        /* module name */ String,
        /* function name */ String,
    ),
    NativeFunctionNotFound(
        /* module name */ String,
        /* function name */ String,
    ),
    MemoryBlockFound(
        /* module name */ String,
        /* memory block name */ String,
    ),
    TableNotFound(/* module name */ String, /* table name */ String),
    GlobalVariableNotFound(
        /* module name */ String,
        /* global variable name */ String,
    ),

    // 以下几个异常是在运行程序时触发的
    ElementItemNotFound(
        /* vm module index */ usize,
        /* table index */ usize,
        /* element index */ usize,
    ),
    DataItemNotFound(
        /* vm module index */ usize,
        /* memory block index */ usize,
        /* data index */ usize,
    ),
}

impl Display for ObjectNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectNotFound::ModuleNotFound(module_name) => {
                write!(f, "cannot find module \"{}\"", module_name)
            }
            ObjectNotFound::FunctionNotFound(module_name, function_name) => write!(
                f,
                "cannot find function \"{}\" in module \"{}\"",
                function_name, module_name
            ),
            ObjectNotFound::NativeFunctionNotFound(module_name, function_name) => write!(
                f,
                "cannot find function \"{}\" in module \"{}\"",
                function_name, module_name
            ),
            ObjectNotFound::MemoryBlockFound(module_name, memory_block_name) => write!(
                f,
                "cannot find function \"{}\" in module \"{}\"",
                memory_block_name, module_name
            ),
            ObjectNotFound::TableNotFound(module_name, table_name) => write!(
                f,
                "cannot find function \"{}\" in module \"{}\"",
                table_name, module_name
            ),
            ObjectNotFound::GlobalVariableNotFound(module_name, global_variable_name) => write!(
                f,
                "cannot find function \"{}\" in module \"{}\"",
                global_variable_name, module_name
            ),
            ObjectNotFound::ElementItemNotFound(module_index, table_index, element_index) => {
                write!(
                    f,
                    "cannot find the element item #{} in the table #{} of the module #{}",
                    element_index, table_index, module_index
                )
            }
            ObjectNotFound::DataItemNotFound(module_index, memory_block_index, data_index) => {
                write!(
                    f,
                    "cannot find the data item #{} in the memory block #{} of the module #{}",
                    data_index, memory_block_index, module_index
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Overflow {
    MemoryPageExceed(/* actual */ u32, /* max allowed */ u32),
    TableSizeExceed(/* actual */ u32, /* max allowed */ u32),
}

impl Display for Overflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Overflow::MemoryPageExceed(actual, max) => {
                write!(
                    f,
                    "memory pages {} exceeds the limit, maximum allowed {}",
                    actual, max
                )
            }
            Overflow::TableSizeExceed(actual, max) => {
                write!(
                    f,
                    "table size {} exceeds the limit, maximum allowed {}",
                    actual, max
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OutOfRange {
    BlockRelativeDepthOutOfRange(/* relative_depth */ usize, /* max */ usize),
    ElementIndexOutOfRange(/* element index */ usize, /* max */ usize),

    /// 暂时用不上，仅当支持多表格时才有此异常
    TableIndexOutOfRange(/* table index */ usize, /* max */ usize),

    /// 暂时用不上，仅当支持多内存块时才有此异常
    MemoryBlockIndexOutOfRange(/* memory block index */ usize, /* max */ usize),
}

impl Display for OutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutOfRange::BlockRelativeDepthOutOfRange(relative_depth, max) => write!(
                f,
                "the relative depth of the block {} is out of range, maximum {}",
                relative_depth, max
            ),
            OutOfRange::ElementIndexOutOfRange(element_index, max) => write!(
                f,
                "the element index {} is out of range, maximum {}",
                element_index, max
            ),
            OutOfRange::TableIndexOutOfRange(table_index, max) => write!(
                f,
                "the table index {} is out of range, maximum {}",
                table_index, max
            ),
            OutOfRange::MemoryBlockIndexOutOfRange(memory_block_index, max) => write!(
                f,
                "the memory block index {} is out of range, maximum {}",
                memory_block_index, max
            ),
        }
    }
}

#[derive(Debug)]
pub struct NativeError {
    pub internal_error: Box<dyn InternalError>,
    pub module_name: String,
}

pub trait InternalError: Debug + Display {
    fn as_any(&self) -> &dyn Any;
}

impl NativeError {
    pub fn new(internal_error: Box<dyn InternalError>, module_name: &str) -> Self {
        NativeError {
            internal_error: internal_error,
            module_name: module_name.to_owned(),
        }
    }
}

impl Display for NativeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "native module \"{}\" error: {}",
            self.module_name,
            self.internal_error.to_string()
        )
    }
}
