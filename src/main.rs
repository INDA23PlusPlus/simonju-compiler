mod token;
mod lexer;
mod ast;
mod parser;
mod semantic_analyzer;
mod ssa;
mod generator;
mod rust_transpiler;

use lexer::tokenize_program;
use parser::parse_program;
use semantic_analyzer::validate_program;
use rust_transpiler::transpile_program_to_rust;

use std::{env, fs, process, error};

// SSA (Single Static Assignment), hjälper optimering om varje variabel endast tilldelas en gång.

// TODO: finish up the transpiler.
// TODO: fix the issue with left recursion not allowing statements such as "let x be 1 + 2 + 5;".
//       In the parser, functions assume that the left hand side is of a particular variant, 
//       meaning Token::And always results in Expr::And{ Expr::Sent, Sent },
//       instead of Expr::And{ Expr, Sent }. 
//       This results in operators of the same order being unable to be chained together.
//       1 * 1 * 1 and 1 + 1 + 1 does not work, but 1 * 1 + 1 * 1 does.
//       One weird one that does not work is 1 + 1 * 1 + 1.

struct Config<'a> {
    pub input_path: &'a str,
    pub output_path: &'a str,
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }

    let input_path = &args[1];
    let output_path = &args[2];

    Ok(Config { input_path, output_path })
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.input_path)?;

    let tokens = tokenize_program(&contents).expect("tokenization failed");

    println!("tokens = {:#?}", tokens);

    let program = parse_program(tokens).expect("parsing failed");

    println!("ast = {:#?}", program);

    validate_program(&program).expect("semantic analysis failed");

    std::fs::write(format!("{}.rs", config.output_path), transpile_program_to_rust(&program)).expect("write failed");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args).unwrap_or_else(
        |e| {
            println!("{e}");
            process::exit(1);
        }
    );

    if let Err(e) = run(config) {
        println!("{e}");
        process::exit(1);
    }
}
