# ACBr Rust — primeiro port experimental

Primeiro recorte do ACBr para Rust: `ValidarCodigoUF`, da unit `ACBrDFeUtil`.
A crate `acbr-dfe` é independente de runtime e de bibliotecas externas (`no_std`).
Ela compara o comportamento com a função Pascal original extraída do commit fixado em `NOTICE.md`.

```rust
assert!(acbr_dfe::validar_codigo_uf(42));
assert!(!acbr_dfe::validar_codigo_uf(99));
```

Execute `cargo test --workspace` e `python3 scripts/verify_differential.py`. O teste diferencial precisa de Git, Free Pascal e do espelho git-svn na VPS; o caminho pode ser mudado com `ACBR_MIRROR_DIR`.

**Compatibilidade:** por fidelidade à referência, `91` retorna `false`: o delimitador final está ausente na constante Pascal. Não significa que essa seja a regra fiscal correta. A função é uma prova de conceito, não valida documentos fiscais completos, nem hardware, nem equivale ao ACBr inteiro.

Consulte [NOTICE.md](NOTICE.md) para origem, autoria e licença LGPL-2.1-or-later.
