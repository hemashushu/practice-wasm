// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    any::Any,
    fmt::{Debug, Display},
};

use anvm_ast::{
    instruction,
    types::{Value, ValueType},
};

pub fn make_operand_data_types_mismatch_engine_error(
    instruction_name: &str,
    expected_types: Vec<ValueType>,
    actual_values: Vec<&Value>,
) -> EngineError {
    EngineError::TypeMismatch(TypeMismatch::OperandDataTypeMismatch(
        instruction_name.to_owned(),
        expected_types,
        actual_values
            .iter()
            .map(|v| v.get_type())
            .collect::<Vec<ValueType>>(),
    ))
}

/// 引擎程序自身的错误
///
/// 注意应用程序所产生的错误不属于引擎错误，不过为了便于调试应用程序，
/// 应用程序的模块结构以及指令的错误会经由 EngineError 抛出。
/// 而应用程序的业务逻辑错误则不在此范围。
#[derive(Debug)]
pub enum EngineError {
    OutOfRange(OutOfRange),
    Overflow(Overflow),
    ObjectNotFound(ObjectNotFound),
    Unsupported(Unsupported),
    TypeMismatch(TypeMismatch),
    InvalidOperation(InvalidOperation),
    NativeError(NativeError),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EngineError::OutOfRange(s) => write!(f, "{}", s),
            EngineError::Overflow(s) => write!(f, "{}", s),
            EngineError::ObjectNotFound(s) => write!(f, "{}", s),
            EngineError::Unsupported(s) => write!(f, "{}", s),
            EngineError::TypeMismatch(s) => write!(f, "{}", s),
            EngineError::InvalidOperation(s) => write!(f, "{}", s),
            EngineError::NativeError(s) => write!(f, "{}", s),
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

#[derive(Debug, PartialEq, Clone)]
pub enum Unsupported {
    UnsupportedConstantExpressionInstruction(instruction::Instruction),

    /// 当 vm 支持多表格时，需要移除此异常
    UnsupportedMultipleTable,

