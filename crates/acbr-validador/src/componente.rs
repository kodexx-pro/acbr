// SPDX-License-Identifier: LGPL-2.1-or-later
//! API de componente para seleção de documento e opções comuns.
use crate::*;
use alloc::{
    format,
    string::{String, ToString},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoDocumento {
    Cpf,
    Cnpj,
    Uf,
    InscricaoEstadual,
    Cheque,
    Pis,
    Cep,
    CartaoCredito,
    Suframa,
    Gtin,
    Renavam,
    Email,
    Cnh,
    PrefixoGtin,
    Caepf,
    Placa,
    Cns,
}

/// Configuração equivalente às opções principais do componente ACBr.
#[derive(Debug, Clone)]
pub struct Validador {
    pub tipo: TipoDocumento,
    pub documento: String,
    pub complemento: String,
    pub ignorar_caracteres: String,
    pub ajustar_tamanho: bool,
    pub permitir_vazio: bool,
}
impl Default for Validador {
    fn default() -> Self {
        Self {
            tipo: TipoDocumento::Cpf,
            documento: String::new(),
            complemento: String::new(),
            ignorar_caracteres: "./-".into(),
            ajustar_tamanho: false,
            permitir_vazio: false,
        }
    }
}
impl Validador {
    fn limpo(&self) -> String {
        if self.tipo == TipoDocumento::Email {
            return self
                .documento
                .trim_matches(|c: char| c.is_ascii() && c <= ' ')
                .into();
        }
        let s: String = self
            .documento
            .chars()
            .filter(|c| !self.ignorar_caracteres.contains(*c))
            .map(|c| c.to_ascii_uppercase())
            .collect();
        s.trim_matches(|c: char| c.is_ascii() && c <= ' ').into()
    }
    pub fn validar(&self) -> Result<(), ValidationError> {
        let mut n = self.limpo();
        if n.is_empty() {
            return if self.permitir_vazio {
                Ok(())
            } else {
                Err(ValidationError::Empty)
            };
        }
        if self.tipo == TipoDocumento::Email {
            return validar_email(&self.documento);
        }
        // The public validators normalize these punctuation marks by default;
        // if a custom ignore set leaves one present, the Pascal numeric tests reject it.
        if n.chars().any(|c| matches!(c, '.' | '/' | '-')) {
            return Err(ValidationError::InvalidCharacter);
        }
        let target = match self.tipo {
            TipoDocumento::Cpf | TipoDocumento::Pis | TipoDocumento::Renavam => Some(11),
            TipoDocumento::Cnpj | TipoDocumento::Caepf => Some(14),
            TipoDocumento::Cep => Some(8),
            TipoDocumento::Cns => Some(15),
            _ => None,
        };
        if self.ajustar_tamanho {
            if let Some(size) = target {
                n = if n.len() < size {
                    format!("{:0>width$}", n, width = size)
                } else {
                    n.chars().take(size).collect()
                };
            }
        }
        match self.tipo {
            TipoDocumento::Cpf => validar_cpf(&n),
            TipoDocumento::Cnpj => validar_cnpj(&n),
            TipoDocumento::Uf => validar_uf(&n),
            TipoDocumento::InscricaoEstadual => {
                crate::ie::validar_ie_com_ajuste(&n, &self.complemento, self.ajustar_tamanho)
            }
            TipoDocumento::Cheque => validar_cheque(&n),
            TipoDocumento::Pis => validar_pis(&n),
            TipoDocumento::Cep => validar_cep(&n, &self.complemento),
            TipoDocumento::CartaoCredito => validar_cartao_credito(&n),
            TipoDocumento::Suframa => validar_suframa(&n),
            TipoDocumento::Gtin => validar_gtin(&n),
            TipoDocumento::Renavam => validar_renavam(&n),
            TipoDocumento::Cnh => validar_cnh(&n),
            TipoDocumento::PrefixoGtin => validar_prefixo_gtin(&n),
            TipoDocumento::Caepf => validar_caepf(&n),
            TipoDocumento::Placa => validar_placa(&n),
            TipoDocumento::Cns => validar_cns(&n),
            TipoDocumento::Email => validar_email(&n),
        }
    }
    /// Executa o callback em caso de erro, passando a categoria Rust.
    /// Não reproduz o texto do evento Delphi `OnMsgErro`.
    pub fn validar_com_evento<F>(&self, mut on_erro: F) -> Result<(), ValidationError>
    where
        F: FnMut(ValidationError),
    {
        let resultado = self.validar();
        if let Err(erro) = resultado {
            on_erro(erro);
        }
        resultado
    }
    pub fn formatar(&self) -> String {
        let n = self.limpo();
        match self.tipo {
            TipoDocumento::Cpf => formatar_cpf(&n),
            TipoDocumento::Cnpj => formatar_cnpj(&n),
            TipoDocumento::Caepf => formatar_caepf(&n),
            TipoDocumento::InscricaoEstadual => formatar_ie(&n, &self.complemento),
            TipoDocumento::Cheque => formatar_cheque(&n),
            TipoDocumento::Pis => formatar_pis(&n),
            TipoDocumento::Cep => formatar_cep(&n),
            TipoDocumento::Suframa => formatar_suframa(&n),
            TipoDocumento::Placa => formatar_placa(&n),
            _ => n.to_string(),
        }
    }
}

/// Valida com as opções padrão do ACBr.
pub fn validar_documento(
    tipo: TipoDocumento,
    documento: &str,
    complemento: &str,
) -> Result<(), ValidationError> {
    Validador {
        tipo,
        documento: documento.into(),
        complemento: complemento.into(),
        ..Validador::default()
    }
    .validar()
}
/// Formata com as opções padrão do componente.
pub fn formatar_documento(tipo: TipoDocumento, documento: &str, complemento: &str) -> String {
    Validador {
        tipo,
        documento: documento.into(),
        complemento: complemento.into(),
        ..Validador::default()
    }
    .formatar()
}
/// Sobrecarga equivalente ao CEP inteiro do componente Pascal.
pub fn validar_cep_numero(cep: i32, uf: &str) -> Result<(), ValidationError> {
    validar_cep(&cep.to_string(), uf)
}
/// Sobrecarga equivalente à formatação de CEP inteiro.
pub fn formatar_cep_numero(cep: i32) -> String {
    formatar_cep(&cep.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn component_options_and_formatting() {
        let mut v = Validador::default();
        assert_eq!(v.validar(), Err(ValidationError::Empty));
        v.permitir_vazio = true;
        assert!(v.validar().is_ok());
        v.permitir_vazio = false;
        v.documento = "529.982.247-25".into();
        assert!(v.validar().is_ok());
        v.ignorar_caracteres = String::new();
        assert!(v.validar().is_err());
        v.ignorar_caracteres = "./-".into();
        v.documento = "52998224725".into();
        assert_eq!(v.formatar(), "529.982.247-25");
        v.documento = "52998224725999".into();
        v.ajustar_tamanho = true;
        assert!(v.validar().is_ok());
        v.ajustar_tamanho = false;
        let mut called = false;
        assert!(v.validar_com_evento(|_| called = true).is_err());
        assert!(called);
        v.tipo = TipoDocumento::Cnpj;
        v.documento = "11222333000181".into();
        assert!(v.validar().is_ok());
        v.tipo = TipoDocumento::InscricaoEstadual;
        v.documento = "isento".into();
        v.complemento.clear();
        assert!(v.validar().is_ok());
    }
}
