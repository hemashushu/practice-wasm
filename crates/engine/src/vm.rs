// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::{
    instruction::{self, BlockType},
    types::{check_value_types, Value, ValueType, ValueTypeCheckError},
};

use crate::{
    error::EngineError,
    interpreter,
    native_module::NativeModule,
    object::{FunctionItem, Instruction},
    vm_global_variable::VMGlobalVariable,
    vm_memory::VMMemory,
    vm_module::VMModule,
    vm_stack::VMStack,
    vm_table::VMTable,
};

/// VM 的当前状态信息
///
/// VM 的状态信息主要有两个：
///
/// - 当前栈帧的位置信息（fp, lp, bp）
/// - 当前指令的位置信息（即 pc，这里由 vm_module_index，function_index，program_counter 3 个变量组成）
///
/// 当创建一个新的栈帧时，VM 的状态会保存到栈帧的信息段里。
/// 当弹出一个栈帧时，栈帧的信息段里的数据会恢复到 VM 的状态。
pub struct Status {
    /// 当前栈帧的开始地址
    ///
    /// 对于调用帧来说，它也是第 0 个实参的地址
    pub frame_pointer: usize,

    /// 局部变量段的开始地址
    ///
    /// 也就是当前调用帧的地址
    /// 因为 WebAssembly 的流程控制结构块（比如 block/loop/if）跟函数调用
    /// 的工作方式非常类似，所以无论是函数调用调用还是进入一个结构块，
    /// 当前的解析器都会创建一个新的栈帧。
    ///
    /// 但结构块没有自己的局部变量，它的局部变量来自最近一次函数调用所创建的帧，
    /// - 当新的栈帧是因为函数调用而创建时，local_pointer 和 frame_pointer
    ///   的值是相等的，
    /// - 当新的栈帧是由进入结构块而创建时，则 local_pointer
    ///   保持跟上一个栈帧的 local_pointer 相同。
    pub local_pointer: usize,

    /// 信息段开始的地址
    ///
    /// 当前栈帧开始的一段是函数调用的实参以及局部变量，然后
    /// 是一段固定大小的关于上一帧的信息，诸如上一个栈帧的
    /// fp, lp, bp, ra (包括 vm module index, function index, address) 等信息，
    /// 最后才是一段运算操作数。
    ///
    /// base_pointer 记录局部变量列表的结束位置，从而可以让
    /// 栈维护程序较方便地读取信息段数据。
    ///
    /// `base_pointer = frame_pointer + 局部变量数量`
    pub base_pointer: usize,

    /// 当前指令所在的模块，指令执行完毕之后，则是下一个待执行指令所在的模块
    ///
    /// vm_module_index、function_index 以及 address 3 个数值
    /// 共同组成了传统体系结构当中的 program counter (pc) 值，用于指示下一条指令的位置（地址）。
    ///
    /// 注意：
    /// 如果一个本地函数被调用，vm 的 pc 并不会进入本地模块的本地函数，也不会创建调用帧，而是
    /// 直接把实参从栈里弹出，然后由宿主调用本地函数，再将返回值压入栈，然后 pc 值移动到调用指令
    /// 的下一条指令。
    ///
    /// 比如:
    ///
    /// ```wat
    /// 0001 i32.const 10
    /// 0002 call 0       ;; 假设索引 0 函数为本地函数
    /// 0003 drop
    /// ```
    ///
    /// 如果当前的 pc 是 `0002`，则执行完当前指令后，pc 值是 `0003`，而不像
    /// 普通函数那样，pc 值会变成目标函数的首个指令的位置。
    pub vm_module_index: usize,

    /// 当前指令所在的函数，指令执行完毕之后，则是下一个待执行指令所在的函数
    pub function_index: usize,

