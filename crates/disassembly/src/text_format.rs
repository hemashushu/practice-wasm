// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::name_package::NamePackage;
use anvm_ast::{
    ast::{
        CodeItem, DataItem, ElementItem, ExportItem, GlobalItem, ImportDescriptor, ImportItem,
        Limit, MemoryType, Module, TableType, TypeItem,
    },
    instruction::{BlockType, Instruction, MemoryArgument},
};
use std::fmt::Write;

/// 转换指令到文本格式
pub trait TextFormat {
    /// 参数 option_item_index 为（ast::Module 首层字段）项目的索引
    ///
    /// 比如对于函数项 CodeItem，则 item_index 为 CodeItem 的索引。
    /// 对于二级字段，则需要根据情况传入所需的索引，
    /// 比如 CodeItem 里的 LocalGroup 对象以及 block/loop/if 等指令对象，
    /// 它们同样需要 CodeItem 的索引以及对象自己的索引，这时需要传入的应该是它们
    /// 所在的 CodeItem 的索引。
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result;

    fn to_text(&self, name_package: &NamePackage, option_item_index: Option<u32>) -> String {
        let mut f = String::new();
        self.text(name_package, option_item_index, &mut f).unwrap();
        f
    }
}

impl TextFormat for TypeItem {
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例
        // (type $name (func (param i32 i32) (result i32 i32)))
        // (type (;0;) (func (result i32)))

        let mut text_fragments: Vec<String> = vec![];
        text_fragments.push("(type".to_string());

        let type_index = option_item_index.unwrap();
        if let Some(type_name) = name_package.get_type_name(&type_index) {
            text_fragments.push(format!("${}", type_name));
        } else {
            text_fragments.push(format!("(;{};)", type_index));
        }

        let mut function_type_text_fragments: Vec<String> = vec![];
        function_type_text_fragments.push("(func".to_string());

        let TypeItem::FunctionType(type_item) = self;

        if type_item.params.len() > 0 {
            function_type_text_fragments.push(format!(
                "(param {})",
                type_item
                    .params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ));
        }

        if type_item.results.len() > 0 {
            function_type_text_fragments.push(format!(
                "(result {})",
                type_item
                    .results
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ));
        }

        text_fragments.push(format!("{}))", function_type_text_fragments.join(" ")));

        write!(f, "{}", text_fragments.join(" "))
    }
}

/// 在 ast::Module 当中直接使用 TableType 作为 TableItem，
/// 所以这里 TableType 转换为 text 实际上是 TableItem 的文本。
impl TextFormat for TableType {
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例：
        // (table (;0;) 1 funcref)
        // (table $name 1 8 funcref)
        let mut text_fragments: Vec<String> = vec![];

        text_fragments.push("(table".to_string());

        let table_index = option_item_index.unwrap();
        if let Some(table_name) = name_package.get_table_name(&table_index) {
            text_fragments.push(format!("${}", table_name));
        } else {
            text_fragments.push(format!("(;{};)", table_index));
        }

        match self.limit {
            Limit::AtLeast(min) => {
                text_fragments.push(format!("{} funcref)", min));
            }
            Limit::Range(min, max) => {
                text_fragments.push(format!("{} {} funcref)", min, max));
            }
        }

        write!(f, "{}", text_fragments.join(" "))
    }
}

/// 在 ast::Module 当中直接使用 MemoryType 作为 MemoryItem，
/// 所以这里 MemoryType 转换为 text 实际上是 MemoryItem 的文本。
impl TextFormat for MemoryType {
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例：
        // (memory (;0;) 1)
        // (memory $name 1 8)
        let mut text_fragments: Vec<String> = vec![];

        text_fragments.push("(memory".to_string());

        let memory_block_index = option_item_index.unwrap();
        if let Some(memory_block_name) = name_package.get_memory_block_name(&memory_block_index) {
            text_fragments.push(format!("${}", memory_block_name));
        } else {
            text_fragments.push(format!("(;{};)", memory_block_index));
        }

        match self.limit {
            Limit::AtLeast(min) => {
                text_fragments.push(format!("{})", min));
            }
            Limit::Range(min, max) => {
                text_fragments.push(format!("{} {})", min, max));
            }
        }

        write!(f, "{}", text_fragments.join(" "))
    }
}

impl TextFormat for GlobalItem {
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>, // 这个 global_variable_index 的值是包括了导入项的全部全局项范围之内的索引
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例：
        // (global $__stack_pointer (mut i32) i32.const 1048576)
        // (global (;1;) i32 i32.const 1048576)
        let mut text_fragments: Vec<String> = vec![];
        text_fragments.push("(global".to_string());

        let global_variable_index = option_item_index.unwrap();
        if let Some(global_variable_name) =
            name_package.get_global_variable_name(&global_variable_index)
        {
            text_fragments.push(format!("${}", global_variable_name));
        } else {
            text_fragments.push(format!("(;{};)", global_variable_index));
        }

        let value_type_name = self.global_type.value_type.to_string();
        if self.global_type.mutable {
            text_fragments.push(format!("(mut {})", value_type_name));
        } else {
            text_fragments.push(format!("{}", value_type_name));
        }

        text_fragments.push(format!(
            "{})",
            format_constant_expression(&self.initialize_instruction_items)
        ));

        write!(f, "{}", text_fragments.join(" "))
    }
}

