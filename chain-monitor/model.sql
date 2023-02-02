set colsep ';'
--set headsep off
set pagesize 50000
set trimspool on
set linesize 19999
--set numwidth 5
set feedback off
set heading on
set underline off

spool &1

ALTER SESSION SET NLS_DATE_FORMAT = 'DD/MM/YYYY HH24:MI:SS';
SELECT * FROM ics_x_c_job_exec
WHERE trunc(inicio) >= SYSDATE -2 ORDER BY inicio DESC;

spool off
