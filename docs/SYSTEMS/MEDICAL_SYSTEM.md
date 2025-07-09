# MEDICAL SYSTEM - TRUST-NO-1

## Visión General

El sistema médico de TRUST-NO-1 simula lesiones realistas, tratamientos médicos complejos y gestión de la salud del personaje, añadiendo profundidad táctica a la supervivencia.

## Anatomía del Jugador

### Sistema de Salud por Zonas

#### Zonas Corporales
```
Cabeza:     100 HP - Crítica
Tórax:      150 HP - Crítica  
Estómago:   120 HP - Vital
Brazo Izq:  80 HP  - No vital
Brazo Der:  80 HP  - No vital
Pierna Izq: 90 HP  - No vital
Pierna Der: 90 HP  - No vital
```

#### Estados Globales
```
Sangre:        5000ml (máximo)
Hidratación:   100 (0-100)
Energía:       100 (0-100)
Temperatura:   36.5°C (35-40°C)
Consciencia:   100% (0-100%)
Dolor:         0 (0-100)
Toxicidad:     0 (0-100)
```

### Zonas Críticas vs No Vitales

#### Zonas Críticas
- Cabeza/Tórax a 0 HP = Muerte instantánea
- Daño afecta consciencia directamente
- Sangrado más severo
- Prioridad de tratamiento

#### Zonas No Vitales
- 0 HP = Extremidad inutilizable
- Penalizaciones específicas
- Posible amputación
- Muerte por desangrado

## Tipos de Heridas

### Heridas por Proyectil

#### Entrada/Salida
- Entrada: Sangrado moderado
- Salida: Sangrado severo
- Sin salida: Bala alojada
- Fragmentación: Múltiples heridas

#### Severidad
```
Pistola:        Leve-Moderada
Rifle:          Moderada-Severa
Sniper:         Severa-Crítica
Escopeta:       Variable (distancia)
```

### Heridas por Melee

#### Tipos
```
Corte:          Sangrado externo
Punzante:       Sangrado interno
Contundente:    Contusión/Fractura
```

#### Profundidad
- Superficial: Solo sangrado leve
- Profunda: Daño a órganos
- Crítica: Hemorragia severa

### Fracturas

#### Tipos
```
Simple:         Movilidad reducida 50%
Compuesta:      Movilidad 0%, sangrado
Múltiple:       Incapacitación total
```

#### Localización
- Brazo: No puede apuntar estable
- Pierna: Velocidad -70%, no sprint
- Costillas: Stamina -50%

### Quemaduras

#### Grados
```
Primer Grado:   Dolor leve, -10 HP
Segundo Grado:  Dolor moderado, -30 HP
Tercer Grado:   Dolor severo, -60 HP, infección
```

#### Efectos
- Deshidratación acelerada
- Vulnerabilidad a infecciones
- Cicatrices permanentes

## Condiciones Médicas

### Sangrado

#### Tipos
```
Leve:           -30ml/min
Moderado:       -60ml/min
Severo:         -120ml/min
Arterial:       -240ml/min
```

#### Efectos por Pérdida de Sangre
```
4000-5000ml:    Normal
3000-4000ml:    Mareo leve, -10% stamina
2000-3000ml:    Visión túnel, -30% stamina
1000-2000ml:    Desmayos, -60% stamina
<1000ml:        Muerte inminente
```

### Infecciones

#### Etapas
```
Etapa 1:        Sin síntomas (0-2h)
Etapa 2:        Fiebre leve (2-6h)
Etapa 3:        Fiebre alta, debilidad (6-12h)
Etapa 4:        Sepsis, muerte probable (12h+)
```

#### Causas
- Heridas sin tratar
- Vendajes sucios
- Agua contaminada
- Exposición a cadáveres

### Shock

#### Tipos
```
Hipovolémico:   Por pérdida de sangre
Neurogénico:    Por dolor extremo
Séptico:        Por infección severa
```

#### Síntomas
- Pulso acelerado
- Sudoración fría
- Confusión
- Pérdida de consciencia

### Fracturas

#### Efectos Inmediatos
- Dolor intenso
- Movilidad comprometida
- Posible sangrado interno
- Shock neurogénico

#### Complicaciones
- Mal alineamiento
- Infección (fractura abierta)
- Daño nervioso
- Necrosis

## Items Médicos

### Vendajes y Hemostáticos

#### Vendaje Básico
```
Uso: Detener sangrado leve
Tiempo: 3 segundos
Efectividad: 30% reducción
Usos: 1
```

#### Vendaje de Presión
```
Uso: Sangrado moderado
Tiempo: 5 segundos
Efectividad: 50% reducción
Usos: 1
```

#### Gasa Hemostática
```
Uso: Sangrado severo
Tiempo: 7 segundos
Efectividad: 70% reducción
Usos: 2
```

#### Torniquete
```
Uso: Sangrado arterial (extremidades)
Tiempo: 4 segundos
Efectividad: 100% stop
Daño: 1 HP/min a la extremidad
```

### Medicamentos

#### Analgésicos

##### Aspirina
```
Efecto: -20 dolor
Duración: 300 segundos
Toxicidad: +5
Hidratación: -5
```

##### Morfina
```
Efecto: -80 dolor
Duración: 600 segundos
Toxicidad: +30
Efectos: Visión borrosa, -20% precisión
```

