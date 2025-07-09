# BEVY ARCHITECTURE - TRUST-NO-1

## Visión General

TRUST-NO-1 utiliza Bevy como motor principal, aprovechando su arquitectura ECS (Entity Component System) para crear un juego modular, performante y mantenible.

## Principios de Diseño ECS

### Entity Component System

#### Entities
- Identificadores únicos sin datos
- Representan objetos del juego (jugadores, items, proyectiles)
- Creadas y destruidas dinámicamente

#### Components  
- Datos puros sin lógica
- Ejemplo: Position, Health, Velocity, Inventory
- Composición sobre herencia

#### Systems
- Lógica que opera sobre componentes
- Ejecutan en paralelo cuando es posible
- Ordenados por dependencias

#### Resources
- Estado global compartido
- Configuraciones, conexiones, caches
- Accesibles desde cualquier sistema

## Arquitectura de Plugins

### Plugin Principal

El juego se estructura como una colección de plugins Bevy:

```
TrustNo1Plugin
├── CorePlugin          // Sistemas base
├── NetworkingPlugin    // Cliente o servidor
├── InventoryPlugin     // Sistema de inventario
├── BallisticsPlugin    // Física de proyectiles
├── MedicalPlugin       // Sistema médico
├── WeaponsPlugin       // Armas y combate
├── BuildingPlugin      // Construcción
├── VehiclePlugin       // Vehículos
├── WorldPlugin         // Gestión del mundo
└── UIPlugin           // Interfaz (solo cliente)
```

### Estructura de un Plugin

Cada plugin sigue este patrón:
1. Registrar componentes para replicación
2. Añadir recursos de configuración
3. Registrar eventos del sistema
4. Añadir sistemas en orden correcto
5. Configurar stages si es necesario

## Organización de Sistemas

### System Sets

Agrupación lógica de sistemas para ordenamiento:

```
InputSet        → Procesamiento de input
MovementSet     → Física y movimiento  
CombatSet       → Combate y daño
InventorySet    → Gestión de inventario
NetworkSet      → Sincronización de red
RenderSet       → Preparación de render (cliente)
```

### Ordenamiento de Sistemas

Los sistemas se ejecutan en este orden cada frame:

1. **Pre-Update**
   - Leer input del jugador
   - Recibir datos de red
   - Actualizar timers

2. **Update**
   - Procesar comandos
   - Ejecutar lógica del juego
   - Resolver física
   - Aplicar daño

3. **Post-Update**
   - Sincronizar estado
   - Enviar datos de red
   - Limpiar entidades muertas

## Componentes Core

### Componentes Compartidos (Cliente/Servidor)

#### Transform Components
- `Position`: Posición en el mundo
- `Rotation`: Orientación
- `GridPosition`: Posición en grid de chunks

#### Estado Components
- `Health`: Salud actual y máxima
- `Stamina`: Resistencia
- `Temperature`: Temperatura corporal
- `Hunger`, `Thirst`: Necesidades

#### Inventory Components
- `Inventory`: Grid de items
- `Equipment`: Items equipados
- `Weight`: Peso actual y máximo

#### Combat Components
- `Weapon`: Arma equipada
- `Armor`: Protección por zona
- `DamageDealer`: Inflige daño
- `DamageReceiver`: Recibe daño

### Componentes Solo-Servidor

#### Authority Components
- `Owner`: Jugador propietario
- `Permissions`: Acceso y permisos
- `NetworkId`: ID de red único

#### Validation Components
- `LastValidPosition`: Anti-cheat
- `ActionCooldowns`: Rate limiting
- `TrustScore`: Nivel de confianza

### Componentes Solo-Cliente

#### Rendering Components
- `Mesh`: Modelo 3D
- `Material`: Texturas y shaders
- `AnimationState`: Estado de animación
- `ParticleEmitter`: Efectos visuales

#### UI Components
- `Interactable`: Puede ser clickeado
- `Tooltip`: Información hover
- `HealthBar`: Barra de vida visible

## Recursos del Sistema

### Recursos Compartidos

#### Configuration Resources
- `GameConfig`: Configuración general
- `BallisticsConfig`: Física de proyectiles
- `ItemDatabase`: Definiciones de items
- `RecipeDatabase`: Recetas de crafting

#### State Resources
- `GameState`: Estado actual del juego
- `TimeOfDay`: Hora del mundo
- `Weather`: Clima actual

### Recursos del Servidor

#### Network Resources
- `NetworkManager`: Gestión de conexiones
- `PlayerSessions`: Sesiones activas
- `ReplicationRules`: Qué replicar

