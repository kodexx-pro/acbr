// SPDX-License-Identifier: LGPL-2.1-or-later
//! Recorte de cálculo módulo 11 de `TACBrCalcDigito` (SVN r49422).
#![no_std]
extern crate alloc;

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

/// Erros de parsing, tamanho ou overflow nas conversões numéricas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    InvalidDigit,
    InvalidLength,
    Overflow,
}

/// Round equivalente ao `SimpleRoundToEX` do ACBr (meio afasta de zero).
pub fn simple_round_to_ex(value: f64, digit: i32) -> f64 {
    let factor = libm::pow(10.0, f64::from(digit));
    if !factor.is_finite() || factor == 0.0 {
        return value;
    }
    if value < 0.0 {
        libm::trunc((value / factor) - 0.5) * factor
    } else {
        libm::trunc((value / factor) + 0.5) * factor
    }
}

/// Corrige ruído de ponto flutuante arredondando nove casas antes de truncar.
pub fn trunc_fix(value: f64) -> i64 {
    libm::trunc(simple_round_to_ex(value, -9)) as i64
}

fn round_ties_even(value: f64) -> f64 {
    let lower = libm::floor(value);
    let fraction = value - lower;
    if fraction < 0.5 {
        lower
    } else if fraction > 0.5 {
        lower + 1.0
    } else if (lower as i64) & 1 == 0 {
        lower
    } else {
        lower + 1.0
    }
}

/// Arredonda conforme a regra ABNT NBR 5891/77, com empate para o par.
pub fn round_abnt(value: f64, digits: i32, delta: f64) -> f64 {
    let pow = libm::pow(10.0, f64::from(digits.saturating_abs()));
    if !pow.is_finite() || pow == 0.0 || !value.is_finite() {
        return value;
    }
    let negative = value < 0.0;
    let pow_value = libm::fabs(value) / 10.0;
    let int_value = libm::trunc(pow_value);
    let frac_value = pow_value - libm::trunc(pow_value);
    let scaled = simple_round_to_ex(frac_value * 10.0 * pow, -9);
    let mut int_calc = libm::trunc(scaled);
    let frac_calc = libm::trunc((scaled - libm::trunc(scaled)) * 100.0);
    if frac_calc > 50.0 {
        int_calc += 1.0;
    } else if frac_calc == 50.0 {
        let last_number = round_ties_even((int_calc / 10.0 - libm::trunc(int_calc / 10.0)) * 10.0);
        if (last_number as i64) % 2 != 0 {
            int_calc += 1.0;
        } else if ((scaled * 10.0) - libm::trunc(scaled * 10.0)) > delta {
            int_calc += 1.0;
        }
    }
    let result = (int_value * 10.0) + int_calc / pow;
    if negative {
        -result
    } else {
        result
    }
}

/// Trunca em casas decimais mantendo o sinal e parte inteira.
pub fn trunc_to(value: f64, digits: i32) -> f64 {
    if !value.is_finite() {
        return value;
    }
    let pow = libm::pow(10.0, f64::from(digits.saturating_abs()));
    if !pow.is_finite() || pow == 0.0 {
        return value;
    }
    libm::trunc(value) + trunc_fix((value - libm::trunc(value)) * pow) as f64 / pow
}

/// Retorna -1, 0 ou 1 comparando doubles com tolerância absoluta.
pub fn compara_valor(a: f64, b: f64, tolerancia: f64) -> i32 {
    let diff = simple_round_to_ex(libm::fabs(a - b), -9);
    if diff <= tolerancia {
        0
    } else if a < b {
        -1
    } else {
        1
    }
}

pub fn test_bit(value: i32, index: u8) -> bool {
    index < 32 && ((value as u32 >> index) & 1) == 1
}
pub fn set_bit(value: i32, index: u8) -> i32 {
    if index < 32 {
        value | (1_i32.wrapping_shl(u32::from(index)))
    } else {
        value
    }
}
pub fn clear_bit(value: i32, index: u8) -> i32 {
    if index < 32 {
        value & !(1_i32.wrapping_shl(u32::from(index)))
    } else {
        value
    }
}
pub fn put_bit(value: i32, index: u8, state: bool) -> i32 {
    if state {
        set_bit(value, index)
    } else {
        clear_bit(value, index)
    }
}

