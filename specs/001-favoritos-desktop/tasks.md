# Tasks: Favoritos Desktop

**Input**: Design documents from `/specs/001-favoritos-desktop/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Incluidos según constitution.md (80% cobertura para código de negocio)

**Organization**: Tareas agrupadas por user story para implementación y testing independientes.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Puede ejecutarse en paralelo (diferentes archivos, sin dependencias)
- **[Story]**: A qué user story pertenece (US1, US2, US3, US4, US5)
- Incluye rutas exactas de archivos en descripciones

## Path Conventions

- **Frontend**: `src/` (SvelteKit)
- **Backend**: `src-tauri/src/` (Rust)
- **Tests**: `tests/` (Vitest frontend, cargo test backend)
- **Components UI**: `src/lib/components/ui/` (Shadcn-Svelte)
- **Components Dominio**: `src/lib/components/favoritos/`
- **Stores**: `src/lib/stores/`
- **Services**: `src/lib/services/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Inicialización del proyecto y estructura básica

- [X] **T001** Crear estructura de directorios del proyecto según quickstart.md
- [X] **T002** Inicializar proyecto Tauri v2 + SvelteKit con dependencias base
- [X] **T003** [P] Configurar Tailwind CSS v3.4+ con paleta de colores fija en `tailwind.config.js`
- [X] **T004** [P] Instalar y configurar Shadcn-Svelte en `src/lib/components/ui/`
- [X] **T005** [P] Configurar TypeScript estricto en `tsconfig.json`
- [X] **T006** [P] Configurar Vitest para testing frontend en `vitest.config.ts`
- [X] **T007** [P] Configurar Playwright para E2E testing

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Infraestructura core que DEBE estar completa antes de CUALQUIER user story

**⚠️ CRITICAL**: Ningún trabajo de user story puede comenzar hasta que esta fase esté completa

### Backend Rust (Tauri)

- [X] **T008** Crear módulo `config.rs` en `src-tauri/src/` para gestión de configuración segura (keychain)
- [X] **T009** Crear módulo `api.rs` en `src-tauri/src/` para cliente HTTP de API externa
- [X] **T010** Crear módulo `cache.rs` en `src-tauri/src/` para gestión de caché de favicons
- [X] **T011** Crear módulo `tray.rs` en `src-tauri/src/` para lógica de System Tray
- [X] **T012** Crear módulo `notifications.rs` en `src-tauri/src/` para notificaciones nativas
- [X] **T013** Crear módulo `commands.rs` en `src-tauri/src/` con definición de comandos Tauri

### Frontend TypeScript

- [X] **T014** Crear store `configuracion.ts` en `src/lib/stores/` para estado de configuración
- [X] **T015** Crear servicio `api-client.ts` en `src/lib/services/` para llamadas HTTP a API
- [X] **T016** Crear servicio `tauri.ts` en `src/lib/services/` como wrapper de comandos IPC
- [X] **T017** Crear servicio `favicons.ts` en `src/lib/services/` para gestión de caché de favicons

### Base de Datos Local

- [X] **T018a** [P] Crear servicio `database.ts` en `src/lib/services/` para persistencia en IndexedDB (caché de lista de favoritos para modo offline)

### Tipos Compartidos

- [X] **T018** Crear `types/api.ts` en `src/lib/types/` con interfaces TypeScript de entidades (Favorito, Carpeta, Etiqueta, etc.)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Configuración Inicial de API (Priority: P1) 🎯 MVP

**Goal**: Usuario puede configurar API Key y URL base para empezar a usar la app

**Independent Test**: Iniciar app por primera vez → redirige a config → ingresar credenciales válidas → validación exitosa → persiste al reiniciar

**Acceptance**: FR-001, FR-002, FR-003, FR-004

### Tests for User Story 1

> **NOTE: Escribir estos tests PRIMERO, asegurar que FALLEN antes de implementación**

- [X] **T019** [P] [US1] Test unitario de `config_get` retorna null cuando no hay config en `tests/unit/config.test.ts`
- [X] **T020** [P] [US1] Test unitario de `config_save` guarda y recupera credenciales en `tests/unit/config.test.ts`
- [X] **T021** [P] [US1] Test unitario de `config_validate` con respuesta exitosa en `tests/unit/config.test.ts`
- [X] **T022** [P] [US1] Test de integración de flujo completo de configuración en `tests/integration/config-flow.test.ts`

### Backend Implementation for US1

