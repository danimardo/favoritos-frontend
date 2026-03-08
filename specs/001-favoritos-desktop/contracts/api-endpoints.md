# API Endpoints Contract - Favoritos API

**Feature**: 001-favoritos-desktop
**Date**: 2026-03-07
**Version**: 1.0.0

## Overview

Este documento define el contrato entre la aplicación Favoritos Desktop y la API REST de Favoritos. La aplicación consume estos endpoints para todas las operaciones de datos.

---

## Base Configuration

| Propiedad | Valor |
|-----------|-------|
| **Base URL** | Configurable por usuario (stored in keychain) |
| **Protocol** | HTTPS |
| **Authentication** | Bearer Token (API Key) |
| **Content-Type** | `application/json` |
| **Timeout** | 30 seconds |

---

## Headers Estándar

```http
Authorization: Bearer {api_key}
Content-Type: application/json
User-Agent: Favoritos-Desktop/1.0
```

---

## Endpoints

### 1. Health Check

Verifica que la API está disponible y que las credenciales son válidas.

```http
GET /health
```

**Request**:
```http
Authorization: Bearer {api_key}
```

**Response 200 OK**:
```json
{
  "status": "ok",
  "timestamp": "2026-03-07T10:00:00Z"
}
```

**Error 401 Unauthorized**:
```json
{
  "error": "invalid_api_key",
  "message": "La API Key proporcionada no es válida"
}
```

---

### 2. Listar Favoritos

Obtiene la lista de favoritos del usuario con soporte para paginación.

```http
GET /bookmarks
```

**Query Parameters**:

| Parámetro | Tipo | Default | Descripción |
|-----------|------|---------|-------------|
| `page` | number | 1 | Número de página |
| `limit` | number | 50 | Items por página (máx: 100) |
| `search` | string | null | Filtro de búsqueda (título, URL, tags) |
| `folder` | string | null | Filtrar por carpeta |
| `tags` | string | null | Filtrar por etiquetas (comma-separated) |
| `sort` | string | createdAt | Campo de ordenamiento |
| `order` | string | desc | Dirección: asc/desc |

**Request**:
```http
GET /bookmarks?page=1&limit=50&sort=createdAt&order=desc
Authorization: Bearer {api_key}
```

**Response 200 OK**:
```json
{
  "success": true,
  "data": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "url": "https://example.com",
      "title": "Ejemplo",
      "description": "Un sitio de ejemplo",
      "domain": "example.com",
      "faviconUrl": "https://example.com/favicon.ico",
      "tags": ["tech", "news"],
      "folder": "Work",
      "visitCount": 5,
      "isArchived": false,
      "isRead": true,
      "createdAt": "2026-01-01T00:00:00Z",
      "updatedAt": "2026-03-07T10:00:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "limit": 50,
    "total": 150,
    "totalPages": 3
  }
}
```

**Error 401 Unauthorized**: API Key inválida
**Error 500 Internal Server Error**: Error del servidor

---

### 3. Obtener Favorito por ID

```http
GET /bookmarks/{id}
```

**Response 200 OK**:
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "url": "https://example.com",
    "title": "Ejemplo",
    "description": "Un sitio de ejemplo",
    "domain": "example.com",
    "faviconUrl": "https://example.com/favicon.ico",
    "tags": ["tech", "news"],
    "folder": "Work",
    "visitCount": 5,
    "isArchived": false,
    "isRead": true,
    "createdAt": "2026-01-01T00:00:00Z",
    "updatedAt": "2026-03-07T10:00:00Z"
  }
}
```

**Error 404 Not Found**: Favorito no existe

---

### 4. Crear Favorito

```http
POST /bookmarks
```

**Request Body**:
```json
{
  "url": "https://example.com",
  "title": "Ejemplo",
  "description": "Un sitio de ejemplo",
  "tags": ["tech", "news"],
  "folder": "Work"
}
```

**Response 201 Created**:
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "url": "https://example.com",
    "title": "Ejemplo",
    "description": "Un sitio de ejemplo",
    "domain": "example.com",
    "faviconUrl": "https://example.com/favicon.ico",
    "tags": ["tech", "news"],
    "folder": "Work",
    "visitCount": 0,
    "isArchived": false,
    "isRead": false,
    "createdAt": "2026-03-07T10:00:00Z",
    "updatedAt": "2026-03-07T10:00:00Z"
  }
}
```

**Error 400 Bad Request**: Datos inválidos
**Error 409 Conflict**: URL ya existe

---

### 5. Actualizar Favorito

```http
PUT /bookmarks/{id}
```

