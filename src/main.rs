use crate::types::types::first;
use crate::types::types::json_to_input;
use crate::types::types::to_csv;
use crate::types::types::to_output;
use crate::types::types::Renta;
use std::io::{self, BufRead};
mod types;

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<Renta> = stdin.lock().lines().map(json_to_input).collect();
    let item = first(&inputs).unwrap();

    to_csv(to_output(item));
}