/// 因为 import item 的索引是分类计数的，所以无法通过
/// "给结构体 ImportItem 实现 TextFormat 特性" 来实现从 ImportItem 转换到 text。
pub fn format_import_items(
    import_items: &[ImportItem],
    name_package: &NamePackage,
) -> (
    Vec<String>,
    /* function_index */ u32,
    /* table_index */ u32,
    /* memory_block_index */ u32,
    /* global_variable_index */ u32,
) {
    // 示例
    // (import "env" "putc" (func $fputc (type $ft0)))
    // (import "env" "print" (func (;0;) (type 1)))
    // (import "env" "tab0" (table $name 1 8 funcref))
    // (import "env" "mem0" (memory $name 1 8))
    // (import "env" "g1" (global $g1 i32))
    // (import "env" "g2" (global $g2 (mut i32)))

    let mut function_index: u32 = 0;
    let mut table_index: u32 = 0;
    let mut memory_block_index: u32 = 0;
    let mut global_variable_index: u32 = 0;

    let lines = import_items
        .iter()
        .map(|import_item| {
            let module_name = &import_item.module_name;
            let item_name = &import_item.item_name;

            match &import_item.import_descriptor {
                ImportDescriptor::FunctionTypeIndex(type_index) => {
                    // 示例
                    // (import "env" "putc" (func $fputc (type $ft0)))
                    // (import "env" "print" (func (;0;) (type 1)))
                    let mut text_fragments: Vec<String> = vec![];

                    text_fragments.push("(import".to_string());
                    text_fragments.push(format!("\"{}\" \"{}\"", module_name, item_name));

                    let mut import_function_text_fragments: Vec<String> = vec![];
                    import_function_text_fragments.push("(func".to_string());

                    if let Some(function_name) = name_package.get_function_name(&function_index) {
                        import_function_text_fragments.push(format!("${}", function_name));
                    } else {
                        import_function_text_fragments.push(format!("(;{};)", function_index));
                    }

                    if let Some(type_name) = name_package.get_type_name(type_index) {
                        import_function_text_fragments.push(format!("(type ${}))", type_name));
                    } else {
                        import_function_text_fragments.push(format!("(type {}))", type_index));
                    }

                    text_fragments.push(format!("{})", import_function_text_fragments.join(" ")));

                    function_index += 1; // 累加 function_index
                    text_fragments.join(" ")
                }
                ImportDescriptor::TableType(table_type) => {
                    // 示例
                    // (import "env" "tab0" (table $name 1 8 funcref))
                    let mut text_fragments: Vec<String> = vec![];

                    text_fragments.push("(import".to_string());
                    text_fragments.push(format!("\"{}\" \"{}\"", module_name, item_name));

                    text_fragments.push(format!(
                        "{})",
                        table_type.to_text(name_package, Some(table_index))
                    ));

                    table_index += 1; // 累加 table_index
                    text_fragments.join(" ")
                }
                ImportDescriptor::MemoryType(memory_type) => {
                    // 示例
                    // (import "env" "mem0" (memory $name 1 8))
                    let mut text_fragments: Vec<String> = vec![];

                    text_fragments.push("(import".to_string());
                    text_fragments.push(format!("\"{}\" \"{}\"", module_name, item_name));

                    text_fragments.push(format!(
                        "{})",
                        memory_type.to_text(name_package, Some(memory_block_index))
                    ));

                    memory_block_index += 1; // 累加 memory_block_index
                    text_fragments.join(" ")
                }
                ImportDescriptor::GlobalType(global_type) => {
                    // 示例
                    // (import "env" "g1" (global $g1 i32))
                    // (import "env" "g2" (global $g2 (mut i32)))

                    let mut text_fragments: Vec<String> = vec![];

                    text_fragments.push("(import".to_string());
                    text_fragments.push(format!("\"{}\" \"{}\"", module_name, item_name));

                    let mut global_type_text_fragments: Vec<String> = vec![];
                    global_type_text_fragments.push("(global".to_string());

                    if let Some(global_variable_name) =
                        name_package.get_global_variable_name(&global_variable_index)
                    {
                        global_type_text_fragments.push(format!("${}", global_variable_name));
                    } else {
                        global_type_text_fragments.push(format!("(;{};)", global_variable_index));
                    }

                    if global_type.mutable {
                        global_type_text_fragments
                            .push(format!("(mut {}))", global_type.value_type.to_string()));
                    } else {
                        global_type_text_fragments
                            .push(format!("{})", global_type.value_type.to_string()));
                    }

                    text_fragments.push(format!("{})", global_type_text_fragments.join(" ")));

                    global_variable_index += 1; // 累加 global_variable_index
                    text_fragments.join(" ")
                }
            }
        })
        .collect::<Vec<String>>();

    (
        lines,
        function_index,
        table_index,
        memory_block_index,
        global_variable_index,
    )
}

impl TextFormat for BlockType {
    fn text(
        &self,
        name_package: &NamePackage,
        _option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例
        // block
        //       ^^
        // block (result i32)
        // block (result i64)
        // block (result f32)
        // block (result f64)
        //       ^----------^
        // block (type $ft0)
        //       ^---------^
        // block (type 1)
        //       ^------^

        match self {
            // 内置的数据类型
            Self::ResultI32 => write!(f, "(result i32)"),
            Self::ResultI64 => write!(f, "(result i64)"),
            Self::ResultF32 => write!(f, "(result f32)"),
            Self::ResultF64 => write!(f, "(result f64)"),
            Self::ResultEmpty => write!(f, ""),

            // 来自类型表的类型
            Self::TypeIndex(type_index) => {
                if let Some(type_name) = name_package.get_type_name(type_index) {
                    write!(f, "(type ${})", type_name)
                } else {
                    write!(f, "(type {})", type_index)
                }
            }
        }
    }
}

impl TextFormat for MemoryArgument {
    fn text(
        &self,
        _name_package: &NamePackage,
        _option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例
        // i32.load offset=100 align=4
        //          ^----------------^
        write!(f, "offset={} align={}", self.offset, 2i32.pow(self.align))
    }
}

impl TextFormat for Instruction {
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>, // 这个函数索引的值是 "包括导入和内部函数范围之内" 的索引值
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例：
        // block
        // i32.const 123
        // local.get $a
        // call $func0
        // end
        match self {
            Self::Unreachable => write!(f, "unreachable"),
            Self::Nop => write!(f, "nop"),
            Self::Block(block_type, block_index) => {
                let mut text_fragments: Vec<String> = vec![];
                text_fragments.push("block".to_string());

                if let Some(function_index) = option_item_index {
                    let option_block_label =
                        name_package.get_block_lable(&function_index, block_index);
                    if let Some(block_label) = option_block_label {
                        text_fragments.push(format!("${}", block_label));
                    }
                }

                let block_type_text = block_type.to_text(name_package, None);
                if block_type_text != "" {
                    text_fragments.push(block_type_text);
                }

                write!(f, "{}", text_fragments.join(" "))
            }
            Self::Loop(block_type, block_index) => {
                let mut text_fragments: Vec<String> = vec![];
                text_fragments.push("loop".to_string());

                if let Some(function_index) = option_item_index {
                    let option_block_label =
                        name_package.get_block_lable(&function_index, block_index);
                    if let Some(block_label) = option_block_label {
                        text_fragments.push(format!("${}", block_label));
                    }
                }

                let block_type_text = block_type.to_text(name_package, None);
                if block_type_text != "" {
                    text_fragments.push(block_type_text);
                }

                write!(f, "{}", text_fragments.join(" "))
            }
            Self::If(block_type, block_index) => {
                let mut text_fragments: Vec<String> = vec![];
                text_fragments.push("if".to_string());

                if let Some(function_index) = option_item_index {
                    let option_block_label =
                        name_package.get_block_lable(&function_index, block_index);
                    if let Some(block_label) = option_block_label {
                        text_fragments.push(format!("${}", block_label));
                    }
                }

                let block_type_text = block_type.to_text(name_package, None);
                if block_type_text != "" {
                    text_fragments.push(block_type_text);
                }

                write!(f, "{}", text_fragments.join(" "))
            }
            Self::Else => write!(f, "else"),
            Self::End => write!(f, "end"),
            Self::Br(relative_deepth) => {
                write!(f, "br {}", relative_deepth)
            }
            Self::BrIf(relative_deepth) => {
                write!(f, "br_if {}", relative_deepth)
            }
            Self::BrTable(relative_depths, default_relative_depth) => {
                let depths = relative_depths
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "br_table {} {}", depths, default_relative_depth)
            }
            Self::Return => write!(f, "return"),
            Self::Call(function_index) => {
                if let Some(function_name) = name_package.get_function_name(function_index) {
                    write!(f, "call ${}", function_name)
                } else {
                    write!(f, "call {}", function_index)
                }
            }
            Self::CallIndirect(type_index, _table_index) => {
                // table_index 暂时用不上
                if let Some(type_name) = name_package.get_type_name(type_index) {
                    write!(f, "call_indirect (type ${})", type_name)
                } else {
                    write!(f, "call_indirect (type {})", type_index)
                }
            }

            Self::Drop => write!(f, "drop"),
            Self::Select => write!(f, "select"),

