#!/bin/bash

echo "@$HOME/model.sql $HOME/chains.csv" | sqlplus -s RC_ADM/'$Cobranca2018#'@rcprd-p1 2> /dev/null

