pub mod types {

    pub trait FormulaToCsv<T> {
        fn calculate(&self) -> f64;
        fn to_output(&self) -> Vec<T>;
        fn to_csv(&self, outputs: Vec<T>);
    }

    pub struct Renta {
        s: f64,
        i: f64,
        a: f64,
        m: f64,
    }

    pub struct Output {
        pub periodo: i64,
        pub pago: f64,
        pub interes: f64,
        pub abono: f64,
        pub capital_pagado: f64,
        pub saldo_insoluto: f64,
    }

    impl FormulaToCsv<Output> for Renta {
        fn calculate(&self) -> f64 {
            let Renta { s, i, m, a } = &self;
            let base = 1.0 + (i / m);
            let exponente = m * a;
            let arriba = f64::powf(base, exponente) - 1.0;
            s / (arriba / (i / m))
        }

        fn to_output(&self) -> Vec<Output> {
            let Renta { s, i, m, a } = &self;
            let renta = *&self.calculate();
            let mut result = Vec::new();
            let rounds = ((m * a) + 1.0) as i64;
            let interes_periodo = i / m;
            let pago = renta + (interes_periodo * s);
            let mut saldo_insoluto = *s;
            let mut capital_pagado = 0.0;
            for periodo in 0..rounds {
                let o = match periodo {
                    0 => Output {
                        periodo,
                        pago: 0 as f64,
                        interes: 0 as f64,
                        abono: 0 as f64,
                        capital_pagado: 0 as f64,
                        saldo_insoluto,
                    },
                    1 => {
                        saldo_insoluto -= renta;
                        capital_pagado = renta;
                        Output {
                            periodo,
                            pago,
                            interes: interes_periodo * s,
                            abono: renta as f64,
                            capital_pagado,
                            saldo_insoluto,
                        }
                    }
                    _ => {
                        let interes = interes_periodo * saldo_insoluto;
                        let abono = pago - interes;
                        capital_pagado += abono;
                        saldo_insoluto -= abono;
                        Output {
                            periodo,
                            pago,
                            interes,
                            abono,
                            capital_pagado,
                            saldo_insoluto,
                        }
                    }
                };
                result.push(o);
            }
            result
        }

        fn to_csv(&self, outputs: Vec<Output>) {
            println!("PERIODO,PAGO,INTERES,ABONO,CAPITAL PAGADO,SALDO INSOLUTO");
            for o in outputs {
                println!(
                    "{:?},{:.2},{:.2},{:.2},{:.2},{:.2}",
                    o.periodo, o.pago, o.interes, o.abono, o.capital_pagado, o.saldo_insoluto
                );
            }
        }
    }
}
