// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use anvm_parser::{
    ast::{
        self, ExportDescriptor, ExportItem, ImportDescriptor, ImportItem, Limit,
        MemoryType, TableType,
    },
    types::Value,
};

use crate::{
    instance::{EngineError, Function, GlobalVariable, Memory, Module, Table},
    vm_control_stack::VMControlStack,
    vm_function::VMFunction,
    vm_global_variable::VMGlobalVariable,
    vm_memory::VMMemory,
    vm_operand_stack::VMOperandStack,
    vm_table::VMTable,
};

pub struct VMModule {
    pub operand_stack: VMOperandStack,
    pub control_stack: VMControlStack,

    pub table: Rc<RefCell<dyn Table>>,
    pub memory: Rc<RefCell<dyn Memory>>,
    pub functions: Vec<Rc<dyn Function>>,
    pub global_variables: Vec<Rc<RefCell<dyn GlobalVariable>>>,

    pub current_call_frame_base_pointer: u32,

    name: String,
    export_items: Vec<ExportItem>,
}

impl Module for VMModule {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_export_table(&self, name: &str) -> Result<Rc<RefCell<dyn Table>>, EngineError> {
        let option_export_item = self
            .export_items
            .iter()
            .find(|export_item| export_item.name == name);

        if let Some(ExportItem {
            name: _,
            export_descriptor: ExportDescriptor::TableIndex(index),
        }) = option_export_item
        {
            // 目前 WebAssembly 只支持一个表格
            if *index != 0 {
                Err(EngineError::InvalidOperation(
                    "only table with index value 0 is allowed".to_string(),
                ))
            } else {
                Ok(Rc::clone(&self.table))
            }
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot find the specified table \"{}.{}\"",
                self.name, name
            )))
        }
    }

    fn get_export_memory(&self, name: &str) -> Result<Rc<RefCell<dyn Memory>>, EngineError> {
        let option_export_item = self
            .export_items
            .iter()
            .find(|export_item| export_item.name == name);

        if let Some(ExportItem {
            name: _,
            export_descriptor: ExportDescriptor::MemoryBlockIndex(index),
        }) = option_export_item
        {
            // 目前 WebAssembly 只支持一个内存块
            if *index != 0 {
                Err(EngineError::InvalidOperation(
                    "only memory with index value 0 is allowed".to_string(),
                ))
            } else {
                Ok(Rc::clone(&self.memory))
            }
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot find the specified memory \"{}.{}\"",
                self.name, name
            )))
        }
    }

    fn get_export_function(&self, name: &str) -> Result<Rc<dyn Function>, EngineError> {
        let option_export_item = self
            .export_items
            .iter()
            .find(|export_item| export_item.name == name);

        if let Some(ExportItem {
            name: _,
            export_descriptor: ExportDescriptor::FunctionIndex(index),
        }) = option_export_item
        {
            Ok(Rc::clone(&self.functions[*index as usize]))
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot find the specified function \"{}.{}\"",
                self.name, name
            )))
        }
    }

    fn get_export_global_variable(
        &self,
        name: &str,
    ) -> Result<Rc<RefCell<dyn GlobalVariable>>, EngineError> {
        let option_export_item = self
            .export_items
            .iter()
            .find(|export_item| export_item.name == name);

        if let Some(ExportItem {
            name: _,
            export_descriptor: ExportDescriptor::GlobalItemIndex(index),
        }) = option_export_item
        {
            Ok(Rc::clone(&self.global_variables[*index as usize]))
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot find the specified function \"{}.{}\"",
                self.name, name
            )))
        }
    }

    fn eval_function(&self, name: &str, args: &[Value]) -> Result<Vec<Value>, EngineError> {
        let rc_function = self.get_export_function(name)?;
        rc_function.as_ref().eval(args)
    }
}

