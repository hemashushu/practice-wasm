// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::name_package::NamePackage;
use anvm_ast::instruction::{BlockType, Instruction, MemoryArgument};
use std::fmt::Write;

/// 转换指令到文本格式
pub trait TextFormat {
    /// 参数 function_index 为指令所在的函数的索引
    ///
    /// - block/loop/if 等流程控制结构块指令允许有 `标签`（即名称），而名称是按照函数分组存储于
    ///   模块的自定义项当中，所以在反汇编这 3 个指令时，需要函数的索引值
    /// - 全局变量的初始值、元素项和数据项的偏移值等常量表达式之中的指令则不需要 function_index。
    fn text(
        &self,
        name_package: &NamePackage,
        option_function_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result;

    fn to_text(
        // instruction_convert: &dyn TextFormat,
        &self,
        name_package: &NamePackage,
        option_function_index: Option<u32>,
    ) -> String {
        let mut s = String::new();
        self.text(name_package, option_function_index, &mut s)
            .unwrap();
        s
    }
}

impl TextFormat for BlockType {
    fn text(
        &self,
        name_package: &NamePackage,
        _option_function_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
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
        _option_function_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        write!(f, "offset={} align={}", self.offset, 2i32.pow(self.align))
    }
}

impl TextFormat for Instruction {
    fn text(
        &self,
        name_package: &NamePackage,
        option_function_index: Option<u32>,
        f: &mut String,
    ) -> std::fmt::Result {
        match self {
            Self::Unreachable => write!(f, "unreachable"),
            Self::Nop => write!(f, "nop"),
            Self::Block(block_type, block_index) => {
                let mut fragments: Vec<String> = vec![];
                fragments.push("block".to_string());

                if let Some(function_index) = option_function_index {
                    let option_block_label =
                        name_package.get_block_lable(&function_index, block_index);
                    if let Some(block_label) = option_block_label {
                        fragments.push(format!("${}", block_label));
                    }
                }

                let block_type_text = block_type.to_text(name_package, None);
                if block_type_text != "" {
                    fragments.push(block_type_text);
                }

                write!(f, "{}", fragments.join(" "))
            }
            Self::Loop(block_type, block_index) => {
                let mut fragments: Vec<String> = vec![];
                fragments.push("loop".to_string());

                if let Some(function_index) = option_function_index {
                    let option_block_label =
                        name_package.get_block_lable(&function_index, block_index);
                    if let Some(block_label) = option_block_label {
                        fragments.push(format!("${}", block_label));
                    }
                }

                let block_type_text = block_type.to_text(name_package, None);
                if block_type_text != "" {
                    fragments.push(block_type_text);
                }

                write!(f, "{}", fragments.join(" "))
            }
            Self::If(block_type, block_index) => {
                let mut fragments: Vec<String> = vec![];
                fragments.push("if".to_string());

                if let Some(function_index) = option_function_index {
                    let option_block_label =
                        name_package.get_block_lable(&function_index, block_index);
                    if let Some(block_label) = option_block_label {
                        fragments.push(format!("${}", block_label));
                    }
                }

                let block_type_text = block_type.to_text(name_package, None);
                if block_type_text != "" {
                    fragments.push(block_type_text);
                }

                write!(f, "{}", fragments.join(" "))
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
                    if let Some(function_index) = option_function_index {
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
                    if let Some(function_index) = option_function_index {
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
                    if let Some(function_index) = option_function_index {
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
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Load(memory_argument) => write!(
                f,
                "i64.load {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::F32Load(memory_argument) => write!(
                f,
                "f32.load {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::F64Load(memory_argument) => write!(
                f,
                "f64.load {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I32Load8S(memory_argument) => write!(
                f,
                "i32.load8_s {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I32Load8U(memory_argument) => write!(
                f,
                "i32.load8_u {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I32Load16S(memory_argument) => write!(
                f,
                "i32.load16_s {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I32Load16U(memory_argument) => write!(
                f,
                "i32.load16_u {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Load8S(memory_argument) => write!(
                f,
                "i64.load8_s {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Load8U(memory_argument) => write!(
                f,
                "i64.load8_u {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Load16S(memory_argument) => write!(
                f,
                "i64.load16_s {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Load16U(memory_argument) => write!(
                f,
                "i64.load16_u {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Load32S(memory_argument) => write!(
                f,
                "i64.load32_s {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Load32U(memory_argument) => write!(
                f,
                "i64.load32_u {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I32Store(memory_argument) => write!(
                f,
                "i32.store {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Store(memory_argument) => write!(
                f,
                "i64.store {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::F32Store(memory_argument) => write!(
                f,
                "f32.store {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::F64Store(memory_argument) => write!(
                f,
                "f64.store {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I32Store8(memory_argument) => write!(
                f,
                "i32.store8 {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I32Store16(memory_argument) => write!(
                f,
                "i32.store16 {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Store8(memory_argument) => write!(
                f,
                "i64.store8 {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Store16(memory_argument) => write!(
                f,
                "i64.store16 {}",
                memory_argument.to_text(name_package, option_function_index)
            ),
            Self::I64Store32(memory_argument) => write!(
                f,
                "i64.store32 {}",
                memory_argument.to_text(name_package, option_function_index)
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
            Self::F32ReinterpretI32 => write!(f, "i32.reinterpret_i32"),
            Self::F64ReinterpretI64 => write!(f, "i64.reinterpret_i64"),

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

#[cfg(test)]
mod tests {
    use anvm_ast::{
        ast::{
            CustomItem, FunctionIndexAndBlockLabelsPair, FunctionIndexAndLocalVariableNamesPair,
            IndexNamePair, Module, NameCollection,
        },
        instruction::{BlockType, Instruction, MemoryArgument},
    };
    use pretty_assertions::assert_eq;

    use crate::{name_package::NamePackage, text_format::TextFormat};

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
}
