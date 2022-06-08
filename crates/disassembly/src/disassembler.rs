// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use anvm_ast::ast::Module;

use crate::{
    name_package::NamePackage,
    text_format::{format_function_item, format_import_items, get_function_items, TextFormat},
};

pub fn module_to_text(module: &Module) -> String {
    let mut lines: Vec<String> = vec![];

    let name_package = NamePackage::new(module);

    // 转换 type 段
    lines.extend(
        module
            .type_items
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_text(&name_package, Some(index as u32))),
    );

    // 转换 import 段
    let (import_lines, function_index, table_index, memory_block_index, global_variable_index) =
        format_import_items(&module.import_items, &name_package);

    lines.extend(import_lines);

    // 转换 table 段
    lines.extend(
        module
            .tables
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_text(&name_package, Some(index as u32 + table_index))),
    );

    // 转换 memory 段
    lines.extend(
        module
            .memory_blocks
            .iter()
            .enumerate()
            .map(|(index, item)| {
                item.to_text(&name_package, Some(index as u32 + memory_block_index))
            }),
    );

    // 转换 global 段
    lines.extend(module.global_items.iter().enumerate().map(|(index, item)| {
        item.to_text(&name_package, Some(index as u32 + global_variable_index))
    }));

    // 转换 export 段
    lines.extend(
        module
            .export_items
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_text(&name_package, Some(index as u32))),
    );

    // 转换 start 段
    if let Some(start_function_index) = module.start_function_index {
        if let Some(start_function_name) = name_package.get_function_name(&start_function_index) {
            lines.push(format!("(start ${})", start_function_name));
        } else {
            lines.push(format!("(start {})", start_function_index));
        }
    }

    // 转换 element 段
    lines.extend(
        module
            .element_items
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_text(&name_package, Some(index as u32))),
    );

    // 转换 function 段（二进制模块里的 function 段和 code 段）
    let function_items = get_function_items(&module); //, function_index as usize);
    lines.extend(
        function_items
            .iter()
            .enumerate()
            .map(|(index, function_item)| {
                format_function_item(function_item, &name_package, index as u32 + function_index)
            })
            .flatten(),
    );

    // 转换 data 段
    lines.extend(
        module
            .data_items
            .iter()
            .enumerate()
            .map(|(index, item)| item.to_text(&name_package, Some(index as u32))),
    );

    let mut text_fragments: Vec<String> = vec![];
    text_fragments.push("(module".to_string()); // 添加模块的首行
    text_fragments.extend(lines.iter().map(|line| format!("    {}", line)));
    text_fragments.push(")".to_string()); // 添加模块的尾行

    text_fragments.join("\n")
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use anvm_ast::ast;
    use anvm_binary_parser::parser;

    use pretty_assertions::assert_eq;

    use crate::disassembler::module_to_text;

    // 辅助方法
    fn get_test_binary_resource(filename: &str) -> Vec<u8> {
        let mut path_buf = env::current_dir().unwrap();

        // 使用 `cargo test` 测试时，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm/crates/engine`；
        //
        // 但如果使用 vscode 的源码编辑框里面的 `debug` 按钮开始调试，
        // `env::current_dir()` 函数获得的当前目录为
        // `./xiaoxuan-vm`。
        //
        // 下面语句用于处理这种情况。

        if !path_buf.ends_with("disassembly") {
            path_buf.push("crates");
            path_buf.push("disassembly");
        }
        let fullname_buf = path_buf.join("resources").join(filename);
        let fullname = fullname_buf.to_str().unwrap();
        fs::read(fullname).expect(&format!(
            "failed to read the specified binary file: {}",
            fullname
        ))
    }

    fn get_test_ast_module(filename: &str) -> ast::Module {
        let bytes = get_test_binary_resource(filename);
        parser::parse(&bytes).unwrap()
    }

    fn get_test_text_resource(filename: &str) -> String {
        let bytes = get_test_binary_resource(filename);
        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn test_module_to_text_1() {
        let m = get_test_ast_module("test-section-1.wasm");
        let t = module_to_text(&m);
        let e = get_test_text_resource("test-section-1.expect.wat");
        assert_eq!(t, e);
    }

    #[test]
    fn test_module_to_text_2() {
        let m = get_test_ast_module("test-section-2.wasm");
        let t = module_to_text(&m);
        let e = get_test_text_resource("test-section-2.expect.wat");
        assert_eq!(t, e);
    }

    #[test]
    fn test_module_to_text_3() {
        let m = get_test_ast_module("test-section-3.wasm");
        let t = module_to_text(&m);
        let e = get_test_text_resource("test-section-3.expect.wat");
        assert_eq!(t, e);
    }

    #[test]
    fn test_module_to_text_custom() {
        let m = get_test_ast_module("test-section-custom.wasm");
        let t = module_to_text(&m);
        let e = get_test_text_resource("test-section-custom.expect.wat");
        assert_eq!(t, e);
    }
}
