#!/usr/bin/env python3
"""Build an isolated Pascal harness from the original ACBr methods and compare."""
import os
import random
import re
import subprocess
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MIRROR = os.environ.get('ACBR_MIRROR_DIR', '/home/ubuntu/acbr-mirror-data/trunk2')
COMMIT = '9f42f9a3375cd11e369be1ecdcc570b7dcaff814'
source = subprocess.check_output(['git', '-C', MIRROR, 'show', f'{COMMIT}:Fontes/ACBrDiversos/ACBrValidador.pas']).decode('cp1252')

def extract(start, next_marker):
    start_pos = source.lower().find(start.lower(), source.lower().find('implementation'))
    if start_pos < 0: raise RuntimeError('Não encontrou ' + start)
    end_pos = source.lower().find(next_marker.lower(), start_pos + len(start))
    if end_pos < 0: raise RuntimeError('Não encontrou fim de ' + start)
    return source[start_pos:end_pos]

methods = '\n'.join([
    extract('Procedure TACBrValidador.ValidarCheque', 'Procedure TACBrValidador.ValidarCNPJ'),
    extract('procedure TACBrValidador.ValidarEmail', 'Procedure TACBrValidador.ValidarCEP'),
    extract('Procedure TACBrValidador.ValidarCEP', 'Procedure TACBrValidador.ValidarIE'),
    extract('Procedure TACBrValidador.ValidarCNPJ', 'Procedure TACBrValidador.ValidarCPF'),
    extract('Procedure TACBrValidador.ValidarCPF', 'procedure TACBrValidador.ValidarEmail'),
    extract('procedure TACBrValidador.ValidarPrefixoGTIN', 'procedure TACBrValidador.ValidarRenavam'),
    extract('Procedure TACBrValidador.ValidarIE', 'Procedure TACBrValidador.ValidarUF'),
    extract('procedure TACBrValidador.ValidarCNS', 'procedure TACBrValidador.ValidarPrefixoGTIN'),
    extract('procedure TACBrValidador.ValidarGTIN', 'procedure TACBrValidador.ValidarCAEPF'),
    extract('Procedure TACBrValidador.ValidarUF', 'procedure TACBrValidador.ValidarCNS'),
    extract('procedure TACBrValidador.ValidarRenavam', 'procedure TACBrValidador.ValidarGTIN'),
    extract('procedure TACBrValidador.ValidarCAEPF', '{ Rotina extraida do site:'),
    extract('Procedure TACBrValidador.ValidarCartaoCredito', '// Rotina para Valida'),
    extract('procedure TACBrValidador.ValidarCNH', '{------------------------------ TACBrCalcDigito'),
    extract('procedure TACBrCalcDigito.Calcular', 'procedure TACBrCalcDigito.CalculoPadrao'),
    extract('procedure TACBrCalcDigito.CalculoPadrao', '(*{$IfDef HAS_REGEXPR}')
])
header = r'''program oracle;
{$mode objfpc}{$H+}
uses cwstring, SysUtils, StrUtils, Math, Variants, Classes;
const cUFsValidas=',AC,AL,AP,AM,BA,CE,DF,ES,GO,MA,MT,MS,MG,PA,PB,PR,PE,PI,RJ,RN,RS,RO,RR,SC,SP,SE,TO,EX,';
type TACBrCalcDigFormula = (frModulo11, frModulo10PIS, frModulo10);
TACBrCalcDigito = class
  fsMultIni, fsMultFim, fsMultAtu, fsDigitoFinal, fsSomaDigitos, fsModuloFinal: Integer;
  fsFormulaDigito: TACBrCalcDigFormula;
  fsDocto: String;
  procedure Calcular; procedure CalculoPadrao;
  property Documento: String read fsDocto write fsDocto;
  property MultiplicadorInicial: Integer read fsMultIni write fsMultIni;
  property MultiplicadorFinal: Integer read fsMultFim write fsMultFim;
  property FormulaDigito: TACBrCalcDigFormula read fsFormulaDigito write fsFormulaDigito;
  property DigitoFinal: Integer read fsDigitoFinal;
  property ModuloFinal: Integer read fsModuloFinal;
  property SomaDigitos: Integer read fsSomaDigitos;
end;
TACBrValidador = class
  fsAjustarTamanho, fsExibeDigitoCorreto: Boolean;
  fsDocto, fsMsgErro, fsDigitoCalculado, fsComplemento, fsDocumento: String;
  Modulo: TACBrCalcDigito;
  property Documento: String read fsDocumento write fsDocumento;
  procedure ValidarEmail; procedure ValidarIE; procedure ValidarPrefixoGTIN; procedure ValidarCNS; procedure ValidarCheque; procedure ValidarCEP; procedure ValidarCPF; procedure ValidarCNPJ; procedure ValidarGTIN; procedure ValidarUF(const UF:String); procedure ValidarPIS; procedure ValidarPlaca; procedure ValidarRenavam; procedure ValidarSuframa; procedure ValidarCAEPF; procedure ValidarCartaoCredito; procedure ValidarCNH;
end;
function CharIsNum(c: Char): Boolean; begin Result:=c in ['0'..'9']; end;
function ACBrStr(s: String): String; begin Result:=s; end;
function NaoEstaVazio(s: String): Boolean; begin Result:=s<>''; end;
function StrIsNumber(s: String): Boolean;
var c: Char;
begin Result := Length(s)>0; for c in s do if not (c in ['0'..'9']) then Exit(False); end;
function StrIsAlphaNum(s: String): Boolean;
var c: Char;
begin Result := Length(s)>0; for c in s do if not (c in ['0'..'9','A'..'Z','a'..'z']) then Exit(False); end;
function PadLeft(s: String; n: Integer; c: Char = ' '): String;
begin if Length(s)<n then Result:=StringOfChar(c,n-Length(s))+s else Result:=Copy(s,1,n); end;
'''
footer = r'''
var v:TACBrValidador; line, kind, input:String; i,p,j:Integer; sl:TStringList; delim:Char;
begin
 v:=TACBrValidador.Create; v.Modulo:=TACBrCalcDigito.Create;
 while not EOF do begin
  ReadLn(line); p:=Pos('|',line); kind:=Copy(line,1,p-1); input:=Copy(line,p+1,Length(line)); v.fsDocumento:=input;
  v.fsDocto:=''; for i:=1 to Length(input) do if ((kind='cpfX') or ((kind='cpfZ') and (input[i]<>'*')) or ((kind<>'cpfX') and (kind<>'cpfZ') and not (input[i] in ['.','/','-']))) then v.fsDocto:=v.fsDocto+UpperCase(input[i]);
  v.fsAjustarTamanho:=(Copy(kind,1,2)='ia') or (kind='cpfA') or (kind='cnpjA') or (kind='pisA') or (kind='renavamA') or (kind='caepfA') or (kind='cepA') or (kind='cnsA');
  v.fsDocto:=Trim(v.fsDocto); if kind='email' then v.fsDocto:=Trim(input); v.fsMsgErro:=''; v.fsDigitoCalculado:='';
  v.fsComplemento:=''; if (Copy(kind,1,2)='ie') or (Copy(kind,1,2)='ia') then v.fsComplemento:=Copy(kind,3,2) else if kind='cepSC' then v.fsComplemento:='SC' else if kind='cepsc' then v.fsComplemento:='sc';
  if kind='emailList' then begin
    delim:=' '; if Pos(';',input)>0 then delim:=';' else if Pos(',',input)>0 then delim:=',' else if Pos('|',input)>0 then delim:='|';
    sl:=TStringList.Create; sl.Delimiter:=delim; sl.StrictDelimiter:=True; sl.QuoteChar:='"'; if delim=' ' then sl.Add(input) else sl.DelimitedText:=input;
    j:=0; while (j<sl.Count) and (v.fsMsgErro='') do begin v.fsDocumento:=sl[j]; if Trim(sl[j])='' then v.fsMsgErro:='empty' else v.ValidarEmail; Inc(j); end; sl.Free;
  end
  else if (v.fsDocto='') and (kind<>'cpfE') then v.fsMsgErro:='empty' else if (v.fsDocto='') and (kind='cpfE') then v.fsMsgErro:=''
  else if (kind='cpf') or (kind='cpfA') or (kind='cpfE') or (kind='cpfX') or (kind='cpfZ') then v.ValidarCPF else if (kind='cnpj') or (kind='cnpjA') then v.ValidarCNPJ else if kind='prefixo' then v.ValidarPrefixoGTIN else if kind='gtin' then v.ValidarGTIN else if kind='uf' then v.ValidarUF(v.fsDocto) else if kind='placa' then v.ValidarPlaca else if (kind='pis') or (kind='pisA') then v.ValidarPIS else if (kind='renavam') or (kind='renavamA') then v.ValidarRenavam else if kind='suframa' then v.ValidarSuframa else if (kind='caepf') or (kind='caepfA') then v.ValidarCAEPF else if kind='cartao' then v.ValidarCartaoCredito else if kind='email' then v.ValidarEmail else if (Copy(kind,1,2)='ie') or (Copy(kind,1,2)='ia') then v.ValidarIE else if (kind='cns') or (kind='cnsA') then v.ValidarCNS else if kind='cnh' then v.ValidarCNH else if kind='cheque' then v.ValidarCheque else if (kind='cep') or (kind='cepSC') or (kind='cepsc') or (kind='cepA') then v.ValidarCEP else Halt(2);
  if v.fsMsgErro='' then WriteLn('1') else WriteLn('0');
 end;
 v.Modulo.Free; v.Free;
end.
'''
rng=random.Random(49422)
samples=[('cpf','529.982.247-25'),('cpf','11111111111'),('cpf',''),('cnpj','12.ABC.345/01DE-35'),('cnpj','00000000000000'),('gtin','7894900011517')]
for kind, lengths, alphabet in [('cpf',[0,10,11,12],'0123456789'),('cnpj',[0,13,14,15],'0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ'),('gtin',[0,7,8,12,13,14,15],'0123456789')]:
 for _ in range(2000):
  value=''.join(rng.choice(alphabet) for _ in range(rng.choice(lengths)))
  samples.append((kind,value))
