// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use std::vec;

use anvm_ast::{
    ast::{
        CodeItem, CustomItem, DataItem, ElementItem, ExportDescriptor, ExportItem,
        FunctionIndexAndBlockLabelsPair, FunctionIndexAndLocalVariableNamesPair, FunctionType,
        GlobalItem, GlobalType, ImportDescriptor, ImportItem, IndexNamePair, Limit, LocalGroup,
        MemoryType, Module, NameCollection, TableType, TypeItem,
    },
    instruction::{BlockType, Instruction, MemoryArgument},
    opcode,
    types::ValueType,
};

use crate::{error::ParseError, leb128decoder};

/// 二进制模块以一个 4 个字节的幻数 `0x00 0x61 0x73 0x6d` 开始。
/// 转成 ascii 则是 `0x00` 和 `asm`
const MAGIC_NUMBER: u32 = 0x6d736100;

/// 二进制格式的版本号，占用了 4 个字节
/// 当前解析器只支持版本 1（little endian）
const VERSION: u32 = 0x00000001;

/// 二进制各个段（section）的 id
const SECTION_CUSTOM_ID: u8 = 0; // 0
const SECTION_TYPE_ID: u8 = 1; // 1
const SECTION_IMPORT_ID: u8 = 2; // 2
const SECTION_FUNCTION_ID: u8 = 3; // 3
const SECTION_TABLE_ID: u8 = 4; // 4
const SECTION_MEMORY_ID: u8 = 5; // 5
const SECTION_GLOBAL_ID: u8 = 6; // 6
const SECTION_EXPORT_ID: u8 = 7; // 7
const SECTION_START_ID: u8 = 8; // 8
const SECTION_ELEMENT_ID: u8 = 9; // 9
const SECTION_CODE_ID: u8 = 10; // 10
const SECTION_DATA_ID: u8 = 11; // 11

/// 在 `函数类型段` 里的 `类型项` 的 tag 值目前只能是 `0x60`
const FUNCTION_TYPE_TAG: u8 = 0x60;

/// 值类型的 tag
const VALUE_TYPE_TAG_I32: u8 = 0x7f; // i32
const VALUE_TYPE_TAG_I64: u8 = 0x7E; // i64
const VALUE_TYPE_TAG_F32: u8 = 0x7D; // f32
const VALUE_TYPE_TAG_F64: u8 = 0x7C; // f64

/// 导入项描述 tag
const IMPORT_TAG_FUNCTION: u8 = 0;
const IMPORT_TAG_TABLE: u8 = 1;
const IMPORT_TAG_MEMORY: u8 = 2;
const IMPORT_TAG_GLOBAL: u8 = 3;

/// 表项的 tag，目前只支持 func_ref
const TABLE_TYPE_TAG_FUNC_REF: u8 = 0x70;

/// 全局变量的可变性 tag，0 == 常量
const GLOBAL_VARIABLE_TAG_IMMUTABLE: u8 = 0;

/// 全局变量的可变性 tag，1 == 变量
const GLOBAL_VARIABLE_TAG_MUTABLE: u8 = 1;

/// 块结构的返回值类型
/// 块结构（比如 `block`，`loop`，`if` 等）能够返回一个值，
/// 可以理解为一种内嵌的（无自己局部变量的）函数。
const BLOCK_TYPE_I32: i32 = -1; // 返回 i32
const BLOCK_TYPE_I64: i32 = -2; // 返回 i64
const BLOCK_TYPE_F32: i32 = -3; // 返回 f32
const BLOCK_TYPE_F64: i32 = -4; // 返回 f64
const BLOCK_TYPE_EMPTY: i32 = -64; // 无返回

const EXPORT_TAG_FUNCTION: u8 = 0;
const EXPORT_TAG_TABLE: u8 = 1;
const EXPORT_TAG_MEM: u8 = 2;
const EXPORT_TAG_GLOBAL: u8 = 3;

const NAME_COLLECTION_KIND_FUNCTION_NAMES: u8 = 0x01;
const NAME_COLLECTION_KIND_FUNCTION_LOCAL_VARIABLE_NAMES: u8 = 0x02;
const NAME_COLLECTION_KIND_FUNCTION_BLOCK_LABELS: u8 = 0x03;
const NAME_COLLECTION_KIND_TYPE_NAMES: u8 = 0x04;
const NAME_COLLECTION_KIND_TABLE_NAMES: u8 = 0x05;
const NAME_COLLECTION_KIND_MEMORY_BLOCK_NAMES: u8 = 0x06;
const NAME_COLLECTION_KIND_GLOBAL_VARIABLE_NAMES: u8 = 0x07;
const NAME_COLLECTION_KIND_ELEMENT_NAMES: u8 = 0x08;
const NAME_COLLECTION_KIND_DATA_NAMES: u8 = 0x09;

pub fn parse(source: &[u8]) -> Result<Module, ParseError> {
    parse_module(source)
}

/// # 解析二进制 module
///
/// module = magic_number:u32 + version:u32 + <section>
fn parse_module(source: &[u8]) -> Result<Module, ParseError> {
    let mut remains = source;

    // 读取幻数
    let (magic_number, post_magic_number) = read_fixed_u32(remains)?;
    if magic_number != MAGIC_NUMBER {
        return Err(ParseError::Unsupported(
            "unsupported file format".to_string(),
        ));
    }
    remains = post_magic_number;

    // 读取版本号
    let (version_number, post_version_number) = read_fixed_u32(remains)?;
    if version_number != VERSION {
        return Err(ParseError::Unsupported(format!(
            "unsupported version: {}",
            version_number
        )));
    }
    remains = post_version_number;

    parse_sections(remains)
}

/// # 解析 module 的各个段（section）
///
/// section = section_id:u8 + byte_length:u32 + section_data
///
/// 每一个段（section）均以一个 section_id 开头，然后是一个描述
/// 该段内容长度的 u32 整数，然后是该段的内容（数据）。
///
/// `自定义段` 的出现顺序不固定，而且可以出现多次，其他段的出现顺序是固定的
/// （按照 section id 从小到大排列）且不重复。
fn parse_sections(source: &[u8]) -> Result<Module, ParseError> {
    let mut module = Module {
        custom_items: vec![],
        type_items: vec![],
        import_items: vec![],
        internal_function_to_type_index_list: vec![],
        tables: vec![],
        memory_blocks: vec![],
        global_items: vec![],
        export_items: vec![],
        start_function_index: None,
        element_items: vec![],
        code_items: vec![],
        data_items: vec![],
    };

    let mut remains = source;

    // 一个模块里的段的数量是未知的，所以需要不断地消耗（consume）段数据，直到
    // 所有模块数据数据都被消耗光为止
    loop {
        // 如果剩余的数据已消耗光，则退出循环
        if remains.len() == 0 {
            break;
        }

        let (section_id, post_section_id) = read_byte(remains)?;
        let (content_length, post_content_length) = read_u32(post_section_id)?;
        let (section_data, post_section_data) =
            post_content_length.split_at(content_length as usize);
        remains = post_section_data;

        match section_id {
            SECTION_CUSTOM_ID => {
                let custom_item = parse_custom_section(section_data)?;
                module.custom_items.push(custom_item);
            }
            SECTION_TYPE_ID => {
                module.type_items = parse_type_section(section_data)?;
            }
            SECTION_IMPORT_ID => {
                module.import_items = parse_import_section(section_data)?;
            }
            SECTION_FUNCTION_ID => {
                module.internal_function_to_type_index_list =
                    parse_function_list_section(section_data)?;
            }
            SECTION_TABLE_ID => {
                module.tables = parse_table_section(section_data)?;
            }
            SECTION_MEMORY_ID => {
                module.memory_blocks = parse_memory_section(section_data)?;
            }
            SECTION_GLOBAL_ID => {
                module.global_items = parse_global_section(section_data)?;
            }
            SECTION_EXPORT_ID => {
                module.export_items = parse_export_section(section_data)?;
            }
            SECTION_START_ID => {
                let start_function_index = parse_start_function_section(section_data)?;
                module.start_function_index = Some(start_function_index);
            }
            SECTION_ELEMENT_ID => {
                module.element_items = parse_element_section(section_data)?;
            }
            SECTION_CODE_ID => {
                module.code_items = parse_function_code_section(section_data)?;
            }
            SECTION_DATA_ID => {
                module.data_items = parse_data_section(section_data)?;
            }
            _ => {
                return Err(ParseError::SyntaxError(format!(
                    "invalid section id: {}",
                    section_id
                )))
            }
        }
    }

    Ok(module)
}

/// # 解析 `自定义段`
///
/// custom_section = section_id:u8 + section_length:u32 + name + section_content
/// name = byte_length:u32 + byte{*}
/// section_content = byte{+}
///
/// 自定义段的内容一般是记录函数的名称、局部变量（包括参数）的名称等信息，
/// 自定义段的内容不影响程序的运算过程，所以也可以直接忽略。
///
/// 当 custom section 的 name 值为 "name" 时，段的内容为：
/// section_content = name_collection_item{+}
///
/// 注意上面是 name_collection_item{+} 而不是 <name_collection_item>，也就是说
/// name_collection_item 的具体数量未知，需要不断地迭代消耗（consume）数据直到所有
/// section_content 数据都被消耗完毕为止。
///
/// name_collection_item = kind:u8 + content_length:u32 + <type_name_item|function_name_item|global_name_item|...>
///
/// 当 kind 为 1 时，name_collection_item 的内容为 function_name_item
///
/// function_name_item = function_index:u32 + name:string
///
/// ```code
/// 0x011e | 01 23       | function names ;; kind == 0x01, content length = 0x23
/// 0x0120 | 0b          | 11 count
/// 0x0121 | 00 01 30    | Naming { index: 0, name: "0" }
/// 0x0124 | 01 01 31    | Naming { index: 1, name: "1" }
/// 0x0127 | 02 01 32    | Naming { index: 2, name: "2" }
/// ...
/// ```
///
/// 当 kind 为下列数值时，name_collection_item 的内容结构跟 function_name_item 一致：
///
/// - 4，项内容为 type name item
/// - 5，项内容为 table name item
/// - 6，项内容为 memory name item
/// - 7，项内容为 global name item
///
/// 二进制格式均为
///
/// item = item_index:u32 + item_name:string
///
/// 当 kind 为 2 时，项内容为 <function_local_name_item>
///
/// function_local_name_item = function_index:u32 + <local_name_item>
/// local_name_item = local_variable_index:u32 + local_variable_name:string
///
/// ```code
/// 0x0143 | 02 0b       | local names
/// 0x0145 | 01          | 1 count
/// 0x0146 | 08          | function index 8 locals      ;; function_local_name_item 开始
/// 0x0147 | 02          | 2 count
/// 0x0148 | 00 01 69    | Naming { index: 0, name: "i" }
/// 0x014b | 01 03 73 75 | Naming { index: 1, name: "sum" }
/// ...
/// ```
///
/// 当 kind 为 3 时，项内容为 <function_block_label_item>，结构跟 <function_local_name_item>
/// 一致，这里不再赘述。
///
fn parse_custom_section(source: &[u8]) -> Result<CustomItem, ParseError> {
    let (name, post_name) = read_string(source)?;

    // 目前只解析 `name` 为 "name" 的 custom_section
    let custom_item = if name == "name" {
        let name_collection_items = continue_parse_name_collection_items(post_name)?;
        CustomItem::NameCollections(name_collection_items)
    } else {
        CustomItem::Other(name, post_name.to_vec())
    };

    Ok(custom_item)
}

