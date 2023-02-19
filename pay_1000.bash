#!/bin/bash

# Generate unique IDs and amounts
ids=($(printf "%04d\n" $(seq 0 999)))
amounts=($(awk -v min=100.0 -v max=2000.0 'BEGIN{srand(); for (i=1;i<=1000;i++) printf "%.2f\n", min+rand()*(max-min)}'))

# Send payment requests and print results in a table
for i in {0..999}; do
    id=${ids[$i]}
    amount=${amounts[$i]}
    curl "http://127.0.0.1:8080/payment?id=$id&amount=$amount" || true
done | sort -k2
