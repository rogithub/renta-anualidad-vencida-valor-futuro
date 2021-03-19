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
}
