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
    let mut value = if remove_spaces {
        s.trim().into()
    } else {
        s.into()
    };
    let sep = if separator.is_empty() { " " } else { separator };
    while value.chars().count() < len {
        value.push_str(sep);
    }
    pad_right(&value, len, fill)
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
    text.chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' | 'Á' | 'À' | 'Â' | 'Ã' | 'Ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' | 'É' | 'È' | 'Ê' | 'Ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' | 'Í' | 'Ì' | 'Î' | 'Ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' | 'Ó' | 'Ò' | 'Ô' | 'Õ' | 'Ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' | 'Ú' | 'Ù' | 'Û' | 'Ü' => 'u',
            'ç' | 'Ç' => 'c',
            'ñ' | 'Ñ' => 'n',
            _ => c,
        })
        .collect()
}
