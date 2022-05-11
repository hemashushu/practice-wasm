// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::ast::{
    CustomItem, FunctionIndexAndBlockLabelsPair, FunctionIndexAndLocalVariableNamesPair,
    IndexNamePair, Module, NameCollection,
};

pub fn get_type_name(module: &Module, type_index: u32) -> Option<String> {
    let name_collections = find_name_collections(module);
    let index_name_pairs = name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::TypeNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&IndexNamePair>>();

    find_name_by_index(index_name_pairs, type_index)
}

pub fn get_function_name(module: &Module, function_index: u32) -> Option<String> {
    let name_collections = find_name_collections(module);
    let index_name_pairs = name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::FunctionNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&IndexNamePair>>();

    find_name_by_index(index_name_pairs, function_index)
}

pub fn get_local_variable_name(
    module: &Module,
    function_index: u32,
    local_variable_index: u32,
) -> Option<String> {
    let name_collections = find_name_collections(module);

    let function_index_and_local_variable_names_pairs = name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::LocalVariableNamesPairList(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&FunctionIndexAndLocalVariableNamesPair>>();

    let option_index_name_pairs = function_index_and_local_variable_names_pairs
        .iter()
        .find(|item| item.function_index == function_index)
        .map(|item| {
            item.local_variable_names
                .iter()
                .collect::<Vec<&IndexNamePair>>()
        });

    if let Some(index_name_pairs) = option_index_name_pairs {
        find_name_by_index(index_name_pairs, local_variable_index)
    } else {
        None
    }
}

pub fn get_block_lable(module: &Module, function_index: u32, block_index: u32) -> Option<String> {
    let name_collections = find_name_collections(module);

    let function_index_and_local_variable_names_pairs = name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::BlockLabelsPairList(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&FunctionIndexAndBlockLabelsPair>>();

    let option_index_name_pairs = function_index_and_local_variable_names_pairs
        .iter()
        .find(|item| item.function_index == function_index)
        .map(|item| item.block_labels.iter().collect::<Vec<&IndexNamePair>>());

    if let Some(index_name_pairs) = option_index_name_pairs {
        find_name_by_index(index_name_pairs, block_index)
    } else {
        None
    }
}

pub fn get_global_variable_name(module: &Module, global_variable_index: u32) -> Option<String> {
    let name_collections = find_name_collections(module);
    let index_name_pairs = name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::GlobalVariableNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&IndexNamePair>>();

    find_name_by_index(index_name_pairs, global_variable_index)
}

pub fn get_memory_block_name(module: &Module, memory_block_index: u32) -> Option<String> {
    let name_collections = find_name_collections(module);
    let index_name_pairs = name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::MemoryBlockNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&IndexNamePair>>();

    find_name_by_index(index_name_pairs, memory_block_index)
}

pub fn get_table_name(module: &Module, table_index: u32) -> Option<String> {
    let name_collections = find_name_collections(module);
    let index_name_pairs = name_collections
        .iter()
        .filter_map(|item| match item {
            NameCollection::TableNames(pairs) => Some(pairs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&IndexNamePair>>();

    find_name_by_index(index_name_pairs, table_index)
}

fn find_name_collections(module: &Module) -> Vec<&NameCollection> {
    module
        .custom_items
        .iter()
        .filter_map(|item| match item {
            CustomItem::NameCollection(ncs) => Some(ncs),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&NameCollection>>()
}

fn find_name_by_index(index_name_pairs: Vec<&IndexNamePair>, index: u32) -> Option<String> {
    index_name_pairs
        .iter()
        .find(|item| item.index == index)
        .map(|item| format!("{}{}", "$", item.name))
}
