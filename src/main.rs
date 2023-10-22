mod token;
mod lexer;
mod ast;
mod parser;
mod iter_clone;

use lexer::tokenize_program;
use parser::parse_program;

// SSA (Single Static Assignment), hjälper optimering om varje variabel endast tilldelas en gång.

fn main() {
    let code = "
        # Fibonacci #

        let x be (-1)*8;

        let n be 0;
        let m be 1;

        rep 9 {
            set m to m + n;
            set n to m - n;
        }

        print n;
    ";

    let tokens = tokenize_program(code).expect("tokenization failed");

    for token in tokens.clone() {
        println!("{token}");
    }

    let program = parse_program(tokens).expect("parsing failed");

    println!("Number of statements: {}", program.block.statements.len());

    println!("{program}");
}
