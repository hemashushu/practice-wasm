// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{instruction::Instruction, types::ValueType};

/// # 模块
///
/// 二进制和文本格式的 WebAssembly 对应的模块结构是相同的，这里以
/// 二进制模块的定义为主。
///
/// 结构的详细文档参阅：
/// <https://webassembly.github.io/spec/core/binary/modules.html>
///
#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    /// 自定义项目列表，（section id 0）
    pub custom_items: Vec<CustomItem>,

    /// 类型列表，（section id 1）
    /// 目前类型列表只支持列出函数类型（即函数签名），
    /// 需注意不同的函数可能有相同的签名，所以类型列表的数量并不等于函数的数量
    pub type_items: Vec<TypeItem>,

    /// 导入项列表，（section id 2）
    pub import_items: Vec<ImportItem>,

    /// 函数列表，（section id 3）
    /// 按顺序列出所有的**内部函数**所对应的类型，
    /// 函数的主体部分则位于代码列表。
    ///
    /// 注意，
    /// 内部函数的索引值并非总是从 0 开始，当一个模块有
    /// 导入的函数时，索引值从导入函数开始计算，所以第一个内部函数的索引值
    /// 等于导入函数的数量。
    pub function_list: Vec<u32>,

    /// 表格列表，（section id 4）
    /// 表格用于储存 `元素`，`表格` 和 `元素` 合在一起通常用于实现
    /// 函数的间接调用，目前只支持声明或导入 1 个表格
    pub tables: Vec<TableType>,

    /// 内存块描述列表，（section id 5）
    /// 目前只支持声明或导入 1 项内存块
    pub memory_blocks: Vec<MemoryType>,

    /// 全局变量列表，（section id 6）
    pub global_items: Vec<GlobalItem>,

    /// 导出项列表，（section id 7）
    pub export_items: Vec<ExportItem>,

    /// 应用程序入口函数的索引，（section id 8）
    /// 入口函数即 `主函数` 或者 `main 函数`
    pub start_function_index: Option<u32>,

    /// 元素项列表，（section id 9）
    /// 目前元素项只能用于列出函数索引
    pub element_items: Vec<ElementItem>,

    /// 函数的主体项列表，（section id 10）
    /// 函数的主体即一系列连续的指令
    pub code_items: Vec<CodeItem>,

    /// 内存的初始化数据，（section id 11）
    pub data_items: Vec<DataItem>,
}

/// # 自定义项
///
/// - 自定义段可以出现多次，出现的位置也不限。
///   一般用于存放函数的名称、局部变量（含参数）的名称等信息，不参与运算。
/// - 一个自定义段里可能有多个自定义项。
///
/// # 自定义段
///
/// ## 二进制格式
///
/// custom_section = 0x00:byte + content_length:u32 + name:string + byte{*}
///
/// - `0x00:byte` 是段 id，数据类型是 byte；
/// - `content_length:u32` 是该段的内容正文长度，即该段当中除了 `段 id` 以及 `content_length` 这两项之外的所有
///   数据的字节数，数据类型是 u32；
/// - `{*}` 表示重复 0 次或多次，`{+}` 表示重复 1 次或多次，`{?}` 表示重复 0 次或 1 次。
///
// #[derive(Debug, PartialEq, Clone)]
// pub struct CustomItem {
//     pub content: CustomItemContent,
// }

#[derive(Debug, PartialEq, Clone)]
pub enum CustomItem {
    NameCollections(Vec<NameCollection>),
    Other(String, Vec<u8>), // params: (name, data)
}

