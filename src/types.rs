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
                let interes_periodo = (i / m) * s;
                let pago = renta + interes_periodo;
                let saldo_insoluto = s.clone();
                let mut interes_acumulado = interes_periodo;
                let abono_periodo = pago - interes_acumulado;
                let _capital_pagado = pago - interes_acumulado;
                let mut abono_acumulado = abono_periodo;
                let capital_pagado = saldo_insoluto - abono_acumulado;
                let rounds = ((m * n) + 1.0) as i64;
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
                    result.push(Output {
                        periodo: periodo,
                        pago: pago,
                        interes: interes_acumulado,
                        abono: abono_periodo,
                        capital_pagado: capital_pagado,
                        saldo_insoluto: saldo_insoluto.clone(),
                    });

                    interes_acumulado = interes_acumulado + interes_periodo;
                    abono_acumulado = abono_acumulado + abono_periodo;
                }
            }
        }
        result
    }

    pub fn output_to_json(output: &Output) -> String {
        serde_json::to_string(output).unwrap()
    }
}
