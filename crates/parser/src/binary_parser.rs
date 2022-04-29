// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{rc::Rc, vec};

use crate::{
    ast::{
        CodeItem, DataItem, ElementItem, ExportDescriptor, ExportItem, FunctionType, GlobalItem,
        GlobalType, ImportDescriptor, ImportItem, Limit, LocalGroup, MemoryType, Module, TableType,
    },
    instruction::{self, Instruction, MemoryArg},
    leb128decoder,
    types::ValueType,
};

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

pub fn parse(source: &[u8]) -> Result<Module, ParseError> {
    parse_module(source)
}

/// # 解析二进制 module
///
/// module = magic_number:u32 + version:u32 + <section>
fn parse_module(source: &[u8]) -> Result<Module, ParseError> {
    let mut remains = source;
    let (magic_number, post_magic_number) = read_fixed_u32(remains)?;
    if magic_number != MAGIC_NUMBER {
        return Err(ParseError::SyntaxError("invalid magic number".to_string()));
    }
    remains = post_magic_number;

    let (version, post_version) = read_fixed_u32(remains)?;
    if version != VERSION {
        return Err(ParseError::Unsupported(format!(
            "unsupported version: {}",
            version
        )));
    }
    remains = post_version;

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
        function_types: vec![],
        import_items: vec![],
        function_list: vec![],
        table_types: vec![],
        memory_blocks: vec![],
        global_items: vec![],
        export_items: vec![],
        start_function_index: None,
        element_items: vec![],
        code_items: vec![],
        data_items: vec![],
    };

    let mut remains = source;

    loop {
        if let Some((section_id, rest)) = remains.split_first() {
            remains = match *section_id {
                SECTION_CUSTOM_ID => {
                    let post_section = parse_custom_section(rest)?;
                    post_section
                }
                SECTION_TYPE_ID => {
                    let (function_types, post_section) = parse_function_type_section(rest)?;
                    module.function_types = function_types;
                    post_section
                }
                SECTION_IMPORT_ID => {
                    let (import_items, post_section) = parse_import_section(rest)?;
                    module.import_items = import_items;
                    post_section
                }
                SECTION_FUNCTION_ID => {
                    let (function_list, post_section) = parse_function_list_section(rest)?;
                    module.function_list = function_list;
                    post_section
                }
                SECTION_TABLE_ID => {
                    let (table_types, post_section) = parse_table_section(rest)?;
                    module.table_types = table_types;
                    post_section
                }
                SECTION_MEMORY_ID => {
                    let (memory_blocks, post_section) = parse_memory_section(rest)?;
                    module.memory_blocks = memory_blocks;
                    post_section
                }
                SECTION_GLOBAL_ID => {
                    let (global_items, post_section) = parse_global_section(rest)?;
                    module.global_items = global_items;
                    post_section
                }
                SECTION_EXPORT_ID => {
                    let (export_items, post_section) = parse_export_section(rest)?;
                    module.export_items = export_items;
                    post_section
                }
                SECTION_START_ID => {
                    let (function_index, post_section) = parse_start_function_section(rest)?;
                    module.start_function_index = Some(function_index);
                    post_section
                }
                SECTION_ELEMENT_ID => {
                    let (element_items, post_section) = parse_element_section(rest)?;
                    module.element_items = element_items;
                    post_section
                }
                SECTION_CODE_ID => {
                    let (code_items, post_section) = parse_function_code_section(rest)?;
                    module.code_items = code_items;
                    post_section
                }
                SECTION_DATA_ID => {
                    let (data_items, post_section) = parse_data_section(rest)?;
                    module.data_items = data_items;
                    post_section
                }
                _ => {
                    return Err(ParseError::SyntaxError(format!(
                        "invalid section id: {}",
                        section_id
                    )))
                }
            }
        } else {
            break;
        }
    }

    Ok(module)
}

/// # 解析自定义段
///
/// custom_section = section_id:u8 + section_length:u32 + name + data
/// name = byte_length:u32 + byte{*}
///
/// 自定义段的内容一般是记录函数的名称、局部变量的名称等信息，
/// 不影响程序的运算过程，所以也可以直接忽略。
fn parse_custom_section(source: &[u8]) -> Result<&[u8], ParseError> {
    let (section_length, post_section_length) = read_u32(source)?;
    // 暂时不解析自定义段的内容
    Ok(&post_section_length[section_length as usize..])
}

/// # 解析（函数）类型段
///
/// type_section = 0x01:byte + content_length:u32 + <function_type>
/// function_type = 0x60 + <value_type> + <value_type>
///                      ^
///                      |--- 目前 `类型项` 只支持函数类型， `0x60` 表示函数类型
fn parse_function_type_section(source: &[u8]) -> Result<(Vec<FunctionType>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    let (item_count, post_item_count) = read_u32(post_section_length)?;

    let mut remains = post_item_count;
    let mut function_types = Vec::<FunctionType>::with_capacity(item_count as usize);
    for _ in 0..item_count {
        let (function_type, post_function_type) = continue_parse_function_type(remains)?;
        function_types.push(function_type);
        remains = post_function_type;
    }
    Ok((function_types, remains))
}

/// function_type = 0x60 + <value_type> + <value_type>
///                      ^
///                      |--- 目前 `类型项` 只支持函数类型， `0x60` 表示函数类型
fn continue_parse_function_type(source: &[u8]) -> Result<(FunctionType, &[u8]), ParseError> {
    let (tag, post_tag) = read_byte(source)?;
    if tag != FUNCTION_TYPE_TAG {
        return Err(ParseError::Unsupported(
            "only type of function is supported".to_string(),
        ));
    }

    let (param_types, post_param_types) = continue_parse_value_types(post_tag)?;
    let (result_types, post_result_types) = continue_parse_value_types(post_param_types)?;

    let function_type = FunctionType {
        params: param_types,
        results: result_types,
    };

    Ok((function_type, post_result_types))
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
/// import_description = tag:byte + (function_type_index | table_type | memory_type | global_type)
fn parse_import_section(source: &[u8]) -> Result<(Vec<ImportItem>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    let (item_count, post_item_count) = read_u32(post_section_length)?;

    let mut remains = post_item_count;
    let mut import_items = Vec::<ImportItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (import_item, post_import_item) = continue_parse_import_item(remains)?;
        import_items.push(import_item);
        remains = post_import_item;
    }

    Ok((import_items, remains))
}

