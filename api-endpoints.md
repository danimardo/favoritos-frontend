# API REST Contracts

**Feature**: API Personal de Gestión de Favoritos Web
**Version**: v1
**Base URL**: `/api/v1`

---

## Authentication

Todas las peticiones DEBEN incluir el header:

```
X-API-Key: <tu-api-key>
```

**Excepción**: `GET /health` (público)

---

## Response Format Convention

### Éxito (Single Resource)

```json
{
  "success": true,
  "data": { /* resource object */ }
}
```

### Éxito (Listado)

```json
{
  "success": true,
  "data": [ /* array of resources */ ],
  "meta": {
    "page": 1,
    "pageSize": 20,
    "total": 125
  }
}
```

### Error

```json
{
  "success": false,
  "error": {
    "code": "BOOKMARK_DUPLICATE",
    "message": "Ya existe un bookmark activo con esa URL."
  }
}
```

---

## Endpoints

### Health Check

#### `GET /health`

**Público** (no requiere API Key)

**Response** (200 OK):
```json
{
  "status": "ok",
  "timestamp": "2026-03-07T10:00:00Z",
  "version": "1.0.0",
  "database": {
    "status": "ok",
    "latency_ms": 12
  }
}
```

**Error** (503 Service Unavailable):
```json
{
  "status": "error",
  "database": {
    "status": "error",
    "error": "Connection refused"
  }
}
```

---

### Bookmarks

#### `GET /api/v1/bookmarks`

Lista todos los bookmarks con filtros y paginación.

**Query Params**:
| Parámetro | Tipo | Default | Descripción |
|-----------|------|---------|-------------|
| `page` | number | 1 | Número de página |
| `pageSize` | number | 20 | Tamaño (máx 100) |
| `folderId` | number | - | Filtrar por carpeta |
| `tagId` | number | - | Filtrar por tag |
| `domain` | string | - | Filtrar por dominio |
| `isFavorite` | boolean | - | Filtrar destacados |
| `isArchived` | boolean | - | Filtrar archivados |
| `isRead` | boolean | - | Filtrar leídos |
| `search` | string | - | Búsqueda textual |
| `sortBy` | string | createdAt | Campo: createdAt, updatedAt, title, lastVisitedAt, visitCount |
| `sortDirection` | string | desc | Dirección: asc, desc |
| `includeDeleted` | boolean | false | Incluir eliminados |

**Response** (200 OK):
```json
{
  "success": true,
  "data": [
    {
      "id": 12,
      "folderId": 3,
      "url": "https://www.rust-lang.org/",
      "normalizedUrl": "https://www.rust-lang.org",
      "title": "Rust",
      "description": "Lenguaje de programación",
      "notes": "Revisar ownership",
      "domain": "www.rust-lang.org",
      "faviconUrl": null,
      "previewImageUrl": null,
      "isFavorite": true,
      "isArchived": false,
      "isRead": false,
      "visitCount": 5,
      "lastVisitedAt": "2026-03-07T10:00:00Z",
      "createdAt": "2026-03-01T10:00:00Z",
      "updatedAt": "2026-03-05T10:00:00Z",
      "deletedAt": null,
      "folder": { "id": 3, "name": "Programación" },
      "tags": [ { "id": 2, "name": "rust", "color": "#ff6600" } ],
      "metadata": {
        "fetchedTitle": "Rust Programming Language",
        "httpStatusCode": 200,
        "contentType": "text/html",
        "fetchStatus": "ok",
        "lastCheckedAt": "2026-03-07T10:05:00Z"
      }
    }
  ],
  "meta": {
    "page": 1,
    "pageSize": 20,
    "total": 1
  }
}
```

---

#### `GET /api/v1/bookmarks/{id}`

Obtener un bookmark por ID.

**Path Params**:
- `id`: number - ID del bookmark

**Response** (200 OK):
```json
{
  "success": true,
  "data": { /* bookmark object */ }
}
```

**Error** (404 Not Found):
```json
{
  "success": false,
  "error": {
    "code": "BOOKMARK_NOT_FOUND",
    "message": "Bookmark no encontrado"
  }
}
```

---

#### `POST /api/v1/bookmarks`

Crear un nuevo bookmark.

**Body**:
```json
{
  "url": "https://www.rust-lang.org/",
  "title": "Rust",
  "description": "Lenguaje de programación",
  "notes": "Revisar ownership",
  "folderId": 3,
  "tagIds": [2, 5],
  "isFavorite": true,
  "isArchived": false,
  "isRead": false
}
```

**Validation Rules**:
- `url`: Requerido, URL válida (http/https)
- `title`: Opcional, string, máx 500 caracteres
- `description`: Opcional, texto libre
- `notes`: Opcional, texto libre
- `folderId`: Opcional, debe existir
- `tagIds`: Opcional, array de números, deben existir
- `isFavorite`, `isArchived`, `isRead`: Opcionales, booleanos

**Response** (201 Created):
```json
{
  "success": true,
  "data": { /* created bookmark object with generated id */ }
}
```

**Error** (409 Conflict):
```json
{
  "success": false,
  "error": {
    "code": "BOOKMARK_DUPLICATE",
    "message": "Ya existe un bookmark activo con esa URL."
  }
}
```

