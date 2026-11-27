# Revisão do port de ACBrValidador

Referência: `Fontes/ACBrDiversos/ACBrValidador.pas`, SVN **r49422**, commit `9f42f9a3375cd11e369be1ecdcc570b7dcaff814`. Escopo: comportamento da revisão fixada, não homologação das regras fiscais.

## Inventário de tipos

Os 17 tipos de `TACBrValTipoDocto` têm uma função Rust de validação: CPF, CNPJ, UF, inscrição estadual, cheque, PIS, CEP, cartão de crédito, SUFRAMA, GTIN, RENAVAM, e-mail, CNH, prefixo GTIN, CAEPF, placa e CNS. O método `Validador::validar` seleciona pelo enum `TipoDocumento`.

A inscrição estadual inclui as 27 UFs, EX e ISENTO conforme a referência. Os testes incluem entradas aceitas e rejeitadas em cada uma das 27 UFs; isso não prova cobertura de todas as faixas, regras históricas ou combinações de opções.

## Outras superfícies

- Implementadas funções de formatação de CPF, CNPJ, CAEPF, CPF/CNPJ, placa, IE/máscara por UF, cheque, PIS, CEP, SUFRAMA, telefone e máscaras numérica/dinâmica/remoção.
- Implementados seleção de tipo, documento, complemento, caracteres ignorados, ajuste de tamanho e permitir vazio. O teste diferencial de opções cobre amostras desses casos; não esgota todas as combinações nem caracteres Unicode.
- Implementado `CalculadorDigito` com os três modos e pesos iniciais/finais/atual. Ele retorna erro de overflow em vez de reproduzir comportamento indefinido ou dependente do compilador.
- `validar_com_evento` entrega categoria Rust. `validar_detalhado` retorna mensagem, documento validado e dígito calculado; o argumento controla a exibição do dígito. `validar_detalhado_com_evento` entrega a mensagem ao callback e retorna `Err` em caso de falha. Isso oferece equivalentes de uso para as propriedades e evento Delphi, sem reproduzir a ABI ou exceções Delphi. A categoria Rust `erro` não é um código Delphi.
- Não há paridade de ciclo de vida/integração visual do `TACBrComponent`, eventos Delphi ou compatibilidade binária. O comportamento Unicode pode variar em relação às configurações do compilador Pascal; CNS rejeita explicitamente não ASCII.

## Evidência

- `cargo test --workspace` e `cargo fmt --all --check`.
- `scripts/verify_full_component.sh`: compila a unit Pascal original com dependências reais e compara 130.976 entradas de validação, 15.000 de formatação e 12.000 de cálculo e 20.210 resultados detalhados, incluindo mensagens, documento, dígito e callback, usando corpus determinísticos e entradas predominantemente ASCII.
- `verify_util_differential.py`: 9.000 comparações dos filtros de texto com funções Pascal extraídas.
- Testes executados em Linux ARM64 com Rust 1.75 e Free Pascal 3.2.2; outros sistemas e arquiteturas ainda não foram testados.

## Limites e próximos passos

1. Os testes ampliados não demonstram equivalência para todas as entradas, combinações de opções, Unicode ou variantes históricas das inscrições estaduais. Em particular, o comportamento de codificação pode variar entre compiladores; CNS rejeita caracteres não ASCII.
2. A execução comparativa foi realizada em Linux ARM64. Outros sistemas e arquiteturas ainda precisam ser verificados, assim como as regras fiscais aplicáveis por especialistas.
3. A atualização automática do espelho SVN não porta automaticamente as alterações para Rust nem revisa a referência fixada. A transposição por agentes ainda não é automática.
4. Interfaces visuais e o ciclo de vida do componente Delphi permanecem fora do escopo da API Rust.
