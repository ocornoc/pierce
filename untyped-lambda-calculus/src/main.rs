mod eval;
mod parser;
mod naming;

use parser::parse;
use naming::{remove_names, restore_names};

fn main() {
    let inputs = [
        r"((\x. (x (\y. y))) (\z. z))",
        r"((\x. x) ((\x. x) (\z. ((\x. x) z))))",
        r"(\y. (\z. ((\x. (\y. (x y))) (y z))))",
    ];

    for input in &inputs {
        println!("\nInput: {}", input);
        if let Some(named_term) = parse(input) {
            println!("Parsed term: {}", named_term);
            let mut term = remove_names(named_term);
            println!("Nameless term: {}", term);
            while term.reduce() {}
            println!("After reduction: {}", term);
            println!("After renaming: {}", restore_names(term));
        }
    }
}
