// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{rc::Rc, cell::RefCell};

use anvm_parser::{
    ast::{self, ExportDescriptor},
    types::Value,
};

use crate::{
    instance::{EngineError, Export, Function, GlobalVariable, Memory, Module, Table},
    vm_control_stack::ControlStack,
    vm_memory::VMMemory,
    vm_operand_stack::OperandStack,
};

pub struct VMModule {
    pub operand_stack: OperandStack,
    pub control_stack: ControlStack,

    pub table: Rc<dyn Table>,
    pub memory: Rc<RefCell<dyn Memory>>,
    pub functions: Vec<Rc<dyn Function>>,
    pub global_variables: Vec<Rc<RefCell<dyn GlobalVariable>>>,

    pub current_call_frame_base_pointer: u32,
    ast_module: ast::Module,
}

impl Module for VMModule {
    fn get_export(&self, name: &str) -> Result<Export, EngineError> {
        let option_export_item = self.ast_module.export_items.iter().find(|export_item| export_item.name == name);

        if let Some(selected_export_item) = option_export_item {
            match selected_export_item.export_descriptor {
                ExportDescriptor::FunctionIndex(index) => {
                    Ok(Export::Function(Rc::clone(&self.functions[index as usize])))
                }
                ExportDescriptor::TableIndex(index) => {
                    // 目前只有一个表格可以导出
                    if index != 0 {
                        Err(EngineError::InvalidOperation("only table with index value 0 is allowed".to_string()))
                    } else {
                        Ok(Export::Table(Rc::clone(&self.table)))
                    }
                }
                ExportDescriptor::MemoryBlockIndex(index) => {
                    // 目前只有一个内存块可以导出
                    if index != 0 {
                        Err(EngineError::InvalidOperation("only memory with index value 0 is allowed".to_string()))
                    } else {
                        Ok(Export::Memory(Rc::clone(&self.memory)))
                    }
                }
                ExportDescriptor::GlobalItemIndex(index) => Ok(Export::GlobalVariable(Rc::clone(
                    &self.global_variables[index as usize],
                ))),
            }
        }else {
            Err(EngineError::ObjectNotFound(
                "cannot find the specified export item".to_string(),
            ))
        }
    }

    fn eval_func(&self, name: &str, args: &[Value]) -> Result<Vec<Value>, EngineError> {
        if let Export::Function(function) = self.get_export(name)? {
            function.as_ref().eval(args)
        } else {
            Err(EngineError::ObjectNotFound("cannot find the specified function".to_string()))
        }
    }

    fn get_global_value(&self, name: &str) -> Result<Value, EngineError> {
        if let Export::GlobalVariable(global_variable) = self.get_export(name)? {
            Ok(global_variable.as_ref().borrow().get_value())
        } else {
            Err(EngineError::ObjectNotFound("cannot find the specified global variable".to_string()))
        }
    }

    fn set_global_value(&mut self, name: &str, value: Value) -> Result<(), EngineError> {
        if let Export::GlobalVariable(global_variable) = self.get_export(name)? {
            global_variable.as_ref().borrow_mut().set_value(value)
        } else {
            Err(EngineError::ObjectNotFound("cannot find the specified global variable".to_string()))
        }
    }
}

impl VMModule {
    pub fn do_loop(&self) {
        //
    }
}
