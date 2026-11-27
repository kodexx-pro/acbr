# Revisão do port de ACBrValidador

Referência: `Fontes/ACBrDiversos/ACBrValidador.pas`, SVN **r49422**, commit `9f42f9a3375cd11e369be1ecdcc570b7dcaff814`. Escopo: comportamento da revisão fixada, não homologação das regras fiscais.

## Inventário de tipos

Os 17 tipos de `TACBrValTipoDocto` têm uma função Rust de validação: CPF, CNPJ, UF, inscrição estadual, cheque, PIS, CEP, cartão de crédito, SUFRAMA, GTIN, RENAVAM, e-mail, CNH, prefixo GTIN, CAEPF, placa e CNS. O método `Validador::validar` seleciona pelo enum `TipoDocumento`.

A inscrição estadual inclui as 27 UFs, EX e ISENTO conforme a referência. Os testes incluem entradas aceitas e rejeitadas em cada uma das 27 UFs; isso não prova cobertura de todas as faixas, regras históricas ou combinações de opções.

## Outras superfícies

- Implementadas funções de formatação de CPF, CNPJ, CAEPF, CPF/CNPJ, placa, IE/máscara por UF, cheque, PIS, CEP, SUFRAMA, telefone e máscaras numérica/dinâmica/remoção.
- Implementados seleção de tipo, documento, complemento, caracteres ignorados, ajuste de tamanho e permitir vazio. O teste diferencial de opções cobre amostras desses casos; não esgota todas as combinações nem caracteres Unicode.
- Implementado `CalculadorDigito` com os três modos e pesos iniciais/finais/atual. Ele retorna erro de overflow em vez de reproduzir comportamento indefinido ou dependente do compilador.
- `validar_com_evento` chama um callback Rust com `ValidationError`; `Result` substitui o fluxo comum de exceção. **Ainda não reproduz `MsgErro` e `OnMsgErro` com as mensagens textuais originais**, `ExibeDigitoCorreto`, nem as propriedades `DigitoCalculado` e `DoctoValidado` do componente. Os valores de `ValidationError` são categorias, não códigos/mensagens originais.
- Não há paridade de ciclo de vida/integração visual do `TACBrComponent`, eventos Delphi ou compatibilidade binária. O comportamento Unicode pode variar em relação às configurações do compilador Pascal; CNS rejeita explicitamente não ASCII.

## Evidência

- `cargo test --workspace` e `cargo fmt --all --check`.
- `scripts/verify_full_component.sh`: compila a unit Pascal original com dependências reais e compara 130.976 entradas de validação, 15.000 de formatação e 12.000 de cálculo, usando corpus determinísticos. A verificação de validação compara sucesso/falha, não textos de erro, dígito calculado ou eventos.
- `verify_util_differential.py`: 9.000 comparações dos filtros de texto com funções Pascal extraídas.
- Testes executados em Linux ARM64 com Rust 1.75 e Free Pascal 3.2.2; outros sistemas e arquiteturas ainda não foram testados.

## Pendências antes de declarar paridade integral

1. Implementar e comparar mensagens textuais por tipo e opção, callback e exceções, `ExibeDigitoCorreto`, `DigitoCalculado` e `DoctoValidado`.
2. Ampliar testes da API configurável, formatos, Unicode e valores de borda, inclusive para todas as variantes históricas de inscrição estadual.
3. Testar nos demais alvos e obter revisão especializada das regras fiscais aplicáveis.
4. Automatizar a revisão de mudanças upstream e atualizar a referência quando necessário. A transposição por agentes ainda não é automática.
