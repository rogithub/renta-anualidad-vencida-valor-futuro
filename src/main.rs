use crate::lib::types::Renta;
use clap::Clap;
mod lib;

fn main() {
    let input: Renta = Renta::parse();
    input.to_csv(input.to_output());
}
