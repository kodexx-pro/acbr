// SPDX-License-Identifier: LGPL-2.1-or-later
//! Port das rotinas portáveis de strings da `ACBrUtil.Strings` (SVN r49422).
#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};

/// Mantém apenas os dígitos ASCII, como `OnlyNumber`.
pub fn only_number(input: &str) -> String {
    input.chars().filter(char::is_ascii_digit).collect()
}

/// Mantém apenas letras ASCII, como `OnlyAlpha`.
pub fn only_alpha(input: &str) -> String {
    input.chars().filter(char::is_ascii_alphabetic).collect()
}

/// Mantém apenas letras e dígitos ASCII, como `OnlyAlphaNum`.
pub fn only_alpha_num(input: &str) -> String {
    input.chars().filter(char::is_ascii_alphanumeric).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ascii_filters() {
        assert_eq!(only_number("Ab12-٣"), "12");
        assert_eq!(only_alpha("Açb 12"), "Ab");
        assert_eq!(only_alpha_num("Açb-12"), "Ab12");
    }
}

/// `CharIsAlpha`: aceita letras ASCII maiúsculas e minúsculas.
pub fn char_is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// `CharIsNum`: aceita somente os dígitos ASCII de 0 a 9.
pub fn char_is_num(c: char) -> bool {
    c.is_ascii_digit()
}

/// `CharIsAlphaNum`: letras e dígitos ASCII.
pub fn char_is_alpha_num(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

/// `CharIsHexa`: dígito hexadecimal ASCII.
pub fn char_is_hexa(c: char) -> bool {
    c.is_ascii_hexdigit()
}

pub fn char_is_binary(c: char) -> bool {
    matches!(c, '0' | '1')
}

pub fn char_is_base64(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '+' | '/')
}

/// `StrIsAlpha`; como na unit Pascal, a string vazia retorna `true`.
pub fn str_is_alpha(s: &str) -> bool {
    s.chars().all(char_is_alpha)
}

/// `StrIsAlphaNum`; como na unit Pascal, a string vazia retorna `true`.
pub fn str_is_alpha_num(s: &str) -> bool {
    s.chars().all(char_is_alpha_num)
}

/// `StrIsNumber`; a string vazia retorna `false`.
pub fn str_is_number(s: &str) -> bool {
    !s.is_empty() && s.chars().all(char_is_num)
}

pub fn str_is_hexa(s: &str) -> bool {
    s.chars().all(char_is_hexa)
}

pub fn str_is_binary(s: &str) -> bool {
    s.chars().all(char_is_binary)
}

/// Replica o predicado de forma da unit: múltiplo de quatro e até dois `=` finais.
pub fn str_is_base64(s: &str) -> bool {
    if s.is_empty() || s.len() % 4 != 0 {
        return false;
    }
    let unpadded = s.trim_end_matches('=');
    s.len() - unpadded.len() <= 2 && unpadded.chars().all(char_is_base64)
}

/// Replica `PadLeft`: quando excede o tamanho, mantém os primeiros caracteres.
pub fn pad_left(s: &str, len: usize, fill: char) -> String {
    let chars: alloc::vec::Vec<char> = s.chars().collect();
    if chars.len() >= len {
        chars.into_iter().take(len).collect()
    } else {
        let mut out = alloc::string::String::new();
        out.extend(core::iter::repeat(fill).take(len - chars.len()));
        out.extend(chars);
        out
    }
}

/// Replica `PadRight`: quando excede o tamanho, mantém os primeiros caracteres.
pub fn pad_right(s: &str, len: usize, fill: char) -> String {
    let mut out: String = s.chars().take(len).collect();
    let current = out.chars().count();
    out.extend(core::iter::repeat(fill).take(len.saturating_sub(current)));
    out
}

pub fn pad_center(s: &str, len: usize, fill: char) -> String {
    let chars: alloc::vec::Vec<char> = s.chars().collect();
    if chars.len() >= len {
        return chars.into_iter().take(len).collect();
    }
    let left = (len - chars.len()) / 2;
    let mut out = alloc::string::String::new();
    out.extend(core::iter::repeat(fill).take(left));
    out.extend(chars);
    while out.chars().count() < len {
        out.push(fill);
    }
    out
}

