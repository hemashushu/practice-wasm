// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    any::Any,
    cell::RefCell,
    rc::{Rc, Weak},
};

use anvm_ast::{
    ast::{FunctionType, LocalGroup},
    instruction::Instruction,
    types::Value,
};

use crate::{
    ins_function,
    object::{EngineError, Function},
    vm_module::{do_loop, VMModule},
};

pub struct VMFunction {
    pub function_type: Rc<FunctionType>,
    pub function_index: usize,
    pub function_kind: VMFunctionKind,

    name: Option<String>,
}

pub enum VMFunctionKind {
    /// 内部函数
    ///
    /// 当前模块内定义的函数
    Internal {
        local_groups: Vec<LocalGroup>,
        expression: Rc<Vec<Instruction>>,
        vm_module: Weak<RefCell<VMModule>>,
    },

    /// 外部函数
    ///
    /// 即来自其他模块的函数，因为一个模块可以从其他模块导入函数，
    /// 所有在模块的函数列表里，即包含在当前模块
    /// 内部定义的 `内部函数` 函数，也包含从外部导入的 `外部函数`。
    External(Rc<dyn Function>),
}

impl VMFunction {
    pub fn new_internal_function(
        function_type: Rc<FunctionType>,
        function_index: usize,
        name: Option<String>,
        local_groups: Vec<LocalGroup>,
        expression: Rc<Vec<Instruction>>,
        vm_module: Weak<RefCell<VMModule>>,
    ) -> Self {
        VMFunction {
            function_type,
            function_index,
            function_kind: VMFunctionKind::Internal {
                local_groups,
                expression,
                vm_module,
            },
            name,
        }
    }

    pub fn new_external_function(
        function_type: Rc<FunctionType>,
        function_index: usize,
        name: Option<String>,
        rc_function: Rc<dyn Function>,
    ) -> Self {
        VMFunction {
            function_type,
            function_index,
            function_kind: VMFunctionKind::External(rc_function),
            name,
        }
    }
}

impl Function for VMFunction {
    /// 从 vm 外部（即宿主）或者其他模块调用函数
    fn eval(&self, args: &[Value]) -> Result<Vec<Value>, EngineError> {
        // 注意模块内的函数有可能是从外部导入的
        match &self.function_kind {
            VMFunctionKind::Internal {
                local_groups,
                expression,
                vm_module,
            } => {
                let rc_vm_module = match vm_module.upgrade() {
                    Some(rc) => rc,
                    _ => panic!("failed to get the module instance"),
                };

                eval_internal_function(
                    rc_vm_module,
                    &self.function_type,
                    self.function_index,
                    local_groups,
                    expression,
                    args,
                )
            }
            VMFunctionKind::External(r) => {
                // 对于 `外部函数`，使用它自己的 eval() 方法求值，
                // 也就是说它会作为其所在的模块的 `内部函数` 来求值。
                r.as_ref().eval(args)
            }
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 从 vm 外部（即宿主）或者其他模块调用 "（函数所在的）模块内部定义的" 函数
///
/// 从 vm 外部调用模块内部函数时，将入的实参压入操作数栈，
/// 参数列表左边（小索引端）的实参先压入，
/// 对于返回结果，先弹出的数值放置在结果数组的右边（大索引端）。
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
fn eval_internal_function(
    vm_module: Rc<RefCell<VMModule>>,
    function_type: &Rc<FunctionType>,
    function_index: usize,
    local_groups: &Vec<LocalGroup>,
    instructions: &Rc<Vec<Instruction>>,
    args: &[Value],
) -> Result<Vec<Value>, EngineError> {
    if args.len() != function_type.params.len() {
        return Err(EngineError::InvalidOperation(
            "the number of arguments and parameters do not match".to_string(),
        ));
    }

    push_arguments(Rc::clone(&vm_module), args);
    ins_function::call_internal_function(
        Rc::clone(&vm_module),
        function_type,
        function_index,
        local_groups,
        instructions,
    );
    do_loop(Rc::clone(&vm_module))?;
    let result_values = pop_results(Rc::clone(&vm_module), function_type);
    Ok(result_values)
}

fn push_arguments(vm_module: Rc<RefCell<VMModule>>, args: &[Value]) {
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .push_values(args);
}

fn pop_results(vm_module: Rc<RefCell<VMModule>>, function_type: &FunctionType) -> Vec<Value> {
    let count = function_type.results.len();
    vm_module
        .as_ref()
        .borrow_mut()
        .operand_stack
        .pop_values(count)
}