#[derive(Debug, PartialEq, Clone)]
pub enum NameCollection {
    TypeNames(Vec<IndexNamePair>),
    FunctionNames(Vec<IndexNamePair>),
    LocalVariableNamesPairList(Vec<FunctionIndexAndLocalVariableNamesPair>),
    BlockLabelsPairList(Vec<FunctionIndexAndBlockLabelsPair>),
    GlobalVariableNames(Vec<IndexNamePair>),
    MemoryBlockNames(Vec<IndexNamePair>),
    TableNames(Vec<IndexNamePair>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionIndexAndLocalVariableNamesPair {
    pub function_index: u32,
    pub local_variable_names: Vec<IndexNamePair>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionIndexAndBlockLabelsPair {
    pub function_index: u32,
    pub block_labels: Vec<IndexNamePair>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IndexNamePair {
    pub index: u32,
    pub name: String,
}

/// # 类型项
///
/// 记录函数、流程控制结构块等的类型
///
/// # 类型段
///
/// ## 二进制格式
///
/// type_section = 0x01:byte + content_length:u32 + function_type_items_count:u32 + function_type{+}
/// 其中 0x01 是段 id
///
/// > 注，为了简化起见，以下使用 `<...>` 代表 `items_count:u32 + item{*}` 这种结构
///
/// 上面的二进制格式简写为：
/// type_section = 0x01:byte + content_length:u32 + <function_type>
/// function_type = 0x60 + <value_type> + <value_type>
///                      ^       ^                 ^
///                      |       |--- 参数类型列表   |--- 返回值类型列表
///                      |
///                      |--- 目前 `类型项` 只支持函数类型， `0x60` 表示函数类型
///
/// - 上面的 <value_type> 是简写，表示 `value_type_count:u32 + value_type:byte{*}`
/// - 因为 WebAssembly 基本数据类型只有 4 种，所以 `value_type` 的数据类型是 byte
///
/// ## 文本格式
///
/// (type (func (param i32) (param i32) (result i32)))
///
/// 可以添加标识符（比如 `$ft1`）让编译器为自动生成的索引命名:
///
/// (type $ft1 (func (param i32) (param i32) (result i32)))
///
/// 多个参数的数据类型可以写在同一个 param 列表里，同样道理，多个返回值的数据类型
/// 也可以写在同一个 result 列表里：
///
/// (type $ft1 (func (param i32 i32) (result i32 i32)))
///
#[derive(Debug, PartialEq, Clone)]
pub enum TypeItem {
    FunctionType(FunctionType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionType {
    pub params: Vec<ValueType>,
    pub results: Vec<ValueType>,
}

/// # 导入项
///
/// 支持导入函数，还支持导入表格、内存块、全局变量：
///
/// # 导入段
///
/// ## 二进制格式
///
/// import_section = 0x02 + content_length:u32 + <import_item>
/// import_item = module_name:string + member_name:string + import_descriptor
/// import_description = tag:byte + (type_index | table_type | memory_type | global_type)
///
/// module_name 和 member_name 是字符串以 utf-8 编码后的字节数组。
///
/// > WebAssembly 会在字符串字节数组之前添加一个 u32 整数用于表明字节数组的长度（即字符串正文的长度），
///   注意这个长度值不包括这个当前这个 u32 整数本身占用的空间。
///
/// ## 文本格式
///
/// (type $ft1 (func (param i32 i32) (result i32)))
/// (import "env" "f1" (func $f1 (type $ft1)))
///
/// `env` 和 `f1` 分别是导入项的模块名和成员名，`func` 表示导入项的类型是函数。`$ft1` 和 `f1` 可以内联：
///
/// (import "env" "f1" (func $f1 (param i32 i32) (result i32))) ;; 导入函数
/// (import "env" "t1" (table $t 1 8 funcref))                  ;; 导入表格
/// (import "env" "m1" (memory $m 4 16))                        ;; 导入内存块
/// (import "env" "g1" (global $g1 i32))                        ;; 导入全局常量
/// (import "env" "g2" (global $g1 (mut i32)))                  ;; 导入全局变量
///
/// 导入项也可以内联到函数、表、内存和全局变量段当中：
///
/// (func $f1 (import "env" "f1") (type $ft1))
/// (table $t1 (import "env" "t1") 1 8 funcref)
/// (memory $m1 (import "env" "m1") 4 16)
/// (global $g1 (import "env" "g1") i32)
/// (global $g2 (import "env" "g2") (mut i32))
///
#[derive(Debug, PartialEq, Clone)]
pub struct ImportItem {
    pub module_name: String,
    pub item_name: String,
    pub import_descriptor: ImportDescriptor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImportDescriptor {
    FunctionTypeIndex(u32), // 导入函数
    TableType(TableType),   // 导入表
    MemoryType(MemoryType), // 导入内存块
    GlobalType(GlobalType), // 导入全局变量
}

/// # 补充 A：函数（列表）段
///
/// 函数列表列出所有的函数所对应的类型，至于函数的主体（指令列表）部分则位于代码列表
///
/// ## 二进制格式
///
/// function_section = 0x03 + content_length:u32 + <type_index>
///
/// 函数列表仅列出该函数的类型的索引，比如 function_types 里有 2 条记录：
///
/// - type0 = (u32, u32) u32
/// - type1 = (f32) u32
///
/// 则函数列表 `00 01 01 00` 表示一共有 4 个函数：
///
/// - func0 (u32, u32) u32
/// - func1 (f32) u32
/// - func2 (f32) u32
/// - func3 (u32, u32) u32
///
/// 注意：
///
/// 函数的索引有可能不是从 0 开始，比如导入了 3 个函数，则这个列表的第一个函数
/// 的索引应该是 3。
///
/// ## 文本格式
///
/// (type $ft1 (func (param i32 i32) (result i32)))
/// (func $add (type $ft1)                      ;; $ft1 是类型索引的标识符，$add 是函数索引的标识符
///     (local i64 i64)                         ;; 声明两个局部变量
///     (i64.add (local.get 2) (local.get 3))   ;; 访问上面两个局部变量，local.get 指令使用了内联方式书写
///     (drop)
///     (i32.add (local.get 0) (local.get 1))   ;; 访问函数的两个参数，函数参数也是局部变量
/// )
///
/// 局部变量（包括函数的参数）也可以用标识符替代数字索引值：
///
/// (func $add (param $a i32) (param $b i32) (result i32)
///     (local $x i64)
///     (local $y i64)
///     (i64.add (local.get $x) (local.get $y)) ;; 数字索引换成了标识符
///     (drop)
///     (i32.add (local.get $a) (local.get $b))
/// )

/// # 表类型
///
/// 表段只存储了 `表类型` 这一项数据
///
/// # 表段
///
/// `表` 用于存储 `元素项`，目前 `元素项` 只支持函数引用（函数索引），
/// `表` 和 `元素项` 用于列出一组函数的索引，然后在执行 `call_indirect` 指令时，
/// 根据栈顶的操作数获取该列表中的一个函数，从而实现 `动态` 选择被调用的函数。
///
/// `动态函数调用` 相当于高级语言里的 `函数指针`（或者数据类型为 `函数` 的参数）
///
/// ## 二进制格式
///
/// table_section = 0x04 + content_length:u32 + <table_type> // 目前一个模块仅支持声明一个表项
/// table_type = 0x70 + limits
///              ^
///              |--- 0x70 表示该表项存储的是 funcref
///
/// 表项仅用于说明表的容量，真正的内容（即函数索引列表）被存储在元素项里，
/// 元素段存储的是表的初始化数据。
///
/// ## 文本格式
///
/// (func $f1)
/// (func $f2)
/// (table 1 10 funcref)                    ;; 表的类型暂时只能是 `funcref`
/// (elem (offset (i32.const 1)) $f1 $f2)   ;; 元素项的偏移值需要使用 `const`表达式
///
/// 元素项也可以内联到表段里：
///
/// (table funcref          ;; 自动决定了表的 limit 值，即 min = 2, max = 2（max 与 min 的值相同）
///     (elem $f1 $f2)      ;; 自动决定了偏移值为 0
/// )
///
#[derive(Debug, PartialEq, Clone)]
pub struct TableType {
    pub limit: Limit,
}

/// # 限制值
///
/// Limit 用于表示内存块和表的最小值（min）和最大值（max），
/// 其中 max 值可以省略，当省略 max 值时（此时 max 的值为 0）表示不限制上限
///
/// min 和 max 都是闭区间，比如 (1,10) 表示从 1 到 10，包括 1 和 10。
///
/// ## 二进制格式
///
/// limits = tag:byte + min:u32 + max:u32
///
/// min 是下限值，max 是上限值
/// 当 tag == 0 时，表示省略了上限，只有 min 值
/// 当 tag == 1 时，表示同时指出了 min 值和 max 值
///
/// 示例：
///
/// 00 01       ;; tag == 0，min 值为 1，省略了 max 值（所以 max 值对应的字节序列也不会有）
/// 01 01 02    ;; tag == 1，min 值为 1，max 值为 2
///
#[derive(Debug, PartialEq, Clone)]
pub enum Limit {
    Range(u32, u32),
    AtLeast(u32),
}

impl Limit {
    pub fn new_by_min(min: u32) -> Self {
        Limit::AtLeast(min)
    }

    /// 根据指定范围来创建 Limit 实例
    ///
    /// 跟常见程序语言里的 range 不一样，min 和 max 的值都是 `包括的`（`included`），
    /// 比如 Range(0, 10) 表示从 0 到 10 一共 11 个数字，因为数字 `10` 是 `包括的`。
    pub fn new_by_range(min: u32, max: u32) -> Self {
        Limit::Range(min, max)
    }

    pub fn get_min(&self) -> u32 {
        match *self {
            Limit::AtLeast(min) => min,
            Limit::Range(min, _) => min,
        }
    }
}

/// # 内存类型
///
/// 内存段里只存储了 `内存类型` 这一项数据
///
/// # 内存段
///
/// 内存段用于声明模块的内存块信息（即 `内存类型`），内存块的初始化数据位于数据段里。
///
/// ## 二进制格式
///
/// memory_section = 0x05 + content_length:u32 + <memory_type> // 目前一个模块仅支持声明一个内存块
/// memory_type = limits
///
/// ## 文本格式
///
/// (memory 1 16)                           ;; 指定 limit 值，即 min 和 max
/// (data (offset (i32.const 10)) "foo")    ;; 数据偏移量需要使用 `const` 表达式
/// (data (offset (i32.const 20)) "bar")
///
/// 将数据内联到内存段里：
///
/// (memory             ;; 自动 limit, min = 1, max = 1
///     (data "foo")    ;; 自动偏移值 0
///     (data "bar")
/// )
///
/// WebAssembly 规范里使用名称 `memory` 表示一个连续（线性）的字节区域，
/// 目前一个模块只能定义或者导入一个 `memory`，但以后可能会支持多个 `memory`，
/// 为了避免歧义，在 `Module` 里，使用 `memory block`（即 `内存块`）来命名内存项。
///
#[derive(Debug, PartialEq, Clone)]
pub struct MemoryType {
    pub limit: Limit,
}

/// # 全局变量项
///
/// 全局段里所存储的数据
///
/// # 全局段
///
/// 全局段列出模块所有全局变量（包括全局常量），全局变量项
/// 需要指出变量是否可变，以及初始值（使用常量表达式）。
///
/// ## 二进制格式
///
/// global_section = 0x06 + content_length:u32 + <global_item>
/// global_item = global_type + initialize_expression
/// global_type = val_type:byte + mut:byte
/// mut = (0|1)                             // 0 表示不可变，1 表示可变
/// initialize_expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
///
/// 全局项示例：
///
/// - 7f                ;; 当前全局变量的数据类型是 i32
/// - 01                ;; 表示该变量的值可变
/// - 41 80 80 c0 00    ;; 初始值表达式开始，指令是 i32.const 0x100000
/// - 0b                ;; 初始值表达式结束
///
/// ## 文本格式
///
/// (global $g0 (mut i32) (i32.const 10))   ;; 全局变量
/// (global $g1 i32 (i32.const 20))         ;; 全局常量
/// (func
///     (global.get $g0)
///     (global.get $g1)
/// )
///
#[derive(Debug, PartialEq, Clone)]
pub struct GlobalItem {
    pub global_type: GlobalType,
    pub initialize_instruction_items: Vec<Instruction>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GlobalType {
    pub value_type: ValueType,
    pub mutable: bool,
}

/// # 导出项
///
/// 一个模块可以导出：函数、表、内存、全局变量
///
/// # 导出段
///
/// ## 二进制格式
///
/// export_section = 0x07 + content_length:u32 + <export_item>
/// export_item = name:string + export_descriptor
/// export_descriptor = tag:byte + (function_index | table_index | memory_block_index | global_item_index)
///
/// ## 文本格式
///
/// (export "f1" (func $f1))
/// (export "f2" (func $f2))
/// (export "t1" (table $t1))
/// (export "m1" (memory $m1))
/// (export "g1" (global $g1))
/// (export "g2" (global $g2))
///
/// 导出项也可以内联到函数、表、内存、全局变量
///
/// (func $f (export "f1") ...)
/// (func $t (export "t") ...)
/// (func $m (export "m") ...)
/// (func $g (export "g1") ...)
///
#[derive(Debug, PartialEq, Clone)]
pub struct ExportItem {
    /// 导出项的名称
    /// 导出项不需要指定当前模块的名称（注：导入时则需同时指出导入模块和导入项的名称）
    pub name: String,

    /// 导出项描述
    pub export_descriptor: ExportDescriptor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExportDescriptor {
    FunctionIndex(u32),    // 导出函数，参数为函数索引
    TableIndex(u32),       // 导出表，参数为表的索引
    MemoryBlockIndex(u32), // 导出内存块，参数为内存块索引
    GlobalItemIndex(u32),  // 导出全局变量，参数为全局变量项目的索引
}

/// # 补充 B：起始函数索引段
///
/// 指定 wasm 加载后自动开始执行的函数（比如 main 函数）
///
/// ## 二进制格式
///
/// start_section: 0x08 + content_length:u32 + function_index
///
/// ## 文本格式
///
/// (module
///     (func $main ...)
///     (start $main)       ;; start 指令后面跟着起始函数的索引值或者标识符
/// )

/// # 元素项
///
/// 元素项的内容目前只能是函数的索引。
///
/// # 元素段
///
/// 元素段用于存储表的初始化数据
///
/// ## 二进制格式
///
/// element_section = 0x09 + content_length:u32 + <element_item>
/// element_item = table_index:u32 + offset_expression + <function_index>
/// offset_expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
///
/// 元素段里的每个项目的内容由 3 部分组成：
/// 1. 表的索引，因为目前一个模块只支持一张表，所以它的值恒等于 0；
/// 2. 表内偏移量，是一个常量表达式；
/// 3. 函数索引值列表
///
/// ## 文本格式
///
/// (elem (offset (i32.const 1)) $f1 $f2)   ;; 元素项的偏移值需要使用（const）表达式
///
/// 元素项也可以内联到表段里：
///
/// (table funcref (elem $f1 $f2))          ;; 元素项的偏移值会自动从 0 开始计算
///
#[derive(Debug, PartialEq, Clone)]
pub struct ElementItem {
    /// 表索引，目前 WebAssembly 标准只支持 0
    pub table_index: u32,

    /// 偏移值表达式（指令列表）
    pub offset_instruction_items: Vec<Instruction>,

    /// 函数索引列表
    /// function_indices 这个列表会从指定的偏移值开始，把一系列函数的索引紧密排列，
    /// 但这一组函数之间并没有必然的关联，只是恰好排列在一起而已。
    pub function_indices: Vec<u32>,
}

/// # 代码项
///
/// 一个函数对应这一项代码项
///
/// # 代码段
///
/// ## 二进制格式
///
/// code_section = 0x0a + content_length:u32 + <code_item>
/// code_item = code_length:u32 + <local_group> + expression
/// local_group = local_variable_count:u32 + value_type:byte
/// expression = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
///
/// code_length 表示该项目的内容总大小，包括表达式结尾的 0x0B。
///
/// 示例：
///
/// ```wat
/// (func (param $a i32) (param $b i32)
///     (local $la i32)
///     (local $lb i32)
///     (local i64 i64)
///     (global.get $g1)
///     (global.set $g2)
///     (local.get $a)
///     (local.set $b)
/// )
///
/// - 0e          | size of function
/// - 02          | 2 local blocks
/// - 02 7f       | 2 locals of type I32
/// - 02 7e       | 2 locals of type I64
/// - 23 00       | GlobalGet { global_index: 0 }   ;; 表达式开始
/// - 24 01       | GlobalSet { global_index: 1 }   ;;
/// - 20 00       | LocalGet { local_index: 0 }     ;;
/// - 21 01       | LocalSet { local_index: 1 }     ;;
/// - 0b          | End                             ;; 表达式结束
/// ```
///
/// ## 文本格式
///
/// 表达式是一系列指令的罗列，比如：
///
/// ```wat
/// ...
/// block
/// i32.const 10
/// end
/// ```
///
/// 指令序列可以折叠，方法是在先在指令前后加上括号：
///
/// ```wat
/// ...
/// (block)
/// (i32.const 10)
/// (end)
/// ```
///
/// 然后将部分指令放置在其他允许折叠的指令（比如二元运算指令、块结构指令等），于是
/// 上面的代码可以进一步可以写成：
///
/// ```wat
/// ...
/// (block
///     (i32.const 10)
/// )
/// ```
///
/// 下面一则来自官方文档的示例：
/// <https://webassembly.github.io/spec/core/text/instructions.html#folded-instructions>
///
/// ```wat
/// ...
/// (local.get $x) (i32.const 2) i32.add (i32.const 3) i32.mul
///
/// (i32.mul
///     (i32.add (local.get $x) (i32.const 2))
///     (i32.const 3)
/// )
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct CodeItem {
    /// 局部变量组列表，连续多个相同类型的局部变量被分为一组
    pub local_groups: Vec<LocalGroup>,

    /// 指令列表
    pub instruction_items: Vec<Instruction>,
}

/// 指令项
///
/// 指令项包括了指令本身（类型和参数）以及指令的位置等信息
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionItem {
    pub instruction: Instruction,
    pub location: Location,
}

/// 局部变量信息组
#[derive(Debug, PartialEq, Clone)]
pub struct LocalGroup {
    pub variable_count: u32,   // 变量的数量
    pub value_type: ValueType, // 数据类型
}

/// 指令在源码（包括二进制和文本格式）当中的位置
#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// # 数据项
///
/// 数据段存储着内存的初始化数据
///
/// # 数据段
///
/// ## 二进制格式
///
/// data_section = 0x0b + content_length:u32 + <data_item>
/// data_item = memory_block_index:u32 + offset_expression + data:byte{*}
/// offset_expression = = byte{*} + 0x0B  // 表达式（指令列表）以 0x0B 结尾
///
/// 数据段每一项由 3 部分组成：
/// 1. 内存块索引，因为目前一个模块只能有一块内存，所以这个值恒等于 0
/// 2. 内存偏移值，一个常量表达式
/// 3. 初始数据
///
/// 数据项示例：
/// - 00                ;; 内存块索引
/// - 41 80 80 c0 00    ;; 偏移值表达式开始，指令是 i32.const(0x41) 0x100000
/// - 0b                ;; 偏移值表达式结束标记
/// - 0e                ;; 内容长度 14 字节（0x0e）
/// - 48 65 6c 6c 6f    ;; "Hello"
/// - 2c 20             ;; ", "
/// - 57 6f 72 6c 64    ;; "World"
/// - 21 0A             ;; "!\n"
///
/// ## 文本格式
///
/// 初始数据使用字符串的形式指定，内容可以是
/// - 单一字符："abc文字"（字符将会以 utf-8 形式编码）
/// - 十六进制 byte: "\de\ad\be\ef\00"
/// - Unicode code point: "\u{1234}\u{5678}"
///
#[derive(Debug, PartialEq, Clone)]
pub struct DataItem {
    /// 内存块索引，目前 WebAssembly 标准只支持 0
    pub memory_block_index: u32,

    /// 偏移值表达式（指令列表）
    pub offset_instruction_items: Vec<Instruction>,

    /// 内容
    pub data: Vec<u8>,
}
