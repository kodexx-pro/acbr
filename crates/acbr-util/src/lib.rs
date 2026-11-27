// SPDX-License-Identifier: LGPL-2.1-or-later
//! Filtros ASCII de `ACBrUtil.Strings` (SVN r49422).
#![no_std]
extern crate alloc;
use alloc::string::String;

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