#### Antibióticos

##### Amoxicilina
```
Efecto: Reduce infección 40%
Dosis: 3 requeridas
Intervalo: 1 hora
Toxicidad: +10 por dosis
```

##### Antibióticos IV
```
Efecto: Cura infección 80%
Tiempo: 30 segundos aplicar
Requiere: Skill médico alto
```

#### Estimulantes

##### Adrenalina
```
Efecto: +50 consciencia, +100 stamina
Duración: 120 segundos
Crash: -30 stamina después
Toxicidad: +20
```

##### Combat Stim
```
Efecto: -30% daño recibido, +20% velocidad
Duración: 180 segundos
Crash: Agotamiento severo
Toxicidad: +40
```

### Kits Médicos

#### IFAK (Individual First Aid Kit)
```
Contenido: Vendajes x3, Morfina x1
Heal: +50 HP zona
Tiempo: 5 segundos
Usos: 3
```

#### Trauma Kit
```
Contenido: Todo para trauma mayor
Heal: +85 HP, para sangrado severo
Trata: Fracturas simples
Tiempo: 8 segundos
Usos: 5
```

#### Kit Quirúrgico
```
Uso: Cirugía de campo
Trata: Balas alojadas, fracturas complejas
Heal: 100% zona
Tiempo: 15 segundos
Requiere: Skill alto + superficie estable
```

### Items Especializados

#### Bolsa de Sangre
```
Restaura: +1000ml sangre
Tiempo: 30 segundos
Requiere: Tipo compatible o universal
Riesgo: Infección si no estéril
```

#### Desfibrilador
```
Uso: Revivir muerte reciente (<2 min)
Éxito: 60% primer intento, -20% cada siguiente
Requiere: Batería cargada
Daño: Posibles quemaduras
```

#### Splint
```
Uso: Inmovilizar fracturas
Efecto: Permite movimiento limitado
Duración: Hasta curación completa
Penalización: -30% velocidad
```

## Mecánicas de Aplicación

### Proceso de Curación

#### Auto-Tratamiento
- Tiempo x1.5 normal
- Efectividad -20%
- Algunas zonas inaccesibles
- Riesgo de error

#### Tratamiento por Otros
- Tiempo normal
- Efectividad completa
- Todas las zonas accesibles
- Bonus por skill médico

### Animaciones y Tiempo

#### Factores que Afectan Tiempo
- Tipo de tratamiento
- Skill del aplicador
- Condiciones (lluvia, combate)
- Equipamiento disponible

#### Interrupción
- Movimiento cancela
- Daño cancela
- Progreso parcial perdido
- Item puede perderse

## Sistema de Skills Médicos

### Niveles de Habilidad

```
Novato (0-25):      Vendajes básicos
Entrenado (26-50):  Medicamentos, IFAK
Experto (51-75):    Trauma, cirugía simple
Maestro (76-100):   Cirugía compleja, bonus velocidad
```

### Ganancia de Experiencia

```
Vendaje aplicado:           +5 XP
Hemorragia detenida:       +10 XP
Fractura tratada:          +20 XP
Cirugía exitosa:           +50 XP
Vida salvada:             +100 XP
```

### Bonificaciones por Skill

```
Velocidad:      +1% por 10 skill
Efectividad:    +0.5% por 5 skill  
Diagnóstico:    Información extra
Economía:       -1 uso kits por 50 skill
```

## Estados y Efectos

### Efectos Positivos

#### Analgesia
- Reducción de dolor
- Permite acciones normales
- Duración variable

#### Regeneración
- +1-5 HP/min según medicamento
- Consume energía extra
- Stack limitado

#### Inmunidad Temporal
- Post-antibióticos
- Resistencia a infección
- 2 horas duración

### Efectos Negativos

#### Adicción
- Uso repetido de opioides
- Necesidad creciente
- Síndrome de abstinencia

#### Sobredosis
- Toxicidad >80
- Vómitos, convulsiones
- Posible paro cardíaco

#### Alergias
- Reacción a medicamentos
- Shock anafiláctico
- Requiere epinefrina

## Muerte y Resucitación

### Proceso de Muerte

#### Consciencia 0%
1. Jugador cae inconsciente
2. Timer de 2 minutos
3. Puede ser estabilizado
4. Muerte si timer expira

#### Condiciones de Muerte Instantánea
- Cabeza/Tórax a 0 HP
- Sangre <500ml
- Temperatura <32°C o >42°C
- Toxicidad 100

### Resucitación

#### Requisitos
- <2 minutos desde muerte
- Desfibrilador con carga
- Causa de muerte tratable

#### Proceso
1. Aplicar desfibrilador
2. RCP si falla
3. Tratar causa original
4. Estabilizar vitales

#### Consecuencias
- Pérdida de skills temporales
- Debilidad extrema
- Posible daño cerebral

## Integración con Otros Sistemas

### Con Inventario
- Peso de items médicos
- Espacio ocupado
- Organización por prioridad

### Con Combate
- Aplicar mientras en cover
- Riesgo de interrupción
- Priorización de heridas

### Con Supervivencia
- Medicina afecta hambre/sed
- Infecciones por comida/agua
- Temperatura afecta curación

---

Este sistema médico añade capas de estrategia y realismo, donde la preparación y conocimiento médico pueden ser la diferencia entre la vida y la muerte.