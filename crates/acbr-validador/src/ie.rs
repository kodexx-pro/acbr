// SPDX-License-Identifier: LGPL-2.1-or-later
//! Port do algoritmo e das tabelas de `ValidarIE` do ACBr SVN r49422.
use super::{normalize, ValidationError};
use alloc::{format, string::String, vec::Vec};

const W: [[i32; 14]; 14] = [
    [0, 2, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5, 6],
    [0, 0, 2, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5],
    [2, 0, 3, 4, 5, 6, 7, 8, 9, 2, 3, 4, 5, 6],
    [0, 2, 3, 4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0],
    [0, 2, 3, 4, 5, 6, 7, 0, 0, 8, 9, 0, 0, 0],
    [0, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5],
    [0, 2, 3, 4, 5, 6, 7, 2, 3, 4, 5, 6, 7, 8],
    [0, 0, 2, 3, 4, 5, 6, 7, 2, 3, 4, 5, 6, 7],
    [0, 0, 2, 1, 2, 1, 2, 1, 2, 1, 1, 2, 1, 0],
    [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 2, 3, 0],
    [0, 0, 0, 0, 10, 8, 7, 6, 5, 4, 3, 1, 0, 0],
    [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 3, 0, 0],
    [0, 0, 2, 3, 4, 5, 6, 7, 8, 3, 4, 5, 6, 7],
];

