// SPDX-License-Identifier: LGPL-2.1-or-later
//! Calculador de dígitos baseado em `TACBrCalcDigito` do ACBr.
use alloc::string::{String, ToString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormulaDigito {
    Modulo11,
    Modulo10Pis,
    Modulo10,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculoError {
    Overflow,
}
#[derive(Debug, Clone)]
pub struct CalculadorDigito {
    pub documento: String,
    pub multiplicador_inicial: i32,
    pub multiplicador_final: i32,
    pub multiplicador_atual: i32,
    pub formula: FormulaDigito,
    pub digito_final: i32,
    pub soma_digitos: i32,
    pub modulo_final: i32,
}
impl Default for CalculadorDigito {
    fn default() -> Self {
        Self {
            documento: String::new(),
            multiplicador_inicial: 2,
            multiplicador_final: 9,
            multiplicador_atual: 0,
            formula: FormulaDigito::Modulo11,
            digito_final: 0,
            soma_digitos: 0,
            modulo_final: 0,
        }
    }
}
impl CalculadorDigito {
    pub fn calculo_padrao(&mut self) {
        self.multiplicador_inicial = 2;
        self.multiplicador_final = 9;
        self.multiplicador_atual = 0;
        self.formula = FormulaDigito::Modulo11;
    }
    pub fn calcular(&mut self) -> Result<(), CalculoError> {
        self.soma_digitos = 0;
        self.digito_final = 0;
        self.modulo_final = 0;
        let ini = self.multiplicador_inicial;
        let fim = self.multiplicador_final;
        let mut base = if self.multiplicador_atual >= ini && self.multiplicador_atual <= fim {
            self.multiplicador_atual
        } else {
            ini
        };
        for &b in self.documento.as_bytes().iter().rev() {
            let mut valor = (i32::from(b) - 48)
                .checked_mul(base)
                .ok_or(CalculoError::Overflow)?;
            if self.formula == FormulaDigito::Modulo10 && valor > 9 {
                let d = valor.to_string();
                let bytes = d.as_bytes();
                valor = i32::from(bytes[0] - b'0') + i32::from(bytes[1] - b'0');
            }
            self.soma_digitos = self
                .soma_digitos
                .checked_add(valor)
                .ok_or(CalculoError::Overflow)?;
            if ini > fim {
                base = base.checked_sub(1).ok_or(CalculoError::Overflow)?;
                if base < fim {
                    base = ini
                }
            } else {
                base = base.checked_add(1).ok_or(CalculoError::Overflow)?;
                if base > fim {
                    base = ini
                }
            }
        }
        let modulo = if self.formula == FormulaDigito::Modulo10 {
            10
        } else {
            11
        };
        self.modulo_final = self.soma_digitos % modulo;
        self.digito_final = match self.formula {
            FormulaDigito::Modulo11 => {
                if self.modulo_final < 2 {
                    0
                } else {
                    11 - self.modulo_final
                }
            }
            FormulaDigito::Modulo10Pis => {
                let d = 11 - self.modulo_final;
                if d >= 10 {
                    0
                } else {
                    d
                }
            }
            FormulaDigito::Modulo10 => {
                let d = 10 - self.modulo_final;
                if d >= 10 {
                    0
                } else {
                    d
                }
            }
        };
        Ok(())
    }
}
