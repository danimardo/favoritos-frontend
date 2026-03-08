# Novedades de la API - Sistema Multiusuario

## 📋 Resumen de cambios

La API ha sido transformada de un sistema monousuario a un **sistema multiusuario completo**. Ahora cada usuario tiene sus propios bookmarks, folders y tags completamente aislados.

---

## 🔐 Autenticación

### Antes
- Se usaba una sola API key global (`X-API-Key`)
- Todos los datos compartidos entre todos los usuarios

### Ahora
- Cada usuario tiene su propia API key
- Los datos están completamente aislados por usuario
- Registro y login de usuarios

---

## 🆕 Nuevos Endpoints

### 1. Registro de Usuario

```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "usuario@ejemplo.com",
  "password": "Password123!",
  "name": "Nombre Opcional"
}
```

**Respuesta:**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "1",
      "email": "usuario@ejemplo.com",
      "name": "Nombre Opcional"
    },
    "apiKey": "fp_64caractereshexadecimales..."
  }
}
```

**Importante:** Guarda la `apiKey` devuelta, ya que solo se muestra una vez.

---

### 2. Login de Usuario

```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "usuario@ejemplo.com",
  "password": "Password123!"
}
```

**Respuesta:**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "1",
      "email": "usuario@ejemplo.com",
      "name": "Nombre Opcional"
    },
    "apiKey": "fp_nueva_api_key..."
  }
}
```

**Nota:** Cada login genera una nueva API key.

---

### 3. Perfil de Usuario

```http
GET /api/v1/users/me
X-API-Key: fp_tu_api_key
```

**Respuesta:**
```json
{
  "success": true,
  "data": {
    "id": "1",
    "email": "usuario@ejemplo.com",
    "name": "Nombre Opcional",
    "isActive": true,
    "createdAt": "2026-03-08T08:00:00.000Z"
  }
}
```

---

### 4. Actualizar Perfil

```http
PUT /api/v1/users/me
Content-Type: application/json
X-API-Key: fp_tu_api_key

{
  "email": "nuevo@email.com",
  "name": "Nuevo Nombre"
}
```

---

### 5. Crear Nueva API Key

```http
POST /api/v1/users/me/api-keys
Content-Type: application/json
X-API-Key: fp_tu_api_key

{
  "name": "API Key para móvil"
}
```

**Respuesta:**
```json
{
  "success": true,
  "data": {
    "apiKey": "fp_nueva_api_key...",
    "createdAt": "2026-03-08T08:00:00.000Z"
  }
}
```

---

### 6. Listar API Keys

```http
GET /api/v1/users/me/api-keys
X-API-Key: fp_tu_api_key
```

**Respuesta:**
```json
{
  "success": true,
  "data": [
    {
      "id": "1",
      "name": "Default API Key",
      "lastUsedAt": "2026-03-08T08:00:00.000Z",
      "expiresAt": null,
      "isActive": true,
      "createdAt": "2026-03-08T08:00:00.000Z"
    }
  ]
}
```

---

### 7. Revocar API Key

```http
DELETE /api/v1/users/me/api-keys/:id
X-API-Key: fp_tu_api_key
```

---

## 🔄 Cambios en Endpoints Existente

### Todos los endpoints protegidos ahora requieren API Key

```http
GET /api/v1/bookmarks
X-API-Key: fp_tu_api_key

POST /api/v1/bookmarks
X-API-Key: fp_tu_api_key
Content-Type: application/json
```

**Importante:** La API key debe ser la del usuario autenticado. Cada usuario solo ve sus propios datos.

---

## 📌 Endpoints Disponibles

### Autenticación
| Método | Endpoint | Descripción | ¿Requiere API Key? |
|--------|----------|-------------|-------------------|
| POST | `/api/v1/auth/register` | Registrar usuario | ❌ No |
| POST | `/api/v1/auth/login` | Login | ❌ No |

### Usuario
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/v1/users/me` | Obtener perfil |
| PUT | `/api/v1/users/me` | Actualizar perfil |
| POST | `/api/v1/users/me/api-keys` | Crear API key |
| GET | `/api/v1/users/me/api-keys` | Listar API keys |
| DELETE | `/api/v1/users/me/api-keys/:id` | Revocar API key |

### Bookmarks
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/v1/bookmarks` | Listar bookmarks |
| POST | `/api/v1/bookmarks` | Crear bookmark |
| GET | `/api/v1/bookmarks/:id` | Obtener bookmark |
| PATCH | `/api/v1/bookmarks/:id` | Actualizar bookmark |
| DELETE | `/api/v1/bookmarks/:id` | Eliminar bookmark |
| POST | `/api/v1/bookmarks/:id/visit` | Registrar visita |
| POST | `/api/v1/bookmarks/:id/restore` | Restaurar eliminado |
| POST | `/api/v1/bookmarks/bulk` | Crear múltiples |
| GET | `/api/v1/bookmarks/:id/metadata` | Obtener metadatos |
| POST | `/api/v1/bookmarks/:id/metadata/refresh` | Refrescar metadatos |

