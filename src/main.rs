mod token;
mod lexer;

use lexer::tokenize;

fn main() {
    let code =  "let x eat 7; if x = 7 out x;";

    let tokenized_code = tokenize(code).expect("tokenization failed");

    for x in '1'..='9' {
        println!("{x}");
    }

    for token in tokenized_code {
        println!("{token}");
    }
}
