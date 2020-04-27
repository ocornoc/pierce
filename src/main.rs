mod eval;
mod naming;
mod parser;
mod ty;

use naming::{remove_names, restore_names};
use parser::parse;
use ty::type_of;

fn main() {
    let inputs =
        [r"((\x:((Unit -> Unit) -> (Unit -> Unit)). (x (\y:Unit. y))) (\z:(Unit -> Unit). z))"];

    for input in &inputs {
        run(input);
    }
}

fn run(input: &str) -> Option<()> {
    println!("\nInput: {}", input);
    let named_term = parse(input)?;
    println!("Parsed term: {}", named_term);
    let ty = type_of(&named_term)?;
    println!("Type of term: {}", ty);
    let mut term = remove_names(named_term)?;
    println!("Nameless term: {}", term);
    term.evaluate();
    println!("After evaluation: {}", term);
    println!("After renaming: {}", restore_names(term)?);
    Some(())
}