    /// 当前函数或者结构块的（签名）类型
    ///
    /// 一般函数的（签名）类型可以通过 function_index 间接找到，不过由于当前 VM 使用了跟执行 `一般函数`
    /// 的方式执行结构块，所以当前栈帧还有可能是结构块，所以需要额外一个字段用于记录当前栈帧的 "函数" 类型。
    pub frame_type: BlockType,

    /// 当前指令在指令序列里所处的位置，指令执行完毕之后，则是下一个待执行指令的位置
    pub address: usize,
}

// 设定一个特殊的初始值，表示 vm 尚未开始任何函数调用
pub const INITIAL_FRAME_POINTER: usize = (0 - 1) as usize;

impl Status {
    pub fn new() -> Self {
        Self {
            frame_pointer: INITIAL_FRAME_POINTER, // 设定一个特殊的初始值，表示 vm 尚未开始任何函数调用
            local_pointer: INITIAL_FRAME_POINTER,
            base_pointer: INITIAL_FRAME_POINTER,
            vm_module_index: 0,
            function_index: 0,
            frame_type: BlockType::TypeIndex(0),
            address: 0,
        }
    }
}

pub struct Resource {
    pub memory_blocks: Vec<VMMemory>,
    pub tables: Vec<VMTable>,
    pub global_variables: Vec<VMGlobalVariable>,

    pub native_modules: Vec<NativeModule>,
    pub vm_modules: Vec<VMModule>,
}

impl Resource {
    pub fn new(
        // stack: VMStack,
        memory_blocks: Vec<VMMemory>,
        tables: Vec<VMTable>,
        global_variables: Vec<VMGlobalVariable>,
        native_modules: Vec<NativeModule>,
        vm_modules: Vec<VMModule>,
    ) -> Self {
        Self {
            // stack,
            memory_blocks,
            tables,
            global_variables,
            native_modules,
            vm_modules,
        }
    }
}

pub struct VM {
    pub stack: VMStack,
    pub status: Status,
    pub resource: Resource,
}

pub enum CallFunctionResult {
    /// 调用的是普通函数，虚拟机进入待命状态
    ///
    /// 需要执行 recur/recur_without_break/step/step_without_into/step_out 等
    /// 方法执行函数的指令
    ///
    /// 参数是最终结果的数据的数量
    Standby(usize),

    /// 调用的是本地函数，直接得出结果
    Immediate(Vec<Value>),
}

impl VM {
    /// 从 vm 外部（即宿主）调用函数，并进行求值
    /// 直到函数所有指令执行完毕。
    pub fn eval_function_by_index(
        &mut self,
        vm_module_index: usize,
        function_index: usize,
        arguments: &[Value],
    ) -> Result<Vec<Value>, EngineError> {
        let result = self.call_function_by_index(vm_module_index, function_index, arguments)?;

        match result {
            CallFunctionResult::Immediate(values) => Ok(values),
            CallFunctionResult::Standby(result_count) => {
                self.recur_without_break()?;
                Ok(self.pop_results(result_count))
            }
        }
    }

    /// 将指定实参压入栈，并将 pc 的值指向函数的
    /// 第一个指令，但并不会开始执行指令。
    ///
    /// 返回函数返回值的数量
    pub fn call_function_by_index(
        &mut self,
        vm_module_index: usize,
        function_index: usize,
        arguments: &[Value],
    ) -> Result<CallFunctionResult, EngineError> {
        let function_item = {
            let vm_module = &self.resource.vm_modules[vm_module_index];
            let function_item = &vm_module.function_items[function_index];
            function_item.to_owned()
        };

        match function_item {
            FunctionItem::Native {
                native_module_index,
                type_index,
                function_index,
            } => {
                let result = self.call_native_function(
                    native_module_index,
                    type_index,
                    function_index,
                    arguments,
                )?;
                Ok(CallFunctionResult::Immediate(result))
            }
            FunctionItem::External {
                vm_module_index: target_vm_module_index,
                type_index: target_type_index,
                function_index: target_function_index,
                internal_function_index: target_internal_function_index,
                start_address,
                end_address,
            } => {
                let result = self.call_module_function(
                    target_vm_module_index,
                    target_type_index,
                    target_function_index,
                    target_internal_function_index,
                    start_address,
                    arguments,
                )?;
                Ok(CallFunctionResult::Standby(result))
            }
            // FunctionItem::Internal {
            //     type_index,
            //     internal_function_index,
            //     start_address,
            //     end_address,
            // } => {
            //     let result = self.call_module_function(
            //         vm_module_index,
            //         type_index,
            //         function_index,
            //         internal_function_index,
            //         start_address,
            //         arguments,
            //     )?;
            //     Ok(CallFunctionResult::Standby(result))
            // }
        }
    }

