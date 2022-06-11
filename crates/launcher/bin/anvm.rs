// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env;

use anvm_ast::types::Value;
use anvm_launcher::{disassembly, execute_function};

/// 编译之后将会得到程序 `./target/debug/anvm`
/// 然后通过诸如 `$ anvm app.wasm` 命令来运行 WebAssembly 应用程序，
///
/// 有时如果不想编译就直接运行 XiaoXuan VM（比如正在修改及调试 XiaoXuan VM 程序），
/// 也可以通过命令 `cargo run` 来启动 XiaoXuan VM，例如：
///
/// `$ cargo run --bin anvm -- app.wasm`
///
/// 这跟 `$ anvm app.wasm` 的效果是一样的，也就是说，
/// `$ cargo run --bin anvm -- ...` 等价于 `$ anvm ...`。
fn main() {
    print_version();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_usage();
    } else {
        match args[1].as_str() {
            "-d" | "--disassembly" => {
                process_disassembly_command(&args[2..]);
            }
            _ => {
                process_execute_function_command(&args[1..]);
            }
        }
    }
}

fn print_version() {
    println!(
        "\
XiaoXuan VM 0.1.0-beta
"
    );
}

fn print_usage() {
    println!(
        "\
Usage:

    $ anvm module_names
       [-f module_name::function_name arg0 ... argN]
       [-- command -o --option -arg0 val0 -arg1=val1 --argumentN valueN]

e.g.

    $ anvm app.wasm
    $ anvm math.wasm app.wasm
    $ anvm app.wasm --function app::test_add 10 20
    $ anvm console.wasm -- command
    $ anvm console.wasm -- convert -d 123 --format hex
    $ anvm --disassembly input.wasm output.wat
"
    );
}

fn process_disassembly_command(fragments: &[String]) {
    if fragments.len() != 2 {
        println!(
            "\
Please specify both the input and output file names, e.g.

    $ anvm --disassembly input.wasm output.wat
    $ anvm -d input.wasm output.wat
"
        )
    } else {
        let input_filepath = &fragments[0];
        let output_filepath = &fragments[1];
        disassembly(input_filepath, output_filepath);
    }
}