/// Remove todas as ocorrências da substring (comparação literal e sensível a caixa).
pub fn remove_string(needle: &str, text: &str) -> String {
    if needle.is_empty() {
        return text.into();
    }
    text.replace(needle, "")
}

/// Remove substrings da maior para a menor, como `RemoveStrings`.
pub fn remove_strings(text: &str, needles: &[&str]) -> String {
    let mut ordered = needles.to_vec();
    ordered.sort_by_key(|s| s.len());
    let mut out = text.to_string();
    for needle in ordered.into_iter().rev() {
        if !needle.is_empty() {
            out = out.replace(needle, "");
        }
    }
    out
}

pub fn remove_zeros_left(number: &str) -> String {
    let trimmed = number.trim_start_matches('0');
    if trimmed.is_empty() && !number.is_empty() {
        "0".into()
    } else {
        trimmed.into()
    }
}

/// Conta ocorrências permitindo sobreposição, como `CountStr`/`PosLast`.
pub fn count_str(text: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    text.match_indices(needle).count()
}

/// Busca a ocorrência N, retornando posição de base 1 em bytes (`0` se ausente).
pub fn pos_at(needle: &str, text: &str, occurrence: usize) -> usize {
    if needle.is_empty() || occurrence == 0 {
        return 0;
    }
    text.match_indices(needle)
        .nth(occurrence - 1)
        .map_or(0, |(i, _)| i + 1)
}

/// Posição da última ocorrência, de base 1 em bytes (`0` se ausente).
pub fn pos_last(needle: &str, text: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    text.match_indices(needle).last().map_or(0, |(i, _)| i + 1)
}

/// Acrescenta zeros à esquerda e trunca mantendo os primeiros caracteres.
pub fn poem_zeros(text: &str, len: usize) -> String {
    pad_left(text.trim(), len, '0')
}

/// Preenche sem truncar, como `TBStrZero`.
pub fn tb_str_zero(text: &str, len: usize) -> String {
    if text.chars().count() >= len {
        return text.into();
    }
    let mut out = alloc::string::String::new();
    out.extend(core::iter::repeat('0').take(len - text.chars().count()));
    out.push_str(text);
    out
}

pub fn left_str(s: &str, len: usize) -> String {
    s.chars().take(len).collect()
}

pub fn right_str(s: &str, len: usize) -> String {
    let count = s.chars().count();
    s.chars().skip(count.saturating_sub(len)).collect()
}

pub fn pad_space(s: &str, len: usize, separator: &str, fill: char, remove_spaces: bool) -> String {
    pad_space_full(s, len, separator, fill, remove_spaces)
}

pub fn if_empty_then<'a>(value: &'a str, default: &'a str, trim: bool) -> &'a str {
    if (if trim { value.trim() } else { value }).is_empty() {
        default
    } else {
        value
    }
}

pub fn remove_double_spaces(s: &str) -> String {
    let mut out = alloc::string::String::new();
    let mut was_space = false;
    for c in s.trim().chars() {
        if c == ' ' {
            if !was_space {
                out.push(c);
            }
            was_space = true;
        } else {
            was_space = false;
            out.push(c);
        }
    }
    out
}

pub fn remove_line_break_final(text: &str, line_break: &str) -> String {
    let suffix = if line_break.is_empty() {
        if cfg!(windows) {
            "\r\n"
        } else {
            "\n"
        }
    } else {
        line_break
    };
    text.strip_suffix(suffix).unwrap_or(text).into()
}

pub fn tira_pontos(text: &str) -> String {
    text.chars()
        .filter(|c| !matches!(c, '/' | ',' | '-' | '.' | ')' | '(' | ' '))
        .collect()
}

pub fn space(len: usize) -> String {
    alloc::string::String::from_iter(core::iter::repeat(' ').take(len))
}
pub fn linha_simples(len: usize) -> String {
    alloc::string::String::from_iter(core::iter::repeat('-').take(len))
}
pub fn linha_dupla(len: usize) -> String {
    alloc::string::String::from_iter(core::iter::repeat('=').take(len))
}

pub fn match_text(text: &str, values: &[&str]) -> bool {
    values.iter().any(|value| *value == text)
}

pub fn find_delimiter_in_text(text: &str, delimiters: &str) -> Option<char> {
    let delimiters = if delimiters.is_empty() {
        ";,|"
    } else {
        delimiters
    };
    delimiters.chars().find(|c| text.contains(*c))
}

