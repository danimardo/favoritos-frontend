# Research - Favoritos Desktop

**Feature**: 001-favoritos-desktop
**Date**: 2026-03-07
**Status**: Complete

## Overview

Este documento documenta las decisiones técnicas tomadas para la implementación de Favoritos Desktop, basadas en la constitución del proyecto y las mejores prácticas de la industria.

---

## Decisiones Técnicas

### 1. Arquitectura de Comunicación Frontend ↔ Backend

**Decisión**: Usar comandos Tauri IPC con tipado TypeScript

**Racional**:
- Tauri v2 proporciona un sistema de IPC type-safe con `@tauri-apps/api/core`
- Permite llamada asíncrona desde TypeScript a funciones Rust
- Mejor rendimiento que WebSockets para este caso de uso
- Más simple que GraphQL o gRPC para una app de escritorio

**Alternativas consideradas**:
- **WebSockets**: Rechazado - Sobrecarga innecesaria para una app local
- **HTTP local**: Rechazado - Añade latencia y complejidad de servidor

**Implementación**:
```typescript
// Frontend (TypeScript)
import { invoke } from '@tauri-apps/api/core';
const favoritos = await invoke('get_favoritos', { token: apiToken });
```

```rust
// Backend (Rust)
#[tauri::command]
async fn get_favoritos(token: String) -> Result<Vec<Favorito>, String> {
    // ...
}
```

---

### 2. Gestión de Estado en Frontend

**Decisión**: Svelte stores con patrón de servicios

**Racional**:
- Svelte stores son nativos del framework y reactivos
- Permiten separación clara entre estado UI y lógica de negocio
- Fáciles de testear aisladamente
- Compatible con la separación de responsabilidades de la constitución

**Alternativas consideradas**:
- **SvelteKit load function**: Rechazado - Solo para carga inicial de routes
- **Context API**: Rechazado - No es nativo de Svelte

**Implementación**:
```typescript
// stores/favoritos.ts
import { writable } from 'svelte/store';
import { apiClient } from '$lib/services/api-client';

export const favoritosStore = writable<Favorito[]>([]);

export const loadFavoritos = async () => {
    const data = await apiClient.getFavoritos();
    favoritosStore.set(data);
};
```

---

### 3. Caché de Favicons

**Decisión**: Sistema de caché local en directorio temporal de Tauri

**Racional**:
- Tauri proporciona acceso al directorio temp del SO
- Permite persistencia entre sesiones
- Evita peticiones repetitivas a servicios de favicon
- Se puede limpiar automáticamente si se detecta corrupción

**Alternativas consideradas**:
- **Base de datos (SQLite)**: Rechazado - Overkill para archivos simples
- **LocalStorage**: Rechazado - Límite de tamaño, no soporta binarios eficientemente

**Implementación**:
```rust
// Backend (Rust)
use tauri::api::path::temp_dir();
use std::path::PathBuf;

fn get_cache_dir() -> PathBuf {
    temp_dir().join("favoritos-cache").join("favicons")
}
```

---

### 4. Almacenamiento Seguro de Credenciales

**Decisión**: Keychain/Secret Store nativo vía Tauri

**Racional**:
- Tauri v2 ofrece `@tauri-apps/plugin-credentials` o alternativas
- Usa el keychain del SO (Windows Credential Manager, macOS Keychain, etc.)
- Cumple con el requisito de la constitución de almacenamiento seguro
- No expone credenciales en texto plano

**Alternativas consideradas**:
- **LocalStorage**: Rechazado - Prohibido por la constitución (inseguro)
- **Archivo encriptado local**: Rechazado - Más complejo y menos seguro

**Implementación**:
```typescript
import * as credential from '@tauri-apps/plugin-credential';

// Guardar
await credential.setPassword('api-config', 'api-key', apiKey);

// Leer
const entry = await credential.getPassword('api-config', 'api-key');
```

---

### 5. Búsqueda Global (Alt+Space)

**Decisión**: Segunda ventana Tauri sin decoraciones

**Racional**:
- Tauri soporta múltiples ventanas
- Permite ventana "always on top" centrada
- Shadcn Command component es ideal para este caso de uso
- Independiente de la ventana principal

**Alternativas consideradas**:
- **Dialogo modal**: Rechazado - Bloquea la ventana principal
- **Dropdown dentro de la app**: Rechazado - Requiere abrir la app primero

**Implementación**:
```rust
// tauri.conf.json - configurar ventana secundaria
{
  "windows": [{
    "label": "search",
    "title": "Buscar Favoritos",
    "width": 600,
    "height": 400,
    "decorations": false,
    "alwaysOnTop": true,
    "center": true,
    "visible": false
  }]
}
```

---

### 6. System Tray

**Decisión**: Tauri System Tray API con menú contextual

**Racional**:
- Soporte nativo en Tauri v2
- Permite minimizar en lugar de cerrar
- Menú contextual para acciones rápidas
- Compatible con Windows, macOS y Linux

