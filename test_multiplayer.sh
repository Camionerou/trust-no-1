#!/bin/bash

echo "🎮 TRUST-NO-1 MULTIPLAYER TEST"
echo "=============================="

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Ejecuta este script desde Trust-No-1/"
    exit 1
fi

echo "🔍 Verificando servidor..."
if lsof -i :7777 > /dev/null 2>&1; then
    echo "✅ Servidor detectado en puerto 7777"
else
    echo "❌ No se detectó servidor en puerto 7777"
    echo "🚀 Iniciando servidor..."
    RUST_LOG=info cargo run --bin tn1-server &
    SERVER_PID=$!
    echo "⏳ Esperando a que el servidor se inicie..."
    sleep 10
    
    if lsof -i :7777 > /dev/null 2>&1; then
        echo "✅ Servidor iniciado correctamente"
    else
        echo "❌ Error: No se pudo iniciar el servidor"
        exit 1
    fi
fi

echo ""
echo "🎯 Iniciando clientes de prueba..."
echo "📋 Logs: RUST_LOG=info habilitado"
echo ""

# Función para iniciar un cliente
start_client() {
    local client_num=$1
    echo "🚀 Iniciando Cliente #$client_num..."
    RUST_LOG=info cargo run --bin trust-no-1 &
    sleep 2
}

# Preguntar cuántos clientes iniciar
echo "¿Cuántos clientes quieres iniciar? (recomendado: 2-3)"
read -p "Número de clientes: " num_clients

if ! [[ "$num_clients" =~ ^[0-9]+$ ]] || [ "$num_clients" -lt 1 ] || [ "$num_clients" -gt 5 ]; then
    echo "❌ Número inválido. Usando 2 clientes por defecto."
    num_clients=2
fi

echo ""
echo "🎮 Iniciando $num_clients clientes..."

for i in $(seq 1 $num_clients); do
    start_client $i
done

echo ""
echo "✅ Todos los clientes iniciados!"
echo ""
echo "🎯 INSTRUCCIONES DE PRUEBA:"
echo "=========================="
echo "1. Cada ventana es un cliente diferente"
echo "2. Haz clic en cada ventana para capturar el cursor"
echo "3. Usa WASD para mover cada jugador"
echo "4. Deberías ver:"
echo "   - Tu esfera (beige) con tag 'LOCAL'"
echo "   - Esferas de otros (verdes) con tag 'REMOTO'"
echo "5. Usa ESC para liberar el cursor"
echo "6. Presiona F4 para debug info"
echo ""
echo "🔧 Para detener todo: Ctrl+C en esta terminal"
echo ""

# Mostrar conexiones activas
echo "🌐 Conexiones actuales:"
lsof -i :7777

# Esperar a que el usuario termine
echo ""
echo "Presiona Ctrl+C para detener todos los procesos..."
wait 