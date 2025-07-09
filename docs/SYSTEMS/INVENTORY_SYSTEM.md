# INVENTORY SYSTEM - TRUST-NO-1

## Visión General

El sistema de inventario de TRUST-NO-1 utiliza un diseño grid-based inspirado en juegos como Escape From Tarkov, donde cada item ocupa un espacio físico específico y el peso afecta directamente al gameplay.

## Diseño Core

### Grid-Based Layout

#### Estructura de Grid
- Tamaño variable según contenedor
- Celdas de tamaño uniforme
- Items ocupan múltiples celdas
- Rotación de items permitida (90°)

#### Tipos de Contenedores
```
Inventario Personal:
- Mochila: Variable (6x6 hasta 12x12)
- Bolsillos: 2x2 (x4 bolsillos)
- Rig Táctico: Variable según tipo
- Secure Container: 2x2 hasta 3x3

Almacenamiento:
- Cofres: 6x4 hasta 12x8
- Armarios: 10x20
- Vehículos: Variable por tipo
- Stash Personal: Expandible
```

### Sistema de Peso

#### Cálculo de Peso
- Peso base del contenedor
- Suma de items contenidos
- Afecta velocidad de movimiento
- Afecta consumo de stamina

#### Niveles de Carga
```
Ligero (0-20kg):     Sin penalización
Medio (20-40kg):     -10% velocidad, +20% stamina drain
Pesado (40-60kg):    -25% velocidad, +50% stamina drain  
Sobrecargado (60kg+): -50% velocidad, +100% stamina drain, no sprint
```

## Gestión de Items

### Propiedades de Items

#### Dimensiones
- Ancho en celdas (1-6)
- Alto en celdas (1-8)
- Rotable o no
- Forma irregular permitida

#### Atributos Comunes
- Peso en kg
- Durabilidad (si aplica)
- Stack máximo
- Valor base
- Categoría y sub-categoría

### Categorías de Items

#### Armas
- Rifles: 5x2 a 6x2
- Pistolas: 2x1 a 3x2
- Snipers: 6x2 a 7x2
- Melee: Variable

#### Equipamiento
- Cascos: 2x2 a 3x3
- Chalecos: 3x3 a 4x4
- Mochilas: 3x3 a 5x5
- Rigs: 3x2 a 4x3

#### Consumibles
- Medicina: 1x1 a 2x3
- Comida: 1x1 a 2x2
- Bebida: 1x2 a 1x3
- Boosters: 1x1

#### Recursos
- Componentes: Variable
- Materiales: 1x1 stackable
- Herramientas: 1x2 a 3x3
- Electrónicos: Variable

## Interacciones

### Operaciones Básicas

#### Drag & Drop
- Preview de posición válida
- Highlight de espacio ocupado
- Auto-rotación si no cabe
- Swap automático si es posible

#### Quick Actions
- Double-click: Equipar/Usar
- Right-click: Menú contextual
- Shift-click: Transfer rápido
- Ctrl-click: Split stack

### Validaciones

#### Al Añadir Items
1. Verificar espacio disponible
2. Comprobar peso total
3. Validar restricciones del contenedor
4. Confirmar ownership

#### Al Mover Items
1. Verificar destino válido
2. Mantener atomicidad
3. Actualizar peso
4. Sincronizar con servidor

## Sistema de Loot

### Generación de Loot

#### Loot Tables
- Probabilidades por categoría
- Rareza de items
- Condición/durabilidad variable
- Cantidad para stackables

#### Spawn Points
```
Militar:      Armas, munición, equipamiento táctico
Médico:       Medicinas, vendajes, instrumentos
Industrial:   Herramientas, componentes, materiales
Residencial:  Comida, ropa, items básicos
Especial:     Items únicos, alta rareza
```

### Contenedores de Loot

#### Tipos
- Cajas de armas
- Botiquines médicos
- Cajas de herramientas
- Mochilas abandonadas
- Cadáveres de jugadores

#### Respawn
- Timer por tipo de contenedor
- Influenciado por actividad de jugadores
- Eventos especiales con mejor loot

## Equipamiento

### Slots de Equipamiento

#### Slots Principales
- Arma Primaria (espalda)
- Arma Secundaria (pistolera)
- Arma Melee (cinturón)
- Casco
- Chaleco/Armor
- Mochila
- Rig Táctico

#### Slots Secundarios
- Guantes
- Máscara/Gafas
- Auriculares
- Brazalete (identificación)

### Modificadores de Equipamiento

#### Bonificaciones
- Protección por zona
- Capacidad de carga extra
- Slots adicionales
- Resistencias especiales

#### Penalizaciones
- Reducción de velocidad
- Mayor ruido al moverse
- Reducción de ergonomía
- Limitación de visión

## Trading e Intercambio

### Trade Direct

#### Proceso
1. Iniciación de trade
2. Colocar items en ventana
3. Confirmación mutua
4. Validación servidor
5. Intercambio atómico

#### Restricciones
- Distancia máxima 2 metros
- No en combate
- Items no dañados criticamente

### Dropping Items

#### Mecánica
- Drop individual o contenedor
- Física realista al caer
- Tiempo de despawn (30 min)
- Visible para otros jugadores

## Persistencia

### Guardado de Estado

#### Información Guardada
- Posición de cada item
- Durabilidad actual
- Attachments en armas
- Custom data (inscripciones, etc)

#### Frecuencia
- Auto-save cada 30 segundos
- Save inmediato en acciones importantes
- Backup antes de raids

### Secure Container

#### Propiedades Especiales
- Items no se pierden al morir
- Tamaño limitado
- Restricciones de items
- No weapons, solo valuables

## Optimizaciones

### Network

#### Sincronización Parcial
- Solo cambios visibles
- Compresión de movimientos
- Batch de operaciones
- Delta updates

#### Priorización
- Inventario propio: Alta
- Inventarios cercanos: Media
- Loot containers: Baja

### Performance

#### Culling
- No procesar inventarios lejanos
- LOD para items dropped
- Instancing para items comunes

#### Caching
- Preview de drag precalculado
- Espacios válidos cacheados
- Peso total incremental

## UI/UX

### Visualización

#### Grid Visual
- Líneas de grid sutiles
- Highlight on hover
- Colores por estado
- Iconos de alta calidad

#### Información de Items
- Tooltip detallado
- Comparación rápida
- Estado de durabilidad
- Peso y valor

### Sorting y Filtros

#### Auto-Sort
- Por categoría
- Por tamaño
- Por valor
- Optimización de espacio

#### Búsqueda
- Por nombre
- Por categoría
- Por propiedades
- Highlighting de resultados

## Casos Especiales

### Containers Anidados

#### Reglas
- Máximo 2 niveles de anidación
- No recursión infinita
- Peso se propaga hacia arriba
- Límites de performance

### Items Especiales

#### Armas con Attachments
- Guardado de configuración
- Preview con attachments
- Peso combinado
- Tamaño variable

#### Stacks
- Merge automático
- Split con slider
- Límites por tipo
- Transferencia parcial

---

Este sistema de inventario proporciona profundidad táctica mientras mantiene una interfaz intuitiva y performance óptimo.