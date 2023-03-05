set colsep ';'
--set headsep off
set pagesize 50000
set trimspool on
set linesize 19999
--set numwidth 5
set feedback off
set heading on
--set underline off

spool &1

ALTER SESSION SET NLS_DATE_FORMAT = 'DD/MM/YYYY';
SELECT SYSDATE,
trunc(arq_dt_vencimento) as vencimento,
CASE
WHEN status_processamento is not null
THEN DECODE(status_processamento,
1, 'Carregada',
2, 'Aguardando Processamento',
3, 'Em Processamento',
4, 'Processado com suscesso',
5, 'Processado com Falha')
ELSE DECODE(status_fatura,
1, 'Aguardando Fatura',
2, 'Fatura Não Encontrada',
3, 'Fatura com Falha',
4, 'Fatura Vencida',
5, 'Download da Fatura Realizado',
6, 'Aguardando Fatura Resumida')
END as status,
CASE
WHEN LOWER(arq_email) not like '%@contaonline-whatsapp.oi.com.br%'
THEN 'E-MAIL'
ELSE 'WHATSAPP'
END as tipo,
CASE
WHEN nome_arquivo_saida like '%ARBOR%'
THEN 'Movel'
WHEN nome_arquivo_saida like '%SISRAF%'
THEN 'Fixa R1'
WHEN nome_arquivo_saida like '%SFA%'
THEN 'Fixa R2'
WHEN nome_arquivo_saida like '%TV%'
THEN 'TV'
END as produto,
COUNT(1) as quantidade
FROM ged360bd.vw_cnt_ems_dados_notificacao
WHERE arq_dt_vencimento BETWEEN SYSDATE AND SYSDATE + 15 -- PERIODO DE VENCIMENTO
AND data_processamento_brscan BETWEEN SYSDATE - 30 AND SYSDATE + 15 -- PERIODO DE PROCESSAMENTO
--AND trunc(arq_dt_vencimento) = to_date('16/09/2022') -- DATA DE VENCIMENTO
--AND nome_arquivo_saida like '%ARBOR%' -- PRODUTO
--AND LOWER(arq_email) like '%@contaonline-whatsapp.oi.com.br%' -- TIPO DE PROCESSO
AND status_fatura in (1,6) -- STATUS
GROUP BY
trunc(arq_dt_vencimento),
status_processamento,
CASE
WHEN LOWER(arq_email) not like '%@contaonline-whatsapp.oi.com.br%'
THEN 'E-MAIL'
ELSE 'WHATSAPP'
END,
status_fatura,
CASE
WHEN nome_arquivo_saida like '%ARBOR%'
THEN 'Movel'
WHEN nome_arquivo_saida like '%SISRAF%'
THEN 'Fixa R1'
WHEN nome_arquivo_saida like '%SFA%'
THEN 'Fixa R2'
WHEN nome_arquivo_saida like '%TV%'
THEN 'TV'
END
ORDER BY vencimento, produto, tipo, status;

spool off
