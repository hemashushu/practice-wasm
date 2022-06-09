use std::{env, fs};

use anvm_binary_parser::parser;
use anvm_disassembly::disassembler::module_to_text;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!(
            "\
Usage:

$ anvm app.wasm
$ anvm core.wasm std.wasm app.wasm
$ anvm app.wasm --function app::add 123 456
$ anvm --disassembly input.wasm output.wat

you can replace `anvm` with `cargo run --bin anvm --` if you don't want to compile the program before you run it. e.g.

$ cargo run --bin anvm -- app.wasm
"
        );
    } else {
        match args[1].as_str() {
            "--disassembly" | "-d" => {
                if args.len() != 4 {
                    println!(
                        "\
please specify the input and output file names, e.g.

$ anvm --disassembly input.wasm output.wat
$ anvm -d input.wasm output.wat
                    "
                    )
                } else {
                    let input_filepath = args[2].clone();
                    let output_filepath = args[3].clone();
                    disassembly(&input_filepath, &output_filepath);
                }
            }
            _ => {
                println!("running application has not implemented yet, try the disassembly instead");
            }
        }
    }
}

fn disassembly(input_filepath: &str, output_filepath: &str) {
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
