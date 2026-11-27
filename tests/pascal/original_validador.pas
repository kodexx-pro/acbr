program original_validador;
{$mode delphi}{$H+}
uses cwstring, SysUtils, ACBrValidador;
var v:TACBrValidador; line,kind,input:String; p:Integer; valid:Boolean;
begin
 v:=TACBrValidador.Create(nil);
 while not EOF do begin
  ReadLn(line);p:=Pos('|',line);kind:=Copy(line,1,p-1);input:=Copy(line,p+1,Length(line));
  v.IgnorarChar:='./-';v.AjustarTamanho:=False;v.PermiteVazio:=False;v.RaiseExcept:=False;v.ExibeDigitoCorreto:=False;v.Complemento:='';
  if kind='emailList' then valid:=ACBrValidador.ValidarEmail(input)=''
  else begin
   if (kind='cpf') or (kind='cpfA') or (kind='cpfE') or (kind='cpfX') or (kind='cpfZ') then v.TipoDocto:=docCPF
   else if (kind='cnpj') or (kind='cnpjA') then v.TipoDocto:=docCNPJ
   else if kind='prefixo' then v.TipoDocto:=docPrefixoGTIN
   else if kind='gtin' then v.TipoDocto:=docGTIN
   else if kind='uf' then v.TipoDocto:=docUF
   else if kind='placa' then v.TipoDocto:=docPlacaMercosul
   else if (kind='pis') or (kind='pisA') then v.TipoDocto:=docPIS
   else if (kind='renavam') or (kind='renavamA') then v.TipoDocto:=docRenavam
   else if kind='suframa' then v.TipoDocto:=docSuframa
   else if (kind='caepf') or (kind='caepfA') then v.TipoDocto:=docCAEPF
   else if kind='cartao' then v.TipoDocto:=docCartaoCredito
   else if kind='email' then v.TipoDocto:=docEmail
   else if (Copy(kind,1,2)='ie') or (Copy(kind,1,2)='ia') then begin v.TipoDocto:=docInscEst;v.Complemento:=Copy(kind,3,2);end
   else if (kind='cns') or (kind='cnsA') then v.TipoDocto:=docCNS
   else if kind='cnh' then v.TipoDocto:=docCNH
   else if kind='cheque' then v.TipoDocto:=docNumCheque
   else if (kind='cep') or (kind='cepSC') or (kind='cepsc') or (kind='cepA') then begin v.TipoDocto:=docCEP;if kind='cepSC' then v.Complemento:='SC' else if kind='cepsc' then v.Complemento:='sc';end
   else Halt(2);
   v.AjustarTamanho:=(Copy(kind,1,2)='ia') or (kind='cpfA') or (kind='cnpjA') or (kind='pisA') or (kind='renavamA') or (kind='caepfA') or (kind='cepA') or (kind='cnsA');
   if kind='cpfE' then v.PermiteVazio:=True else if kind='cpfX' then v.IgnorarChar:='' else if kind='cpfZ' then v.IgnorarChar:='*'; v.Documento:=input; valid:=v.Validar;
  end;
  if valid then WriteLn('1') else WriteLn('0');
 end;
 v.Free;
end.
