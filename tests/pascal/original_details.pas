program original_details;
{$mode delphi}{$H+}
uses cwstring, SysUtils, ACBrValidador;
type TObserver=class
 count:Integer; message:String;
 procedure OnError(Mensagem:String);
end;
procedure TObserver.OnError(Mensagem:String);begin Inc(count);message:=Mensagem;end;
function Hex(const S:String):String;var i:Integer;begin Result:='';for i:=1 to Length(S) do Result:=Result+IntToHex(Ord(S[i]),2);end;
var v:TACBrValidador;o:TObserver; line,kind,flags,input:String;p:Integer;valid:Boolean;
begin
 v:=TACBrValidador.Create(nil);o:=TObserver.Create;v.OnMsgErro:=o.OnError;
 while not EOF do begin
  ReadLn(line);p:=Pos('|',line);kind:=Copy(line,1,p-1);Delete(line,1,p);p:=Pos('|',line);flags:=Copy(line,1,p-1);input:=Copy(line,p+1,Length(line));
  v.IgnorarChar:='./-';v.AjustarTamanho:=Pos('A',flags)>0;v.PermiteVazio:=Pos('E',flags)>0;v.RaiseExcept:=False;v.ExibeDigitoCorreto:=Pos('S',flags)>0;v.Complemento:='';o.count:=0;o.message:='';
  if kind='cpf' then v.TipoDocto:=docCPF else if kind='cnpj' then v.TipoDocto:=docCNPJ else if kind='uf' then v.TipoDocto:=docUF
  else if kind='cheque' then v.TipoDocto:=docNumCheque else if kind='pis' then v.TipoDocto:=docPIS
  else if Copy(kind,1,3)='cep' then begin v.TipoDocto:=docCEP;v.Complemento:=Copy(kind,4,2);end
  else if kind='cartao' then v.TipoDocto:=docCartaoCredito else if kind='suframa' then v.TipoDocto:=docSuframa
  else if kind='gtin' then v.TipoDocto:=docGTIN else if kind='renavam' then v.TipoDocto:=docRenavam
  else if kind='email' then v.TipoDocto:=docEmail else if kind='cnh' then v.TipoDocto:=docCNH
  else if kind='prefixo' then v.TipoDocto:=docPrefixoGTIN else if kind='caepf' then v.TipoDocto:=docCAEPF
  else if kind='placa' then v.TipoDocto:=docPlacaMercosul else if kind='cns' then v.TipoDocto:=docCNS
  else if Copy(kind,1,2)='ie' then begin v.TipoDocto:=docInscEst;v.Complemento:=Copy(kind,3,2);end else Halt(2);
  v.Documento:=input;valid:=v.Validar;
  if valid then Write('1') else Write('0');
  WriteLn('|',Hex(v.MsgErro),'|',Hex(v.DoctoValidado),'|',Hex(v.DigitoCalculado),'|',o.count,'|',Hex(o.message));
 end;v.Free;o.Free;
end.
