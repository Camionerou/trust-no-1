#!/bin/bash

echo "🎮 INICIANDO TRUST-NO-1 CLIENTE"
echo "================================"

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Ejecuta este script desde Trust-No-1/Trust-No-1/"
    echo "📁 Directorio actual: $(pwd)"
    echo "📁 Directorio esperado: .../Trust-No-1/Trust-No-1/"
    exit 1
fi

echo "✅ Directorio correcto detectado"

# Verificar conexión al servidor
echo "🔍 Verificando servidor..."
if lsof -i :7777 > /dev/null 2>&1; then
    echo "✅ Servidor detectado en puerto 7777"
    echo "🌐 Modo: Cliente-Servidor (Networking activo)"
else
    echo "⚠️  No se detectó servidor en puerto 7777"
    echo "🔄 Modo: Offline (Física local)"
    echo ""
    echo "Para ejecutar el servidor:"
    echo "RUST_LOG=info cargo run --bin tn1-server -p tn1_server"
fi

echo ""
echo "🚀 Compilando y ejecutando cliente..."
echo "📋 Logs: RUST_LOG=info habilitado"
echo ""

# Ejecutar el cliente
RUST_LOG=info cargo run --bin trust-no-1 -p tn1_client

echo ""
echo "👋 Cliente cerrado"