use crate::lib::types::FormulaToCsv;
use crate::lib::types::Renta;
mod lib;
#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();    

    let s = matches.value_of("suma").unwrap().parse::<f64>().unwrap();
    let i = matches.value_of("interes").unwrap().parse::<f64>().unwrap();
    let a = matches.value_of("years").unwrap().parse::<f64>().unwrap();
    let m = matches.value_of("mensualidades").unwrap_or("12.0").parse::<f64>().unwrap();

    let input = Renta { s, i, a, m };
    let rows = input.to_csv(input.to_output());

    println!("PERIODO,PAGO,INTERES,ABONO,CAPITAL PAGADO,SALDO INSOLUTO");
    for r in rows {
        println!("{}", r);
    }

}
