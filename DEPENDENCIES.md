# DEPENDENCIES - TRUST-NO-1

Dependencias completas del proyecto organizadas por categoría y propósito.

## Estructura Workspace

El proyecto usa un workspace Cargo con tres crates principales:
- `tn1_shared`: Código compartido entre cliente y servidor
- `tn1_server`: Servidor dedicado (headless)
- `tn1_client`: Cliente del juego

## Dependencias Compartidas (workspace)

### Core Engine
- **bevy** = "0.15"
  - Motor de juego ECS principal
  - Features varían entre cliente y servidor

### Serialización
- **serde** = { version = "1.0", features = ["derive"] }
  - Serialización/deserialización de datos
- **bincode** = "1.3"
  - Serialización binaria eficiente para red

### Utilidades
- **uuid** = { version = "1.10", features = ["v4", "serde"] }
  - Identificadores únicos para entidades
- **anyhow** = "1.0"
  - Manejo de errores simplificado
- **thiserror** = "1.0"
  - Definición de tipos de error custom

## Dependencias del Servidor

### Bevy Headless
```toml
bevy = { 
    version = "0.15", 
    default-features = false, 
    features = [
        "bevy_asset",
        "bevy_scene", 
        "multi_threaded",
        "serialize",
        "bevy_state",
        "bevy_time"
    ]
}
```

### Networking
- **bevy_replicon** = { version = "0.28", features = ["server"] }
  - Replicación de entidades para Bevy
- **quinn** = "0.11"
  - Protocolo QUIC (UDP confiable)
- **tokio** = { version = "1.40", features = ["full"] }
  - Runtime async para operaciones I/O

### Base de Datos
- **sqlx** = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "json", "uuid", "time", "migrate"] }
  - Acceso a PostgreSQL con queries async
- **redis** = { version = "0.27", features = ["tokio-comp", "connection-manager"] }
  - Cache y pub/sub
- **sea-orm** = { version = "1.0", features = ["sqlx-postgres", "runtime-tokio-rustls"] }
  - ORM opcional para operaciones complejas

### Física
- **bevy_rapier3d** = { version = "0.27", features = ["parallel", "simd-stable", "debug-render-3d"] }
  - Motor de física 3D para balística y colisiones

### Seguridad
- **argon2** = "0.5"
  - Hashing seguro de contraseñas
- **jsonwebtoken** = "9.3"
  - Autenticación JWT
- **rand** = "0.8"
  - Generación segura de números aleatorios

### Monitoreo
- **tracing** = "0.1"
  - Framework de logging estructurado
- **tracing-subscriber** = { version = "0.3", features = ["env-filter", "json"] }
  - Configuración de logs
- **prometheus** = "0.13"
  - Métricas del servidor
- **opentelemetry** = "0.24"
  - Tracing distribuido

### Utilidades del Servidor
- **dashmap** = "6.0"
  - HashMap concurrente para estado compartido
- **rayon** = "1.10"
  - Paralelización de cálculos
- **parking_lot** = "0.12"
  - Mutexes más rápidos que std
- **flate2** = "1.0"
  - Compresión de datos
- **chrono** = { version = "0.4", features = ["serde"] }
  - Manejo de fechas y tiempos

## Dependencias del Cliente

### Bevy Completo
```toml
bevy = { 
    version = "0.15", 
    features = [
        "dynamic_linking",  # Desarrollo más rápido
        "wayland",          # Soporte Linux
        "x11",              # Soporte Linux
        "webgl2",           # Futuro soporte web
        "serialize",
        "mp3",              # Formatos de audio
        "vorbis",
        "wav"
    ]
}
```

### Networking Cliente
- **bevy_replicon** = { version = "0.28", features = ["client"] }
  - Cliente de replicación
- **reqwest** = { version = "0.12", features = ["json", "rustls-tls"] }
  - Cliente HTTP para API REST