# Exercise valid, invalid and decorated inputs with deterministic generation.
for _ in range(1500):
 for kind,size in [('cpf',9),('cnpj',12),('gtin',12)]:
  prefix=''.join(rng.choice('0123456789' if kind!='cnpj' else '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ') for _ in range(size))
  if kind=='gtin':
   digit=(10-sum(int(x)*(3 if i%2==0 else 1) for i,x in enumerate(reversed(prefix)))%10)%10
   value=prefix+str(digit)
  else:
   def dv(s,maxweight):
    total=sum((ord(c)-48)*(2+i%(maxweight-1)) for i,c in enumerate(reversed(s)))%11
    return 0 if total<2 else 11-total
   value=prefix+str(dv(prefix,11 if kind=='cpf' else 9))
   value+=str(dv(value,11 if kind=='cpf' else 9))
  samples.extend([(kind,value),(kind,value[:-1]+str((int(value[-1])+1)%10))])
samples.extend((kind,f'{value:08d}') for kind in ['cep','cepSC','cepsc'] for value in [0,999999,1000000,19999999,20000000,76799999,76800000,76999999,77000000,78900000,88000000,89999999,90000000,99999999])
samples.extend(('emailList',v) for v in ['a@abc.com;b@def.com', 'a@abc.com; b@def.com', 'a@abc.com;', 'a@abc.com,,b@def.com', '"a@abc.com";"b@def.com"', 'aa"b@abc.com;ccc@dd.com', 'a@abc.com;b@def.com,ccc@dd.com'])
for _ in range(2500):
 parts=[rng.choice(['a@abc.com','b@def.com',' bad@a.com','a@bc.com','x','"abc@xyz.com"','abc"x@def.com','']) for _ in range(rng.randrange(1,5))]
 samples.append(('emailList',rng.choice([';',',','|']).join(parts)))