---

#### `PUT /api/v1/bookmarks/{id}`

Actualizar un bookmark existente.

**Path Params**:
- `id`: number - ID del bookmark

**Body**: Mismo que POST

**Response** (200 OK):
```json
{
  "success": true,
  "data": { /* updated bookmark object */ }
}
```

---

#### `PATCH /api/v1/bookmarks/{id}`

Actualización parcial (campos específicos).

**Path Params**:
- `id`: number - ID del bookmark

**Body**:
```json
{
  "isArchived": true
}
```

---

#### `DELETE /api/v1/bookmarks/{id}`

Borrado lógico de bookmark.

**Path Params**:
- `id`: number - ID del bookmark

**Response** (204 No Content):
Sin cuerpo

---

#### `POST /api/v1/bookmarks/{id}/restore`

Restaurar bookmark eliminado.

**Path Params**:
- `id`: number - ID del bookmark

**Response** (200 OK):
```json
{
  "success": true,
  "data": { /* restored bookmark object */ }
}
```

---

#### `POST /api/v1/bookmarks/{id}/visit`

Registrar visita a bookmark.

**Path Params**:
- `id`: number - ID del bookmark

**Body** (opcional):
```json
{
  "markAsRead": true
}
```

**Response** (200 OK):
```json
{
  "success": true,
  "data": {
    "id": 12,
    "visitCount": 6,
    "lastVisitedAt": "2026-03-07T12:00:00Z"
  }
}
```

---

#### `POST /api/v1/bookmarks/bulk`

Importación masiva de bookmarks.

**Body**:
```json
{
  "bookmarks": [
    {
      "url": "https://example.com",
      "title": "Example",
      "folderId": null,
      "tagIds": [1, 2]
    }
  ]
}
```

**Límite**: Máximo 20,000 bookmarks

**Response** (201 Created o 202 Accepted):
```json
{
  "success": true,
  "data": {
    "imported": 18,
    "skipped": 2,
    "skippedUrls": ["https://example.com"]
  }
}
```

---

### Metadatos

#### `GET /api/v1/bookmarks/{id}/metadata`

Obtener metadatos de un bookmark.

#### `POST /api/v1/bookmarks/{id}/metadata/refresh`

Refrescar metadatos (reintentar scraping).

#### `PUT /api/v1/bookmarks/{id}/metadata`

Actualizar metadatos manualmente.

---

### Folders

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/v1/folders` | Listar (tree o plano) |
| GET | `/api/v1/folders/{id}` | Obtener por ID |
| POST | `/api/v1/folders` | Crear |
| PUT | `/api/v1/folders/{id}` | Actualizar |
| DELETE | `/api/v1/folders/{id}` | Borrar (409 si tiene contenido) |

---

### Tags

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/v1/tags` | Listar (paginado) |
| GET | `/api/v1/tags/{id}` | Obtener por ID |
| POST | `/api/v1/tags` | Crear |
| PUT | `/api/v1/tags/{id}` | Actualizar |
| DELETE | `/api/v1/tags/{id}` | Borrar |

---

### Auxiliares

#### `GET /api/v1/domains?limit=20`

Obtener dominios más usados.

#### `GET /api/v1/stats`

Obtener estadísticas básicas.

**Response**:
```json
{
  "success": true,
  "data": {
    "totalBookmarks": 120,
    "favoriteBookmarks": 18,
    "archivedBookmarks": 9,
    "readBookmarks": 55,
    "totalFolders": 12,
    "totalTags": 28
  }
}
```

---

## HTTP Status Codes

| Código | Uso |
|--------|-----|
| 200 OK | Respuesta exitosa |
| 201 Created | Recurso creado |
| 202 Accepted | Aceptado para procesamiento asíncrono |
| 204 No Content | Sin contenido (borrados) |
| 400 Bad Request | Petición inválida |
| 401 Unauthorized | API Key inválida |
| 404 Not Found | Recurso no encontrado |
| 409 Conflict | Conflicto (duplicados, restricciones) |
| 422 Unprocessable Entity | Error de validación |
| 429 Too Many Requests | Rate limit excedido |
| 500 Internal Server Error | Error del servidor |
| 503 Service Unavailable | Servicio no disponible |

---

## Error Codes Reference

| Código | Descripción |
|--------|-----------|
| `BOOKMARK_DUPLICATE` | Ya existe bookmark activo con esa URL |
| `BOOKMARK_NOT_FOUND` | Bookmark no encontrado |
| `FOLDER_NOT_FOUND` | Carpeta no encontrada |
| `FOLDER_NOT_EMPTY` | Carpeta contiene subcarpetas o bookmarks |
| `TAG_NOT_FOUND` | Tag no encontrado |
| `FOLDER_NAME_EXISTS` | Ya existe carpeta con ese nombre en el mismo padre |
| `TAG_NAME_EXISTS` | Ya existe tag con ese nombre |
| `INVALID_URL` | URL mal formada o sin esquema válido |
| `RATE_LIMIT_EXCEEDED` | Límite de peticiones excedido |
| `INVALID_API_KEY` | API Key inválida o faltante |
