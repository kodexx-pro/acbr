program original_calc;
{$mode delphi}{$H+}
uses cwstring, SysUtils, Classes, ACBrValidador;
var c:TACBrCalcDigito;line:String;p:TStringList;
begin
 c:=TACBrCalcDigito.Create;p:=TStringList.Create;p.StrictDelimiter:=True;p.Delimiter:='|';
 while not EOF do begin ReadLn(line);p.DelimitedText:=line;
 c.FormulaDigito:=TACBrCalcDigFormula(StrToInt(p[0]));c.MultiplicadorInicial:=StrToInt(p[1]);c.MultiplicadorFinal:=StrToInt(p[2]);c.MultiplicadorAtual:=StrToInt(p[3]);c.Documento:=p[4];c.Calcular;
 WriteLn(c.SomaDigitos,'|',c.ModuloFinal,'|',c.DigitoFinal);
 end;p.Free;c.Free;
end.
