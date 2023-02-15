#!/bin/bash

endpoint="http://ged360.intranet/ws/econtas/get-token"
header="Content-Type: application/json"
data='{"camArquivoQrcode": "NAS:DAAEQAQ51CC5DFEF1A653FB73EE20738", "dataDeExpiracao": "15-02-2023 23:59:59"}'
response=$(curl --insecure --request POST --url "${endpoint}" --header "${header}" --data "${data}"2>&1)

if [[ "${response}" != *"\"type\":\"success\""* ]]; then
    ./tg -845994286 "⚠️ ATENÇÃO ⚠️"
    ./tg -845994286 "O GED 360 retornou algo diferente de type=sucesso"
    ./tg -845994286 "Resposta do endpoint: ${response}"
else
    ./tg -845994286 "✅ GED360 [QRCode] retornou sucesso."
fi
