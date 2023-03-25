use std::env;
use std::fs;
use std::io::{self, BufRead};

mod assembler;
mod ir;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let arg1 = &args[1];
            if arg1 == "--repl" {
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    println!("{}", assembler::instr_to_mif(&0, &assembler::parse_line(&line.unwrap())));
                }
            } else {
                let contents = fs::read_to_string(arg1)
                    .expect("Unable to read given file");
                assembler::parse(contents, arg1.to_owned() + ".mif")
                    .expect("Unable to parse the given file")
            }
        },
        3 => {
            let in_file_name = &args[1];
            let out_file_name = &args[2];
            let contents = fs::read_to_string(in_file_name )
                .expect("Unable to read given file");
            assembler::parse(contents, out_file_name.to_owned())
                .expect("Unable to parse the given file")
        }
        _ => help()
    }
}

fn help() {
    println!("Usage: ./main [ input_file | --repl ] [ output_file ]?")
}