**Request Body** (todos los campos opcionales):
```json
{
  "url": "https://example.com/updated",
  "title": "Ejemplo Actualizado",
  "description": "Nueva descripción",
  "tags": ["tech"],
  "folder": "Personal"
}
```

**Response 200 OK**: Retorna el favorito actualizado

**Error 404 Not Found**: Favorito no existe

---

### 6. Eliminar Favorito

```http
DELETE /bookmarks/{id}
```

**Response 204 No Content**

**Error 404 Not Found**: Favorito no existe

---

### 7. Incrementar Contador de Visitas

```http
POST /bookmarks/{id}/visit
```

**Response 200 OK**:
```json
{
  "success": true,
  "data": {
    "visitCount": 6
  }
}
```

---

### 8. Obtener Metadata de URL

Obtiene el título y favicon de una URL sin crear el favorito.

```http
GET /metadata?url={url}
```

**Request**:
```http
GET /metadata?url=https%3A%2F%2Fexample.com
Authorization: Bearer {api_key}
```

**Response 200 OK**:
```json
{
  "success": true,
  "data": {
    "url": "https://example.com",
    "title": "Example Domain",
    "faviconUrl": "https://example.com/favicon.ico",
    "domain": "example.com"
  }
}
```

**Error 400 Bad Request**: URL inválida
**Error 503 Service Unavailable**: No se pudo obtener metadata

---

### 9. Listar Carpetas

```http
GET /folders
```

**Response 200 OK**:
```json
{
  "success": true,
  "data": [
    {
      "id": "folder-1",
      "name": "Work",
      "order": 1,
      "createdAt": "2026-01-01T00:00:00Z"
    },
    {
      "id": "folder-2",
      "name": "Personal",
      "order": 2,
      "createdAt": "2026-01-02T00:00:00Z"
    }
  ]
}
```

---

### 10. Crear Carpeta

```http
POST /folders
```

**Request Body**:
```json
{
  "name": "Projects",
  "order": 3
}
```

**Response 201 Created**:
```json
{
  "success": true,
  "data": {
    "id": "folder-3",
    "name": "Projects",
    "order": 3,
    "createdAt": "2026-03-07T10:00:00Z"
  }
}
```

---

### 11. Listar Etiquetas

```http
GET /tags
```

**Response 200 OK**:
```json
{
  "success": true,
  "data": [
    {
      "id": "tag-1",
      "name": "tech",
      "color": "#E6E6FA"
    },
    {
      "id": "tag-2",
      "name": "news",
      "color": "#A9A9A1"
    }
  ]
}
```

---

### 12. Crear Etiqueta

```http
POST /tags
```

**Request Body**:
```json
{
  "name": "dev",
  "color": "#483C32"
}
```

**Response 201 Created**: Retorna la etiqueta creada

---

### 13. Importar Favoritos

```http
POST /import
```

**Request Body** (formato de archivo):
```json
{
  "favorites": [
    {
      "url": "https://example.com",
      "title": "Ejemplo",
      "description": "Descripción",
      "tags": ["tech"],
      "folder": "Work"
    }
  ]
}
```

**Response 200 OK**:
```json
{
  "success": true,
  "data": {
    "imported": 85,
    "skipped": 15,
    "errors": []
  }
}
```

---

## Errores Estándar

Todos los errores siguen este formato:

```json
{
  "error": "error_code",
  "message": "Descripción legible del error",
  "details": {}
}
```

### Códigos de Error Comunes

| Código | HTTP | Descripción |
|--------|------|-------------|
| `invalid_api_key` | 401 | API Key inválida o expirada |
| `invalid_request` | 400 | Parámetros de solicitud inválidos |
| `not_found` | 404 | Recurso no encontrado |
| `conflict` | 409 | Recurso ya existe (ej: URL duplicada) |
| `rate_limit_exceeded` | 429 | Demás solicitudes |
| `server_error` | 500 | Error interno del servidor |

---

## Notas de Implementación

1. **Campos Ignorados por el Frontend**:
   - `isArchived`: Se recibe de la API pero no se muestra en la UI
   - `isRead`: Se recibe de la API pero no se muestra en la UI

2. **Manejo de Timeouts**:
   - Para requests de metadata, usar timeout de 10 segundos
   - Para otras operaciones, usar timeout de 30 segundos

3. **Reintentos**:
   - Implementar reintentos exponenciales para errores 5xx
   - Máximo 3 reintentos

4. **Offline**:
   - Cachear respuestas exitosas para lectura offline
   - Mostrar indicador de "modo offline" cuando la API no está disponible
