use crate::types::types::first;
use crate::types::types::json_to_input;
use crate::types::types::renta;
use crate::types::types::Input;
use std::io::{self, BufRead};
mod types;

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<Input> = stdin.lock().lines().map(json_to_input).collect();
    let item = first(&inputs).unwrap();

    println!("{}", renta(&item))
}
