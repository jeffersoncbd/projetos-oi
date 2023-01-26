#!/bin/bash
set +x

#          SCAR - Script de Compactação, Arquivamento e Remoção
#          ‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾
#  d8888b.  .o88b.  .d88b.  d8b   db  |  Criado por Julio Cesar de L. Silva
#  VP  `8D d8P  Y8 .8P  Y8. 888o  88  |    - Equipe 3CON - Nov de 2021
#     ooY' 8P      88    88 88V8o 88  |  Atualizado por Jefferson Carlos B. Diogo
#     ~~b. 8b      88    88 88 V8o88  |    - Equipe 3CON - Nov de 2022
#  db   8D Y8b  d8 `8b  d8' 88  V888  |
#  Y8888P'  `Y88P'  `Y88P'  VP   V8P  |

project_folder="$1"
folders_list_path="$project_folder/valid-list-of-folders"

if [ "$2" == "simulate_error" ]; then
  echo "Simulated error" >&2
  exit 1
fi

function main_logger() {
  identation="$2"
  echo "${identation}[$(date +'%H:%M:%S')] SCAR -> $1" >> "$project_folder/logs.txt"
}

function commands_logger() {
  echo -ne "$1" >> "$project_folder/logs.txt"
}

if [ "$folders_list_path" == "/valid-list-of-folders" ]; then
  echo "O caminho para o arquivo com a lista de pastas deve ser informado." >&2
  exit 1
fi

if [ ! -f "$folders_list_path" ]; then
  echo "O caminho para o arquivo com a lista de pastas não foi encontrado." >&2
  exit 1
fi

# Mapeia arquivo em array
mapfile -t lines <<< "$(cat "$folders_list_path")"

# Array de pastas e comandos
declare -a folders=()

# Array de comandos aceitos
declare -A acceptedCommands=( [zip]="gzip" [move]="mv" [remove]="rm -f" )

newFolder=""

# Itera linhas
for line in "${lines[@]}"; do
  # string trim
  line=$(echo "$line" | xargs)
  # Pega o primeiro caractere (definição da linha)
  flag=${line:0:1}

  # Ignora linhas vazias e de comentários
  if [ "$flag" == "" ] || [ "$flag" == "#" ]; then
    continue
  fi

  # Verifica se linha se refere a uma pasta
  if [ "$flag" == "/" ]; then
    # Verifica se já existe' uma pasta em definição
    if [ "$newFolder" != "" ]; then
      # Empurra pasta em definição para array de pastas
      folders[${#folders[@]}]="$newFolder"
    fi

    # Inicia definição de pasta
    newFolder="$line"
    continue
  fi

  # Complementa definição de pasta com comandos
  newFolder="$newFolder $line"
done
# Empurra ultima pasta em definição para array de pastas
folders[${#folders[@]}]="$newFolder"

main_logger "A lista de pastas foi carregada, iniciando execução de comandos."

# Itera pastas
for folder in "${folders[@]}"; do
  # Define caminho absoluto da pasta
  folderPath=$(echo "$folder" | awk -F' - ' '{print $1}')
  # Define lista de comandos a serem executados
  read -ra commands <<< "$(echo "$folder" | sed -e 's/ - /|/g' -e 's/ /+/g' -e 's/|/ /g' | cut -d ' ' -f 2-)"

  main_logger "processando a pasta \"$folderPath\"..." "  "

  # Executa comandos
  for userCommand in "${commands[@]}"; do
    # Comando principal
    mainCommand=$(awk -F '+' '{print $2}' <<< "$userCommand")
    moveToPath=""
    daysForDelete=""

    # Comando de busca
    days=$(awk -F '+' '{print $1}' <<< "$userCommand")
    ignoreZips=""
    # Verifica se arquivos serão zipados, se sim, faz findCommand ignorar já zipados...
    if [ "$mainCommand" == "zip" ]; then
      ignoreZips="! -name '*.gz'"
    fi
    findCommand="find $folderPath -maxdepth 1 -type f $ignoreZips -mtime +$days"

    # Verifica se arquivos serão zipados e define dias para exclusão dos zips
    if [ "$mainCommand" == "zip" ]; then
      daysForDelete=$(awk -F '+' '{print $3}' <<< "$userCommand")
    fi
    # Verifica se arquivos serão movidos e define dias para exclusão dos arquivos movidos
    if [ "$mainCommand" == "move" ]; then
      moveToPath=$(awk -F '+' '{print $3}' <<< "$userCommand")
      daysForDelete=$(awk -F '+' '{print $4}' <<< "$userCommand")
    fi

    # Verifica se foi definido um dia para exclusão, se sim, executa o comando de exclusão
    if [ "$daysForDelete" != "" ]; then
      folderToDelete=$([ "$mainCommand" == "move" ] && echo "$moveToPath" || echo "$folderPath")

      eval "find $folderToDelete -maxdepth 1 -type f -mtime +$daysForDelete -exec rm -f {} \;"
    fi

    mapfile -t files <<< "$(eval "$findCommand")"

    declare -A commands_description=( [zip]="compactando" [move]="movendo" [remove]="removendo" )
    ((files_lenght="${#files[@]}"-1))
    commands_logger "    ${commands_description[$mainCommand]} $files_lenght arquivos: "

    for file_name in "${files[@]}"; do {
      # if para previnir chamada infinita
      if [ "$file_name" != "" ]; then
        # Trata arquivos reenviados (possíveis duplicados)
        if [ -f "$file_name.gz" ] && [ "$mainCommand" == "zip" ]; then
          suffix_id=1
          while true; do
            if [ ! -f "$file_name-${suffix_id}o_reenvio.gz" ]; then
              break;
            fi
            suffix_id=$(("$suffix_id"+1))
          done
          eval "${acceptedCommands[$mainCommand]} -S \"-${suffix_id}o_reenvio.gz\" $file_name $moveToPath"
        else
          eval "${acceptedCommands[$mainCommand]} $file_name $moveToPath"
        fi
      fi
    } done
    commands_logger "OK\n"
  done

  commands_logger "\n"
done

main_logger "Todas as pastas foram processadas! encerrando script..."
