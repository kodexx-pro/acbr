#!/usr/bin/env python3
"""Compare mensagens, documento, dígito e evento com a unit Pascal original."""
import os, random, subprocess, sys
randomizer = random.Random(993)
states = 'AC AL AP AM BA CE DF ES GO MA MT MS MG PA PB PR PE PI RJ RN RS RO RR SC SP SE TO'.split()
kinds = 'cpf cnpj uf cheque pis cep cepSC cartao suframa gtin renavam email cnh prefixo caepf placa cns ie ieEX ieZZ'.split() + ['ie'+s for s in states]
cases=[]
for kind in kinds:
    for i in range(400):
        size=randomizer.randrange(19)
        alphabet='0123456789' if i % 4 else '0123456789ABC./-'
        value=''.join(randomizer.choice(alphabet) for _ in range(size))
        flags=''.join(x for x in 'AES' if randomizer.randrange(2))
        cases.append(f'{kind}|{flags}|{value}')
    for value in ('', 'ISENTO', '00000000000', '52998224725', '123456789', 'a@b.com'):
        for flags in ('','A','E','S','AES'):
            cases.append(f'{kind}|{flags}|{value}')
subprocess.run(['cargo','build','--offline','--quiet','-p','acbr-validador','--example','details_oracle'],check=True)
data='\n'.join(cases)+'\n'
def run(command):
    return subprocess.run([command],input=data,text=True,capture_output=True,check=True).stdout.splitlines()
original=run(os.environ['ACBR_DETAILS_ORACLE'])
ported=run('target/debug/examples/details_oracle')
if len(original)!=len(cases) or len(ported)!=len(cases):
    sys.exit('Número inesperado de resultados')
for case,expected,actual in zip(cases,original,ported):
    fields=expected.split('|')
    # OnMsgErro é CP1252 no FPC testado; Rust entrega texto Unicode ao callback.
    fields[5]=bytes.fromhex(fields[5]).decode('cp1252').encode().hex().upper()
    expected='|'.join(fields)
    if expected != actual:
        sys.exit(f'Divergência: {case!r}\nPascal: {expected}\nRust:   {actual}')
print(f'{len(cases)} comparações de detalhes: OK')
