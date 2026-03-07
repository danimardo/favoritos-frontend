# Tauri Commands Contract - Favoritos Desktop

**Feature**: 001-favoritos-desktop
**Date**: 2026-03-07
**Version**: 1.0.0

## Overview

Este documento define el contrato IPC entre el frontend (TypeScript/Svelte) y el backend (Rust) a través de comandos Tauri.

---

## Convenio de Nomenclatura

- **Comandos**: `snake_case` en Rust, se invocan con el mismo nombre desde TypeScript
- **Errores**: Todos retornan `Result<T, String>` donde el string es el mensaje de error
- **Async**: Los comandos que hacen I/O deben ser `async`

---

## Comandos de Configuración

### 1. config_get

Obtiene la configuración almacenada de API.

```rust
#[tauri::command]
async fn config_get() -> Result<ApiConfig, String>
```

**Return Type**:
```typescript
interface ApiConfig {
    apiUrl: string | null;
    apiKey: string | null;
}
```

**Frontend Call**:
```typescript
import { invoke } from '@tauri-apps/api/core';

const config = await invoke<ApiConfig>('config_get');
```

---

### 2. config_save

Guarda la configuración de API en el keychain.

```rust
#[tauri::command]
async fn config_save(api_url: String, api_key: String) -> Result<(), String>
```

**Parameters**:
| Nombre | Tipo | Descripción |
|-------|------|-------------|
| `api_url` | string | URL base de la API |
| `api_key` | string | API Key para autenticación |

**Validation**:
- `api_url` debe ser una URL válida (http/https)
- `api_key` no debe estar vacío

**Frontend Call**:
```typescript
await invoke('config_save', {
    apiUrl: 'https://api.example.com',
    apiKey: 'sk-1234567890'
});
```

---

### 3. config_clear

Elimina la configuración almacenada.

```rust
#[tauri::command]
async fn config_clear() -> Result<(), String>
```

---

### 4. config_validate

Valida la configuración actual contra el endpoint `/health` de la API.

```rust
#[tauri::command]
async fn config_validate() -> Result<HealthStatus, String>
```

**Return Type**:
```typescript
interface HealthStatus {
    status: 'ok' | 'error';
    message: string;
    timestamp?: string;
}
```

---

## Comandos de Favoritos

### 5. favorites_get

Obtiene la lista de favoritos con paginación.

```rust
#[tauri::command]
async fn favorites_get(
    page: u32,
    limit: u32,
    search: Option<String>,
    folder: Option<String>,
    tags: Option<String>,
    sort: String,
    order: String
) -> Result<FavoritesResponse, String>
```

**Return Type**:
```typescript
interface FavoritesResponse {
    data: Favorito[];
    pagination: {
        page: number;
        limit: number;
        total: number;
        totalPages: number;
    };
}

interface Favorito {
    id: string;
    url: string;
    title: string;
    description?: string;
    domain: string;
    faviconUrl?: string;
    faviconCached?: string;
    tags: string[];
    folder?: string;
    visitCount: number;
    createdAt: string;
    updatedAt: string;
}
```

---

### 6. favorites_get_by_id

Obtiene un favorito específico por ID.

```rust
#[tauri::command]
async fn favorites_get_by_id(id: String) -> Result<Favorito, String>
```

---

### 7. favorites_create

Crea un nuevo favorito.

```rust
#[tauri::command]
async fn favorites_create(data: CreateFavoritoData) -> Result<Favorito, String>
```

**Parameters**:
```typescript
interface CreateFavoritoData {
    url: string;
    title: string;
    description?: string;
    tags?: string[];
    folder?: string;
}
```

---

### 8. favorites_update

Actualiza un favorito existente.

```rust
#[tauri::command]
async fn favorites_update(id: String, data: UpdateFavoritoData) -> Result<Favorito, String>
```

**Parameters**:
```typescript
interface UpdateFavoritoData {
    url?: string;
    title?: string;
    description?: string;
    tags?: string[];
    folder?: string;
}
```

---

### 9. favorites_delete

Elimina un favorito.

```rust
#[tauri::command]
async fn favorites_delete(id: String) -> Result<(), String>
```

---

### 10. favorites_visit

Registra una visita al favorito e incrementa el contador.

```rust
#[tauri::command]
async fn favorites_visit(id: String) -> Result<u32, String>
```

**Return**: Nuevo valor de `visitCount`

**Side Effect**: Abre la URL en el navegador predeterminado

---

### 11. favorites_open_url

Abre una URL en el navegador predeterminado.

```rust
#[tauri::command]
async fn favorites_open_url(url: String) -> Result<(), String>
```

---

## Comandos de Metadata

### 12. metadata_fetch

Obtiene metadata de una URL sin crear el favorito.

```rust
#[tauri::command]
async fn metadata_fetch(url: String) -> Result<UrlMetadata, String>
```

**Return Type**:
```typescript
interface UrlMetadata {
    url: string;
    title: string;
    faviconUrl: string;
    domain: string;
}
```

---

## Comandos de Caché

### 13. cache_favicon_get

Obtiene un favicon desde la caché local.

```rust
#[tauri::command]
async fn cache_favicon_get(domain: String) -> Result<Option<String>, String>
```

**Return**: Ruta local del favicon o `null` si no está cacheado

---

### 14. cache_favicon_set

Guarda un favicon en la caché local.

```rust
#[tauri::command]
async fn cache_favicon_set(domain: String, url: String) -> Result<String, String>
```

