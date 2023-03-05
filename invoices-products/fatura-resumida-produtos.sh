#!/bin/bash

rm "$HOME/result-products.csv" 2> /dev/null

source "$HOME/.credentials"

echo "@$HOME/.queries/products.sql $HOME/result-products.csv" | /oracle/app/product/11.2.0.4/bin/sqlplus -s opuser/$DB_PASS@prgedbd-p1 2> /dev/null
sed -i '1d' "$HOME/result-products.csv"
sed -i '2d' "$HOME/result-products.csv"