fn continue_parse_name_collection_items(source: &[u8]) -> Result<Vec<NameCollection>, ParseError> {
    let mut remains = source;

    let mut name_collection_items: Vec<NameCollection> = vec![];

    // name_collection_item 的数量是未知的，需要不断地消耗剩余数据，直到消耗光为止。
    loop {
        // 如果剩余的数据已消耗光，则退出循环
        if remains.len() == 0 {
            break;
        }

        let (kind, post_kind) = read_byte(remains)?;
        let (content_length, post_content_length) = read_u32(post_kind)?;
        let (item_data, post_item_data) = post_content_length.split_at(content_length as usize);
        remains = post_item_data;

        let name_collection_item = match kind {
            NAME_COLLECTION_KIND_FUNCTION_NAMES => {
                let (index_name_pairs, _) = continue_parse_index_name_pairs(item_data)?;
                NameCollection::FunctionNames(index_name_pairs)
            }
            NAME_COLLECTION_KIND_TYPE_NAMES => {
                let (index_name_pairs, _) = continue_parse_index_name_pairs(item_data)?;
                NameCollection::TypeNames(index_name_pairs)
            }
            NAME_COLLECTION_KIND_TABLE_NAMES => {
                let (index_name_pairs, _) = continue_parse_index_name_pairs(item_data)?;
                NameCollection::TableNames(index_name_pairs)
            }
            NAME_COLLECTION_KIND_MEMORY_BLOCK_NAMES => {
                let (index_name_pairs, _) = continue_parse_index_name_pairs(item_data)?;
                NameCollection::MemoryBlockNames(index_name_pairs)
            }
            NAME_COLLECTION_KIND_GLOBAL_VARIABLE_NAMES => {
                let (index_name_pairs, _) = continue_parse_index_name_pairs(item_data)?;
                NameCollection::GlobalVariableNames(index_name_pairs)
            }
            NAME_COLLECTION_KIND_FUNCTION_LOCAL_VARIABLE_NAMES => {
                let local_variable_names_pairs =
                    continue_parse_local_variable_names_pairs(item_data)?;
                NameCollection::LocalVariableNamesPairList(local_variable_names_pairs)
            }
            NAME_COLLECTION_KIND_FUNCTION_BLOCK_LABELS => {
                let block_labels_pairs = continue_parse_block_labels_pairs(item_data)?;
                NameCollection::BlockLabelsPairList(block_labels_pairs)
            }
            NAME_COLLECTION_KIND_ELEMENT_NAMES => {
                let (index_name_pairs, _) = continue_parse_index_name_pairs(item_data)?;
                NameCollection::ElementNames(index_name_pairs)
            }
            NAME_COLLECTION_KIND_DATA_NAMES => {
                let (index_name_pairs, _) = continue_parse_index_name_pairs(item_data)?;
                NameCollection::DataNames(index_name_pairs)
            }
            _ => {
                return Err(ParseError::Unsupported(format!(
                    "unsupported custom name collection kind: {}",
                    kind
                )));
            }
        };

        name_collection_items.push(name_collection_item);
    }

    Ok(name_collection_items)
}

fn continue_parse_index_name_pairs(
    source: &[u8],
) -> Result<(Vec<IndexNamePair>, &[u8]), ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;
    let mut index_name_pairs = Vec::<IndexNamePair>::with_capacity(item_count as usize);

    let mut remains = post_item_count;
    for _ in 0..item_count {
        let (index_name_pair, post_pair) = continue_parse_index_name_pair(remains)?;
        index_name_pairs.push(index_name_pair);
        remains = post_pair;
    }

    Ok((index_name_pairs, remains))
}

fn continue_parse_index_name_pair(source: &[u8]) -> Result<(IndexNamePair, &[u8]), ParseError> {
    let (index, post_index) = read_u32(source)?;
    let (name, post_name) = read_string(post_index)?;
    let index_name_pair = IndexNamePair { index, name };
    Ok((index_name_pair, post_name))
}

fn continue_parse_local_variable_names_pairs(
    source: &[u8],
) -> Result<Vec<FunctionIndexAndLocalVariableNamesPair>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;
    let mut names_pairs =
        Vec::<FunctionIndexAndLocalVariableNamesPair>::with_capacity(item_count as usize);

    let mut remains = post_item_count;
    for _ in 0..item_count {
        let (function_index_and_local_variable_names_pair, post_pair) =
            continue_parse_function_index_and_local_variable_names_pair(remains)?;
        names_pairs.push(function_index_and_local_variable_names_pair);
        remains = post_pair;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"custom\" for local name segment".to_string(),
        ))
    } else {
        Ok(names_pairs)
    }
}

fn continue_parse_function_index_and_local_variable_names_pair(
    source: &[u8],
) -> Result<(FunctionIndexAndLocalVariableNamesPair, &[u8]), ParseError> {
    let (function_index, post_function_index) = read_u32(source)?;
    let (local_variable_names, post_names) = continue_parse_index_name_pairs(post_function_index)?;
    let function_index_and_local_variable_names_pair = FunctionIndexAndLocalVariableNamesPair {
        function_index,
        local_variable_names,
    };
    Ok((function_index_and_local_variable_names_pair, post_names))
}

fn continue_parse_block_labels_pairs(
    source: &[u8],
) -> Result<Vec<FunctionIndexAndBlockLabelsPair>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;
    let mut labels_pairs =
        Vec::<FunctionIndexAndBlockLabelsPair>::with_capacity(item_count as usize);

    let mut remains = post_item_count;
    for _ in 0..item_count {
        let (function_index_and_block_labels_pair, post_pair) =
            continue_parse_function_index_and_block_labels_pair(remains)?;
        labels_pairs.push(function_index_and_block_labels_pair);
        remains = post_pair;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"custom\" for block label".to_string(),
        ))
    } else {
        Ok(labels_pairs)
    }
}

fn continue_parse_function_index_and_block_labels_pair(
    source: &[u8],
) -> Result<(FunctionIndexAndBlockLabelsPair, &[u8]), ParseError> {
    let (function_index, post_function_index) = read_u32(source)?;
    let (block_labels, post_names) = continue_parse_index_name_pairs(post_function_index)?;
    let function_index_and_block_labels_pair = FunctionIndexAndBlockLabelsPair {
        function_index,
        block_labels,
    };
    Ok((function_index_and_block_labels_pair, post_names))
}

/// # 解析 `类型段`
///
/// type_section = 0x01:byte + content_length:u32 + <function_type|other_type>
/// function_type = 0x60 + <value_type> + <value_type>
///                 ^
///                 |--- 目前 `类型项` 只支持函数类型， `0x60` 表示函数类型
fn parse_type_section(source: &[u8]) -> Result<Vec<TypeItem>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut type_items = Vec::<TypeItem>::with_capacity(item_count as usize);
    for _ in 0..item_count {
        let (type_item, post_type_item) = continue_parse_type_item(remains)?;
        type_items.push(type_item);
        remains = post_type_item;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"type\"".to_string(),
        ))
    } else {
        Ok(type_items)
    }
}

/// type = 0x60 + <value_type> + <value_type>
///        ^
///        |--- 目前 `类型项` 只支持函数类型， `0x60` 表示函数类型
fn continue_parse_type_item(source: &[u8]) -> Result<(TypeItem, &[u8]), ParseError> {
    let (tag, post_tag) = read_byte(source)?;
    if tag != FUNCTION_TYPE_TAG {
        return Err(ParseError::Unsupported(
            "only the \"type of function\" is supported in the \"type\" section".to_string(),
        ));
    }

    let (param_types, post_param_types) = continue_parse_value_types(post_tag)?;
    let (result_types, post_result_types) = continue_parse_value_types(post_param_types)?;

    let type_item = TypeItem::FunctionType(FunctionType {
        params: param_types,
        results: result_types,
    });

    Ok((type_item, post_result_types))
}

/// <value_type> = items_count:u32 + item{+}
fn continue_parse_value_types(source: &[u8]) -> Result<(Vec<ValueType>, &[u8]), ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut value_types = Vec::<ValueType>::with_capacity(item_count as usize);
    for _ in 0..item_count {
        let (value_type, post_value_type) = continue_parse_value_type(remains)?;
        value_types.push(value_type);
        remains = post_value_type;
    }

    Ok((value_types, remains))
}

/// value_type = u32
fn continue_parse_value_type(source: &[u8]) -> Result<(ValueType, &[u8]), ParseError> {
    let (tag, post_tag) = read_byte(source)?;
    let value_type = match tag {
        VALUE_TYPE_TAG_I32 => ValueType::I32,
        VALUE_TYPE_TAG_I64 => ValueType::I64,
        VALUE_TYPE_TAG_F32 => ValueType::F32,
        VALUE_TYPE_TAG_F64 => ValueType::F64,
        _ => {
            return Err(ParseError::Unsupported(format!(
                "unsupported value type: {}",
                tag
            )));
        }
    };

    Ok((value_type, post_tag))
}

