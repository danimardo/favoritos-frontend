# Implementation Plan: Favoritos Desktop

**Branch**: `001-favoritos-desktop` | **Date**: 2026-03-07 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-favoritos-desktop/spec.md`

## Summary

Construir una aplicación de escritorio nativa que consume la API REST de Favoritos, permitiendo a los usuarios gestionar sus enlaces desde una interfaz minimalista tipo Raycast/Notion. La app usa Tauri v2 para integración nativa (System Tray, atajos globales) y SvelteKit con Shadcn-Svelte para la UI.

## Technical Context

**Language/Version**: TypeScript 5.x (Frontend), Rust 1.70+ (Backend Tauri)
**Primary Dependencies**:
- Tauri v2.8.0+ (Desktop framework)
- SvelteKit (Latest stable)
- Shadcn-Svelte (Componentes UI)
- Tailwind CSS v3.4+ (Estilos)
- Lucide Icons (Iconografía)

**Storage**:
- API Key y Base URL: Tauri password store (seguro)
- Favoritos (lista): IndexedDB (para lectura offline)
- Configuración general: localStorage o Tauri store
- Caché de favicons: Tauri app data dir (persistente)

**Testing**: Vitest (Frontend), cargo test (Rust backend)

**Target Platform**: Windows 10/11 (prioridad), macOS y Linux (futuro)

**Project Type**: desktop-app (Aplicación de escritorio híbrida)

**Performance Goals**:
- Búsqueda local: <50ms
- Apertura búsqueda global: <200ms
- Carga inicial (100 items): <3s
- Memoria idle: <100MB
- Escala: Hasta 10,000 favoritos (manteniendo 60fps)

**Constraints**:
- Sin modo claro/oscuro (paleta fija)
- Separación estricta frontend/backend
- Offline: lectura de datos ya cargados via IndexedDB
- Atajos de teclado obligatorios

**Scale/Scope**:
- Hasta 10,000 favoritos por usuario
- 5 user stories (P1: 2, P2: 2, P3: 1)
- 25 requerimientos funcionales

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principio | Estado | Notas |
|-----------|--------|-------|
| **I. Stack Tecnológico** | ✅ PASS | Tauri v2, SvelteKit, Tailwind, Shadcn-Svelte |
| **II. Separación de Responsabilidades** | ✅ PASS | Frontend (UI) → Servicios → Tauri (Backend) |
| **III. Diseño Visual** | ✅ PASS | Paleta fija Taupe/Parchment, sin modo oscuro |
| **IV. UX Minimalista** | ✅ PASS | Componentes Shadcn exclusivamente |
| **V. Integración Nativa** | ✅ PASS | System Tray, `Alt+Space`, cierre inteligente |
| **VI. Rendimiento** | ✅ PASS | Lazy loading, caché favicons, filtrado local |
| **VII. Testing (80%)** | ✅ PASS | Planificado para servicios y stores |
| **VIII. Accesibilidad WCAG** | ✅ PASS | Atajos de teclado, ARIA, contraste 4.5:1 |

**Gate Status**: ✅ **ALL PASSED** - Puede proceder a Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/001-favoritos-desktop/
├── spec.md              # Especificación funcional
├── plan.md              # Este archivo
├── research.md          # Phase 0: Decisiones técnicas
├── data-model.md        # Phase 1: Modelo de datos
├── quickstart.md        # Phase 1: Guía de inicio
├── contracts/           # Phase 1: Contratos de API
│   ├── api-endpoints.md # Endpoints de API Favoritos
│   └── tauri-commands.md # Comandos IPC Rust ↔ Frontend
└── tasks.md             # Phase 2: Tareas de implementación
```

### Source Code (repository root)

```text
favoritos-frontend/
├── src/                          # SvelteKit Frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/               # Componentes Shadcn-Svelte
│   │   │   └── favoritos/        # Componentes específicos del dominio
│   │   │       ├── FavoritoCard.svelte
│   │   │       ├── FavoritoDialog.svelte
│   │   │       └── BuscadorGlobal.svelte
│   │   ├── stores/               # Svelte stores
│   │   │   ├── favoritos.ts
│   │   │   └── configuracion.ts
│   │   └── services/             # Capa de servicios (abstracción Tauri)
│   │       ├── api-client.ts     # Cliente HTTP para API Favoritos
│   │       ├── favicons.ts       # Servicio de caché de favicons
│   │       └── tauri.ts          # Wrapper de comandos Tauri
│   ├── routes/
│   │   ├── +layout.svelte        # Layout principal
│   │   ├── (dashboard)/          # Rutas agrupadas
│   │   │   ├── +page.svelte      # Dashboard principal
│   │   │   └── configuracion/
│   │   │       └── +page.svelte  # Panel de configuración
│   │   └── busqueda-global/      # Ventana de búsqueda global
│   │       └── +page.svelte
│   └── app.html                  # HTML entry point
├── src-tauri/                    # Rust Backend
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # Lib entry
│   │   ├── commands.rs           # Comandos expuestos al frontend
│   │   ├── tray.rs               # Lógica de System Tray
│   │   ├── api.rs                # Cliente HTTP para API externa
│   │   ├── config.rs             # Gestión de configuración segura
│   │   ├── notifications.rs      # Notificaciones nativas
│   │   └── cache.rs              # Gestión de caché (favicons)
│   ├── Cargo.toml                # Dependencias Rust
│   ├── tauri.conf.json           # Configuración Tauri
│   └── icons/                    # Iconos de la app
├── tests/                        # Tests e2e (Playwright)
│   ├── unit/
│   └── integration/
├── package.json                  # Dependencias Node
├── tailwind.config.js            # Configuración Tailwind + paleta
├── svelte.config.js              # Configuración SvelteKit
└── tsconfig.json                 # Configuración TypeScript (strict)
```

