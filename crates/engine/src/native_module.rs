// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::any::Any;

use anvm_ast::{
    ast::FunctionType,
    types::{Value, ValueType},
};

use crate::{error::NativeError, vm::VM};

pub type NativeFunction =
    fn(&mut VM, native_module_index: usize, &[Value]) -> Result<Vec<Value>, NativeError>;

/// 本地函数的本地模块
///
/// 本地模块的结构模仿普通二进制模块的结构，即把：
///
/// - 函数的类型列表
/// - `函数-类型` 的映射表
/// - 函数列表
/// - 函数的名称
/// - 局部变量（即型参）名称
///
/// 都分开存放。
pub struct NativeModule {
    pub name: String,

    // 函数类型总列表
    pub function_types: Vec<FunctionType>,

    // 函数列表
    pub function_to_type_index_list: Vec<usize>,
    pub native_functions: Vec<NativeFunction>,

    pub function_names: Vec<String>,
    pub local_variable_names: Vec<Vec<String>>,

    pub module_context: Box<dyn ModuleContext>,
}

pub trait ModuleContext {
    fn as_any(&mut self) -> &mut dyn Any;
}

impl NativeModule {
    pub fn new(name: &str, module_context: Box<dyn ModuleContext>) -> Self {
        Self {
            name: name.to_string(),
            function_types: vec![],

            function_to_type_index_list: vec![],
            native_functions: vec![],
            function_names: vec![],
            local_variable_names: vec![],

            module_context: module_context,
        }
    }

    pub fn add_native_function(
        &mut self,
        name: &str,
        params: Vec<ValueType>,
        param_names: Vec<&str>,
        results: Vec<ValueType>,
        native_function: NativeFunction,
    ) {
        let function_type_index = self.add_function_type(params, results);

        self.function_to_type_index_list.push(function_type_index);
        self.native_functions.push(native_function);
        self.function_names.push(name.to_string());
        self.local_variable_names.push(
            param_names
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
    }

    /// 根据函数名称获得函数的索引值
    ///
    /// 注意：对于本地模块来说
    /// - 所有函数都是导出函数，所以 `导出名称` 实际上
    ///   就是函数名称（对于普通模块来说则不一定是，普通模块的函数名称只是个仅供
    ///   反汇编时所使用的额外的属性）。
    /// - 函数的索引就是模块内所有本地函数的索引，本地模块没有导入函数，所以第一个
    ///   本地很熟的索引值就是 0。
    pub fn find_function_index_by_exported_name(&self, name: &str) -> Option<usize> {
        self.function_names
            .iter()
            .enumerate()
            .find(|item| item.1 == name)
            .map(|item| item.0)
    }

    /// 创建并添加新的 FunctionType 到函数类型总列表，或者
    /// 返回已存在的 FunctionType 项目的索引值。
    fn add_function_type(&mut self, params: Vec<ValueType>, results: Vec<ValueType>) -> usize {
        let function_type = FunctionType { params, results };

        let option_function_type_index = self
            .function_types
            .iter()
            .enumerate()
            .find(|item| item.1 == &function_type)
            .map(|item| item.0);

        if let Some(function_type_index) = option_function_type_index {
            function_type_index
        } else {
            let count = self.function_types.len();
            self.function_types.push(function_type);
            count
        }
    }
}

pub struct EmptyModuleContext {
    //
}

impl ModuleContext for EmptyModuleContext {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl EmptyModuleContext {
    pub fn new() -> Self {
        Self {}
    }
}
