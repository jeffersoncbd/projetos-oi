#!/bin/bash

# Cria um novo arquivo de acordo com os parametros recebidos

folder_path="$1"
file_id="$2"
size="$3"
days_ago="$4"

if [ "$folder_path" == "" ]; then
  echo "deve ser informada a pasta em que o arquivo deve ser criado - \"\$1\"" >&2
  exit 1
fi

if [ ! -d "$folder_path" ]; then
  echo "não foi encontrada a pasta em que o arquivo deve ser criado - \"\$1\"" >&2
  exit 1
fi

if [ "$file_id" == "" ]; then
  echo "deve ser informado o ID do arquivo a ser criado - \"\$2\"" >&2
  exit 1
fi

if [ "$size" == "" ]; then
  echo "o tamanho do arquivo deve ser informado (Mb) - \"\$3\"" >&2
  exit 1
fi

if ! [[ "$size" =~ ^[0-9]+$ ]] ; then
  echo "o tamanho do arquivo deve ser um número inteiro (Mb) - \"\$3\"" >&2
  exit 1
fi

if [ "$days_ago" == "" ]; then
  days_ago=0
fi

if ! [[ "$days_ago" =~ ^[0-9]+$ ]] ; then
  echo "o dia de criação do arquivo deve ser um número inteiro - \"\$4\"" >&2
  exit 1
fi

file_name="file-$file_id"

now=$(date +%Y%m%d)
date=$(date --date="${now} - ${days_ago} day" +%Y%m%d)
dd if=/dev/zero of="$folder_path/$file_name" bs=1M count="$size" 2> /dev/null
touch -amt "${date}1200.00" "$folder_path/$file_name"
