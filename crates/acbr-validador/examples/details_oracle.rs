use acbr_validador::{TipoDocumento as T, Validador};
use std::io::{self, BufRead};
fn hex(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{:02X}", b)).collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.splitn(3, '|');
        let k = p.next().unwrap();
        let f = p.next().unwrap();
        let s = p.next().unwrap();
        let t = match k {
            "cpf" => T::Cpf,
            "cnpj" => T::Cnpj,
            "uf" => T::Uf,
            "cheque" => T::Cheque,
            "pis" => T::Pis,
            "cartao" => T::CartaoCredito,
            "suframa" => T::Suframa,
            "gtin" => T::Gtin,
            "renavam" => T::Renavam,
            "email" => T::Email,
            "cnh" => T::Cnh,
            "prefixo" => T::PrefixoGtin,
            "caepf" => T::Caepf,
            "placa" => T::Placa,
            "cns" => T::Cns,
            _ if k.starts_with("cep") => T::Cep,
            _ if k.starts_with("ie") => T::InscricaoEstadual,
            _ => panic!(),
        };
        let complemento = if k.starts_with("cep") {
            &k[3..]
        } else if k.starts_with("ie") {
            &k[2..]
        } else {
            ""
        };
        let v = Validador {
            tipo: t,
            documento: s.into(),
            complemento: complemento.into(),
            ajustar_tamanho: f.contains('A'),
            permitir_vazio: f.contains('E'),
            ..Default::default()
        };
        let mut event = String::new();
        let r = v
            .validar_detalhado_com_evento(f.contains('S'), |m| event = m.into())
            .unwrap_or_else(|e| e);
        println!(
            "{}|{}|{}|{}|{}|{}",
            u8::from(r.valido),
            hex(&r.mensagem),
            hex(&r.documento_validado),
            hex(&r.digito_calculado),
            u8::from(!r.valido),
            hex(&event)
        );
    }
}
