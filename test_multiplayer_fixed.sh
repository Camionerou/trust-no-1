#!/bin/bash

echo "🚀 INICIANDO PRUEBA MULTIJUGADOR TRUST-NO-1 (VERSIÓN ARREGLADA)"
echo "================================================================="

# Compilar proyecto
echo "📦 Compilando proyecto..."
cargo build
if [ $? -ne 0 ]; then
    echo "❌ Error al compilar"
    exit 1
fi

echo "✅ Compilación exitosa"

# Función para limpiar procesos al salir
cleanup() {
    echo ""
    echo "🧹 Limpiando procesos..."
    pkill -f "tn1-server"
    pkill -f "trust-no-1"
    exit 0
}

# Configurar trap para limpiar al salir
trap cleanup SIGINT SIGTERM

# Iniciar servidor
echo ""
echo "🌐 Iniciando servidor (logs minimizados)..."
./target/debug/tn1-server &
SERVER_PID=$!

# Esperar a que el servidor se inicie
sleep 2

# Verificar que el servidor está corriendo
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo "❌ Error: El servidor no se inició correctamente"
    exit 1
fi

echo "✅ Servidor iniciado (PID: $SERVER_PID)"

# Iniciar primer cliente
echo ""
echo "🎮 Iniciando Cliente 1..."
echo "   - Jugador LOCAL será AZUL 🔵"
echo "   - Jugadores REMOTOS serán ROJOS 🔴"
echo "   - Presiona F1 para ver estadísticas del servidor"
echo "   - Presiona F3 para debug info"
echo "   - ESC para liberar cursor"
./target/debug/trust-no-1 &
CLIENT1_PID=$!

# Esperar un poco
sleep 3

# Iniciar segundo cliente
echo ""
echo "🎮 Iniciando Cliente 2..."
echo "   - Cada cliente ve su propio jugador como AZUL"
echo "   - Los otros jugadores se ven como ROJOS"
./target/debug/trust-no-1 &
CLIENT2_PID=$!

echo ""
echo "🎯 PRUEBAS A REALIZAR:"
echo "====================="
echo ""
echo "1. 🔵 COLORES DE JUGADORES:"
echo "   ✓ Tu jugador debe ser AZUL"
echo "   ✓ Otros jugadores deben ser ROJOS"
echo ""
echo "2. 📹 CÁMARA:"
echo "   ✓ La cámara debe seguir a TU jugador azul"
echo "   ✓ No debe haber jitter o saltos"
echo "   ✓ Movimiento de cámara debe ser suave"
echo ""
echo "3. 🎮 CONTROLES:"
echo "   ✓ WASD para movimiento"
echo "   ✓ Espacio para saltar"
echo "   ✓ Shift para correr"
echo "   ✓ F1 para estadísticas del servidor"
echo "   ✓ F3 para debug info"
echo ""
echo "4. 🌐 NETWORKING:"
echo "   ✓ Movimiento debe sincronizarse entre clientes"
echo "   ✓ El panel F1 debe mostrar 1 local + 1 remoto"
echo "   ✓ Sin spam en logs del servidor"
echo ""
echo "5. 🏷️ IDENTIFICACIÓN:"
echo "   ✓ Tags flotantes: 'LOCAL - AZUL' y 'REMOTO - ROJO'"
echo ""

echo "🚀 AMBOS CLIENTES INICIADOS"
echo "Servidor logs: Solo errores y warnings"
echo "Presiona Ctrl+C para detener todo"

# Esperar indefinidamente
wait 