    /// 当 vm 支持多内存块时，需要移除此异常
    UnsupportedMultipleMemoryBlock,
}

impl Display for Unsupported {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unsupported::UnsupportedConstantExpressionInstruction(inst) => {
                write!(
                    f,
                    "does not support instruction \"{:?}\" in the constant expression",
                    inst
                )
            }
            Unsupported::UnsupportedMultipleTable => {
                write!(f, "does not support multiple tables")
            }
            Unsupported::UnsupportedMultipleMemoryBlock => {
                write!(f, "does not support multiple memory blocks")
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeMismatch {
    OperandDataTypeMismatch(
        /* instruction_name */ String,
        /* expected_types */ Vec<ValueType>,
        /* actual_types */ Vec<ValueType>,
    ),
    ConstantExpressionValueTypeMismatch(
        /* expected_type */ ValueType,
        /* actual_type */ ValueType,
    ),
    ImportedGlobalVariableTypeMismatch(
        /* module_name */ String,
        /* import_name */ String,
    ),
    ImportedMemoryBlockTypeMismatch(/* module_name */ String, /* import_name */ String),
    ImportedTableTypeMismatch(/* module_name */ String, /* import_name */ String),
    ImportedFunctionTypeMismatch(/* module_name */ String, /* import_name */ String),

    SetGlobalVariableValueTypeMismatch(
        /* vm_module_index */ usize,
        /* global_variable_index */ usize,
        /* expected_type */ ValueType,
        /* actual_type */ ValueType,
    ),
    DynamicCallNativeFunctionTypeMismatch(
        /* native_module_index */ usize,
        /* function_index */ usize,
    ),
    DynamicCallFunctionTypeMismatch(
        /* vm_module_index */ usize,
        /* function_index */ usize,
    ),
    FunctionCallArgumentTypeMismatch {
        vm_module_index: usize,
        function_index: usize,
        parameter_index: usize,
        parameter_type: ValueType,
        value_type: ValueType,
    },
    NativeFunctionCallArgumentTypeMismatch {
        native_module_index: usize,
        function_index: usize,
        parameter_index: usize,
        parameter_type: ValueType,
        value_type: ValueType,
    },
    BlockCallArgumentTypeMismatch {
        vm_module_index: usize,
        function_index: usize,
        block_index: usize,
        parameter_index: usize,
        parameter_type: ValueType,
        value_type: ValueType,
    },
    LoopBlockRecurArgumentTypeMismatch {
        vm_module_index: usize,
        function_index: usize,
        source_block_index: usize,
        relative_depth: usize,
        parameter_index: usize,
        parameter_type: ValueType,
        valuetype: ValueType,
    },

    SelectInstructionConsequentTypeMismatch(
        /* consequent_value_type */ ValueType,
        /* alternate_value_type */ ValueType,
    ),
    FunctionResultTypeMismatch {
        vm_module_index: usize,
        function_index: usize,
        result_index: usize,
        result_type: ValueType,
        value_type: ValueType,
    },
    BlockResultTypeMismatch {
        vm_module_index: usize,
        function_index: usize,
        block_index: usize,
        result_index: usize,
        result_type: ValueType,
        value_type: ValueType,
    },
}

impl Display for TypeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeMismatch::OperandDataTypeMismatch(
                instruction_name,
                expected_types,
                actual_types,
            ) => {
                write!(f,
                    "operand data type for instruction \"{}\" mismatch, expected types: [{}], actual types: [{}]",
                    instruction_name,
                    expected_types.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "),
                    actual_types.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "))
            }
            TypeMismatch::ConstantExpressionValueTypeMismatch(expected_type, actual_type) => {
                write!(f,
                    "constant expression value data type mismatch, expected type: \"{}\", actual type: \"{}\"",
                    expected_type, actual_type)
            }
            TypeMismatch::ImportedGlobalVariableTypeMismatch(module_name, function_name) => {
                write!(
                    f,
                    "imported function \"{}\" (module \"{}\") type does not match",
                    function_name, module_name
                )
            }
            TypeMismatch::ImportedMemoryBlockTypeMismatch(module_name, memory_block_name) => {
                write!(
                    f,
                    "imported memory \"{}\" (module \"{}\") type does not match",
                    memory_block_name, module_name,
                )
            }
            TypeMismatch::ImportedTableTypeMismatch(module_name, table_name) => {
                write!(
                    f,
                    "imported table \"{}\" (module \"{}\") type does not match",
                    table_name, module_name,
                )
            }
            TypeMismatch::ImportedFunctionTypeMismatch(module_name, global_variable_name) => {
                write!(
                    f,
                    "imported global variable \"{}\" (module \"{}\") type does not match",
                    global_variable_name, module_name,
                )
            }
            TypeMismatch::SetGlobalVariableValueTypeMismatch(
                vm_module_index,
                global_variable_index,
                expected_type,
                actual_type,
            ) => {
                write!(f, "the data type of value for global variable #{} (module #{}) does not match, expected type: \"{}\", actual type: \"{}\"",
                    global_variable_index,
                    vm_module_index,
                    expected_type,
                    actual_type)
            }
            TypeMismatch::DynamicCallNativeFunctionTypeMismatch(
                native_module_index,
                function_index,
            ) => {
                write!(
                    f,
                    "failed to call native function #{} (module #{}), the type of function does not match",
                    function_index, native_module_index
                )
            }
            TypeMismatch::DynamicCallFunctionTypeMismatch(vm_module_index, function_index) => {
                write!(
                    f,
                    "failed to call function #{} (module #{}), the type of function does not match",
                    function_index, vm_module_index
                )
            }
            TypeMismatch::FunctionCallArgumentTypeMismatch {
                vm_module_index,
                function_index,
                parameter_index,
                parameter_type,
                value_type,
            } => {
                write!(f,
                    "failed to call function {} (module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                    function_index,
                    vm_module_index,
                    parameter_index,
                    parameter_type,
                    value_type)
            }
            TypeMismatch::NativeFunctionCallArgumentTypeMismatch {
                native_module_index,
                function_index,
                parameter_index,
                parameter_type,
                value_type,
            } => {
                write!(f,
                    "failed to call native function {} (module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                    function_index,
                    native_module_index,
                    parameter_index,
                    parameter_type,
                    value_type)
            }
            TypeMismatch::BlockCallArgumentTypeMismatch {
                vm_module_index,
                function_index,
                block_index,
                parameter_index,
                parameter_type,
                value_type,
            } => {
                write!(f,
                    "failed to enter block {} (function {}, module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                    block_index,
                    function_index,
                    vm_module_index,
                    parameter_index,
                    parameter_type,
                    value_type)
            }
            TypeMismatch::LoopBlockRecurArgumentTypeMismatch {
                vm_module_index,
                function_index,
                source_block_index,
                relative_depth,
                parameter_index,
                parameter_type,
                valuetype,
            } => {
                write!(f,
                    "failed to recur block {} to relative depth {} (function {}, module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                    source_block_index,
                    relative_depth,
                    function_index,
                    vm_module_index,
                    parameter_index,
                    parameter_type,
                    valuetype)
            }
            TypeMismatch::SelectInstructionConsequentTypeMismatch(
                consequent_value_type,
                alternate_value_type,
            ) => {
                write!(f,
                    "the operand data type of the alternate \"{}\" of the instruction \"select\" should be the same as consequent \"{}\"",
                    alternate_value_type, consequent_value_type)
            }
            TypeMismatch::FunctionResultTypeMismatch {
                vm_module_index,
                function_index,
                result_index,
                result_type,
                value_type,
            } => {
                write!(f,
                    "failed to return result from function {} (module {}), The data type of result {} does not match, expected: {}, actual: {}",
                    function_index,
                    vm_module_index,
                    result_index ,
                    result_type,
                    value_type
                )
            }
            TypeMismatch::BlockResultTypeMismatch {
                vm_module_index,
                function_index,
                block_index,
                result_index,
                result_type,
                value_type,
            } => {
                write!(f,
                    "failed to return result from block {} (function {}, module {}), The data type of result {} does not match, expected: {}, actual: {}",
                    block_index,
                    function_index,
                    vm_module_index,
                    result_index,
                    result_type,
                    value_type)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum InvalidOperation {
    ImmutableGlobalVariable(
        /* vm_module_index */ usize,
        /* global_variable_index */ usize,
    ),
    IncorrectFunctionCallArgumentCount {
        vm_module_index: usize,
        function_index: usize,
        parameters_count: usize,
        values_count: usize,
    },
    IncorrectNativeFunctionCallArgumentCount {
        native_module_index: usize,
        function_index: usize,
        parameters_count: usize,
        values_count: usize,
    },
    NotEnoughOperandForNativeFunctionCall {
        native_module_index: usize,
        function_index: usize,
        parameters_count: usize,
        operands_count: usize,
    },
    NotEnoughOperandForFunctionCall {
        vm_module_index: usize,
        function_index: usize,
        parameters_count: usize,
        operands_count: usize,
    },
    NotEnoughOperandForBlockCall {
        vm_module_index: usize,
        function_index: usize,
        block_index: usize,
        parameters_count: usize,
        operands_count: usize,
    },
    NotEnoughOperandForLoopBlockRecur {
        vm_module_index: usize,
        function_index: usize,
        source_block_index: usize,
        relative_depth: usize,
        parameters_count: usize,
        operands_count: usize,
    },
    NotEnoughOperandForFunctionResult {
        vm_module_index: usize,
        function_index: usize,
        results_count: usize,
        operands_count: usize,
    },
    NotEnoughOperandForBlockResult {
        vm_module_index: usize,
        function_index: usize,
        block_index: usize,
        results_count: usize,
        operands_count: usize,
    },
    NotEnoughOperandForFunctionBreakToResult {
        vm_module_index: usize,
        function_index: usize,
        results_count: usize,
        operands_count: usize,
    },
    NotEnoughOperandForBlockBreakToResult {
        vm_module_index: usize,
        function_index: usize,
        source_block_index: usize,
        relative_depth: usize,
        results_count: usize,
        operands_count: usize,
    },
    Unreachable,
}

impl Display for InvalidOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidOperation::ImmutableGlobalVariable(vm_module_index, global_variable_index) => {
                write!(
                    f,
                    "the specified global variable #{} (module #{}) is immutable",
                    vm_module_index, global_variable_index
                )
            }
            InvalidOperation::IncorrectFunctionCallArgumentCount {
                vm_module_index,
                function_index,
                parameters_count,
                values_count,
            } => {
                write!(f,
                    "failed to call function #{} (module #{}). The number of parameters does not match, expected: {}, actual: {}",
                    function_index,
                    vm_module_index,
                    parameters_count,
                    values_count)
            }
            InvalidOperation::IncorrectNativeFunctionCallArgumentCount {
                native_module_index,
                function_index,
                parameters_count,
                values_count,
            } => {
                write!(f,
                    "failed to call native function #{} (module #{}). The number of parameters does not match, expected: {}, actual: {}",
                    function_index,
                    native_module_index,
                    parameters_count,
                    values_count)
            }
            InvalidOperation::NotEnoughOperandForNativeFunctionCall {
                native_module_index,
                function_index,
                parameters_count,
                operands_count,
            } => {
                write!(f,
                    "failed to call function #{} (native module #{}), not enough operands, expected: {}, actual: {}",
                    function_index,
                    native_module_index,
                    parameters_count,
                    operands_count)
            }
            InvalidOperation::NotEnoughOperandForFunctionCall {
                vm_module_index,
                function_index,
                parameters_count,
                operands_count,
            } => {
                write!(f,
                    "failed to call function #{} (module #{}), not enough operands, expected: {}, actual: {}",
                    function_index,
                    vm_module_index,
                    parameters_count,
                    operands_count)
            }
            InvalidOperation::NotEnoughOperandForBlockCall {
                vm_module_index,
                function_index,
                block_index,
                parameters_count,
                operands_count,
            } => {
                write!(f,
                    "failed to call block #{} (function #{}, module #{}), not enough operands, expected: {}, actual: {}",
                    block_index,
                    function_index,
                    vm_module_index,
                    parameters_count,
                    operands_count)
            }
            InvalidOperation::NotEnoughOperandForLoopBlockRecur {
                vm_module_index,
                function_index,
                source_block_index,
                relative_depth,
                parameters_count,
                operands_count,
            } => {
                write!(f,
                    "failed to recur block #{} to relative depth #{} (function #{}, module #{}), not enough operands, expected: {}, actual: {}",
                    source_block_index,
                    relative_depth,
                    function_index,
                    vm_module_index,
                    parameters_count,
                    operands_count)
            }
            InvalidOperation::NotEnoughOperandForFunctionResult {
                vm_module_index,
                function_index,
                results_count,
                operands_count,
            } => {
                write!(f,
                    "failed to return result from function #{} (module #{}), not enough operands, expected: {}, actual: {}",
                    function_index,
                    vm_module_index,
                    results_count,
                    operands_count)
            }
            InvalidOperation::NotEnoughOperandForBlockResult {
                vm_module_index,
                function_index,
                block_index,
                results_count,
                operands_count,
            } => {
                write!(f,
                    "failed to return result from block #{} (function #{}, module #{}), not enough operands, expected: {}, actual: {}",
                    block_index,
                    function_index,
                    vm_module_index,
                    results_count,
                    operands_count)
            }
            InvalidOperation::NotEnoughOperandForFunctionBreakToResult {
                vm_module_index,
                function_index,
                results_count,
                operands_count,
            } => {
                write!(f,
                    "failed to break function #{} to result (module #{}), not enough operands, expected: {}, actual: {}",
                    function_index,
                    vm_module_index,
                    results_count,
                    operands_count)
            }
            InvalidOperation::NotEnoughOperandForBlockBreakToResult {
                vm_module_index,
                function_index,
                source_block_index,
                relative_depth,
                results_count,
                operands_count,
            } => {
                write!(f,
                    "failed to break block #{} to relative depth {} block (function #{}, module #{}), not enough operands, expected: {}, actual: {}",
                    source_block_index,
                    relative_depth,
                    function_index,
                    vm_module_index,
                    results_count,
                    operands_count)
            }
            InvalidOperation::Unreachable => write!(f, "instruction \"unreachable\" is executed"),
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