            Self::LocalGet(local_variable_index) => {
                let option_variable_name = {
                    if let Some(function_index) = option_item_index {
                        name_package.get_local_variable_name(&function_index, local_variable_index)
                    } else {
                        None
                    }
                };

                if let Some(variable_name) = option_variable_name {
                    write!(f, "local.get ${}", variable_name)
                } else {
                    write!(f, "local.get {}", local_variable_index)
                }
            }
            Self::LocalSet(local_variable_index) => {
                let option_variable_name = {
                    if let Some(function_index) = option_item_index {
                        name_package.get_local_variable_name(&function_index, local_variable_index)
                    } else {
                        None
                    }
                };

                if let Some(variable_name) = option_variable_name {
                    write!(f, "local.set ${}", variable_name)
                } else {
                    write!(f, "local.set {}", local_variable_index)
                }
            }
            Self::LocalTee(local_variable_index) => {
                let option_variable_name = {
                    if let Some(function_index) = option_item_index {
                        name_package.get_local_variable_name(&function_index, local_variable_index)
                    } else {
                        None
                    }
                };

                if let Some(variable_name) = option_variable_name {
                    write!(f, "local.tee ${}", variable_name)
                } else {
                    write!(f, "local.tee {}", local_variable_index)
                }
            }
            Self::GlobalGet(global_variable_index) => {
                if let Some(variable_name) =
                    name_package.get_global_variable_name(global_variable_index)
                {
                    write!(f, "global.get ${}", variable_name)
                } else {
                    write!(f, "global.get {}", global_variable_index)
                }
            }
            Self::GlobalSet(global_variable_index) => {
                if let Some(variable_name) =
                    name_package.get_global_variable_name(global_variable_index)
                {
                    write!(f, "global.set ${}", variable_name)
                } else {
                    write!(f, "global.set {}", global_variable_index)
                }
            }

            Self::I32Load(memory_argument) => write!(
                f,
                "i32.load {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Load(memory_argument) => write!(
                f,
                "i64.load {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::F32Load(memory_argument) => write!(
                f,
                "f32.load {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::F64Load(memory_argument) => write!(
                f,
                "f64.load {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I32Load8S(memory_argument) => write!(
                f,
                "i32.load8_s {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I32Load8U(memory_argument) => write!(
                f,
                "i32.load8_u {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I32Load16S(memory_argument) => write!(
                f,
                "i32.load16_s {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I32Load16U(memory_argument) => write!(
                f,
                "i32.load16_u {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Load8S(memory_argument) => write!(
                f,
                "i64.load8_s {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Load8U(memory_argument) => write!(
                f,
                "i64.load8_u {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Load16S(memory_argument) => write!(
                f,
                "i64.load16_s {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Load16U(memory_argument) => write!(
                f,
                "i64.load16_u {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Load32S(memory_argument) => write!(
                f,
                "i64.load32_s {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Load32U(memory_argument) => write!(
                f,
                "i64.load32_u {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I32Store(memory_argument) => write!(
                f,
                "i32.store {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Store(memory_argument) => write!(
                f,
                "i64.store {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::F32Store(memory_argument) => write!(
                f,
                "f32.store {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::F64Store(memory_argument) => write!(
                f,
                "f64.store {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I32Store8(memory_argument) => write!(
                f,
                "i32.store8 {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I32Store16(memory_argument) => write!(
                f,
                "i32.store16 {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Store8(memory_argument) => write!(
                f,
                "i64.store8 {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Store16(memory_argument) => write!(
                f,
                "i64.store16 {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::I64Store32(memory_argument) => write!(
                f,
                "i64.store32 {}",
                memory_argument.to_text(name_package, option_item_index)
            ),
            Self::MemorySize(_memory_block_index) => write!(f, "memory.size"), // memory_block_index 暂时用不上
            Self::MemoryGrow(_memory_block_index) => write!(f, "memory.grow"), // memory_block_index 暂时用不上

            Self::I32Const(immediate_number) => write!(f, "i32.const {}", immediate_number),
            Self::I64Const(immediate_number) => write!(f, "i64.const {}", immediate_number),
            Self::F32Const(immediate_number) => write!(f, "f32.const {}", immediate_number),
            Self::F64Const(immediate_number) => write!(f, "f64.const {}", immediate_number),

            Self::I32Eqz => write!(f, "i32.eqz"),
            Self::I32Eq => write!(f, "i32.eq"),
            Self::I32Ne => write!(f, "i32.ne"),
            Self::I32LtS => write!(f, "i32.lt_s"),
            Self::I32LtU => write!(f, "i32.lt_u"),
            Self::I32GtS => write!(f, "i32.gt_s"),
            Self::I32GtU => write!(f, "i32.gt_u"),
            Self::I32LeS => write!(f, "i32.le_s"),
            Self::I32LeU => write!(f, "i32.le_u"),
            Self::I32GeS => write!(f, "i32.ge_s"),
            Self::I32GeU => write!(f, "i32.ge_u"),

            Self::I64Eqz => write!(f, "i64.eqz"),
            Self::I64Eq => write!(f, "i64.eq"),
            Self::I64Ne => write!(f, "i64.ne"),
            Self::I64LtS => write!(f, "i64.lt_s"),
            Self::I64LtU => write!(f, "i64.lt_u"),
            Self::I64GtS => write!(f, "i64.gt_s"),
            Self::I64GtU => write!(f, "i64.gt_u"),
            Self::I64LeS => write!(f, "i64.le_s"),
            Self::I64LeU => write!(f, "i64.le_u"),
            Self::I64GeS => write!(f, "i64.ge_s"),
            Self::I64GeU => write!(f, "i64.ge_u"),

            Self::F32Eq => write!(f, "f32.eq"),
            Self::F32Ne => write!(f, "f32.ne"),
            Self::F32Lt => write!(f, "f32.lt"),
            Self::F32Gt => write!(f, "f32.gt"),
            Self::F32Le => write!(f, "f32.le"),
            Self::F32Ge => write!(f, "f32.ge"),

            Self::F64Eq => write!(f, "f64.eq"),
            Self::F64Ne => write!(f, "f64.ne"),
            Self::F64Lt => write!(f, "f64.lt"),
            Self::F64Gt => write!(f, "f64.gt"),
            Self::F64Le => write!(f, "f64.le"),
            Self::F64Ge => write!(f, "f64.ge"),

            Self::I32Clz => write!(f, "i32.clz"),
            Self::I32Ctz => write!(f, "i32.ctz"),
            Self::I32PopCnt => write!(f, "i32.popcnt"),

            Self::I32Add => write!(f, "i32.add"),
            Self::I32Sub => write!(f, "i32.sub"),
            Self::I32Mul => write!(f, "i32.mul"),
            Self::I32DivS => write!(f, "i32.div_s"),
            Self::I32DivU => write!(f, "i32.div_u"),
            Self::I32RemS => write!(f, "i32.rem_s"),
            Self::I32RemU => write!(f, "i32.rem_u"),
            Self::I32And => write!(f, "i32.and"),
            Self::I32Or => write!(f, "i32.or"),
            Self::I32Xor => write!(f, "i32.xor"),
            Self::I32Shl => write!(f, "i32.shl"),
            Self::I32ShrS => write!(f, "i32.shr_s"),
            Self::I32ShrU => write!(f, "i32.shr_u"),
            Self::I32Rotl => write!(f, "i32.rotl"),
            Self::I32Rotr => write!(f, "i32.rotr"),

            Self::I64Clz => write!(f, "i64.clz"),
            Self::I64Ctz => write!(f, "i64.ctz"),
            Self::I64PopCnt => write!(f, "i64.popcnt"),

            Self::I64Add => write!(f, "i64.add"),
            Self::I64Sub => write!(f, "i64.sub"),
            Self::I64Mul => write!(f, "i64.mul"),
            Self::I64DivS => write!(f, "i64.div_s"),
            Self::I64DivU => write!(f, "i64.div_u"),
            Self::I64RemS => write!(f, "i64.rem_s"),
            Self::I64RemU => write!(f, "i64.rem_u"),
            Self::I64And => write!(f, "i64.and"),
            Self::I64Or => write!(f, "i64.or"),
            Self::I64Xor => write!(f, "i64.xor"),
            Self::I64Shl => write!(f, "i64.shl"),
            Self::I64ShrS => write!(f, "i64.shr_s"),
            Self::I64ShrU => write!(f, "i64.shr_u"),
            Self::I64Rotl => write!(f, "i64.rotl"),
            Self::I64Rotr => write!(f, "i64.rotr"),