    fn call_module_function(
        &mut self,
        vm_module_index: usize,
        type_index: usize,
        function_index: usize,
        internal_function_index: usize,
        address: usize,
        arguments: &[Value],
    ) -> Result<usize, EngineError> {
        let (parameter_types, local_variable_types, result_count) = {
            let vm_module = &self.resource.vm_modules[vm_module_index];
            let function_type = &vm_module.function_types[type_index];
            let local_variable_types =
                &vm_module.internal_function_local_variable_types_list[internal_function_index];
            (
                function_type.params.to_owned(),
                local_variable_types.to_owned(),
                function_type.results.len(),
            )
        };

        let parameter_count = parameter_types.len();

        // 核对实参的数据类型和数量
        match check_value_types(arguments, &parameter_types) {
            Err(ValueTypeCheckError::LengthMismatch) => {
                return Err(EngineError::InvalidOperation(format!(
                    "failed to call function {} (module {}). The number of parameters does not match, expected: {}, actual: {}",
                    function_index, vm_module_index, parameter_count, arguments.len())));
            }
            Err(ValueTypeCheckError::DataTypeMismatch(index)) => {
                return Err(EngineError::InvalidOperation(format!(
                    "failed to call function {} (module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                    function_index, vm_module_index, index + 1,
                    parameter_types[index],
                    arguments[index].get_type())));
            }
            _ => {
                // pass
            }
        }

        // 压入实参
        self.push_arguments(arguments);

        // 压入调用栈
        // 第一个调用栈的 previous_frame_pointer 等信息的值都为 0。
        self.push_call_frame(parameter_count, &local_variable_types, 0);

        // 更新状态，
        let status = &mut self.status;

        status.vm_module_index = vm_module_index;
        status.function_index = function_index;
        status.frame_type = BlockType::TypeIndex(type_index as u32);
        status.address = address;

        // 返回函数返回值的数量
        Ok(result_count)
    }

    fn call_native_function(
        &mut self,
        native_module_index: usize,
        type_index: usize,
        function_index: usize,
        arguments: &[Value],
    ) -> Result<Vec<Value>, EngineError> {
        let (parameter_types, result_types, native_function) = {
            let native_module = &self.resource.native_modules[native_module_index];
            let function_type = &native_module.function_types[type_index];
            let native_function = native_module.native_functions[function_index];

            (
                function_type.params.to_owned(),
                function_type.results.to_owned(),
                native_function,
            )
        };

        let parameter_count = parameter_types.len();

        // 核对实参的数据类型和数量
        match check_value_types(&arguments, &parameter_types) {
            Err(ValueTypeCheckError::LengthMismatch) => {
                return Err(EngineError::InvalidOperation(format!(
                    "failed to call function {} (native module {}). The number of parameters does not match, expected: {}, actual: {}",
                    function_index, native_module_index, parameter_count, arguments.len())));
            }
            Err(ValueTypeCheckError::DataTypeMismatch(index)) => {
                return Err(EngineError::InvalidOperation(format!(
                    "failed to call function {} (native module {}). The data type of parameter {} does not match, expected: {}, actual: {}",
                    function_index, native_module_index, index + 1,
                    parameter_types[index],
                    arguments[index].get_type())));
            }
            _ => {
                // pass
            }
        }

        let result = native_function(&arguments);

        match result {
            Ok(result_values) => Ok(result_values),
            Err(e) => Err(EngineError::NativeError(e)),
        }
    }