/// import_item = module_name:string + member_name:string + import_descriptor
/// import_description = tag:byte + (function_type_index | table_type | memory_type | global_type)
fn continue_parse_import_item(source: &[u8]) -> Result<(ImportItem, &[u8]), ParseError> {
    let (module_name, post_module_name) = read_string(source)?;
    let (name, post_name) = read_string(post_module_name)?;

    let (tag, post_tag) = read_byte(post_name)?;
    let mut remains = post_tag;

    let import_descriptor = match tag {
        IMPORT_TAG_FUNCTION => {
            let (function_type_index, post_function_type_index) = read_u32(remains)?;
            remains = post_function_type_index;
            ImportDescriptor::FunctionTypeIndex(function_type_index)
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
        module_name: module_name,
        name: name,
        import_descriptor: import_descriptor,
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
            "only function reference is supported in the table".to_string(),
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
/// function_section = 0x03 + content_length:u32 + <function_type_index>
/// function_type_index = u32
fn parse_function_list_section(source: &[u8]) -> Result<(Vec<u32>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    read_u32_vec(post_section_length)
}

/// # 解析表段
///
/// table_section = 0x04 + content_length:u32 + <table_type> // 目前一个模块仅支持声明一个表项
/// table_type = 0x70 + limits
///              ^
///              |--- 0x70 表示该表项存储的是 funcref
fn parse_table_section(source: &[u8]) -> Result<(Vec<TableType>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    let (item_count, post_item_count) = read_u32(post_section_length)?;

    if item_count > 1 {
        return Err(ParseError::Unsupported(
            "only one table is supported".to_string(),
        ));
    }

    let mut remains = post_item_count;
    let mut table_types = Vec::<TableType>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (table_type, post_table_type) = continue_parse_table_type(remains)?;
        table_types.push(table_type);
        remains = post_table_type;
    }

    Ok((table_types, remains))
}

/// # 解析内存段
///
/// memory_section = 0x05 + content_length:u32 + <memory_type> // 目前一个模块仅支持声明一个内存块
/// memory_type = limits
fn parse_memory_section(source: &[u8]) -> Result<(Vec<MemoryType>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    let (item_count, post_item_count) = read_u32(post_section_length)?;

    if item_count > 1 {
        return Err(ParseError::Unsupported(
            "only one memory block is supported".to_string(),
        ));
    }

    let mut remains = post_item_count;
    let mut memory_types = Vec::<MemoryType>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (memory_type, post_memory_type) = continue_parse_memory_type(remains)?;
        memory_types.push(memory_type);
        remains = post_memory_type;
    }

    Ok((memory_types, remains))
}

/// # 解析全局（变量）段
///
/// global_section = 0x06 + content_length:u32 + <global_item>
/// global_item = global_type + initialize_expression
/// global_type = val_type:byte + mut:byte
/// mut = (0|1)                             // 0 表示不可变，1 表示可变
/// initialize_expression = byte{*} + 0x0B  // 表达式/字节码以 0x0B 结尾
fn parse_global_section(source: &[u8]) -> Result<(Vec<GlobalItem>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    let (item_count, post_item_count) = read_u32(post_section_length)?;

    let mut remains = post_item_count;
    let mut global_items = Vec::<GlobalItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (global_type, post_global_type) = continue_parse_global_type(remains)?;
        let (initialize_expression, post_expression) = continue_parse_expression(post_global_type)?;

        global_items.push(GlobalItem {
            global_type: global_type,
            init_expression: initialize_expression,
        });

        remains = post_expression;
    }

    Ok((global_items, remains))
}

/// # 解析指令表达式
///
/// 指令表达式一般用在 `全局项的初始值` 以及 `数据项的偏移值`。
/// 指令表达式一般由一个 `const` 指令加一个 `end` 指令构成。
fn continue_parse_expression(source: &[u8]) -> Result<(Vec<Instruction>, &[u8]), ParseError> {
    continue_parse_instructions_until_opcode_end(source)
}

/// # 解析指令序列
///
/// 不断解析下一个指令，直到遇到 `end` 指令为止
/// 注意解析指令序列过程中，如果遇到结构块，会进入递归解析，
/// 所以这个 `end` 指令必然是完成的一个结构的结束指令。
fn continue_parse_instructions_until_opcode_end(
    source: &[u8],
) -> Result<(Vec<Instruction>, &[u8]), ParseError> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut remains = source;

    loop {
        let (instruction, post_instruction) = continue_parse_instruction(remains)?;
        let found_end = instruction == Instruction::End;
        instructions.push(instruction);
        remains = post_instruction;

        if found_end {
            break;
        }
    }

    Ok((instructions, remains))
}

/// # 解析指令序列
///
/// 不断解析下一个指令，直到遇到 `else` 指令或者 `end` 指令为止。
/// 这个解析过程是专门为解析 `if` 指令使用。
///
/// 注意解析指令序列过程中，如果遇到结构块，会进入递归解析，
/// 所以找到的 `else` 或者 `end` 指令必然是完成的一个结构的结束指令。
fn continue_parse_instructions_until_opcode_else_or_end(
    source: &[u8],
) -> Result<(Vec<Instruction>, bool, &[u8]), ParseError> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut remains = source;

    let is_else = loop {
        let (instruction, post_instruction) = continue_parse_instruction(remains)?;
        let found_end = instruction == Instruction::End;
        let found_else = instruction == Instruction::Else;
        instructions.push(instruction);
        remains = post_instruction;

        if found_end || found_else {
            break found_else;
        }
    };

    Ok((instructions, is_else, remains))
}

