select * from ics_x_c_job_exec
where 
--status in ('OK','ERRO','EXEC')
--and cadeia = 'CA'
trunc(inicio) >= sysdate-2
order by inicio desc
-- DMS
-- CAMPANHA
