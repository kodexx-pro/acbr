// SPDX-License-Identifier: LGPL-2.1-or-later
//! Recorte de cálculo módulo 11 de `TACBrCalcDigito` (SVN r49422).
#![no_std]

/// Calcula módulo 11 sobre caracteres ASCII 0-9 e A-Z, convertendo cada byte
/// para seu valor ASCII menos 48, como o ACBr. O multiplicador avança de
/// `peso_inicial` a `peso_final` da direita para a esquerda e volta ao início.
/// Retorna `None` para caracteres fora desse conjunto ou pesos nulos/decrescentes.
/// Esta API cobre apenas essa configuração do algoritmo Pascal.
pub fn modulo11_ascii(input: &[u8], peso_inicial: u8, peso_final: u8) -> Option<u8> {
    if peso_inicial == 0 || peso_inicial > peso_final {
        return None;
    }
    let mut peso = peso_inicial;
    let mut resto = 0u32;
    for &b in input.iter().rev() {
        if !b.is_ascii_digit() && !b.is_ascii_uppercase() {
            return None;
        }
        resto = (resto + u32::from(b - b'0') * u32::from(peso)) % 11;
        peso = if peso == peso_final {
            peso_inicial
        } else {
            peso + 1
        };
    }
    Some(if resto < 2 { 0 } else { (11 - resto) as u8 })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checks() {
        assert_eq!(modulo11_ascii(b"529982247", 2, 11), Some(2));
        assert_eq!(modulo11_ascii(b"12ABC34501DE", 2, 9), Some(3));
        assert_eq!(modulo11_ascii(b"12-3", 2, 9), None);
        assert_eq!(modulo11_ascii(b"123", 9, 2), None);
    }
}
