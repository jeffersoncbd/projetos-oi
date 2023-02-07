#!/bin/bash

endpoint="http://186.230.30.26:8081/v1/oi-fatura-ura/json?cpf_cnpj=34863338953&mes_referencia=122021&contrato=37378905&valor=240.43&vencimento=26012022&sistema_origem=4"

response=$(curl -I "${endpoint}" 2>&1)
status_code=$(echo "${response}" | grep HTTP/ | awk -F' ' '{print $2}')

if [ "${status_code}" -ne 200 ]; then
    /webaplic/.bin/tg -845994286 "⚠️ ATENÇÃO ⚠️"
    /webaplic/.bin/tg -845994286 "Falha na conexão com link da FAST"
    /webaplic/.bin/tg -845994286 "Código HTTP retornado: ${status_code}"

    echo "${response}" >&2
    exit 1
fi
