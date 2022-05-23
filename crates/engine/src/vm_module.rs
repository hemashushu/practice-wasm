// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::ast::FunctionType;

use crate::object::{FunctionItem, Instruction};

pub struct VMModule {
    /// 模块的名称
    pub name: String,

    /// 当前模块的表在 VM 表实例里的索引
    /// 目前 WebAssembly 只允许定义一张表
    pub table_index: usize,

    /// 当前模块的内存块在 VM 内存块实例里的索引
    /// 目前 WebAssembly 只允许定义一块内存
    pub memory_index: usize,

    // /// 函数列表，包括导入的函数和模块内定义的函数
    // pub functions: Vec<Rc<dyn Function>>,
    /// 全局变量列表
    pub global_variable_indexes: Vec<usize>,

    // /// 记录第一个局部变量（局部变量也包括函数参数）在栈中的位置，
    // /// 用于方便按索引访问栈中的局部变量。
    // ///
    // /// 注意，目前函数的局部变量是直接在操作数栈中开辟的，而没有另外再设一个
    // /// 局部变量表。
    // ///
    // /// 它的值等于从栈顶开始第一个 `函数调用帧`（`call frame`）的 `frame pointer`，
    // /// 对于函数内的流程控制所产生 `块结构控制帧`，不更新此成员的值。
    // /// 在 VMModule 记录此值，是为了避免每次访问局部变量时的重复计算。
    // pub current_call_frame_base_pointer: usize,

    // /// 包含一份模块的语法树对象，用于动态维护模块的内容
    // pub ast_module: ast::Module,

    // 为方便起见，这里复制了一份 `类型列表` 和 `函数-类型映射表`
    pub function_types: Vec<FunctionType>,
    pub function_to_type_indexes: Vec<usize>,

    // 这是经过转换后的函数位置列表以及指令列表
    pub function_items: Vec<FunctionItem>,
    pub instructions: Vec<Instruction>,

    /// 保留一份 AST Module。
    /// 用于动态改变模块结构（比如增加函数）、反汇编等
    /// 注意 AST Module 在执行指令过程中是完全不使用的。
    pub ast_module: anvm_ast::ast::Module,
}

