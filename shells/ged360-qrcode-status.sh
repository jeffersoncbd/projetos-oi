#!/bin/bash

endpoint="http://ged360.intranet/ws/econtas/get-token"
header="Content-Type: application/json"
data='{"camArquivoQrcode": "NAS:DAAEQAQ51CC5DFEF1A653FB73EE20738", "dataDeExpiracao": "15-02-2023 23:59:59"}'
response=$(curl --insecure --request POST --url "${endpoint}" --header "${header}" --data "${data}" 2>&1)

response=$(tail -n +4 <<< "$response")

if [[ "${response}" != *"\"type\":\"success\""* ]]; then
    /webaplic/.bin/tg "-845994286" $'⚠️ ATENÇÃO ⚠️\nFalha na conexão com link do GED360 QRCode\nResposta do endpoint:\n'"${response}"
fi