samples.extend(('email',v) for v in ['a@ab.com','aaa@bbbb','test@example.com',' a@ab.com','aa..x@ab.com','aa@a..com','x@y.zz','a@abc.com'])
for _ in range(4000):
 samples.append(('email',''.join(rng.choice('abcd0123.@_ -+!') for _ in range(rng.randrange(0,25)))))
for _ in range(5000):
 samples.append(('cns', rng.choice('012789') + ''.join(rng.choice('0123456789') for _ in range(14))))
for kind in ['cep','cepSC','cepsc','cheque']:
 for _ in range(3000):
  samples.append((kind,''.join(rng.choice('0123456789') for _ in range(rng.choice([0,7,8,9,11])))))
samples.extend([('uf',x) for x in ['SC','SP','EX','zz',' sc ','','BRA']])
for kind,lengths,alphabet in [('placa',[0,6,7,8],'ABCXYZ0123456789'),('pis',[0,10,11,12],'0123456789'),('renavam',[0,10,11,12],'0123456789'),('suframa',[0,8,9,10],'0123456789'),('caepf',[0,13,14,15],'0123456789'),('cartao',[0,1,13,16,19],'0123456789'),('cnh',[0,10,11,12],'0123456789')]:
 for _ in range(3000):
  samples.append((kind,''.join(rng.choice(alphabet) for _ in range(rng.choice(lengths)))))
