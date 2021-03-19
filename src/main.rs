use crate::types::types::first;
use crate::types::types::json_to_input;
use crate::types::types::to_output;
use crate::types::types::Input;
use crate::types::types::Output;
use std::io::{self, BufRead};
mod types;

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<Input> = stdin.lock().lines().map(json_to_input).collect();
    let item = first(&inputs).unwrap();

    println!("PERIODO,PAGO,INTERES,ABONO,CAPITAL PAGADO,SALDO INSOLUTO");
    let outputs: Vec<Output> = to_output(item);
    for o in outputs {
        println!(
            "{:?},{:.2},{:.2},{:.2},{:.2},{:.2}",
            o.periodo, o.pago, o.interes, o.abono, o.capital_pagado, o.saldo_insoluto
        );
    }
}
