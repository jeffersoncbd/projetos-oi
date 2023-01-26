#!/bin/bash

folders_list_path="$1" # Path do arquivo com lista de pastas e comandos do usuário
validated_file_folder="$2" # Path da pasta onde será copiado o arquivo válido

# Função para padronizar erros
function validator_throw_error() {
  local error_flag="[Error] validator ->"
  local message_error="$1"
  local -a rows=("$@")

  echo "$error_flag $message_error" >&2
  for row in "${rows[@]:1}"; do
    echo "$folders_list_path: $row" >&2
  done
  exit 1
}

# Função para extrair dados de uma linha (número, flag e conteudo)
function extract_row_metadata() {
  local complete_row="$1"
  row["content"]=$(awk -F '|' '{print $2}' <<< "$complete_row")
  row["number"]=$(awk -F '|' '{print $1}' <<< "$complete_row")
  row["flag"]=${row["content"]:0:1}
}
# Função identica a anterior, mas para comparações paralelas
function extract_temp_row_metadata() {
  local complete_row="$1"
  temp_row["content"]=$(awk -F '|' '{print $2}' <<< "$complete_row")
  temp_row["number"]=$(awk -F '|' '{print $1}' <<< "$complete_row")
  temp_row["flag"]=${temp_row["content"]:0:1}
}

function remove_empty_rows_and_comments() {
  local -a temp=()
  for i in $(seq "${#rows[@]}"); do
    local row
    row="${rows[$i-1]}"

    # string trim
    row=$(echo "$row" | xargs)

    local first_character=${row:0:1}
    if [ "$row" == "" ] || [ "$first_character" == "#" ]; then
      continue
    fi
    temp["${#temp[@]}"]="$i|$row"
  done
  rows=( "${temp[@]}" )
}

function verify_flags() {
  local -A accepted_flags=( ["/"]=1 ["-"]=1 )
  for raw_row in "${rows[@]}"; do
    local -A row
    extract_row_metadata "$raw_row"

    local flag="${accepted_flags[${row["flag"]}]}"
    if [ "$flag" == "" ]; then
      local error_message="As linhas devem começar com \"/\" ou \"-\""
      local row_error="linha ${row["number"]}: ${row["content"]}"
      validator_throw_error "$error_message" "$row_error"
    fi
  done
}

function verify_folders_commands() {
  for i in $(seq "${#rows[@]}"); do
    local -A row
    extract_row_metadata "${rows[$i-1]}"

    if [ "${row["flag"]}" == "/" ]; then
      local -A temp_row
      extract_temp_row_metadata "${rows[$i]}"

      if [ "${temp_row["flag"]}" != "-" ]; then
        local error_message="Toda pasta deve ter ao menos um comando (iniciado por \"-\")"
        local row_1="linha ${row["number"]}: ${row["content"]}"
        local row_2="linha $(("${row["number"]}"+1)): ${temp_row["content"]}"
        validator_throw_error "$error_message" "$row_1" "$row_2"
      fi
    fi
  done
}

function valid_folders() {
  for raw_row in "${rows[@]}"; do
    local -A row
    extract_row_metadata "$raw_row"
    if [ "${row["flag"]}" != "/" ]; then
      continue
    fi
    if [ ! -d "${row["content"]}" ]; then
      local error_message="Uma pasta informada no arquivo não foi encontrada"
      local row_error="linha ${row["number"]}: ${row["content"]}"
      validator_throw_error "$error_message" "$row_error"
    fi
  done
}

