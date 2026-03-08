# Especificaciones: Favoritos Desktop (Frontend para API Favoritos)

**Tecnologías**: Tauri v2, SvelteKit, Tailwind CSS, Shadcn-Svelte.
**Arquitectura**: Aplicación de escritorio nativa orientada a consumo de API REST.

---

## 0. Conceptos de Diseño y UX

### 0.1 Estética General
- **Inspiración**: Minimalista, limpia, con aires de editor de código o herramienta de productividad (tipo Notion/Raycast).
- **Componentes**: Basados en **Shadcn-Svelte** para una consistencia visual de alta calidad.
- **Iconografía**: Lucide Icons (integrados en Shadcn).

### 0.2 Paleta de Colores (Fija - Sin Modo Claro/Oscuro)
| Color | Hex | Uso |
|-------|-----|-----|
| **Taupe** | `#483C32` | Fondos principales, barras laterales |
| **Grey Olive** | `#A9A9A1` | Textos secundarios, bordes sutiles |
| **Silver** | `#C0C0C0` | Detalles, iconos desactivados |
| **Parchment** | `#F5F5DC` | Fondo de tarjetas, áreas de contenido |
| **Powder Blush** | `#E6E6FA` | Destacados, botones de acción primaria |

---

## 1. Integración con el Sistema (Tauri Core)

### 1.1 Comportamiento de Ventana
- **Cierre Inteligente**: Al pulsar el botón de cerrar (X), la ventana se oculta (`hide`), pero el proceso sigue ejecutándose en el **System Tray**.
- **Restaurar**: Doble clic en el icono del tray o clic derecho > "Abrir" restaura la ventana.
- **Salir**: Solo se puede cerrar la aplicación mediante Clic derecho en Tray > "Salir" o `Ctrl+Q` dentro de la app.

### 1.2 System Tray (Bandeja del Sistema)
- **Icono**: Versión simplificada del logo en blanco/transparente.
- **Menú Contextual**:
  - Abrir Favoritos Desktop
  - Nuevo Favorito rápido (`Ctrl+N`)
  - ---
  - Salir

### 1.3 Notificaciones Nativas
- Se enviarán notificaciones del sistema en los siguientes casos:
  - Error crítico de conexión con la API.
  - Éxito al importar volúmenes grandes de bookmarks.
  - Metadata obtenida correctamente para un nuevo link pegado.

### 1.4 Búsqueda Rápida Global (Spotlight Style)
- **Atajo**: `Alt + Space` (configurable).
- **Acción**: Abre una ventana pequeña, centrada y sin bordes con un componente **Shadcn Command**.
- **Funcionalidad**: Permite buscar y abrir un favorito en el navegador sin tener que abrir la ventana principal de la app.

---

## 2. Autenticación y Perfil (Sistema Multiusuario)

### 2.1 Flujo de Acceso
- **Validación Inicial**: Al arrancar, la aplicación verifica si existe una sesión activa (API Key válida y perfil de usuario accesible).
- **Puerta de Entrada**: Si no hay sesión, se redirige automáticamente a la pantalla de **Login / Registro**.
- **Login**: El usuario introduce su Email y Contraseña. Al tener éxito, la API devuelve una API Key única para la sesión que se almacena de forma segura en el sistema local.
- **Registro**: Permite a nuevos usuarios crear una cuenta proporcionando Email, Contraseña y Nombre opcional.

### 2.2 Gestión de Sesión
- **Persistencia**: La sesión se mantiene abierta entre reinicios de la aplicación.
- **Validación Automática**: En cada inicio, se intenta obtener el perfil del usuario (`/users/me`). Si la API Key ha sido revocada o ha expirado (error 401), se solicita el login de nuevo.
- **Logout**: Opción disponible en el panel de configuración que elimina las credenciales locales y redirige al login.

---

## 3. Pantallas y Diálogos

### 3.1 Pantalla Principal (Dashboard)
- **Aislamiento de Datos**: El usuario solo visualiza sus propios favoritos, carpetas y etiquetas.
- **Vista**: Grid de tarjetas (Cards) con: Icono, Título, Dominio y Etiquetas.
- **Caché de Favicons**: Los iconos de los sitios web se guardan en la caché local para mejorar el rendimiento.

---

### 3.2 Gestión de Favoritos (Diálogos)

#### Diálogo: Crear/Editar Favorito
- **Gestión On-the-fly**: Permite crear carpetas y etiquetas directamente desde el diálogo.
- **Borrado**: Realiza un borrado definitivo del favorito para el usuario actual.

---

### 3.3 Perfil y Ajustes (Anteriormente Configuración)
- **Información del Usuario**: Muestra el Email, Nombre y la URL de la API conectada.
- **Cerrar Sesión**: Botón para salir de la cuenta de forma segura.
- **Ajustes Técnicos**: Permite configurar el nivel de log (Error, Info, Debug, etc.) de la aplicación.

---

## 4. Especificación de Datos y Rendimiento

### 4.1 Modelo de Datos simplificado
- Se ignoran los campos `isArchived` e `isRead` en el frontend para simplificar la interfaz.
- El campo `visitCount` se actualiza automáticamente al abrir un link desde la app.

### 4.2 Rendimiento
- **Lazy Loading**: La lista principal carga los favoritos bajo demanda al hacer scroll.
- **Filtrado Local**: Respuesta instantánea (<50ms) en las búsquedas sobre los datos cargados.

---

## 5. Mapa de Atajos de Teclado

| Atajo | Ámbito | Acción |
|-------|--------|--------|
| `Alt + Space` | Global | Búsqueda rápida (ventana mini) |
| `Ctrl + N` | App | Abrir diálogo nuevo favorito |
| `Ctrl + F` | App | Enfocar buscador principal |
| `Ctrl + ,` | App | Abrir Perfil y Ajustes |
| `Ctrl + Q` | App | Salir completamente de la aplicación |
| `Esc` | Ventana | Cerrar diálogos o minimizar al tray |

---
*Fin de la especificación actualizada - Sistema Multiusuario v2*
