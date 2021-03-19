pub mod types {
    use serde::{Deserialize, Serialize};
    use std::io::{self};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Input {
        #[serde(rename_all = "camelCase")]
        Renta { s: f64, i: f64, m: f64, n: f64 },
    }

    pub fn json_to_input(line: io::Result<String>) -> Input {
        let json = &line.unwrap();
        let deserialized: Input = serde_json::from_str(json).unwrap();
        deserialized
    }

    pub fn first<T>(v: &Vec<T>) -> Option<&T> {
        v.first()
    }

    pub fn renta(input: &Input) -> f64 {
        match input {
            Input::Renta { s, i, m, n } => {
                let base = 1.0 + (i / m);
                let exponente = m * n;

                let arriba = f64::powf(base, exponente) - 1.0;

                let r = s / (arriba / (i / m));

                r
            }
        }
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

    pub fn to_output(item: &Input) -> Vec<Output> {
        let renta = renta(item);
        let mut result = Vec::new();
        match item {
            Input::Renta { s, i, m, n } => {
                let rounds = ((m * n) + 1.0) as i64;
                let interes_periodo = i / m;
                let pago = renta + (interes_periodo * s);
                let mut saldo_insoluto = s.clone();
                let mut capital_pagado = 0.0;

                for periodo in 0..rounds {
                    if periodo == 0 {
                        result.push(Output {
                            periodo: periodo,
                            pago: 0 as f64,
                            interes: 0 as f64,
                            abono: 0 as f64,
                            capital_pagado: 0 as f64,
                            saldo_insoluto: saldo_insoluto.clone(),
                        });
                        continue;
                    }

                    if periodo == 1 {
                        saldo_insoluto = saldo_insoluto - renta;
                        capital_pagado = renta;
                        result.push(Output {
                            periodo: periodo,
                            pago: pago as f64,
                            interes: interes_periodo * s,
                            abono: renta as f64,
                            capital_pagado: capital_pagado,
                            saldo_insoluto: saldo_insoluto,
                        });
                        continue;
                    }
                    let interes = interes_periodo * saldo_insoluto;
                    let abono = pago - interes;
                    capital_pagado = capital_pagado + abono;
                    saldo_insoluto = saldo_insoluto - abono;
                    result.push(Output {
                        periodo: periodo,
                        pago: pago,
                        interes: interes,
                        abono: abono,
                        capital_pagado: capital_pagado,
                        saldo_insoluto: saldo_insoluto,
                    });
                }
            }
        }
        result
    }
    /*
        pub fn output_to_json(output: &Output) -> String {
            serde_json::to_string(output).unwrap()
        }
    */
}
