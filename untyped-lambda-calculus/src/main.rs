mod eval;
mod parser;

use parser::parse;

fn main() {
    let inputs = [
        r"(\x. not a term)",
        r"((\x. (x y)) (\z. z))",
        r"((\x. (\y. (x y))) (y z))",
        r"((\x. (y x)) x)",
        r"(\x. (y. z))",
        r"(\x. !x)",
        r"((\x. (\y. (x y))) (y z)))",
        r"((\x. (\y. (x y))) (y z)",
    ];

    for input in &inputs {
        println!("\nInput: {}", input);
        if let Some(mut term) = parse(input) {
            println!("Parsed term: {}", term);
            term.reduce();
            println!("After reduction: {}", term);
        }
    }
}
