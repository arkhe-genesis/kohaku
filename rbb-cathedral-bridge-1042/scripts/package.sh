#!/bin/bash
# Substrato 1042 - RBB-CATHEDRAL-BRIDGE
# Script para empacotar o projeto

cd "$(dirname "$0")/.."

echo "Empacotando projeto rbb-cathedral-bridge-1042..."
tar -czvf rbb-cathedral-bridge-1042.tar.gz \
    --exclude='__pycache__' \
    --exclude='*.pyc' \
    --exclude='*.tar.gz' \
    .

echo "Pacote gerado: rbb-cathedral-bridge-1042.tar.gz"