/// # 解析导入项
///
/// import_section = 0x02 + content_length:u32 + <import_item>
/// import_item = module_name:string + member_name:string + import_descriptor
/// import_description = tag:byte + (type_index | table_type | memory_type | global_type)
fn parse_import_section(source: &[u8]) -> Result<Vec<ImportItem>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut import_items = Vec::<ImportItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (import_item, post_import_item) = continue_parse_import_item(remains)?;
        import_items.push(import_item);
        remains = post_import_item;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"import\"".to_string(),
        ))
    } else {
        Ok(import_items)
    }
}

/// import_item = module_name:string + member_name:string + import_descriptor
/// import_description = tag:byte + (type_index | table_type | memory_type | global_type)
fn continue_parse_import_item(source: &[u8]) -> Result<(ImportItem, &[u8]), ParseError> {
    let (module_name, post_module_name) = read_string(source)?;
    let (item_name, post_item_name) = read_string(post_module_name)?;

    let (tag, post_tag) = read_byte(post_item_name)?;
    let mut remains = post_tag;

    let import_descriptor = match tag {
        IMPORT_TAG_FUNCTION => {
            let (type_index, post_type_index) = read_u32(remains)?;
            remains = post_type_index;
            ImportDescriptor::FunctionTypeIndex(type_index)
        }
        IMPORT_TAG_TABLE => {
            let (table_type, post_table_type) = continue_parse_table_type(remains)?;
            remains = post_table_type;
            ImportDescriptor::TableType(table_type)
        }
        IMPORT_TAG_MEMORY => {
            let (memory_type, post_memory_type) = continue_parse_memory_type(remains)?;
            remains = post_memory_type;
            ImportDescriptor::MemoryType(memory_type)
        }
        IMPORT_TAG_GLOBAL => {
            let (global_type, post_global_type) = continue_parse_global_type(remains)?;
            remains = post_global_type;
            ImportDescriptor::GlobalType(global_type)
        }
        _ => {
            return Err(ParseError::Unsupported(format!(
                "unsupported import kind: {}",
                tag
            )));
        }
    };

    let import_item = ImportItem {
        module_name,
        item_name,
        import_descriptor,
    };

    Ok((import_item, remains))
}

/// table_type = 0x70 + limits
///              ^
///              |--- 0x70 表示该表项存储的是 funcref
fn continue_parse_table_type(source: &[u8]) -> Result<(TableType, &[u8]), ParseError> {
    let (tag, post_tag) = read_byte(source)?;
    if tag != TABLE_TYPE_TAG_FUNC_REF {
        Err(ParseError::Unsupported(
            "only the \"function reference\" type table is supported in the \"import\" section"
                .to_string(),
        ))
    } else {
        let (limit, post_limit) = continue_parse_limit(post_tag)?;
        Ok((TableType { limit: limit }, post_limit))
    }
}

/// memory_type = limits
fn continue_parse_memory_type(source: &[u8]) -> Result<(MemoryType, &[u8]), ParseError> {
    let (limit, post_limit) = continue_parse_limit(source)?;
    Ok((MemoryType { limit: limit }, post_limit))
}

/// global_type = val_type:byte + mut:byte
/// mut = (0|1)                             // 0 表示不可变，1 表示可变
fn continue_parse_global_type(source: &[u8]) -> Result<(GlobalType, &[u8]), ParseError> {
    let (value_type, post_value_type) = continue_parse_value_type(source)?;
    let (tag, post_tag) = read_byte(post_value_type)?;
    match tag {
        GLOBAL_VARIABLE_TAG_IMMUTABLE => Ok((
            GlobalType {
                value_type: value_type,
                mutable: false,
            },
            post_tag,
        )),
        GLOBAL_VARIABLE_TAG_MUTABLE => Ok((
            GlobalType {
                value_type: value_type,
                mutable: true,
            },
            post_tag,
        )),
        _ => Err(ParseError::SyntaxError(format!(
            "invalid global mutable tag: {}",
            tag
        ))),
    }
}

/// # 解析 limit
///
/// limits = tag:byte + min:u32 + max:u32
/// 当 tag == 0 时，表示省略了上限，只有 min 值
/// 当 tag == 1 时，表示同时指出了 min 值和 max 值
fn continue_parse_limit(source: &[u8]) -> Result<(Limit, &[u8]), ParseError> {
    let (tag, post_tag) = read_byte(source)?;
    let (min, post_min) = read_u32(post_tag)?;

    // 仅当 tag == 1 时，才有 max 数据
    match tag {
        1 => {
            let (max, post_max) = read_u32(post_min)?;
            Ok((Limit::Range(min, max), post_max))
        }
        0 => Ok((Limit::AtLeast(min), post_min)),
        _ => Err(ParseError::SyntaxError(format!(
            "invalid limit tag: {}",
            tag
        ))),
    }
}

/// # 解析函数（列表）段
///
/// function_section = 0x03 + content_length:u32 + <type_index>
/// type_index = u32
fn parse_function_list_section(source: &[u8]) -> Result<Vec<u32>, ParseError> {
    let (type_index_list, post_type_index_list) = read_u32_vec(source)?;

    if post_type_index_list.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"function\"".to_string(),
        ))
    } else {
        Ok(type_index_list)
    }
}

/// # 解析表段
///
/// table_section = 0x04 + content_length:u32 + <table_type> // 目前一个模块仅支持声明一个表项
/// table_type = 0x70 + limits
///              ^
///              |--- 0x70 表示该表项存储的是 funcref
///
/// 注意，如果一个模块已经导入了一张表，而在表段再次定义一个表，
/// 则应该视为错误，因为目前 WebAssembly 只允许一张表，
/// 但在语法解析阶段不抛出错误，留到运行再抛出。
fn parse_table_section(source: &[u8]) -> Result<Vec<TableType>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    if item_count > 1 {
        return Err(ParseError::Unsupported(
            "only one table is allowed in a module".to_string(),
        ));
    }

    let mut remains = post_item_count;
    let mut table_types = Vec::<TableType>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (table_type, post_table_type) = continue_parse_table_type(remains)?;
        table_types.push(table_type);
        remains = post_table_type;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"table\"".to_string(),
        ))
    } else {
        Ok(table_types)
    }
}

/// # 解析内存段
///
/// memory_section = 0x05 + content_length:u32 + <memory_type> // 目前一个模块仅支持声明一个内存块
/// memory_type = limits
///
/// 注意，如果一个模块已经导入了一个内存块，而在内存段再次定义一个内存块，
/// 则应该视为错误，因为目前 WebAssembly 只允许一个内存块。
///
/// 但在语法解析阶段不抛出错误，留到运行再抛出。
fn parse_memory_section(source: &[u8]) -> Result<Vec<MemoryType>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    if item_count > 1 {
        return Err(ParseError::Unsupported(
            "only one memory block is allowed in a module".to_string(),
        ));
    }

    let mut remains = post_item_count;
    let mut memory_types = Vec::<MemoryType>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (memory_type, post_memory_type) = continue_parse_memory_type(remains)?;
        memory_types.push(memory_type);
        remains = post_memory_type;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"memory\"".to_string(),
        ))
    } else {
        Ok(memory_types)
    }
}

/// # 解析全局（变量）段
///
/// global_section = 0x06 + content_length:u32 + <global_item>
/// global_item = global_type + initialize_expression
/// global_type = val_type:byte + mut:byte
/// mut = (0|1)                             // 0 表示不可变，1 表示可变
/// initialize_expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
fn parse_global_section(source: &[u8]) -> Result<Vec<GlobalItem>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut global_items = Vec::<GlobalItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (global_type, post_global_type) = continue_parse_global_type(remains)?;
        let (initialize_instruction_items, post_instruction_items) =
            continue_parse_expression(post_global_type)?;

        global_items.push(GlobalItem {
            global_type,
            initialize_instruction_items,
        });

        remains = post_instruction_items;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"global\"".to_string(),
        ))
    } else {
        Ok(global_items)
    }
}

/// # 解析指令表达式
///
/// 指令表达式一般用在 `全局项的初始值` 以及 `数据项的偏移值`。
/// 指令表达式一般由一个 `const` 指令加一个 `end` 指令构成。
fn continue_parse_expression(source: &[u8]) -> Result<(Vec<Instruction>, &[u8]), ParseError> {
    let mut instruction_items: Vec<Instruction> = vec![];
    let mut remains = source;
    let mut block_index: u32 = 0;

    // 表达式的指令数量是未知的，需要不断地解析剩余的数据，直到找到 `end` 指令为止。
    loop {
        let (instruction_item, post_instruction, next_block_index) =
            continue_parse_instruction_item(remains, block_index)?;

        let found_end_instruction = instruction_item == Instruction::End;

        instruction_items.push(instruction_item);
        block_index = next_block_index;
        remains = post_instruction;

        if found_end_instruction {
            break;
        }
    }

    Ok((instruction_items, remains))
}