fn process_execute_function_command(fragments: &[String]) {
    let mut module_filepaths: Vec<String> = vec![];
    let mut entry_module_function_name: Option<(String, String)> = None;
    let mut function_arguments: Vec<Value> = vec![];
    let mut application_arguments: Vec<String> = vec![];

    let mut remains = fragments;
    let mut found_arguments: bool = false;

    loop {
        remains = match remains.split_first() {
            Some((first, rest)) => match first.as_str() {
                "-f" | "--function" => {
                    found_arguments = true;

                    match continue_parse_entry_module_function_name(rest) {
                        Ok((e, f, post_mod_func_name)) => {
                            entry_module_function_name = e;
                            function_arguments = f;
                            post_mod_func_name
                        }
                        Err(message) => {
                            println!("{}", message);
                            return;
                        }
                    }
                }
                "--" => {
                    found_arguments = true;

                    match continue_parse_application_arguments(rest) {
                        Ok((a, post_app_args)) => {
                            application_arguments = a;
                            post_app_args
                        }
                        Err(message) => {
                            println!("{}", message);
                            return;
                        }
                    }
                }
                _ => {
                    if found_arguments || first.starts_with("-") {
                        println!(
                            "\
Unexpected VM launcher argument: \"{}\"
",
                            first
                        );
                        print_usage();
                        return;
                    }

                    module_filepaths.push(first.to_owned());
                    rest
                }
            },
            None => {
                break;
            }
        };
    }

    match execute_function(
        &module_filepaths,
        entry_module_function_name,
        &function_arguments,
        &application_arguments,
    ) {
        Ok((results, exit_code)) => {
            println!(
                "\
program exit normally with code: {}",
                exit_code
            );

            if results.len() > 0 {
                println!(
                    "\
function return values: [{}]",
                    results
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
        }
        Err(e) => {
            println!(
                "\
program terminated unexpectedly, error message: {}",
                e
            )
        }
    }
}

fn continue_parse_entry_module_function_name(
    fragments: &[String],
) -> Result<(Option<(String, String)>, Vec<Value>, &[String]), String> {
    // --f module_name::function_name arg0 ... argN
    //     ^------------------------^

    if let Some((module_function_name_pair, rest)) = fragments.split_first() {
        let parts: Vec<&str> = module_function_name_pair.split("::").collect();
        if parts.len() != 2 {
            Err(format!(
                "\
Wrong format of function name: {}

please specify the name of entry module and function as \"module_name::function_name\", e.g.

    $ anvm app.wasm --function app::test_add

where the module name is the base name of the WASM file (i.e. file name without extension)
",
                module_function_name_pair
            ))
        } else {
            let module_name = parts[0].to_owned();
            let function_name = parts[1].to_owned();
            let entry_module_function_name = Some((module_name, function_name));

            if rest.len() > 0 {
                let (function_arguments, post_func_args) = continue_parse_function_arguments(rest)?;
                Ok((
                    entry_module_function_name,
                    function_arguments,
                    post_func_args,
                ))
            } else {
                Ok((entry_module_function_name, vec![], rest))
            }
        }
    } else {
        Err("\
Please specify the name of entry module and function as \"module_name::function_name\", e.g.

    $ anvm app.wasm --function app::test_add

where the module name is the base name of the WASM file name (i.e. without extension). \
you can also specify the parameters of the function, e.g.

    $ anvm app.wasm --function app::test_add 10 20
            "
        .to_string())
    }
}

fn continue_parse_function_arguments(
    fragments: &[String],
) -> Result<(Vec<Value>, &[String]), String> {
    // --f module_name::function_name arg0 ... argN
    //                                ^-----------^
    // function argument data types:
    //
    // | type | example       |
    // | ---- | ------------- |
    // | i32  | 123    123i   |
    // | i64  | 3l     345l   |
    // | f32  | 3.142  3.142f |
    // | f64  | 2d     2.718d |

    let mut remains = fragments;
    let mut function_args: Vec<Value> = vec![];

    loop {
        remains = match remains.split_first() {
            Some((first, rest)) => {
                if first == "--" {
                    break;
                } else {
                    let value = if first.ends_with("d") {
                        // f64
                        let a = &first[0..first.len() - 1];
                        a.parse::<f64>().map(|f| Value::F64(f)).map_err(|_| ())
                    } else if first.ends_with("f") {
                        // f32
                        let a = &first[0..first.len() - 1];
                        a.parse::<f32>().map(|f| Value::F32(f)).map_err(|_| ())
                    } else if first.ends_with("l") {
                        // i64
                        let a = &first[0..first.len() - 1];
                        a.parse::<i64>().map(|i| Value::I64(i)).map_err(|_| ())
                    } else if first.ends_with("i") {
                        // i32
                        let a = &first[0..first.len() - 1];
                        a.parse::<i32>().map(|i| Value::I32(i)).map_err(|_| ())
                    } else if first.contains('.') {
                        // 带有小数点、且无类型后缀的数字默认作为 f32
                        first.parse::<f32>().map(|f| Value::F32(f)).map_err(|_| ())
                    } else {
                        // 无小数点、且无类型后缀的数字默认作为 i32
                        first.parse::<i32>().map(|i| Value::I32(i)).map_err(|_| ())
                    };

                    if let Ok(v) = value {
                        function_args.push(v);
                        rest
                    } else {
                        return Err(format!(
                            "\
failed to parse \"{}\" as a function argument, only integer and float number are supported, e.g.

    | type | example       |
    | ---- | ------------- |
    | i32  | 123    123i   |
    | i64  | 3l     345l   |
    | f32  | 3.142  3.142f |
    | f64  | 2d     2.718d |",
                            first
                        ));
                    }
                }
            }
            _ => {
                break;
            }
        };
    }

    Ok((function_args, remains))
}

fn continue_parse_application_arguments(
    fragments: &[String],
) -> Result<(Vec<String>, &[String]), String> {
    // -- command -o --option -arg0 val0 -arg1=val1 --argumentN valueN
    //    ^----------------------------------------------------------^
    //
    // 注意，参数之间使用空格分隔，双引号或者单引号包围起来的字符串中间的空格不会作为分隔符。

    let mut remains = fragments;
    let mut application_args: Vec<String> = vec![];

    loop {
        remains = match remains.split_first() {
            Some((first, rest)) => {
                application_args.push(first.to_owned());
                rest
            }
            _ => {
                break;
            }
        };
    }

    Ok((application_args, remains))
}