#[derive(Clone)]
struct Config {
    len: usize,
    p: Vec<&'static str>,
    xr: &'static str,
    xm: i32,
    xt: usize,
    yr: &'static str,
    ym: i32,
    yt: usize,
    f: i32,
    g: i32,
}
impl Config {
    fn new(len: usize, p: &[&'static str]) -> Self {
        Self {
            len,
            p: p.into(),
            xr: "E",
            xm: 11,
            xt: 1,
            yr: "",
            ym: 0,
            yt: 0,
            f: 0,
            g: 0,
        }
    }
    fn x(mut self, rot: &'static str, m: i32, t: usize) -> Self {
        self.xr = rot;
        self.xm = m;
        self.xt = t;
        self
    }
    fn y(mut self, rot: &'static str, m: i32, t: usize) -> Self {
        self.yr = rot;
        self.ym = m;
        self.yt = t;
        self
    }
    fn f(mut self, f: i32) -> Self {
        self.f = f;
        self
    }
    fn g(mut self, g: i32) -> Self {
        self.g = g;
        self
    }
}
const D: &str = "0-9";
const X: &str = "DVX";
const Y: &str = "DVY";
/// Valida inscrição estadual conforme o ACBr na revisão r49422.
/// `uf` corresponde ao campo Complemento Pascal (sensível a maiúsculas).
pub fn validar_ie(input: &str, uf: &str) -> Result<(), ValidationError> {
    validar_ie_com_ajuste(input, uf, false)
}
pub(crate) fn validar_ie_com_ajuste(
    input: &str,
    uf: &str,
    ajustar: bool,
) -> Result<(), ValidationError> {
    let mut n = normalize(input);
    if n.is_empty() {
        return Err(ValidationError::Empty);
    }
    if n == "ISENTO" {
        return Ok(());
    }
    if !matches!(
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
    ) {
        return Err(ValidationError::InvalidCharacter);
    }
    if !n.is_ascii()
        || n.len() < 2
        || !n.as_bytes()[1..].iter().all(u8::is_ascii_digit)
        || !(n.as_bytes()[0].is_ascii_digit() || n.as_bytes()[0] == b'P')
    {
        return Err(ValidationError::InvalidCharacter);
    }
    let z = |s: &str, k: usize| -> String { format!("{:0>width$}", s, width = k) };
    let c = match uf {
        "AC" if n.len() == 9 => Config::new(9, &[X, D, D, D, D, D, D, "1", "0"]),
        "AC" => Config::new(13, &[Y, X, D, D, D, D, D, D, D, D, D, "1", "0"])
            .x("E", 11, 2)
            .y("E", 11, 1),
        "AL" => Config::new(9, &[X, D, D, D, D, D, D, "4", "2"]).x("BD", 11, 1),
        "AP" => {
            let mut a = Config::new(9, &[X, D, D, D, D, D, D, "3", "0"]).x("CE", 11, 1);
            if n.as_str() >= "030170010" && n.as_str() <= "030190229" {
                a.f = 1
            } else if n.as_str() >= "030190230" {
                a.xr = "E"
            }
            a
        }
        "AM" | "ES" | "SC" | "SE" => Config::new(9, &[X, D, D, D, D, D, D, D, D]),
        "BA" => {
            if n.len() < 9 {
                n = z(&n, 9)
            }
            let m = if b"0123458".contains(&n.as_bytes()[1]) {
                10
            } else {
                11
            };
            Config::new(9, &[X, Y, D, D, D, D, D, D, D])
                .x("E", m, 2)
                .y("E", m, 3)
        }
        "CE" => Config::new(9, &[X, D, D, D, D, D, D, D, "0"]),
        "DF" => Config::new(13, &[Y, X, D, D, D, D, D, D, D, D, D, "7,8", "0"])
            .x("E", 11, 2)
            .y("E", 11, 1),
        "GO" => {
            let prefix = n.get(..2).and_then(|v| v.parse::<u8>().ok()).unwrap_or(0);
            if !(prefix == 10 || prefix == 11 || (20..=29).contains(&prefix)) {
                return Err(ValidationError::InvalidCharacter);
            }
            let g = i32::from(n.as_str() >= "101031050" && n.as_str() <= "101199979");
            Config::new(9, &[X, D, D, D, D, D, D, D, "1,2"]).g(g)
        }
        "MA" => Config::new(9, &[X, D, D, D, D, D, D, "2", "1"]),
        "MT" => {
            if n.len() == 9 {
                n = z(&n, 11)
            }
            Config::new(11, &[X, D, D, D, D, D, D, D, D, D, D])
        }
        "MS" => Config::new(9, &[X, D, D, D, D, D, D, "0,8", "2,5"]),
        "MG" => Config::new(13, &[Y, X, D, D, D, D, D, D, D, D, D, D, D])
            .x("AE", 10, 10)
            .y("E", 11, 11),
        "PA" => Config::new(9, &[X, D, D, D, D, D, D, "5,6,7,8,9", "1,7"]),
        "PB" => Config::new(9, &[X, D, D, D, D, D, D, "6", "1"]),
        "PR" => Config::new(10, &[Y, X, D, D, D, D, D, D, D, D])
            .x("E", 11, 9)
            .y("E", 11, 8),
        "PE" if n.len() == 14 => {
            Config::new(14, &[X, D, D, D, D, D, D, D, D, D, D, "1-9", "8", "1"])
                .x("E", 11, 7)
                .f(1)
        }
        "PE" => Config::new(9, &[Y, X, D, D, D, D, D, D, D])
            .x("E", 11, 14)
            .y("E", 11, 7),
        "PI" => Config::new(9, &[X, D, D, D, D, D, D, "9", "1"]),
        "RJ" => Config::new(8, &[X, D, D, D, D, D, D, "1,7,8,9"]).x("E", 11, 8),
        "RN" if n.len() == 9 => Config::new(9, &[X, D, D, D, D, D, D, "0", "2"]).x("BD", 11, 1),
        "RN" => Config::new(10, &[X, D, D, D, D, D, D, D, "0", "2"]).x("BD", 11, 11),
        "RS" => Config::new(10, &[X, D, D, D, D, D, D, D, D, D]),
        "RO" if n.len() == 9 => Config::new(9, &[X, D, D, D, D, D, D, D, "1-9"])
            .x("E", 11, 4)
            .f(1),
        "RO" => Config::new(14, &[X, D, D, D, D, D, D, D, D, D, D, D, D, D]).f(1),
        "RR" => Config::new(9, &[X, D, D, D, D, D, D, "4", "2"]).x("D", 9, 5),
        "SP" if n.starts_with('P') => {
            Config::new(13, &[D, D, D, X, D, D, D, D, D, D, D, D, "P"]).x("D", 11, 12)
        }
        "SP" => Config::new(12, &[Y, D, D, X, D, D, D, D, D, D, D, D])
            .x("D", 11, 12)
            .y("D", 11, 13),
        "TO" if n.len() == 11 => {
            Config::new(11, &[X, D, D, D, D, D, D, "1,2,3,9", "0,9", "9", "2"]).x("E", 11, 6)
        }
        "TO" => Config::new(9, &[X, D, D, D, D, D, D, D, D]),
        _ => return Err(ValidationError::InvalidLength),
    };
    if ajustar {
        n = z(&n, c.len).chars().take(c.len).collect();
    }
    if n.len() != c.len {
        return Err(ValidationError::InvalidLength);
    }
    let mut digits = [b' '; 14];
    digits[14 - n.len()..].copy_from_slice(n.as_bytes());
    let mut dx = 0i32;
    let mut dy = 0i32;
    for (pos, &ch) in digits.iter().enumerate() {
        let idx = 13 - pos;
        let pat = c.p.get(idx).copied().unwrap_or("");
        let ok = match pat {
            "" => ch == b' ',
            "DVX" => {
                dx = i32::from(ch.saturating_sub(b'0'));
                ch.is_ascii_digit()
            }
            "DVY" => {
                dy = i32::from(ch.saturating_sub(b'0'));
                ch.is_ascii_digit()
            }
            "0-9" => ch.is_ascii_digit(),
            "1-9" => (b'1'..=b'9').contains(&ch),
            _ => pat.as_bytes().contains(&ch),
        };
        if !ok {
            return Err(ValidationError::InvalidCharacter);
        }
    }
    for (rot, md, tp, expected) in [(c.xr, c.xm, c.xt, dx), (c.yr, c.ym, c.yt, dy)] {
        if tp == 0 {
            break;
        }
        let mut sum = 0;
        let mut extra = 0;
        for (pos, &ch) in digits.iter().enumerate() {
            if ch.is_ascii_digit() {
                let m = i32::from(ch - b'0') * W[tp - 1][13 - pos];
                sum += m;
                extra += m / 10;
            }
        }
        if rot.contains('A') {
            sum += extra
        } else if rot.contains('B') {
            sum *= 10
        } else if rot.contains('C') {
            sum += 5 + 4 * c.f
        }
        let mut dv = sum % md;
        if rot.contains('E') {
            dv = md - dv
        }
        if dv == 10 {
            dv = c.g
        } else if dv == 11 {
            dv = c.f
        }
        if dv != expected {
            return Err(ValidationError::InvalidCheckDigit);
        }
    }
    Ok(())
}