            Self::F32Abs => write!(f, "f32.abs"),
            Self::F32Neg => write!(f, "f32.neg"),
            Self::F32Ceil => write!(f, "f32.ceil"),
            Self::F32Floor => write!(f, "f32.floor"),
            Self::F32Trunc => write!(f, "f32.trunc"),
            Self::F32Nearest => write!(f, "f32.nearest"),
            Self::F32Sqrt => write!(f, "f32.sqrt"),
            Self::F32Add => write!(f, "f32.add"),
            Self::F32Sub => write!(f, "f32.sub"),
            Self::F32Mul => write!(f, "f32.mul"),
            Self::F32Div => write!(f, "f32.div"),
            Self::F32Min => write!(f, "f32.min"),
            Self::F32Max => write!(f, "f32.max"),
            Self::F32CopySign => write!(f, "f32.copysign"),

            Self::F64Abs => write!(f, "f64.abs"),
            Self::F64Neg => write!(f, "f64.neg"),
            Self::F64Ceil => write!(f, "f64.ceil"),
            Self::F64Floor => write!(f, "f64.floor"),
            Self::F64Trunc => write!(f, "f64.trunc"),
            Self::F64Nearest => write!(f, "f64.nearest"),
            Self::F64Sqrt => write!(f, "f64.sqrt"),
            Self::F64Add => write!(f, "f64.add"),
            Self::F64Sub => write!(f, "f64.sub"),
            Self::F64Mul => write!(f, "f64.mul"),
            Self::F64Div => write!(f, "f64.div"),
            Self::F64Min => write!(f, "f64.min"),
            Self::F64Max => write!(f, "f64.max"),
            Self::F64CopySign => write!(f, "f64.copysign"),

            Self::I32WrapI64 => write!(f, "i32.wrap_i64"),
            Self::I32TruncF32S => write!(f, "i32.trunc_f32_s"),
            Self::I32TruncF32U => write!(f, "i32.trunc_f32_u"),
            Self::I32TruncF64S => write!(f, "i32.trunc_f64_s"),
            Self::I32TruncF64U => write!(f, "i32.trunc_f64_u"),
            Self::I64ExtendI32S => write!(f, "i64.extend_i32_s"),
            Self::I64ExtendI32U => write!(f, "i64.extend_i32_u"),
            Self::I64TruncF32S => write!(f, "i64.trunc_f32_s"),
            Self::I64TruncF32U => write!(f, "i64.trunc_f32_u"),
            Self::I64TruncF64S => write!(f, "i64.trunc_f64_s"),
            Self::I64TruncF64U => write!(f, "i64.trunc_f64_u"),
            Self::F32ConvertI32S => write!(f, "f32.convert_i32_s"),
            Self::F32ConvertI32U => write!(f, "f32.convert_i32_u"),
            Self::F32ConvertI64S => write!(f, "f32.convert_i64_s"),
            Self::F32ConvertI64U => write!(f, "f32.convert_i64_u"),
            Self::F32DemoteF64 => write!(f, "f32.demote_f64"),
            Self::F64ConvertI32S => write!(f, "f64.convert_i32_s"),
            Self::F64ConvertI32U => write!(f, "f64.convert_i32_u"),
            Self::F64ConvertI64S => write!(f, "f64.convert_i64_s"),
            Self::F64ConvertI64U => write!(f, "f64.convert_i64_u"),
            Self::F64PromoteF32 => write!(f, "f64.promote_f32"),
            Self::I32ReinterpretF32 => write!(f, "i32.reinterpret_f32"),
            Self::I64ReinterpretF64 => write!(f, "i64.reinterpret_f64"),
            Self::F32ReinterpretI32 => write!(f, "f32.reinterpret_i32"),
            Self::F64ReinterpretI64 => write!(f, "f64.reinterpret_i64"),

            Self::I32Extend8S => write!(f, "i32.extend8_s"),
            Self::I32Extend16S => write!(f, "i32.extend16_s"),
            Self::I64Extend8S => write!(f, "i64.extend8_s"),
            Self::I64Extend16S => write!(f, "i64.extend16_s"),
            Self::I64Extend32S => write!(f, "i64.extend32_s"),

            Self::I32TruncSatF32S => write!(f, "i32.trunc_sat_f32_s"),
            Self::I32TruncSatF32U => write!(f, "i32.trunc_sat_f32_u"),
            Self::I32TruncSatF64S => write!(f, "i32.trunc_sat_f64_s"),
            Self::I32TruncSatF64U => write!(f, "i32.trunc_sat_f64_u"),
            Self::I64TruncSatF32S => write!(f, "i64.trunc_sat_f32_s"),
            Self::I64TruncSatF32U => write!(f, "i64.trunc_sat_f32_u"),
            Self::I64TruncSatF64S => write!(f, "i64.trunc_sat_64_s"),
            Self::I64TruncSatF64U => write!(f, "i64.trunc_sat_64_u"),
        }
    }
}

/// 将 type_index 和 code_item 打包起来
///
/// 一个完整的函数由：名称、签名（参数和返回值类型）、局部变量
/// 以及指令序列组成。
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionItem {
    pub type_index: u32,
    pub type_item: TypeItem,
    pub code_item: CodeItem,
}

