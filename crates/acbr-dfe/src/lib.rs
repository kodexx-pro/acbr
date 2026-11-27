// SPDX-License-Identifier: LGPL-2.1-or-later
//! Funções de documentos fiscais portadas do Projeto ACBr.
#![no_std]

/// Reproduz `ValidarCodigoUF` de `ACBrDFeUtil.pas` na revisão registrada.
///
/// O valor 91 retorna `false` nesta versão para corresponder exatamente ao
/// comportamento da implementação de referência, inclusive seu delimitador final.
/// Não use esta função como fonte independente de regras oficiais de SEFAZ.
///
/// ```
/// use acbr_dfe::validar_codigo_uf;
/// assert!(validar_codigo_uf(42));
/// assert!(!validar_codigo_uf(99));
/// ```
pub const fn validar_codigo_uf(codigo: i32) -> bool {
    matches!(
        codigo,
        12 | 27
            | 16
            | 13
            | 29
            | 23
            | 53
            | 32
            | 52
            | 21
            | 51
            | 50
            | 31
            | 15
            | 25
            | 41
            | 26
            | 22
            | 33
            | 24
            | 43
            | 11
            | 14
            | 42
            | 35
            | 28
            | 17
            | 90
    )
}

#[cfg(test)]
mod tests {
    use super::validar_codigo_uf;

    #[test]
    fn accepted_reference_codes() {
        for n in [
            12, 27, 16, 13, 29, 23, 53, 32, 52, 21, 51, 50, 31, 15, 25, 41, 26, 22, 33, 24, 43, 11,
            14, 42, 35, 28, 17, 90,
        ] {
            assert!(validar_codigo_uf(n), "código {n}");
        }
    }

    #[test]
    fn invalid_and_reference_edge_cases() {
        for n in [i32::MIN, -1, 0, 1, 10, 36, 89, 91, 99, i32::MAX] {
            assert!(!validar_codigo_uf(n), "código {n}");
        }
    }
}
