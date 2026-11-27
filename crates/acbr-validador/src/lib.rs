// SPDX-License-Identifier: LGPL-2.1-or-later
//! Validações, formatos e calculador portados de `ACBrValidador` (SVN r49422).
//! Perfis padrões: não ajusta tamanho, não permite vazio, ignora `.`, `/` e `-`.
#![no_std]
extern crate alloc;
use alloc::string::String;
mod calculador;
pub use calculador::*;
mod componente;
pub use componente::*;
mod formato;
pub use formato::*;
mod ie;
pub use ie::validar_ie;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    Empty,
    InvalidLength,
    InvalidCharacter,
    InvalidCheckDigit,
    RepeatedDigits,
}

fn normalize(input: &str) -> String {
    let cleaned: String = input
        .chars()
        .filter(|c| !matches!(c, '.' | '/' | '-'))
        .map(|c| c.to_ascii_uppercase())
        .collect();
    // Delphi/FPC Trim no ASCII removes control bytes through space at both ends.
    cleaned
        .trim_matches(|c: char| c.is_ascii() && c <= ' ')
        .into()
}

pub fn validar_cpf(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if b.len() != 11 {
        return Err(ValidationError::InvalidLength);
    }
    if !b.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidCharacter);
    }
    if b.iter().all(|v| *v == b[0]) {
        return Err(ValidationError::RepeatedDigits);
    }
    let first = acbr_math::modulo11_ascii(&b[..9], 2, 11).unwrap();
    let second = acbr_math::modulo11_ascii(&b[..10], 2, 11).unwrap();
    if b[9] != b'0' + first || b[10] != b'0' + second {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

pub fn validar_cnpj(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if b.len() != 14 {
        return Err(ValidationError::InvalidLength);
    }
    if !b.iter().all(u8::is_ascii_alphanumeric) {
        return Err(ValidationError::InvalidCharacter);
    }
    if b.iter().all(|&v| v == b'0') {
        return Err(ValidationError::RepeatedDigits);
    }
    let first = acbr_math::modulo11_ascii(&b[..12], 2, 9).unwrap();
    let second = acbr_math::modulo11_ascii(&b[..13], 2, 9).unwrap();
    if b[12] != b'0' + first || b[13] != b'0' + second {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

pub fn validar_gtin(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if !b.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidCharacter);
    }
    if !matches!(b.len(), 8 | 12 | 13 | 14) {
        return Err(ValidationError::InvalidLength);
    }
    let sum: u32 = b[..b.len() - 1]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &v)| u32::from(v - b'0') * if i % 2 == 0 { 3 } else { 1 })
        .sum();
    if b[b.len() - 1] != b'0' + ((10 - sum % 10) % 10) as u8 {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cpf() {
        assert!(validar_cpf("529.982.247-25").is_ok());
        assert!(validar_cpf("529.982.247-24").is_err());
        assert!(validar_cpf("111.111.111-11").is_err());
        assert!(validar_cpf("529 98224725").is_err());
    }
    #[test]
    fn cnpj() {
        assert!(validar_cnpj("11.222.333/0001-81").is_ok());
        assert!(validar_cnpj("12.ABC.345/01DE-35").is_ok());
        assert!(validar_cnpj("12.ABC.345/01DE-36").is_err());
        assert!(validar_cnpj("00.000.000/0000-00").is_err());
    }
    #[test]
    fn gtin() {
        assert!(validar_gtin("7894900011517").is_ok());
        assert!(validar_gtin("7894900011518").is_err());
        assert!(validar_gtin("00000000").is_ok());
        assert!(validar_gtin("123").is_err());
    }
}

/// Verifica a sigla de UF aceita pelo ACBr, incluindo EX.
pub fn validar_uf(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    if n.is_empty() {
        return Err(ValidationError::Empty);
    }
    if matches!(
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
        Ok(())
    } else {
        Err(ValidationError::InvalidCharacter)
    }
}

/// O ACBr verifica sete caracteres alfanuméricos, sem validar posições.
pub fn validar_placa(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if b.len() != 7 {
        return Err(ValidationError::InvalidLength);
    }
    if !b.iter().all(u8::is_ascii_alphanumeric) {
        return Err(ValidationError::InvalidCharacter);
    }
    Ok(())
}

fn numeric(input: &str, len: usize) -> Result<String, ValidationError> {
    let n = normalize(input);
    if n.is_empty() {
        return Err(ValidationError::Empty);
    }
    if n.len() != len {
        return Err(ValidationError::InvalidLength);
    }
    if !n.as_bytes().iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidCharacter);
    }
    Ok(n)
}