impl VMModule {
    pub fn new(
        name: &str,
        ast_module: ast::Module,
        module_map: &HashMap<&str, Rc<RefCell<dyn Module>>>,
        option_memory_data: Option<Vec<u8>>,
    ) -> Result<Rc<RefCell<VMModule>>, EngineError> {
        let operand_stack = VMOperandStack::new();
        let control_stack = VMControlStack::new();
        let rc_table = create_table(&ast_module, module_map)?;
        let rc_memory = create_memory(&ast_module, module_map, option_memory_data)?;
        let export_items = ast_module.export_items.clone();

        let vm_module = VMModule {
            operand_stack,
            control_stack,
            table: Rc::clone(&rc_table),
            memory: Rc::clone(&rc_memory),
            functions: vec![], // 因为 Function 有对 VMModule 实例的反向引用，所以先用一个空的函数列表顶替。
            global_variables: vec![], // 因为 Global 里面有指令表达式需要 VMModule 实例来执行，所以先用一个空的全局列表顶替。
            current_call_frame_base_pointer: 0,
            name: name.to_string(),
            export_items,
        };

        let rc_module = Rc::new(RefCell::new(vm_module));

        // 填充 table 和 memory 的初始值
        fill_table_elements(&ast_module, Rc::clone(&rc_module))?;
        fill_memory_datas(&ast_module, Rc::clone(&rc_module))?;

        // 替换 VMModule 实例的 global_variables 成员的值。
        let global_variables =
            create_global_variables(&ast_module, module_map, Rc::clone(&rc_module))?;
        rc_module.as_ref().borrow_mut().global_variables = global_variables;

        // 替换 VMModule 实例的 functions 成员的值。
        let weak_module = Rc::downgrade(&rc_module);
        let functions = create_functions(&ast_module, module_map, weak_module)?;
        rc_module.as_ref().borrow_mut().functions = functions;

        Ok(rc_module)
    }

    pub fn do_loop(&self) {
        //
        todo!()
    }
}

fn create_table(
    ast_module: &ast::Module,
    module_map: &HashMap<&str, Rc<RefCell<dyn Module>>>,
) -> Result<Rc<RefCell<dyn Table>>, EngineError> {
    // 检查是否有表类型的导入项
    let option_import_item = ast_module
        .import_items
        .iter()
        .find(|item| matches!(item.import_descriptor, ImportDescriptor::TableType(_)));

    if let Some(ImportItem {
        module_name,
        name,
        import_descriptor: ImportDescriptor::TableType(table_type),
    }) = option_import_item
    {
        // 检查到有表类型的导入项

        let option_module = module_map.get(module_name.as_str());
        if let Some(rc_module) = option_module {
            let rc_table = rc_module.as_ref().borrow().get_export_table(name)?;

            if table_type != &rc_table.as_ref().borrow().get_table_type() {
                Err(EngineError::InvalidOperation(
                    "the type of the imported table does not match the declaration".to_string(),
                ))
            } else {
                Ok(Rc::clone(&rc_table))
            }
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot found the specified module \"{}\"",
                module_name
            )))
        }
    } else {
        // 无表类型的导入项，创建新表

        let table = if let Some(first) = ast_module.table_types.first() {
            // 根据定义创建新表
            VMTable::new(first.clone())
        } else {
            // 创建默认表（容量为 0 的表）
            VMTable::new(TableType {
                limit: Limit::Range(0, 0),
            })
        };

        Ok(Rc::new(RefCell::new(table)))
    }
}

fn create_memory(
    ast_module: &ast::Module,
    module_map: &HashMap<&str, Rc<RefCell<dyn Module>>>,
    option_memory_data: Option<Vec<u8>>,
) -> Result<Rc<RefCell<dyn Memory>>, EngineError> {
    // 检查是否有内存类型的导入项
    let option_import_item = ast_module
        .import_items
        .iter()
        .find(|item| matches!(item.import_descriptor, ImportDescriptor::MemoryType(_)));

    if let Some(ImportItem {
        module_name,
        name,
        import_descriptor: ImportDescriptor::MemoryType(memory_type),
    }) = option_import_item
    {
        // 检查到有内存类型的导入项

        let option_module = module_map.get(module_name.as_str());
        if let Some(rc_module) = option_module {
            let rc_memory = rc_module.as_ref().borrow().get_export_memory(name)?;

            if memory_type != &rc_memory.as_ref().borrow().get_memory_type() {
                Err(EngineError::InvalidOperation(
                    "the type of the imported memory does not match the declaration".to_string(),
                ))
            } else {
                Ok(Rc::clone(&rc_memory))
            }
        } else {
            Err(EngineError::ObjectNotFound(format!(
                "cannot found the specified module \"{}\"",
                module_name
            )))
        }
    } else {
        // 创建新内存块

        let memory = if let Some(init_data) = option_memory_data {
            // 使用指定的初始话数据创建内存块
            VMMemory::new_with_init_data(init_data)
        } else {
            if let Some(first) = ast_module.memory_blocks.first() {
                // 根据定义创建新内存块
                VMMemory::new(first.clone())
            } else {
                // 创建默认内存块（页面最小值为 1 的内存块）
                VMMemory::new(MemoryType {
                    limit: Limit::AtLeast(1),
                })
            }
        };

        Ok(Rc::new(RefCell::new(memory)))
    }
}

