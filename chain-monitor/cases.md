# CASES DE MONITORAÇÃO

### Inicio de uma JOB/Cadeia

Fazer job anterior ao monitorado acionar o job-monitor quando ele for finalizado passando como parâmetro o `nome do job monitorado`.
_OBS: job-monitor aguardaria 1s para executar `ps -ef` e confirmar execução._

> O monitor iria acionar o bot do telegram com a hora de inicio do JOB.

### Fim de um JOB/Cadeia

Fazer o próprio job acionar o job-monitor quando terminar passando como parâmetro seu `próprio nome` e `código de saída` (exit 0).

> O monitor iria acionar o bot do telegram com a hora de término do JOB.

### Erro em um JOB

Fazer o próprio job acionar o job-monitor passando como parâmetro seu `próprio nome` e `código de saída` (exit 1).
_Similar ao que estamos fazendo com a cadeia BCV._

> O monitor iria acionar o bot do telegram com a hora do erro e nome do job.

### Alerta de um JOB (exit 2, por exemplo)

Fazer o próprio job acionar o job-monitor passado o seu `próprio nome`, `código de saída` e `motivo`.
_por exemplo: `Não há faixas suficientes` da cadeia CAMP AUTO_
