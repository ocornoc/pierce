mod eval;
mod parser;
mod ty;
mod ctx;

use ctx::{desugar, restore};
use parser::parse;

fn main() {
    let inputs =
        [r"(let i = (\z:(Unit -> Unit). z) in ((\x:((Unit -> Unit) -> (Unit -> Unit)). (x (\y:Unit. y))) i))"];

    for input in &inputs {
        run(input).unwrap();
    }
}

fn run(input: &str) -> Option<()> {
    println!("\nInput: {}", input);
    let named_term = parse(input)?;
    println!("Parsed term: {}", named_term);
    let (mut term, ty) = desugar(named_term)?;
    println!("Type of term: {}", ty);
    println!("Nameless term: {}", term);
    term.evaluate();
    println!("After evaluation: {}", term);
    println!("After renaming: {}", restore(term)?);
    Some(())
}
