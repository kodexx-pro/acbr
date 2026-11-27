// SPDX-License-Identifier: LGPL-2.1-or-later
//! Resultado detalhado inspirado nas propriedades do componente Pascal.
use crate::{TipoDocumento as T, Validador, ValidationError};
use alloc::{
    format,
    string::{String, ToString},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultadoDetalhado {
    pub valido: bool,
    pub mensagem: String,
    pub documento_validado: String,
    pub digito_calculado: String,
    pub erro: Option<ValidationError>,
}
fn pad(s: &str, n: usize) -> String {
    let mut o = String::new();
    for _ in s.chars().count()..n {
        o.push('0')
    }
    o.extend(s.chars().take(n));
    o
}
fn nums(b: &[u8]) -> bool {
    b.iter().all(u8::is_ascii_digit)
}
fn dv(b: &[u8], end: u8) -> u8 {
    crate::acbr_math_bridge(b, 2, end)
}
fn nome(t: T) -> &'static str {
    match t {
        T::Cpf => "CPF",
        T::Cnpj => "CNPJ",
        T::Uf => "UF",
        T::InscricaoEstadual => "Inscrição Estadual",
        T::Cheque => "Número de Cheque",
        T::Pis => "PIS",
        T::Cep => "CEP",
        T::CartaoCredito => "Número de Cartão",
        T::Suframa => "SUFRAMA",
        T::Gtin => "GTIN",
        T::Renavam => "Renavam",
        T::Email => "E-Mail",
        T::Cnh => "Carteira Nacional de Habilitação",
        T::PrefixoGtin => "Prefixo do Código GTIN",
        T::Caepf => "CAEPF",
        T::Placa => "Placa",
        T::Cns => "Número Cartão Nacional do SUS",
    }
}
impl Validador {
    /// Retorna mensagem, documento normalizado e dígito calculado, quando houver.
    pub fn validar_detalhado(&self, mostrar_digito: bool) -> ResultadoDetalhado {
        let mut n = self.limpo();
        let mut msg = String::new();
        let mut dig = String::new();
        if n.is_empty() {
            if !self.permitir_vazio {
                msg = format!("{} não pode ser vazio.", nome(self.tipo));
            }
        } else {
            let size = match self.tipo {
                T::Cpf | T::Pis | T::Renavam => Some(11),
                T::Cnpj | T::Caepf => Some(14),
                T::Cep => Some(8),
                T::Cns => Some(15),
                _ => None,
            };
            if self.ajustar_tamanho {
                if let Some(size) = size {
                    n = pad(&n, size);
                }
            }
            let b = n.as_bytes();
            match self.tipo {
                T::Cpf => {
                    if b.len() != 11 || !nums(b) {
                        msg = "CPF deve ter 11 dígitos. (Apenas números)".into()
                    } else if b.iter().all(|x| *x == b[0]) {
                        msg = "CPF inválido !".into()
                    } else {
                        let d1 = dv(&b[..9], 11);
                        let mut second = b[..9].to_vec();
                        second.push(b'0' + d1);
                        let d2 = dv(&second, 11);
                        dig = format!("{}{}", d1, d2);
                        if &n[9..] != dig {
                            msg = "CPF inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(".. Dígito calculado: {}", dig));
                            }
                        }
                    }
                }
                T::Cnpj => {
                    if b.len() != 14 || !b.iter().all(u8::is_ascii_alphanumeric) {
                        msg = "CNPJ deve ter 14 dígitos. (Apenas letras e números)".into()
                    } else if b.iter().all(|&x| x == b'0') {
                        msg = "CNPJ inválido.".into()
                    } else {
                        let d1 = dv(&b[..12], 9);
                        let mut tmp = b[..12].to_vec();
                        tmp.push(b'0' + d1);
                        let d2 = dv(&tmp, 9);
                        dig = format!("{}{}", d1, d2);
                        if &n[12..] != dig {
                            msg = "CNPJ inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(".. Digito calculado: {}", dig));
                            }
                        }
                    }
                }
                T::Caepf => {
                    if b.len() != 14 || !nums(b) {
                        msg = "CAEPF deve ter 14 dígitos. (Apenas números)".into()
                    } else if b.iter().all(|&x| x == b'0') {
                        msg = "CAEPF inválido.".into()
                    } else {
                        let d1 = dv(&b[..12], 9);
                        let mut tmp = b[..12].to_vec();
                        tmp.push(b'0' + d1);
                        let d2 = dv(&tmp, 9);
                        let result = (u16::from(d1) * 10 + u16::from(d2) + 12) % 100;
                        if &n[12..] != format!("{:02}", result) {
                            msg = "CAEPF inválido.".into();
                            if mostrar_digito {
                                msg.push_str(".. Digito calculado: ");
                            }
                        }
                    }
                }
                T::Uf => {
                    if !matches!(
                        n.as_str(),
                        "AC" | "AL"
                            | "AP"
                            | "AM"
                            | "BA"
                            | "CE"
                            | "DF"
                            | "ES"
                            | "GO"
                            | "MA"
                            | "MT"
                            | "MS"
                            | "MG"
                            | "PA"
                            | "PB"
                            | "PR"
                            | "PE"
                            | "PI"
                            | "RJ"
                            | "RN"
                            | "RS"
                            | "RO"
                            | "RR"
                            | "SC"
                            | "SP"
                            | "SE"
                            | "TO"
                            | "EX"
                    ) {
                        msg = format!("UF inválido: {}", n)
                    }
                }
                T::Placa => {
                    if b.len() != 7 || !b.iter().all(u8::is_ascii_alphanumeric) {
                        msg = "Placa deve ter 7 dígitos. (Somente letras e números)".into()
                    }
                }
                T::Cheque => {
                    if !nums(b) {
                        msg = "Digite apenas os números do Cheque".into()
                    } else {
                        dig = dv(&b[..b.len() - 1], 9).to_string();
                        if &n[n.len() - 1..] != dig {
                            msg = "Número de Cheque inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(".. Dígito correto: {}", dig));
                            }
                        }
                    }
                }
                T::Pis => {
                    if b.len() != 11 || !nums(b) {
                        msg = "PIS deve ter 11 dígitos. (Apenas números)".into()
                    } else {
                        dig = dv(&b[..10], 9).to_string();
                        if &n[10..] != dig {
                            msg = "PIS inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(".. Dígito calculado: {}", dig));
                            }
                        }
                    }
                }
                T::Renavam => {
                    if b.len() != 11 || !nums(b) {
                        msg = "RENAVAM deve ter 11 dígitos. (Apenas números)".into()
                    } else {
                        let sum: u32 = b[..10]
                            .iter()
                            .zip(b"3298765432")
                            .map(|(&x, &w)| u32::from(x - b'0') * u32::from(w - b'0'))
                            .sum();
                        let d = (sum * 10) % 11;
                        dig = (if d == 10 { 0 } else { d }).to_string();
                        if &n[10..] != dig {
                            msg = "RENAVAM inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(".. Dígito calculado: {}", dig));
                            }
                        }
                    }
                }
                T::Suframa => {
                    if b.len() < 9 || !nums(b) {
                        msg = "Código SUFRAMA deve ter no mínimo 9 dígitos. (Apenas números)".into()
                    } else {
                        dig = dv(&b[..8], 9).to_string();
                        if &n[8..9] != dig {
                            msg = "Número SUFRAMA inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(" Dígito calculado: {}", dig));
                            }
                        }
                    }
                }
                T::CartaoCredito => {
                    if !nums(b) {
                        msg = "Cartão deve ter apenas Números".into()
                    } else {
                        let sum: u32 = b[..b.len() - 1]
                            .iter()
                            .enumerate()
                            .map(|(i, &x)| {
                                let v = u32::from(x - b'0') * if i % 2 == 0 { 2 } else { 1 };
                                v / 10 + v % 10
                            })
                            .sum();
                        dig = ((10 - sum % 10) % 10).to_string();
                        if &n[n.len() - 1..] != dig {
                            msg = "Numero do Cartão Inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(".. Dígito calculado: {}", dig));
                            }
                        }
                    }
                }
                T::Gtin | T::PrefixoGtin => {
                    if !nums(b) {
                        msg = "Código GTIN inválido, o código GTIN deve conter somente números."
                            .into()
                    } else if !matches!(b.len(), 8 | 12 | 13 | 14) {
                        msg="Código GTIN inválido, o código GTIN deve ter 8, 12, 13 ou 14 caracteres.".into()
                    } else {
                        let sum: u32 = b[..b.len() - 1]
                            .iter()
                            .rev()
                            .enumerate()
                            .map(|(i, &x)| u32::from(x - b'0') * if i % 2 == 0 { 3 } else { 1 })
                            .sum();
                        dig = ((10 - sum % 10) % 10).to_string();
                        if &n[n.len() - 1..] != dig {
                            msg = "Código GTIN inválido.".into();
                            if mostrar_digito {
                                msg.push_str(&format!(" Dígito calculado: {}", dig));
                            }
                        } else if self.tipo == T::PrefixoGtin
                            && crate::validar_prefixo_gtin(&n).is_err()
                        {
                            let padded = pad(&n, 14);
                            let idx = if &padded[..6] == "000000" { 6 } else { 1 };
                            let prefix: u16 = padded[idx..idx + 3].parse().unwrap_or(0);
                            msg = format!(
                                "Prefixo \"{}\" do GTIN \"{}\" informado inválido",
                                prefix, self.documento
                            );
                        }
                    }
                }
                T::Cnh => {
                    if b.len() != 11 || !nums(b) {
                        msg = "C.N.H. deve ter 11 dígitos. (Apenas números)".into()
                    } else if b.iter().all(|x| *x == b[0]) {
                        msg = "C.N.H. não deve conter números repetidos !".into()
                    } else if crate::validar_cnh(&n).is_err() {
                        msg = "C.N.H. Inválida !!".into()
                    }
                }
                T::Cns => {
                    if b.len() != 15 {
                        msg = "O número do Cartão Nacional do SUS deve ter 15 dígitos".into()
                    } else if crate::validar_cns(&n).is_err() {
                        msg = "O número do Cartão Nacional do SUS informado é inválido!".into()
                    }
                }
                T::Email => {
                    if self.documento.len() < 8 {
                        msg = "e-mail não pode conter menos do que 8 caracteres!".into()
                    } else if crate::validar_email_unico(&self.documento).is_err() {
                        msg = "e-mail inválido!".into()
                    }
                }
                T::Cep => {
                    if b.len() != 8 || !nums(b) {
                        msg = "CEP deve ter 8 dígitos. (Apenas números)".into()
                    } else {
                        let uf = self
                            .complemento
                            .trim_matches(|c: char| c.is_ascii() && c <= ' ');
                        if !uf.is_empty()
                            && !matches!(
                                uf,
                                "AC" | "AL"
                                    | "AP"
                                    | "AM"
                                    | "BA"
                                    | "CE"
                                    | "DF"
                                    | "ES"
                                    | "GO"
                                    | "MA"
                                    | "MT"
                                    | "MS"
                                    | "MG"
                                    | "PA"
                                    | "PB"
                                    | "PR"
                                    | "PE"
                                    | "PI"
                                    | "RJ"
                                    | "RN"
                                    | "RS"
                                    | "RO"
                                    | "RR"
                                    | "SC"
                                    | "SP"
                                    | "SE"
                                    | "TO"
                                    | "EX"
                            )
                        {
                            msg = format!("UF inválido: {}", uf)
                        } else if crate::validar_cep(&n, uf).is_err() {
                            msg = if uf.is_empty() {
                                "CEP inválido !".into()
                            } else {
                                format!("CEP inválido para {} !", uf)
                            };
                        }
                    }
                }
                T::InscricaoEstadual => {
                    let (m, doc, d) = crate::ie::detalhes_ie(
                        &n,
                        &self.complemento,
                        self.ajustar_tamanho,
                        mostrar_digito,
                    );
                    msg = m;
                    n = doc;
                    dig = d;
                }
            }
        }
        let valido = msg.is_empty();
        let erro = if valido {
            None
        } else {
            Some(
                self.validar()
                    .err()
                    .unwrap_or(ValidationError::InvalidCharacter),
            )
        };
        ResultadoDetalhado {
            valido,
            mensagem: msg,
            documento_validado: n,
            digito_calculado: dig,
            erro,
        }
    }
    /// Entrega mensagem textual ao callback e mantém o erro explícito em `Result`.
    pub fn validar_detalhado_com_evento<F>(
        &self,
        mostrar_digito: bool,
        mut callback: F,
    ) -> Result<ResultadoDetalhado, ResultadoDetalhado>
    where
        F: FnMut(&str),
    {
        let out = self.validar_detalhado(mostrar_digito);
        if out.valido {
            Ok(out)
        } else {
            callback(&out.mensagem);
            Err(out)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resultado_e_evento() {
        let v = Validador {
            tipo: T::Cpf,
            documento: "529.982.247-24".into(),
            ..Default::default()
        };
        let mut evento = String::new();
        let resultado = v
            .validar_detalhado_com_evento(true, |m| evento = m.into())
            .unwrap_err();
        assert_eq!(resultado.documento_validado, "52998224724");
        assert_eq!(resultado.digito_calculado, "25");
        assert_eq!(resultado.mensagem, "CPF inválido... Dígito calculado: 25");
        assert_eq!(evento, resultado.mensagem);
        assert_eq!(v.validar_detalhado(false).mensagem, "CPF inválido.");
    }
    #[test]
    fn email_do_componente_nao_aceita_lista() {
        let v = Validador {
            tipo: T::Email,
            documento: "a@b.com;c@d.com".into(),
            ..Default::default()
        };
        assert!(v.validar().is_err());
        assert!(!v.validar_detalhado(false).valido);
    }
    #[test]
    fn inscricao_estadual_exibe_digito() {
        let v = Validador {
            tipo: T::InscricaoEstadual,
            documento: "123456788".into(),
            complemento: "SC".into(),
            ..Default::default()
        };
        let r = v.validar_detalhado(true);
        assert_eq!(r.digito_calculado, "9");
        assert_eq!(
            r.mensagem,
            "Insc.Estadual inválida para SC Dígito verificador inválido... Calculado: 9"
        );
    }
}
