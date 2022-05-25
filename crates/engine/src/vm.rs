// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::{
    ast::LocalGroup,
    types::{Value, ValueType},
};

use crate::{
    error::EngineError, interpreter, native_module::NativeModule, object::FunctionItem,
    vm_global_variable::VMGlobalVariable, vm_memory::VMMemory, vm_module::VMModule,
    vm_stack::VMStack, vm_table::VMTable,
};

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
    /// fp, lp, bp, ra(包括 vm index, func index, addr) 等信息，
    /// 最后才是一段运算操作数。
    ///
    /// base_pointer 记录局部变量列表的结束位置，从而可以让
    /// 栈维护程序较方便地读取信息段数据。
    ///
    /// `base_pointer = frame_pointer + 局部变量数量`
    pub base_pointer: usize,

    /// 待执行指令所在的模块
    /// vm_module_index、function_index 以及 program_counter 3 个数值
    /// 共同组成了传统体系结构当中的 pc 值，用于指示下一条指令的位置（地址）。
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

    /// 待执行指令所在的函数
    pub function_index: usize,

    /// 待执行指令所处的位置
    pub program_counter: usize,
}

impl Status {
    pub fn new() -> Self {
        Self {
            frame_pointer: 0,
            local_pointer: 0,
            base_pointer: 0,
            vm_module_index: 0,
            function_index: 0,
            program_counter: 0,
        }
    }
}

pub struct Context {
    pub stack: VMStack,
    pub memory_blocks: Vec<VMMemory>,
    pub tables: Vec<VMTable>,
    pub global_variables: Vec<VMGlobalVariable>,

    pub native_modules: Vec<NativeModule>,
    pub vm_modules: Vec<VMModule>,
}

impl Context {
    pub fn new(
        stack: VMStack,
        memory_blocks: Vec<VMMemory>,
        tables: Vec<VMTable>,
        global_variables: Vec<VMGlobalVariable>,
        native_modules: Vec<NativeModule>,
        vm_modules: Vec<VMModule>,
    ) -> Self {
        Self {
            stack,
            memory_blocks,
            tables,
            global_variables,
            native_modules,
            vm_modules,
        }
    }
}

pub struct VM {
    // pub module_map: HashMap<String, Rc<RefCell<dyn Module>>>,
    // pub vm_modules: Vec<VMModule>,
    // pub modules: Vec<Module>,
    pub status: Status,
    pub context: Context,
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
        self.call_function_by_index(vm_module_index, function_index, arguments)?;

        loop {
            let is_program_end = self.next()?;
            if is_program_end {
                break;
            }
        }