**Alternativas consideradas**:
- **Minimizar a barra de tareas**: Rechazado - No cumple requisito de constitución

**Implementación**:
```rust
use tauri::{SystemTray, SystemTrayMenu, SystemTrayMenuItem};

fn tray_menu() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("open", "Abrir Favoritos"))
        .add_item(CustomMenuItem::new("new", "Nuevo Favorito"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Salir"));

    SystemTray::new().with_menu(menu)
}
```

---

### 7. Lazy Loading de Favoritos

**Decisión**: Paginación virtual con Intersection Observer

**Racional**:
- Permite manejar grandes colecciones (10,000+ items)
- No bloquea la UI durante carga
- Mejor UX que scroll infinito tradicional
- Compatible con SvelteKit

**Alternativas consideradas**:
- **Cargar todo al inicio**: Rechazado - No escala a 10,000 items
- **Scroll infinito tradicional**: Rechazado - Puede causar problemas de memoria

**Implementación**:
```typescript
import { onMount } from 'svelte';

let page = 1;
const pageSize = 50;
let loading = false;

const loadMore = async () => {
    if (loading) return;
    loading = true;
    const newItems = await apiClient.getFavoritos(page, pageSize);
    favoritosStore.update(current => [...current, ...newItems]);
    page++;
    loading = false;
};

onMount(() => {
    // Intersection Observer implementation
});
```

---

### 8. Validación de Configuración de API

**Decisión**: Llamada al endpoint `/health` con timeout

**Racional**:
- Endpoint estándar para verificación de conectividad
- Timeout previene bloqueos
- Feedback visual inmediato al usuario

**Alternativas consideradas**:
- **Solo validación de formato de URL**: Rechazado - No confirma conectividad real

---

### 9. Importación de JSON

**Decisión**: Validación de esquema + importación por lotes

**Racional**:
- Permite importar grandes volúmenes (1,000+ items)
- Validación previa previerte corrupción de datos
- Notificación con resumen da feedback al usuario

**Implementación**:
```rust
#[tauri::command]
async fn import_favoritos(
    file_path: String,
    api_token: String
) -> Result<ImportResult, String> {
    // 1. Leer y validar JSON
    // 2. Enviar a API por lotes
    // 3. Retornar resumen (importados, saltados, errores)
}
```

---

### 10. Paleta de Colores en Tailwind

**Decisión**: Extender tema de Tailwind con colores personalizados

**Racional**:
- Mantiene consistencia en toda la app
- Fácil de mantener
- Shadcn-Svelte usa Tailwind, se integra directamente

**Implementación**:
```javascript
// tailwind.config.js
export default {
  theme: {
    extend: {
      colors: {
        taupe: '#483C32',
        greyOlive: '#A9A9A1',
        silver: '#C0C0C0',
        parchment: '#F5F5DC',
        powderBlush: '#E6E6FA'
      }
    }
  }
};
```

---

## Patrones de Mejores Prácticas

### Error Handling

```typescript
try {
    const result = await invoke('comando', { params });
} catch (error) {
    // Log para debugging
    console.error('[Command Error]', error);
    // Notificación al usuario
    showErrorNotification(error.message || 'Error en la operación');
}
```

### Logging

```rust
// Rust
eprintln!("[Error] {}: {}", context, error);
println!("[Info] Operation completed: {}", result);

// TypeScript
console.log('[ApiService] Fetching favorites...');
console.error('[ApiService] Failed to fetch:', error);
```

### Testing Strategy

| Capa | Herramienta | Cobertura Objetivo |
|------|-------------|-------------------|
| Servicios TypeScript | Vitest | 80%+ |
| Comandos Rust | cargo test | 80%+ |
| Componentes UI | Testing Library | Opcional |
| E2E | Playwright | Flujos críticos |

---

## Dependencias Externas

| Dependencia | Versión | Propósito |
|-------------|---------|-----------|
| `@tauri-apps/api` | ^2.x | Comunicación IPC |
| `@tauri-apps/plugin-credential` | ^2.x | Almacenamiento seguro |
| `@sveltejs/kit` | latest | Framework frontend |
| `shadcn-svelte` | latest | Componentes UI |
| `tailwindcss` | ^3.4 | Estilos |
| `lucide-svelte` | latest | Iconos |
| `vitest` | latest | Testing frontend |
| `playwright` | latest | E2E testing |

---

## Riesgos y Mitigaciones

| Riesgo | Impacto | Mitigación |
|--------|---------|------------|
| API externa no disponible | Alto | Offline read, reintentos exponenciales |
| Corrupción de caché | Medio | Detección y reconstrucción automática |
| Performance con 10k items | Medio | Lazy loading, virtualización |
| Compatibilidad SO cruzado | Bajo | Test en Windows prioritario, otros después |

---

## Referencias

- [Tauri v2 Documentation](https://v2.tauri.app/)
- [SvelteKit Docs](https://kit.svelte.dev/)
- [Shadcn-Svelte](https://www.shadcn-svelte.com/)
- [Tailwind CSS](https://tailwindcss.com/)