fn create_functions(
    ast_module: &ast::Module,
    module_map: &HashMap<&str, Rc<RefCell<dyn Module>>>,
    weak_module: Weak<RefCell<VMModule>>,
) -> Result<Vec<Rc<dyn Function>>, EngineError> {
    let mut functions = Vec::<Rc<dyn Function>>::new();

    // 先导入函数（假如存在的话）

    let import_items = ast_module
        .import_items
        .iter()
        .filter(|item| {
            matches!(
                item.import_descriptor,
                ImportDescriptor::FunctionTypeIndex(_)
            )
        })
        .collect::<Vec<&ImportItem>>();

    for import_item in import_items {
        if let ImportItem {
            module_name,
            name,
            import_descriptor: ImportDescriptor::FunctionTypeIndex(function_type_index),
        } = import_item
        {
            let option_module = module_map.get(module_name.as_str());
            if let Some(rc_module) = option_module {
                let rc_function = rc_module.as_ref().borrow().get_export_function(name)?;

                let function_type = &ast_module.function_types[*function_type_index as usize];
                if function_type != &rc_function.as_ref().borrow().get_function_type() {
                    return Err(EngineError::InvalidOperation(
                        "the type of the imported function does not match the declaration"
                            .to_string(),
                    ));
                } else {
                    let function = VMFunction::new_external_function(
                        function_type.clone(),
                        Rc::clone(&rc_function),
                    );
                    functions.push(Rc::new(function));
                }
            } else {
                return Err(EngineError::ObjectNotFound(format!(
                    "cannot found the specified module \"{}\"",
                    module_name
                )));
            }
        }
    }

    // 再添加当前模块内定义的函数

    for (function_index, function_type_index) in ast_module.function_list.iter().enumerate() {
        let function_type = ast_module.function_types[*function_type_index as usize].clone();
        let code_item = &ast_module.code_items[function_index];
        let local_groups = code_item.local_groups.clone();
        let expression = Rc::clone(&code_item.expression);

        let function = VMFunction::new_internal_function(
            function_type,
            local_groups,
            expression,
            Weak::clone(&weak_module),
        );
        functions.push(Rc::new(function));
    }

    Ok(functions)
}

fn create_global_variables(
    ast_module: &ast::Module,
    module_map: &HashMap<&str, Rc<RefCell<dyn Module>>>,
    rc_module: Rc<RefCell<VMModule>>,
) -> Result<Vec<Rc<RefCell<dyn GlobalVariable>>>, EngineError> {
    let mut global_variables = Vec::<Rc<RefCell<dyn GlobalVariable>>>::new();

    // 先导入全局变量（假如存在的话）

    let import_items = ast_module
        .import_items
        .iter()
        .filter(|item| matches!(item.import_descriptor, ImportDescriptor::GlobalType(_)))
        .collect::<Vec<&ImportItem>>();

    for import_item in import_items {
        if let ImportItem {
            module_name,
            name,
            import_descriptor: ImportDescriptor::GlobalType(global_type),
        } = import_item
        {
            let option_module = module_map.get(module_name.as_str());
            if let Some(rc_module) = option_module {
                let rc_global_variable = rc_module
                    .as_ref()
                    .borrow()
                    .get_export_global_variable(name)?;

                if global_type != &rc_global_variable.as_ref().borrow().get_global_type() {
                    return Err(EngineError::InvalidOperation(
                        "the type of the imported global variable does not match the declaration"
                            .to_string(),
                    ));
                } else {
                    global_variables.push(Rc::clone(&rc_global_variable));
                }
            } else {
                return Err(EngineError::ObjectNotFound(format!(
                    "cannot found the specified module \"{}\"",
                    module_name
                )));
            }
        }
    }

    // 再添加当前模块内定义的全局变量

    for global_item in &ast_module.global_items {
        let expression = &global_item.init_expression;
        let value = Value::I32(0); // todo:: 执行指令
        let global_variable = VMGlobalVariable::new(global_item.global_type.clone(), value);

        global_variables.push(Rc::new(RefCell::new(global_variable)));
    }

    Ok(global_variables)
}

fn fill_table_elements(
    ast_module: &ast::Module,
    rc_module: Rc<RefCell<VMModule>>,
) -> Result<(), EngineError> {
    // todo
    Ok(())
}

fn fill_memory_datas(
    ast_module: &ast::Module,
    rc_module: Rc<RefCell<VMModule>>,
) -> Result<(), EngineError> {
    // todo
    Ok(())
}
