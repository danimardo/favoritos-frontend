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

## 2. Pantallas y Diálogos

### 2.1 Pantalla Principal (Dashboard)

#### Comportamiento de Inicio
- **Validación**: Al arrancar, comprueba si existe `api_key` y `base_url` en el almacenamiento local.
- **Redirección**: Si no existen, abre automáticamente el **Panel de Configuración** con un mensaje: "Configura tu API para empezar".

#### Componentes de la Lista
- **Caché de Favicons**: Los iconos de los sitios web se guardan en la caché temporal de Tauri para evitar peticiones repetitivas a la API y mejorar el scroll.
- **Vista**: Grid de tarjetas (Cards) con: Icono, Título, Dominio y Etiquetas.

---

### 2.2 Gestión de Favoritos (Diálogos)

#### Diálogo: Crear/Editar Favorito
- **Gestión de Carpetas/Etiquetas**: 
  - Dentro del selector de Carpeta, hay una opción "[+] Nueva Carpeta".
  - El selector de etiquetas permite crear una nueva simplemente escribiendo y pulsando `Enter`.
  - *Nota: No existe una sección de gestión de carpetas independiente en esta versión.*
- **Borrado**: El botón "Eliminar" realiza un borrado definitivo (el frontend no ofrece opción de restaurar, aunque la API lo permita internamente).

---

### 2.3 Configuración y Datos

#### Panel de Configuración
- **Campos**: URL de API, API Key.
- **Botón Probar**: Valida contra `/health` y muestra un check verde si es correcto.
- **Importación**: Única vía para carga masiva de archivos JSON. Al terminar, muestra una notificación nativa con el resumen (importados/saltados).

---

## 3. Especificación de Datos y Rendimiento

### 3.1 Modelo de Datos simplificado
- Se ignoran los campos `isArchived` e `isRead` en el frontend para simplificar la interfaz.
- El campo `visitCount` se actualiza automáticamente al abrir un link desde la app.

### 3.2 Rendimiento
- **Lazy Loading**: La lista principal carga los favoritos bajo demanda al hacer scroll.
- **Filtrado Local**: La barra de búsqueda rápida filtra sobre los favoritos ya cargados en memoria para una respuesta instantánea (<50ms).

---

## 4. Mapa de Atajos de Teclado (Actualizado)

| Atajo | Ámbito | Acción |
|-------|--------|--------|
| `Alt + Space` | Global | Búsqueda rápida (ventana mini) |
| `Ctrl + N` | App | Abrir diálogo nuevo favorito |
| `Ctrl + F` | App | Enfocar buscador principal |
| `Ctrl + ,` | App | Abrir configuración |
| `Ctrl + Q` | App | Salir completamente de la aplicación |
| `Esc` | Ventana | Cerrar diálogos o minimizar al tray |

---

## 5. Roadmap de Implementación (Refinado)

### Fase 1: Tauri + SvelteKit Setup
- Configuración de System Tray y ventana básica.
- Implementación de la paleta de colores fija en Tailwind.

### Fase 2: Conectividad y Configuración
- Pantalla de configuración y validación de API Key.
- Lógica de redirección inicial.

### Fase 3: Core CRUD
- Listado con Cards y caché de favicons.
- Diálogos de creación/edición con creación "on-the-fly" de carpetas/tags.

### Fase 4: Integración OS
- Búsqueda global `Alt+Space`.
- Notificaciones y comportamiento de minimizado al Tray.

---
*Fin de la especificación actualizada*
