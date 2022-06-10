// Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs;

use anvm_ast::types::Value;
use anvm_binary_parser::parser;
use anvm_disassembly::disassembler::module_to_text;

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
    module_filenames: &[String],
    entry_module_function_name: Option<(String, String)>,
    function_arguments: &[Value],
    application_arguments: &[String],
) {
    println!("mods: {:#?}", module_filenames);
    println!("exec func: {:#?}", entry_module_function_name);
    println!("func args: {:#?}", function_arguments);
    println!("app args: {:#?}", application_arguments);
}
