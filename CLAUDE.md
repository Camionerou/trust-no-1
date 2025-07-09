# CLAUDE.md - Guía de Desarrollo con IA para TRUST-NO-1

Este documento contiene instrucciones específicas para el desarrollo asistido por IA en el proyecto TRUST-NO-1.

## Visión General del Proyecto

TRUST-NO-1 es un shooter táctico multijugador con:
- Motor Bevy (Rust) con arquitectura ECS
- Servidor autoritativo usando bevy_replicon
- Cliente como renderizador sin lógica de gameplay
- Base de datos PostgreSQL para persistencia
- Redis para cache y estado temporal

## Arquitectura de Plugins Bevy

### Estructura de Plugin Estándar

Cada sistema del juego debe implementarse como un plugin Bevy independiente:

```
src/plugins/
├── inventory/
│   ├── mod.rs         # Plugin principal
│   ├── components.rs  # Componentes ECS
│   ├── systems.rs     # Sistemas de lógica
│   ├── events.rs      # Eventos del sistema
│   └── resources.rs   # Recursos compartidos
```

### Convenciones de Nomenclatura

**Componentes**:
- Usar sustantivos descriptivos
- Ejemplo: `Health`, `Inventory`, `WeaponStats`

**Sistemas**:
- Usar verbos que describan la acción
- Ejemplo: `update_ballistics`, `handle_damage`, `sync_inventory`

**Eventos**:
- Usar formato `[Acción]Event`
- Ejemplo: `ItemPickupEvent`, `DamageDealtEvent`

**Recursos**:
- Usar formato `[Sistema]Config` o `[Sistema]State`
- Ejemplo: `BallisticsConfig`, `NetworkState`

## Patrones ECS Requeridos

### Separación Cliente-Servidor

**Componentes Compartidos** (en `tn1_shared`):
- Todos los componentes que se replican
- Estructuras de datos comunes
- Eventos de red

**Sistemas del Servidor** (en `tn1_server`):
- Toda la lógica de gameplay
- Validación de acciones
- Gestión de estado autoritativo
- Sincronización con base de datos

**Sistemas del Cliente** (en `tn1_client`):
- Renderizado y visualización
- Predicción client-side
- Interpolación de entidades
- Manejo de input (solo envío al servidor)

### Replicación de Entidades

Componentes que DEBEN replicarse:
- Transform (posición, rotación)
- Health y estados vitales
- Información visual básica
- Estados de equipamiento visible

Componentes que NO se replican:
- Inventario completo (solo visible)
- Estadísticas detalladas
- Información privada del jugador

## Networking con bevy_replicon

### Principios de Sincronización

1. **Servidor Autoritativo**: El servidor tiene la verdad absoluta
2. **Predicción**: El cliente predice movimiento local
3. **Reconciliación**: Corregir discrepancias suavemente
4. **Priorización**: Sincronizar primero lo más importante

### Validación Server-Side

TODA acción del jugador debe validarse:
- Rango de acción válido
- Recursos suficientes
- Estado permitido
- Rate limiting
- Anti-cheat checks

## Base de Datos y Persistencia

### PostgreSQL
- Usar transacciones para operaciones críticas
- Índices en campos de búsqueda frecuente
- JSONB para datos flexibles
- UUID para identificadores

### Redis
- TTL apropiado para cada tipo de cache
- Pub/Sub para eventos en tiempo real
- Listas para colas de procesamiento
- Sets ordenados para leaderboards

## Sistemas Core del Juego

### Sistema de Inventario
- Grid-based con gestión de peso
- Validación de espacio antes de añadir items
- Sincronización parcial (solo cambios)

### Sistema de Balística
- Cálculos en el servidor
- Cliente solo muestra trayectorias
- Validación de line-of-sight
- Detección de colisiones con Rapier3D

### Sistema Médico
- Estados de salud por zona corporal
- Efectos acumulativos
- Tiempos de aplicación realistas
- Sincronización de animaciones

### Sistema de Construcción
- Validación de placement en servidor
- Sistema de permisos por clan
- Durabilidad y daño estructural
- Preview en cliente, ejecución en servidor

## Optimización y Performance

### Prioridades de Optimización
1. Minimizar tráfico de red
2. Reducir queries a base de datos
3. Cachear agresivamente
4. Paralelizar sistemas independientes

### Métricas a Monitorear
- Tick rate del servidor (objetivo: 64Hz)
- Latencia de red por región
- Uso de CPU/RAM por sistema
- Queries por segundo a DB

## Testing y Validación

### Tests Requeridos
- Unit tests para lógica core
- Integration tests para networking
- Stress tests con múltiples clientes
- Tests de seguridad y anti-cheat

### Proceso de Review
1. Verificar separación cliente-servidor
2. Confirmar validaciones server-side
3. Revisar impacto en performance
4. Comprobar sincronización correcta

## Herramientas de Debug

### Para Desarrollo
- bevy-inspector-egui para inspección
- Logs estructurados con tracing
- Métricas con Prometheus
- Visualización de física con debug lines

### Para Producción
- Monitoring con Grafana
- Alertas automáticas
- Logs centralizados
- Análisis de comportamiento

## Referencias Importantes

- [Arquitectura del Juego](docs/ARCHITECTURE/GAME_ARCHITECTURE.md)
- [Arquitectura Bevy](docs/ARCHITECTURE/BEVY_ARCHITECTURE.md)
- [Sistemas Detallados](docs/SYSTEMS/)
- [Base de Datos](DATABASE_ARCHITECTURE.md)
- [Seguridad](docs/TECHNICAL/SECURITY_MEASURES.md)

## Flujo de Trabajo Git

### Branches
- `main`: Código estable
- `develop`: Desarrollo activo
- `feature/*`: Nuevas características
- `fix/*`: Corrección de bugs

### Commits
- Mensajes descriptivos en español
- Formato: `[Sistema] Acción realizada`
- Ejemplo: `[Inventario] Añadir validación de peso`

### Pull Requests
- Descripción clara del cambio
- Tests incluidos
- Documentación actualizada
- Review por al menos un desarrollador

## Principios de Diseño

1. **Realismo Táctico**: Priorizar mecánicas realistas
2. **Fair Play**: Sin pay-to-win, solo cosmetics
3. **Profundidad**: Sistemas complejos pero intuitivos
4. **Performance**: 60+ FPS en hardware medio
5. **Escalabilidad**: Arquitectura para 100+ jugadores

## Notas Finales

- Este documento es la guía principal para desarrollo con IA
- Siempre verificar cambios contra estos principios
- Mantener consistencia con la arquitectura establecida
- Priorizar seguridad y performance sobre features

---

**Recuerda**: El servidor es la única fuente de verdad. El cliente solo renderiza.