fn continue_parse_instruction(source: &[u8]) -> Result<(Instruction, &[u8]), ParseError> {
    let mut remains = source;

    let (opcode, post_opcode) = read_byte(remains)?;
    remains = post_opcode;

    let instruction = match opcode {
        // 控制指令
        instruction::UNREACHABLE => Instruction::Unreachable,
        instruction::NOP => Instruction::Nop,
        instruction::BLOCK => {
            // block = opcode_block + block_type:i32 + instructions + opcode_end
            let (block_type, post_block_type) = read_i32(remains)?;
            let option_value_type = get_value_type_by_block_type(block_type)?;
            let (body, post_body) = continue_parse_instructions_until_opcode_end(post_block_type)?;
            remains = post_body;
            Instruction::Block {
                result: option_value_type,
                body: Rc::new(body),
            }
        }
        instruction::LOOP => {
            // loop = opcode_loop + block_type:i32 + instructions + opcode_end
            let (block_type, post_block_type) = read_i32(remains)?;
            let option_value_type = get_value_type_by_block_type(block_type)?;
            let (body, post_body) = continue_parse_instructions_until_opcode_end(post_block_type)?;
            remains = post_body;
            Instruction::Loop {
                result: option_value_type,
                body: Rc::new(body),
            }
        }
        instruction::IF => {
            // if = opcode_if + block_type:i32 + (instructions + opcode_else){?} + instructions + opcode_end
            let (block_type, post_block_type) = read_i32(remains)?;
            let option_value_type = get_value_type_by_block_type(block_type)?;
            let (consequent_body, is_else, post_then_body) =
                continue_parse_instructions_until_opcode_else_or_end(post_block_type)?;
            let (alternate_body, post_else_body) = if is_else {
                continue_parse_instructions_until_opcode_end(post_then_body)?
            } else {
                (vec![], post_then_body)
            };
            remains = post_else_body;
            Instruction::If {
                result: option_value_type,
                consequet_body: Rc::new(consequent_body),
                alternate_body: Rc::new(alternate_body),
            }
        }
        instruction::ELSE => Instruction::Else,
        instruction::END => Instruction::End,
        instruction::BR => {
            // br = opcode_br + relative_depth:u32
            let (relative_depth, post_relative_depth) = read_u32(remains)?;
            remains = post_relative_depth;
            Instruction::Br(relative_depth)
        }
        instruction::BR_IF => {
            // br_if = opcode_br_if + relative_depth:u32
            let (relative_depth, post_relative_depth) = read_u32(remains)?;
            remains = post_relative_depth;
            Instruction::BrIf(relative_depth)
        }
        instruction::BR_TABLE => {
            // br_table = opcode_br_table + <relative_depth> + relative_depth:u32
            let (branch_relative_depths, post_branch) = read_u32_vec(remains)?;
            let (default_relative_depth, post_default) = read_u32(post_branch)?;
            remains = post_default;
            Instruction::BrTable {
                relative_depths: branch_relative_depths,
                default_relative_depth,
            }
        }
        instruction::RETURN => Instruction::Return,
        instruction::CALL => {
            // call = opcode_call + function_index:u32
            let (function_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::Call(function_index)
        }
        instruction::CALL_INDIRECT => {
            let (function_index, post_function_index) = read_u32(remains)?;
            remains = consume_zero(post_function_index)?;
            Instruction::CallIndirect(function_index)
        }
        instruction::DROP => Instruction::Drop,
        instruction::SELECT => Instruction::Select,

        // 变量指令
        instruction::LOCAL_GET => {
            // local.get = opcode_local.get + local_index:u32
            let (local_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::LocalGet(local_index)
        }
        instruction::LOCAL_SET => {
            // local.set = opcode_local.set + local_index:u32
            let (local_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::LocalSet(local_index)
        }
        instruction::LOCAL_TEE => {
            // local.tee = opcode_local.tee + local_index:u32
            let (local_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::LocalTee(local_index)
        }
        instruction::GLOBAL_GET => {
            // global.get = opcode_global.get + global_index:u32
            let (global_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::GlobalGet(global_index)
        }
        instruction::GLOBAL_SET => {
            // global.set = opcode_global.set + global_index:u32
            let (global_index, post_index) = read_u32(remains)?;
            remains = post_index;
            Instruction::GlobalSet(global_index)
        }

        // 内存指令
        instruction::I32_LOAD => {
            // i32.load = opcode_i32.load + memory_arg
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Load(memory_arg)
        }
        instruction::I64_LOAD => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Load(memory_arg)
        }
        instruction::F32_LOAD => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::F32Load(memory_arg)
        }
        instruction::F64_LOAD => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::F64Load(memory_arg)
        }
        instruction::I32_LOAD8_S => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Load8S(memory_arg)
        }
        instruction::I32_LOAD8_U => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Load8U(memory_arg)
        }
        instruction::I32_LOAD16_S => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Load16S(memory_arg)
        }
        instruction::I32_LOAD16_U => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Load16U(memory_arg)
        }
        instruction::I64_LOAD8_S => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Load8S(memory_arg)
        }
        instruction::I64_LOAD8_U => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Load8U(memory_arg)
        }
        instruction::I64_LOAD16_S => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Load16S(memory_arg)
        }
        instruction::I64_LOAD16_U => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Load16U(memory_arg)
        }
        instruction::I64_LOAD32_S => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Load32S(memory_arg)
        }
        instruction::I64_LOAD32_U => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Load32U(memory_arg)
        }
        instruction::I32_STORE => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Store(memory_arg)
        }
        instruction::I64_STORE => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Store(memory_arg)
        }
        instruction::F32_STORE => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::F32Store(memory_arg)
        }
        instruction::F64_STORE => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::F64Store(memory_arg)
        }
        instruction::I32_STORE8 => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Store8(memory_arg)
        }
        instruction::I32_STORE16 => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I32Store16(memory_arg)
        }
        instruction::I64_STORE8 => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Store8(memory_arg)
        }
        instruction::I64_STORE16 => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Store16(memory_arg)
        }
        instruction::I64_STORE32 => {
            let (memory_arg, post_memory_arg) =
                continue_parse_memory_load_and_store_argument(remains)?;
            remains = post_memory_arg;
            Instruction::I64Store32(memory_arg)
        }
        instruction::MEMORY_SIZE => {
            // memory.size = opcode_memory.size + memory_block_index:u32
            remains = consume_zero(remains)?;
            Instruction::MemorySize(0)
        }
        instruction::MEMORY_GROW => {
            // memory.grow = opcode_memory.grow + memory_block_index:u32
            remains = consume_zero(remains)?;
            Instruction::MemoryGrow(0)
        }

        // 常量指令
        instruction::I32_CONST => {
            // i32.const = opcode_i32.const + number:(signed)i32
            let (number, post_number) = read_i32(remains)?;
            remains = post_number;
            Instruction::I32Const(number)
        }
        instruction::I64_CONST => {
            // i64.const = opcode_i64.const + number:(signed)i64
            let (number, post_number) = read_i64(remains)?;
            remains = post_number;
            Instruction::I64Const(number)
        }
        instruction::F32_CONST => {
            // f32.const = opcode_f32.const + number:(signed)f32
            let (number, post_number) = read_f32(remains)?;
            remains = post_number;
            Instruction::F32Const(number)
        }
        instruction::F64_CONST => {
            // f64.const = opcode_f64.const + number:(signed)f64
            let (number, post_number) = read_f64(remains)?;
            remains = post_number;
            Instruction::F64Const(number)
        }

        // 数值指令
        instruction::I32_EQZ => Instruction::I32Eqz,
        instruction::I32_EQ => Instruction::I32Eq,
        instruction::I32_NE => Instruction::I32Ne,
        instruction::I32_LT_S => Instruction::I32LtS,
        instruction::I32_LT_U => Instruction::I32LtU,
        instruction::I32_GT_S => Instruction::I32GtS,
        instruction::I32_GT_U => Instruction::I32GtU,
        instruction::I32_LE_S => Instruction::I32LeS,
        instruction::I32_LE_U => Instruction::I32LeU,
        instruction::I32_GE_S => Instruction::I32GeS,
        instruction::I32_GE_U => Instruction::I32GeU,
        instruction::I64_EQZ => Instruction::I64Eqz,
        instruction::I64_EQ => Instruction::I64Eq,
        instruction::I64_NE => Instruction::I64Ne,
        instruction::I64_LT_S => Instruction::I64LtS,
        instruction::I64_LT_U => Instruction::I64LtU,
        instruction::I64_GT_S => Instruction::I64GtS,
        instruction::I64_GT_U => Instruction::I64GtU,
        instruction::I64_LE_S => Instruction::I64LeS,
        instruction::I64_LE_U => Instruction::I64LeU,
        instruction::I64_GE_S => Instruction::I64GeS,
        instruction::I64_GE_U => Instruction::I64GeU,
        instruction::F32_EQ => Instruction::F32Eq,
        instruction::F32_NE => Instruction::F32Ne,
        instruction::F32_LT => Instruction::F32Lt,
        instruction::F32_GT => Instruction::F32Gt,
        instruction::F32_LE => Instruction::F32Le,
        instruction::F32_GE => Instruction::F32Ge,
        instruction::F64_EQ => Instruction::F64Eq,
        instruction::F64_NE => Instruction::F64Ne,
        instruction::F64_LT => Instruction::F64Lt,
        instruction::F64_GT => Instruction::F64Gt,
        instruction::F64_LE => Instruction::F64Le,
        instruction::F64_GE => Instruction::F64Ge,
        instruction::I32_CLZ => Instruction::I32Clz,
        instruction::I32_CTZ => Instruction::I32Ctz,
        instruction::I32_POP_CNT => Instruction::I32PopCnt,
        instruction::I32_ADD => Instruction::I32Add,
        instruction::I32_SUB => Instruction::I32Sub,
        instruction::I32_MUL => Instruction::I32Mul,
        instruction::I32_DIV_S => Instruction::I32DivS,
        instruction::I32_DIV_U => Instruction::I32DivU,
        instruction::I32_REM_S => Instruction::I32RemS,
        instruction::I32_REM_U => Instruction::I32RemU,
        instruction::I32_AND => Instruction::I32And,
        instruction::I32_OR => Instruction::I32Or,
        instruction::I32_XOR => Instruction::I32Xor,
        instruction::I32_SHL => Instruction::I32Shl,
        instruction::I32_SHR_S => Instruction::I32ShrS,
        instruction::I32_SHR_U => Instruction::I32ShrU,
        instruction::I32_ROTL => Instruction::I32Rotl,
        instruction::I32_ROTR => Instruction::I32Rotr,
        instruction::I64_CLZ => Instruction::I64Clz,
        instruction::I64_CTZ => Instruction::I64Ctz,
        instruction::I64_POP_CNT => Instruction::I64PopCnt,
        instruction::I64_ADD => Instruction::I64Add,
        instruction::I64_SUB => Instruction::I64Sub,
        instruction::I64_MUL => Instruction::I64Mul,
        instruction::I64_DIV_S => Instruction::I64DivS,
        instruction::I64_DIV_U => Instruction::I64DivU,
        instruction::I64_REM_S => Instruction::I64RemS,
        instruction::I64_REM_U => Instruction::I64RemU,
        instruction::I64_AND => Instruction::I64And,
        instruction::I64_OR => Instruction::I64Or,
        instruction::I64_XOR => Instruction::I64Xor,
        instruction::I64_SHL => Instruction::I64Shl,
        instruction::I64_SHR_S => Instruction::I64ShrS,
        instruction::I64_SHR_U => Instruction::I64ShrU,
        instruction::I64_ROTL => Instruction::I64Rotl,
        instruction::I64_ROTR => Instruction::I64Rotr,
        instruction::F32_ABS => Instruction::F32Abs,
        instruction::F32_NEG => Instruction::F32Neg,
        instruction::F32_CEIL => Instruction::F32Ceil,
        instruction::F32_FLOOR => Instruction::F32Floor,
        instruction::F32_TRUNC => Instruction::F32Trunc,
        instruction::F32_NEAREST => Instruction::F32Nearest,
        instruction::F32_SQRT => Instruction::F32Sqrt,
        instruction::F32_ADD => Instruction::F32Add,
        instruction::F32_SUB => Instruction::F32Sub,
        instruction::F32_MUL => Instruction::F32Mul,
        instruction::F32_DIV => Instruction::F32Div,
        instruction::F32_MIN => Instruction::F32Min,
        instruction::F32_MAX => Instruction::F32Max,
        instruction::F32_COPY_SIGN => Instruction::F32CopySign,
        instruction::F64_ABS => Instruction::F64Abs,
        instruction::F64_NEG => Instruction::F64Neg,
        instruction::F64_CEIL => Instruction::F64Ceil,
        instruction::F64_FLOOR => Instruction::F64Floor,
        instruction::F64_TRUNC => Instruction::F64Trunc,
        instruction::F64_NEAREST => Instruction::F64Nearest,
        instruction::F64_SQRT => Instruction::F64Sqrt,
        instruction::F64_ADD => Instruction::F64Add,
        instruction::F64_SUB => Instruction::F64Sub,
        instruction::F64_MUL => Instruction::F64Mul,
        instruction::F64_DIV => Instruction::F64Div,
        instruction::F64_MIN => Instruction::F64Min,
        instruction::F64_MAX => Instruction::F64Max,
        instruction::F64_COPY_SIGN => Instruction::F64CopySign,
        instruction::I32_WRAP_I64 => Instruction::I32WrapI64,
        instruction::I32_TRUNC_F32_S => Instruction::I32TruncF32S,
        instruction::I32_TRUNC_F32_U => Instruction::I32TruncF32U,
        instruction::I32_TRUNC_F64_S => Instruction::I32TruncF64S,
        instruction::I32_TRUNC_F64_U => Instruction::I32TruncF64U,
        instruction::I64_EXTEND_I32_S => Instruction::I64ExtendI32S,
        instruction::I64_EXTEND_I32_U => Instruction::I64ExtendI32U,
        instruction::I64_TRUNC_F32_S => Instruction::I64TruncF32S,
        instruction::I64_TRUNC_F32_U => Instruction::I64TruncF32U,
        instruction::I64_TRUNC_F64_S => Instruction::I64TruncF64S,
        instruction::I64_TRUNC_F64_U => Instruction::I64TruncF64U,
        instruction::F32_CONVERT_I32_S => Instruction::F32ConvertI32S,
        instruction::F32_CONVERT_I32_U => Instruction::F32ConvertI32U,
        instruction::F32_CONVERT_I64_S => Instruction::F32ConvertI64S,
        instruction::F32_CONVERT_I64_U => Instruction::F32ConvertI64U,
        instruction::F32_DEMOTE_F64 => Instruction::F32DemoteF64,
        instruction::F64_CONVERT_I32_S => Instruction::F64ConvertI32S,
        instruction::F64_CONVERT_I32_U => Instruction::F64ConvertI32U,
        instruction::F64_CONVERT_I64_S => Instruction::F64ConvertI64S,
        instruction::F64_CONVERT_I64_U => Instruction::F64ConvertI64U,
        instruction::F64_PROMOTE_F32 => Instruction::F64PromoteF32,
        instruction::I32_REINTERPRET_F32 => Instruction::I32ReinterpretF32,
        instruction::I64_REINTERPRET_F64 => Instruction::I64ReinterpretF64,
        instruction::F32_REINTERPRET_I32 => Instruction::F32ReinterpretI32,
        instruction::F64_REINTERPRET_I64 => Instruction::F64ReinterpretI64,
        instruction::I32_EXTEND8_S => Instruction::I32Extend8S,
        instruction::I32_EXTEND16_S => Instruction::I32Extend16S,
        instruction::I64_EXTEND8_S => Instruction::I64Extend8S,
        instruction::I64_EXTEND16_S => Instruction::I64Extend16S,
        instruction::I64_EXTEND32_S => Instruction::I64Extend32S,
        instruction::TRUNC_SAT => {
            // trunc_sat = opcode_trunc_sat:byte + opcode_trunc_sat_sub_opcode:byte
            let (sub_opcode, post_sub_opcode) = read_byte(remains)?;

            if sub_opcode > instruction::I64_TRUNC_SAT_F64_U {
                return Err(ParseError::Unsupported(format!(
                    "unsupported trunc_sat sub-opcode: {}",
                    sub_opcode
                )));
            }

            remains = post_sub_opcode;
            Instruction::TruncSat(sub_opcode)
        }
        _ => {
            return Err(ParseError::Unsupported(format!(
                "unsupported instruction opcode: {}",
                opcode
            )));
        }
    };

    Ok((instruction, remains))
}

