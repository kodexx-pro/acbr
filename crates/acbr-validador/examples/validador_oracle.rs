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
            "emailList" => acbr_validador::validar_email(value).is_ok(),
            "email" => acbr_validador::validar_email(value).is_ok(),
            "cns" => acbr_validador::validar_cns(value).is_ok(),
            "cheque" => acbr_validador::validar_cheque(value).is_ok(),
            "cep" => acbr_validador::validar_cep(value, "").is_ok(),
            "cepSC" => acbr_validador::validar_cep(value, "SC").is_ok(),
            "cepsc" => acbr_validador::validar_cep(value, "sc").is_ok(),
            "cpfE" | "cpfX" | "cpfZ" => {
                let mut v = acbr_validador::Validador {
                    documento: value.into(),
                    ..Default::default()
                };
                if kind == "cpfE" {
                    v.permitir_vazio = true;
                } else if kind == "cpfX" {
                    v.ignorar_caracteres = "".into();
                } else {
                    v.ignorar_caracteres = "*".into();
                }
                v.validar().is_ok()
            }
            "cpfA" | "cnpjA" | "pisA" | "cepA" | "renavamA" | "caepfA" | "cnsA" => {
                let tipo = match kind {
                    "cpfA" => acbr_validador::TipoDocumento::Cpf,
                    "cnpjA" => acbr_validador::TipoDocumento::Cnpj,
                    "pisA" => acbr_validador::TipoDocumento::Pis,
                    "cepA" => acbr_validador::TipoDocumento::Cep,
                    "renavamA" => acbr_validador::TipoDocumento::Renavam,
                    "caepfA" => acbr_validador::TipoDocumento::Caepf,
                    _ => acbr_validador::TipoDocumento::Cns,
                };
                acbr_validador::Validador {
                    tipo,
                    documento: value.into(),
                    ajustar_tamanho: true,
                    ..Default::default()
                }
                .validar()
                .is_ok()
            }
            _ if kind.starts_with("ia") => acbr_validador::Validador {
                tipo: acbr_validador::TipoDocumento::InscricaoEstadual,
                documento: value.into(),
                complemento: kind[2..].into(),
                ajustar_tamanho: true,
                ..Default::default()
            }
            .validar()
            .is_ok(),
            _ if kind.starts_with("ie") => acbr_validador::validar_ie(value, &kind[2..]).is_ok(),
            _ => panic!("tipo desconhecido"),
        };
        println!("{}", u8::from(valid));
    }
}
