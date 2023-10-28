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

// SSA (Single Static Assignment), hjälper optimering om varje variabel endast tilldelas en gång.

fn function() { let mut n = ((((((0)))))); let mut m = ((((((1)))))); for _ in 0..(((((((9)))))) as i32).abs() { m = (((((((m)))) + (((n)))))); n = (((((((m)))) - (((n))))));  } println!("{}", ((((((n)))))));  }

fn main() {
    let code = "
        # Fibonacci #

        let n be 0;
        let m be 1;

        rep 9 {
            set m to m + n;
            set n to m - n;
        }

        print n;
    ";

    let tokens = tokenize_program(code).expect("tokenization failed");

    let program = parse_program(tokens).expect("parsing failed");

    validate_program(&program).expect("semantic analysis failed");

    std::fs::write("fibonacci.rs", transpile_program_to_rust(&program)).expect("write failed");

    function();
}
