# TRUST-NO-1 🎮

Un shooter táctico multijugador de supervivencia en mundo abierto que combina lo mejor de DayZ, Rust y Escape From Tarkov.

## 🚀 Alpha 0.0.1 - Primera Release Funcional

Esta versión alpha incluye:
- ✅ Arquitectura cliente-servidor autoritativa
- ✅ Sistema de networking con protocolo JSON
- ✅ Sincronización multijugador funcional
- ✅ Movimiento y física básica
- ✅ Etiquetas de jugador (azul local, rojo remoto)
- ✅ Cámara first-person con controles de mouse

## Visión del Proyecto

TRUST-NO-1 es un juego first-person shooter que enfatiza:
- Combate táctico realista con balística avanzada
- Supervivencia en un mundo hostil post-apocalíptico
- Construcción y defensa de bases
- Sistema de clanes y guerras territoriales
- Economía impulsada por jugadores
- Mundo persistente con ciudades y comunidades

## Características Principales

### Combate y Balística
- Física realista de proyectiles (gravedad, resistencia del aire, penetración)
- Sistema modular de armas con cientos de combinaciones
- Daño por zonas corporales específicas
- Armaduras con niveles de protección realistas

### Supervivencia
- Sistema médico complejo con heridas, enfermedades y tratamientos
- Hambre, sed, temperatura y fatiga
- Crafting extensivo desde recursos básicos hasta componentes avanzados
- Sistema de habilidades que mejoran con el uso

### Mundo Abierto
- Mapa masivo con múltiples biomas
- Ciudades generadas proceduralmente
- Eventos dinámicos del mundo
- Ciclo día/noche con efectos en gameplay

### Multijugador
- Servidores dedicados con 100+ jugadores simultáneos
- Sistema de clanes con jerarquías y roles
- Territorios conquistables y defendibles
- Comercio entre jugadores y economía emergente

## Stack Tecnológico

- **Motor**: Bevy 0.15 (Rust)
- **Networking**: bevy_replicon (servidor autoritativo)
- **Física**: Rapier3D
- **Base de Datos**: PostgreSQL + Redis
- **Backend**: Arquitectura cliente-servidor dedicada
- **Plataformas**: PC (Windows, Linux, macOS)

## Estructura del Proyecto

```
trust-no-1/
├── crates/
│   ├── tn1_shared/    # Código compartido cliente/servidor
│   ├── tn1_server/    # Servidor dedicado (headless)
│   └── tn1_client/    # Cliente del juego
├── docs/              # Documentación completa
├── assets/            # Assets del juego
└── tools/             # Herramientas de desarrollo
```

## Estado del Desarrollo

🚧 **Pre-Alpha** - En desarrollo activo

### Sistemas Implementados
- [ ] Arquitectura base cliente-servidor
- [ ] Sistema de movimiento y física
- [ ] Sistema de inventario grid-based
- [ ] Balística y combate
- [ ] Sistema médico
- [ ] Construcción modular
- [ ] Vehículos
- [ ] Sistema de clanes
- [ ] Economía y comercio

## Documentación

- [Arquitectura del Juego](docs/ARCHITECTURE/GAME_ARCHITECTURE.md)
- [Arquitectura Bevy](docs/ARCHITECTURE/BEVY_ARCHITECTURE.md)
- [Sistemas del Juego](docs/SYSTEMS/)
- [Base de Datos](DATABASE_ARCHITECTURE.md)
- [Dependencias](DEPENDENCIES.md)
- [Guía de Contribución](docs/DEVELOPMENT/CONTRIBUTION_GUIDE.md)

## Instalación para Desarrolladores

### Prerequisitos
- Rust 1.75+ 
- PostgreSQL 15+
- Redis 7+
- Git LFS (para assets)

### Setup Rápido Alpha 0.0.1
```bash
# Clonar repositorio
git clone https://github.com/tuusuario/trust-no-1.git
cd trust-no-1

# Compilar todo el proyecto
cargo build --release

# Lanzar servidor
cargo run --bin tn1-server

# En otra terminal, lanzar cliente
cargo run --bin trust-no-1
```

### Controles del Juego
- **WASD** - Movimiento
- **Espacio** - Saltar
- **Shift** - Correr
- **Mouse** - Mirar alrededor
- **Click** - Capturar cursor
- **ESC** - Liberar cursor
- **F3** - Debug info
- **F4** - Debug en consola

## Contribuir

¡Contribuciones son bienvenidas! Por favor lee nuestra [Guía de Contribución](docs/DEVELOPMENT/CONTRIBUTION_GUIDE.md) para más detalles.

### Áreas que Necesitan Ayuda
- Modelado 3D y animaciones
- Diseño de niveles y mundo
- Balance de gameplay
- Testing y QA
- Documentación

## Comunidad

- Discord: [Únete a nuestro servidor](#)
- Forum: [Discusiones](#)
- Twitter: [@TrustNo1Game](#)

## Licencia

Este proyecto está licenciado bajo GPL-3.0 - ver [LICENSE](LICENSE) para detalles.

## Agradecimientos

Inspirado por:
- DayZ - Por el concepto de supervivencia hardcore
- Rust - Por el sistema de construcción y raids
- Escape From Tarkov - Por el combate táctico y realismo

---

**TRUST NO ONE. SURVIVE EVERYTHING.**