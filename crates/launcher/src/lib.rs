// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;
use std::path::Path;

use anvm_ast::types::Value;
use anvm_binary_parser::parser;
use anvm_disassembly::disassembler::module_to_text;
use anvm_engine::instance::{
    create_instance, find_ast_module_export_function, get_entry_module_and_function_index,
};
use anvm_engine::object::NamedAstModule;

pub fn disassembly(input_filepath: &str, output_filepath: &str) {
    println!(
        "disassembly \"{}\" into \"{}\"",
        input_filepath, output_filepath
    );

    let bytes: Vec<u8> = fs::read(input_filepath).expect(&format!(
        "failed to read the specified file: {}",
        input_filepath
    ));

    let module = parser::parse(&bytes).unwrap();
    let text = module_to_text(&module);

    fs::write(output_filepath, text).expect(&format!(
        "failed to write the specified file: {}",
        output_filepath
    ));

    println!("ok");
}

pub fn execute_function(
    module_filepaths: &[String],
    entry_module_function_name: Option<(String, String)>,
    function_arguments: &[Value],
    application_arguments: &[String],
) -> Result<(Vec<Value>, i32), String> {
    let named_ast_modules = load_ast_modules(module_filepaths)?;

    let (vm_module_index, function_index) =
        // 用户指定了入口模块及函数
        if let Some((target_module_name, target_function_name_or_index)) =
            entry_module_function_name
        {
            let (target_module_index, target_ast_module) = named_ast_modules
                .iter()
                .enumerate()
                .find_map(|(module_index, item)| {
                    if item.name == target_module_name {
                        Some((module_index, &item.module))
                    } else {
                        None
                    }
                })
                .ok_or(format!("no module \"{}\" found.", target_module_name))?;

            if let Ok(i) = target_function_name_or_index.parse::<u32>() {
                // 命令行当中的函数名称参数的值是一个整数
                (target_module_index, i as usize)
            } else {
                // 命令行当中的函数名称参数的值是一个字符串
                let target_function_index = find_ast_module_export_function(
                    target_ast_module,
                    &target_function_name_or_index,
                )
                .ok_or(format!(
                    "can not found the specified exported function \"{}\" in module \"{}\"",
                    target_function_name_or_index, target_module_name
                ))?;

                (target_module_index, target_function_index as usize)
            }
        } else {
            // 用户没有指定入口模块和函数，需要搜索 ast module 当中 `start` 段的值或者名称为 `main` 的导出函数
            get_entry_module_and_function_index(&named_ast_modules).ok_or("\
cannot find the entry function.
please specify the name of the entry module and function on the command line, e.g.

    $ anvm app.wasm -f app::function_name

function index is also supported, e.g.

    $ anvm app.wasm -f app::1
".to_string())?
        };

    println!(
        "execute function \"{}::{}\"",
        named_ast_modules[vm_module_index].name, function_index
    );

    let mut vm = create_instance(vec![], &named_ast_modules).map_err(|e| e.to_string())?;
    let results = vm
        .eval_function_by_index(vm_module_index, function_index, function_arguments)
        .map_err(|e| format!("execute function error: {}", e.to_string()))?;

    Ok((results, 0))
}

fn load_ast_modules(module_filepaths: &[String]) -> Result<Vec<NamedAstModule>, String> {
    let mut named_ast_modules: Vec<NamedAstModule> = vec![];

    for filepath in module_filepaths {
        let p = Path::new(filepath);
        let basename = p
            .file_stem()
            .ok_or(format!(
                "can not get the module name from file path \"{}\"",
                filepath
            ))?
            .to_str()
            .ok_or(format!(
                "file path \"{}\" contains invalid Unicode char",
                filepath
            ))?;

        let bytes =
            fs::read(filepath).map_err(|_| format!("failed to open file \"{}\"", filepath))?;

        let ast_module = parser::parse(&bytes).map_err(|e| e.to_string())?;

        let named_ast_module = NamedAstModule::new(basename, ast_module);
        named_ast_modules.push(named_ast_module);
    }

    Ok(named_ast_modules)
}