    /// 执行剩余的指令，遇到断点时中断
    pub fn recur(&mut self) -> Result<bool, EngineError> {
        todo!()
    }

    /// 执行剩余的指令，无视断点
    pub fn recur_without_break(&mut self) -> Result<(), EngineError> {
        loop {
            let is_program_end = self.step()?;
            if is_program_end {
                break;
            }
        }
        Ok(())
    }

    /// 执行当前一条指令
    ///
    /// 如果遇到普通函数调用，则跟踪进入函数内部。
    ///
    /// 当程序（或者说第一个被调用的函数）的最后一条指令（即 `end 指令`）执行
    /// 之后，函数返回 true，否则返回 false。
    pub fn step(&mut self) -> Result<bool, EngineError> {
        let vm_module_index = self.status.vm_module_index;
        let address = self.status.address;

        let instruction =
            self.resource.vm_modules[vm_module_index].instructions[address].to_owned();
        let is_program_end = interpreter::exec_instruction(self, &instruction)?;

        Ok(is_program_end)
    }

    /// 执行当前一条指令
    ///
    /// 如果遇到普通函数调用，则自动执行该函数的所有指令，直到返回结果位置，并停留在下一条指令。
    /// 也就是说，遇到普通函数调用时，不跟踪进入函数的内部。
    ///
    /// 如果遇到断点，仍然会优先中断
    pub fn step_without_into(&mut self) -> Result<bool, EngineError> {
        todo!()
    }

    /// 执行一系列指令，直到当前函数的所有指令结束为止，
    /// 即把函数的最后一条指令，`end 指令` 执行完毕，并停留在调用该函数的指令的下一条指令。
    ///
    /// 如果遇到断点，仍然会优先中断
    pub fn jump_out(&mut self) -> Result<bool, EngineError> {
        todo!()
    }

    /// 从 vm 外部（即宿主）调用 "模块内部定义的" 函数
    ///
    /// 示例：
    ///
    /// |  START            END       |
    /// |  |                ^         |
    /// |  V                |         |
    /// | (a,b,c)          (x,y)      |
    /// |  | | | push       ^ ^       |
    /// |  V V V            | | pop   |
    /// |
    /// |--- 栈顶。---|   |--- 栈顶。---|
    /// | - c        |   |            |
    /// | - b        |   | - y        |
    /// | - a        |   | - x        |
    /// | - ...      |   | - ...      |
    /// |--- 栈底。---|   |--- 栈底。---|
    /// |                             |
    /// |    internal function        |
    /// |-----------------------------|
    ///
    /// 注：
    /// 从 vm 外部调用模块内部函数时，将实参压入操作数栈，
    /// 参数列表左边（小索引端）的实参先压入，
    fn push_arguments(&mut self, arguments: &[Value]) {
        self.stack.push_values(arguments);
    }

    /// 从 vm 内部返回结果给外部（即宿主）函数调用者
    /// 先弹出的数值放置在结果数组的右边（大索引端）。
    fn pop_results(&mut self, result_count: usize) -> Vec<Value> {
        self.stack.pop_values(result_count)
    }