pub fn format_function_item(
    function_item: &FunctionItem,
    name_package: &NamePackage,
    function_index: u32, // 这个函数索引的值是 "包括导入和内部函数范围之内" 的索引值
) -> Vec<String> {
    // 示例
    //
    // (func $name (;type $name;) (param $name i32) (param $name i32) (result i32)
    //    (local $name i32)
    //    instruction_0
    //    instruction_1
    //    ...
    // )
    //
    // 当函数、参数和局部变量无名称时，使用注释序号
    //
    // (func (;0;) (;type 1;) (param (;0;) i32) (param (;1;) i32) (result i32)
    //     (local (;2;) i32)
    //     instruction_0
    //     instruction_1
    //     ...
    // )

    let mut local_variable_index: u32 = 0;

    // 生成 type 文本
    //
    // 这里无法重用 TypeItem 的 to_text 方法，因为单独的 TypeItem 的文本
    // 跟在函数里的 TypeITem 不一样。
    let mut inline_type_text_fragments: Vec<String> = vec![];

    if let Some(type_name) = name_package.get_type_name(&function_item.type_index) {
        inline_type_text_fragments.push(format!("(;type ${};)", type_name));
    } else {
        inline_type_text_fragments.push(format!("(;type {};)", function_item.type_index));
    }

    let TypeItem::FunctionType(function_type) = &function_item.type_item;

    for value_type in &function_type.params {
        if let Some(param_name) =
            name_package.get_local_variable_name(&function_index, &local_variable_index)
        {
            inline_type_text_fragments.push(format!(
                "(param ${} {})",
                param_name,
                value_type.to_string()
            ));
        } else {
            inline_type_text_fragments.push(format!(
                "(param (;{};) {})",
                local_variable_index,
                value_type.to_string()
            ));
        }

        local_variable_index += 1;
    }

    // 生成 result 文本
    if function_type.results.len() > 0 {
        inline_type_text_fragments.push(format!(
            "(result {})",
            function_type
                .results
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ));
    }

    // 生成 first line 文本
    let mut first_line_text_fragments: Vec<String> = vec![];

    first_line_text_fragments.push("(func".to_string());

    if let Some(function_name) = name_package.get_function_name(&function_index) {
        first_line_text_fragments.push(format!("${}", function_name));
    } else {
        first_line_text_fragments.push(format!("(;{};)", function_index));
    }

    first_line_text_fragments.push(inline_type_text_fragments.join(" ")); // 添加 type 文本到 first line 文本

    // 生成 line 文本
    let mut text_fragments: Vec<String> = vec![];

    text_fragments.push(first_line_text_fragments.join(" "));

    // 添加局部变量
    for local_variable_group in &function_item.code_item.local_groups {
        let value_type_name = local_variable_group.value_type.to_string();

        for _ in 0..local_variable_group.variable_count {
            if let Some(local_variable_name) =
                name_package.get_local_variable_name(&function_index, &local_variable_index)
            {
                text_fragments.push(format!(
                    "    (local ${} {})",
                    local_variable_name, value_type_name
                ));
            } else {
                text_fragments.push(format!(
                    "    (local (;{};) {})",
                    local_variable_index, value_type_name
                ));
            }

            local_variable_index += 1;
        }
    }

    // 添加指令
    //
    // 因为使用了 `(func ...)` 结构，其中右括号表示 `end 指令`，
    // 所以函数指令序列最后一个 `end 指令` 需要丢弃掉。
    let instruction_items = &function_item.code_item.instruction_items;

    let mut block_level = 0;

    for instruction in &instruction_items[0..instruction_items.len() - 1] {
        // 维护结构块的层级
        let indent_level = match instruction {
            Instruction::Block(_, _) | Instruction::Loop(_, _) | Instruction::If(_, _) => {
                block_level += 1;
                block_level - 1
            }
            Instruction::Else => block_level - 1,
            Instruction::End => {
                block_level -= 1;
                block_level
            }
            _ => block_level,
        };

        text_fragments.push(format!(
            "{}{}",
            "    ".repeat(indent_level + 1),
            instruction.to_text(name_package, Some(function_index))
        ));
    }

    // 添加最后一行
    text_fragments.push(")".to_string());

    // write!(f, "{}", text_fragments.join("\n"))
    text_fragments
}

pub fn get_function_items(module: &Module) -> Vec<FunctionItem> {
    let type_items = &module.type_items;
    let internal_function_to_type_index_list = &module.internal_function_to_type_index_list;

    module
        .code_items
        .iter()
        .enumerate()
        .map(|(internal_function_index, code_item)| {
            let type_index = internal_function_to_type_index_list[internal_function_index];

            FunctionItem {
                type_index: type_index,
                type_item: type_items[type_index as usize].clone(),
                code_item: code_item.to_owned(),
            }
        })
        .collect::<Vec<FunctionItem>>()
}

impl TextFormat for ExportItem {
    fn text(
        &self,
        name_package: &NamePackage,
        _option_item_index: Option<u32>, // export item 不使用 item_index
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例
        // (export "f1" (func $f1))
        // (export "f2" (func $f2))
        // (export "t1" (table $t1))
        // (export "m1" (memory $m1))
        // (export "g1" (global $g1))
        // (export "g2" (global $g2))

        let tail = match self.export_descriptor {
            anvm_ast::ast::ExportDescriptor::FunctionIndex(function_index) => {
                if let Some(function_name) = name_package.get_function_name(&function_index) {
                    format!("(func ${})", function_name)
                } else {
                    format!("(func {})", function_index)
                }
            }
            anvm_ast::ast::ExportDescriptor::TableIndex(table_index) => {
                if let Some(table_name) = name_package.get_table_name(&table_index) {
                    format!("(table ${})", table_name)
                } else {
                    format!("(table {})", table_index)
                }
            }
            anvm_ast::ast::ExportDescriptor::MemoryBlockIndex(memory_block_index) => {
                if let Some(memory_block_name) =
                    name_package.get_memory_block_name(&memory_block_index)
                {
                    format!("(memory ${})", memory_block_name)
                } else {
                    format!("(memory {})", memory_block_index)
                }
            }
            anvm_ast::ast::ExportDescriptor::GlobalItemIndex(global_variable_index) => {
                if let Some(global_variable_name) =
                    name_package.get_global_variable_name(&global_variable_index)
                {
                    format!("(global ${})", global_variable_name)
                } else {
                    format!("(global {})", global_variable_index)
                }
            }
        };

        write!(f, "(export \"{}\" {})", self.name, tail)
    }
}

impl TextFormat for ElementItem {
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例
        // (elem $elem_one (offset (i32.const 1)) $func0 $func1)
        // (elem (;0;) (offset (i32.const 3)) 2 3 4)

        let mut text_fragments: Vec<String> = vec![];

        text_fragments.push("(elem".to_string());

        let element_index = option_item_index.unwrap();

        if let Some(element_name) = name_package.get_element_name(&element_index) {
            text_fragments.push(format!("${}", element_name));
        } else {
            text_fragments.push(format!("(;{};)", element_index));
        }

        let offset_text = format!(
            "(offset ({}))",
            format_constant_expression(&self.offset_instruction_items)
        );
        text_fragments.push(offset_text);

        let function_indices_text = self
            .function_indices
            .iter()
            .map(
                |function_index| match name_package.get_function_name(function_index) {
                    Some(function_name) => format!("${}", function_name),
                    None => function_index.to_string(),
                },
            )
            .collect::<Vec<String>>()
            .join(" ");
        text_fragments.push(format!("{})", function_indices_text));

        write!(f, "{}", text_fragments.join(" "))
    }
}

impl TextFormat for DataItem {
    fn text(
        &self,
        name_package: &NamePackage,
        option_item_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        // 示例
        // (data $name (offset (i32.const 10)) "\11\22\33")
        // (data (;1;) (offset (i32.const 20)) "\aa\bb\cc")

        let mut text_fragments: Vec<String> = vec![];

        text_fragments.push("(data".to_string());

        let data_index = option_item_index.unwrap();

        if let Some(data_name) = name_package.get_data_name(&data_index) {
            text_fragments.push(format!("${}", data_name));
        } else {
            text_fragments.push(format!("(;{};)", data_index));
        }

        // text_fragments.push(self.memory_block_index.to_string());

        let offset_text = format!(
            "(offset ({}))",
            format_constant_expression(&self.offset_instruction_items)
        );
        text_fragments.push(offset_text);

        let bytes_text = self
            .data
            .iter()
            .map(|byte| format!("\\{:02x}", byte))
            .collect::<Vec<String>>()
            .join("");
        text_fragments.push(format!("\"{}\")", bytes_text));

        write!(f, "{}", text_fragments.join(" "))
    }
}

pub fn format_constant_expression(instructions: &[Instruction]) -> String {
    let first_instruction = instructions.first().unwrap();
    match first_instruction {
        Instruction::I32Const(value) => format!("i32.const {}", value),
        Instruction::I64Const(value) => format!("i64.const {}", value),
        Instruction::F32Const(value) => format!("f32.const {}", value),
        Instruction::F64Const(value) => format!("f64.const {}", value),
        _ => panic!("unsupported constant expression instruction"),
    }
}

#[cfg(test)]
mod tests {
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
    use pretty_assertions::assert_eq;

    use crate::{
        name_package::NamePackage,
        text_format::{format_function_item, TextFormat},
    };

    use super::{format_import_items, get_function_items};