pub fn split(delimiter: char, text: &str) -> alloc::vec::Vec<String> {
    text.split(delimiter).map(ToString::to_string).collect()
}

pub fn split_to_string(parts: &[&str], delimiter: &str) -> String {
    parts.join(delimiter)
}

pub fn retornar_conteudo_entre(
    text: &str,
    start: &str,
    end: &str,
    include_markers: bool,
) -> Option<String> {
    let first = text.find(start)?;
    let content_start = first + start.len();
    let last = text[content_start..].find(end)? + content_start;
    if include_markers {
        Some(text[first..last + end.len()].into())
    } else {
        Some(text[content_start..last].into())
    }
}

/// Divide uma lista delimitada respeitando campos entre aspas; aspas externas são removidas.
pub fn add_delimited_text_to_list(
    text: &str,
    delimiter: char,
    quote: char,
) -> alloc::vec::Vec<String> {
    if text.is_empty() {
        return alloc::vec::Vec::new();
    }
    let mut values = alloc::vec::Vec::new();
    let mut current = alloc::string::String::new();
    let mut quoted = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if quote != '\0' && c == quote {
            if quoted && chars.peek() == Some(&quote) {
                current.push(quote);
                chars.next();
            } else {
                quoted = !quoted;
            }
        } else if c == delimiter && !quoted {
            values.push(core::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    values.push(current);
    values
}

pub fn change_line_break(text: &str, replacement: &str) -> String {
    if text.trim().is_empty() {
        return text.into();
    }
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    normalized.replace('\n', replacement)
}

/// Replica `SubStrEmSubStr`: verifica se algum item delimitado por `|` está presente.
pub fn substr_em_substr(search_in: &str, candidates: &str) -> bool {
    candidates
        .split('|')
        .filter(|s| !s.is_empty())
        .any(|candidate| search_in.contains(candidate))
}

/// Quebra texto em linhas preservando palavras sempre que possível.
pub fn quebra_linhas(text: &str, columns: usize, break_char: char) -> String {
    if columns == 0 {
        return text.into();
    }
    let mut result = alloc::string::String::new();
    for (line_index, source_line) in text.lines().enumerate() {
        if line_index > 0 {
            result.push('\n');
        }
        let mut current = alloc::string::String::new();
        for word in source_line.split_inclusive(break_char) {
            let word = word.trim_end_matches(break_char);
            if !current.is_empty() && current.chars().count() + 1 + word.chars().count() > columns {
                result.push_str(current.trim_end());
                result.push('\n');
                current.clear();
            }
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
            if word.chars().count() > columns {
                result.push_str(&current);
                result.push('\n');
                current.clear();
            }
        }
        result.push_str(&current);
    }
    result
}

/// Translitera o conjunto comum de letras acentuadas latinas para ASCII.
pub fn tira_acentos(text: &str) -> String {
    let bytes = acbr_str_to_ansi(text);
    let mut output = alloc::vec::Vec::with_capacity(bytes.len());
    for byte in bytes {
        let c = tira_acento_byte(byte);
        output.push(if matches!(c, 32..=126 | 8 | 10 | 13) {
            c
        } else {
            b' '
        });
    }
    String::from_utf8(output).unwrap_or_default()
}

/// Erros explícitos para conversão de páginas de código não implementadas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodePageError {
    Unsupported(u16),
}

fn encoding_for_code_page(cp: u16) -> Option<&'static encoding_rs::Encoding> {
    use encoding_rs::*;
    match cp {
        0 | 1252 => Some(WINDOWS_1252),
        1250 => Some(WINDOWS_1250),
        1251 => Some(WINDOWS_1251),
        1253 => Some(WINDOWS_1253),
        1254 => Some(WINDOWS_1254),
        1255 => Some(WINDOWS_1255),
        1256 => Some(WINDOWS_1256),
        1257 => Some(WINDOWS_1257),
        1258 => Some(WINDOWS_1258),
        874 => Some(WINDOWS_874),
        28592 => Some(ISO_8859_2),
        65001 => Some(UTF_8),
        _ => None,
    }
}

