# Favoritos Desktop Constitution

## Core Principles

### I. Stack Tecnológico (NEGOCIABLE)
**Descripción**: La aplicación usará Tauri v2 + SvelteKit como framework principal, con Tailwind CSS y Shadcn-Svelte para la interfaz.

**Versiones obligatorias**:
- **Tauri**: v2.8.0+ (v2 en package.json `@tauri-apps/cli: ^2`)
- **SvelteKit**: v2.9.0+ (`@sveltejs/kit: ^2.9.0`)
- **Svelte**: v5.0.0+ (`svelte: ^5.0.0`)
- **Tailwind CSS**: v4.2.1+ (`tailwindcss: ^4.2.1`)
- **TypeScript**: v5.6.2+ (`typescript: ~5.6.2`)
- **Rust**: 1.70+ (backend)
- **Node.js**: v20+ (build)
- **Shadcn-Svelte**: vía `bits-ui` v2.16.3+
- **Lucide Icons**: integrados en Shadcn

**Excepciones**: Solo con aprobación explícita.

---

### II. Diseño Visual (NO NEGOCIABLE)
**Descripción**: La aplicación DEBE mantener una paleta de colores fija, SIN modo claro/oscuro.

**Paleta obligatoria**:
| Color | Hex | Uso |
|-------|-----|-----|
| **Taupe** | `#483C32` | Fondos principales, barras laterales |
| **Grey Olive** | `#A9A9A1` | Textos secundarios, bordes sutiles |
| **Silver** | `#C0C0C0` | Detalles, iconos desactivados |
| **Parchment** | `#F5F5DC` | Fondo de tarjetas, áreas de contenido |
| **Powder Blush** | `#E6E6FA` | Destacados, botones de acción primaria |

**Implementación**: Colores definidos en `tailwind.config.js` como tema fijo.

---

### III. Separación de Responsabilidades (NO NEGOCIABLE)
**Descripción**: La arquitectura DEBE mantener una separación estricta entre lógica de negocio y presentación.

**Capas obligatorias**:
```
src/
├── lib/
│   ├── components/
│   │   ├── ui/              # Shadcn-Svelte (PRESENTACIÓN)
│   │   └── favoritos/      # Componentes de dominio (PRESENTACIÓN)
│   ├── stores/              # Estado reactivo (LÓGICA)
│   ├── services/           # Abstracción API/IPC (LÓGICA)
│   └── types/               # TypeScript interfaces (CONTRATOS)
src-tauri/src/
    ├── commands.rs          # Handlers IPC (PUENTE)
    ├── api.rs               # Cliente HTTP (LÓGICA)
    ├── config.rs            # Gestión config (LÓGICA)
    ├── cache.rs             # Caché favicons (LÓGICA)
    └── tray.rs              # System Tray (INTEGRACIÓN)
```

**Reglas**:
- Componentes Svelte NUNCA llaman directamente a `fetch` o `window.__TAURI__`
- Siempre usar servicios中介 (services layer)
- Stores contienen lógica de estado, NO lógica de negocio pesada

---

### IV. Almacenamiento de Datos (NO NEGOCIABLE)
**Descripción**: El almacenamiento DEBE seguir reglas específicas según el tipo de dato.

| Tipo de Dato | Método de Almacenamiento | Ubicación |
|--------------|-------------------------|-----------|
| **Credenciales API** (api_key, base_url) | Tauri password store / keychain | Seguro |
| **Configuración app** | Tauri store o localStorage | Persistente |
| **Caché favicons** | Directorio temporal Tauri | Transient |

**Reglas**:
- NUNCA almacenar credenciales en localStorage
- NUNCA exponer API key en código frontend
- Siempre pasar por comandos Tauri para operaciones sensibles

---

### V. Testing (NO NEGOCIABLE)
**Descripción**: El código DEBE mantener cobertura mínima de pruebas.

