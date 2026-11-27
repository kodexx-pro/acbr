#!/usr/bin/env python3
"""Compare ASCII formatting with extracted ACBrValidador methods."""
import os,random,re,subprocess,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
MIRROR=os.environ.get('ACBR_MIRROR_DIR','/home/ubuntu/acbr-mirror-data/trunk2')
REV='9f42f9a3375cd11e369be1ecdcc570b7dcaff814'
src=subprocess.check_output(['git','-C',MIRROR,'show',f'{REV}:Fontes/ACBrDiversos/ACBrValidador.pas']).decode('cp1252')
start=src.lower().index('function formatarfone(',src.lower().index('implementation'))
end=src.lower().index('function formatardocumento(',start)
block=src[start:end]
header='''program oracle;
{$mode objfpc}{$H+}
uses SysUtils, StrUtils, Math;
function OnlyNumber(s:String):String;var c:Char;begin Result:='';for c in s do if c in ['0'..'9'] then Result:=Result+c;end;
function OnlyAlphaNum(s:String):String;var c:Char;begin Result:='';for c in s do if c in ['0'..'9','A'..'Z','a'..'z'] then Result:=Result+c;end;
function PadLeft(s:String;n:Integer;c:Char=' '):String;begin if Length(s)<n then Result:=StringOfChar(c,n-Length(s))+s else Result:=Copy(s,1,n);end;
function PadRight(s:String;n:Integer;c:Char=' '):String;begin if Length(s)<n then Result:=s+StringOfChar(c,n-Length(s)) else Result:=Copy(s,1,n);end;
function RemoveZerosEsquerda(s:String):String;var i:Integer;begin i:=1;while (i<Length(s)) and (s[i]='0') do Inc(i);Result:=Copy(s,i,Length(s));end;
function NaoEstaVazio(s:String):Boolean;begin Result:=s<>'';end;
function FormatarMascaraNumerica(const ANumValue: string; const Mascara: String): String; forward;
'''
footer='''
var line,k,v,a:String; p:Integer;
begin
 while not EOF do begin
 ReadLn(line);p:=Pos('|',line);k:=Copy(line,1,p-1);Delete(line,1,p);p:=Pos('|',line);v:=Copy(line,1,p-1);a:=Copy(line,p+1,Length(line));
 if k='cpf' then WriteLn(FormatarCPF(v)) else if k='cnpj' then WriteLn(FormatarCNPJ(v)) else if k='caepf' then WriteLn(FormatarCAEPF(v)) else if k='cpfcnpj' then WriteLn(FormatarCNPJouCPF(v)) else if k='placa' then WriteLn(FormatarPlaca(v)) else if k='cheque' then WriteLn(FormatarCheque(v)) else if k='pis' then WriteLn(FormatarPIS(v)) else if k='cep' then WriteLn(FormatarCEP(v)) else if k='suframa' then WriteLn(FormatarSUFRAMA(v)) else if k='mask' then WriteLn(FormatarMascaraNumerica(v,a)) else if k='dyn' then WriteLn(FormatarMascaraDinamica(v,a)) else if k='remove' then WriteLn(RemoverMascara(v,a)) else if k='ie' then WriteLn(FormatarIE(v,a)) else if k='maskie' then WriteLn(MascaraIE(v,a)) else if k='fone' then WriteLn(FormatarFone(v,a)) else Halt(2);
 end;
end.
'''
rng=random.Random(49422); samples=[]
for kind in ['cpf','cnpj','caepf','cpfcnpj','placa','cheque','pis','cep','suframa','fone','ie','maskie','mask','dyn','remove']:
 for _ in range(1000):
  val=''.join(rng.choice('0123456789ABCDEF.-/ ') for _ in range(rng.randrange(0,22)))
  if kind in ['ie','maskie']: arg=rng.choice(['AC','SC','SP','PE','RN','TO','XX'])
  elif kind in ['mask','dyn','remove']: arg=rng.choice(['***.***-**','(**)****-****','', '**************','A**'])
  elif kind=='fone': arg=rng.choice(['','49','11'])
  else: arg=''
  samples.append((kind,val,arg))
data=''.join('|'.join(x)+'\n' for x in samples)
with tempfile.TemporaryDirectory(prefix='acbr-format-') as tmp:
 p=Path(tmp)/'oracle.pas';p.write_text(header+block+footer,encoding='cp1252')
 subprocess.run(['fpc','-v0','-o'+str(Path(tmp)/'oracle'),str(p)],check=True)
 exp=subprocess.run([str(Path(tmp)/'oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
 subprocess.run(['cargo','build','--offline','-p','acbr-validador','--example','format_oracle'],cwd=ROOT,check=True)
 got=subprocess.run([str(ROOT/'target/debug/examples/format_oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
full_bin=os.environ.get('ACBR_FORMAT_ORACLE')
if full_bin:
 full=subprocess.run([full_bin],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
 if len(full)!=len(samples):raise AssertionError(f'unit completa: {len(full)} resultados')
 for item,a,b in zip(samples,full,got):
  if a!=b:raise AssertionError(f'UNIT COMPLETA {item}: Pascal={a!r}, Rust={b!r}')
 print(f'OK: {len(samples)} comparações de formatação com a unit completa.')
if len(exp)!=len(samples) or len(got)!=len(samples):raise AssertionError(f'quantidade {len(exp)} {len(got)}')
for x,a,b in zip(samples,exp,got):
 if a!=b:raise AssertionError(f'{x}: Pascal={a!r}, Rust={b!r}')
print(f'OK: {len(samples)} comparações de formatação com ACBr r49422.')
