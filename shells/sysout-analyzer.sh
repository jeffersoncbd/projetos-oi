#!/bin/bash

message=$1;
script_name=$2;
search_term=$3;
tg_chat_id=$4

# gerar_links_martech_sh

result=$(< "$(find /controlM/ctma9005/ctm/sysout/ -name "${script_name}*" -type f | sort -n | tail -1)" grep "${search_term}")

if [ "${result}" != "" ]; then
    /webtools/bot/tg "${tg_chat_id}" $'⚠️ ATENÇÃO ⚠️\n'"${message}"$'\n'"${result}"
fi