for uf in ['AC','AL','AP','AM','BA','CE','DF','ES','GO','MA','MT','MS','MG','PA','PB','PR','PE','PI','RJ','RN','RS','RO','RR','SC','SP','SE','TO','EX']:
 for _ in range(1800):
  length=rng.choice([8,9,10,11,12,13,14])
  prefix={'AC':'01','AL':'24','AP':'03','CE':'0','DF':'07','GO':'10','MA':'12','MS':'28','PA':'15','PB':'16','PI':'19','RN':'20','RR':'24','TO':'29'}.get(uf,'')
  value=(prefix+''.join(rng.choice('0123456789') for _ in range(length)))[:length]
  if uf=='SP' and rng.randrange(5)==0: value='P'+value[1:]
  samples.append(('ie'+uf,value))
 samples.extend([('ie'+uf,'ISENTO'),('ie'+uf,''),('ie'+uf,'0')])
samples.extend(('prefixo',v) for k,v in samples if k=='gtin')
samples.extend((kind, value) for kind in ['cpfE','cpfX','cpfZ'] for value in ['', ' ', '529.982.247-25', '52998224725', '529*982*247*25', '...', '00000000000'])
for kind in ['cpfA','cnpjA','pisA','renavamA','cepA','caepfA','cnsA']:
 for _ in range(1500):
  samples.append((kind,''.join(rng.choice('0123456789') for _ in range(rng.choice([0,5,8,9,10,11,12,13,14,15,16])))))
for uf in ['AC','AL','AP','AM','BA','CE','DF','ES','GO','MA','MT','MS','MG','PA','PB','PR','PE','PI','RJ','RN','RS','RO','RR','SC','SP','SE','TO']:
 for _ in range(200):
  samples.append(('ia'+uf,''.join(rng.choice('0123456789') for _ in range(rng.choice([2,7,8,9,10,11,12,13,14,15])))))
data=''.join(k+'|'+v+'\n' for k,v in samples)
with tempfile.TemporaryDirectory(prefix='acbr-validator-') as tmp:
 p=Path(tmp)/'oracle.pas'; p.write_text(header+methods+footer,encoding='cp1252')
 subprocess.run(['fpc','-v0','-o'+str(Path(tmp)/'oracle'),str(p)],check=True)
 expected=subprocess.run([str(Path(tmp)/'oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
 subprocess.run(['cargo','build','--offline','-p','acbr-validador','--example','validador_oracle'],cwd=ROOT,check=True)
 actual=subprocess.run([str(ROOT/'target/debug/examples/validador_oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
full_bin=os.environ.get('ACBR_FULL_ORACLE')
if full_bin:
 full=subprocess.run([full_bin],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
 if len(full)!=len(samples): raise AssertionError(f'unit completa retornou {len(full)} em vez de {len(samples)}')
 for item,a,b in zip(samples,full,actual):
  if a!=b: raise AssertionError(f'UNIT COMPLETA {item}: Pascal={a}, Rust={b}')
 print(f'OK: {len(samples)} comparações contra a unit ACBrValidador completa.')
if len(expected)!=len(samples) or len(actual)!=len(samples): raise AssertionError(f'lengths {len(expected)} {len(actual)} {len(samples)}')
from collections import Counter
ie_valid = Counter(k[2:] for (k,v),result in zip(samples,expected) if k.startswith('ie') and v not in ('ISENTO','') and result=='1')
print('Inscrições válidas por UF:', dict(sorted(ie_valid.items())))
for item,a,b in zip(samples,expected,actual):
 if a!=b: raise AssertionError(f'{item}: Pascal={a}, Rust={b}')
print(f'OK: {len(samples)} entradas; métodos Pascal extraídos da revisão r49422 coincidem com Rust.')
