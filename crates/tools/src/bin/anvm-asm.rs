// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env;

use anvm_tools::{assembly, disassembly};

fn main() {
    print_version();
    let original_args: Vec<String> = env::args().collect();

    if original_args.len() == 1 {
        print_usage();
    } else {
        let args = &original_args[1..];
        let input_filename = &args[0];
        if args.len() > 1 {
            //
        }
    }
}

fn print_version() {
    println!(
        "\
XiaoXuan VM Assembly Tools v0.1.0-beta
Translate between the WebAssembly binary and text format
"
    );
}

fn print_usage() {
    println!(
        "\
USAGE:

    $ anvm-asm module_file_name [-o output_file_name]

OPTIONS:

    -o, --output        Specify the output file name

EXAMPLES:

    Translate the WebAssembly binary format to text

    $ anvm-asm app.wasm

    Translate the WebAssembly text format to binary

    $ anvm-asm app.wat

    Translate the WebAssembly binary format to text to a specified file

    $ anvm-asm app.wasm -o /path/to/app.wat
"
    );
}

fn find_option_value(short_name: &str, full_name: &str) -> Result<Option<String>, ()> {
    todo!()
}

fn process_disassembly_command(fragments: &[String]) {
    if fragments.len() != 2 {
        println!(
            "\
Please specify both the input and output file names, e.g.

    $ anvm input.wasm -t output.wat
    $ anvm input.wasm --text output.wat
"
        )
    } else {
        let input_filepath = &fragments[0];
        let output_filepath = &fragments[1];
        disassembly(input_filepath, output_filepath);
    }
}

fn process_assembly_command(fragments: &[String]) {
    if fragments.len() != 2 {
        println!(
            "\
Please specify both the input and output file names, e.g.

    $ anvm input.wat -b output.wasm
    $ anvm input.wat --binary output.wasm
"
        )
    } else {
        let input_filepath = &fragments[0];
        let output_filepath = &fragments[1];
        assembly(input_filepath, output_filepath);
    }
}