**Return**: Ruta local donde se guardó el favicon

---

### 15. cache_favicon_clear

Limpia toda la caché de favicons.

```rust
#[tauri::command]
async fn cache_favicon_clear() -> Result<(), String>
```

---

### 16. cache_clear_all

Limpia toda la caché (favicons y otros datos temporales).

```rust
#[tauri::command]
async fn cache_clear_all() -> Result<(), String>
```

---

## Comandos de Carpetas

### 17. folders_get

Obtiene la lista de carpetas.

```rust
#[tauri::command]
async fn folders_get() -> Result<Vec<Carpeta>, String>
```

**Return Type**:
```typescript
interface Carpeta {
    id: string;
    name: string;
    order: number;
    createdAt: string;
}
```

---

### 18. folders_create

Crea una nueva carpeta.

```rust
#[tauri::command]
async fn folders_create(name: String, order: Option<u32>) -> Result<Carpeta, String>
```

---

## Comandos de Etiquetas

### 19. tags_get

Obtiene la lista de etiquetas.

```rust
#[tauri::command]
async fn tags_get() -> Result<Vec<Etiqueta>, String>
```

**Return Type**:
```typescript
interface Etiqueta {
    id: string;
    name: string;
    color?: string;
}
```

---

### 20. tags_create

Crea una nueva etiqueta.

```rust
#[tauri::command]
async fn tags_create(name: String, color: Option<String>) -> Result<Etiqueta, String>
```

---

## Comandos de Importación

### 21. import_favorites

Importa favoritos desde un archivo JSON.

```rust
#[tauri::command]
async fn import_favorites(file_path: String) -> Result<ImportResult, String>
```

**Return Type**:
```typescript
interface ImportResult {
    imported: number;
    skipped: number;
    errors: string[];
}
```

---

## Comandos de Sistema

### 22. window_show_search

Muestra la ventana de búsqueda global.

```rust
#[tauri::command]
async fn window_show_search() -> Result<(), String>
```

---

### 23. window_hide_main

Oculta la ventana principal (minimiza al tray).

```rust
#[tauri::command]
async fn window_hide_main() -> Result<(), String>
```

---

### 24. app_show_notification

Muestra una notificación nativa del sistema.

```rust
#[tauri::command]
async fn app_show_notification(
    title: String,
    body: String,
    icon: Option<String>
) -> Result<(), String>
```

---

### 25. app_quit

Cierra completamente la aplicación.

```rust
#[tauri::command]
async fn app_quit() -> Result<(), String>
```

---

## Manejo de Errores

Todos los comandos retornan `Result<T, String>`. El frontend debe manejar los errores:

```typescript
try {
    const result = await invoke('favorites_get', { page: 1, limit: 50 });
    // Procesar resultado
} catch (error) {
    // Mostrar error al usuario
    console.error('[Tauri Command Error]', error);
    showErrorNotification(error as string);
}
```

### Códigos de Error Comunes

| Error | Descripción |
|-------|-------------|
| `API_KEY_NOT_SET` | No hay configuración de API |
| `INVALID_URL` | URL inválida |
| `NETWORK_ERROR` | Error de conexión con la API |
| `CACHE_ERROR` | Error al acceder a la caché |
| `PERMISSION_DENIED` | Permiso denegado del sistema |

---

## Type Definitions (TypeScript)

```typescript
// types/tauri-commands.ts

export interface ApiConfig {
    apiUrl: string | null;
    apiKey: string | null;
}

export interface HealthStatus {
    status: 'ok' | 'error';
    message: string;
    timestamp?: string;
}

export interface Favorito {
    id: string;
    url: string;
    title: string;
    description?: string;
    domain: string;
    faviconUrl?: string;
    faviconCached?: string;
    tags: string[];
    folder?: string;
    visitCount: number;
    createdAt: string;
    updatedAt: string;
}

export interface CreateFavoritoData {
    url: string;
    title: string;
    description?: string;
    tags?: string[];
    folder?: string;
}

export interface UpdateFavoritoData {
    url?: string;
    title?: string;
    description?: string;
    tags?: string[];
    folder?: string;
}

export interface FavoritesResponse {
    data: Favorito[];
    pagination: {
        page: number;
        limit: number;
        total: number;
        totalPages: number;
    };
}

export interface UrlMetadata {
    url: string;
    title: string;
    faviconUrl: string;
    domain: string;
}

export interface Carpeta {
    id: string;
    name: string;
    order: number;
    createdAt: string;
}

export interface Etiqueta {
    id: string;
    name: string;
    color?: string;
}

export interface ImportResult {
    imported: number;
    skipped: number;
    errors: string[];
}
```

---

## Registro de Comandos

En `src-tauri/src/lib.rs` o `main.rs`:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Configuración
            config_get,
            config_save,
            config_clear,
            config_validate,
            // Favoritos
            favorites_get,
            favorites_get_by_id,
            favorites_create,
            favorites_update,
            favorites_delete,
            favorites_visit,
            favorites_open_url,
            // Metadata
            metadata_fetch,
            // Caché
            cache_favicon_get,
            cache_favicon_set,
            cache_favicon_clear,
            cache_clear_all,
            // Carpetas
            folders_get,
            folders_create,
            // Etiquetas
            tags_get,
            tags_create,
            // Importación
            import_favorites,
            // Sistema
            window_show_search,
            window_hide_main,
            app_show_notification,
            app_quit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