- [X] **T023** [US1] Implementar comando `config_get` en `src-tauri/src/commands.rs`
- [X] **T024** [US1] Implementar comando `config_save` en `src-tauri/src/commands.rs` usando keychain
- [X] **T025** [US1] Implementar comando `config_validate` en `src-tauri/src/commands.rs` llamando a `/health`
- [X] **T026** [US1] Implementar comando `config_clear` en `src-tauri/src/commands.rs`

### Frontend Implementation for US1

- [X] **T027** [P] [US1] Crear componente `ConfigForm.svelte` en `src/lib/components/favoritos/` con campos URL y API Key
- [X] **T028** [P] [US1] Crear componente `ConfigDialog.svelte` en `src/lib/components/favoritos/` para modal de config
- [X] **T029** [US1] Crear página `+page.svelte` en `src/routes/(dashboard)/configuracion/` con formulario de config
- [X] **T030** [US1] Implementar lógica de validación de conexión en `ConfigForm.svelte` (check verde/error)
- [X] **T031** [US1] Implementar redirección automática a config en `src/routes/+layout.svelte` si no hay credenciales
- [X] **T032** [US1] Implementar persistencia de config usando Tauri keychain en `src/lib/stores/configuracion.ts`

**Checkpoint**: User Story 1 fully functional - Configuración completa y persistente

---

## Phase 4: User Story 2 - Gestión de Favoritos CRUD (Priority: P1) 🎯 MVP

**Goal**: Usuario puede crear, ver, editar y eliminar favoritos

**Independent Test**: Crear favorito → aparece en lista → editar → cambios se reflejan → eliminar → desaparece

**Acceptance**: FR-005, FR-006, FR-007, FR-008, FR-009, FR-010, FR-011, FR-012, FR-014

### Tests for US2

- [X] **T033** [P] [US2] Test unitario de `favorites_create` en `tests/unit/favoritos.test.ts`
- [X] **T034** [P] [US2] Test unitario de `favorites_get` con paginación en `tests/unit/favoritos.test.ts`
- [X] **T035** [P] [US2] Test unitario de `favorites_update` en `tests/unit/favoritos.test.ts`
- [X] **T036** [P] [US2] Test unitario de `favorites_delete` en `tests/unit/favoritos.test.ts`
- [X] **T037** [P] [US2] Test unitario de `metadata_fetch` en `tests/unit/metadata.test.ts`
- [X] **T038** [P] [US2] Test de integración de CRUD completo en `tests/integration/favoritos-crud.test.ts`

### Backend Implementation for US2

- [X] **T039** [US2] Implementar comando `favorites_get` en `src-tauri/src/commands.rs` con paginación
- [X] **T040** [US2] Implementar comando `favorites_get_by_id` en `src-tauri/src/commands.rs`
- [X] **T041** [US2] Implementar comando `favorites_create` en `src-tauri/src/commands.rs`
- [X] **T042** [US2] Implementar comando `favorites_update` en `src-tauri/src/commands.rs`
- [X] **T043** [US2] Implementar comando `favorites_delete` en `src-tauri/src/commands.rs`
- [X] **T044** [US2] Implementar comando `metadata_fetch` en `src-tauri/src/commands.rs`
- [X] **T045** [US2] Implementar comando `favorites_visit` en `src-tauri/src/commands.rs` (incrementa contador + abre URL)
- [X] **T046** [US2] Implementar comando `favorites_open_url` en `src-tauri/src/commands.rs`
- [X] **T047** [US2] Implementar comandos `folders_get` y `folders_create` en `src-tauri/src/commands.rs`
- [X] **T048** [US2] Implementar comandos `tags_get` y `tags_create` en `src-tauri/src/commands.rs`
- [X] **T049** [US2] Implementar comandos de caché `cache_favicon_get`, `cache_favicon_set` en `src-tauri/src/commands.rs`

### Frontend Implementation for US2

