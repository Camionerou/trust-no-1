#!/bin/bash

echo "🛑 Deteniendo todos los procesos de TRUST-NO-1..."

# Detener servidor y clientes
pkill -f "tn1-server"
pkill -f "trust-no-1"

# Esperar un momento
sleep 2

# Verificar que se hayan detenido
if lsof -i :7777 > /dev/null 2>&1; then
    echo "⚠️  Algunos procesos siguen corriendo en puerto 7777"
    echo "🔧 Forzando cierre..."
    sudo lsof -ti:7777 | xargs sudo kill -9 2>/dev/null
else
    echo "✅ Todos los procesos detenidos correctamente"
fi

echo "🏁 Limpieza completada" 