### Folders
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/v1/folders` | Listar folders |
| POST | `/api/v1/folders` | Crear folder |
| GET | `/api/v1/folders/:id` | Obtener folder |
| PATCH | `/api/v1/folders/:id` | Actualizar folder |
| DELETE | `/api/v1/folders/:id` | Eliminar folder |

### Tags
| Método | Endpoint | Descripción |
|--------|----------|-------------|
| GET | `/api/v1/tags` | Listar tags |
| POST | `/api/v1/tags` | Crear tag |
| GET | `/api/v1/tags/:id` | Obtener tag |
| PATCH | `/api/v1/tags/:id` | Actualizar tag |
| DELETE | `/api/v1/tags/:id` | Eliminar tag |

### Health
| Método | Endpoint | Descripción | ¿Requiere API Key? |
|--------|----------|-------------|-------------------|
| GET | `/api/v1/health` | Health check | ❌ No |

---

## 🔑 Formato de la API Key

Las API keys tienen el formato: `fp_` + 64 caracteres hexadecimales

Ejemplo: `fp_1abd0269eefa5f0e52e7d0179bf265ef71e6ac36a53ef40ea8ae6f8005302923`

---

## ⚠️ Errores Comunes

### API Key inválida o no proporcionada
```json
{
  "success": false,
  "error": {
    "code": "INVALID_API_KEY",
    "message": "API Key no proporcionada"
  }
}
```

### Email ya registrado
```json
{
  "success": false,
  "error": {
    "code": "CONFLICT",
    "message": "El email ya está registrado"
  }
}
```

### Credenciales inválidas
```json
{
  "success": false,
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Credenciales inválidas"
  }
}
```

---

## 📝 Reglas de Validación

### Password
- Mínimo 8 caracteres
- Recomendado: usar mayúsculas, minúsculas, números y símbolos

### Email
- Debe ser un email válido
- Se almacena en minúsculas
- Debe ser único

### URL de Bookmark
- Debe ser una URL válida
- Se normaliza automáticamente (se añade `/` final si no existe)

---

## 🚀 Flujo de Autenticación Recomendado

1. **Registro (primera vez):**
   ```javascript
   const response = await fetch('/api/v1/auth/register', {
     method: 'POST',
     headers: { 'Content-Type': 'application/json' },
     body: JSON.stringify({
       email: 'usuario@ejemplo.com',
       password: 'Password123!',
       name: 'Mi Nombre'
     })
   });
   const { data } = await response.json();
   // Guardar apiKey en localStorage o AsyncStorage
   localStorage.setItem('apiKey', data.apiKey);
   ```

2. **Login (usos posteriores):**
   ```javascript
   const response = await fetch('/api/v1/auth/login', {
     method: 'POST',
     headers: { 'Content-Type': 'application/json' },
     body: JSON.stringify({
       email: 'usuario@ejemplo.com',
       password: 'Password123!'
     })
   });
   const { data } = await response.json();
   // Actualizar apiKey
   localStorage.setItem('apiKey', data.apiKey);
   ```

3. **Peticiones autenticadas:**
   ```javascript
   const response = await fetch('/api/v1/bookmarks', {
     headers: {
       'X-API-Key': localStorage.getItem('apiKey')
     }
   });
   ```

---

## 📦 URL Base

- **Desarrollo:** `http://localhost:3004/api/v1`
- **Producción:** (según configuración del servidor)

---

## 🔄 Cambios en las Respuestas

### Bookmark - Ahora incluye metadatos
```json
{
  "id": "1",
  "url": "https://example.com",
  "title": "Ejemplo",
  "metadata": {
    "fetchStatus": "pending", // or "success", "error"
    "fetchedTitle": "Título obtenido",
    "fetchedDescription": "Descripción obtenida",
    "lastCheckedAt": "2026-03-08T08:00:00.000Z"
  }
}
```

---

## 📚 Documentación Adicional

- **Swagger UI:** `http://localhost:3004/api/docs`
- **Health Check:** `http://localhost:3004/api/v1/health`

---

## 🎯 Checklist para el Frontend

- [ ] Implementar pantalla de registro
- [ ] Implementar pantalla de login
- [ ] Guardar API key de forma segura
- [ ] Incluir `X-API-Key` header en todas las peticiones (excepto auth)
- [ ] Manejar errores de autenticación (401)
- [ ] Implementar pantalla de perfil de usuario
- [ ] Permitir al usuario crear múltiples API keys
- [ ] Permitir al usuario revocar API keys
- [ ] Mostrar el email del usuario autenticado

---

## 📞 Notas Importantes

1. **La API key es secreta** - No debe exponerse en URLs o logs
2. **Cada login genera una nueva API key** - Las anteriores siguen funcionando
3. **Los datos están aislados** - Cada usuario solo ve sus propios datos
4. **No hay endpoint de logout** - Simplemente elimina la API key del cliente
5. **No hay refresh token** - Si olvidas la API key, haz login de nuevo

---

*Generado el 2026-03-08*
