use acbr_validador::{CalculadorDigito, FormulaDigito};
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let s = line.unwrap();
        let p: Vec<_> = s.split('|').collect();
        let mut c = CalculadorDigito {
            documento: p[4].into(),
            multiplicador_inicial: p[1].parse().unwrap(),
            multiplicador_final: p[2].parse().unwrap(),
            multiplicador_atual: p[3].parse().unwrap(),
            formula: match p[0] {
                "0" => FormulaDigito::Modulo11,
                "1" => FormulaDigito::Modulo10Pis,
                _ => FormulaDigito::Modulo10,
            },
            ..Default::default()
        };
        c.calcular().unwrap();
        println!("{}|{}|{}", c.soma_digitos, c.modulo_final, c.digito_final);
    }
}