/// 将结构块的返回类型转换为函数返回类型
/// 因为在解析器的实现时，结构块将会当作一种简化的函数来处理
///
/// block_type = (signed) i32
fn get_value_type_by_block_type(block_type: i32) -> Result<Option<ValueType>, ParseError> {
    match block_type {
        BLOCK_TYPE_I32 => Ok(Some(ValueType::I32)),
        BLOCK_TYPE_I64 => Ok(Some(ValueType::I64)),
        BLOCK_TYPE_F32 => Ok(Some(ValueType::F32)),
        BLOCK_TYPE_F64 => Ok(Some(ValueType::F64)),
        BLOCK_TYPE_EMPTY => Ok(None),
        _ => Err(ParseError::Unsupported(format!(
            "unsupported block type: {}",
            block_type
        ))),
    }
}

/// 解析内存数据加载和存储指令的参数
///
/// 参数有两个：
///
/// 第一个是 `align`，即对齐方式，数据类型是整数，单位是 `字节`，
/// 二进制格式里 `align` 存储的是 2 的对数，比如：
/// 1 表示对齐 2^1 个字节，
/// 2 表示对齐 2^2 个字节。
///
/// 注意文本格式里就是字节数，比如文本格式的 8 对应二进制格式的 3 (2^3)。
///
/// 第二个是 `offset`，即偏移值，单位是 `字节`。
///
/// 二进制格式：
///
/// memory_load_and_store_argument = align:u32 + offset:u32
///
/// 文本格式：
/// (i32.load offset=200 align=8) ;; 注意先写 offset 后写 align
///
/// 在文本格式里缺省 `align` 值时，
/// 对于 i32.load/i32.store，默认对齐 4 个字节
/// 对于 i64.load/i64.store，默认对齐 8 个字节
fn continue_parse_memory_load_and_store_argument(
    source: &[u8],
) -> Result<(MemoryArg, &[u8]), ParseError> {
    let (align, post_align) = read_u32(source)?;
    let (offset, post_offset) = read_u32(post_align)?;
    Ok((MemoryArg { align, offset }, post_offset))
}