impl VMModule {
    pub fn new(
        name: String,
        table_index: usize,
        memory_index: usize,
        global_variable_indexes: Vec<usize>,
        function_types: Vec<FunctionType>,
        function_to_type_indexes: Vec<usize>,
        function_items: Vec<FunctionItem>,
        instructions: Vec<Instruction>,
        ast_module: anvm_ast::ast::Module,
    ) -> Self {
        Self {
            name,
            table_index,
            memory_index,
            global_variable_indexes,
            function_types,
            function_to_type_indexes,
            function_items,
            instructions,
            ast_module,
        }
    }
}
// impl Module for VMModule {
//     fn get_name(&self) -> &str {
//         &self.name
//     }
//
//     fn get_export_table(&self, name: &str) -> Result<Rc<RefCell<dyn Table>>, EngineError> {
//         let option_export_item = self
//             .ast_module
//             .export_items
//             .iter()
//             .find(|export_item| export_item.name == name);
//
//         if let Some(ExportItem {
//             name: _,
//             export_descriptor: ExportDescriptor::TableIndex(index),
//         }) = option_export_item
//         {
//             // 目前 WebAssembly 只支持一个表格
//             if *index != 0 {
//                 Err(EngineError::InvalidOperation(
//                     "only table with index value 0 is allowed".to_string(),
//                 ))
//             } else {
//                 Ok(Rc::clone(&self.table))
//             }
//         } else {
//             Err(EngineError::ObjectNotFound(format!(
//                 "cannot find the specified table \"{}.{}\"",
//                 self.name, name
//             )))
//         }
//     }
//
//     fn get_export_memory(&self, name: &str) -> Result<Rc<RefCell<dyn Memory>>, EngineError> {
//         let option_export_item = self
//             .ast_module
//             .export_items
//             .iter()
//             .find(|export_item| export_item.name == name);
//
//         if let Some(ExportItem {
//             name: _,
//             export_descriptor: ExportDescriptor::MemoryBlockIndex(index),
//         }) = option_export_item
//         {
//             // 目前 WebAssembly 只支持一个内存块
//             if *index != 0 {
//                 Err(EngineError::InvalidOperation(
//                     "only memory with index value 0 is allowed".to_string(),
//                 ))
//             } else {
//                 Ok(Rc::clone(&self.memory))
//             }
//         } else {
//             Err(EngineError::ObjectNotFound(format!(
//                 "cannot find the specified memory \"{}.{}\"",
//                 self.name, name
//             )))
//         }
//     }
//
//     fn get_export_function(&self, name: &str) -> Result<Rc<dyn Function>, EngineError> {
//         let option_export_item = self
//             .ast_module
//             .export_items
//             .iter()
//             .find(|export_item| export_item.name == name);
//
//         if let Some(ExportItem {
//             name: _,
//             export_descriptor: ExportDescriptor::FunctionIndex(index),
//         }) = option_export_item
//         {
//             Ok(Rc::clone(&self.functions[*index as usize]))
//         } else {
//             Err(EngineError::ObjectNotFound(format!(
//                 "cannot find the specified function \"{}.{}\"",
//                 self.name, name
//             )))
//         }
//     }
//
//     fn get_export_global_variable(
//         &self,
//         name: &str,
//     ) -> Result<Rc<RefCell<dyn GlobalVariable>>, EngineError> {
//         let option_export_item = self
//             .ast_module
//             .export_items
//             .iter()
//             .find(|export_item| export_item.name == name);
//
//         if let Some(ExportItem {
//             name: _,
//             export_descriptor: ExportDescriptor::GlobalItemIndex(index),
//         }) = option_export_item
//         {
//             Ok(Rc::clone(&self.global_variables[*index as usize]))
//         } else {
//             Err(EngineError::ObjectNotFound(format!(
//                 "cannot find the specified function \"{}.{}\"",
//                 self.name, name
//             )))
//         }
//     }
//
//     fn get_exports(&self) -> Vec<Export> {
//         todo!()
//     }
//
//     fn dump_memory(&self, address: usize, length: usize) -> Vec<u8> {
//         let memory = self.memory.as_ref().borrow();
//         let data = memory.read_bytes(address, length);
//         data.to_vec()
//     }
//
//     fn get_function_by_index(&self, index: usize) -> Rc<dyn Function> {
//         Rc::clone(&self.functions[index])
//     }
// }
//
// impl VMModule {
//     pub fn new(
//         name: &str,
//         ast_module: ast::Module,
//         module_map: &HashMap<String, Rc<RefCell<dyn Module>>>,
//         option_memory_data: Option<Vec<u8>>,
//     ) -> Result<Rc<RefCell<VMModule>>, EngineError> {
//         let operand_stack = VMOperandStack::new();
//         let control_stack = VMControlStack::new();
//         let rc_table = create_table(&ast_module, module_map)?;
//         let rc_memory = create_memory(&ast_module, module_map, option_memory_data)?;
//
//         let vm_module = VMModule {
//             operand_stack,
//             control_stack,
//             table: Rc::clone(&rc_table),
//             memory: Rc::clone(&rc_memory),
//             functions: vec![], // 因为 Function 有对 VMModule 实例的反向引用，所以先用一个空的函数列表顶替。
//             global_variables: vec![], // 因为 Global 里面有指令表达式需要 VMModule 实例来执行，所以先用一个空的全局列表顶替。
//             current_call_frame_base_pointer: 0,
//             name: name.to_string(),
//             ast_module: ast_module.clone(), // 克隆一份模块的语法树对象
//         };
//
//         let rc_module = Rc::new(RefCell::new(vm_module));
//
//         // 替换 VMModule 实例的 global_variables 成员的值。
//         let global_variables =
//             create_global_variables(&ast_module, module_map, Rc::clone(&rc_module))?;
//         rc_module.as_ref().borrow_mut().global_variables = global_variables;
//
//         // 替换 VMModule 实例的 functions 成员的值。
//         let weak_module = Rc::downgrade(&rc_module);
//         let functions = create_functions(&ast_module, module_map, weak_module)?;
//         rc_module.as_ref().borrow_mut().functions = functions;
//
//         // 填充 table 和 memory 的初始值
//         fill_table_elements(&ast_module, Rc::clone(&rc_module))?;
//         fill_memory_datas(&ast_module, Rc::clone(&rc_module))?;
//
//         Ok(rc_module)
//     }
// }
//
// /// 创建新的控制栈帧
// pub fn enter_control_block(
//     vm_module: Rc<RefCell<VMModule>>,
//     frame_type: FrameType,
//     function_type: &Rc<FunctionType>,
//     function_index: Option<usize>,
//     instructions: &Rc<Vec<Instruction>>,
//     local_variable_count: usize,
// ) {
//     let mut module = vm_module.as_ref().borrow_mut();
//
//     let frame_pointer =
//         module.operand_stack.get_operand_count() - function_type.as_ref().params.len();
//
//     let stack_frame = VMStackFrame::new(
//         frame_type.clone(),
//         Rc::clone(function_type),
//         function_index,
//         Rc::clone(instructions),
//         frame_pointer,
//         local_variable_count,
//     );
//
//     module.control_stack.push_frame(stack_frame);
//
//     if frame_type == FrameType::Call {
//         module.current_call_frame_base_pointer = frame_pointer;
//     }
// }
//
// /// 删除当前控制栈帧
// pub fn leave_control_block(vm_module: Rc<RefCell<VMModule>>) {
//     let mut module = vm_module.as_ref().borrow_mut();
//     let stack_frame = module.control_stack.pop_frame();
//
//     // 做一些离开 `被调用者` 之后的清理工作
//
//     // 丢弃自当前函数调用帧以后产生的所有操作数槽（包括局部变量槽），
//     // 即丢弃 `被调用者` 产生的残留数据。
//
//     // 计算残留数据的大小，根据是除了返回值之外，其他的都属于残留数据
//     let result_count = stack_frame.function_type.as_ref().results.len();
//     let residue_count =
//         module.operand_stack.get_operand_count() - stack_frame.frame_pointer - result_count;
//
//     if residue_count > 0 {
//         // 先弹出有用的数据（即返回值）
//         let result_values = module.operand_stack.pop_values(result_count);
//         // 丢弃残留数据
//         module.operand_stack.pop_values(residue_count);
//         // 再压入有用的数据
//         module.operand_stack.push_values(&result_values);
//     }
//
//     // 如果当前栈帧是 `函数调用帧`，则还需要更新 current_call_frame_base_pointer 的值
//     if stack_frame.frame_type == FrameType::Call && module.control_stack.get_frame_count() > 0 {
//         let last_call_frame = module.control_stack.get_last_call_frame();
//         module.current_call_frame_base_pointer = last_call_frame.frame_pointer;
//     }
// }
//
// /// 重新执行一下当前流程控制结构块
// /// 用于 loop 流程控制结构块
// pub fn repeat_control_block(vm_module: Rc<RefCell<VMModule>>) {
//     let stack_size = vm_module
//         .as_ref()
//         .borrow()
//         .operand_stack
//         .get_operand_count();
//
//     // 这里用于限制 borrow_mut 的作用范围
//     let (target_block_argument_count, frame_pointer) = {
//         let module = vm_module.as_ref().borrow();
//         let stack_frame = module.control_stack.peek_frame();
//
//         let target_block_argument_count = stack_frame.function_type.as_ref().params.len();
//         let frame_pointer = stack_frame.frame_pointer;
//
//         (target_block_argument_count, frame_pointer)
//     };
//
//     // 这里用于限制 borrow_mut 的作用范围
//     {
//         let mut module = vm_module.as_ref().borrow_mut();
//
//         // 注意这里要弹出的操作数的数量是 “目标层参数所需的数量”，而不是 "当前函数的的返回值的数量"。
//         let target_block_arguments = module.operand_stack.pop_values(target_block_argument_count);
//
//         // 丢弃当前栈帧的残留的数据
//         // 即从 `frame pointer` 到栈顶的操作数
//         module.operand_stack.pop_values(stack_size - frame_pointer);
//
//         // 将实参重新压入操作数栈
//         module.operand_stack.push_values(&target_block_arguments);
//     }
//
//     // 这里用于限制 borrow_mut 的作用范围
//     {
//         let mut module = vm_module.as_ref().borrow_mut();
//         // let stack_frame = module.control_stack.peek_frame();
//
//         // 重置 pc 值
//         // stack_frame.program_counter = 0;
//         module.control_stack.reset_program_counter();
//     }
// }
//
// pub fn do_loop(vm_module: Rc<RefCell<VMModule>>) -> Result<(), EngineError> {
//     // 如果从 vm 外部调用 call 指令之后，控制栈
//     // 应该有 1 个栈帧，这时 start_depth 值为 1。
//     //
//     // 当一个函数调用外部的函数，然后外部的函数又再次调用当前 VMModule 的
//     // 其他函数时（注意，不同模块的两个函数暂时不支持相互循环调用），
//     // do_loop() 方法会再次被激活，此时的 start_depth 的值就不是 1。
//     //
//     // 所以这里的 start_depth 不能假设为 1，应该由
//     // get_frame_count() 方法获取。
//     let start_depth = vm_module.as_ref().borrow().control_stack.get_frame_count();
//
//     while vm_module.as_ref().borrow().control_stack.get_frame_count() >= start_depth {
//         // 这里用于限制 borrow_mut 的作用范围
//         let (instructions, program_counter) = {
//             let module = vm_module.as_ref().borrow();
//             let stack_frame = module.control_stack.peek_frame();
//
//             let instructions = Rc::clone(&stack_frame.instructions);
//             let program_counter = stack_frame.program_counter;
//             (instructions, program_counter)
//         };
//
//         if program_counter == instructions.len() {
//             leave_control_block(Rc::clone(&vm_module));
//         } else {
//             // 向前移动一个指令
//             move_forward_instruction(Rc::clone(&vm_module), instructions, program_counter)?;
//         }
//     }
//
//     Ok(())
// }
//
// fn move_forward_instruction(
//     vm_module: Rc<RefCell<VMModule>>,
//     instructions: Rc<Vec<Instruction>>,
//     program_counter: usize,
// ) -> Result<(), EngineError> {
//     // 这里用于限制 borrow_mut 的作用范围
//     {
//         let mut module = vm_module.as_ref().borrow_mut();
//         // let stack_frame = module.control_stack.peek_frame();
//         // stack_frame.program_counter = program_counter + 1;
//         module.control_stack.increase_program_counter();
//     }
//
//     let instruction = &instructions.as_ref()[program_counter];
//     exec_instruction(vm_module, &instruction)?;
//     Ok(())
// }
//
// fn create_table(
//     ast_module: &ast::Module,
//     module_map: &HashMap<String, Rc<RefCell<dyn Module>>>,
// ) -> Result<Rc<RefCell<dyn Table>>, EngineError> {
//     // 检查是否有表类型的导入项
//     let option_import_item = ast_module
//         .import_items
//         .iter()
//         .find(|item| matches!(item.import_descriptor, ImportDescriptor::TableType(_)));
//
//     if let Some(ImportItem {
//         module_name,
//         name,
//         import_descriptor: ImportDescriptor::TableType(table_type),
//     }) = option_import_item
//     {
//         // 检查到有表类型的导入项
//
//         let option_module = module_map.get(module_name);
//         if let Some(source_module) = option_module {
//             let rc_table = source_module.as_ref().borrow().get_export_table(name)?;
//
//             // 核对一下导入表的 "实际类型" 是否跟 "导入语句所声明的类型" 一致
//             if table_type != &rc_table.as_ref().borrow().get_table_type() {
//                 Err(EngineError::InvalidOperation(
//                     "the type of the imported table does not match the declaration".to_string(),
//                 ))
//             } else {
//                 Ok(Rc::clone(&rc_table))
//             }
//         } else {
//             Err(EngineError::ObjectNotFound(format!(
//                 "cannot found the specified module \"{}\"",
//                 module_name
//             )))
//         }
//     } else {
//         // 无表类型的导入项，创建新表
//
//         let table = if let Some(first) = ast_module.table_types.first() {
//             // 根据定义创建新表
//             VMTable::new(first.clone())
//         } else {
//             // 创建默认表（容量最小值为 0，不限最大值的表）
//             VMTable::new(TableType {
//                 limit: Limit::AtLeast(0),
//             })
//         };
//
//         Ok(Rc::new(RefCell::new(table)))
//     }
// }
//
// fn create_memory(
//     ast_module: &ast::Module,
//     module_map: &HashMap<String, Rc<RefCell<dyn Module>>>,
//     option_memory_data: Option<Vec<u8>>,
// ) -> Result<Rc<RefCell<dyn Memory>>, EngineError> {
//     // 检查是否有内存类型的导入项
//     let option_import_item = ast_module
//         .import_items
//         .iter()
//         .find(|item| matches!(item.import_descriptor, ImportDescriptor::MemoryType(_)));
//
//     if let Some(ImportItem {
//         module_name,
//         name,
//         import_descriptor: ImportDescriptor::MemoryType(memory_type),
//     }) = option_import_item
//     {
//         // 检查到有内存类型的导入项
//
//         let option_module = module_map.get(module_name);
//         if let Some(source_module) = option_module {
//             let rc_memory = source_module.as_ref().borrow().get_export_memory(name)?;
//
//             // 核对一下导入内存的 "实际类型" 是否跟 "导入语句所声明的类型" 一致
//             if memory_type != &rc_memory.as_ref().borrow().get_memory_type() {
//                 Err(EngineError::InvalidOperation(
//                     "the type of the imported memory does not match the declaration".to_string(),
//                 ))
//             } else {
//                 Ok(Rc::clone(&rc_memory))
//             }
//         } else {
//             Err(EngineError::ObjectNotFound(format!(
//                 "cannot found the specified module \"{}\"",
//                 module_name
//             )))
//         }
//     } else {
//         // 创建新内存块
//
//         let memory = if let Some(init_data) = option_memory_data {
//             // 使用指定的初始话数据创建内存块
//             VMMemory::new_with_init_data(init_data)
//         } else {
//             if let Some(first) = ast_module.memory_blocks.first() {
//                 // 根据定义创建新内存块
//                 VMMemory::new(first.clone())
//             } else {
//                 // 创建默认内存块（页面最小值为 0，不限最大值的内存块）
//                 VMMemory::new(MemoryType {
//                     limit: Limit::AtLeast(0),
//                 })
//             }
//         };
//
//         Ok(Rc::new(RefCell::new(memory)))
//     }
// }
//
// fn create_functions(
//     ast_module: &ast::Module,
//     module_map: &HashMap<String, Rc<RefCell<dyn Module>>>,
//     weak_module: Weak<RefCell<VMModule>>,
// ) -> Result<Vec<Rc<dyn Function>>, EngineError> {
//     let mut functions = Vec::<Rc<dyn Function>>::new();
//
//     // 先导入函数（假如存在的话）
//
//     let import_items = ast_module
//         .import_items
//         .iter()
//         .filter(|item| {
//             matches!(
//                 item.import_descriptor,
//                 ImportDescriptor::FunctionTypeIndex(_)
//             )
//         })
//         .collect::<Vec<&ImportItem>>();
//
//     for (index, import_item) in import_items.iter().enumerate() {
//         if let ImportItem {
//             module_name,
//             name,
//             import_descriptor: ImportDescriptor::FunctionTypeIndex(function_type_index),
//         } = import_item
//         {
//             let option_module = module_map.get(module_name);
//             if let Some(source_module) = option_module {
//                 let rc_function = source_module.as_ref().borrow().get_export_function(name)?;
//
//                 // 核对一下导入函数的 "实际类型" 是否跟 "导入语句所声明的类型" 一致
//                 let rc_function_type = &ast_module.function_types[*function_type_index as usize];
//                 if rc_function_type.as_ref()
//                     != rc_function.as_ref().borrow().get_function_type().as_ref()
//                 {
//                     return Err(EngineError::InvalidOperation(
//                         "the type of the imported function does not match the declaration"
//                             .to_string(),
//                     ));
//                 } else {
//                     let function = VMFunction::new_external_function(
//                         Rc::clone(rc_function_type),
//                         index,
//                         Some(name.clone()),
//                         Rc::clone(&rc_function),
//                     );
//                     functions.push(Rc::new(function));
//                 }
//             } else {
//                 return Err(EngineError::ObjectNotFound(format!(
//                     "cannot found the specified module \"{}\"",
//                     module_name
//                 )));
//             }
//         }
//     }
//
//     // 再添加当前模块内定义的函数
//     let imported_count = functions.len();
//
//     for (function_index, function_type_index) in ast_module.function_list.iter().enumerate() {
//         let rc_function_type = &ast_module.function_types[*function_type_index as usize];
//         let code_item = &ast_module.code_items[function_index];
//         let local_groups = code_item.local_groups.clone();
//         let rc_expression = Rc::clone(&code_item.expression);
//
//         let function = VMFunction::new_internal_function(
//             Rc::clone(rc_function_type),
//             imported_count + function_index,
//             None,
//             local_groups,
//             rc_expression,
//             Weak::clone(&weak_module),
//         );
//         functions.push(Rc::new(function));
//     }
//
//     Ok(functions)
// }
//
// fn create_global_variables(
//     ast_module: &ast::Module,
//     module_map: &HashMap<String, Rc<RefCell<dyn Module>>>,
//     vm_module: Rc<RefCell<VMModule>>,
// ) -> Result<Vec<Rc<RefCell<dyn GlobalVariable>>>, EngineError> {
//     let mut global_variables = Vec::<Rc<RefCell<dyn GlobalVariable>>>::new();
//
//     // 先导入全局变量（假如存在的话）
//
//     let import_items = ast_module
//         .import_items
//         .iter()
//         .filter(|item| matches!(item.import_descriptor, ImportDescriptor::GlobalType(_)))
//         .collect::<Vec<&ImportItem>>();
//
//     for import_item in import_items {
//         if let ImportItem {
//             module_name,
//             name,
//             import_descriptor: ImportDescriptor::GlobalType(global_type),
//         } = import_item
//         {
//             let option_module = module_map.get(module_name);
//             if let Some(source_module) = option_module {
//                 let rc_global_variable = source_module
//                     .as_ref()
//                     .borrow()
//                     .get_export_global_variable(name)?;
//
//                 // 核对一下导入全局变量的 "实际类型" 是否跟 "导入语句所声明的类型" 一致
//                 if global_type != &rc_global_variable.as_ref().borrow().get_global_type() {
//                     return Err(EngineError::InvalidOperation(
//                         "the type of the imported global variable does not match the declaration"
//                             .to_string(),
//                     ));
//                 } else {
//                     global_variables.push(Rc::clone(&rc_global_variable));
//                 }
//             } else {
//                 return Err(EngineError::ObjectNotFound(format!(
//                     "cannot found the specified module \"{}\"",
//                     module_name
//                 )));
//             }
//         }
//     }
//
//     // 再添加当前模块内定义的全局变量
//
//     for global_item in &ast_module.global_items {
//         // 初始值表达式，通常是一个 i32.const 指令
//         let expression = &global_item.init_expression;
//         exec_instructions(Rc::clone(&vm_module), expression)?;
//
//         // 操作数栈的顶端操作数，即初始值
//         let value = vm_module.as_ref().borrow_mut().operand_stack.pop();
//         let global_variable = VMGlobalVariable::new(global_item.global_type.clone(), value);
//
//         global_variables.push(Rc::new(RefCell::new(global_variable)));
//     }
//
//     Ok(global_variables)
// }
//
// fn fill_table_elements(
//     ast_module: &ast::Module,
//     vm_module: Rc<RefCell<VMModule>>,
// ) -> Result<(), EngineError> {
//     for element_item in &ast_module.element_items {
//         // 表索引，目前 WebAssembly 标准只支持 0
//         if element_item.table_index != 0 {
//             return Err(EngineError::InvalidOperation(
//                 "only table index 0 is supported".to_string(),
//             ));
//         }
//
//         // 偏移值表达式，通常是一个 i32.const 指令
//         let expression = &element_item.offset_expression;
//         exec_instructions(Rc::clone(&vm_module), expression)?;
//
//         // 操作数栈的顶端操作数，即偏移值表达式的运算结果，表示表中的元素位置（索引）
//         let value = vm_module.as_ref().borrow_mut().operand_stack.pop();
//         let offset = match value {
//             Value::I32(v) => v as usize,
//             _ => {
//                 return Err(EngineError::InvalidOperation(
//                     "table offset should be a i32 number".to_string(),
//                 ));
//             }
//         };
//
//         let rc_functions: Vec<Rc<dyn Function>> = {
//             let module = vm_module.as_ref().borrow();
//
//             element_item
//                 .function_indices
//                 .iter()
//                 .map(|function_index| module.get_function_by_index(*function_index as usize))
//                 .collect()
//         };
//
//         let module = vm_module.as_ref().borrow();
//         let mut table = module.table.as_ref().borrow_mut();
//
//         for (index, rc_function) in rc_functions.iter().enumerate() {
//             table.set_element(offset + index, Rc::clone(rc_function))?;
//         }
//     }
//
//     Ok(())
// }
//
// fn fill_memory_datas(
//     ast_module: &ast::Module,
//     vm_module: Rc<RefCell<VMModule>>,
// ) -> Result<(), EngineError> {
//     for data_item in &ast_module.data_items {
//         // 内存块索引，目前 WebAssembly 标准只支持 0
//         if data_item.memory_index != 0 {
//             return Err(EngineError::InvalidOperation(
//                 "only memory index 0 is supported".to_string(),
//             ));
//         }
//
//         // 偏移值表达式，通常是一个 i32.const 指令
//         let expression = &data_item.offset_expression;
//         exec_instructions(Rc::clone(&vm_module), expression)?;
//
//         // 操作数栈的顶端操作数，即偏移值表达式的运算结果，表示内存的有效地址
//         let value = vm_module.as_ref().borrow_mut().operand_stack.pop();
//         let offset = match value {
//             Value::I32(v) => v as usize,
//             _ => {
//                 return Err(EngineError::InvalidOperation(
//                     "memory offset should be a i32 number".to_string(),
//                 ));
//             }
//         };
//
//         let module = vm_module.as_ref().borrow();
//         let mut memory = module.memory.as_ref().borrow_mut();
//
//         memory.write_bytes(offset, &data_item.data);
//     }
//
//     Ok(())
// }