/// # 解析一个指令
///
/// location_start 表示指令在二进制文件当中的开始位置，用于记录指令对应的位置，
/// 如果不需要记录指令的位置，可以传入 0.
/// block_index 表示当前结构块开始的索引，在一个函数主体里，可能存在 0 个或多个
/// 结构块，作为初始值应该传入 0，然后在当前函数的返回值的第三个值获得下一个结构块的开始索引。
fn continue_parse_instruction_item(
    source: &[u8],
    // location_start: usize,
    block_index: u32,
) -> Result<(Instruction, &[u8], u32), ParseError> {
    let mut remains = source;

    let (opcode, post_opcode) = read_byte(remains)?;
    remains = post_opcode;

    let instruction = match opcode {
        // 控制指令
        opcode::UNREACHABLE => Instruction::Unreachable,
        opcode::NOP => Instruction::Nop,
        opcode::BLOCK => {
            // block = opcode_block + block_type:i32
            let (block_type_value, post_block_type) = read_i32(remains)?;
            remains = post_block_type;

            let block_type = get_block_type(block_type_value)?;
            Instruction::Block(block_type, block_index)
        }
        opcode::LOOP => {
            // loop = opcode_loop + block_type:i32 + instructions + opcode_end
            let (block_type_value, post_block_type_value) = read_i32(remains)?;
            remains = post_block_type_value;

            let block_type = get_block_type(block_type_value)?;
            Instruction::Loop(block_type, block_index)
        }
        opcode::IF => {
            // if = opcode_if + block_type:i32 + (instructions + opcode_else){?} + instructions + opcode_end
            let (block_type_value, post_block_type) = read_i32(remains)?;
            remains = post_block_type;

            let block_type = get_block_type(block_type_value)?;
            Instruction::If(block_type, block_index)
        }
        opcode::ELSE => Instruction::Else,
        opcode::END => Instruction::End,
        opcode::BR => {
            // br = opcode_br + relative_depth:u32
            let (relative_depth, post_relative_depth) = read_u32(remains)?;
            remains = post_relative_depth;
            Instruction::Br(relative_depth)
        }
        opcode::BR_IF => {
            // br_if = opcode_br_if + relative_depth:u32
            let (relative_depth, post_relative_depth) = read_u32(remains)?;
            remains = post_relative_depth;
            Instruction::BrIf(relative_depth)
        }
        opcode::BR_TABLE => {
            // br_table = opcode_br_table + <relative_depth> + relative_depth:u32
            let (relative_depths, post_relative_depths) = read_u32_vec(remains)?;
            let (default_relative_depth, post_default_relative_depth) =
                read_u32(post_relative_depths)?;
            remains = post_default_relative_depth;

            Instruction::BrTable(relative_depths, default_relative_depth)
        }
        opcode::RETURN => Instruction::Return,
        opcode::CALL => {
            // call = opcode_call + function_index:u32
            let (function_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::Call(function_index)
        }
        opcode::CALL_INDIRECT => {
            // call_indirect = opcode_call_indirect + type_index:u32 + table_index:u32
            let (type_index, post_type_index) = read_u32(remains)?;
            let (table_index, post_table_index) = read_u32(post_type_index)?;
            remains = post_table_index;
            Instruction::CallIndirect(type_index, table_index)
        }
        opcode::DROP => Instruction::Drop,
        opcode::SELECT => Instruction::Select,

        // 变量指令
        opcode::LOCAL_GET => {
            // local.get = opcode_local.get + local_index:u32
            let (local_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::LocalGet(local_index)
        }
        opcode::LOCAL_SET => {
            // local.set = opcode_local.set + local_index:u32
            let (local_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::LocalSet(local_index)
        }
        opcode::LOCAL_TEE => {
            // local.tee = opcode_local.tee + local_index:u32
            let (local_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::LocalTee(local_index)
        }
        opcode::GLOBAL_GET => {
            // global.get = opcode_global.get + global_index:u32
            let (global_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::GlobalGet(global_index)
        }
        opcode::GLOBAL_SET => {
            // global.set = opcode_global.set + global_index:u32
            let (global_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::GlobalSet(global_index)
        }

        // 内存指令
        opcode::I32_LOAD => {
            // i32.load = opcode_i32.load + memory_argument
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Load(memory_argument)
        }
        opcode::I64_LOAD => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Load(memory_argument)
        }
        opcode::F32_LOAD => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::F32Load(memory_argument)
        }
        opcode::F64_LOAD => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::F64Load(memory_argument)
        }
        opcode::I32_LOAD8_S => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Load8S(memory_argument)
        }
        opcode::I32_LOAD8_U => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Load8U(memory_argument)
        }
        opcode::I32_LOAD16_S => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Load16S(memory_argument)
        }
        opcode::I32_LOAD16_U => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Load16U(memory_argument)
        }
        opcode::I64_LOAD8_S => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Load8S(memory_argument)
        }
        opcode::I64_LOAD8_U => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Load8U(memory_argument)
        }
        opcode::I64_LOAD16_S => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Load16S(memory_argument)
        }
        opcode::I64_LOAD16_U => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Load16U(memory_argument)
        }
        opcode::I64_LOAD32_S => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Load32S(memory_argument)
        }
        opcode::I64_LOAD32_U => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Load32U(memory_argument)
        }
        opcode::I32_STORE => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Store(memory_argument)
        }
        opcode::I64_STORE => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Store(memory_argument)
        }
        opcode::F32_STORE => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::F32Store(memory_argument)
        }
        opcode::F64_STORE => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::F64Store(memory_argument)
        }
        opcode::I32_STORE8 => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Store8(memory_argument)
        }
        opcode::I32_STORE16 => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I32Store16(memory_argument)
        }
        opcode::I64_STORE8 => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Store8(memory_argument)
        }
        opcode::I64_STORE16 => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Store16(memory_argument)
        }
        opcode::I64_STORE32 => {
            let (memory_argument, post_memory_argument) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_argument;
            Instruction::I64Store32(memory_argument)
        }
        opcode::MEMORY_SIZE => {
            // memory.size = opcode_memory.size + memory_block_index:u32
            let (memory_block_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::MemorySize(memory_block_index)
        }
        opcode::MEMORY_GROW => {
            // memory.grow = opcode_memory.grow + memory_block_index:u32
            let (memory_block_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::MemoryGrow(memory_block_index)
        }

        // 常量指令
        opcode::I32_CONST => {
            // i32.const = opcode_i32.const + number:(signed)i32
            let (number, post_number) = read_i32(remains)?;
            remains = post_number;
            Instruction::I32Const(number)
        }
        opcode::I64_CONST => {
            // i64.const = opcode_i64.const + number:(signed)i64
            let (number, post_number) = read_i64(remains)?;
            remains = post_number;
            Instruction::I64Const(number)
        }
        opcode::F32_CONST => {
            // f32.const = opcode_f32.const + number:(signed)f32
            let (number, post_number) = read_f32(remains)?;
            remains = post_number;
            Instruction::F32Const(number)
        }
        opcode::F64_CONST => {
            // f64.const = opcode_f64.const + number:(signed)f64
            let (number, post_number) = read_f64(remains)?;
            remains = post_number;
            Instruction::F64Const(number)
        }

        // 数值指令
        opcode::I32_EQZ => Instruction::I32Eqz,
        opcode::I32_EQ => Instruction::I32Eq,
        opcode::I32_NE => Instruction::I32Ne,
        opcode::I32_LT_S => Instruction::I32LtS,
        opcode::I32_LT_U => Instruction::I32LtU,
        opcode::I32_GT_S => Instruction::I32GtS,
        opcode::I32_GT_U => Instruction::I32GtU,
        opcode::I32_LE_S => Instruction::I32LeS,
        opcode::I32_LE_U => Instruction::I32LeU,
        opcode::I32_GE_S => Instruction::I32GeS,
        opcode::I32_GE_U => Instruction::I32GeU,
        opcode::I64_EQZ => Instruction::I64Eqz,
        opcode::I64_EQ => Instruction::I64Eq,
        opcode::I64_NE => Instruction::I64Ne,
        opcode::I64_LT_S => Instruction::I64LtS,
        opcode::I64_LT_U => Instruction::I64LtU,
        opcode::I64_GT_S => Instruction::I64GtS,
        opcode::I64_GT_U => Instruction::I64GtU,
        opcode::I64_LE_S => Instruction::I64LeS,
        opcode::I64_LE_U => Instruction::I64LeU,
        opcode::I64_GE_S => Instruction::I64GeS,
        opcode::I64_GE_U => Instruction::I64GeU,
        opcode::F32_EQ => Instruction::F32Eq,
        opcode::F32_NE => Instruction::F32Ne,
        opcode::F32_LT => Instruction::F32Lt,
        opcode::F32_GT => Instruction::F32Gt,
        opcode::F32_LE => Instruction::F32Le,
        opcode::F32_GE => Instruction::F32Ge,
        opcode::F64_EQ => Instruction::F64Eq,
        opcode::F64_NE => Instruction::F64Ne,
        opcode::F64_LT => Instruction::F64Lt,
        opcode::F64_GT => Instruction::F64Gt,
        opcode::F64_LE => Instruction::F64Le,
        opcode::F64_GE => Instruction::F64Ge,
        opcode::I32_CLZ => Instruction::I32Clz,
        opcode::I32_CTZ => Instruction::I32Ctz,
        opcode::I32_POP_CNT => Instruction::I32PopCnt,
        opcode::I32_ADD => Instruction::I32Add,
        opcode::I32_SUB => Instruction::I32Sub,
        opcode::I32_MUL => Instruction::I32Mul,
        opcode::I32_DIV_S => Instruction::I32DivS,
        opcode::I32_DIV_U => Instruction::I32DivU,
        opcode::I32_REM_S => Instruction::I32RemS,
        opcode::I32_REM_U => Instruction::I32RemU,
        opcode::I32_AND => Instruction::I32And,
        opcode::I32_OR => Instruction::I32Or,
        opcode::I32_XOR => Instruction::I32Xor,
        opcode::I32_SHL => Instruction::I32Shl,
        opcode::I32_SHR_S => Instruction::I32ShrS,
        opcode::I32_SHR_U => Instruction::I32ShrU,
        opcode::I32_ROTL => Instruction::I32Rotl,
        opcode::I32_ROTR => Instruction::I32Rotr,
        opcode::I64_CLZ => Instruction::I64Clz,
        opcode::I64_CTZ => Instruction::I64Ctz,
        opcode::I64_POP_CNT => Instruction::I64PopCnt,
        opcode::I64_ADD => Instruction::I64Add,
        opcode::I64_SUB => Instruction::I64Sub,
        opcode::I64_MUL => Instruction::I64Mul,
        opcode::I64_DIV_S => Instruction::I64DivS,
        opcode::I64_DIV_U => Instruction::I64DivU,
        opcode::I64_REM_S => Instruction::I64RemS,
        opcode::I64_REM_U => Instruction::I64RemU,
        opcode::I64_AND => Instruction::I64And,
        opcode::I64_OR => Instruction::I64Or,
        opcode::I64_XOR => Instruction::I64Xor,
        opcode::I64_SHL => Instruction::I64Shl,
        opcode::I64_SHR_S => Instruction::I64ShrS,
        opcode::I64_SHR_U => Instruction::I64ShrU,
        opcode::I64_ROTL => Instruction::I64Rotl,
        opcode::I64_ROTR => Instruction::I64Rotr,
        opcode::F32_ABS => Instruction::F32Abs,
        opcode::F32_NEG => Instruction::F32Neg,
        opcode::F32_CEIL => Instruction::F32Ceil,
        opcode::F32_FLOOR => Instruction::F32Floor,
        opcode::F32_TRUNC => Instruction::F32Trunc,
        opcode::F32_NEAREST => Instruction::F32Nearest,
        opcode::F32_SQRT => Instruction::F32Sqrt,
        opcode::F32_ADD => Instruction::F32Add,
        opcode::F32_SUB => Instruction::F32Sub,
        opcode::F32_MUL => Instruction::F32Mul,
        opcode::F32_DIV => Instruction::F32Div,
        opcode::F32_MIN => Instruction::F32Min,
        opcode::F32_MAX => Instruction::F32Max,
        opcode::F32_COPY_SIGN => Instruction::F32CopySign,
        opcode::F64_ABS => Instruction::F64Abs,
        opcode::F64_NEG => Instruction::F64Neg,
        opcode::F64_CEIL => Instruction::F64Ceil,
        opcode::F64_FLOOR => Instruction::F64Floor,
        opcode::F64_TRUNC => Instruction::F64Trunc,
        opcode::F64_NEAREST => Instruction::F64Nearest,
        opcode::F64_SQRT => Instruction::F64Sqrt,
        opcode::F64_ADD => Instruction::F64Add,
        opcode::F64_SUB => Instruction::F64Sub,
        opcode::F64_MUL => Instruction::F64Mul,
        opcode::F64_DIV => Instruction::F64Div,
        opcode::F64_MIN => Instruction::F64Min,
        opcode::F64_MAX => Instruction::F64Max,
        opcode::F64_COPY_SIGN => Instruction::F64CopySign,
        opcode::I32_WRAP_I64 => Instruction::I32WrapI64,
        opcode::I32_TRUNC_F32_S => Instruction::I32TruncF32S,
        opcode::I32_TRUNC_F32_U => Instruction::I32TruncF32U,
        opcode::I32_TRUNC_F64_S => Instruction::I32TruncF64S,
        opcode::I32_TRUNC_F64_U => Instruction::I32TruncF64U,
        opcode::I64_EXTEND_I32_S => Instruction::I64ExtendI32S,
        opcode::I64_EXTEND_I32_U => Instruction::I64ExtendI32U,
        opcode::I64_TRUNC_F32_S => Instruction::I64TruncF32S,
        opcode::I64_TRUNC_F32_U => Instruction::I64TruncF32U,
        opcode::I64_TRUNC_F64_S => Instruction::I64TruncF64S,
        opcode::I64_TRUNC_F64_U => Instruction::I64TruncF64U,
        opcode::F32_CONVERT_I32_S => Instruction::F32ConvertI32S,
        opcode::F32_CONVERT_I32_U => Instruction::F32ConvertI32U,
        opcode::F32_CONVERT_I64_S => Instruction::F32ConvertI64S,
        opcode::F32_CONVERT_I64_U => Instruction::F32ConvertI64U,
        opcode::F32_DEMOTE_F64 => Instruction::F32DemoteF64,
        opcode::F64_CONVERT_I32_S => Instruction::F64ConvertI32S,
        opcode::F64_CONVERT_I32_U => Instruction::F64ConvertI32U,
        opcode::F64_CONVERT_I64_S => Instruction::F64ConvertI64S,
        opcode::F64_CONVERT_I64_U => Instruction::F64ConvertI64U,
        opcode::F64_PROMOTE_F32 => Instruction::F64PromoteF32,
        opcode::I32_REINTERPRET_F32 => Instruction::I32ReinterpretF32,
        opcode::I64_REINTERPRET_F64 => Instruction::I64ReinterpretF64,
        opcode::F32_REINTERPRET_I32 => Instruction::F32ReinterpretI32,
        opcode::F64_REINTERPRET_I64 => Instruction::F64ReinterpretI64,
        opcode::I32_EXTEND8_S => Instruction::I32Extend8S,
        opcode::I32_EXTEND16_S => Instruction::I32Extend16S,
        opcode::I64_EXTEND8_S => Instruction::I64Extend8S,
        opcode::I64_EXTEND16_S => Instruction::I64Extend16S,
        opcode::I64_EXTEND32_S => Instruction::I64Extend32S,
        opcode::EXTENSION => {
            // trunc_sat = opcode_trunc_sat:byte + opcode_trunc_sat_sub_opcode:byte
            let (sub_opcode, post_sub_opcode) = read_byte(remains)?;
            remains = post_sub_opcode;

            continue_parse_extension_instructions(sub_opcode)?
        }
        _ => {
            return Err(ParseError::Unsupported(format!(
                "unsupported instruction opcode: {}",
                opcode
            )));
        }
    };

    // 如果指令是 `block`，`loop` 和 `if`，则让 `block_index` 递增
    let next_block_index = match instruction {
        Instruction::Block(_, _) | Instruction::Loop(_, _) | Instruction::If(_, _) => {
            block_index + 1
        }
        _ => block_index,
    };

    Ok((instruction, remains, next_block_index))
}