/// # 解析导出段
///
/// export_section = 0x07 + content_length:u32 + <export_item>
/// export_item = name:string + export_descriptor
/// export_descriptor = tag:byte + (function_index | table_index | memory_block_index | global_item_index)
fn parse_export_section(source: &[u8]) -> Result<(Vec<ExportItem>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;

    let (item_count, post_item_count) = read_u32(post_section_length)?;

    let mut remains = post_item_count;
    let mut export_items = Vec::<ExportItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (export_item, post_export_item) = continue_parse_export_item(remains)?;
        export_items.push(export_item);
        remains = post_export_item;
    }

    Ok((export_items, remains))
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

    Ok((
        ExportItem {
            name,
            export_descriptor,
        },
        post_index,
    ))
}

/// # 解析起始函数索引段
///
/// start_section: 0x08 + content_length:u32 + function_index
fn parse_start_function_section(source: &[u8]) -> Result<(u32, &[u8]), ParseError> {
    let (_, post_section_length) = read_u32(source)?;
    let (function_index, post_function_index) = read_u32(post_section_length)?;
    Ok((function_index as u32, post_function_index))
}

/// # 解析元素段
///
/// element_section = 0x09 + content_length:u32 + <element_item>
/// element_item = table_index:u32 + offset_expression + <function_index>
/// offset_expression = byte{*} + 0x0B  // 表达式/字节码以 0x0B 结尾
fn parse_element_section(source: &[u8]) -> Result<(Vec<ElementItem>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    let (item_count, post_item_count) = read_u32(post_section_length)?;

    let mut remains = post_item_count;
    let mut element_items = Vec::<ElementItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (element_item, post_element_item) = continue_parse_element_item(remains)?;
        element_items.push(element_item);
        remains = post_element_item;
    }

    Ok((element_items, remains))
}