**Structure Decision**: Estructura monorepo híbrida típica de Tauri. El frontend (SvelteKit) está en `src/` y el backend Rust en `src-tauri/`. Los componentes Shadcn se instalan en `src/lib/components/ui/`. Los componentes específicos del dominio van en `src/lib/components/favoritos/`.

## Complexity Tracking

> **No hay violaciones a la constitución que requieran justificación**

El proyecto respeta todos los principios arquitectónicos definidos en la constitución.

---

## Phase 0: Research ✅ COMPLETE

**Archivo**: [research.md](./research.md)

Todas las decisiones técnicas han sido documentadas:

| Decisión | Selección | Racional |
|----------|-----------|-----------|
| Comunicación Frontend↔Backend | Tauri IPC type-safe | Nativo, mejor rendimiento |
| Gestión de Estado | Svelte stores + servicios | Nativo, separación de responsabilidades |
| Caché de Favicons | Directorio temporal Tauri | Persistencia, performance |
| Almacenamiento Seguro | Keychain nativo | Seguridad (constitución) |
| Búsqueda Global | Segunda ventana Tauri | Spotlight-style, sin decoraciones |
| System Tray | Tauri SystemTray API | Integración nativa |
| Lazy Loading | Intersection Observer | Escala a 10,000 items |
| Validación API | Endpoint `/health` | Verificación estándar |
| Importación JSON | Validación + lotes | Manejo de volúmenes grandes |
| Paleta de Colores | Tailwind extendido | Consistencia en toda la app |

## Phase 1: Design ✅ COMPLETE

### Data Model

**Archivo**: [data-model.md](./data-model.md)

Entidades definidas:
- **Favorito**: Enlace con URL, título, descripción, dominio, favicon, tags, carpeta, visitCount
- **Carpeta**: Agrupación de favoritos
- **Etiqueta**: Clasificación flexible
- **Configuración**: Credenciales de API (almacenamiento seguro)
- **ImportResult**: Resultado de importación

### Contracts

**Directorio**: [contracts/](./contracts/)

| Archivo | Contenido |
|---------|-----------|
| `api-endpoints.md` | 13 endpoints de API REST de Favoritos |
| `tauri-commands.md` | 25 comandos IPC Rust ↔ TypeScript |

### Quickstart Guide

**Archivo**: [quickstart.md](./quickstart.md)

Guía completa de setup incluyendo:
- Prerrequisitos (Node.js v20, Rust 1.70+)
- Creación de proyecto Tauri + SvelteKit
- Configuración de Tailwind con paleta de colores
- Instalación de Shadcn-Svelte
 Scripts de desarrollo
- Configuración de testing (Vitest, Playwright)

### Agent Context Update

**Archivo actualizado**: `CLAUDE.md`

Se ha añadido la información del lenguaje del proyecto:
- TypeScript 5.x (Frontend)
- Rust 1.70+ (Backend Tauri)

---

## Phase 2: Tasks ✅ COMPLETE

**Archivo**: [tasks.md](./tasks.md)

Tareas generadas: **113 tareas** organizadas en 8 fases

| Fase | Tareas | Descripción |
|------|--------|-------------|
| Phase 1: Setup | 7 | Estructura y configuración base |
| Phase 2: Foundational | 10 | Infraestructura core (bloqueante) |
| Phase 3: US1 Config | 14 | Configuración inicial de API (P1) |
| Phase 4: US2 CRUD | 31 | Gestión de favoritos (P1) |
| Phase 5: US3 Search | 9 | Búsqueda global (P2) |
| Phase 6: US4 System | 13 | Integración nativa (P2) |
| Phase 7: US5 Import | 8 | Importación masiva (P3) |
| Phase 8: Polish | 12 | Pulido y cross-cutting |

**MVP Tasks (Phases 1-4)**: 60 tareas → Configuración + CRUD completo

---

## Summary

| Phase | Status | Artefactos |
|-------|--------|------------|
| **Phase 0: Research** | ✅ Complete | `research.md` |
| **Phase 1: Design** | ✅ Complete | `data-model.md`, `contracts/`, `quickstart.md` |
| **Phase 2: Tasks** | ✅ Complete | `tasks.md` (113 tareas) |

## Next Steps

### Para Iniciar Implementación

1. **Seguir quickstart.md** para configurar el proyecto
2. **Ejecutar tareas en orden** según `tasks.md`
3. **O usar** `/speckit.implement` para ejecución guiada

### Estrategia Recomendada

```
1. Phase 1: Setup (T001-T007)          → Fundamento
2. Phase 2: Foundational (T008-T018)    → Infraestructura
3. Phase 3: US1 Config (T019-T032)      → MVP viable
4. Phase 4: US2 CRUD (T033-T061)        → MVP completo 🎯
5. Phase 5+: Resto de features          → Producto completo
```

**Ready for implementation** - Plan completo, tareas generadas, listo para desarrollar.