    #[test]
    fn test_type_item_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::TypeNames(vec![IndexNamePair {
                    index: 0,
                    name: "typ0".to_string(),
                }]),
            ])],
            type_items: vec![
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32, ValueType::I32],
                    results: vec![ValueType::I32, ValueType::I32],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![ValueType::I32],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![],
                }),
            ],
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

        let name_package = NamePackage::new(&module);

        assert_eq!(
            module.type_items[0].to_text(&name_package, Some(0)),
            "(type $typ0 (func (param i32 i32) (result i32 i32)))"
        );

        assert_eq!(
            module.type_items[1].to_text(&name_package, Some(1)),
            "(type (;1;) (func (param i32)))"
        );

        assert_eq!(
            module.type_items[2].to_text(&name_package, Some(2)),
            "(type (;2;) (func (result i32)))"
        );

        assert_eq!(
            module.type_items[3].to_text(&name_package, Some(3)),
            "(type (;3;) (func))"
        );
    }

    #[test]
    fn test_table_type_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::TableNames(vec![IndexNamePair {
                    index: 0,
                    name: "tab0".to_string(),
                }]),
            ])],
            type_items: vec![],
            import_items: vec![],
            internal_function_to_type_index_list: vec![],
            tables: vec![
                TableType {
                    limit: Limit::Range(1, 8),
                },
                TableType {
                    limit: Limit::AtLeast(4),
                },
            ],
            memory_blocks: vec![],
            global_items: vec![],
            export_items: vec![],
            start_function_index: None,
            element_items: vec![],
            code_items: vec![],
            data_items: vec![],
        };

        let name_package = NamePackage::new(&module);

        assert_eq!(
            module.tables[0].to_text(&name_package, Some(0)),
            "(table $tab0 1 8 funcref)"
        );

        assert_eq!(
            module.tables[1].to_text(&name_package, Some(1)),
            "(table (;1;) 4 funcref)"
        );
    }

    #[test]
    fn test_memory_type_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::MemoryBlockNames(vec![IndexNamePair {
                    index: 0,
                    name: "mem0".to_string(),
                }]),
            ])],
            type_items: vec![],
            import_items: vec![],
            internal_function_to_type_index_list: vec![],
            tables: vec![],
            memory_blocks: vec![
                MemoryType {
                    limit: Limit::Range(1, 8),
                },
                MemoryType {
                    limit: Limit::AtLeast(4),
                },
            ],
            global_items: vec![],
            export_items: vec![],
            start_function_index: None,
            element_items: vec![],
            code_items: vec![],
            data_items: vec![],
        };

        let name_package = NamePackage::new(&module);

        assert_eq!(
            module.memory_blocks[0].to_text(&name_package, Some(0)),
            "(memory $mem0 1 8)"
        );

        assert_eq!(
            module.memory_blocks[1].to_text(&name_package, Some(1)),
            "(memory (;1;) 4)"
        );
    }

    #[test]
    fn test_global_item_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::GlobalVariableNames(vec![IndexNamePair {
                    index: 0,
                    name: "stack_pointer".to_string(),
                }]),
            ])],
            type_items: vec![],
            import_items: vec![],
            internal_function_to_type_index_list: vec![],
            tables: vec![],
            memory_blocks: vec![],
            global_items: vec![
                GlobalItem {
                    global_type: GlobalType {
                        value_type: ValueType::I32,
                        mutable: true,
                    },
                    initialize_instruction_items: vec![
                        Instruction::I32Const(1000),
                        Instruction::End,
                    ],
                },
                GlobalItem {
                    global_type: GlobalType {
                        value_type: ValueType::I64,
                        mutable: false,
                    },
                    initialize_instruction_items: vec![
                        Instruction::I64Const(2000),
                        Instruction::End,
                    ],
                },
            ],
            export_items: vec![],
            start_function_index: None,
            element_items: vec![],
            code_items: vec![],
            data_items: vec![],
        };

        let name_package = NamePackage::new(&module);

        assert_eq!(
            module.global_items[0].to_text(&name_package, Some(0)),
            "(global $stack_pointer (mut i32) i32.const 1000)"
        );

        assert_eq!(
            module.global_items[1].to_text(&name_package, Some(1)),
            "(global (;1;) i64 i64.const 2000)"
        );
    }

    #[test]
    fn test_import_items_to_text() {
        let module = Module {
            custom_items: vec![
                CustomItem::NameCollections(vec![NameCollection::TypeNames(vec![IndexNamePair {
                    index: 0,
                    name: "typ0".to_string(),
                }])]),
                CustomItem::NameCollections(vec![NameCollection::FunctionNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "func0".to_string(),
                    },
                ])]),
                CustomItem::NameCollections(vec![NameCollection::TableNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "tab0".to_string(),
                    },
                ])]),
                CustomItem::NameCollections(vec![NameCollection::MemoryBlockNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "mem0".to_string(),
                    },
                ])]),
                CustomItem::NameCollections(vec![NameCollection::GlobalVariableNames(vec![
                    IndexNamePair {
                        index: 0,
                        name: "g0".to_string(),
                    },
                ])]),
            ],
            type_items: vec![
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32, ValueType::I32],
                    results: vec![],
                }),
            ],
            import_items: vec![
                ImportItem {
                    // (import "env" "putc" (func $func0 (type $typ0)))
                    module_name: "env".to_string(),
                    item_name: "putc".to_string(),
                    import_descriptor: ImportDescriptor::FunctionTypeIndex(0),
                },
                ImportItem {
                    // (import "env" "print" (func (;1;) (type 1)))
                    module_name: "env".to_string(),
                    item_name: "print".to_string(),
                    import_descriptor: ImportDescriptor::FunctionTypeIndex(1),
                },
                ImportItem {
                    // (import "share" "main_table" (table $tab0 1 4 funcref))
                    module_name: "share".to_string(),
                    item_name: "main_table".to_string(),
                    import_descriptor: ImportDescriptor::TableType(TableType {
                        limit: Limit::Range(1, 4),
                    }),
                },
                ImportItem {
                    // (import "share" "minor_table" (table (;1;) 8 funcref))
                    module_name: "share".to_string(),
                    item_name: "minor_table".to_string(),
                    import_descriptor: ImportDescriptor::TableType(TableType {
                        limit: Limit::AtLeast(8),
                    }),
                },
                ImportItem {
                    // (import "share" "main_memory" (memory $mem0 1 2))
                    module_name: "share".to_string(),
                    item_name: "main_memory".to_string(),
                    import_descriptor: ImportDescriptor::MemoryType(MemoryType {
                        limit: Limit::Range(1, 2),
                    }),
                },
                ImportItem {
                    // (import "share" "minor_memory" (memory (;1;) 6))
                    module_name: "share".to_string(),
                    item_name: "minor_memory".to_string(),
                    import_descriptor: ImportDescriptor::MemoryType(MemoryType {
                        limit: Limit::AtLeast(6),
                    }),
                },
                ImportItem {
                    // (import "common" "heap_pointer" (global $g0 (mut i32)))
                    module_name: "common".to_string(),
                    item_name: "heap_pointer".to_string(),
                    import_descriptor: ImportDescriptor::GlobalType(GlobalType {
                        mutable: true,
                        value_type: ValueType::I32,
                    }),
                },
                ImportItem {
                    // (import "common" "stack_pointer" (global (;1;) i64))
                    module_name: "common".to_string(),
                    item_name: "stack_pointer".to_string(),
                    import_descriptor: ImportDescriptor::GlobalType(GlobalType {
                        mutable: false,
                        value_type: ValueType::I64,
                    }),
                },
            ],
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

        let name_package = NamePackage::new(&module);

        let (lines, function_index, table_index, memory_block_index, global_variable_index) =
            format_import_items(&module.import_items, &name_package);

        let expected: Vec<&str> = vec![
            "(import \"env\" \"putc\" (func $func0 (type $typ0)))",
            "(import \"env\" \"print\" (func (;1;) (type 1)))",
            "(import \"share\" \"main_table\" (table $tab0 1 4 funcref))",
            "(import \"share\" \"minor_table\" (table (;1;) 8 funcref))",
            "(import \"share\" \"main_memory\" (memory $mem0 1 2))",
            "(import \"share\" \"minor_memory\" (memory (;1;) 6))",
            "(import \"common\" \"heap_pointer\" (global $g0 (mut i32)))",
            "(import \"common\" \"stack_pointer\" (global (;1;) i64))",
        ];

        assert_eq!(lines, expected);

        assert_eq!(function_index, 2);
        assert_eq!(table_index, 2);
        assert_eq!(memory_block_index, 2);
        assert_eq!(global_variable_index, 2);
    }

    #[test]
    fn test_instruction_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::TypeNames(vec![IndexNamePair {
                    index: 1,
                    name: "t1".to_string(),
                }]),
                NameCollection::FunctionNames(vec![IndexNamePair {
                    index: 1,
                    name: "f1".to_string(),
                }]),
                NameCollection::LocalVariableNamesPairList(vec![
                    FunctionIndexAndLocalVariableNamesPair {
                        function_index: 2,
                        local_variable_names: vec![
                            IndexNamePair {
                                index: 1,
                                name: "l1".to_string(),
                            },
                            IndexNamePair {
                                index: 2,
                                name: "l2".to_string(),
                            },
                        ],
                    },
                ]),
                NameCollection::BlockLabelsPairList(vec![FunctionIndexAndBlockLabelsPair {
                    function_index: 2,
                    block_labels: vec![IndexNamePair {
                        index: 2,
                        name: "b2".to_string(),
                    }],
                }]),
                NameCollection::GlobalVariableNames(vec![IndexNamePair {
                    index: 1,
                    name: "g1".to_string(),
                }]),
            ])],
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

        let name_package = NamePackage::new(&module);

        // 这里只测试部分带有直接操作数的指令
        let instructions: Vec<Instruction> = vec![
            Instruction::Unreachable,
            Instruction::Block(BlockType::ResultI32, 0),
            Instruction::Block(BlockType::ResultEmpty, 0),
            Instruction::Block(BlockType::TypeIndex(0), 0),
            Instruction::Loop(BlockType::ResultI64, 1),
            Instruction::Loop(BlockType::ResultEmpty, 1),
            Instruction::Loop(BlockType::TypeIndex(1), 1), // type 1 有名称 $t1
            Instruction::If(BlockType::ResultF32, 2),      // function 2 的 block 2 有名称 $b2
            Instruction::If(BlockType::ResultEmpty, 2),    // function 2 的 block 2 有名称 $b2
            Instruction::If(BlockType::TypeIndex(2), 2),   // function 2 的 block 2 有名称 $b2
            Instruction::Br(0),
            Instruction::Br(1),
            Instruction::BrIf(0),
            Instruction::BrIf(1),
            Instruction::BrTable(vec![0], 1),
            Instruction::BrTable(vec![0, 1], 2),
            Instruction::Call(0),
            Instruction::Call(1), // function 1 有名称 $f1
            Instruction::CallIndirect(0, 0),
            Instruction::CallIndirect(1, 0), // type 1 有名称 $t1
            Instruction::LocalGet(0),
            Instruction::LocalSet(1), // function 2 的 local 1 有名称 $l1
            Instruction::LocalTee(2), // function 2 的 local 2 有名称 $l2
            Instruction::GlobalGet(0),
            Instruction::GlobalSet(1), // global 1 有名称 $g1
            Instruction::I32Load(MemoryArgument {
                offset: 100,
                align: 2,
            }),
            Instruction::I64Load(MemoryArgument {
                offset: 200,
                align: 3,
            }),
            Instruction::MemorySize(0),
            Instruction::MemoryGrow(0),
            Instruction::I32Const(100),
            Instruction::I64Const(200),
            Instruction::F32Const(2.414),
            Instruction::F64Const(1.618),
        ];

        let expected: Vec<&str> = vec![
            "unreachable",
            "block (result i32)",
            "block",
            "block (type 0)",
            "loop (result i64)",
            "loop",
            "loop (type $t1)",
            "if $b2 (result f32)",
            "if $b2",
            "if $b2 (type 2)",
            "br 0",
            "br 1",
            "br_if 0",
            "br_if 1",
            "br_table 0 1",
            "br_table 0 1 2",
            "call 0",
            "call $f1",
            "call_indirect (type 0)",
            "call_indirect (type $t1)",
            "local.get 0",
            "local.set $l1",
            "local.tee $l2",
            "global.get 0",
            "global.set $g1",
            "i32.load offset=100 align=4",
            "i64.load offset=200 align=8",
            "memory.size",
            "memory.grow",
            "i32.const 100",
            "i64.const 200",
            "f32.const 2.414",
            "f64.const 1.618",
        ];

        let actual = instructions
            .iter()
            .map(|instruction| instruction.to_text(&name_package, Some(2)))
            .collect::<Vec<String>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_function_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::TypeNames(vec![IndexNamePair {
                    index: 0,
                    name: "t0".to_string(),
                }]),
                NameCollection::FunctionNames(vec![IndexNamePair {
                    index: 0,
                    name: "f0".to_string(),
                }]),
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
                                name: "x".to_string(),
                            },
                            IndexNamePair {
                                index: 3,
                                name: "y".to_string(),
                            },
                        ],
                    },
                ]),
                NameCollection::BlockLabelsPairList(vec![FunctionIndexAndBlockLabelsPair {
                    function_index: 0,
                    block_labels: vec![IndexNamePair {
                        index: 0,
                        name: "b0".to_string(),
                    }],
                }]),
            ])],
            type_items: vec![
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32, ValueType::I32],
                    results: vec![ValueType::I32, ValueType::I32],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![ValueType::I32],
                    results: vec![],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![ValueType::I32],
                }),
                TypeItem::FunctionType(FunctionType {
                    params: vec![],
                    results: vec![],
                }),
            ],
            import_items: vec![],
            internal_function_to_type_index_list: vec![0, 1, 2, 3],
            tables: vec![],
            memory_blocks: vec![],
            global_items: vec![],
            export_items: vec![],
            start_function_index: None,
            element_items: vec![],
            code_items: vec![
                CodeItem {
                    local_groups: vec![LocalGroup {
                        variable_count: 2,
                        value_type: ValueType::I32,
                    }],
                    instruction_items: vec![
                        Instruction::Block(BlockType::ResultEmpty, 0),
                        Instruction::LocalGet(0),
                        Instruction::LocalGet(1),
                        Instruction::Block(BlockType::ResultI32, 1),
                        Instruction::LocalSet(2),
                        Instruction::LocalSet(3),
                        Instruction::Block(BlockType::TypeIndex(0), 2),
                        Instruction::I32Const(0),
                        Instruction::End,
                        Instruction::End,
                        Instruction::End,
                        Instruction::Block(BlockType::TypeIndex(1), 3),
                        Instruction::I32Const(1),
                        Instruction::End,
                        Instruction::End,
                    ],
                },
                CodeItem {
                    local_groups: vec![LocalGroup {
                        variable_count: 1,
                        value_type: ValueType::I32,
                    }],
                    instruction_items: vec![
                        Instruction::Block(BlockType::ResultEmpty, 0),
                        Instruction::LocalGet(0),
                        Instruction::LocalGet(1),
                        Instruction::End,
                        Instruction::End,
                    ],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![Instruction::I32Const(2), Instruction::End],
                },
                CodeItem {
                    local_groups: vec![],
                    instruction_items: vec![Instruction::I32Const(3), Instruction::End],
                },
            ],
            data_items: vec![],
        };

        let name_package = NamePackage::new(&module);
        let internal_function_index_offset: usize = 0;
        let function_items = get_function_items(&module); //, internal_function_index_offset);

        let text0 = format_function_item(
            &function_items[0],
            &name_package,
            0 + internal_function_index_offset as u32,
        )
        .join("\n");

        assert_eq!(
            text0,
            "(func $f0 (;type $t0;) (param $a i32) (param $b i32) (result i32 i32)
    (local $x i32)
    (local $y i32)
    block $b0
        local.get $a
        local.get $b
        block (result i32)
            local.set $x
            local.set $y
            block (type $t0)
                i32.const 0
            end
        end
    end
    block (type 1)
        i32.const 1
    end
)"
        );

        let text1 = format_function_item(
            &function_items[1],
            &name_package,
            1 + internal_function_index_offset as u32,
        )
        .join("\n");

        assert_eq!(
            text1,
            "(func (;1;) (;type 1;) (param (;0;) i32)
    (local (;1;) i32)
    block
        local.get 0
        local.get 1
    end
)"
        );

        let text2 = format_function_item(
            &function_items[2],
            &name_package,
            2 + internal_function_index_offset as u32,
        )
        .join("\n");

        assert_eq!(
            text2,
            "(func (;2;) (;type 2;) (result i32)
    i32.const 2
)"
        );

        let text3 = format_function_item(
            &function_items[3],
            &name_package,
            3 + internal_function_index_offset as u32,
        )
        .join("\n");

        assert_eq!(
            text3,
            "(func (;3;) (;type 3;)
    i32.const 3
)"
        );
    }

    #[test]
    fn test_export_item_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::FunctionNames(vec![IndexNamePair {
                    index: 0,
                    name: "func0".to_string(),
                }]),
                NameCollection::TableNames(vec![IndexNamePair {
                    index: 0,
                    name: "tab0".to_string(),
                }]),
                NameCollection::MemoryBlockNames(vec![IndexNamePair {
                    index: 0,
                    name: "mem0".to_string(),
                }]),
                NameCollection::GlobalVariableNames(vec![IndexNamePair {
                    index: 0,
                    name: "global0".to_string(),
                }]),
            ])],
            type_items: vec![],
            import_items: vec![],
            internal_function_to_type_index_list: vec![],
            tables: vec![],
            memory_blocks: vec![],
            global_items: vec![],
            export_items: vec![
                ExportItem {
                    name: "f0".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(0),
                },
                ExportItem {
                    name: "f1".to_string(),
                    export_descriptor: ExportDescriptor::FunctionIndex(1),
                },
                ExportItem {
                    name: "t0".to_string(),
                    export_descriptor: ExportDescriptor::TableIndex(0),
                },
                ExportItem {
                    name: "t1".to_string(),
                    export_descriptor: ExportDescriptor::TableIndex(1),
                },
                ExportItem {
                    name: "m0".to_string(),
                    export_descriptor: ExportDescriptor::MemoryBlockIndex(0),
                },
                ExportItem {
                    name: "m1".to_string(),
                    export_descriptor: ExportDescriptor::MemoryBlockIndex(1),
                },
                ExportItem {
                    name: "g0".to_string(),
                    export_descriptor: ExportDescriptor::GlobalItemIndex(0),
                },
                ExportItem {
                    name: "g1".to_string(),
                    export_descriptor: ExportDescriptor::GlobalItemIndex(1),
                },
            ],
            start_function_index: None,
            element_items: vec![],
            code_items: vec![],
            data_items: vec![],
        };

        let name_package = NamePackage::new(&module);

        assert_eq!(
            module.export_items[0].to_text(&name_package, None),
            "(export \"f0\" (func $func0))"
        );

        assert_eq!(
            module.export_items[1].to_text(&name_package, None),
            "(export \"f1\" (func 1))"
        );

        assert_eq!(
            module.export_items[2].to_text(&name_package, None),
            "(export \"t0\" (table $tab0))"
        );

        assert_eq!(
            module.export_items[3].to_text(&name_package, None),
            "(export \"t1\" (table 1))"
        );

        assert_eq!(
            module.export_items[4].to_text(&name_package, None),
            "(export \"m0\" (memory $mem0))"
        );

        assert_eq!(
            module.export_items[5].to_text(&name_package, None),
            "(export \"m1\" (memory 1))"
        );

        assert_eq!(
            module.export_items[6].to_text(&name_package, None),
            "(export \"g0\" (global $global0))"
        );

        assert_eq!(
            module.export_items[7].to_text(&name_package, None),
            "(export \"g1\" (global 1))"
        );
    }

    #[test]
    fn test_element_item_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::FunctionNames(vec![IndexNamePair {
                    index: 0,
                    name: "func0".to_string(),
                }]),
                NameCollection::FunctionNames(vec![IndexNamePair {
                    index: 1,
                    name: "func1".to_string(),
                }]),
                NameCollection::ElementNames(vec![IndexNamePair {
                    index: 0,
                    name: "elem0".to_string(),
                }]),
            ])],
            type_items: vec![],
            import_items: vec![],
            internal_function_to_type_index_list: vec![],
            tables: vec![],
            memory_blocks: vec![],
            global_items: vec![],
            export_items: vec![],
            start_function_index: None,
            element_items: vec![
                ElementItem {
                    table_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(10), Instruction::End],
                    function_indices: vec![0, 1, 2, 3],
                },
                ElementItem {
                    table_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(20), Instruction::End],
                    function_indices: vec![4],
                },
            ],
            code_items: vec![],
            data_items: vec![],
        };

        let name_package = NamePackage::new(&module);

        assert_eq!(
            module.element_items[0].to_text(&name_package, Some(0)),
            "(elem $elem0 (offset (i32.const 10)) $func0 $func1 2 3)"
        );

        assert_eq!(
            module.element_items[1].to_text(&name_package, Some(1)),
            "(elem (;1;) (offset (i32.const 20)) 4)"
        );
    }

    #[test]
    fn test_data_item_to_text() {
        let module = Module {
            custom_items: vec![CustomItem::NameCollections(vec![
                NameCollection::DataNames(vec![IndexNamePair {
                    index: 0,
                    name: "data_foo".to_string(),
                }]),
            ])],
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
            data_items: vec![
                DataItem {
                    memory_block_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(10), Instruction::End],
                    data: vec![0x11, 0x22, 0x33],
                },
                DataItem {
                    memory_block_index: 0,
                    offset_instruction_items: vec![Instruction::I32Const(20), Instruction::End],
                    data: vec![0xaa, 0x0b, 0x09],
                },
            ],
        };

        let name_package = NamePackage::new(&module);

        assert_eq!(
            module.data_items[0].to_text(&name_package, Some(0)),
            "(data $data_foo (offset (i32.const 10)) \"\\11\\22\\33\")"
        );

        assert_eq!(
            module.data_items[1].to_text(&name_package, Some(1)),
            "(data (;1;) (offset (i32.const 20)) \"\\aa\\0b\\09\")"
        );
    }
}
