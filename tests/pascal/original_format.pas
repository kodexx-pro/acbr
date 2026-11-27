program original_format;
{$mode delphi}{$H+}
uses cwstring, SysUtils, ACBrValidador;
var line,k,v,a:String; p:Integer;
begin
 while not EOF do begin
 ReadLn(line);p:=Pos('|',line);k:=Copy(line,1,p-1);Delete(line,1,p);p:=Pos('|',line);v:=Copy(line,1,p-1);a:=Copy(line,p+1,Length(line));
 if k='cpf' then WriteLn(FormatarCPF(v)) else if k='cnpj' then WriteLn(FormatarCNPJ(v)) else if k='caepf' then WriteLn(FormatarCAEPF(v)) else if k='cpfcnpj' then WriteLn(FormatarCNPJouCPF(v)) else if k='placa' then WriteLn(FormatarPlaca(v)) else if k='cheque' then WriteLn(FormatarCheque(v)) else if k='pis' then WriteLn(FormatarPIS(v)) else if k='cep' then WriteLn(FormatarCEP(v)) else if k='suframa' then WriteLn(FormatarSUFRAMA(v)) else if k='mask' then WriteLn(FormatarMascaraNumerica(v,a)) else if k='dyn' then WriteLn(FormatarMascaraDinamica(v,a)) else if k='remove' then WriteLn(RemoverMascara(v,a)) else if k='ie' then WriteLn(FormatarIE(v,a)) else if k='maskie' then WriteLn(MascaraIE(v,a)) else if k='fone' then WriteLn(FormatarFone(v,a)) else Halt(2);
 end;
end.
