#!/bin/bash

filtro=$1

# --------------- Variáveis --------------- #
PROJECT_FOLDER="/webaplic/bot_status_faturas"
TG_TOKEN="5600687475:AAFtM9o3pcnECnDCMeM42OC0Rb1etknjk_Q"
# TG_DESTINATARY_ID="-1001780198496"
TG_DESTINATARY_ID="5828637972" # Jefferson
CSV_FILE="${PROJECT_FOLDER}/email_seguro_produtos.csv"
SQL_FILE="${PROJECT_FOLDER}/queries/email_seguro_produtos.sql"
# ----------------------------------------- #

# EXECUÇÃO DE QUERY NO BANCO DE DADOS COM SPOOL DE CSV
source "${PROJECT_FOLDER}/.credentials"
echo "@${SQL_FILE} ${CSV_FILE}" | /oracle/app/product/11.2.0.4/bin/sqlplus -s opuser/${DB_PASS}@prgedbd-p1 > /dev/null
sed -i '1d' "${CSV_FILE}"
sed -i '2d' "${CSV_FILE}"

# EXECUÇÃO DA HABILIDADE DO BOT
${PROJECT_FOLDER}/relatorios/email_seguro_produtos "${CSV_FILE}" "${TG_TOKEN}" "${TG_DESTINATARY_ID}" "${filtro}"

rm -f "${CSV_FILE}" > /dev/null