const CP437: [u16; 128] = [
    0x00C7, 0x00FC, 0x00E9, 0x00E2, 0x00E4, 0x00E0, 0x00E5, 0x00E7, 0x00EA, 0x00EB, 0x00E8, 0x00EF,
    0x00EE, 0x00EC, 0x00C4, 0x00C5, 0x00C9, 0x00E6, 0x00C6, 0x00F4, 0x00F6, 0x00F2, 0x00FB, 0x00F9,
    0x00FF, 0x00D6, 0x00DC, 0x00A2, 0x00A3, 0x00A5, 0x20A7, 0x0192, 0x00E1, 0x00ED, 0x00F3, 0x00FA,
    0x00F1, 0x00D1, 0x00AA, 0x00BA, 0x00BF, 0x2310, 0x00AC, 0x00BD, 0x00BC, 0x00A1, 0x00AB, 0x00BB,
    0x2591, 0x2592, 0x2593, 0x2502, 0x2524, 0x2561, 0x2562, 0x2556, 0x2555, 0x2563, 0x2551, 0x2557,
    0x255D, 0x255C, 0x255B, 0x2510, 0x2514, 0x2534, 0x252C, 0x251C, 0x2500, 0x253C, 0x255E, 0x255F,
    0x255A, 0x2554, 0x2569, 0x2566, 0x2560, 0x2550, 0x256C, 0x2567, 0x2568, 0x2564, 0x2565, 0x2559,
    0x2558, 0x2552, 0x2553, 0x256B, 0x256A, 0x2518, 0x250C, 0x2588, 0x2584, 0x258C, 0x2590, 0x2580,
    0x03B1, 0x00DF, 0x0393, 0x03C0, 0x03A3, 0x03C3, 0x00B5, 0x03C4, 0x03A6, 0x0398, 0x03A9, 0x03B4,
    0x221E, 0x03C6, 0x03B5, 0x2229, 0x2261, 0x00B1, 0x2265, 0x2264, 0x2320, 0x2321, 0x00F7, 0x2248,
    0x00B0, 0x2219, 0x00B7, 0x221A, 0x207F, 0x00B2, 0x25A0, 0x00A0,
];

const CP850: [u16; 128] = [
    0x00C7, 0x00FC, 0x00E9, 0x00E2, 0x00E4, 0x00E0, 0x00E5, 0x00E7, 0x00EA, 0x00EB, 0x00E8, 0x00EF,
    0x00EE, 0x00EC, 0x00C4, 0x00C5, 0x00C9, 0x00E6, 0x00C6, 0x00F4, 0x00F6, 0x00F2, 0x00FB, 0x00F9,
    0x00FF, 0x00D6, 0x00DC, 0x00F8, 0x00A3, 0x00D8, 0x00D7, 0x0192, 0x00E1, 0x00ED, 0x00F3, 0x00FA,
    0x00F1, 0x00D1, 0x00AA, 0x00BA, 0x00BF, 0x00AE, 0x00AC, 0x00BD, 0x00BC, 0x00A1, 0x00AB, 0x00BB,
    0x2591, 0x2592, 0x2593, 0x2502, 0x2524, 0x00C1, 0x00C2, 0x00C0, 0x00A9, 0x2563, 0x2551, 0x2557,
    0x255D, 0x00A2, 0x00A5, 0x2510, 0x2514, 0x2534, 0x252C, 0x251C, 0x2500, 0x253C, 0x00E3, 0x00C3,
    0x255A, 0x2554, 0x2569, 0x2566, 0x2560, 0x2550, 0x256C, 0x00A4, 0x00F0, 0x00D0, 0x00CA, 0x00CB,
    0x00C8, 0x0131, 0x00CD, 0x00CE, 0x00CF, 0x2518, 0x250C, 0x2588, 0x2584, 0x00A6, 0x00CC, 0x2580,
    0x00D3, 0x00DF, 0x00D4, 0x00D2, 0x00F5, 0x00D5, 0x00B5, 0x00FE, 0x00DE, 0x00DA, 0x00DB, 0x00D9,
    0x00FD, 0x00DD, 0x00AF, 0x00B4, 0x00AD, 0x00B1, 0x2017, 0x00BE, 0x00B6, 0x00A7, 0x00F7, 0x00B8,
    0x00B0, 0x00A8, 0x00B7, 0x00B9, 0x00B3, 0x00B2, 0x25A0, 0x00A0,
];

