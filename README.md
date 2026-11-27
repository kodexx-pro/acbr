# ACBr Rust

Port independente e experimental de funcionalidades do Projeto ACBr. A fonte original está no [SVN oficial](https://svn.code.sf.net/p/acbr/code/trunk2); este repositório inclui um [espelho não oficial](https://github.com/kodexx-pro/acbr/tree/upstream/trunk2).

## Crates e cobertura

| Crate | Recorte implementado | Referência |
| --- | --- | --- |
| `acbr-dfe` | Validação do código de UF em documentos fiscais. | `ACBrDFeUtil.pas`, SVN r49417. |
| `acbr-util` | Filtros de dígitos, letras e alfanuméricos ASCII. | `ACBrUtil.Strings.pas`, SVN r49422. |
| `acbr-math` | Recorte do módulo 11 com pesos crescentes e entrada ASCII numérica/alfanumérica. | `TACBrCalcDigito`, SVN r49422. |
| `acbr-validador` | Todos os 17 tipos de validação, incluindo inscrição estadual por UF e e-mail; formatação e calculador de dígitos. | `ACBrValidador.pas`, SVN r49422. |

O componente oferece os 17 tipos, formatadores, calculador, opções e resultados detalhados com mensagem, documento, dígito e callback. A comparação cobre a revisão fixada do ACBr; paridade integral em todas as entradas e plataformas ainda não está demonstrada. Consulte a [revisão](docs/ACBRVALIDADOR_REVIEW.md).

As crates são `no_std`; `acbr-util` e `acbr-validador` usam `alloc`. O CI executa `cargo test` e verifica formatação em Linux e Windows a cada push e pull request. A validação diferencial contra a unit Pascal completa foi executada na VPS Linux ARM64. Os resultados são experimentais, sem homologação fiscal.

## Uso

```toml
[dependencies]
acbr-validador = { git = "https://github.com/kodexx-pro/acbr.git", branch = "rust" }
```

```rust
use acbr_validador::{validar_cpf, validar_cnpj, validar_gtin, validar_cep};

assert!(validar_cpf("529.982.247-25").is_ok());
assert!(validar_cnpj("11.222.333/0001-81").is_ok());
assert!(validar_gtin("7894900011517").is_ok());
assert!(validar_cep("88000-000", "SC").is_ok());
```

As funções retornam `Result<(), ValidationError>`. A struct `Validador` permite selecionar tipo, documento, complemento, caracteres ignorados, ajuste de tamanho e vazio permitido; `validar_com_evento` recebe um callback Rust com a categoria de erro. Os valores de erro são categorias próprias de Rust, não as mensagens do ACBr. O recorte usa as opções padrão do componente: remove `.`, `/` e `-`, converte letras ASCII para maiúsculas, não completa o tamanho nem permite valor vazio. Não generalize essa API para todas as configurações possíveis do ACBr. Para CNS, a implementação rejeita explicitamente caracteres não ASCII para evitar falhas em limites UTF-8; a equivalência com a referência nesses casos não foi estabelecida.

Para obter as propriedades equivalentes a `MsgErro`, `DoctoValidado` e `DigitoCalculado`, com a opção equivalente a `ExibeDigitoCorreto`:

```rust
use acbr_validador::{TipoDocumento, Validador};

let validador = Validador {
    tipo: TipoDocumento::Cpf,
    documento: "529.982.247-24".into(),
    ..Default::default()
};
let resultado = validador.validar_detalhado(true);
assert!(!resultado.valido);
assert_eq!(resultado.documento_validado, "52998224724");
assert_eq!(resultado.digito_calculado, "25");
assert_eq!(resultado.mensagem, "CPF inválido... Dígito calculado: 25");

// O callback recebe a mensagem quando a validação falha.
let resultado = validador.validar_detalhado_com_evento(true, |mensagem| {
    eprintln!("{mensagem}");
});
assert!(resultado.is_err());
```

O callback é texto UTF-8 e a falha é retornada em `Result`, preservando o resultado detalhado; não são exceções nem eventos binários Delphi. A categoria `erro` é uma classificação Rust e não substitui a mensagem. O comportamento de codificação de textos pode depender da plataforma de origem.

Crates Git podem ser fixadas por `rev` em vez de `branch`; mantenha `Cargo.lock` no projeto consumidor.

## Testes

```bash
git clone --single-branch --branch rust https://github.com/kodexx-pro/acbr.git acbr-rust
cd acbr-rust
cargo test --workspace
```

Para comparações diferenciais, instale Python 3 e Free Pascal e aponte para um clone do espelho contendo os commits identificados em [NOTICE.md](NOTICE.md):

```bash
export ACBR_MIRROR_DIR=/caminho/para/o/clone-do-espelho
python3 scripts/verify_differential.py
python3 scripts/verify_util_differential.py
python3 scripts/verify_validador_differential.py
python3 scripts/verify_format_differential.py
python3 scripts/verify_calc_differential.py
```

Os scripts individuais compilam funções ou métodos Pascal extraídos com adaptadores. Para compilar a **unit completa**, é necessário Lazarus com as unidades de desenvolvimento, Free Pascal com FCL e os fontes originais disponíveis no espelho. Em Ubuntu 24.04, os pacotes usados na VPS são `fp-compiler`, `fp-units-fcl` e `lazarus-src-3.0` (com `--no-install-recommends`). Depois execute:

```bash
export ACBR_MIRROR_DIR=/caminho/para/o/clone-do-espelho
# Opcional se o Lazarus estiver em outro caminho:
export LAZARUS_DIR=/usr/lib/lazarus/3.0
scripts/verify_full_component.sh
```

Na VPS Linux ARM64, a unit completa coincidiu com o Rust em **130.976 validações**, **15.000 formatações** e **12.000 cálculos** e **20.210 resultados detalhados**, incluindo mensagens, documento, dígito e callback. O corpus é determinístico e não prova equivalência para toda entrada ou plataforma. A revisão detalhada está em [ACBRVALIDADOR_REVIEW.md](docs/ACBRVALIDADOR_REVIEW.md).

## Compatibilidade e origem

A função Pascal `ValidarCodigoUF` contém `91` sem o delimitador final na constante e retorna `false`; o port reproduz o comportamento, sem afirmar que seja a regra fiscal correta. CEP e prefixos GTIN reproduzem tabelas fixadas no ACBr e não consultam fontes online.

O cabeçalho original, a autoria, os caminhos e as revisões estão em [NOTICE.md](NOTICE.md). Os trechos citados declaram LGPL-2.1-or-later; consulte [LICENSE](LICENSE). Os avisos originais citam ainda origens específicas, incluindo JCL. Outras units devem ser examinadas antes de portá-las.

A adaptação automática por agentes ainda não foi implementada; atualmente o espelho SVN → Git é automático e os ports são desenvolvidos e verificados manualmente.