fn continue_parse_extension_instructions(sub_opcode: u8) -> Result<Instruction, ParseError> {
    match sub_opcode {
        opcode::I32_TRUNC_SAT_F32_S => Ok(Instruction::I32TruncSatF32S),
        opcode::I32_TRUNC_SAT_F32_U => Ok(Instruction::I32TruncSatF32U),
        opcode::I32_TRUNC_SAT_F64_S => Ok(Instruction::I32TruncSatF64S),
        opcode::I32_TRUNC_SAT_F64_U => Ok(Instruction::I32TruncSatF64U),
        opcode::I64_TRUNC_SAT_F32_S => Ok(Instruction::I64TruncSatF32S),
        opcode::I64_TRUNC_SAT_F32_U => Ok(Instruction::I64TruncSatF32U),
        opcode::I64_TRUNC_SAT_F64_S => Ok(Instruction::I64TruncSatF64S),
        opcode::I64_TRUNC_SAT_F64_U => Ok(Instruction::I64TruncSatF64U),
        _ => Err(ParseError::Unsupported(format!(
            "unsupported extension instruction sub-opcode: {}",
            sub_opcode
        ))),
    }
}

/// 将流程控制结构块的返回类型转换为函数返回类型
/// 因为在解析器的实现时，流程控制结构块将会当作一种简化的函数来处理
///
/// block_type = (signed) i32
fn get_block_type(value: i32) -> Result<BlockType, ParseError> {
    match value {
        BLOCK_TYPE_I32 => Ok(BlockType::ResultI32),
        BLOCK_TYPE_I64 => Ok(BlockType::ResultI64),
        BLOCK_TYPE_F32 => Ok(BlockType::ResultF32),
        BLOCK_TYPE_F64 => Ok(BlockType::ResultF64),
        BLOCK_TYPE_EMPTY => Ok(BlockType::ResultEmpty),
        _ if value >= 0 => Ok(BlockType::TypeIndex(value as u32)),
        _ => Err(ParseError::Unsupported(format!(
            "unsupported block type: {}",
            value
        ))),
    }
}

/// 解析内存数据加载和存储指令的参数
///
/// 参数有两个：
///
/// 第一个是 `offset`，即偏移值，单位是 `字节`。
///
/// 第二个是 `align`，即对齐方式，数据类型是整数，单位是 `字节`，
/// 二进制格式里 `align` 的值是 2 的对数，比如：
/// 1 表示对齐 2^1 个字节，
/// 2 表示对齐 2^2 个字节。
///
/// 文本格式里 `align` 的值就是字节数，比如文本格式的 8 对应二进制格式的 3 (2^3)。
///
/// 二进制格式：
///
/// memory_load_and_store_argument = align:u32 + offset:u32
///
/// 文本格式：
///
/// (i32.load offset=200 align=8) ;; 注意先写 offset 后写 align
///
/// 在文本格式里缺省 `align` 值时，
/// 对于 i32.load/i32.store，默认对齐 4 个字节
/// 对于 i64.load/i64.store，默认对齐 8 个字节
fn continue_parse_memory_load_and_store_argument(
    source: &[u8],
) -> Result<(MemoryArgument, &[u8]), ParseError> {
    let (align, post_align) = read_u32(source)?;
    let (offset, post_offset) = read_u32(post_align)?;
    let memory_argument = MemoryArgument { align, offset };
    Ok((memory_argument, post_offset))
}

/// # 解析导出段
///
/// export_section = 0x07 + content_length:u32 + <export_item>
/// export_item = name:string + export_descriptor
/// export_descriptor = tag:byte + (function_index | table_index | memory_block_index | global_item_index)
fn parse_export_section(source: &[u8]) -> Result<Vec<ExportItem>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut export_items = Vec::<ExportItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (export_item, post_export_item) = continue_parse_export_item(remains)?;
        export_items.push(export_item);
        remains = post_export_item;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"export\"".to_string(),
        ))
    } else {
        Ok(export_items)
    }
}

/// export_item = name:string + export_descriptor
/// export_descriptor = tag:byte + (function_index | table_index | memory_block_index | global_item_index)
fn continue_parse_export_item(source: &[u8]) -> Result<(ExportItem, &[u8]), ParseError> {
    let (name, post_name) = read_string(source)?;

    let (tag, post_tag) = read_byte(post_name)?;
    let (index, post_index) = read_u32(post_tag)?;

    let export_descriptor = match tag {
        EXPORT_TAG_FUNCTION => ExportDescriptor::FunctionIndex(index),
        EXPORT_TAG_TABLE => ExportDescriptor::TableIndex(index),
        EXPORT_TAG_MEM => ExportDescriptor::MemoryBlockIndex(index),
        EXPORT_TAG_GLOBAL => ExportDescriptor::GlobalItemIndex(index),
        _ => {
            return Err(ParseError::Unsupported(format!(
                "unsupported export kind: {}",
                tag
            )));
        }
    };

    let export_item = ExportItem {
        name,
        export_descriptor,
    };

    Ok((export_item, post_index))
}

