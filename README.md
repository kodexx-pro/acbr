# ACBr Rust

> Port independente e experimental de funcionalidades do Projeto ACBr para Rust.

Este projeto começa com uma função real do ACBr e testes de comparação com sua implementação Pascal. A fonte oficial continua no [SVN do ACBr](https://svn.code.sf.net/p/acbr/code/trunk2); a Kodexx mantém um [espelho Git não oficial](https://github.com/kodexx-pro/acbr/tree/upstream/trunk2).

## O que está implementado

| Crate | Funcionalidade | Estado |
| --- | --- | --- |
| `acbr-dfe` | `validar_codigo_uf(i32) -> bool` | Experimental; comparada com a função Pascal fixada na revisão SVN `r49417`. |

A crate não possui dependências externas e é `no_std`. Foi compilada e testada em Linux ARM64. Isso não representa homologação fiscal nem compatibilidade de outras plataformas ou dispositivos.

## Como usar

No `Cargo.toml` da sua aplicação Rust, adicione:

```toml
[dependencies]
acbr-dfe = { git = "https://github.com/kodexx-pro/acbr.git", branch = "rust" }
```

Use a função:

```rust
use acbr_dfe::validar_codigo_uf;

assert!(validar_codigo_uf(42)); // Santa Catarina
assert!(!validar_codigo_uf(99));
```

A branch `rust` pode receber mudanças durante o desenvolvimento. Para builds reproduzíveis, fixe um commit com `rev = "<SHA>"` no lugar de `branch` e mantenha o `Cargo.lock` da aplicação.

## Como executamos os testes

Para clonar o código Rust e executar os testes locais:

```bash
git clone --single-branch --branch rust https://github.com/kodexx-pro/acbr.git acbr-rust
cd acbr-rust
cargo test --workspace
```

O teste diferencial usa Free Pascal, Python 3 e Git. Ele extrai a função original do commit do ACBr registrado em `NOTICE.md`, compila-a isoladamente e compara seus resultados com os do Rust. Informe um clone Git do espelho que contenha aquele commit:

```bash
export ACBR_MIRROR_DIR=/caminho/para/o/clone-do-espelho
python3 scripts/verify_differential.py
```

Na VPS usada para o desenvolvimento, o caminho padrão já é `/home/ubuntu/acbr-mirror-data/trunk2`. Na primeira execução, o script compila também o exemplo Rust usado na comparação. O teste extrai a função original, não compila a unit ACBr completa.

## Um detalhe da referência

A constante Pascal contém o código `91` sem o delimitador final exigido pela busca; por isso a função de referência retorna `false` para esse valor. O port acompanha esse comportamento para que a diferença não passe despercebida. Isso não é uma afirmação sobre a regra fiscal correta.

## Próximos passos

Ampliar os recortes portados e a cobertura diferencial, validar as regras fiscais com fontes oficiais e criar uma esteira para identificar alterações upstream. A adaptação automática por agentes ainda não foi implementada.

## Origem e licença

A autoria do código de referência pertence aos colaboradores do Projeto ACBr. Este port usa a licença **LGPL-2.1-or-later** declarada no arquivo original. Veja [NOTICE.md](NOTICE.md), que preserva o cabeçalho, a origem e o commit de referência, e [LICENSE](LICENSE).
