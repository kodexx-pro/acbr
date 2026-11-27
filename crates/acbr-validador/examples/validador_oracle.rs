use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("leitura");
        let (kind, value) = line.split_once('|').expect("tipo|valor");
        let valid = match kind {
            "cpf" => acbr_validador::validar_cpf(value).is_ok(),
            "cnpj" => acbr_validador::validar_cnpj(value).is_ok(),
            "prefixo" => acbr_validador::validar_prefixo_gtin(value).is_ok(),
            "gtin" => acbr_validador::validar_gtin(value).is_ok(),
            "uf" => acbr_validador::validar_uf(value).is_ok(),
            "placa" => acbr_validador::validar_placa(value).is_ok(),
            "pis" => acbr_validador::validar_pis(value).is_ok(),
            "renavam" => acbr_validador::validar_renavam(value).is_ok(),
            "suframa" => acbr_validador::validar_suframa(value).is_ok(),
            "caepf" => acbr_validador::validar_caepf(value).is_ok(),
            "cartao" => acbr_validador::validar_cartao_credito(value).is_ok(),
            "cnh" => acbr_validador::validar_cnh(value).is_ok(),
            "cns" => acbr_validador::validar_cns(value).is_ok(),
            "cheque" => acbr_validador::validar_cheque(value).is_ok(),
            "cep" => acbr_validador::validar_cep(value, "").is_ok(),
            "cepSC" => acbr_validador::validar_cep(value, "SC").is_ok(),
            "cepsc" => acbr_validador::validar_cep(value, "sc").is_ok(),
            _ => panic!("tipo desconhecido"),
        };
        println!("{}", u8::from(valid));
    }
}