pub fn validar_pis(input: &str) -> Result<(), ValidationError> {
    let n = numeric(input, 11)?;
    let b = n.as_bytes();
    let first = acbr_math::modulo11_ascii(&b[..10], 2, 9).unwrap();
    if b[10] != b'0' + first {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

pub fn validar_renavam(input: &str) -> Result<(), ValidationError> {
    let n = numeric(input, 11)?;
    let b = n.as_bytes();
    let sum: u32 = b[..10]
        .iter()
        .zip(b"3298765432")
        .map(|(&x, &w)| u32::from(x - b'0') * u32::from(w - b'0'))
        .sum();
    let mut dv = (sum * 10) % 11;
    if dv == 10 {
        dv = 0
    }
    if b[10] != b'0' + dv as u8 {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

pub fn validar_suframa(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if b.len() < 9 {
        return Err(ValidationError::InvalidLength);
    }
    if !b.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidCharacter);
    }
    let dv = acbr_math::modulo11_ascii(&b[..8], 2, 9).unwrap();
    if b[8] != b'0' + dv {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

pub fn validar_caepf(input: &str) -> Result<(), ValidationError> {
    let n = numeric(input, 14)?;
    let b = n.as_bytes();
    if b.iter().all(|&v| v == b'0') {
        return Err(ValidationError::RepeatedDigits);
    }
    let d1 = acbr_math::modulo11_ascii(&b[..12], 2, 9).unwrap();
    let mut tmp = [0u8; 13];
    tmp[..12].copy_from_slice(&b[..12]);
    tmp[12] = b'0' + d1;
    let d2 = acbr_math::modulo11_ascii(&tmp, 2, 9).unwrap();
    let dv = (d1 * 10 + d2 + 12) % 100;
    if b[12] != b'0' + dv / 10 || b[13] != b'0' + dv % 10 {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

/// Reproduz o cálculo do ACBr, que alterna pesos a partir do primeiro dígito.
pub fn validar_cartao_credito(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if !b.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidCharacter);
    }
    let sum: u32 = b[..b.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let v = u32::from(x - b'0') * if i % 2 == 0 { 2 } else { 1 };
            v / 10 + v % 10
        })
        .sum();
    if b[b.len() - 1] != b'0' + ((10 - sum % 10) % 10) as u8 {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

pub fn validar_cnh(input: &str) -> Result<(), ValidationError> {
    let n = numeric(input, 11)?;
    let b = n.as_bytes();
    if b.iter().all(|&v| v == b[0]) {
        return Err(ValidationError::RepeatedDigits);
    }
    let sum1: u32 = b[..9]
        .iter()
        .enumerate()
        .map(|(i, &v)| u32::from(v - b'0') * (9 - i as u32))
        .sum();
    let r1 = sum1 % 11;
    let d1 = if r1 > 9 { 0 } else { r1 };
    let sum2: u32 = b[..9]
        .iter()
        .enumerate()
        .map(|(i, &v)| u32::from(v - b'0') * (1 + i as u32))
        .sum();
    let mut r2 = (sum2 % 11) as i32 + if r1 == 10 { -2 } else { 0 };
    if r2 < 0 {
        r2 += 11
    }
    if r2 > 9 {
        r2 = 0
    }
    if b[9] != b'0' + d1 as u8 || b[10] != b'0' + r2 as u8 {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

/// Valida a faixa de CEP da referência, opcionalmente restringindo à UF.
/// As faixas são as presentes no ACBr na revisão fixada, sem consulta online.
pub fn validar_cep(input: &str, uf: &str) -> Result<(), ValidationError> {
    let n = numeric(input, 8)?;
    let uf = uf.trim_matches(|c: char| c.is_ascii() && c <= ' ');
    if !uf.is_empty()
        && (!matches!(
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
        ))
    {
        return Err(ValidationError::InvalidCharacter);
    }
    let value: u32 = n.parse().unwrap();
    let ranges: &[(u32, u32, &str)] = &[
        (1000000, 19999999, "SP"),
        (20000000, 28999999, "RJ"),
        (29000000, 29999999, "ES"),
        (30000000, 39999999, "MG"),
        (40000000, 48999999, "BA"),
        (49000000, 49999999, "SE"),
        (50000000, 56999999, "PE"),
        (57000000, 57999999, "AL"),
        (58000000, 58999999, "PB"),
        (59000000, 59999999, "RN"),
        (60000000, 63999999, "CE"),
        (64000000, 64999999, "PI"),
        (65000000, 65999999, "MA"),
        (66000000, 68899999, "PA"),
        (68900000, 68999999, "AP"),
        (69000000, 69299999, "AM"),
        (69300000, 69399999, "RR"),
        (69400000, 69899999, "AM"),
        (69900000, 69999999, "AC"),
        (70000000, 72799999, "DF"),
        (72800000, 72999999, "GO"),
        (73000000, 73699999, "DF"),
        (73700000, 76799999, "GO"),
        (77000000, 77999999, "TO"),
        (78000000, 78899999, "MT"),
        (76800000, 76999999, "RO"),
        (79000000, 79999999, "MS"),
        (80000000, 87999999, "PR"),
        (88000000, 89999999, "SC"),
        (90000000, 99999999, "RS"),
    ];
    if ranges
        .iter()
        .any(|(lo, hi, state)| value >= *lo && value <= *hi && (uf.is_empty() || uf == *state))
    {
        Ok(())
    } else {
        Err(ValidationError::InvalidCheckDigit)
    }
}

pub fn validar_cheque(input: &str) -> Result<(), ValidationError> {
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if !b.iter().all(u8::is_ascii_digit) {
        return Err(ValidationError::InvalidCharacter);
    }
    let dv = acbr_math::modulo11_ascii(&b[..b.len() - 1], 2, 9).unwrap();
    if b[b.len() - 1] != b'0' + dv {
        return Err(ValidationError::InvalidCheckDigit);
    }
    Ok(())
}

/// Valida CNS reproduzindo os cálculos das famílias 1/2 e 7/8/9 no ACBr.
pub fn validar_cns(input: &str) -> Result<(), ValidationError> {
    use alloc::format;
    let n = normalize(input);
    let b = n.as_bytes();
    if b.is_empty() {
        return Err(ValidationError::Empty);
    }
    if b.len() != 15 {
        return Err(ValidationError::InvalidLength);
    }
    if !n.is_ascii() {
        return Err(ValidationError::InvalidCharacter);
    }
    let valid = match b[0] {
        b'1' | b'2' => {
            let sum: i32 = b[..11]
                .iter()
                .enumerate()
                .map(|(i, &v)| (i32::from(v) - 48) * (15 - i as i32))
                .sum();
            let rem = sum % 11;
            let expected = if rem == 10 {
                format!("{}001{}", &n[..11], 11 - ((sum + 2) % 11))
            } else {
                format!("{}000{}", &n[..11], if rem < 2 { 0 } else { 11 - rem })
            };
            n == expected
        }
        b'7' | b'8' | b'9' => {
            let sum: i32 = b
                .iter()
                .enumerate()
                .map(|(i, &v)| (i32::from(v) - 48) * (15 - i as i32))
                .sum();
            sum % 11 == 0
        }
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(ValidationError::InvalidCheckDigit)
    }
}

/// Valida GTIN e sua faixa de prefixo conforme a tabela do ACBr r49422.
pub fn validar_prefixo_gtin(input: &str) -> Result<(), ValidationError> {
    validar_gtin(input)?;
    let n = normalize(input);
    let mut padded = [b'0'; 14];
    padded[14 - n.len()..].copy_from_slice(n.as_bytes());
    let start = if padded[..6] == [b'0'; 6] { 6 } else { 1 };
    let prefix = u16::from(padded[start] - b'0') * 100
        + u16::from(padded[start + 1] - b'0') * 10
        + u16::from(padded[start + 2] - b'0');
    const RANGES: &[(u16, u16)] = &[
        (0, 19),
        (20, 29),
        (30, 39),
        (40, 49),
        (50, 59),
        (60, 139),
        (200, 299),
        (300, 379),
        (380, 380),
        (383, 383),
        (385, 385),
        (387, 387),
        (389, 389),
        (400, 440),
        (450, 459),
        (490, 499),
        (460, 469),
        (470, 470),
        (471, 471),
        (474, 474),
        (475, 475),
        (476, 476),
        (477, 477),
        (478, 478),
        (479, 479),
        (480, 480),
        (481, 481),
        (482, 482),
        (483, 483),
        (484, 484),
        (485, 485),
        (486, 486),
        (487, 487),
        (488, 488),
        (489, 489),
        (500, 509),
        (520, 521),
        (528, 528),
        (529, 529),
        (530, 530),
        (531, 531),
        (535, 535),
        (539, 539),
        (540, 549),
        (560, 560),
        (569, 569),
        (570, 579),
        (590, 590),
        (594, 594),
        (599, 599),
        (600, 601),
        (603, 603),
        (604, 604),
        (608, 608),
        (609, 609),
        (611, 611),
        (613, 613),
        (615, 615),
        (616, 616),
        (618, 618),
        (619, 619),
        (620, 620),
        (621, 621),
        (622, 622),
        (623, 623),
        (624, 624),
        (625, 625),
        (626, 626),
        (627, 627),
        (628, 628),
        (629, 629),
        (640, 649),
        (690, 699),
        (700, 709),
        (729, 729),
        (730, 739),
        (740, 740),
        (741, 741),
        (742, 742),
        (743, 743),
        (744, 744),
        (745, 745),
        (746, 746),
        (750, 750),
        (754, 755),
        (759, 759),
        (760, 769),
        (770, 771),
        (773, 773),
        (775, 775),
        (777, 777),
        (778, 779),
        (780, 780),
        (784, 784),
        (786, 786),
        (789, 790),
        (800, 839),
        (840, 849),
        (850, 850),
        (858, 858),
        (859, 859),
        (860, 860),
        (865, 865),
        (867, 867),
        (868, 869),
        (870, 879),
        (880, 880),
        (884, 884),
        (885, 885),
        (888, 888),
        (890, 890),
        (893, 893),
        (896, 896),
        (899, 899),
        (900, 919),
        (930, 939),
        (940, 949),
        (950, 950),
        (951, 951),
        (955, 955),
        (958, 958),
        (960, 969),
        (977, 977),
        (978, 979),
        (980, 980),
        (981, 984),
        (990, 999),
    ];
    if RANGES.iter().any(|&(a, b)| prefix >= a && prefix <= b) {
        Ok(())
    } else {
        Err(ValidationError::InvalidCharacter)
    }
}

/// Valida endereço(s) com as regras do ACBr; listas aceitam `;`, `,` ou `|`.
/// As regras históricas da referência não substituem a confirmação de entrega.
pub fn validar_email(input: &str) -> Result<(), ValidationError> {
    let delimiter = [';', ',', '|'].into_iter().find(|&c| input.contains(c));
    if let Some(d) = delimiter {
        // TStringList.DelimitedText: separador estrito e aspas duplas.
        let mut current = String::new();
        let mut chars = input.chars().peekable();
        let mut quoted = false;
        while let Some(c) = chars.next() {
            if c == '"' {
                if quoted && chars.peek() == Some(&'"') {
                    chars.next();
                    current.push('"');
                } else if quoted {
                    quoted = false;
                } else if current.is_empty() {
                    quoted = true;
                } else {
                    current.push(c);
                }
            } else if c == d && !quoted {
                validar_email_unico(&current)?;
                current.clear();
            } else {
                current.push(c);
            }
        }
        validar_email_unico(&current)
    } else {
        validar_email_unico(input)
    }
}
fn validar_email_unico(value: &str) -> Result<(), ValidationError> {
    const INVALID: &str = " àâêôûãõáéíóúçüñýÀÂÊÔÛÃÕÁÉÍÓÚÇÜÑÝ*,;:\\|#$%&*§!()][{}<>˜ˆ´ªº+¹²³";
    if value
        .trim_matches(|c: char| c.is_ascii() && c <= ' ')
        .is_empty()
    {
        return Err(ValidationError::Empty);
    }
    if value.len() < 8 {
        return Err(ValidationError::InvalidLength);
    }
    if value.chars().any(|c| INVALID.contains(c)) {
        return Err(ValidationError::InvalidCharacter);
    }
    let mut at = value.match_indices('@');
    let pos = at
        .next()
        .map(|(i, _)| i)
        .ok_or(ValidationError::InvalidCharacter)?;
    if at.next().is_some()
        || pos == 0
        || pos + 1 >= value.len()
        || value.starts_with('.')
        || value.ends_with('.')
        || value.contains("..")
        || value.as_bytes()[pos - 1] == b'.'
        || value.as_bytes()[pos + 1] == b'.'
        || !value[pos + 1..].contains('.')
    {
        return Err(ValidationError::InvalidCharacter);
    }
    Ok(())
}
/// Escolhe CPF para menos de 12 caracteres alfanuméricos, como o ACBr.
pub fn validar_cnpj_ou_cpf(input: &str) -> Result<(), ValidationError> {
    if acbr_util::only_alpha_num(input).len() < 12 {
        validar_cpf(input)
    } else {
        validar_cnpj(input)
    }
}
/// Mantém os caracteres alfanuméricos apenas quando formam CPF ou CNPJ válido.
pub fn only_cnpj_or_cpf(input: &str) -> String {
    let n = acbr_util::only_alpha_num(input);
    if validar_cnpj_ou_cpf(&n).is_ok() {
        n
    } else {
        String::new()
    }
}
