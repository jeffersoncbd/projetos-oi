#!/bin/bash

project_folder="$1"
ambient_type="$2"
print_for_tests="$3"

function dotenv_throw_error() {
  echo "$1" >&2
  exit 1
}

if [ "$project_folder" == "" ]; then
  dotenv_throw_error "o caminho para a pasta do projeto não foi informado \"\$1\""
fi

dot_env=".env"
if [ "$ambient_type" == "dev" ]; then
  dot_env=".env.dev"
fi

if [ ! -f "$project_folder/$dot_env" ]; then
  dotenv_throw_error "não foi encontrado um arquivo \"$dot_env\" na pasta \"${project_folder}/\", crie um usando o .env.example como base"
fi

declare -A process_env

# Mapeia .env em array
mapfile -t lines <<< "$(cat "$project_folder/$dot_env")"

# Itera linhas
for line in "${lines[@]}"; do
  # string trim
  line=$(echo "$line" | xargs)
  # Pega o primeiro caractere (definição da linha)
  flag=${line:0:1}


  # Ignora linhas de comentários e vazias
  if [ "$flag" == "" ] || [ "$flag" == "#" ]; then
    continue
  fi

  key=$(awk -F '=' '{ print $1 }' <<< "$line")
  key=$(echo "$key" | xargs)
  value=$(awk -F '=' '{ print $2 }' <<< "$line")
  value=$(awk -F '#' '{ print $1 }' <<< "$value")
  value=$(echo "$value" | xargs)

#  # shellcheck disable=SC2034
  process_env["$key"]="$value"
done

if [ "$print_for_tests" == "dotenv" ]; then
  for key in "${!process_env[@]}"; do
    echo "$key=\"${process_env["$key"]}\""
  done
fi
