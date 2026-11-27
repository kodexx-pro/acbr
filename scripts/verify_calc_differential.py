#!/usr/bin/env python3
import os,random,subprocess,tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parent.parent
MIRROR=os.environ.get('ACBR_MIRROR_DIR','/home/ubuntu/acbr-mirror-data/trunk2')
REV='9f42f9a3375cd11e369be1ecdcc570b7dcaff814'
src=subprocess.check_output(['git','-C',MIRROR,'show',f'{REV}:Fontes/ACBrDiversos/ACBrValidador.pas']).decode('cp1252')
a=src.lower().index('procedure tacbrcalcdigito.calcular;',src.lower().index('implementation'))
b=src.lower().index('procedure tacbrcalcdigito.calculopadrao;',a)
methods=src[a:b]
header='''program oracle;
{$mode objfpc}{$H+}
uses SysUtils, Classes;
type TACBrCalcDigFormula = (frModulo11, frModulo10PIS, frModulo10);
TACBrCalcDigito = class
 fsMultIni,fsMultFim,fsMultAtu,fsDigitoFinal,fsSomaDigitos,fsModuloFinal:Integer;
 fsFormulaDigito:TACBrCalcDigFormula;fsDocto:String;
 procedure Calcular;
end;
'''
footer='''
var c:TACBrCalcDigito;line:String;p:TStringList;
begin
 c:=TACBrCalcDigito.Create;p:=TStringList.Create;p.StrictDelimiter:=True;p.Delimiter:='|';
 while not EOF do begin ReadLn(line);p.DelimitedText:=line;
 c.fsFormulaDigito:=TACBrCalcDigFormula(StrToInt(p[0]));c.fsMultIni:=StrToInt(p[1]);c.fsMultFim:=StrToInt(p[2]);c.fsMultAtu:=StrToInt(p[3]);c.fsDocto:=p[4];c.Calcular;
 WriteLn(c.fsSomaDigitos,'|',c.fsModuloFinal,'|',c.fsDigitoFinal);
 end;p.Free;c.Free;
end.
'''
rng=random.Random(49422);cases=[]
for _ in range(12000):
 ini=rng.randrange(1,16);fim=rng.randrange(1,16);atual=rng.randrange(0,17);formula=rng.randrange(3)
 doc=''.join(rng.choice('0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ') for _ in range(rng.randrange(1,22)))
 cases.append((str(formula),str(ini),str(fim),str(atual),doc))
data=''.join('|'.join(c)+'\n' for c in cases)
with tempfile.TemporaryDirectory(prefix='acbr-calc-') as tmp:
 p=Path(tmp)/'oracle.pas';p.write_text(header+methods+footer,encoding='cp1252')
 subprocess.run(['fpc','-v0','-o'+str(Path(tmp)/'oracle'),str(p)],check=True)
 a=subprocess.run([str(Path(tmp)/'oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
 subprocess.run(['cargo','build','--offline','-p','acbr-validador','--example','calc_oracle'],cwd=ROOT,check=True)
 b=subprocess.run([str(ROOT/'target/debug/examples/calc_oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
full_bin=os.environ.get('ACBR_CALC_ORACLE')
if full_bin:
 full=subprocess.run([full_bin],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
 if len(full)!=len(cases):raise AssertionError(f'unit completa: {len(full)} resultados')
 for case,x,y in zip(cases,full,b):
  if x!=y:raise AssertionError(f'UNIT COMPLETA {case}: Pascal={x}, Rust={y}')
 print(f'OK: {len(cases)} comparações de cálculo com a unit completa.')
if len(a)!=len(cases) or len(b)!=len(cases):raise AssertionError('tamanho de resultado')
for case,x,y in zip(cases,a,b):
 if x!=y:raise AssertionError(f'{case}: Pascal={x} Rust={y}')
print(f'OK: {len(cases)} comparações de cálculo com ACBr r49422.')
