use acbr_validador::*;
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split('|');
        let k = p.next().unwrap();
        let v = p.next().unwrap_or("");
        let a = p.next().unwrap_or("");
        let out = match k {
            "cpf" => formatar_cpf(v),
            "cnpj" => formatar_cnpj(v),
            "caepf" => formatar_caepf(v),
            "cpfcnpj" => formatar_cnpj_ou_cpf(v),
            "placa" => formatar_placa(v),
            "cheque" => formatar_cheque(v),
            "pis" => formatar_pis(v),
            "cep" => formatar_cep(v),
            "suframa" => formatar_suframa(v),
            "mask" => formatar_mascara_numerica(v, a),
            "dyn" => formatar_mascara_dinamica(v, a),
            "remove" => remover_mascara(v, a),
            "ie" => formatar_ie(v, a),
            "maskie" => mascara_ie(v, a),
            "fone" => formatar_fone(v, a),
            _ => panic!("tipo"),
        };
        println!("{out}");
    }
}
