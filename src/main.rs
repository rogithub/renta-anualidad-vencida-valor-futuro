use crate::lib::types::first;
use crate::lib::types::json_to_input;
use crate::lib::types::to_csv;
use crate::lib::types::to_output;
use crate::lib::types::Renta;
use std::io::{self, BufRead};
mod lib;

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<Renta> = stdin.lock().lines().map(json_to_input).collect();
    let item = first(&inputs).unwrap();

    to_csv(to_output(item));
}