/// # 解析起始函数索引段
///
/// start_section: 0x08 + content_length:u32 + function_index
fn parse_start_function_section(source: &[u8]) -> Result<u32, ParseError> {
    let (function_index, post_function_index) = read_u32(source)?;
    if post_function_index.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"start\"".to_string(),
        ))
    } else {
        Ok(function_index as u32)
    }
}

/// # 解析元素段
///
/// element_section = 0x09 + content_length:u32 + <element_item>
/// element_item = table_index:u32 + offset_expression + <function_index>
/// offset_expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
fn parse_element_section(source: &[u8]) -> Result<Vec<ElementItem>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut element_items = Vec::<ElementItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (element_item, post_element_item) = continue_parse_element_item(remains)?;
        element_items.push(element_item);
        remains = post_element_item;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"element\"".to_string(),
        ))
    } else {
        Ok(element_items)
    }
}

/// element_item = table_index:u32 + offset_expression + <function_index>
/// offset_expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
fn continue_parse_element_item(source: &[u8]) -> Result<(ElementItem, &[u8]), ParseError> {
    let (table_index, post_index) = read_u32(source)?; // table index 总是为 0
    let (offset_instruction_items, post_instruction_items) = continue_parse_expression(post_index)?;
    let (function_indices, post_indices) = read_u32_vec(post_instruction_items)?;

    let element_item = ElementItem {
        table_index: table_index,
        offset_instruction_items,
        function_indices,
    };

    Ok((element_item, post_indices))
}

/// # 解析代码段
///
/// code_section = 0x0a + content_length:u32 + <code_item>
/// code_item = code_content_length:u32 + <local_group> + expression
/// local_group = local_variable_count:u32 + value_type:byte
/// expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
///
/// code_content_length 表示该项目的内容总大小，包括了局部变量声明列表以及指令序列，指令序列结尾的 0x0B。
fn parse_function_code_section(source: &[u8]) -> Result<Vec<CodeItem>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut code_items = Vec::<CodeItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (code_item, post_code_item) = continue_parse_code_item(remains)?;
        code_items.push(code_item);
        remains = post_code_item;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"code\"".to_string(),
        ))
    } else {
        Ok(code_items)
    }
}

/// code_item = code_content_length:u32 + <local_group> + expression
/// local_group = local_variable_count:u32 + value_type:byte
/// expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
fn continue_parse_code_item(source: &[u8]) -> Result<(CodeItem, &[u8]), ParseError> {
    // code_content_length 表示该项目的内容总大小，包括了局部变量声明列表以及指令序列，指令序列结尾的 0x0B。
    let (code_content_length, post_code_content_length) = read_u32(source)?;
    let (code_data, post_code_data) =
        post_code_content_length.split_at(code_content_length as usize);

    let (local_groups, post_local_groups) = continue_parse_local_groups(code_data)?;
    let instruction_items = continue_parse_instruction_items(post_local_groups)?;
    let code_item = CodeItem {
        local_groups,
        instruction_items,
    };

    Ok((code_item, post_code_data))
}

fn continue_parse_local_groups(source: &[u8]) -> Result<(Vec<LocalGroup>, &[u8]), ParseError> {
    let (local_group_count, post_local_group_count) = read_u32(source)?;

    let mut remains = post_local_group_count;
    let mut local_groups = Vec::<LocalGroup>::with_capacity(local_group_count as usize);

    for _ in 0..local_group_count {
        let (variable_count, post_variable_count) = read_u32(remains)?;
        let (value_type, post_value_type) = continue_parse_value_type(post_variable_count)?;

        let local_group = LocalGroup {
            variable_count,
            value_type,
        };
        local_groups.push(local_group);
        remains = post_value_type;
    }

    Ok((local_groups, remains))
}

fn continue_parse_instruction_items(source: &[u8]) -> Result<Vec<Instruction>, ParseError> {
    let mut instruction_items: Vec<Instruction> = vec![];
    let mut remains = source;

    let mut block_index: u32 = 0;

    // 指令序列的指令数量是未知的，需要不断地解析/消耗剩余的数据，直到消耗光所有的数据为止。
    loop {
        if remains.len() == 0 {
            break;
        }

        let (instruction_item, post_instruction, next_block_index) =
            continue_parse_instruction_item(remains, block_index)?;
        instruction_items.push(instruction_item);

        block_index = next_block_index;
        remains = post_instruction;
    }

    if let Some(last) = instruction_items.last() {
        if last != &Instruction::End {
            return Err(ParseError::SyntaxError(
                "instruction sequence should end with the \"end\" instruction".to_string(),
            ));
        }
    }

    Ok(instruction_items)
}

/// # 解析数据段
///
/// data_section = 0x0b + content_length:u32 + <data_item>
/// data_item = memory_block_index:u32 + offset_expression + data:byte{*}
/// offset_expression = = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
fn parse_data_section(source: &[u8]) -> Result<Vec<DataItem>, ParseError> {
    let (item_count, post_item_count) = read_u32(source)?;

    let mut remains = post_item_count;
    let mut data_items = Vec::<DataItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (data_item, post_data_item) = continue_parse_data_item(remains)?;
        data_items.push(data_item);
        remains = post_data_item;
    }

    if remains.len() != 0 {
        Err(ParseError::SyntaxError(
            "unexpected data in section \"data\"".to_string(),
        ))
    } else {
        Ok(data_items)
    }
}

/// data_item = memory_block_index:u32 + offset_expression + data:byte{*}
/// offset_expression = = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
fn continue_parse_data_item(source: &[u8]) -> Result<(DataItem, &[u8]), ParseError> {
    let (memory_block_index, post_index) = read_u32(source)?; // memory index 总是为 0
    let (offset_instruction_items, post_instruction_items) = continue_parse_expression(post_index)?;
    let (data, post_data) = read_byte_vec(post_instruction_items)?;

    let data_item = DataItem {
        memory_block_index,
        offset_instruction_items,
        data,
    };

    Ok((data_item, post_data))
}

// 辅助函数

/// 读取一个字节的内容
/// 返回一个 u8 整数
fn read_byte(source: &[u8]) -> Result<(u8, &[u8]), ParseError> {
    match source.split_first() {
        Some((first, rest)) => Ok((*first, rest)),
        None => Err(ParseError::UnexpectedEnd),
    }
}

fn read_bytes(source: &[u8], length: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if length > source.len() {
        Err(ParseError::UnexpectedEnd)
    } else {
        Ok(source.split_at(length))
    }
}

/// 读取固定长度（4 字节）u32
fn read_fixed_u32(source: &[u8]) -> Result<(u32, &[u8]), ParseError> {
    let (bytes, remains) = read_bytes(source, 4)?;
    Ok((u32::from_le_bytes(bytes.try_into().unwrap()), remains))
}

