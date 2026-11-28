// SPDX-License-Identifier: LGPL-2.1-or-later
//! Filtros ASCII de `ACBrUtil.Strings` (SVN r49422).
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
    c.is_ascii_alphanumeric() && c.is_ascii() || matches!(c, '+' | '/')
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
