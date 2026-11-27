#!/usr/bin/env python3
"""Compare ASCII filters with extracted ACBrUtil.Strings functions."""
import os, random, re, subprocess, tempfile
from pathlib import Path
root=Path(__file__).resolve().parent.parent
mirror=os.environ.get('ACBR_MIRROR_DIR','/home/ubuntu/acbr-mirror-data/trunk2')
rev='9f42f9a3375cd11e369be1ecdcc570b7dcaff814'
src=subprocess.check_output(['git','-C',mirror,'show',f'{rev}:Fontes/ACBrComum/ACBrUtil.Strings.pas']).decode('cp1252')
impl=src.split('implementation',1)[1]
funcs=[]
for name in ['CharIsAlpha','CharIsNum','CharIsAlphaNum','OnlyNumber','OnlyAlpha','OnlyAlphaNum']:
 m=re.search(r'(?ims)^function\s+'+name+r'\s*\([^;]*?\)\s*:\s*(?:Boolean|String)\s*;.*?\bbegin\b.*?^end\s*;',impl)
 if not m: raise RuntimeError(name)
 funcs.append(m.group())
header='program oracle;\n{$mode objfpc}{$H+}\nuses SysUtils;\n'
footer='''\nvar line,k,v:String; p:Integer;\nbegin\n while not EOF do begin\n  ReadLn(line); p:=Pos('|',line); k:=Copy(line,1,p-1); v:=Copy(line,p+1,Length(line));\n  if k='num' then WriteLn(OnlyNumber(v)) else if k='alpha' then WriteLn(OnlyAlpha(v)) else WriteLn(OnlyAlphaNum(v));\n end;\nend.\n'''
rng=random.Random(49422); samples=[]
for _ in range(3000):
 value=''.join(rng.choice('abcXYZ019.-_ @éç') for _ in range(rng.randrange(0,35)))
 for kind in ['num','alpha','alnum']: samples.append((kind,value))
data=''.join(k+'|'+v+'\n' for k,v in samples)
with tempfile.TemporaryDirectory(prefix='acbr-util-') as tmp:
 p=Path(tmp)/'oracle.pas'; p.write_text(header+'\n'.join(funcs)+footer,encoding='cp1252')
 subprocess.run(['fpc','-o'+str(Path(tmp)/'oracle'),str(p)],check=True)
 expected=subprocess.run([str(Path(tmp)/'oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
 subprocess.run(['cargo','build','--offline','-p','acbr-util','--example','util_oracle'],cwd=root,check=True)
 actual=subprocess.run([str(root/'target/debug/examples/util_oracle')],input=data,text=True,stdout=subprocess.PIPE,check=True).stdout.splitlines()
if len(actual)!=len(samples) or len(expected)!=len(samples): raise AssertionError('quantidade divergente')
for item,a,b in zip(samples,expected,actual):
 if a!=b: raise AssertionError(f'{item}: Pascal={a!r}, Rust={b!r}')
print(f'OK: {len(samples)} comparações com ACBrUtil.Strings r49422.')