- [X] **T050** [P] [US2] Crear store `favoritos.ts` en `src/lib/stores/` con lógica de sincronización API ↔ IndexedDB para modo offline
- [X] **T051** [P] [US2] Crear componente `FavoritoCard.svelte` en `src/lib/components/favoritos/` con manejo de clic para llamar a `favorites_visit` (incrementar contador y abrir URL)
- [X] **T052** [P] [US2] Crear componente `FavoritoGrid.svelte` en `src/lib/components/favoritos/` con layout de grid
- [X] **T053** [P] [US2] Crear componente `FavoritoDialog.svelte` en `src/lib/components/favoritos/` para crear/editar
- [X] **T053a** [US2] Crear componente `DeleteConfirmDialog.svelte` en `src/lib/components/favoritos/` para confirmación de borrado
- [X] **T054** [P] [US2] Crear componente `FolderSelector.svelte` en `src/lib/components/favoritos/` con opción "[+] Nueva Carpeta"
- [X] **T055** [P] [US2] Crear componente `TagInput.svelte` en `src/lib/components/favoritos/` para crear tags con Enter
- [X] **T056** [US2] Implementar lazy loading en `FavoritoGrid.svelte` con Intersection Observer
- [X] **T057** [US2] Crear página principal `+page.svelte` en `src/routes/(dashboard)/` con grid de favoritos
- [X] **T058** [US2] Implementar obtención de metadata al pegar URL en `FavoritoDialog.svelte`
- [X] **T059** [US2] Implementar caché de favicons en `src/lib/services/favicons.ts`
- [X] **T060** [US2] Implementar atajo `Ctrl+N` para abrir diálogo de nuevo favorito
- [X] **T061** [US2] Implementar atajo `Esc` para cerrar diálogos
- [X] **T062** [US2] Implementar atajo `Ctrl+F` para enfocar buscador principal en `src/routes/(dashboard)/+page.svelte`
- [X] **T063** [US2] Implementar filtering de campos `isArchived` e `isRead` en respuestas API (ignorar en frontend)

**Checkpoint**: User Story 2 fully functional - CRUD completo de favoritos

---

## Phase 5: User Story 3 - Búsqueda Global (Priority: P2)

**Goal**: Usuario puede buscar y abrir favoritos rápidamente sin abrir la app principal (Alt+Space)

**Independent Test**: App ejecutándose → Alt+Space → ventana abre → escribir → filtrado instantáneo → Enter → abre en navegador

**Acceptance**: FR-013, FR-015

### Tests for US3

- [X] **T071** [P] [US3] Test unitario de filtrado local <50ms en `tests/unit/search.test.ts`
- [X] **T072** [P] [US3] Test de integración de búsqueda global en `tests/integration/search-global.test.ts`

### Backend Implementation for US3

- [X] **T073** [US3] Implementar comando `window_show_search` en `src-tauri/src/commands.rs`

### Frontend Implementation for US3

- [X] **T074** [P] [US3] Crear componente `BuscadorGlobal.svelte` en `src/lib/components/favoritos/` usando Shadcn Command
- [X] **T075** [US3] Crear página `+page.svelte` en `src/routes/busqueda-global/` para ventana de búsqueda
- [X] **T076** [US3] Implementar filtrado local instantáneo (<50ms) en `BuscadorGlobal.svelte` (sin debounce o <50ms para búsqueda en memoria)
- [X] **T077** [US3] Configurar ventana de búsqueda en `src-tauri/tauri.conf.json` (sin decoraciones, always on top)
- [X] **T078** [US3] Implementar atajo global `Alt+Space` en `src-tauri/src/` para abrir búsqueda
- [X] **T079** [US3] Implementar apertura de favorito con Enter en `BuscadorGlobal.svelte`

**Checkpoint**: User Story 3 fully functional - Búsqueda global tipo Spotlight

---

## Phase 6: User Story 4 - Integración con Sistema (Priority: P2)

**Goal**: App se comporta como nativa con System Tray y cierre inteligente

**Independent Test**: App abierta → X → minimiza al tray → doble clic icono → restaura → clic derecho Salir → cierra completamente

**Acceptance**: FR-016, FR-017, FR-018, FR-019, FR-025

### Tests for US4

- [X] **T080** [P] [US4] Test de integración de System Tray en `tests/integration/system-tray.test.ts`
- [X] **T081** [P] [US4] Test de notificaciones nativas en `tests/integration/notifications.test.ts`

### Backend Implementation for US4

- [X] **T082** [US4] Implementar comando `window_hide_main` en `src-tauri/src/commands.rs`
- [X] **T083** [US4] Implementar comando `app_show_notification` en `src-tauri/src/commands.rs`
- [X] **T084** [US4] Implementar comando `app_quit` en `src-tauri/src/commands.rs`
- [X] **T085** [US4] Configurar System Tray en `src-tauri/src/tray.rs` con menú contextual
- [X] **T086** [US4] Implementar menú contextual del tray: "Abrir", "Nuevo Favorito", "Salir"
- [X] **T087** [US4] Implementar cierre inteligente (X → hide) en `src-tauri/src/main.rs`
- [X] **T088** [US4] Implementar doble clic en icono del tray para restaurar ventana
- [X] **T089** [US4] Implementar notificaciones nativas para errores críticos de conexión

### Frontend Implementation for US4

- [X] **T090** [US4] Implementar atajo `Ctrl+Q` en `src/routes/+layout.svelte` para salir completamente
- [X] **T091** [US4] Implementar atajo `Ctrl+,` en `src/routes/+layout.svelte` para abrir configuración
- [X] **T092** [US4] Implementar comportamiento de `Esc` para minimizar al tray cuando no hay diálogos abiertos