const CP852: [u16; 128] = [
    0x00C7, 0x00FC, 0x00E9, 0x00E2, 0x00E4, 0x016F, 0x0107, 0x00E7, 0x0142, 0x00EB, 0x0150, 0x0151,
    0x00EE, 0x0179, 0x00C4, 0x0106, 0x00C9, 0x0139, 0x013A, 0x00F4, 0x00F6, 0x013D, 0x013E, 0x015A,
    0x015B, 0x00D6, 0x00DC, 0x0164, 0x0165, 0x0141, 0x00D7, 0x010D, 0x00E1, 0x00ED, 0x00F3, 0x00FA,
    0x0104, 0x0105, 0x017D, 0x017E, 0x0118, 0x0119, 0x00AC, 0x017A, 0x010C, 0x015F, 0x00AB, 0x00BB,
    0x2591, 0x2592, 0x2593, 0x2502, 0x2524, 0x00C1, 0x00C2, 0x011A, 0x015E, 0x2563, 0x2551, 0x2557,
    0x255D, 0x017B, 0x017C, 0x2510, 0x2514, 0x2534, 0x252C, 0x251C, 0x2500, 0x253C, 0x0102, 0x0103,
    0x255A, 0x2554, 0x2569, 0x2566, 0x2560, 0x2550, 0x256C, 0x00A4, 0x0111, 0x0110, 0x010E, 0x00CB,
    0x010F, 0x0147, 0x00CD, 0x00CE, 0x011B, 0x2518, 0x250C, 0x2588, 0x2584, 0x0162, 0x016E, 0x2580,
    0x00D3, 0x00DF, 0x00D4, 0x0143, 0x0144, 0x0148, 0x0160, 0x0161, 0x0154, 0x00DA, 0x0155, 0x0170,
    0x00FD, 0x00DD, 0x0163, 0x00B4, 0x00AD, 0x02DD, 0x02DB, 0x02C7, 0x02D8, 0x00A7, 0x00F7, 0x00B8,
    0x00B0, 0x00A8, 0x02D9, 0x0171, 0x0158, 0x0159, 0x25A0, 0x00A0,
];

const CP866: [u16; 128] = [
    0x0410, 0x0411, 0x0412, 0x0413, 0x0414, 0x0415, 0x0416, 0x0417, 0x0418, 0x0419, 0x041A, 0x041B,
    0x041C, 0x041D, 0x041E, 0x041F, 0x0420, 0x0421, 0x0422, 0x0423, 0x0424, 0x0425, 0x0426, 0x0427,
    0x0428, 0x0429, 0x042A, 0x042B, 0x042C, 0x042D, 0x042E, 0x042F, 0x0430, 0x0431, 0x0432, 0x0433,
    0x0434, 0x0435, 0x0436, 0x0437, 0x0438, 0x0439, 0x043A, 0x043B, 0x043C, 0x043D, 0x043E, 0x043F,
    0x2591, 0x2592, 0x2593, 0x2502, 0x2524, 0x2561, 0x2562, 0x2556, 0x2555, 0x2563, 0x2551, 0x2557,
    0x255D, 0x255C, 0x255B, 0x2510, 0x2514, 0x2534, 0x252C, 0x251C, 0x2500, 0x253C, 0x255E, 0x255F,
    0x255A, 0x2554, 0x2569, 0x2566, 0x2560, 0x2550, 0x256C, 0x2567, 0x2568, 0x2564, 0x2565, 0x2559,
    0x2558, 0x2552, 0x2553, 0x256B, 0x256A, 0x2518, 0x250C, 0x2588, 0x2584, 0x258C, 0x2590, 0x2580,
    0x0440, 0x0441, 0x0442, 0x0443, 0x0444, 0x0445, 0x0446, 0x0447, 0x0448, 0x0449, 0x044A, 0x044B,
    0x044C, 0x044D, 0x044E, 0x044F, 0x0401, 0x0451, 0x0404, 0x0454, 0x0407, 0x0457, 0x040E, 0x045E,
    0x00B0, 0x2219, 0x00B7, 0x221A, 0x2116, 0x00A4, 0x25A0, 0x00A0,
];

