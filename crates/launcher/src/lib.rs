// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::{fs, io};

use anvm_ast::types::Value;
use anvm_binary_parser::parser;
use anvm_disassembly::disassembler::module_to_text;
use anvm_engine::error::{EngineError, NativeError};
use anvm_engine::instance::{
    create_instance, find_ast_module_export_function, get_entry_module_and_function_index,
};
use anvm_engine::object::NamedAstModule;
use anvm_native_wasi::wasi::new_wasi_module;
use anvm_native_wasi::wasi_module_context::{RealtimeClock, SandboxClock, WASIModuleContext};

pub fn execute_function(
    module_filepaths: &[String],
    entry_module_function_name: Option<(String, String)>,
    function_arguments: &[Value],
    application_arguments: &[String],
    environments: &[(String, String)],
) -> Result<(Vec<Value>, i32), String> {
    let named_ast_modules = load_ast_modules(module_filepaths)?;
    execute_function_by_modules(
        &named_ast_modules,
        entry_module_function_name,
        function_arguments,
        application_arguments,
        environments,
    )
}

/// 返回 (return_values, exit_code)
pub fn execute_function_by_modules(
    named_ast_modules: &[NamedAstModule],
    entry_module_function_name: Option<(String, String)>,
    function_arguments: &[Value],
    application_arguments: &[String],
    environments: &[(String, String)],
) -> Result<(Vec<Value>, i32), String> {
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
            // 用户没有指定入口模块和函数，需要搜索 ast module 当中 `start` 段的值或者名称为 `_start` 的导出函数
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

    let wasi_module_context = WASIModuleContext::new(
        &named_ast_modules[vm_module_index].name,
        application_arguments
            .iter()
            .map(|i| i.to_owned())
            .collect::<Vec<String>>(),
        environments
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<Vec<(String, String)>>(),
        Box::new(SandboxClock::new()),
        Box::new(RealtimeClock::new()),
        Rc::new(RefCell::new(io::stdin())),
        Rc::new(RefCell::new(io::stdout())),
        Rc::new(RefCell::new(io::stderr())),
    );

    let wasi_native_module = new_wasi_module(wasi_module_context);

    let mut vm =
        create_instance(vec![wasi_native_module], &named_ast_modules).map_err(|e| e.to_string())?;
    match vm.eval_function_by_index(vm_module_index, function_index, function_arguments) {
        Ok(results) => Ok((results, 0)),
        Err(e) => {
            match e {
                EngineError::NativeTerminate(ne) => match &ne.native_error {
                    NativeError::Exit(exit_code) => Ok((vec![], *exit_code)),
                    NativeError::Internal(internal_error) => Err(internal_error.to_string()),
                },
                _ => {
                    // todo::
                    // 将错误信息转换为可读的文本
                    Err(e.to_string())
                }
            }
        }
    }
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