/// Converte inteiro para string binária com largura fixa; valores negativos seguem o comportamento ACBr.
pub fn int_to_bin(mut value: i32, digits: usize) -> alloc::string::String {
    let mut out = alloc::vec![b'0'; digits];
    let mut pos = digits;
    while value > 0 && pos > 0 {
        pos -= 1;
        if value & 1 != 0 {
            out[pos] = b'1';
        }
        value >>= 1;
    }
    alloc::string::String::from_utf8(out).expect("saída binária ASCII válida")
}

/// Converte uma representação binária; caracteres diferentes de `1` contam como zero, como no Pascal.
pub fn bin_to_int(value: &str) -> Result<i32, MathError> {
    let mut result = 0_i32;
    for c in value.chars() {
        result = result.checked_mul(2).ok_or(MathError::Overflow)?;
        if c == '1' {
            result = result.checked_add(1).ok_or(MathError::Overflow)?;
        }
    }
    Ok(result)
}

/// Converte bytes BCD para os dígitos ASCII da representação ACBr.
pub fn bcd_to_asc(input: &[u8]) -> alloc::string::String {
    use core::fmt::Write;
    let mut out = alloc::string::String::new();
    for byte in input {
        let hi = byte >> 4;
        let lo = byte & 15;
        let _ = write!(out, "{}", hi);
        if lo > 9 {
            out.push(char::from(48 + lo));
        } else {
            out.push(char::from(b'0' + lo));
        }
    }
    out
}

/// Codifica dígitos em BCD, preenchendo à esquerda; entrada excedente preserva o prefixo ACBr.
pub fn asc_to_bcd(input: &str, bytes: usize) -> Result<alloc::vec::Vec<u8>, MathError> {
    if input.is_empty() || !input.bytes().all(|b| b.is_ascii_digit()) {
        return Err(MathError::InvalidDigit);
    }
    let width = bytes.checked_mul(2).ok_or(MathError::Overflow)?;
    let mut digits = alloc::vec![b'0';width];
    let take = input.len().min(width);
    digits[width - take..].copy_from_slice(&input.as_bytes()[..take]);
    Ok(digits
        .chunks_exact(2)
        .map(|p| ((p[0] - b'0') << 4) | (p[1] - b'0'))
        .collect())
}

pub fn int_to_le_bytes(value: i32, bytes: usize) -> Result<alloc::vec::Vec<u8>, MathError> {
    if bytes > 4 {
        return Err(MathError::InvalidLength);
    }
    Ok(value.to_le_bytes()[..bytes].to_vec())
}
pub fn le_bytes_to_int(bytes: &[u8]) -> Result<i32, MathError> {
    if bytes.len() > 4 {
        return Err(MathError::InvalidLength);
    }
    let mut full = [0_u8; 4];
    for (i, b) in bytes.iter().enumerate() {
        full[i] = *b;
    }
    Ok(i32::from_le_bytes(full))
}
pub fn int_to_be_bytes(value: i32, bytes: usize) -> Result<alloc::vec::Vec<u8>, MathError> {
    if bytes > 4 {
        return Err(MathError::InvalidLength);
    }
    Ok(value.to_be_bytes()[4 - bytes..].to_vec())
}
pub fn be_bytes_to_int(bytes: &[u8]) -> Result<i32, MathError> {
    if bytes.len() > 4 {
        return Err(MathError::InvalidLength);
    }
    let mut full = [0_u8; 4];
    full[4 - bytes.len()..].copy_from_slice(bytes);
    Ok(i32::from_be_bytes(full))
}

/// Decodifica pares hexadecimais; um nibble final isolado é ignorado como na unit Pascal.
pub fn hex_to_ascii(input: &str) -> Result<alloc::vec::Vec<u8>, MathError> {
    let trimmed = input.trim_matches(|c: char| c.is_ascii() && c <= ' ');
    let bytes = trimmed.as_bytes();
    let mut out = alloc::vec::Vec::with_capacity(bytes.len() / 2);
    let mut i = 0;
    while i + 1 < bytes.len() {
        let hi = (bytes[i] as char)
            .to_digit(16)
            .ok_or(MathError::InvalidDigit)?;
        let lo = (bytes[i + 1] as char)
            .to_digit(16)
            .ok_or(MathError::InvalidDigit)?;
        out.push(((hi << 4) | lo) as u8);
        i += 2;
    }
    Ok(out)
}
pub fn try_hex_to_ascii(input: &str) -> Option<alloc::vec::Vec<u8>> {
    hex_to_ascii(input).ok()
}
pub fn hex_to_ascii_def(input: &str, default: &[u8]) -> alloc::vec::Vec<u8> {
    try_hex_to_ascii(input).unwrap_or_else(|| default.into())
}
pub fn ascii_to_hex(input: &[u8]) -> alloc::string::String {
    use core::fmt::Write;
    let mut out = alloc::string::String::with_capacity(input.len() * 2);
    for b in input {
        let _ = write!(out, "{:02X}", b);
    }
    out
}

