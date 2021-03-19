use crate::types::types::first;
use crate::types::types::json_to_input;
use crate::types::types::output_to_json;
use crate::types::types::to_output;
use crate::types::types::Input;
use std::io::{self, BufRead};
mod types;

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<Input> = stdin.lock().lines().map(json_to_input).collect();
    let item = first(&inputs).unwrap();

    let outputs: Vec<String> = to_output(item).iter().map(output_to_json).collect();
    for o in outputs {
        println!("{:?}", o);
    }
}
