#!/bin/bash
set +x

# Caminho absoluto da pasta do projeto / use pwd
PROJECT_FOLDER="/apps/raidc/script/operacao-oi/scar"

if [ "$1" != "" ]; then
  PROJECT_FOLDER="$1"
fi
ambient_type="$2"
log_for_tests="$3"

function start_throw_error() {
  echo "$1" >&2
  exit 1
}

function add_log() {
  echo "$1" >> "$PROJECT_FOLDER/logs.txt"
}

# Verifica caminho da pasta do projeto
if [ ! -f "$PROJECT_FOLDER/src/tools/check.sh" ]; then
  start_throw_error "A pasta do projeto definida não está correta. (404)"
fi
# shellcheck source=/dev/null
check=$(source "$PROJECT_FOLDER/src/tools/check.sh")
if [ "$check" != "pasta do projeto ok!" ]; then
  start_throw_error "A pasta do projeto definida não está correta. (400)"
fi

# Cria arquivo de logs
true > "${PROJECT_FOLDER}/logs.txt"

# Carrega variáveis de ambiente .env
# shellcheck source=/dev/null
source "$PROJECT_FOLDER/src/tools/dotenv.sh" "$PROJECT_FOLDER" "$ambient_type" "$log_for_tests"

# Verifica caminho para arquivo do usuário
# shellcheck disable=SC2154
if [ "${process_env["USER_FOLDERS_LIST_PATH"]}" == "" ]; then
  start_throw_error "não foi informado no arquivo .env o caminho para a lista de pastas do usuário"
fi

add_log "[$(date +'%H:%M:%S')] Validador -> Iniciando validação do arquivo do usuário."

status_to_exit=0
# Verifica se existe arquivo já validado
if [ -f "$PROJECT_FOLDER/valid-list-of-folders" ]; then

  # Verifica se houve alteração no arquivo do usuário
  if [ "${process_env["USER_FOLDERS_LIST_PATH"]}" -nt "$PROJECT_FOLDER/valid-list-of-folders" ]; then
    add_log "[Status] Validador -> O arquivo do usuário foi alterado, iniciando validação..."

    # shellcheck source=/dev/null
    validation_message=$(source "$PROJECT_FOLDER/src/tools/validator.sh" "${process_env["USER_FOLDERS_LIST_PATH"]}" "$PROJECT_FOLDER" 2>&1)
    validation_result="$?"

    # Verifica se script conseguiu validar o arquivo do usuário
    if [ "$validation_result" -eq 0 ]; then
      add_log "[Success] Validator -> Arquivo do usuário validado com sucesso"
    else
      add_log "Validador -> $validation_message"
      errorMessage="O arquivo \"${process_env["USER_FOLDERS_LIST_PATH"]}\" é inválido, mas existe arquivo já validado"
      add_log "[Warning] Validador -> $errorMessage."
      status_to_exit=4
    fi
  else
    add_log "[Status] Validador -> O arquivo do usuário não foi alterado desde a ultima validação."
  fi
else
  add_log '[Warning] Validador -> Não foi encontrado um arquivo já validado, tentando validar o arquivo do usuário...'
  # shellcheck source=/dev/null
  validation_message=$(source "$PROJECT_FOLDER/src/tools/validator.sh" "${process_env["USER_FOLDERS_LIST_PATH"]}" "$PROJECT_FOLDER" 2>&1)
  validation_result="$?"

  # Verifica se scirpt conseguiu validar o arquivo do usuário
  if [ "$validation_result" -eq 0 ]; then
    add_log "[Success] Validator -> Arquivo do usuário validado com sucesso!"
  else
    add_log "Validador -> $validation_message"
    errorMessage="O arquivo \"${process_env["USER_FOLDERS_LIST_PATH"]}\" é inválido e não existe arquivo validado anteriormente"
    add_log "[FatalError] Validador -> $errorMessage."
    start_throw_error "$errorMessage"
  fi
fi

add_log "[$(date +'%H:%M:%S')] Validador -> Validação finalizada!"
add_log ""

add_log "[Status] iniciando execução do SCAR usando o arquivo validado"
add_log ""

# shellcheck source=/dev/null
main_result=$("$PROJECT_FOLDER/src/main.sh" "$PROJECT_FOLDER" "$log_for_tests" 2>&1)
main_status="$?"
if [ "$main_status" -gt 0 ]; then
  error_description="[Error($main_status): main.sh] $main_result"
  add_log "SCAR -> $error_description"
  start_throw_error "$error_description"
fi

exit "$status_to_exit"
