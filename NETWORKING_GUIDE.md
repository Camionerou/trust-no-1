# 🌐 Guía de Networking - Trust No One

## Resumen

Trust-No-1 ahora implementa una arquitectura **cliente-servidor autoritativo** donde:
- ✅ **Cliente**: Solo captura inputs y renderiza
- ✅ **Servidor**: Procesa toda la física y lógica del juego
- ✅ **Networking**: TCP en tiempo real entre cliente y servidor

## 🚀 Cómo Ejecutar

### 1. Ejecutar el Servidor
```bash
cd Trust-No-1/Trust-No-1
RUST_LOG=info cargo run --bin tn1-server -p tn1_server
```

**Logs esperados:**
```
🌐 Servidor TCP iniciado en 127.0.0.1:7777
Servidor Trust-No-1 iniciado correctamente
Modo: Servidor autoritativo headless
```

### 2. Ejecutar el Cliente
```bash
cd Trust-No-1/Trust-No-1
./run_client.sh
```

**Conexión exitosa:**
```
✅ Servidor detectado en puerto 7777
🌐 Modo: Cliente-Servidor (Networking activo)
✅ Conectado al servidor!
```

## 🎮 Modos de Operación

### Modo Conectado (Cliente-Servidor)
- **Física**: Procesada en el servidor
- **Inputs**: Enviados del cliente al servidor
- **Posiciones**: Recibidas del servidor
- **UI Debug**: Muestra "Server authoritative"

### Modo Offline (Solo Cliente)
- **Física**: Local temporal
- **Inputs**: Procesados localmente
- **UI Debug**: Muestra "Local physics"

## 🔧 Arquitectura Técnica

### Protocolo de Comunicación
```
Cliente → Servidor: INPUT:forward:backward:left:right:jump:sprint:yaw:pitch
Servidor → Cliente: POS:x:y:z:player_id
```

### Flujo de Datos
1. **Cliente captura input** (WASD, Shift, Space, Mouse)
2. **Cliente envía input al servidor** via TCP
3. **Servidor procesa física** con validación anti-cheat
4. **Servidor envía posición actualizada** al cliente
5. **Cliente renderiza** la posición autoritativa

### Componentes Clave
- `ClientNetworkingPlugin`: Maneja conexión TCP del cliente
- `NetworkingPlugin`: Procesa inputs y envía posiciones del servidor
- `ServerNetwork`: Estado de conexiones y inputs pendientes
- `PlayerInput`: Estructura de input serializable

## 🛡️ Características de Seguridad

### Servidor Autoritativo
- ✅ Toda la física se procesa en el servidor
- ✅ Validación de límites del mundo
- ✅ Detección de velocidades imposibles
- ✅ Control de saltos y gravedad

### Anti-Cheat
- ✅ Cliente no puede modificar posición directamente
- ✅ Servidor valida todos los inputs
- ✅ Límites de velocidad y movimiento
- ✅ Detección de inputs imposibles

## 🎯 Controles

### Movimiento
- **WASD**: Movimiento direccional
- **Shift**: Sprint (1.5x velocidad)
- **Space**: Salto (con buffer y coyote time)
- **Mouse**: Rotación de cámara

### Debug
- **F3**: Toggle debug UI
- **F4**: Print debug info a consola
- **ESC**: Liberar cursor

## 🔍 Debug y Monitoreo

### UI Debug (F3)
```
🌐 NETWORKING
✅ Conectado al servidor
Modo: Cliente-Servidor

⚙️ ARQUITECTURA
Modo: Cliente autoritativo del servidor
Inputs: Enviados al servidor
Física: Procesada en servidor
```

### Logs del Servidor
```
🔌 Nuevo cliente conectado: 1
👤 Jugador UUID creado para cliente 1
🎮 Input procesado de cliente 1
🎯 Input aplicado a jugador de cliente 1
🦘 Jugador saltó en servidor
```

### Logs del Cliente
```
✅ Conectado al servidor!
📍 Posición actualizada desde servidor: (x, y, z)
Mode: Server authoritative
Physics: Processed on server
```

## 🚨 Troubleshooting

### Cliente no se conecta
```bash
# Verificar que el servidor esté corriendo
lsof -i :7777

# Reiniciar servidor
pkill -f tn1-server
RUST_LOG=info cargo run --bin tn1-server -p tn1_server
```

### Movimiento no funciona
1. Verificar conexión en UI debug (F3)
2. Comprobar logs del servidor para inputs
3. Verificar que el servidor procese física

### Lag o desconexión
- El cliente automáticamente vuelve a física local
- UI muestra "❌ Desconectado" y "Modo: Offline"

## 🎯 Próximos Pasos

### Implementados ✅
- [x] Servidor autoritativo
- [x] Networking TCP
- [x] Procesamiento de inputs
- [x] Sincronización de posiciones
- [x] Fallback a física local

### Por Implementar 🚧
- [ ] Predicción del lado cliente (client-side prediction)
- [ ] Interpolación de posiciones
- [ ] Sistema de armas
- [ ] Múltiples jugadores simultáneos
- [ ] Sincronización de rotación de cámara

## 📝 Notas Técnicas

### Rendimiento
- **TPS Servidor**: 60 ticks por segundo
- **Protocolo**: TCP (confiable pero con latencia)
- **Frecuencia de envío**: Cada frame con input activo

### Limitaciones Actuales
- Solo un jugador por cliente
- Rotación de cámara no sincronizada
- Sin interpolación de movimiento
- TCP puede tener latencia alta

### Arquitectura "Trust No One"
✅ **Implementado**: El cliente nunca hace cálculos de física críticos
✅ **Validado**: Servidor valida todos los inputs
✅ **Seguro**: Imposible hacer trampa modificando el cliente 