### UI y Input
- **bevy_egui** = "0.30"
  - UI inmediata para menús y debug
- **bevy_ui_navigation** = "0.38"
  - Navegación de UI con gamepad
- **leafwing-input-manager** = "0.15"
  - Sistema avanzado de manejo de input

### Gráficos y Efectos
- **bevy_hanabi** = "0.13"
  - Sistema de partículas GPU-based
- **bevy_atmosphere** = "0.10"
  - Cielo dinámico y atmosférico
- **bevy_terrain** = "0.3"
  - Renderizado optimizado de terreno
- **bevy_water** = "0.5"
  - Agua con shaders realistas

### Audio
- **bevy_kira_audio** = "0.20"
  - Sistema de audio avanzado
- **bevy_spatial_audio** = "0.5"
  - Audio 3D espacializado

### Utilidades del Cliente
- **bevy_asset_loader** = "0.21"
  - Carga asíncrona de assets
- **bevy_embedded_assets** = "0.11"
  - Embebir assets en el ejecutable
- **bevy_tweening** = "0.11"
  - Animaciones e interpolaciones
- **image** = { version = "0.25", features = ["jpeg", "png", "webp"] }
  - Procesamiento de imágenes

## Dependencias de Desarrollo

### Testing
- **criterion** = "0.5"
  - Benchmarking de performance
- **proptest** = "1.4"
  - Testing basado en propiedades
- **fake** = "2.9"
  - Generación de datos de prueba
- **mockall** = "0.13"
  - Mocking para tests

### Debug y Desarrollo
- **bevy-inspector-egui** = { version = "0.25", features = ["highlight_changes"] }
  - Inspector de entidades en runtime
- **bevy_prototype_debug_lines** = "0.11"
  - Dibujo de líneas de debug
- **bevy_mod_debugdump** = "0.11"
  - Visualización de grafos de sistemas

## Features del Proyecto

```toml
[features]
default = []
client = ["render", "audio", "input"]
server = ["headless", "database"]
dev = ["inspector", "debug-physics", "hot-reload"]
release = ["optimize", "strip"]
```

## Perfiles de Compilación

### Desarrollo
```toml
[profile.dev]
opt-level = 1              # Algo de optimización
debug = true               # Símbolos de debug

[profile.dev.package."*"]
opt-level = 3              # Dependencias optimizadas
```

### Release
```toml
[profile.release]
opt-level = 3              # Máxima optimización
lto = "fat"                # Link-time optimization
codegen-units = 1          # Mejor optimización
strip = true               # Remover símbolos
panic = "abort"            # Menor tamaño
```

### Distribución
```toml
[profile.dist]
inherits = "release"
lto = "fat"
strip = true
opt-level = "z"            # Optimizar para tamaño
```

## Justificación de Dependencias Clave

### bevy_replicon vs alternativas
- Diseñado específicamente para Bevy
- Integración perfecta con ECS
- Replicación automática de componentes
- Mejor que renet o quinn puro para juegos Bevy

### Rapier3D para física
- Determinístico (importante para networking)
- Performance excelente con SIMD
- Integración oficial con Bevy
- Soporta continuous collision detection

### PostgreSQL + Redis
- PostgreSQL: Datos persistentes con ACID
- Redis: Cache rápido y pub/sub
- Combinación probada en producción
- Escalable horizontalmente

### QUIC (quinn)
- Combina lo mejor de TCP y UDP
- Menor latencia que TCP
- Más confiable que UDP puro
- Multiplexing de streams

## Versiones Mínimas

- Rust: 1.75+
- PostgreSQL: 15+
- Redis: 7+
- Node.js: 20+ (para herramientas)

## Actualización de Dependencias

1. Revisar changelog de Bevy primero
2. Actualizar bevy_* plugins compatibles
3. Ejecutar tests de integración
4. Verificar compatibilidad de saves

---

**Nota**: Este documento debe actualizarse con cada cambio mayor en dependencias.