// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! WebAssembly 二进制格式将诸如函数名称、局部变量（包括参数）名称、结构块的标签等信息
//! 存储在 `自定义段` 里，这些信息对于反汇编、追踪调试过程都很有价值。
//! 当前模块用于从 `自定义段` 里搜寻符号的名称信息。

use std::collections::{HashMap, HashSet};

use anvm_ast::ast::{CustomItem, Module, NameCollection};

/// 模块当中所有 `名称` 的集合
pub struct NamePackage {
    pub type_names: HashMap<u32, String>,
    pub function_names: HashMap<u32, String>,
    pub local_variable_names_map: HashMap<u32, HashMap<u32, String>>,
    pub block_labels_map: HashMap<u32, HashMap<u32, String>>,
    pub global_variable_names: HashMap<u32, String>,
    pub memory_block_names: HashMap<u32, String>,
    pub table_names: HashMap<u32, String>,
    pub element_names: HashMap<u32, String>,
    pub data_names: HashMap<u32, String>,
}

impl NamePackage {
    pub fn new(module: &Module) -> Self {
        let name_collections = get_module_name_collections(module);

        Self {
            type_names: get_type_names(&name_collections),
            function_names: get_function_names(&name_collections),
            local_variable_names_map: get_local_variable_names_map(&name_collections),
            block_labels_map: get_block_lables_map(&name_collections),
            global_variable_names: get_global_variable_names(&name_collections),
            memory_block_names: get_memory_block_names(&name_collections),
            table_names: get_table_names(&name_collections),
            element_names: get_element_names(&name_collections),
            data_names: get_data_names(&name_collections),
        }
    }

    pub fn get_type_name(&self, type_index: &u32) -> Option<&String> {
        self.type_names.get(type_index)
    }

    pub fn get_function_name(&self, function_index: &u32) -> Option<&String> {
        self.function_names.get(function_index)
    }

    pub fn get_local_variable_name(
        &self,
        function_index: &u32, // 这个函数索引的值是 "包括导入和内部函数范围之内" 的索引值
        local_variable_index: &u32,
    ) -> Option<&String> {
        self.local_variable_names_map
            .get(function_index)
            .and_then(|names| names.get(local_variable_index))
    }

    pub fn get_block_lable(
        &self,
        function_index: &u32, // 这个函数索引的值是 "包括导入和内部函数范围之内" 的索引值
        block_index: &u32,
    ) -> Option<&String> {
        self.block_labels_map
            .get(function_index)
            .and_then(|names| names.get(block_index))
    }

    pub fn get_global_variable_name(&self, global_variable_index: &u32) -> Option<&String> {
        self.global_variable_names.get(global_variable_index)
    }

    pub fn get_memory_block_name(&self, memory_block_index: &u32) -> Option<&String> {
        self.memory_block_names.get(memory_block_index)
    }

    pub fn get_table_name(&self, table_index: &u32) -> Option<&String> {
        self.table_names.get(table_index)
    }

    pub fn get_element_name(&self, element_index: &u32) -> Option<&String> {
        self.element_names.get(element_index)
    }

    pub fn get_data_name(&self, data_index: &u32) -> Option<&String> {
        self.data_names.get(data_index)
    }
}

pub fn get_type_names(name_collections: &[NameCollection]) -> HashMap<u32, String> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::TypeNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| (item.index, item.name.to_owned()))
        .collect::<HashMap<u32, String>>()
}

pub fn get_function_names(name_collections: &[NameCollection]) -> HashMap<u32, String> {

    // 对象 `name_set` 用于去除重复的函数名
    // 使用 cargo 编译 Rust 程序到 wasm32-wasi 时会出现
    // 重复函数名称的情况（在 custom 段里），为了避免函数调用错误，
    // 需要把重复的函数名称移除。
    //
    // P.S.
    // wasm-tools (https://github.com/bytecodealliance/wasm-tools) 采用的是重命名的
    // 解决方案，比如遇到重复的函数名 dummy，则函数声明行会更改为 `(func $#func010<dummy> (@name dummy) ...`
    // 其中 `#func010<dummy>` 为新名称，`010` 是函数的索引，`@name` 是
    // WebAssembly Annotation (https://github.com/WebAssembly/annotations)
    let mut name_set: HashSet<String> = HashSet::new();

    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::FunctionNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| (item.index, item.name.to_owned()))
        .filter(|(_, name)| {
            if name_set.contains(name) {
                false
            } else {
                name_set.insert(name.to_owned());
                true
            }
        })
        .collect::<HashMap<u32, String>>()
}

pub fn get_local_variable_names_map(
    name_collections: &[NameCollection],
) -> HashMap<u32, HashMap<u32, String>> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::LocalVariableNamesPairList(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| {
            (
                item.function_index,
                item.local_variable_names
                    .iter()
                    .map(|sub_item| (sub_item.index, sub_item.name.to_owned()))
                    .collect::<HashMap<u32, String>>(),
            )
        })
        .collect::<HashMap<u32, HashMap<u32, String>>>()
}

pub fn get_block_lables_map(
    name_collections: &[NameCollection],
) -> HashMap<u32, HashMap<u32, String>> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::BlockLabelsPairList(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| {
            (
                item.function_index,
                item.block_labels
                    .iter()
                    .map(|sub_item| (sub_item.index, sub_item.name.to_owned()))
                    .collect::<HashMap<u32, String>>(),
            )
        })
        .collect::<HashMap<u32, HashMap<u32, String>>>()
}

pub fn get_global_variable_names(name_collections: &[NameCollection]) -> HashMap<u32, String> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::GlobalVariableNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| (item.index, item.name.to_owned()))
        .collect::<HashMap<u32, String>>()
}

pub fn get_memory_block_names(name_collections: &[NameCollection]) -> HashMap<u32, String> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::MemoryBlockNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| (item.index, item.name.to_owned()))
        .collect::<HashMap<u32, String>>()
}

pub fn get_table_names(name_collections: &[NameCollection]) -> HashMap<u32, String> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::TableNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| (item.index, item.name.to_owned()))
        .collect::<HashMap<u32, String>>()
}

pub fn get_element_names(name_collections: &[NameCollection]) -> HashMap<u32, String> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::ElementNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| (item.index, item.name.to_owned()))
        .collect::<HashMap<u32, String>>()
}

pub fn get_data_names(name_collections: &[NameCollection]) -> HashMap<u32, String> {
    name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::DataNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .map(|item| (item.index, item.name.to_owned()))
        .collect::<HashMap<u32, String>>()
}

fn get_module_name_collections(module: &Module) -> Vec<NameCollection> {
    module
        .custom_items
        .iter()
        .filter_map(|item| match item {
            CustomItem::NameCollections(name_collections) => Some(name_collections),
            _ => None,
        })
        .flatten()
        .map(|item| item.to_owned())
        .collect::<Vec<NameCollection>>()
}
