# Agent Guidelines - Buenas Prácticas para Agentes de Codificación

Este documento establece las normas y buenas prácticas que **todos los agentes de codificación** deben seguir al trabajar en este proyecto.

---

## 1. Principios Fundamentales

### 1.1 Documentos Normativos (Orden de Prioridad)

| Prioridad | Documento | Descripción |
|-----------|----------|-------------|
| 1 | `specs/001-favoritos-desktop/spec.md` | Especificación funcional completa |
| 2 | `.specify/memory/constitution.md` | Reglas de diseño NO negociables |
| 3 | `historias.md` | Historias de usuario |
| 4 | `specs/001-favoritos-desktop/plan.md` | Plan de implementación |
| 5 | `specs/001-favoritos-desktop/tasks.md` | Tareas de desarrollo |

**Regla de oro**: Cualquier decisión de código debe alinearse con estos documentos. Si hay conflicto, consulta al usuario antes de actuar.

### 1.2 Comunicación

- 📢 **Idioma**: Español
- 📝 **Output**: Siempre en español
- ✅ **Confirmación**: Antes de cambios que alteren comportamiento, pide autorización

---

## 2. Reglas de Desarrollo

### 2.1 No Modificar Comportamiento

> *"Si no está roto, no lo arregles"*

- ❌ No hacer refactors no solicitados
- ❌ No aplicar optimizaciones que cambien efectos observables
- ❌ No modificar lógica de negocio fuera de los criterios definidos

### 2.2 Antes de Implementar

Cuando recibas una tarea poco clara:

1. **Detén**: No implementes inmediatamente
2. **Pregunta**: Aclara dudas, huecos o ambigüedades
3. **Espera**: Confirmación del usuario antes de actuar

### 2.3 Documentación Obligatoria

**Todo cambio → debe documentarse en `spec.md`**

Incluye:
- Correcciones de bugs que cambian comportamiento
- Ajustes de lógica
- Nuevas métricas
- Cambios en reglas de negocio
- Correcciones de campos

**Excepción**: Solo cuando el cambio es puramente interno (renombrar variable local, reformatear código).

---

## 3. Trazabilidad

### 3.1 Criterios de Aceptación

Al realizar cambios, **siempre referencia**:
- El ID del criterio de aceptación (FR-XX, SC-XX, US-X)
- La fuente documental (spec.md, constitution.md, historias.md)

### 3.2 Si Hay Ambigüedad

> *"Pide aclaración en lugar de asumir comportamiento"*

---

## 4. Herramientas y Recursos

### 4.1 Documentación Técnica

- **Usa Context7** para consultar APIs y librerías
- Verifica que la implementación use sintaxis moderna
- Confirma compatibilidad con las versiones del stack

### 4.2 Stack del Proyecto

| Componente | Versión |
|------------|---------|
| Tauri | v2.8.0+ |
| SvelteKit | v2.9.0+ |
| Svelte | v5.0.0+ |
| Tailwind CSS | v4.2.1+ |
| TypeScript | v5.6.2+ |
| Rust | 1.70+ |

### 4.3 GitHub

- Usa CLI `gh` para commits y pushes
- Sigue conventional commits si es posible

---

## 5. Estructura del Proyecto

```
favoritos-frontend/
├── src/                          # Frontend SvelteKit
│   ├── lib/
│   │   ├── components/ui/       # Shadcn-Svelte
│   │   ├── components/favoritos/ # Componentes dominio
│   │   ├── stores/              # Estado reactivo
│   │   ├── services/            # Lógica de negocio
│   │   └── types/               # TypeScript interfaces
├── src-tauri/                    # Backend Rust
├── tests/                        # Vitest + Playwright
├── specs/001-favoritos-desktop/  # Documentación
├── .specify/memory/              # Constitución
├── CLAUDE.md                     # Normas Claude
├── GEMINI.md                     # Normas Gemini
└── AGENT_GUIDELINES.md          # Este archivo
```

---

## 6. Paleta de Colores (NO negociable)

| Color | Hex | Uso |
|-------|-----|-----|
| Taupe | `#483C32` | Fondos principales |
| Grey Olive | `#A9A9A1` | Textos secundarios |
| Silver | `#C0C0C0` | Detalles |
| Parchment | `#F5F5DC` | Tarjetas |
| Powder Blush | `#E6E6FA` | Destacados |

---

## Resumen Visual

```
┌─────────────────────────────────────────────────────────────┐
│                    FLUJO DE TRABAJO                        │
├─────────────────────────────────────────────────────────────┤
│  1. Recibir tarea                                          │
│           ↓                                                │
│  2. Consultar documentos normativos                        │
│           ↓                                                │
│  3. ¿Clara? → Implementar                                │
│     ¿Ambigua? → Preguntar antes de actuar                 │
│           ↓                                                │
│  4. ¿Cambia comportamiento?                              │
│     Sí → Solicitar autorización                           │
│     No → Implementar                                      │
│           ↓                                                │
│  5. Documentar en spec.md                                │
│           ↓                                                │
│  6. Commit + Push con gh                                  │
└─────────────────────────────────────────────────────────────┘
```

---

**Versión**: 1.0.0 | **Fecha**: 2026-03-07