    /// 压入函数调用帧
    pub fn push_call_frame(
        &mut self,
        parameter_count: usize,
        local_variable_types: &[ValueType],
        return_address: usize,
    ) {
        let status = &mut self.status;

        // 读取旧的 status
        let previous_frame_pointer = status.frame_pointer;
        let previous_local_pointer = status.local_pointer;
        let previous_base_pointer = status.base_pointer;
        let return_vm_module_index = status.vm_module_index;
        let return_function_index = status.function_index;
        let previous_frame_type = status.frame_type.clone();

        let stack = &mut self.stack;
        let previous_stack_pointer = stack.get_size();

        // 栈帧的起始位置为：
        //
        // 原栈顶（sp）- 参数数量
        let frame_pointer = previous_stack_pointer - parameter_count;

        // 对于 call frame，local pointer 的值跟 frame pointer 一致
        let local_pointer = frame_pointer;

        // 信息段的开始位置为：
        //
        // - 原栈顶（sp） + 函数内局部变量（不包括参数）数量
        // - 当前栈帧的开始位置（fp）+ 局部变量（包括参数）数量
        //
        // 两个数值是一样的
        let base_pointer = previous_stack_pointer + local_variable_types.len();

        // 分配局部变量空槽
        for variable_type in local_variable_types {
            match variable_type {
                ValueType::I32 => stack.push(Value::I32(0)),
                ValueType::I64 => stack.push(Value::I64(0)),
                ValueType::F32 => stack.push(Value::F32(0.0)),
                ValueType::F64 => stack.push(Value::F64(0.0)),
            }
        }

        let (frame_type_class, frame_type_value) = convert_from_frame_type(&previous_frame_type);

        // 写信息段
        stack.push(previous_frame_pointer.into());
        stack.push(previous_local_pointer.into());
        stack.push(previous_base_pointer.into());
        stack.push(return_vm_module_index.into());
        stack.push(return_function_index.into());
        stack.push(frame_type_class.into()); // 使用两个 Value::I64 来记录 frame_type
        stack.push(frame_type_value.into());
        stack.push(return_address.into());

        // 更新 status
        status.frame_pointer = frame_pointer;
        status.local_pointer = local_pointer;
        status.base_pointer = base_pointer;
    }

    /// 压入控制块帧
    pub fn push_control_frame(&mut self, parameter_count: usize, return_address: usize) {
        let status = &mut self.status;

        // 读取旧的 status
        let previous_frame_pointer = status.frame_pointer;
        let previous_local_pointer = status.local_pointer;
        let previous_base_pointer = status.base_pointer;
        let return_vm_module_index = status.vm_module_index;
        let return_function_index = status.function_index;
        let previous_frame_type = status.frame_type.clone();

        let stack = &mut self.stack;
        let previous_stack_pointer = stack.get_size();

        // 栈帧的起始位置为：
        //
        // 当前栈顶（sp）- 参数数量
        let frame_pointer = previous_stack_pointer - parameter_count;

        // 对于 control frame，local pointer 的值跟上一次的 local pointer 一致
        let local_pointer = previous_local_pointer;

        // 信息段的开始位置为：
        //
        // - 原栈顶（sp）
        // - 当前栈帧的开始位置（fp）+ 参数数量（结构块没有自己的局部变量）
        //
        // 两个数值是一样的
        let base_pointer = previous_stack_pointer;

        let (frame_type_class, frame_type_value) = convert_from_frame_type(&previous_frame_type);

        // 写信息段
        stack.push(previous_frame_pointer.into());
        stack.push(previous_local_pointer.into());
        stack.push(previous_base_pointer.into());
        stack.push(return_vm_module_index.into());
        stack.push(return_function_index.into());
        stack.push(frame_type_class.into()); // 使用两个 Value::I64 来记录 frame_type
        stack.push(frame_type_value.into());
        stack.push(return_address.into());

        // 更新 status
        status.frame_pointer = frame_pointer;
        status.local_pointer = local_pointer;
        status.base_pointer = base_pointer;
    }

