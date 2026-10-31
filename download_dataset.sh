#!/bin/bash
set -e

mkdir -p data

echo "1/3. Mengunduh data mentah dari Wikimedia..."
wget -q --show-progress https://dumps.wikimedia.org/other/pageviews/2026/2026-02/pageviews-20260201-000000.gz -O data/pageviews-20260201-000000.gz

echo "2/3. Ekstraksi file..."
gunzip -f -k data/pageviews-20260201-000000.gz

echo "3/3. Membersihkan dan mengurutkan data..."
awk '$2 != "-" { count[$2] += $3 } END { for (k in count) print k "\t" count[k] }' data/pageviews-20260201-000000 | env LC_ALL=C sort -t "$(printf '\t')" -k1,1 > data/dataset_4m.tsv

echo "Dataset tersimpan di data/dataset_4m.tsv"
