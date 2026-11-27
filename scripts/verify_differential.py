#!/usr/bin/env python3
"""Compare the Rust function with the function extracted from a pinned ACBr source."""
import os
from pathlib import Path
import random
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parent.parent
MIRROR = Path(os.environ.get("ACBR_MIRROR_DIR", "/home/ubuntu/acbr-mirror-data/trunk2"))
COMMIT = "ff76164f6413d0341988042be1bbfc33a2d126dd"
SOURCE = "Fontes/ACBrDFe/ACBrDFeUtil.pas"


def run(command, **kwargs):
    return subprocess.run(command, check=True, **kwargs)


def main():
    source = run(["git", "-C", str(MIRROR), "show", f"{COMMIT}:{SOURCE}"],
                 stdout=subprocess.PIPE).stdout.decode("latin-1")
    implementation = source.split("implementation", 1)[1]
    match = re.search(r"(?is)function\s+ValidarCodigoUF\s*\(.*?\)\s*:\s*boolean\s*;.*?\bbegin\b.*?\bend\s*;", implementation)
    if not match:
        raise RuntimeError("Não foi possível localizar a implementação Pascal fixada")
    body = match.group(0)
    pascal = """program acbr_uf_oracle;
{$mode objfpc}{$H+}
uses SysUtils;
""" + body + """
var code: LongInt;
begin
  while not EOF do begin
    ReadLn(code);
    if ValidarCodigoUF(code) then WriteLn('1') else WriteLn('0');
  end;
end.
"""
    rng = random.Random(20261127)
    samples = list(range(-1000, 1001)) + [-2147483648, 2147483647]
    samples += [rng.randint(-2147483648, 2147483647) for _ in range(1000)]
    data = "".join(f"{n}\n" for n in samples)
    with tempfile.TemporaryDirectory(prefix="acbr-uf-") as tmp:
        path = Path(tmp) / "oracle.pas"
        path.write_text(pascal, encoding="latin-1")
        run(["fpc", "-o" + str(Path(tmp) / "oracle"), str(path)],
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        expected = run([str(Path(tmp) / "oracle")], input=data, text=True,
                       stdout=subprocess.PIPE).stdout.splitlines()
        run(["cargo", "build", "--offline", "-p", "acbr-dfe", "--example", "oracle_input"], cwd=ROOT)
        actual = run([str(ROOT / "target/debug/examples/oracle_input")], input=data,
                     text=True, stdout=subprocess.PIPE).stdout.splitlines()
    if len(expected) != len(samples) or len(actual) != len(samples):
        raise AssertionError(f"quantidade de resultados: {len(expected)} / {len(actual)} / {len(samples)}")
    for n, pas, rust in zip(samples, expected, actual):
        if pas != rust:
            raise AssertionError(f"divergência para {n}: Pascal={pas}, Rust={rust}")
    print(f"OK: {len(samples)} entradas; Rust coincide com a função Pascal extraída de {COMMIT[:12]}.")


if __name__ == "__main__":
    main()