function verify_commands() {
  for raw_row in "${rows[@]}"; do
    local -A row
    extract_row_metadata "$raw_row"

    if [ "${row["flag"]}" != "-" ]; then
      continue
    fi

    local -a row_commands
    mapfile -t row_commands <<< "$(echo "${row["content"]:1}" | tr '-' '\n')"

    for command in "${row_commands[@]}"; do
      # string trim
      command=$(echo "$command" | xargs)

      if [ "$command" == "" ]; then
        local error_message="Uma pasta possui linha de comandos vazia"
        local row_error="linha ${row["number"]}: ${row["content"]}"
        validator_throw_error "$error_message" "$row_error"
      fi

      local day_to_exec
      local command_name
      local destiny_path
      local day_to_remove
      day_to_exec="$(awk -F ' ' '{print $1}' <<< "$command")"
      command_name="$(awk -F ' ' '{print $2}' <<< "$command")"

      local regex='^[0-9]+$'
      if ! [[ "$day_to_exec" =~ $regex ]] ; then
        local error_message="Um comando não possui \"d1\" especificado (dias para execução)"
        local row_error="linha ${row["number"]}: ${row["content"]}"
        validator_throw_error "$error_message" "$row_error"
      fi

      local -A accepted_commands=( ["zip"]=1 ["move"]=1 ["remove"]=1 )
      local current_command="${accepted_commands["$command_name"]}"
      if [ "$current_command" == "" ]; then
        local error_message="O comando \"$command_name\" não é aceito (use \"zip\", \"move\" ou \"remove\")"
        local row_error="linha ${row["number"]}: ${row["content"]}"
        validator_throw_error "$error_message" "$row_error"
      fi

      if [ "$command_name" == "move" ]; then
        destiny_path="$(awk -F ' ' '{print $3}' <<< "$command")"
        day_to_remove="$(awk -F ' ' '{print $4}' <<< "$command")"

        if [ "$destiny_path" == "" ]; then
          local error_message="Quando utilizado o comando \"move\" deve ser informado o caminho de destino"
          local row_error="linha ${row["number"]}: ${row["content"]}"
          validator_throw_error "$error_message" "$row_error"
        fi

        if [ ! -d "$destiny_path" ]; then
          local error_message="O caminho do destino para o comando \"move\" é inválido"
          local row_error="linha ${row["number"]}: ${row["content"]}"
          validator_throw_error "$error_message" "$row_error"
        fi
      else
        day_to_remove="$(awk -F ' ' '{print $3}' <<< "$command")"
      fi

      if [ "$day_to_remove" != "" ]; then
        if ! [[ "$day_to_remove" =~ $regex ]]; then
          local error_message="Um comando possui um \"d2\" inválido (dias para remoção) utilize apenas números inteiros"
          local row_error="linha ${row["number"]}: ${row["content"]}"
          validator_throw_error "$error_message" "$row_error"
        fi
      fi
    done
  done
}

if [ "$folders_list_path" == "" ]; then
  validator_throw_error "O parâmetro \"\$1\" (lista de pastas do usuário) não foi informado."
fi

if [ "$validated_file_folder" == "" ]; then
  validator_throw_error "O parâmetro \"\$2\" (destino do arquivo validado) não foi informado."
fi

if [ ! -f "$folders_list_path" ]; then
  validator_throw_error "O arquivo \"$folders_list_path\" não existe."
fi

if [ ! -d "$validated_file_folder" ]; then
  validator_throw_error "A pasta para salvar o arquivo validado \"$validated_file_folder\" não existe."
fi

# Lê arquivo informado
mapfile -t rows <<< "$(cat "$folders_list_path")"

# Remove linhas vazias e comentários
# inclui o número de cada linha no conteudo para que esta informação não seja perdida
remove_empty_rows_and_comments "${rows[@]}"

# Verifica se o arquivo está vazio
if [ "${#rows[@]}" -eq 0 ]; then
  validator_throw_error "O arquivo \"$folders_list_path\" está vazio."
fi

# Verifica se as flags (primeira letra de uma linha) está de acordo
verify_flags

# Verifica se cada pasta possui pelomenos um comando
verify_folders_commands

# Verifica se cada pasta existe no sistema de arquivo do sistema
valid_folders

# Valida todos os comandos de cada pasta
verify_commands

# Sendo válido, copia a lista para ser consumida pelo script principal
cp "$folders_list_path" "$validated_file_folder/valid-list-of-folders"
