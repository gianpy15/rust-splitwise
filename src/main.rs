use splitwise::{cli, context};
use std::process::exit;
fn main() {
    app();
}

fn app() {
    let mut context: context::Context = Default::default();
    if !cli::base::welcome(&mut context).is_ok() {
        exit(-1);
    }
    println!("Goodbye!")
}