/// Converte bytes entre páginas de código; CP 0 representa ANSI brasileiro (1252).
fn decode_dos(input: &[u8], cp: u16) -> Option<String> {
    let table: &[u16; 128] = match cp {
        437 => &CP437,
        850 => &CP850,
        852 => &CP852,
        866 => &CP866,
        _ => return None,
    };
    Some(
        input
            .iter()
            .map(|b| {
                if *b < 128 {
                    char::from(*b)
                } else {
                    char::from_u32(table[(usize::from(*b)) - 128] as u32).unwrap_or('�')
                }
            })
            .collect(),
    )
}
fn encode_dos(input: &str, cp: u16) -> Option<alloc::vec::Vec<u8>> {
    let table: &[u16; 128] = match cp {
        437 => &CP437,
        850 => &CP850,
        852 => &CP852,
        866 => &CP866,
        _ => return None,
    };
    Some(
        input
            .chars()
            .map(|c| {
                let n = c as u32;
                if n < 128 {
                    n as u8
                } else {
                    table
                        .iter()
                        .position(|x| u32::from(*x) == n)
                        .map_or(b'?', |i| (i + 128) as u8)
                }
            })
            .collect(),
    )
}

pub fn translate_string(
    input: &[u8],
    destination: u16,
    current: u16,
) -> Result<alloc::vec::Vec<u8>, CodePageError> {
    let source_cp = if current == 0 { 1252 } else { current };
    let target_cp = if destination == 0 { 1252 } else { destination };
    let decoded = if source_cp == 28591 {
        input.iter().map(|b| char::from(*b)).collect::<String>()
    } else if let Some(value) = decode_dos(input, source_cp) {
        value
    } else {
        let source =
            encoding_for_code_page(source_cp).ok_or(CodePageError::Unsupported(source_cp))?;
        source.decode(input).0.into_owned()
    };
    if target_cp == 28591 {
        return Ok(decoded
            .chars()
            .map(|c| if (c as u32) <= 255 { c as u8 } else { b'?' })
            .collect());
    }
    if let Some(value) = encode_dos(&decoded, target_cp) {
        return Ok(value);
    }
    let target = encoding_for_code_page(target_cp).ok_or(CodePageError::Unsupported(target_cp))?;
    Ok(target.encode(&decoded).0.into_owned())
}

/// ACBr trata literais legados como CP1252 e entrega texto UTF-8 em Rust.
pub fn acbr_str(input: &[u8]) -> String {
    encoding_rs::WINDOWS_1252.decode(input).0.into_owned()
}

pub fn acbr_str_to_ansi(input: &str) -> alloc::vec::Vec<u8> {
    encoding_rs::WINDOWS_1252.encode(input).0.into_owned()
}

pub fn native_string_to_utf8(input: &str) -> alloc::vec::Vec<u8> {
    input.as_bytes().into()
}
pub fn utf8_to_native_string(input: &[u8]) -> String {
    String::from_utf8_lossy(input).into_owned()
}
pub fn native_string_to_ansi(input: &str) -> alloc::vec::Vec<u8> {
    acbr_str_to_ansi(input)
}
pub fn ansi_to_native_string(input: &[u8]) -> String {
    acbr_str(input)
}
pub fn decode_to_string(input: &[u8], is_utf8: bool) -> String {
    if is_utf8 {
        utf8_to_native_string(input)
    } else {
        ansi_to_native_string(input)
    }
}
pub fn ansi_chr(byte: u8) -> u8 {
    byte
}

pub fn only_cpf_cnpj_alpha_num(input: &str) -> String {
    only_alpha_num(input).to_ascii_uppercase()
}
pub fn only_chars_in_set(input: &str, allowed: &str) -> String {
    input.chars().filter(|c| allowed.contains(*c)).collect()
}

/// Transliteration byte a byte segundo a tabela CP1252 de `TiraAcento`.
pub fn tira_acento_byte(byte: u8) -> u8 {
    match byte {
        192..=198 => b'A',
        199 => b'C',
        200..=203 => b'E',
        204..=207 => b'I',
        208 => b'D',
        209 => b'N',
        210..=214 => b'O',
        215 => b'x',
        216 | 248 => b'0',
        217..=220 => b'U',
        221 => b'Y',
        222 | 254 => b'b',
        223 => b'B',
        224..=230 => b'a',
        231 => b'c',
        232..=235 => b'e',
        236..=239 => b'i',
        240 | 242..=246 => b'o',
        247 => b'/',
        241 => b'n',
        249..=252 => b'u',
        253 | 255 => b'y',
        _ => byte,
    }
}