    /// 弹出栈帧
    ///
    /// 此函数不会改变 VM 的 program counter，而是将
    /// 待更新的 pc 值返回。
    pub fn pop_frame(
        &mut self,
        result_count: usize,
    ) -> (
        /* return_vm_module_index */ usize,
        /* return_function_index */ usize,
        /* frame_type */ BlockType,
        /* return_address */ usize,
    ) {
        let status = &mut self.status;

        // 读取当前的 status
        let frame_pointer = status.frame_pointer;
        let local_pointer = status.local_pointer;
        let base_pointer = status.base_pointer;

        let stack = &mut self.stack;

        // 读取信息段
        let previous_frame_pointer: usize = stack.get_value(base_pointer).into();
        let previous_local_pointer: usize = stack.get_value(base_pointer + 1).into();
        let previous_base_pointer: usize = stack.get_value(base_pointer + 2).into();

        let return_vm_module_index: usize = stack.get_value(base_pointer + 3).into();
        let return_function_index: usize = stack.get_value(base_pointer + 4).into();

        // 使用两个 Value::I64 来还原 frame_type
        let frame_type_class: usize = stack.get_value(base_pointer + 5).into();
        let frame_type_value: usize = stack.get_value(base_pointer + 6).into();
        let return_address: usize = stack.get_value(base_pointer + 7).into();

        // 先保存一份返回值
        let results = stack.pop_values(result_count);

        // 删除当前栈帧
        stack.drop_values_at(frame_pointer);

        // 重新压入返回值
        stack.push_values(&results);

        // 更新 status
        status.frame_pointer = previous_frame_pointer;
        status.local_pointer = previous_local_pointer;
        status.base_pointer = previous_base_pointer;

        let frame_type = convert_to_frame_type(frame_type_class, frame_type_value);

        (
            return_vm_module_index,
            return_function_index,
            frame_type,
            return_address,
        )
    }

    /// 对一个常量表达式求值
    /// 目前只支持一个 `t.const 指令` 和一个 `end 指令` 这种常量表达式
    pub fn eval_constant_expression(
        &mut self,
        instructions: &[Instruction],
    ) -> Result<Value, EngineError> {
        VM::get_constant_instruction_value(instructions)
    }

    pub fn get_constant_instruction_value(
        instructions: &[Instruction],
    ) -> Result<Value, EngineError> {
        if let Some(first_instruction) = instructions.first() {
            let value = match first_instruction {
                Instruction::Sequence(instruction::Instruction::I32Const(v)) => Value::I32(*v),
                Instruction::Sequence(instruction::Instruction::I64Const(v)) => Value::I64(*v),
                Instruction::Sequence(instruction::Instruction::F32Const(v)) => Value::F32(*v),
                Instruction::Sequence(instruction::Instruction::F64Const(v)) => Value::F64(*v),
                _ => {
                    return Err(EngineError::InvalidOperation(format!(
                        "does not support instruction \"{:?}\" in the constant expression yet",
                        first_instruction
                    )))
                }
            };
            Ok(value)
        } else {
            Err(EngineError::InvalidOperation(
                "the constant expression is empty".to_string(),
            ))
        }
    }
}

fn convert_from_frame_type(
    block_type: &BlockType,
) -> (
    /* frame_type_class */ usize,
    /* frame_type_value */ usize,
) {
    match block_type {
        BlockType::ResultEmpty => {
            (1, 0) // class 1
        }
        BlockType::ResultI32 => {
            (1, 1) // class 1
        }
        BlockType::ResultI64 => {
            (1, 2) // class 1
        }
        BlockType::ResultF32 => {
            (1, 3) // class 1
        }
        BlockType::ResultF64 => {
            (1, 4) // class 1
        }
        BlockType::TypeIndex(value) => {
            (0, *value as usize) // class 0
        }
    }
}

fn convert_to_frame_type(frame_type_class: usize, frame_type_value: usize) -> BlockType {
    match frame_type_class {
        // class 1
        1 => match frame_type_value {
            0 => BlockType::ResultEmpty,
            1 => BlockType::ResultI32,
            2 => BlockType::ResultI64,
            3 => BlockType::ResultF32,
            4 => BlockType::ResultF64,
            _ => unreachable!(),
        },
        // class 0
        0 => BlockType::TypeIndex(frame_type_value as u32),
        _ => unreachable!(),
    }
}