**Requisitos**:
- **Cobertura mínima**: 80% para código de negocio (servicios, stores)
- **Framework frontend**: Vitest (`vitest: ^4.0.18`)
- **Framework E2E**: Playwright (`@playwright/test: ^1.58.2`)
- **Backend**: cargo test

**Reglas**:
- Tests unitarios ANTES de implementación (TDD)
- Tests de integración para flujos completos
- Tests E2E para user stories completas

---

### VI. UX Minimalista (NO NEGOCIABLE)
**Descripción**: La interfaz DEBE seguir principios de minimalismo y productividad.

**Reglas**:
- Usar EXCLUSIVAMENTE componentes Shadcn-Svelte
- NO crear componentes custom que duplicen funcionalidad Shadcn
- Iconografía: Lucide Icons (vía Shadcn)
- Inspiración: Notion / Raycast / VS Code

---

### VII. Accesibilidad WCAG (NO NEGOCIABLE)
**Descripción**: La aplicación DEBE ser accesible para usuarios con diferentes capacidades.

**Requisitos**:
- Contraste de colores mínimo: 4.5:1
- Navegación por teclado completa (todos los atajos)
- Labels ARIA en todos los inputs y botones
- Atajos de teclado documentados

**Atajos obligatorios**:
| Atajo | Acción |
|-------|--------|
| `Alt + Space` | Búsqueda global |
| `Ctrl + N` | Nuevo favorito |
| `+` | Nuevo favorito (fuera de inputs) |
| `Ctrl + F` | Enfocar buscador |
| `Ctrl + ,` | Abrir configuración |
| `Ctrl + Q` | Salir de la app |
| `Esc` | Cerrar diálogo abierto, o minimizar al tray |

---

### VIII. Rendimiento (NO NEGOCIABLE)
**Descripción**: La aplicación DEBE cumplir objetivos de rendimiento.

**Métricas obligatorias**:
- Búsqueda local: < 50ms
- Ventana búsqueda global: < 200ms
- Carga inicial (100 items): < 3s
- Memoria idle: < 100MB
- Scroll: 60fps con 10,000 favoritos
- Caché favicons: > 95% éxito

**Técnicas requeridas**:
- Lazy loading con Intersection Observer
- Filtrado en memoria (no API)
- Caché de favicons

---

### IX. Integración Nativa (NO NEGOCIABLE)
**Descripción**: La aplicación DEBE comportarse como app nativa.

**Features obligatorias**:
- System Tray persistente
- Cierre inteligente (X → hide, no close)
- Doble clic en tray restaura ventana
- Menú contextual: "Abrir", "Nuevo Favorito", "Salir"
- Notificaciones nativas para errores críticos, importaciones y exportaciones

---

## Constraints Adicionales

### Datos Ignorados en Frontend
- Los campos `isArchived` e `isRead` del modelo de datos DEBEN ser ignorados por el frontend para simplificar la interfaz.

### Offline
- La aplicación debe permitir lectura de datos ya cargados cuando la API no está disponible.

### Plataforma
- **Prioritaria**: Windows 10/11
- **Futuro**: macOS y Linux

---

## Governance

**Version**: 1.1.0 | **Ratified**: 2026-03-07 | **Last Amended**: 2026-03-15

**Reglas**:
- Esta constitución SUPERA cualquier otra práctica o preferencia individual
- Enmiendas requieren documentación, aprobación y plan de migración
- Todas las PRs/reviews deben verificar cumplimiento
- Complejidad debe ser justificada

---

## Artifactos de Referencia

- Especificación: `specs/001-favoritos-desktop/spec.md`
- Plan: `specs/001-favoritos-desktop/plan.md`
- Tareas: `specs/001-favoritos-desktop/tasks.md`
- Historias: `historias.md`
- Normas para Agentes: `CLAUDE.md`, `GEMINI.md`, `AGENT_GUIDELINES.md`