#### Database Resources
- `DatabasePool`: Conexiones PostgreSQL
- `RedisConnection`: Cache Redis
- `TransactionQueue`: Cola de DB

### Recursos del Cliente

#### Rendering Resources
- `AssetHandles`: Assets cargados
- `CameraSettings`: Configuración cámara
- `GraphicsSettings`: Calidad gráfica

#### Input Resources
- `InputBindings`: Mapeo de controles
- `MouseSensitivity`: Sensibilidad
- `KeyboardLayout`: Distribución

## Eventos del Sistema

### Eventos de Gameplay

#### Combat Events
- `DamageEvent`: Daño infligido
- `DeathEvent`: Muerte de entidad
- `WeaponFiredEvent`: Disparo realizado
- `ProjectileHitEvent`: Impacto de proyectil

#### Inventory Events
- `ItemPickupEvent`: Recoger item
- `ItemDropEvent`: Soltar item
- `ItemUseEvent`: Usar item
- `ItemCraftEvent`: Craftear item

#### World Events
- `ChunkLoadEvent`: Cargar chunk
- `ChunkUnloadEvent`: Descargar chunk
- `WeatherChangeEvent`: Cambio de clima
- `DayNightEvent`: Cambio día/noche

### Eventos de Red

#### Connection Events
- `PlayerConnectEvent`: Jugador conecta
- `PlayerDisconnectEvent`: Jugador desconecta
- `NetworkErrorEvent`: Error de red

#### Replication Events
- `EntitySpawnEvent`: Crear entidad
- `EntityDespawnEvent`: Eliminar entidad
- `ComponentUpdateEvent`: Actualizar componente

## Estados del Juego

### GameState Enum
```
MainMenu        → Menú principal
Connecting      → Conectando al servidor
Loading         → Cargando mundo
Playing         → En juego
Paused          → Pausado (solo SP)
Disconnected    → Desconectado
```

### Transiciones de Estado
- Los sistemas se activan/desactivan según estado
- Recursos se cargan/liberan en transiciones
- UI cambia según estado actual

## Optimizaciones Bevy

### Query Optimization

#### Change Detection
- Usar `Changed<T>` para detectar cambios
- `Added<T>` para nuevos componentes
- Evitar queries pesadas cada frame

#### Query Filtering
- Filtrar por componentes marker
- Usar `With<T>` y `Without<T>`
- Cachear queries complejas

### Parallel Systems

#### Criterios para Paralelización
- Sin dependencias mutuas
- No modifican mismos componentes
- Operaciones independientes

#### Sistemas Paralelizables
- Actualización de física por chunk
- Procesamiento de IA por entidad
- Cálculos de visibilidad
- Generación de terreno

### Memory Management

#### Object Pooling
- Pool de proyectiles
- Pool de efectos visuales
- Pool de entidades temporales

#### Component Storage
- Usar componentes pequeños
- Evitar allocaciones en hot path
- Preferir arrays sobre vecs cuando posible

## Integración con Bevy Replicon

### Configuración de Replicación

#### Componentes Replicados
- Marcar con atributo de replicación
- Definir estrategia de sincronización
- Configurar prioridad de red

#### Reglas de Replicación
- Distancia máxima de replicación
- Frecuencia de actualización
- Compresión de datos

### Client Prediction

#### Movimiento Predictivo
- Cliente simula localmente
- Servidor valida y corrige
- Interpolación suave

#### Rollback
- Guardar estados anteriores
- Revertir en caso de discrepancia
- Re-simular con datos correctos

## Herramientas de Desarrollo

### Debug Systems

#### Inspector
- `bevy-inspector-egui` para runtime
- Modificar componentes en vivo
- Visualizar jerarquía de entidades

#### Profiling
- Métricas de sistemas
- Frame time breakdown
- Memory usage

### Hot Reload

#### Assets
- Modelos y texturas
- Configuraciones Ron/JSON
- Shaders

#### Limitaciones
- No se puede hot reload código Rust
- Requiere estructura compatible

## Mejores Prácticas

### Diseño de Componentes
1. Mantener componentes pequeños y enfocados
2. Preferir composición sobre componentes grandes
3. Evitar lógica en componentes
4. Usar tipos primitivos cuando posible

### Diseño de Sistemas
1. Un sistema = una responsabilidad
2. Ordenar por dependencias
3. Minimizar queries por sistema
4. Aprovechar paralelización

### Performance
1. Perfil antes de optimizar
2. Usar change detection
3. Batch operaciones similares
4. Evitar sistemas que corren cada frame sin necesidad

---

Esta arquitectura ECS permite un desarrollo modular y escalable, aprovechando al máximo las capacidades de Bevy.