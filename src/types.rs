pub mod types {
    use serde::{Deserialize, Serialize};
    use std::io::{self};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Renta {
        s: f64,
        i: f64,
        m: f64,
        n: f64,
    }

    impl Renta {
        fn calculate(&self) -> f64 {
            let Renta { s, i, m, n } = &self;
            let base = 1.0 + (i / m);
            let exponente = m * n;
            let arriba = f64::powf(base, exponente) - 1.0;
            let r = s / (arriba / (i / m));
            r
        }
    }

    pub fn json_to_input(line: io::Result<String>) -> Renta {
        let json = &line.unwrap();
        let deserialized: Renta = serde_json::from_str(json).unwrap();
        deserialized
    }

    pub fn first<T>(v: &Vec<T>) -> Option<&T> {
        v.first()
    }
    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Output {
        pub periodo: i64,
        pub pago: f64,
        pub interes: f64,
        pub abono: f64,
        pub capital_pagado: f64,
        pub saldo_insoluto: f64,
    }

    pub fn to_output(item: &Renta) -> Vec<Output> {
        let renta = item.calculate();
        let mut result = Vec::new();
        let Renta { s, i, m, n } = item;
        let rounds = ((m * n) + 1.0) as i64;
        let interes_periodo = i / m;
        let pago = renta + (interes_periodo * s);
        let mut saldo_insoluto = s.clone();
        let mut capital_pagado = 0.0;

        for periodo in 0..rounds {
            let o = match periodo {
                0 => Output {
                    periodo: periodo,
                    pago: 0 as f64,
                    interes: 0 as f64,
                    abono: 0 as f64,
                    capital_pagado: 0 as f64,
                    saldo_insoluto: saldo_insoluto.clone(),
                },
                1 => {
                    saldo_insoluto = saldo_insoluto - renta;
                    capital_pagado = renta;
                    Output {
                        periodo: periodo,
                        pago: pago as f64,
                        interes: interes_periodo * s,
                        abono: renta as f64,
                        capital_pagado: capital_pagado,
                        saldo_insoluto: saldo_insoluto,
                    }
                }
                _ => {
                    let interes = interes_periodo * saldo_insoluto;
                    let abono = pago - interes;
                    capital_pagado = capital_pagado + abono;
                    saldo_insoluto = saldo_insoluto - abono;
                    Output {
                        periodo: periodo,
                        pago: pago,
                        interes: interes,
                        abono: abono,
                        capital_pagado: capital_pagado,
                        saldo_insoluto: saldo_insoluto,
                    }
                }
            };
            result.push(o);
        }
        result
    }

    pub fn to_csv(outputs: Vec<Output>) {
        println!("PERIODO,PAGO,INTERES,ABONO,CAPITAL PAGADO,SALDO INSOLUTO");
        for o in outputs {
            println!(
                "{:?},{:.2},{:.2},{:.2},{:.2},{:.2}",
                o.periodo, o.pago, o.interes, o.abono, o.capital_pagado, o.saldo_insoluto
            );
        }
    }
}