/// element_item = table_index:u32 + offset_expression + <function_index>
/// offset_expression = byte{*} + 0x0B  // 表达式/字节码以 0x0B 结尾
fn continue_parse_element_item(source: &[u8]) -> Result<(ElementItem, &[u8]), ParseError> {
    let remains = consume_zero(source)?; // table index 总是为 0
    let (offset_expression, post_expression) = continue_parse_expression(remains)?;
    let (function_indices, post_indices) = read_u32_vec(post_expression)?;

    Ok((
        ElementItem {
            table_index: 0,
            offset_expression,
            function_indices,
        },
        post_indices,
    ))
}

/// # 解析代码段
///
/// code_section = 0x0a + content_length:u32 + <code_item>
/// code_item = code_length:u32 + <local_group> + expression
/// local_group = local_variable_count:u32 + value_type:byte
/// expression = byte{*} + 0x0B  // 表达式/字节码以 0x0B 结尾
///
/// code_length 表示该项目的内容总大小，包括表达式结尾的 0x0B。
fn parse_function_code_section(source: &[u8]) -> Result<(Vec<CodeItem>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;

    let (item_count, post_item_count) = read_u32(post_section_length)?;

    let mut remains = post_item_count;
    let mut code_items = Vec::<CodeItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (code_item, post_code_item) = continue_parse_code_item(remains)?;
        code_items.push(code_item);
        remains = post_code_item;
    }

    Ok((code_items, remains))
}

/// code_item = code_length:u32 + <local_group> + expression
/// local_group = local_variable_count:u32 + value_type:byte
/// expression = byte{*} + 0x0B  // 表达式/字节码以 0x0B 结尾
fn continue_parse_code_item(source: &[u8]) -> Result<(CodeItem, &[u8]), ParseError> {
    let (_code_length, post_code_length) = read_u32(source)?;

    let (local_group_count, post_local_group_count) = read_u32(post_code_length)?;

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

    let (expression, post_expression) = continue_parse_expression(remains)?;

    Ok((
        CodeItem {
            local_groups,
            expression: Rc::new(expression),
        },
        post_expression,
    ))
}

/// # 解析数据段
///
/// data_section = 0x0b + content_length:u32 + <data_item>
/// data_item = memory_block_index:u32 + offset_expression + data:byte{*}
/// offset_expression = = byte{*} + 0x0B  // 表达式/字节码以 0x0B 结尾
fn parse_data_section(source: &[u8]) -> Result<(Vec<DataItem>, &[u8]), ParseError> {
    let (_section_length, post_section_length) = read_u32(source)?;
    let (item_count, post_item_count) = read_u32(post_section_length)?;

    let mut remains = post_item_count;
    let mut data_items = Vec::<DataItem>::with_capacity(item_count as usize);

    for _ in 0..item_count {
        let (data_item, post_data_item) = continue_parse_data_item(remains)?;
        data_items.push(data_item);
        remains = post_data_item;
    }

    Ok((data_items, remains))
}

/// data_item = memory_block_index:u32 + offset_expression + data:byte{*}
/// offset_expression = = byte{*} + 0x0B  // 表达式/字节码以 0x0B 结尾
fn continue_parse_data_item(source: &[u8]) -> Result<(DataItem, &[u8]), ParseError> {
    let remains = consume_zero(source)?; // memory index 总是为 0
    let (offset_expression, post_expression) = continue_parse_expression(remains)?;
    let (data, post_data) = read_byte_vec(post_expression)?;

    Ok((
        DataItem {
            memory_index: 0,
            offset_expression,
            data,
        },
        post_data,
    ))
}

#[derive(Debug)]
pub enum ParseError {
    //Something(&'static str),
    /// 语法错误
    /// 比如不符合规范的数值
    SyntaxError(String), //&'static str),

    /// 不支持的功能
    /// 比如读取索引值为非 0 的内存块或者表
    Unsupported(String), //&'static str),

    /// leb128 编码或者 UTF-8 编码错误
    DecodingError,

    /// 未预料的结束，即预期的内容不完整
    /// 比如解析一个函数时，尚未到达末尾源文件就已经到末尾了。
    UnexpectedEnd,
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

fn read_n_bytes(source: &[u8], length: usize) -> Result<(&[u8], &[u8]), ParseError> {
    if length > source.len() {
        Err(ParseError::UnexpectedEnd)
    } else {
        Ok(source.split_at(length))
    }
}

fn read_4_bytes(source: &[u8]) -> Result<([u8; 4], &[u8]), ParseError> {
    let (bytes, remains) = read_n_bytes(source, 4)?;

    let mut buf: [u8; 4] = [0; 4];
    bytes
        .iter()
        .enumerate()
        .for_each(|(index, value)| buf[index] = *value);
    Ok((buf, remains))
}

fn read_8_bytes(source: &[u8]) -> Result<([u8; 8], &[u8]), ParseError> {
    let (bytes, remains) = read_n_bytes(source, 8)?;

    let mut buf: [u8; 8] = [0; 8];
    bytes
        .iter()
        .enumerate()
        .for_each(|(index, value)| buf[index] = *value);
    Ok((buf, remains))
}

/// 读取固定长度（4 字节）u32
fn read_fixed_u32(source: &[u8]) -> Result<(u32, &[u8]), ParseError> {
    let (bytes, remains) = read_4_bytes(source)?;
    Ok((u32::from_le_bytes(bytes), remains))
}

/// 读取变长（leb128 编码的）u32
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
fn read_i64(source: &[u8]) -> Result<(i64, &[u8]), ParseError> {
    match leb128decoder::decode_i64(source) {
        Ok((value, length)) => Ok((value, &source[length..])),
        _ => Err(ParseError::DecodingError),
    }
}

/// 读取固定长度的 f32
fn read_f32(source: &[u8]) -> Result<(f32, &[u8]), ParseError> {
    let (bytes, remains) = read_4_bytes(source)?;
    Ok((f32::from_le_bytes(bytes), remains))
}

/// 读取固定长度的 f64
fn read_f64(source: &[u8]) -> Result<(f64, &[u8]), ParseError> {
    let (bytes, remains) = read_8_bytes(source)?;
    Ok((f64::from_le_bytes(bytes), remains))
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
    let (bytes, post_bytes) = read_n_bytes(post_length, length as usize)?;
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

/// 读取一个字节并断言其值为 0
/// 用于读取 memory/data 和 table/element 等需要指定目标对象索引值，
/// 但该索引值只能是 0 的场合
fn consume_zero(source: &[u8]) -> Result<&[u8], ParseError> {
    let (number, remains) = read_u32(source)?;
    if number != 0 {
        Err(ParseError::Unsupported("expected number zero".to_string()))
    } else {
        Ok(remains)
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs, rc::Rc, time::Instant};

    use crate::{
        ast::{
            CodeItem, DataItem, ElementItem, ExportDescriptor, ExportItem, FunctionType,
            GlobalItem, GlobalType, ImportDescriptor, ImportItem, Limit, LocalGroup, MemoryType,
            Module, TableType,
        },
        instruction::{self, Instruction, MemoryArg},
        types::ValueType,
    };

    use super::parse;
    use pretty_assertions::{assert_eq, assert_ne};

    // 辅助方法

    fn get_test_resource_file_binary(filename: &str) -> Vec<u8> {
        let mut path_buf = env::current_dir().expect("failed to get current directory");

        // 使用 `cargo test` 测试时，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm/crates/engine`；
        //
        // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm`。
        //
        // 这里需要处理这种情况。

        if path_buf.ends_with("parser") {
            path_buf.pop();
            path_buf.pop();
        }
        let fullname_buf = path_buf.join("test/resources/parser").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!("failed to read the specified file: {}", fullname))
    }

    #[test]
    fn test_parse_module_sections() {
        // 测试 test-section-1.wasm
        let s0 = get_test_resource_file_binary("test-section-1.wasm");
        let m0 = parse(&s0).unwrap();
        let e0 = Module {
            custom_items: vec![],

            function_types: vec![FunctionType {
                params: vec![],
                results: vec![ValueType::I32],
            }],
            import_items: vec![ImportItem {
                module_name: "env".to_string(),
                name: "__linear_memory".to_string(),
                import_descriptor: ImportDescriptor::MemoryType(MemoryType {
                    limit: Limit::AtLeast(0),
                }),
            }],
            function_list: vec![0],
            table_types: vec![],
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
                expression: Rc::new(vec![
                    Instruction::I32Const(100),
                    Instruction::LocalSet(0),
                    Instruction::LocalGet(0),
                    Instruction::Return,
                    Instruction::End,
                ]),
            }],
            data_items: vec![],
        };
        assert_eq!(e0, m0);

