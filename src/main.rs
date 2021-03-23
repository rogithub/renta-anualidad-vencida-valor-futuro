use crate::lib::types::to_csv;
use crate::lib::types::to_output;
use crate::lib::types::Renta;
use clap::Clap;
mod lib;

fn main() {
    let input: Renta = Renta::parse();
    to_csv(to_output(&input));
}