**Checkpoint**: User Story 4 fully functional - Integración nativa completa

---

## Phase 7: User Story 5 - Importación/Exportación y Enriquecimiento con IA (Priority: P3)

**Goal**: Usuario puede importar/exportar favoritos y enriquecer individualmente con IA

**Independent Test**: Panel config → Importar HTML → importados con/sin IA → notificación con resumen. Exportar en JSON y HTML. Editar favorito → Enriquecer con IA → campos actualizados.

**Acceptance**: FR-022, FR-023, FR-023b, FR-028, FR-029, FR-030, FR-031

### Tests for US5

- [X] **T093** [P] [US5] Test unitario de validación en `tests/unit/import.test.ts`
- [X] **T094** [P] [US5] Test de integración de importación en `tests/integration/import.test.ts`

### Backend Implementation for US5

- [X] **T095** [US5] Implementar comando `import_favorites` en `src-tauri/src/commands.rs` — recibe archivo HTML vía multipart/form-data (`POST /bookmarks/import`) con parámetro `ia`
- [X] **T095b** [US5] Implementar `import_bookmarks` en `api.rs` con `reqwest::multipart` para enviar archivo HTML al servidor
- [X] **T095c** [US5] Implementar comando `favorites_export_html` en `commands.rs` — descarga HTML vía `GET /bookmarks/export` y abre diálogo de guardado
- [X] **T095d** [US5] Implementar `metadata_refresh` en `commands.rs` — dispara `POST /bookmarks/:id/metadata/refresh`, polling de `GET /bookmarks/:id/metadata/status` hasta `fetchStatus: "ok"`, y devuelve favorito actualizado vía `GET /bookmarks/:id`
- [X] **T095e** [US5] Implementar comandos genéricos `preference_get`/`preference_set` para persistir preferencias de UI en `settings.bin`

### Frontend Implementation for US5

- [X] **T096** [P] [US5] Crear componente `ImportDialog.svelte` — acepta archivos .html/.htm, toggle de enriquecimiento con IA, muestra 4 métricas (importados/actualizados/omitidos/total)
- [X] **T097** [US5] Implementar selector de archivo HTML en `ImportDialog.svelte`
- [X] **T098** [US5] Implementar toggle de IA en `ImportDialog.svelte` con descripción informativa
- [X] **T099** [US5] Implementar notificación con resumen (importados/actualizados/omitidos/total) tras importación
- [X] **T100** [US5] Integrar botones "Importar" y "Exportar" en página de configuración
- [X] **T101** [US5] Crear componente `ExportDialog.svelte` — selector de formato (JSON propio / HTML Netscape Bookmark)
- [X] **T102** [US5] Implementar botón "Enriquecer con IA" en `FavoritoForm.svelte` (solo modo edición) con spinner, feedback visual y detección de campos cambiados
- [X] **T103** [US5] Implementar persistencia del tipo de vista (grid/list) entre sesiones en `favoritos.ts`
- [X] **T104** [US5] Añadir campo API Key editable en `ConfigForm.svelte`
- [X] **T105** [US5] Cambiar campo "Descripción Corta" de Input a textarea multilínea en `FavoritoForm.svelte`
- [X] **T106** [US5] Añadir logs de diagnóstico en importación, exportación y enriquecimiento (Rust tracing + console del navegador)
- [X] **T107** [US5] Fix: `#[serde(rename_all = "camelCase")]` en `ConfigResponse` para correcta persistencia de apiUrl y logLevel

**Checkpoint**: User Story 5 fully functional - Importación/Exportación/Enriquecimiento IA completo

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Mejoras que afectan múltiples user stories

