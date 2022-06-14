// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use anvm_ast::{ast::FunctionType, types::ValueType};

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

    /// 当前模块的全局变量在 VM 全局变量实例列表里的索引
    pub global_variable_indexes: Vec<usize>,

    /// 复制一份 `类型列表`
    /// 调用函数时，需要这个函数类型表来确定实参和返回值的数量
    pub function_types: Vec<FunctionType>,

    /// 复制一份内部函数的 `局部变量信息表`
    /// 调用函数时，需要这个局部变量表来分配局部变量空槽
    ///
    /// 注意这个列表的索引是 `内部函数的索引`，而不是 `模块的所有函数的索引`，后者
    /// 包括导入函数和内部函数。
    pub internal_function_local_variable_types_list: Vec<Vec<ValueType>>,

    /// 函数位置信息列表
    /// 包括导入函数和内部函数
    /// 用于从 vm 外部调用模块内部函数时，获取被调用函数的信息
    pub function_items: Vec<FunctionItem>,

    /// 指令列表
    pub instructions: Vec<Instruction>,

    /// 内部函数的名称列表
    /// 用于生成错误信息
    pub internal_function_names: HashMap<usize, String>,
}

impl VMModule {
    pub fn new(
        name: String,
        table_index: usize,
        memory_index: usize,
        global_variable_indexes: Vec<usize>,
        function_types: Vec<FunctionType>,
        internal_function_local_variable_types_list: Vec<Vec<ValueType>>,
        function_items: Vec<FunctionItem>,
        instructions: Vec<Instruction>,
        internal_function_names: HashMap<usize, String>,
    ) -> Self {
        Self {
            name,
            table_index,
            memory_index,
            global_variable_indexes,
            function_types,
            internal_function_local_variable_types_list,
            function_items,
            instructions,
            internal_function_names,
        }
    }
}