/// 读取变长（leb128 编码的）u32
///
/// 注：
/// 大部分指令使用 unsigned int，但有些使用 signed int，
/// 比如 const 指令的立即数和 block type
fn read_u32(source: &[u8]) -> Result<(u32, &[u8]), ParseError> {
    match leb128decoder::decode_u32(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取变长（leb128 编码的）i32
fn read_i32(source: &[u8]) -> Result<(i32, &[u8]), ParseError> {
    match leb128decoder::decode_i32(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取变长（leb128 编码的）i64
///
/// 注：
/// 大部分指令使用 unsigned int，但有些使用 signed int，
/// 比如 const 指令的立即数和 block type
fn read_i64(source: &[u8]) -> Result<(i64, &[u8]), ParseError> {
    match leb128decoder::decode_i64(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取变长（leb128 编码的）i64
fn read_u64(source: &[u8]) -> Result<(u64, &[u8]), ParseError> {
    match leb128decoder::decode_u64(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取固定长度的 f32
fn read_f32(source: &[u8]) -> Result<(f32, &[u8]), ParseError> {
    let (bytes, remains) = read_bytes(source, 4)?;
    Ok((f32::from_le_bytes(bytes.try_into().unwrap()), remains))
}

/// 读取固定长度的 f64
fn read_f64(source: &[u8]) -> Result<(f64, &[u8]), ParseError> {
    let (bytes, remains) = read_bytes(source, 4)?;
    Ok((f64::from_le_bytes(bytes.try_into().unwrap()), remains))
}

/// 读取如下结构的 u32 数组
/// `length:u32 + u32{*}`
fn read_u32_vec(source: &[u8]) -> Result<(Vec<u32>, &[u8]), ParseError> {
    let mut remains = source;
    let (length, post_length) = read_u32(remains)?;

    remains = post_length;

    let mut list: Vec<u32> = vec![];
    for _ in 0..length {
        let (value, post_value) = read_u32(remains)?;
        list.push(value);
        remains = post_value;
    }

    Ok((list, remains))
}

/// 读取如下结构的 byte 数组
/// `length:u32 + u8{*}`
fn read_byte_vec(source: &[u8]) -> Result<(Vec<u8>, &[u8]), ParseError> {
    let (length, post_length) = read_u32(source)?;
    let (bytes, post_bytes) = read_bytes(post_length, length as usize)?;
    Ok((bytes.to_vec(), post_bytes))
}

/// 读取如下结构的 byte 数组并以 UTF-8 编码转换为 String
/// `length:u32 + u8{*}`
fn read_string(source: &[u8]) -> Result<(String, &[u8]), ParseError> {
    let (bytes, post_bytes) = read_byte_vec(source)?;
    match String::from_utf8(bytes) {
        Ok(s) => Ok((s, post_bytes)),
        _ => Err(ParseError::DecodingError),
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use anvm_ast::{
        ast::{
            CodeItem, CustomItem, DataItem, ElementItem, ExportDescriptor, ExportItem,
            FunctionIndexAndBlockLabelsPair, FunctionIndexAndLocalVariableNamesPair, FunctionType,
            GlobalItem, GlobalType, ImportDescriptor, ImportItem, IndexNamePair, Limit, LocalGroup,
            MemoryType, Module, NameCollection, TableType, TypeItem,
        },
        instruction::{BlockType, Instruction, MemoryArgument},
        types::ValueType,
    };

    use super::parse;
    use pretty_assertions::assert_eq;

    // 辅助方法

    fn get_test_binary_resource(filename: &str) -> Vec<u8> {
        let mut path_buf = env::current_dir().unwrap();

        // 使用 `cargo test` 测试时，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm/crates/binary-parser`；
        //
        // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm`。
        //
        // 下面语句用于处理这种情况。

        if !path_buf.ends_with("binary-parser") {
            // path_buf.pop();
            // path_buf.pop();
            path_buf.push("crates");
            path_buf.push("binary-parser");
        }
        let fullname_buf = path_buf.join("resources").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!(
            "failed to read the specified binary file: {}",
            fullname
        ))
    }

    /// 移除暂时不支持的 custom section item
    fn remove_unsupported_custom_section(mut module: Module) -> Module {
        let custom_items = module
            .custom_items
            .iter()
            .filter(|item| {
                if let CustomItem::NameCollections(_) = item {
                    true
                } else {
                    false
                }
            })
            .map(|item| item.to_owned())
            .collect::<Vec<CustomItem>>();
        module.custom_items = custom_items;
        module
    }

    #[test]
    fn test_parse_module_section_1() {
        let binary = get_test_binary_resource("test-section-1.wasm");
        let module = parse(&binary).unwrap();
        let expected = Module {
            custom_items: vec![],

            type_items: vec![TypeItem::FunctionType(FunctionType {
                params: vec![],
                results: vec![ValueType::I32],
            })],
            import_items: vec![ImportItem {
                module_name: "env".to_string(),
                item_name: "__linear_memory".to_string(),
                import_descriptor: ImportDescriptor::MemoryType(MemoryType {
                    limit: Limit::AtLeast(0),
                }),
            }],
            internal_function_to_type_index_list: vec![0],
            tables: vec![],
            memory_blocks: vec![],
            global_items: vec![],
            export_items: vec![],
            start_function_index: None,
            element_items: vec![],
            code_items: vec![CodeItem {
                local_groups: vec![LocalGroup {
                    variable_count: 1,
                    value_type: ValueType::I32,
                }],
                instruction_items: vec![
                    Instruction::I32Const(100),
                    Instruction::LocalSet(0),
                    Instruction::LocalGet(0),
                    Instruction::Return,
                    Instruction::End,
                ],
            }],
            data_items: vec![],
        };
        assert_eq!(expected, remove_unsupported_custom_section(module));
    }

    #[test]
    fn test_parse_module_section_2() {
        let binary = get_test_binary_resource("test-section-2.wasm");
        let module = parse(&binary).unwrap();
        let expected = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::FunctionNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "add".to_string(),
                    },
                    IndexNamePair {
                        index: 1,
                        name: "sub".to_string(),
                    },
                    IndexNamePair {
                        index: 2,
                        name: "inc".to_string(),
                    },
                    IndexNamePair {
                        index: 3,
                        name: "show".to_string(),
                    },
                ]),
                NameCollection::GlobalVariableNames(vec![IndexNamePair {
                    index: 0,
                    name: "__stack_pointer".to_string(),
                }]),
            ])],

            type_items: vec![
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32, ValueType::I32],
                    results: vec![ValueType::I32],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![ValueType::I32],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![],
                }),
            ],
            import_items: vec![],
            internal_function_to_type_index_list: vec![0, 0, 1, 2],
            tables: vec![],
            memory_blocks: vec![MemoryType {
                limit: Limit::AtLeast(16),
            }],
            global_items: vec![
                GlobalItem {
                    global_type: GlobalType {
                        mutable: true,
                        value_type: ValueType::I32,
                    },
                    initialize_instruction_items: vec![
                        Instruction::I32Const(1048576),
                        Instruction::End,
                    ],
                },
                GlobalItem {
                    global_type: GlobalType {
                        mutable: false,
                        value_type: ValueType::I32,
                    },
                    initialize_instruction_items: vec![
                        Instruction::I32Const(1048576),
                        Instruction::End,
                    ],
                },
                GlobalItem {
                    global_type: GlobalType {
                        mutable: false,
                        value_type: ValueType::I32,
                    },
                    initialize_instruction_items: vec![
                        Instruction::I32Const(1048576),
                        Instruction::End,
                    ],
                },
            ],
            export_items: vec![
                ExportItem {
                    name: "memory".to_string(),
                    export_descriptor: ExportDescriptor::MemoryBlockIndex(0),
                },
                ExportItem {
                    name: "add".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(0),
                },
                ExportItem {
                    name: "sub".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(1),
                },
                ExportItem {
                    name: "inc".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(2),
                },
                ExportItem {
                    name: "show".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(3),
                },
                ExportItem {
                    name: "__data_end".to_string(),
                    export_descriptor: ExportDescriptor::GlobalItemIndex(1),
                },
                ExportItem {
                    name: "__heap_base".to_string(),
                    export_descriptor: ExportDescriptor::GlobalItemIndex(2),
                },
            ],
            start_function_index: None,
            element_items: vec![],
            code_items: vec![
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::LocalGet(1),
                        Instruction::LocalGet(0),
                        Instruction::I32Add,
                        Instruction::End,
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::LocalGet(0),
                        Instruction::LocalGet(1),
                        Instruction::I32Sub,
                        Instruction::End,
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::LocalGet(0),
                        Instruction::I32Const(1),
                        Instruction::I32Add,
                        Instruction::End,
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![Instruction::End],
                },
            ],
            data_items: vec![],
        };
        assert_eq!(expected, remove_unsupported_custom_section(module));
    }

    #[test]
    fn test_parse_module_section_3() {
        let binary = get_test_binary_resource("test-section-3.wasm");
        let module = parse(&binary).unwrap();
        let expected = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::FunctionNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "fputc".to_string(),
                    },
                    IndexNamePair {
                        index: 2,
                        name: "f2".to_string(),
                    },
                    IndexNamePair {
                        index: 3,
                        name: "f3".to_string(),
                    },
                ]),
                NameCollection::TypeNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "ft0".to_string(),
                    },
                    IndexNamePair {
                        index: 1,
                        name: "ft1".to_string(),
                    },
                ]),
            ])],

            type_items: vec![
                TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![ValueType::I32],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![],
                }),
            ],
            import_items: vec![
                ImportItem {
                    module_name: "env".to_string(),
                    item_name: "putc".to_string(),
                    import_descriptor: ImportDescriptor::FunctionTypeIndex(0),
                },
                ImportItem {
                    module_name: "env".to_string(),
                    item_name: "print".to_string(),
                    import_descriptor: ImportDescriptor::FunctionTypeIndex(0),
                },
            ],
            internal_function_to_type_index_list: vec![1, 1],
            tables: vec![TableType {
                limit: Limit::Range(2, 4),
            }],
            memory_blocks: vec![MemoryType {
                limit: Limit::Range(1, 8),
            }],
            global_items: vec![],
            export_items: vec![
                ExportItem {
                    name: "i_f2".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(2),
                },
                ExportItem {
                    name: "re_putc".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(0),
                },
            ],
            start_function_index: Some(3),
            element_items: vec![
                ElementItem {
                    table_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(1), Instruction::End],
                    function_indices: vec![2],
                },
                ElementItem {
                    table_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(3), Instruction::End],
                    function_indices: vec![3],
                },
            ],
            code_items: vec![
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::I32Load(MemoryArgument {
                            align: 2,
                            offset: 100,
                        }),
                        Instruction::Call(0),
                        Instruction::Call(3),
                        Instruction::End,
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::I32Load(MemoryArgument {
                            align: 3,
                            offset: 200,
                        }),
                        Instruction::I64Load(MemoryArgument {
                            align: 3,
                            offset: 400,
                        }),
                        Instruction::End,
                    ],
                },
            ],
            data_items: vec![
                DataItem {
                    memory_block_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(100), Instruction::End],
                    data: vec![104, 101, 108, 108, 111],
                },
                DataItem {
                    memory_block_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(200), Instruction::End],
                    data: vec![80, 96, 112],
                },
            ],
        };
        assert_eq!(expected, remove_unsupported_custom_section(module));
    }

    #[test]
    fn test_parse_module_section_custom() {
        let binary = get_test_binary_resource("test-section-custom.wasm");
        let module = parse(&binary).unwrap();
        let expected = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::FunctionNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "fun0".to_string(),
                    },
                    IndexNamePair {
                        index: 1,
                        name: "fun1".to_string(),
                    },
                ]),
                NameCollection::LocalVariableNamesPairList(vec![
                    FunctionIndexAndLocalVariableNamesPair {
                        function_index: 0,
                        local_variable_names: vec![
                            IndexNamePair {
                                index: 0,
                                name: "a".to_string(),
                            },
                            IndexNamePair {
                                index: 1,
                                name: "b".to_string(),
                            },
                            IndexNamePair {
                                index: 2,
                                name: "var2".to_string(),
                            },
                        ],
                    },
                    FunctionIndexAndLocalVariableNamesPair {
                        function_index: 1,
                        local_variable_names: vec![
                            IndexNamePair {
                                index: 0,
                                name: "var0".to_string(),
                            },
                            IndexNamePair {
                                index: 1,
                                name: "var1".to_string(),
                            },
                        ],
                    },
                ]),
                NameCollection::BlockLabelsPairList(vec![
                    FunctionIndexAndBlockLabelsPair {
                        function_index: 0,
                        block_labels: vec![
                            IndexNamePair {
                                index: 0,
                                name: "b0".to_string(),
                            },
                            IndexNamePair {
                                index: 1,
                                name: "b1".to_string(),
                            },
                            IndexNamePair {
                                index: 2,
                                name: "b2".to_string(),
                            },
                            IndexNamePair {
                                index: 4,
                                name: "4".to_string(),
                            },
                            IndexNamePair {
                                index: 5,
                                name: "5".to_string(),
                            },
                        ],
                    },
                    FunctionIndexAndBlockLabelsPair {
                        function_index: 1,
                        block_labels: vec![IndexNamePair {
                            index: 0,
                            name: "l0".to_string(),
                        }],
                    },
                ]),
                NameCollection::TypeNames(vec![IndexNamePair {
                    index: 0,
                    name: "type0".to_string(),
                }]),
                NameCollection::TableNames(vec![IndexNamePair {
                    index: 0,
                    name: "tab0".to_string(),
                }]),
                NameCollection::MemoryBlockNames(vec![IndexNamePair {
                    index: 0,
                    name: "mem0".to_string(),
                }]),
                NameCollection::ElementNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "elem_one".to_string(),
                    },
                    IndexNamePair {
                        index: 1,
                        name: "elem_two".to_string(),
                    },
                ]),
                NameCollection::DataNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "data_foo".to_string(),
                    },
                    IndexNamePair {
                        index: 1,
                        name: "data_bar".to_string(),
                    },
                ]),
            ])],

            type_items: vec![
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32, ValueType::I32],
                    results: vec![ValueType::I32, ValueType::I64],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32, ValueType::I64],
                    results: vec![ValueType::I32],
                }),
            ],
            import_items: vec![],
            internal_function_to_type_index_list: vec![1, 0],
            tables: vec![TableType {
                limit: Limit::Range(2, 4),
            }],
            memory_blocks: vec![MemoryType {
                limit: Limit::Range(1, 2),
            }],
            global_items: vec![],
            export_items: vec![],
            start_function_index: None,
            element_items: vec![
                ElementItem {
                    table_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(1), Instruction::End],
                    function_indices: vec![0],
                },
                ElementItem {
                    table_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(3), Instruction::End],
                    function_indices: vec![1],
                },
            ],
            code_items: vec![
                CodeItem {
                    local_groups: vec![LocalGroup {
                        value_type: ValueType::F32,
                        variable_count: 1,
                    }],
                    instruction_items: vec![
                        Instruction::Block(BlockType::ResultEmpty, 0),
                        Instruction::Block(BlockType::ResultEmpty, 1),
                        Instruction::Block(BlockType::ResultEmpty, 2),
                        Instruction::I32Const(2),
                        Instruction::Br(1),
                        Instruction::End,
                        Instruction::End,
                        Instruction::If(BlockType::ResultEmpty, 3),
                        Instruction::I32Const(3),
                        Instruction::Else,
                        Instruction::I32Const(4),
                        Instruction::End,
                        Instruction::End,
                        Instruction::Block(BlockType::ResultEmpty, 4),
                        Instruction::Block(BlockType::ResultEmpty, 5),
                        Instruction::Br(1),
                        Instruction::End,
                        Instruction::End,
                        Instruction::End,
                    ],
                },
                CodeItem {
                    local_groups: vec![
                        LocalGroup {
                            value_type: ValueType::I32,
                            variable_count: 1,
                        },
                        LocalGroup {
                            value_type: ValueType::I64,
                            variable_count: 1,
                        },
                    ],
                    instruction_items: vec![
                        Instruction::I32Const(100),
                        Instruction::Loop(BlockType::TypeIndex(0), 0),
                        Instruction::Nop,
                        Instruction::Br(0),
                        Instruction::End,
                        Instruction::End,
                    ],
                },
            ],
            data_items: vec![
                DataItem {
                    memory_block_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(10), Instruction::End],
                    data: vec![102, 111, 111],
                },
                DataItem {
                    memory_block_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(20), Instruction::End],
                    data: vec![98, 97, 114],
                },
            ],
        };
        assert_eq!(expected, remove_unsupported_custom_section(module));
    }

    #[test]
    fn test_parse_instruction_const() {
        let binary = get_test_binary_resource("test-instruction-const.wasm");
        let module = parse(&binary).unwrap();
        assert_eq!(
            module.code_items[0],
            CodeItem {
                local_groups: vec![],
                instruction_items: vec![
                    Instruction::F32Const(12.3),
                    Instruction::F32Const(45.6),
                    Instruction::F32Add,
                    Instruction::I32TruncSatF32S,
                    Instruction::Drop,
                    Instruction::End
                ]
            }
        );
    }

    #[test]
    fn test_parse_instruction_variable() {
        let binary = get_test_binary_resource("test-instruction-variable.wasm");
        let module = parse(&binary).unwrap();

        assert_eq!(
            module.global_items,
            vec![
                GlobalItem {
                    global_type: GlobalType {
                        value_type: ValueType::I32,
                        mutable: true
                    },
                    initialize_instruction_items: vec![Instruction::I32Const(1), Instruction::End]
                },
                GlobalItem {
                    global_type: GlobalType {
                        value_type: ValueType::I32,
                        mutable: true
                    },
                    initialize_instruction_items: vec![Instruction::I32Const(2), Instruction::End]
                }
            ]
        );

        assert_eq!(
            module.type_items,
            vec![TypeItem::FunctionType(FunctionType {
                params: vec![ValueType::I32, ValueType::I32],
                results: vec![]
            })]
        );

        assert_eq!(
            module.code_items[0],
            CodeItem {
                local_groups: vec![
                    LocalGroup {
                        value_type: ValueType::I32,
                        variable_count: 2
                    },
                    LocalGroup {
                        value_type: ValueType::I64,
                        variable_count: 2
                    },
                ],
                instruction_items: vec![
                    Instruction::GlobalGet(0),
                    Instruction::GlobalSet(1),
                    Instruction::LocalGet(0),
                    Instruction::LocalSet(1),
                    Instruction::End
                ]
            }
        );
    }

    #[test]
    fn test_parse_instruction_memory() {
        let binary = get_test_binary_resource("test-instruction-memory.wasm");
        let module = parse(&binary).unwrap();

        assert_eq!(
            module.memory_blocks,
            vec![MemoryType {
                limit: Limit::Range(1, 8)
            }]
        );

        assert_eq!(
            module.data_items,
            vec![DataItem {
                memory_block_index: 0,
                offset_instruction_items: vec![Instruction::I32Const(100), Instruction::End],
                data: vec!['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]
            }]
        );

        assert_eq!(
            module.code_items[0],
            CodeItem {
                local_groups: vec![],
                instruction_items: vec![
                    Instruction::I32Const(1),
                    Instruction::I32Const(2),
                    Instruction::I32Load(MemoryArgument {
                        align: 2,
                        offset: 100
                    }),
                    Instruction::I32Store(MemoryArgument {
                        align: 2,
                        offset: 100
                    }),
                    Instruction::MemorySize(0),
                    Instruction::Drop,
                    Instruction::I32Const(4),
                    Instruction::MemoryGrow(0),
                    Instruction::Drop,
                    Instruction::End
                ]
            }
        );
    }

    #[test]
    fn test_parse_instruction_flow_control() {
        let s0 = get_test_binary_resource("test-instruction-flow-control.wasm");
        let m0 = parse(&s0).unwrap();

        assert_eq!(
            m0.code_items[0],
            CodeItem {
                local_groups: vec![],
                instruction_items: vec![
                    Instruction::Block(BlockType::ResultI32, 0),
                    Instruction::I32Const(1),
                    Instruction::Loop(BlockType::ResultI32, 1),
                    Instruction::I32Const(2),
                    Instruction::If(BlockType::ResultI32, 2),
                    Instruction::I32Const(3),
                    Instruction::Else,
                    Instruction::I32Const(4),
                    Instruction::End,
                    Instruction::End,
                    Instruction::End,
                    Instruction::Drop,
                    Instruction::End,
                ]
            }
        );
    }

    #[test]
    fn test_parse_instruction_call() {
        let s0 = get_test_binary_resource("test-instruction-call.wasm");
        let m0 = parse(&s0).unwrap();

        assert_eq!(
            m0.type_items,
            vec![TypeItem::FunctionType(FunctionType {
                params: vec![],
                results: vec![]
            })]
        );

        assert_eq!(m0.internal_function_to_type_index_list, vec![0, 0]);

        assert_eq!(
            m0.tables,
            vec![TableType {
                limit: Limit::Range(3, 3)
            }]
        );

        assert_eq!(
            m0.element_items,
            vec![ElementItem {
                table_index: 0,
                offset_instruction_items: vec![Instruction::I32Const(0), Instruction::End],
                function_indices: vec![1, 1, 1]
            }]
        );

        assert_eq!(
            m0.code_items,
            vec![
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::Call(1),
                        Instruction::I32Const(2),
                        Instruction::CallIndirect(0, 0),
                        Instruction::End
                    ]
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![Instruction::I32Const(100), Instruction::End]
                }
            ]
        );
    }

    #[test]
    fn test_parse_instruction_branch() {
        let s0 = get_test_binary_resource("test-instruction-branch.wasm");
        let m0 = parse(&s0).unwrap();
        assert_eq!(
            m0.code_items,
            vec![
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::Block(BlockType::ResultEmpty, 0),
                        Instruction::I32Const(100),
                        Instruction::Br(0),
                        Instruction::I32Const(101),
                        Instruction::End,
                        Instruction::Loop(BlockType::ResultEmpty, 1),
                        Instruction::I32Const(200),
                        Instruction::Br(0),
                        Instruction::I32Const(201),
                        Instruction::End,
                        Instruction::I32Const(300),
                        Instruction::I32Eqz,
                        Instruction::If(BlockType::ResultEmpty, 2),
                        Instruction::I32Const(400),
                        Instruction::Br(0),
                        Instruction::I32Const(401),
                        Instruction::Else,
                        Instruction::I32Const(500),
                        Instruction::Br(0),
                        Instruction::I32Const(501),
                        Instruction::End,
                        Instruction::End
                    ]
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::Block(BlockType::ResultEmpty, 0),
                        Instruction::Block(BlockType::ResultEmpty, 1),
                        Instruction::Block(BlockType::ResultEmpty, 2),
                        Instruction::Br(1),
                        Instruction::I32Const(100),
                        Instruction::BrIf(2),
                        Instruction::BrTable(vec![0, 1, 2], 3),
                        Instruction::Return,
                        Instruction::End,
                        Instruction::End,
                        Instruction::End,
                        Instruction::End
                    ]
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![
                        Instruction::Block(BlockType::ResultI32, 0),
                        Instruction::I32Const(10),
                        Instruction::End,
                        Instruction::Block(BlockType::TypeIndex(1), 1),
                        Instruction::I32Const(20),
                        Instruction::End,
                        Instruction::End
                    ]
                }
            ]
        )
    }
}