/// Escapa bytes de controle e bytes acima de 127 no formato `\\xNN`.
pub fn binary_string_to_string(input: &[u8]) -> alloc::string::String {
    use core::fmt::Write;
    let mut out = alloc::string::String::new();
    for b in input {
        if *b < 32 || *b > 127 {
            let _ = write!(out, "\\x{:02X}", b);
        } else {
            out.push(char::from(*b));
        }
    }
    out
}

/// Decodifica sequências `\\xNN` em bytes; outras partes UTF-8 são preservadas.
pub fn string_to_binary_string(input: &str) -> alloc::vec::Vec<u8> {
    let bytes = input.as_bytes();
    let mut out = alloc::vec::Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if i + 3 < bytes.len() && bytes[i] == b'\\' && bytes[i + 1] == b'x' {
            if let (Some(hi), Some(lo)) = (
                (bytes[i + 2] as char).to_digit(16),
                (bytes[i + 3] as char).to_digit(16),
            ) {
                out.push(((hi << 4) | lo) as u8);
                i += 4;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    out
}

#[cfg(test)]
mod full_math_tests {
    use super::*;
    #[test]
    fn numeric_rounding_and_comparison() {
        assert_eq!(simple_round_to_ex(1.235, -2), 1.24);
        assert_eq!(simple_round_to_ex(-1.235, -2), -1.24);
        assert_eq!(trunc_fix(1.9999999999), 2);
        assert_eq!(trunc_to(12.349, 2), 12.34);
        assert_eq!(round_abnt(2.5, 0, 0.00001), 2.0);
        assert_eq!(round_abnt(3.5, 0, 0.00001), 4.0);
        assert_eq!(compara_valor(1.0000000001, 1.0, 0.0), 0);
        assert_eq!(compara_valor(1.1, 1.0, 0.0), 1);
    }
    #[test]
    fn bits_and_binary() {
        assert_eq!(put_bit(0, 3, true), 8);
        assert!(test_bit(8, 3));
        assert_eq!(clear_bit(15, 2), 11);
        assert_eq!(set_bit(0, 31), i32::MIN);
        assert_eq!(int_to_bin(100, 8), "01100100");
        assert_eq!(int_to_bin(3, 1), "1");
        assert_eq!(bin_to_int("11111111"), Ok(255));
        assert_eq!(
            bin_to_int("111111111111111111111111111111111"),
            Err(MathError::Overflow)
        );
    }
    #[test]
    fn bcd_and_endian() {
        assert_eq!(bcd_to_asc(&[0x01, 0x47, 0x84, 0x01, 0x70]), "0147840170");
        assert_eq!(asc_to_bcd("123456", 3), Ok(alloc::vec![0x12, 0x34, 0x56]));
        assert_eq!(asc_to_bcd("12x", 2), Err(MathError::InvalidDigit));
        assert_eq!(asc_to_bcd("", 2), Err(MathError::InvalidDigit));
        assert_eq!(int_to_le_bytes(106, 2), Ok(alloc::vec![106, 0]));
        assert_eq!(le_bytes_to_int(&[106, 0]), Ok(106));
        assert_eq!(int_to_be_bytes(106, 2), Ok(alloc::vec![0, 106]));
        assert_eq!(be_bytes_to_int(&[0, 106]), Ok(106));
        assert_eq!(int_to_le_bytes(1, 5), Err(MathError::InvalidLength));
    }
    #[test]
    fn hex_and_binary_escape() {
        assert_eq!(hex_to_ascii(" 4142F "), Ok(alloc::vec![0x41, 0x42]));
        assert_eq!(hex_to_ascii("GG"), Err(MathError::InvalidDigit));
        assert_eq!(ascii_to_hex(&[0, 65, 255]), "0041FF");
        assert_eq!(
            binary_string_to_string(&[0, 65, 127, 255]),
            "\\x00A\u{7f}\\xFF"
        );
        assert_eq!(
            string_to_binary_string("\\x00A\\xFF"),
            alloc::vec![0, 65, 255]
        );
    }
}
