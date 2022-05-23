// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::{ast::LocalGroup, types::Value};

use crate::{
    error::EngineError, native_module::NativeModule, vm_global_variable::VMGlobalVariable,
    vm_memory::VMMemory, vm_module::VMModule, vm_stack::VMStack, vm_table::VMTable,
};

pub struct Status {
    /// 当前栈帧的地址
    /// 对于调用帧来说，它也是第 0 个实参的地址
    pub frame_pointer: usize,

    /// 局部变量段的地址
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

    /// 当前栈帧开始的一段是函数调用的实参以及局部变量，然后
    /// 是一段固定大小的关于上一帧的信息，诸如上一个栈帧的
    /// fp, call_fp, bp, ra 等信息，最后才是一段运算
    /// 操作数。
    /// base_pointer 用于记录局部变量列表的结束位置，从而可以让
    /// 栈维护程序较方便地读取信息段数据。
    ///
    /// `base_pointer = frame_pointer + 局部变量数量`
    pub base_pointer: usize,

    // 当前函数所处的位置
    pub module_index: usize,
    // pub type_index: usize,
    pub function_index: usize,

    // 待执行指令所处的位置
    pub program_counter: usize,
}

impl Status {
    pub fn new() -> Self {
        Self {
            frame_pointer: 0,
            local_pointer: 0,
            base_pointer: 0,
            module_index: 0,
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
    pub fn eval_function_by_index(
        &mut self,
        vm_module_index: usize,
        function_index: usize,
        args: &[Value],
    ) -> Result<Vec<Value>, EngineError> {
        todo!()
    }

    pub fn push_call_frame(
        &mut self,
        argument_count: usize,
        result_count: usize,
        local_groups: &[LocalGroup],
        previous_frame_pointer: usize,
        previous_local_pointer: usize,
        previous_base_pointer: usize,
        //
        return_module_index: usize,
        return_function_index: usize,
        return_address: usize,
    ) -> (
        /* frame_pointer */ usize,
        /* local_pointer */ usize,
        /* base_pointer */ usize,
    ) {
        //
        todo!()
    }

    pub fn push_control_frame(
        &mut self,
        argument_count: usize,
        result_count: usize,
        previous_frame_pointer: usize,
        previous_local_pointer: usize,
        previous_base_pointer: usize,
        //
        // return_module_index: usize,
        // return_function_index: usize,
        return_address: usize,
    ) -> (
        /* frame_pointer */ usize,
        /* local_pointer */ usize,
        /* base_pointer */ usize,
    ) {
        let return_module_index = self.status.module_index;
        let return_function_index = self.status.function_index;

        self.push_call_frame(
            argument_count,
            result_count,
            &vec![],
            previous_frame_pointer,
            previous_local_pointer,
            previous_base_pointer,
            return_module_index,
            return_function_index,
            return_address,
        )
    }
}