- [X] **T101** [P] Documentación de componentes en `src/lib/components/favoritos/README.md`
- [X] **T102** [P] Code cleanup y refactoring según best-practices.md
- [X] **T103** [P] Optimización de performance: verificar búsqueda <50ms
- [X] **T104** [P] Optimización de performance: verificar carga inicial <3s
- [X] **T105** [P] Optimización de memoria: verificar uso <100MB idle
- [X] **T106** [P] Verificar tasa de éxito de caché de favicons (objetivo: >95%) en `src/lib/services/favicons.ts`
- [X] **T107** [P] Additional unit tests para alcanzar 80% cobertura en servicios y stores
- [X] **T108** Security hardening: verificar que no hay credenciales en localStorage
- [X] **T109** [P] Validación de accesibilidad: contraste de colores 4.5:1
- [X] **T110** [P] Validación de accesibilidad: navegación por teclado completa
- [X] **T111** [P] Labels ARIA en todos los inputs y botones
- [X] **T112** Run quickstart.md validation - verificar que todo funciona
- [X] **T113** Actualizar `spec.md` con cualquier cambio de comportamiento observado
- [X] **T114** [US1] Mejorar validación de API usando endpoint protegido en `src-tauri/src/api.rs`
- [X] **T115** Configuración de nivel de log: añadir campo a `config.rs` y UI en `ConfigForm.svelte`
- [X] **T116** Aplicar nivel de log dinámico en `src-tauri/src/lib.rs`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - puede iniciar inmediatamente
- **Foundational (Phase 2)**: Depende de Setup completion - BLOQUEA todos los user stories
- **User Stories (Phases 3-7)**: Todos dependen de Foundational phase completion
  - US1 y US2 (P1) → MVP core
  - US3 y US4 (P2) → Productividad y UX
  - US5 (P3) → Feature adicional
- **Polish (Phase 8)**: Depende de todos los user stories deseados completos

### User Story Dependencies

- **US1 (Configuración)**: Puede iniciar después de Foundational - Sin dependencias de otros stories
- **US2 (CRUD)**: Puede iniciar después de Foundational - Independiente pero usa config de US1
- **US3 (Búsqueda)**: Puede iniciar después de Foundational - Usa datos de US2 pero independiente
- **US4 (Sistema)**: Puede iniciar después de Foundational - Mayormente independiente
- **US5 (Importación)**: Puede iniciar después de Foundational - Independiente

### Within Each User Story

- Tests DEBEN escribirse y FALLAR antes de implementación
- Backend commands antes de frontend services
- Servicios antes de componentes UI
- Componentes UI antes de integración
- Story completa antes de mover a siguiente prioridad

### Parallel Opportunities

```
Phase 1: T003, T004, T005, T006, T007 pueden ejecutarse en paralelo [P]

Phase 2: T008-T017 tienen dependencias, ejecutar en orden

Phase 3 (US1):
  Tests en paralelo: T019, T020, T021, T022
  Componentes en paralelo: T027, T028

Phase 4 (US2):
  Tests en paralelo: T033-T038
  Componentes en paralelo: T050, T051, T052, T053, T054, T055

Phase 5 (US3):
  Tests en paralelo: T071, T072
  Componentes: T074 puede ser paralelo a backend T073

Phase 6 (US4):
  Tests en paralelo: T080, T081
  Frontend: T090, T091, T092 pueden ser paralelos

Phase 7 (US5):
  Tests en paralelo: T093, T094
  Componentes: T096 puede ser paralelo

Phase 8:
  Todo en paralelo: T101-T111 (diferentes archivos)
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

1. Complete Phase 1: Setup (T001-T007)
2. Complete Phase 2: Foundational (T008-T018) ⚠️ **CRITICAL**
3. Complete Phase 3: US1 - Configuración (T019-T032)
4. Complete Phase 4: US2 - CRUD (T033-T061)
5. **STOP and VALIDATE**: App permite config + CRUD completo
6. Demo/Deploy si está listo

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. US1 (Config) → Test independiente → **MVP viable** (config + sin datos)
3. US2 (CRUD) → Test independiente → **MVP completo** (gestión de favoritos)
4. US3 (Búsqueda) → Test independiente → +Productividad
5. US4 (Sistema) → Test independiente → +UX nativa
6. US5 (Importación) → Test independiente → +Feature power users

---

## Notes

- [P] tasks = diferentes archivos, sin dependencias
- [Story] label mapea tarea a user story para trazabilidad
- Cada user story debe ser completado y testeable independientemente
- Verificar que tests fallen antes de implementar
- Commit después de cada tarea o grupo lógico
- Parar en cualquier checkpoint para validar story independientemente
- Evitar: tareas vagas, conflictos de mismo archivo, dependencias cross-story que rompan independencia

---

## Task Summary

| Phase | Tasks | Story | Priority |
|-------|-------|-------|----------|
| 1: Setup | 7 | - | Foundation |
| 2: Foundational | 10 | - | Foundation |
| 3: US1 Config | 14 | Configuración | P1 🎯 |
| 4: US2 CRUD | 31 | Favoritos | P1 🎯 |
| 5: US3 Search | 9 | Búsqueda | P2 |
| 6: US4 System | 13 | Integración | P2 |
| 7: US5 Import | 8 | Importación | P3 |
| 8: Polish | 13 | - | Final |
| **Total** | **113** | | |

**MVP Tasks**: 62 tasks (Phase 1-4) → Configuración + CRUD completo
**Full Product**: 113 tasks → Todas las features