        // 测试 test-section-2.wasm
        let s1 = get_test_resource_file_binary("test-section-2.wasm");
        let m1 = parse(&s1).unwrap();
        let e1 = Module {
            custom_items: vec![],

            function_types: vec![
                FunctionType {
                    params: vec![ValueType::I32, ValueType::I32],
                    results: vec![ValueType::I32],
                },
                FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![ValueType::I32],
                },
                FunctionType {
                    params: vec![],
                    results: vec![],
                },
            ],
            import_items: vec![],
            function_list: vec![0, 0, 1, 2],
            table_types: vec![],
            memory_blocks: vec![MemoryType {
                limit: Limit::AtLeast(16),
            }],
            global_items: vec![
                GlobalItem {
                    global_type: GlobalType {
                        mutable: true,
                        value_type: ValueType::I32,
                    },
                    init_expression: vec![Instruction::I32Const(1048576), Instruction::End],
                },
                GlobalItem {
                    global_type: GlobalType {
                        mutable: false,
                        value_type: ValueType::I32,
                    },
                    init_expression: vec![Instruction::I32Const(1048576), Instruction::End],
                },
                GlobalItem {
                    global_type: GlobalType {
                        mutable: false,
                        value_type: ValueType::I32,
                    },
                    init_expression: vec![Instruction::I32Const(1048576), Instruction::End],
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
                    expression: Rc::new(vec![
                        Instruction::LocalGet(1),
                        Instruction::LocalGet(0),
                        Instruction::I32Add,
                        Instruction::End,
                    ]),
                },
                CodeItem {
                    local_groups: vec![],
                    expression: Rc::new(vec![
                        Instruction::LocalGet(0),
                        Instruction::LocalGet(1),
                        Instruction::I32Sub,
                        Instruction::End,
                    ]),
                },
                CodeItem {
                    local_groups: vec![],
                    expression: Rc::new(vec![
                        Instruction::LocalGet(0),
                        Instruction::I32Const(1),
                        Instruction::I32Add,
                        Instruction::End,
                    ]),
                },
                CodeItem {
                    local_groups: vec![],
                    expression: Rc::new(vec![Instruction::End]),
                },
            ],
            data_items: vec![],
        };
        assert_eq!(e1, m1);

        // 测试 test-section-3.wasm
        let s2 = get_test_resource_file_binary("test-section-3.wasm");
        let m2 = parse(&s2).unwrap();
        let e2 = Module {
            custom_items: vec![],

            function_types: vec![
                FunctionType {
                    params: vec![],
                    results: vec![ValueType::I32],
                },
                FunctionType {
                    params: vec![],
                    results: vec![],
                },
            ],
            import_items: vec![
                ImportItem {
                    module_name: "env".to_string(),
                    name: "putc".to_string(),
                    import_descriptor: ImportDescriptor::FunctionTypeIndex(0),
                },
                ImportItem {
                    module_name: "env".to_string(),
                    name: "print".to_string(),
                    import_descriptor: ImportDescriptor::FunctionTypeIndex(0),
                },
            ],
            function_list: vec![1, 1],
            table_types: vec![TableType {
                limit: Limit::Range(2, 4),
            }],
            memory_blocks: vec![MemoryType {
                limit: Limit::Range(1, 8),
            }],
            global_items: vec![],
            export_items: vec![],
            start_function_index: Some(3),
            element_items: vec![
                ElementItem {
                    table_index: 0,
                    offset_expression: vec![Instruction::I32Const(1), Instruction::End],
                    function_indices: vec![2],
                },
                ElementItem {
                    table_index: 0,
                    offset_expression: vec![Instruction::I32Const(3), Instruction::End],
                    function_indices: vec![3],
                },
            ],
            code_items: vec![
                CodeItem {
                    local_groups: vec![],
                    expression: Rc::new(vec![
                        Instruction::I32Load(MemoryArg {
                            align: 2,
                            offset: 100,
                        }),
                        Instruction::End,
                    ]),
                },
                CodeItem {
                    local_groups: vec![],
                    expression: Rc::new(vec![
                        Instruction::I32Load(MemoryArg {
                            align: 3,
                            offset: 200,
                        }),
                        Instruction::I64Load(MemoryArg {
                            align: 3,
                            offset: 400,
                        }),
                        Instruction::End,
                    ]),
                },
            ],
            data_items: vec![
                DataItem {
                    memory_index: 0,
                    offset_expression: vec![Instruction::I32Const(100), Instruction::End],
                    data: vec![104, 101, 108, 108, 111],
                },
                DataItem {
                    memory_index: 0,
                    offset_expression: vec![Instruction::I32Const(200), Instruction::End],
                    data: vec![80, 96, 112],
                },
            ],
        };
        assert_eq!(e2, m2);
    }

    #[test]
    fn test_parse_instruction_const() {
        let s0 = get_test_resource_file_binary("test-instruction-const.wasm");
        let m0 = parse(&s0).unwrap();
        assert_eq!(
            m0.code_items[0],
            CodeItem {
                local_groups: vec![],
                expression: Rc::new(vec![
                    Instruction::F32Const(12.3),
                    Instruction::F32Const(45.6),
                    Instruction::F32Add,
                    Instruction::TruncSat(instruction::I32_TRUNC_SAT_F32_S),
                    Instruction::Drop,
                    Instruction::End
                ])
            }
        );
    }

    #[test]
    fn test_parse_instruction_variable() {
        let s0 = get_test_resource_file_binary("test-instruction-variable.wasm");
        let m0 = parse(&s0).unwrap();

        assert_eq!(
            m0.global_items,
            vec![
                GlobalItem {
                    global_type: GlobalType {
                        value_type: ValueType::I32,
                        mutable: true
                    },
                    init_expression: vec![Instruction::I32Const(1), Instruction::End]
                },
                GlobalItem {
                    global_type: GlobalType {
                        value_type: ValueType::I32,
                        mutable: true
                    },
                    init_expression: vec![Instruction::I32Const(2), Instruction::End]
                }
            ]
        );

        assert_eq!(
            m0.function_types,
            vec![FunctionType {
                params: vec![ValueType::I32, ValueType::I32],
                results: vec![]
            }]
        );

        assert_eq!(
            m0.code_items[0],
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
                expression: Rc::new(vec![
                    Instruction::GlobalGet(0),
                    Instruction::GlobalSet(1),
                    Instruction::LocalGet(0),
                    Instruction::LocalSet(1),
                    Instruction::End
                ])
            }
        );
    }

    #[test]
    fn test_parse_instruction_memory() {
        let s0 = get_test_resource_file_binary("test-instruction-memory.wasm");
        let m0 = parse(&s0).unwrap();

        assert_eq!(
            m0.memory_blocks,
            vec![MemoryType {
                limit: Limit::Range(1, 8)
            }]
        );

        assert_eq!(
            m0.data_items,
            vec![DataItem {
                memory_index: 0,
                offset_expression: vec![Instruction::I32Const(100), Instruction::End],
                data: vec!['h' as u8, 'e' as u8, 'l' as u8, 'l' as u8, 'o' as u8]
            }]
        );

        assert_eq!(
            m0.code_items[0],
            CodeItem {
                local_groups: vec![],
                expression: Rc::new(vec![
                    Instruction::I32Const(1),
                    Instruction::I32Const(2),
                    Instruction::I32Load(MemoryArg {
                        align: 2,
                        offset: 100
                    }),
                    Instruction::I32Store(MemoryArg {
                        align: 2,
                        offset: 100
                    }),
                    Instruction::MemorySize(0),
                    Instruction::Drop,
                    Instruction::I32Const(4),
                    Instruction::MemoryGrow(0),
                    Instruction::Drop,
                    Instruction::End
                ])
            }
        );
    }

    #[test]
    fn test_parse_instruction_flow_control() {
        let s0 = get_test_resource_file_binary("test-instruction-flow-control.wasm");
        let m0 = parse(&s0).unwrap();

        assert_eq!(
            m0.code_items[0],
            CodeItem {
                local_groups: vec![],
                expression: Rc::new(vec![
                    Instruction::Block {
                        result: Some(ValueType::I32),
                        body: Rc::new(vec![
                            Instruction::I32Const(1),
                            Instruction::Loop {
                                result: Some(ValueType::I32),
                                body: Rc::new(vec![
                                    Instruction::I32Const(2),
                                    Instruction::If {
                                        result: Some(ValueType::I32),
                                        consequet_body: Rc::new(vec![
                                            Instruction::I32Const(3),
                                            Instruction::Else
                                        ]),
                                        alternate_body: Rc::new(vec![
                                            Instruction::I32Const(4),
                                            Instruction::End
                                        ])
                                    },
                                    Instruction::End
                                ])
                            },
                            Instruction::End
                        ])
                    },
                    Instruction::Drop,
                    Instruction::End
                ])
            }
        );
    }

    #[test]
    fn test_parse_instruction_call() {
        let s0 = get_test_resource_file_binary("test-instruction-call.wasm");
        let m0 = parse(&s0).unwrap();

        assert_eq!(
            m0.function_types,
            vec![FunctionType {
                params: vec![],
                results: vec![]
            }]
        );

        assert_eq!(m0.function_list, vec![0, 0]);

        assert_eq!(
            m0.table_types,
            vec![TableType {
                limit: Limit::Range(3, 3)
            }]
        );

        assert_eq!(
            m0.element_items,
            vec![ElementItem {
                table_index: 0,
                offset_expression: vec![Instruction::I32Const(0), Instruction::End],
                function_indices: vec![1, 1, 1]
            }]
        );

        assert_eq!(
            m0.code_items,
            vec![
                CodeItem {
                    local_groups: vec![],
                    expression: Rc::new(vec![
                        Instruction::Call(1),
                        Instruction::I32Const(2),
                        Instruction::CallIndirect(0),
                        Instruction::End
                    ])
                },
                CodeItem {
                    local_groups: vec![],
                    expression: Rc::new(vec![Instruction::I32Const(100), Instruction::End])
                }
            ]
        );
    }

    #[test]
    fn test_parse_instruction_branch() {
        let s0 = get_test_resource_file_binary("test-instruction-branch.wasm");
        let m0 = parse(&s0).unwrap();
        assert_eq!(
            m0.code_items[0],
            CodeItem {
                local_groups: vec![],
                expression: Rc::new(vec![
                    Instruction::Block {
                        result: None,
                        body: Rc::new(vec![
                            Instruction::I32Const(100),
                            Instruction::Br(0),
                            Instruction::I32Const(101),
                            Instruction::End
                        ])
                    },
                    Instruction::Loop {
                        result: None,
                        body: Rc::new(vec![
                            Instruction::I32Const(200),
                            Instruction::Br(0),
                            Instruction::I32Const(201),
                            Instruction::End
                        ])
                    },
                    Instruction::I32Const(300),
                    Instruction::I32Eqz,
                    Instruction::If {
                        result: None,
                        consequet_body: Rc::new(vec![
                            Instruction::I32Const(400),
                            Instruction::Br(0),
                            Instruction::I32Const(401),
                            Instruction::Else
                        ]),
                        alternate_body: Rc::new(vec![
                            Instruction::I32Const(500),
                            Instruction::Br(0),
                            Instruction::I32Const(501),
                            Instruction::End
                        ]),
                    },
                    Instruction::End
                ])
            }
        );
    }
}
