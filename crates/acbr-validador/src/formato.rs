// SPDX-License-Identifier: LGPL-2.1-or-later
//! Formatação do ACBrValidador. Máscaras operam em caracteres Unicode;
//! para equivalência binária Delphi/FPC, utilize dados ASCII.
use acbr_util::{only_alpha_num, only_number};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
fn trim(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_ascii() && c <= ' ')
}
fn pad_left(s: &str, n: usize, pad: char) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() >= n {
        chars[..n].iter().collect()
    } else {
        core::iter::repeat(pad)
            .take(n - chars.len())
            .chain(chars)
            .collect()
    }
}
fn part(s: &str, start: usize, len: usize) -> String {
    s.chars().skip(start).take(len).collect()
}

pub fn formatar_cpf(value: &str) -> String {
    let s = pad_left(&only_number(value), 11, '0');
    format!(
        "{}.{}.{}-{}",
        part(&s, 0, 3),
        part(&s, 3, 3),
        part(&s, 6, 3),
        part(&s, 9, 2)
    )
}
pub fn formatar_cnpj(value: &str) -> String {
    let s = pad_left(&only_alpha_num(value), 14, '0');
    format!(
        "{}.{}.{}/{}-{}",
        part(&s, 0, 2),
        part(&s, 2, 3),
        part(&s, 5, 3),
        part(&s, 8, 4),
        part(&s, 12, 2)
    )
}
pub fn formatar_caepf(value: &str) -> String {
    let s = pad_left(&only_number(value), 14, '0');
    format!(
        "{}.{}.{}/{}-{}",
        part(&s, 0, 3),
        part(&s, 3, 3),
        part(&s, 6, 3),
        part(&s, 9, 3),
        part(&s, 12, 2)
    )
}
pub fn formatar_cnpj_ou_cpf(value: &str) -> String {
    let s = only_alpha_num(value);
    match s.len() {
        11 => formatar_cpf(&s),
        14 => formatar_cnpj(&s),
        _ => value.into(),
    }
}
pub fn formatar_placa(value: &str) -> String {
    let s = trim(value).to_ascii_uppercase();
    format!("{}-{}", part(&s, 0, 3), part(&s, 3, 4))
}
pub fn formatar_cheque(value: &str) -> String {
    let s = pad_left(trim(value), 7, '0');
    format!("{}-{}", part(&s, 0, 6), part(&s, 6, 1))
}
pub fn formatar_pis(value: &str) -> String {
    let s = pad_left(trim(value), 11, '0');
    format!(
        "{}.{}.{}.{}",
        part(&s, 0, 2),
        part(&s, 2, 5),
        part(&s, 7, 3),
        part(&s, 10, 1)
    )
}
pub fn formatar_cep(value: &str) -> String {
    let mut s = only_number(value);
    if s.len() < 5 {
        s = pad_left(&s, 5, '0')
    }
    if s.len() == 5 {
        while s.len() < 8 {
            s.push('0')
        }
    } else {
        s = pad_left(&s, 8, '0')
    }
    format!("{}-{}", part(&s, 0, 5), part(&s, 5, 3))
}
pub fn formatar_suframa(value: &str) -> String {
    value.into()
}

/// Preenche estrelas da direita para a esquerda, com zeros à esquerda.
pub fn formatar_mascara_numerica(value: &str, mask: &str) -> String {
    let v: Vec<char> = trim(value).chars().collect();
    let mut i = v.len();
    let mut out = Vec::new();
    for c in mask.chars().rev() {
        if c == '*' {
            if i > 0 {
                i -= 1;
                out.push(v[i])
            } else {
                out.push('0')
            }
        } else {
            out.push(c)
        }
    }
    out.into_iter().rev().collect()
}
pub fn formatar_mascara_dinamica(value: &str, mask: &str) -> String {
    let v: Vec<char> = trim(value).chars().collect();
    let mut i = 0;
    let mut out = String::new();
    for c in mask.chars() {
        if i >= v.len() {
            break;
        }
        if c == '*' {
            out.push(v[i]);
            i += 1
        } else {
            out.push(c)
        }
    }
    out
}
pub fn remover_mascara(value: &str, mask: &str) -> String {
    trim(value)
        .chars()
        .zip(mask.chars())
        .filter_map(|(v, m)| if m == '*' || m != v { Some(v) } else { None })
        .collect()
}

pub fn mascara_ie(value: &str, uf: &str) -> String {
    let uf = uf.to_ascii_uppercase();
    let len = value.chars().count();
    let mask = match uf.as_str() {
        "AC" => "**.***.***/***-**",
        "AL" | "AP" => "*********",
        "AM" => "**.***.***-*",
        "BA" => "*******-**",
        "CE" => "********-*",
        "DF" => "***********-**",
        "ES" => "*********",
        "GO" => "**.***.***-*",
        "MA" => "*********",
        "MT" => "**********-*",
        "MS" => "**.***.***-*",
        "MG" => "***.***.***/****",
        "PA" => "**-******-*",
        "PB" => "********-*",
        "PR" => "***.*****-**",
        "PE" => {
            if len > 9 {
                "**.*.***.*******-*"
            } else {
                "*******-**"
            }
        }
        "PI" => "*********",
        "RJ" => "**.***.**-*",
        "RN" => {
            if len > 9 {
                "**.*.***.***-*"
            } else {
                "**.***.***-*"
            }
        }
        "RS" => "***/*******",
        "RO" => "*************-*",
        "RR" => "*********",
        "SC" => "***.***.***",
        "SP" => {
            if len > 1 && value.starts_with('P') {
                "*-********.*/***"
            } else {
                "***.***.***.***"
            }
        }
        "SE" => "**.***.***-*",
        "TO" => {
            if len == 11 {
                "**.**.******-*"
            } else {
                "**.***.***-*"
            }
        }
        _ => return core::iter::repeat('*').take(len).collect(),
    };
    mask.to_string()
}
pub fn formatar_ie(value: &str, uf: &str) -> String {
    if trim(value).eq_ignore_ascii_case("ISENTO") {
        return value.into();
    }
    formatar_mascara_numerica(&only_alpha_num(value), &mascara_ie(value, uf))
}

pub fn formatar_fone(value: &str, ddd: &str) -> String {
    let mut s = only_number(value);
    let zero = s.starts_with('0');
    s = s.trim_start_matches('0').into();
    if s.is_empty() || s == "0" {
        return String::new();
    }
    let mut len = s.len();
    if len <= 9 && !ddd.is_empty() {
        s = format!("{}{}", part(ddd, 0, 2), s);
        len += 2
    }
    if len > 12 {
        s = format!(
            "{}{}",
            part(&s, 0, 2),
            part(&s, 2, s.len()).trim_start_matches('0')
        );
        len = s.len()
    }
    let mask = match len {
        9 => "*****-****",
        10 => {
            if zero && part(&s, 1, 2) == "00" {
                "****-***-****"
            } else {
                "(**)****-****"
            }
        }
        11 => "(**)*****-****",
        12 => "+**(**)****-****",
        _ => {
            if len > 12 {
                "+**(**)*****-****"
            } else {
                "****-****"
            }
        }
    };
    formatar_mascara_numerica(&s, mask)
}
