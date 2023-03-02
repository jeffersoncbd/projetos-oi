#!/bin/bash

# --------------- Variáveis --------------- #
PROJECT_FOLDER="/webaplic/bot_status_faturas"
#TG_DESTINATARY_ID="5828637972" # Jefferson
TG_DESTINATARY_ID="-845994286"
TG=""
CSV_FILE="${PROJECT_FOLDER}/control_m_execucoes.csv"
SQL_FILE="${PROJECT_FOLDER}/queries/control_m_execucoes.sql"
XML_FILE="${PROJECT_FOLDER}/RAID_COLLECTIONS.xml"
CHAIN_MONITOR="${PROJECT_FOLDER}/relatorios/monitor_de_cadeias"
# ----------------------------------------- #

yesterday=$(date -d "yesterday" +'%d/%m/%Y')
today=$(date +'%d/%m/%Y')

if [ "${1}" == "dev" ]; then
    #TG_DESTINATARY_ID="5828637972" # Jefferson
    TG_DESTINATARY_ID="163185688" # Gabriel
    PROJECT_FOLDER="/home/jefferson/projects/oi/chain-monitor"
    CHAIN_MONITOR="${PROJECT_FOLDER}/chain-monitor"
    CSV_FILE="${PROJECT_FOLDER}/model.csv"
    XML_FILE="${PROJECT_FOLDER}/RAID_COLLECTIONS.xml"
    TG="/home/jefferson/projects/oi/bot/bot-oi"
    yesterday="29/01/2023"
    today="30/01/2023"
else
    # EXECUÇÃO DE QUERY NO BANCO DE DADOS COM SPOOL DE CSV
    source "${PROJECT_FOLDER}/.credentials"
    echo "@${SQL_FILE} ${CSV_FILE}" | /oracle/app/product/11.2.0.4/bin/sqlplus -s opuser/${DB_PASS}@prgedbd-p1 > /dev/null
    sed -i '1d' "${CSV_FILE}"
    sed -i '2d' "${CSV_FILE}"
fi

# --- CADEIA BLOQ DESBLOQ --- #
CHK_LOAD_RAIDC_start=$(${CHAIN_MONITOR} "${CSV_FILE}" "${XML_FILE}" -i "CHK_LOAD_RAIDC" "--dia=${yesterday}")
DUMMY_RAIDC_EvaluateActions_end=$(${CHAIN_MONITOR} "${CSV_FILE}" "${XML_FILE}" -f "DUMMY_RAIDC_EvaluateActions" "--dia=${today}")
r1_line1="🔹 Cadeia: RAIDC BLOQ DESBLOQ"
r1_line2="Iniciou em ${CHK_LOAD_RAIDC_start}"
r1_line3="Finalizou em ${DUMMY_RAIDC_EvaluateActions_end}"
report1="${r1_line1}"$'\n'"${r1_line2}"$'\n'"${r1_line3}"

# --- CADEIA CAMPANHAS AUTOMATICAS --- #
RAIDC_AutoDistribCampaignGenerator_start=$(${CHAIN_MONITOR} "${CSV_FILE}" "${XML_FILE}" -i "RAIDC_AutoDistribCampaignGenerator" "--dia=${today}")
RAIDC_AutoDistribCampaignGenerator_status=$(${CHAIN_MONITOR} "${CSV_FILE}" "${XML_FILE}" -s "RAIDC_AutoDistribCampaignGenerator" "--dia=${today}")
r2_line1="🔹 Cadeia: CAMPANHA"
r2_line2="Iniciou em ${RAIDC_AutoDistribCampaignGenerator_start}"
r2_line3="No momento está ${RAIDC_AutoDistribCampaignGenerator_status}"
report2="${r2_line1}"$'\n'"${r2_line2}"$'\n'"${r2_line3}"

# --- CADEIA RAJADA --- #
RAIDC_ProcIsencaoRajada_start=$(${CHAIN_MONITOR} "${CSV_FILE}" "${XML_FILE}" -i "RAIDC_ProcIsencaoRajada" "--dia=${today}" --execucao=1)
RAIDC_ProcIsencaoRajada_end=$(${CHAIN_MONITOR} "${CSV_FILE}" "${XML_FILE}" -f "RAIDC_ProcIsencaoRajada" "--dia=${today}" --execucao=1)
r3_line1="🔹 Cadeia: RAJADA 1ª Execução"
r3_line2="Iniciou em ${RAIDC_ProcIsencaoRajada_start}"
r3_line3="Terminou em ${RAIDC_ProcIsencaoRajada_end}"
report3="${r3_line1}"$'\n'"${r3_line2}"$'\n'"${r3_line3}"

# --- SEND TO TELEGRAM --- #
${TG} "${TG_DESTINATARY_ID}" "${report1}"$'\n\n'"${report2}"$'\n\n'"${report3}"
