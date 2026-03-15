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
  - Éxito al importar volúmenes grandes de bookmarks (formato HTML Netscape Bookmark, con opción de enriquecimiento por IA).
  - Éxito al exportar favoritos (formato JSON propio o HTML de marcadores).
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
- **Vista**: Grid de tarjetas (Cards) o lista compacta. El tipo de vista seleccionado (grid/list) se persiste entre sesiones.
- **Caché de Favicons**: Los iconos de los sitios web se guardan en la caché local para mejorar el rendimiento.
- **Sidebar de Navegación**: Barra lateral con dos secciones colapsables tipo acordeón:
  - **Carpetas**: Lista de carpetas del usuario con conteo de favoritos por carpeta.
  - **Etiquetas**: Lista de etiquetas extraídas automáticamente de los favoritos, ordenadas por frecuencia de uso descendente.
  - Ambas secciones se pueden expandir/contraer independientemente para optimizar el espacio.
  - Los filtros de carpeta y etiqueta son mutuamente excluyentes: seleccionar uno deselecciona el otro.
  - El botón "Todos" deselecciona ambos filtros.

---

### 3.2 Gestión de Favoritos (Diálogos)

#### Diálogo: Crear/Editar Favorito
- **Gestión On-the-fly**: Permite crear carpetas y etiquetas directamente desde el diálogo.
- **Descripción multilínea**: El campo "Descripción Corta" es un textarea de varias líneas.
- **Enriquecimiento con IA**: Disponible en dos modos:
  - **Post-creación (automático)**: Al crear un nuevo favorito, el modal permanece abierto y se lanza automáticamente el enriquecimiento. El usuario ve los campos actualizados y puede revisarlos antes de pulsar "Guardar y Cerrar" o "Cerrar sin guardar".
  - **Edición (manual)**: En el modal de edición, un botón "Enriquecer con IA" permite relanzar el enriquecimiento bajo demanda.
  - En ambos casos se usa `POST /bookmarks/:id/metadata/refresh` con polling de `GET /bookmarks/:id/metadata/status` hasta `fetchStatus: "ok"`, y luego `GET /bookmarks/:id` para obtener los datos enriquecidos.
- **Detección de duplicados**: Si se intenta crear un favorito con una URL que ya existe (409 Conflict), la app carga automáticamente los datos existentes en el formulario para revisión y actualización, en lugar de mostrar un error técnico.
- **Borrado**: Realiza un borrado definitivo del favorito para el usuario actual.

#### Botón Nuevo Favorito (FAB)
- **Posición**: Botón flotante circular con icono "+" en la esquina inferior derecha de la pantalla principal.
- **Atajos**: `Ctrl+N` o tecla `+` (fuera de campos de texto).

---

### 3.3 Perfil y Ajustes (Anteriormente Configuración)
- **Información del Usuario**: Muestra el Email, Nombre y la URL de la API conectada.
- **API Key editable**: Permite cambiar la API Key desde la pantalla de configuración (campo tipo password, vacío para mantener la actual).
- **Cerrar Sesión**: Botón para salir de la cuenta de forma segura.
- **Ajustes Técnicos**: Permite configurar el nivel de log (Error, Info, Debug, etc.) de la aplicación.
- **Añadir URL sueltas**: Permite introducir múltiples URLs (una por línea) para crear favoritos de forma secuencial. Valida que cada línea sea una URL válida con protocolo http/https antes de iniciar. Muestra resumen con creados/duplicados/errores al finalizar.

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
| `+` | App | Abrir diálogo nuevo favorito (fuera de inputs) |
| `Ctrl + F` | App | Enfocar buscador principal |
| `Ctrl + ,` | App | Abrir Perfil y Ajustes |
| `Ctrl + Q` | App | Salir completamente de la aplicación |
| `Esc` | Ventana | Cerrar diálogo abierto, o minimizar al tray si no hay diálogos |

---
*Fin de la especificación actualizada - Sistema Multiusuario v2*
