use crate::lib::types::FormulaToCsv;
use crate::lib::types::Renta;
mod lib;
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    //let input: Renta = Renta::parse();
    let matches = App::from_yaml(yaml).get_matches();
    //input.to_csv(input.to_output());

    match matches.o {}
}