        let result_count = self.context.stack.get_size();
        Ok(self.pop_results(result_count))
    }

    /// 将指定实参压入栈，并将 pc 的值指向函数的
    /// 第一个指令，但并不会开始执行指令。
    pub fn call_function_by_index(
        &mut self,
        vm_module_index: usize,
        function_index: usize,
        arguments: &[Value],
    ) -> Result<(), EngineError> {
        let function_item = {
            let vm_module = &self.context.vm_modules[vm_module_index];
            let function_item = &vm_module.function_items[function_index];
            function_item.to_owned()
        };

        match function_item {
            FunctionItem::Native {
                native_module_index,
                type_index,
                function_index,
            } => {
                todo!()
            }
            FunctionItem::External {
                vm_module_index,
                type_index,
                function_index,
                internal_function_index,
                start_index,
                end_index,
            } => {
                todo!()
            }
            FunctionItem::Internal {
                type_index,
                internal_function_index,
                start_index,
                end_index,
            } => {
                // 压入实参
                self.push_arguments(arguments);

                let (argument_count, result_count, local_variable_types) = {
                    let vm_module = &self.context.vm_modules[vm_module_index];
                    let function_type = &vm_module.function_types[type_index];
                    let local_variable_types = &vm_module
                        .internal_function_local_variable_types_list[internal_function_index];
                    (
                        function_type.params.len(),
                        function_type.results.len(),
                        local_variable_types.to_owned(),
                    )
                };

                // 压入调用栈
                // 第一个调用栈的 previous_frame_pointer 等信息的值都为 0。
                // let (frame_pointer, local_pointer, base_pointer) =
                self.push_call_frame(
                    argument_count,
                    result_count,
                    &local_variable_types,
                    // 0, // previous_frame_pointer,
                    // 0, // previous_local_pointer,
                    // 0, // previous_base_pointer,
                    0, // return_vm_module_index,
                    0, // return_function_index,
                    0, // return_address,
                );

                let status = &mut self.status;

                // status.frame_pointer = frame_pointer;
                // status.local_pointer = local_pointer;
                // status.base_pointer = base_pointer;

                status.vm_module_index = vm_module_index;
                status.function_index = function_index;
                status.program_counter = start_index;
            }
        }

        Ok(())
    }

    /// 执行当前的指令（单独一个指令）
    ///
    /// 当程序（或者说第一个被调用的函数）的最后一条指令（即 `end 指令`）执行
    /// 之后，函数返回 true，否则返回 false。
    pub fn next(&mut self) -> Result<bool, EngineError> {
        let vm_module_index = self.status.vm_module_index;
        let addr = self.status.program_counter;

        let instruction = self.context.vm_modules[vm_module_index].instructions[addr].to_owned();

        let is_program_end = interpreter::exec_instruction(self, &instruction)?;

        if !is_program_end {
            self.status.program_counter += 1;
        }

        Ok(is_program_end)
    }

    /// 从 vm 外部（即宿主）调用 "模块内部定义的" 函数
    ///
    /// 从 vm 外部调用模块内部函数时，将实参压入操作数栈，
    /// 参数列表左边（小索引端）的实参先压入，
    ///
    /// 示例：
    ///
    /// |  START            END       |
    /// |  |                ^         |
    /// |  V                |         |
    /// | (a,b,c)          (x,y)      |
    /// |  | | |            ^ ^       |
    /// |  V V V            | |       |
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
    fn push_arguments(&mut self, arguments: &[Value]) {
        self.context.stack.push_values(arguments);
    }

    /// 从 vm 内部返回结果给外部（即宿主）函数调用者
    /// 先弹出的数值放置在结果数组的右边（大索引端）。
    fn pop_results(&mut self, result_count: usize) -> Vec<Value> {
        self.context.stack.pop_values(result_count)
    }

    pub fn push_call_frame(
        &mut self,
        argument_count: usize,
        result_count: usize,
        local_variable_types: &[ValueType],
        //
        // previous_frame_pointer: usize,
        // previous_local_pointer: usize,
        // previous_base_pointer: usize,
        //
        return_vm_module_index: usize,
        return_function_index: usize,
        return_address: usize,
    ) {
        let status = &mut self.status;

        // 读取旧的 status
        let previous_frame_pointer = status.frame_pointer;
        let previous_local_pointer = status.local_pointer;
        let previous_base_pointer = status.base_pointer;

        let stack = &mut self.context.stack;
        let previous_stack_pointer = stack.get_size();

        // 栈帧的起始位置为：
        //
        // 原栈顶（sp）- 参数数量
        let frame_pointer = previous_stack_pointer - argument_count;

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

        // 写信息段
        stack.push(previous_frame_pointer.into());
        stack.push(previous_local_pointer.into());
        stack.push(previous_base_pointer.into());
        stack.push(return_vm_module_index.into());
        stack.push(return_function_index.into());
        stack.push(return_address.into());
        stack.push(result_count.into());

        // 更新 status
        status.frame_pointer = frame_pointer;
        status.local_pointer = local_pointer;
        status.base_pointer = base_pointer;
    }

    pub fn push_control_frame(
        &mut self,
        argument_count: usize,
        result_count: usize,
        //
        // previous_frame_pointer: usize,
        // previous_local_pointer: usize,
        // previous_base_pointer: usize,
        //
        return_address: usize,
    ) {
        let status = &mut self.status;

        // 读取旧的 status
        let previous_frame_pointer = status.frame_pointer;
        let previous_local_pointer = status.local_pointer;
        let previous_base_pointer = status.base_pointer;
        let return_vm_module_index = status.vm_module_index;
        let return_function_index = status.function_index;

        let stack = &mut self.context.stack;
        let previous_stack_pointer = stack.get_size();

        // 栈帧的起始位置为：
        //
        // 当前栈顶（sp）- 参数数量
        let frame_pointer = previous_stack_pointer - argument_count;

        // 对于 control frame，local pointer 的值跟上一次的 local pointer 一致
        let local_pointer = previous_local_pointer;

        // 信息段的开始位置为：
        //
        // - 原栈顶（sp）
        // - 当前栈帧的开始位置（fp）+ 参数数量（结构块没有自己的局部变量）
        //
        // 两个数值是一样的
        let base_pointer = previous_stack_pointer;

        // 写信息段
        stack.push(previous_frame_pointer.into());
        stack.push(previous_local_pointer.into());
        stack.push(previous_base_pointer.into());
        stack.push(return_vm_module_index.into());
        stack.push(return_function_index.into());
        stack.push(return_address.into());
        stack.push(result_count.into());

        // 更新 status
        status.frame_pointer = frame_pointer;
        status.local_pointer = local_pointer;
        status.base_pointer = base_pointer;
    }

    /// 弹出栈帧
    ///
    /// 如果当前栈帧是最后一个栈帧，即意味着首个函数调用（即程序的开始）
    /// 已经结束，也就意味着所有程序已经执行完毕。该函数返回一个 bool 值
    /// 用于表示弹出的是否最后一个栈帧。
    pub fn pop_frame(&mut self) -> bool {
        let status = &mut self.status;

        // 读取当前的 status
        let frame_pointer = status.frame_pointer;
        // let local_pointer = status.local_pointer;
        let base_pointer = status.base_pointer;

        let stack = &mut self.context.stack;

        // 读取信息段
        let previous_frame_pointer: usize = stack.get_value(base_pointer).into();
        let previous_local_pointer: usize = stack.get_value(base_pointer + 1).into();
        let previous_base_pointer: usize = stack.get_value(base_pointer + 2).into();
        let return_vm_module_index: usize = stack.get_value(base_pointer + 3).into();
        let return_function_index: usize = stack.get_value(base_pointer + 4).into();
        let return_address: usize = stack.get_value(base_pointer + 5).into();
        let result_count: usize = stack.get_value(base_pointer + 6).into();

        // 先保存一份返回值
        let result = stack.pop_values(result_count);

        // 删除当前栈帧
        stack.drop_values_at(frame_pointer);

        // 重新压入返回值
        stack.push_values(&result);

        // 更新 status
        status.frame_pointer = previous_frame_pointer;
        status.local_pointer = previous_local_pointer;
        status.base_pointer = previous_base_pointer;

        status.vm_module_index = return_vm_module_index;
        status.function_index = return_function_index;
        status.program_counter = return_address;

        // 如果 previous_frame_pointer 的值等于 0，说明当前栈帧是
        // 首个函数调用的栈帧，也就是说，当这个帧弹出之后，意味着所有程序
        // 已经执行完毕。
        let is_program_end = previous_frame_pointer == 0;
        is_program_end
    }
}