/// Remove linhas vazias de uma lista e retorna a lista compactada.
pub fn remove_empty_lines(lines: &mut alloc::vec::Vec<String>) {
    lines.retain(|line| !line.is_empty());
}

/// Ajusta conteúdo para largura máxima; retorno usa LF, como a função Pascal.
pub fn ajusta_linhas(text: &str, columns: usize, max_lines: usize, pad_lines: bool) -> String {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut result = alloc::string::String::new();
    let mut count = 0;
    for source in normalized.split('\n') {
        let chars: alloc::vec::Vec<char> = source.chars().collect();
        let width = columns.max(1);
        for chunk in chars.chunks(width) {
            if max_lines > 0 && count >= max_lines {
                return result;
            }
            let row: String = chunk.iter().collect();
            if pad_lines {
                result.push_str(&pad_right(&row, columns, ' '));
            } else {
                result.push_str(&row);
            }
            result.push('\n');
            count += 1;
        }
    }
    if result.is_empty() {
        if pad_lines {
            result.push_str(&space(columns));
        }
        result.push('\n');
    }
    result
}

/// Remove LF/CRLF final ou usa o padrão do sistema (LF em Rust nos alvos suportados).
pub fn remove_line_break_final_default(text: &str) -> String {
    if let Some(s) = text.strip_suffix("\r\n") {
        s.into()
    } else {
        text.strip_suffix('\n')
            .or_else(|| text.strip_suffix('\r'))
            .unwrap_or(text)
            .into()
    }
}

/// Quebra lista delimitada e substitui o conteúdo anterior, retornando quantidade de itens.
pub fn quebrar_linha(text: &str, quote: char, delimiter: char) -> alloc::vec::Vec<String> {
    add_delimited_text_to_list(text, delimiter, quote)
}

pub fn random_name_with(len: usize, next: &mut impl FnMut() -> u32) -> String {
    (0..len)
        .map(|_| char::from(b'A' + (next() % 25) as u8))
        .collect()
}

/// Alinha campos distribuindo o preenchimento pelos separadores.
pub fn pad_space_full(text: &str, len: usize, separator: &str, fill: char, trim: bool) -> String {
    if len == 0 {
        return String::new();
    }
    let mut value: String = text.chars().take(len).collect();
    if separator.is_empty() || separator == fill.to_string() {
        return pad_right(&value, len, fill);
    }
    let count = count_str(&value, separator);
    if count == 0 {
        return pad_right(&value, len, fill);
    }
    if trim {
        value = value.trim().into();
    }
    let base = value.chars().count().saturating_sub(count);
    let each = len.saturating_sub(base) / count;
    let remainder = len.saturating_sub(base + each * count);
    let mut result = String::new();
    let mut rest = remainder;
    for part in value.split(separator) {
        result.push_str(part);
        if rest > 0 {
            result.push(fill);
            rest -= 1;
        }
        result.extend(core::iter::repeat(fill).take(each));
    }
    pad_right(&result, len, fill)
}

/// Número de unidades de texto Unicode, equivalente útil a `LengthNativeString`.
pub fn length_native_string(text: &str) -> usize {
    text.chars().count()
}

pub fn left_str_native_string(text: &str, len: usize) -> String {
    left_str(text, len)
}
pub fn right_str_native_string(text: &str, len: usize) -> String {
    right_str(text, len)
}

/// `RPos` com início máximo de busca em posição 1-based (posições UTF-8 em bytes).
pub fn rpos(needle: &str, text: &str, start_pos: usize) -> usize {
    if needle.is_empty() || start_pos == 0 || start_pos > text.len() {
        return 0;
    }
    text[..start_pos.min(text.len())]
        .match_indices(needle)
        .last()
        .map_or(0, |(i, _)| i + 1)
}

pub fn poem_zeros_int(number: i64, len: usize) -> String {
    poem_zeros(&number.to_string(), len)
}

/// Remove apenas linhas vazias, mantendo linhas compostas de espaços.
pub fn remove_empty_lines_from_text(text: &str) -> String {
    text.lines()
        .filter(|line| !line.is_empty())
        .collect::<alloc::vec::Vec<_>>()
        .join("\n")
}
