# Feature Specification: Favoritos Desktop

**Feature Branch**: `001-favoritos-desktop`
**Created**: 2026-03-07
**Status**: Completed
**Implementation Date**: 2026-03-07
**Input**: User description: "Especificaciones completas de la aplicación Favoritos Desktop - Frontend para API Favoritos con Tauri v2, SvelteKit, Tailwind CSS y Shadcn-Svelte"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Autenticación de Usuario (Priority: P1)

Como usuario, necesito registrarme e iniciar sesión con mi email y contraseña para acceder a mis favoritos personales de forma segura.

**Why this priority**: Es el bloqueador absoluto y la base del sistema multiusuario. Sin autenticación, el usuario no puede acceder a sus datos privados.

**Independent Test**:
1. El usuario abre la app por primera vez y se le presenta el formulario de Login/Registro.
2. El usuario crea una cuenta nueva exitosamente.
3. El usuario inicia sesión con sus credenciales.
4. Tras el login, la aplicación obtiene la API Key dinámica y muestra el dashboard.
5. Al reiniciar la app, la sesión persiste automáticamente.

**Acceptance Scenarios**:

1. **Given** la aplicación se inicia sin sesión activa, **When** el usuario abre la app, **Then** se muestra la pantalla de Login con opción de Registro.
2. **Given** el usuario introduce credenciales válidas, **When** hace clic en "Entrar", **Then** se obtiene su perfil y se redirige al dashboard.
3. **Given** el usuario introduce credenciales inválidas, **When** intenta loguearse, **Then** se muestra un mensaje de error claro ("Credenciales inválidas").
4. **Given** el usuario quiere crear una cuenta, **When** completa el formulario de registro, **Then** se crea su usuario e inicia sesión automáticamente.
5. **Given** el usuario está logueado, **When** accede a Ajustes y pulsa "Cerrar Sesión", **Then** se eliminan las credenciales locales y vuelve a la pantalla de Login.

---

### User Story 2 - Gestión de Favoritos CRUD (Priority: P1)

Como usuario, necesito crear, ver, editar y eliminar mis favoritos personales asegurando el aislamiento de datos.

**Why this priority**: Es el valor core de la aplicación. Sin esta funcionalidad, la aplicación no tiene propósito.

**Independent Test**: Se puede probar independientemente realizando operaciones CRUD completas y verificando que los cambios son privados para el usuario.

**Acceptance Scenarios**:

1. **Given** el usuario está logueado, **When** presiona `Ctrl+N` o clic en "Nuevo Favorito", **Then** se abre el diálogo de creación.
2. **Given** el usuario crea un nuevo favorito, **When** completa el formulario y guarda, **Then** el favorito aparece en su lista personal.
4. **Given** el usuario edita un favorito existente, **When** modifica sus datos y guarda, **Then** los cambios se reflejan inmediatamente en la lista
5. **Given** el usuario elimina un favorito, **When** confirma la eliminación, **Then** el favorito desaparece de la lista permanentemente
6. **Given** el usuario crea una etiqueta escribiendo y presionando Enter, **When** completa la acción, **Then** la nueva etiqueta se crea y queda asociada al favorito
7. **Given** el usuario selecciona "[+] Nueva Carpeta" en el selector, **When** ingresa un nombre y confirma, **Then** la nueva carpeta se crea y queda disponible para selección

---

### User Story 3 - Búsqueda Global (Priority: P2)

Como usuario, necesito buscar y abrir rápidamente un favorito sin tener que abrir la ventana principal de la aplicación, para mejorar mi productividad.

**Why this priority**: Es una característica de productividad importante que mejora significativamente la experiencia del usuario, pero la aplicación puede funcionar sin ella (se puede usar la búsqueda en la ventana principal).

**Independent Test**: Se puede probar independientemente activando el atajo global `Alt+Space` y verificando que se abre la ventana de búsqueda, se pueden buscar favoritos y abrirlos en el navegador.

**Acceptance Scenarios**:

1. **Given** la aplicación está ejecutándose en segundo plano, **When** el usuario presiona `Alt+Space`, **Then** se abre una ventana pequeña, centrada y sin bordes con el componente de búsqueda
2. **Given** la ventana de búsqueda está abierta, **When** el usuario escribe términos de búsqueda, **Then** la lista de favoritos se filtra instantáneamente según FR-013 (<50ms)
3. **Given** el usuario selecciona un favorito de la búsqueda, **When** presiona Enter, **Then** el favorito se abre en el navegador predeterminado y la ventana se cierra
4. **Given** la ventana de búsqueda está abierta, **When** el usuario presiona `Esc` o hace clic fuera de la ventana (pierde el foco), **Then** la ventana se oculta automáticamente (Spotlight-style)

---

### User Story 4 - Integración con Sistema (Priority: P2)

Como usuario, necesito que la aplicación se comporte como una app nativa con System Tray y cierre inteligente, para que no interfiera con mi flujo de trabajo.

**Why this priority**: Es importante para la experiencia de usuario nativa, pero la aplicación puede funcionar sin estas características (aunque de manera menos ideal).

**Independent Test**: Se puede probar independientemente verificando el comportamiento de cierre, restauración desde el tray y funcionamiento del menú contextual.

**Acceptance Scenarios**:

1. **Given** la aplicación está abierta, **When** el usuario presiona el botón cerrar (X), **Then** la ventana se oculta pero la aplicación sigue ejecutándose en el System Tray
2. **Given** la aplicación está minimizada al tray, **When** el usuario hace doble clic en el icono del tray, **Then** la ventana se restaura
3. **Given** la aplicación está ejecutándose, **When** el usuario hace clic derecho en el icono del tray y selecciona "Salir", **Then** la aplicación se cierra completamente
4. **Given** la aplicación está abierta, **When** el usuario presiona `Ctrl+Q`, **Then** la aplicación se cierra completamente
5. **Given** la aplicación está minimizada al tray, **When** el usuario hace clic derecho en el icono y selecciona "Nuevo Favorito rápido", **Then** se abre el diálogo de creación de favorito

---

### User Story 5 - Importación Masiva (Priority: P3)

Como usuario, necesito importar una colección grande de favoritos desde un archivo HTML de marcadores (formato Netscape Bookmark, exportado desde Firefox, Chrome, etc.) para migrar mis datos existentes sin tener que agregarlos manualmente. Opcionalmente, puedo activar el enriquecimiento con IA para que el servidor analice cada marcador y complete automáticamente título, descripción y etiquetas.

**Why this priority**: Es una funcionalidad importante pero de uso poco frecuente (one-time o migraciones ocasionales). La app puede funcionar perfectamente sin ella.

**Independent Test**: Se puede probar independientemente cargando un archivo HTML de marcadores válido y verificando que los favoritos se importan correctamente y se muestra el resumen con notificación.

**Acceptance Scenarios**:

1. **Given** el usuario está en el panel de configuración, **When** hace clic en "Importar" y selecciona un archivo HTML de marcadores válido, **Then** los favoritos se importan correctamente vía `POST /bookmarks/import` (multipart/form-data)
2. **Given** la importación finaliza exitosamente, **When** se completa el proceso, **Then** se muestra una notificación nativa con el resumen (importados/actualizados/omitidos/total)
3. **Given** el usuario intenta importar un archivo con formato inválido, **When** inicia la importación, **Then** se muestra un mensaje de error indicando el problema
4. **Given** el usuario activa el toggle de "Enriquecimiento con IA", **When** se envía el archivo, **Then** se incluye el parámetro `ia=true` para que el servidor enriquezca los datos con IA

---

### Edge Cases

- **Qué pasa cuando** la API no responde o no está disponible?
  - La aplicación debe mostrar un mensaje de error claro y permitir reintentar o modificar la configuración
  - Los favoritos ya cargados deben permanecer accesibles (lectura offline)

- **Qué pasa cuando** el caché de favicons se corrompe?
  - La aplicación debe detectar la corrupción, limpiar el caché y reconstruirlo automáticamente
  - No se debe interrumpir la funcionalidad principal

- **Qué pasa cuando** un favicon no puede obtenerse?
  - La aplicación debe usar un icono genérico o placeholder
  - No se debe impedir la creación/visualización del favorito

- **Qué pasa cuando** se intenta importar un archivo HTML con estructura inválida?
  - Se debe mostrar un mensaje de error específico indicando el problema de validación
  - La importación no debe modificar datos existentes

- **Qué pasa cuando** el usuario presiona `Esc` en un diálogo abierto?
  - El diálogo se cierra sin guardar cambios
  - Si la ventana principal está abierta, `Esc` minimiza la aplicación al tray

- **Qué pasa cuando** hay miles de favoritos?
  - La lista debe cargar con lazy loading para no bloquear la UI
  - La búsqueda debe filtrar sobre los datos ya cargados en memoria

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: La aplicación DEBE verificar al inicio si existe una sesión válida (API Key almacenada y perfil accesible).
- **FR-002**: La aplicación DEBE redirigir a la pantalla de Login/Registro si no existe una sesión activa.
- **FR-003**: La aplicación DEBE validar la sesión realizando una petición al perfil del usuario (`GET /users/me`).
- **FR-004**: La aplicación DEBE permitir el registro de nuevos usuarios mediante Email y Contraseña.
- **FR-005**: La aplicación DEBE permitir el inicio de sesión para obtener una API Key dinámica.
- **FR-006**: La aplicación DEBE almacenar de forma segura la API Key obtenida en el almacenamiento local.
- **FR-007**: La aplicación DEBE permitir cerrar sesión (Logout), eliminando las credenciales locales.
- **FR-008**: La aplicación DEBE permitir crear favoritos con URL, título, descripción, carpeta y etiquetas asociados al usuario logueado.
- **FR-009**: La aplicación DEBE obtener automáticamente metadata (favicon, título) al pegar una URL.
- **FR-010**: La aplicación DEBE cachear favicons localmente para evitar peticiones repetitivas.
- **FR-011**: La aplicación DEBE mostrar los favoritos en un grid de tarjetas.
- **FR-012**: La aplicación DEBE implementar lazy loading.
- **FR-013**: La aplicación DEBE filtrar localmente los favoritos en la búsqueda, incluyendo los campos: Título, URL, Descripción, Etiquetas, Carpeta y Notas Personales.
- **FR-014**: La aplicación DEBE incrementar el contador de visitas al abrir un favorito.
- **FR-015**: La aplicación DEBE responder al atajo `Alt+Space` abriendo la búsqueda global.
- **FR-016**: La aplicación DEBE mantener el icono en el System Tray.
- **FR-017**: La aplicación DEBE minimizar al tray al presionar el botón X.
- **FR-018**: La aplicación DEBE permitir salir completamente con `Ctrl+Q` o menú del tray.
- **FR-019**: La aplicación DEBE enviar notificaciones nativas para errores críticos.
- **FR-020**: La aplicación DEBE enviar notificaciones nativas al completar importación.
- **FR-021**: La aplicación DEBE ignorar los campos `isArchived` e `isRead` en el frontend.
- **FR-022**: La aplicación DEBE permitir importar favoritos de forma masiva desde archivos HTML de marcadores (formato Netscape Bookmark) utilizando el endpoint `POST /bookmarks/import` con `multipart/form-data`. Debe ofrecer un toggle para activar enriquecimiento con IA (`ia=true`).
- **FR-023**: La aplicación DEBE mostrar notificación con resumen tras importación (importados, actualizados, omitidos, total).
- **FR-023b**: La aplicación DEBE permitir exportar favoritos en dos formatos a elegir por el usuario mediante un diálogo: JSON (formato propio) y HTML de marcadores (Netscape Bookmark, vía `GET /bookmarks/export`, compatible con Firefox/Chrome/Edge).
- **FR-024**: La aplicación DEBE mantener la paleta de colores fija.
- **FR-025**: La aplicación DEBE responder a los atajos: `Ctrl+N` (nuevo), `Ctrl+F` (buscar), `Ctrl+,` (perfil), `Ctrl+Q` (salir), `Esc` (cerrar/minimizar).
- **FR-026**: La aplicación DEBE permitir al usuario configurar el nivel de log.
- **FR-027**: La aplicación DEBE persistir automáticamente el tamaño y la posición de la ventana.

### Key Entities

- **Usuario**: Entidad principal que representa al dueño de los datos. Atributos: Email, Nombre, ID único.
- **Favorito**: Representa un enlace guardado por el usuario. Atributos: URL, título, descripción, notas personales, dominio, favicon, etiquetas, carpeta, contador de visitas.
- **Carpeta**: Agrupa favoritos. Atributos: nombre, orden/posición.
- **Etiqueta**: Clasificación flexible de favoritos.
- **Sesión**: Almacena las credenciales de conexión activas. Atributos: API URL, API Key dinámica, Usuario actual.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: El usuario puede completar la configuración inicial de la API en menos de 2 minutos
- **SC-002**: La lista de favoritos carga inicialmente en menos de 3 segundos con hasta 100 elementos
- **SC-003**: La búsqueda local responde en menos de 50ms sobre los favoritos cargados en memoria
- **SC-004**: La ventana de búsqueda global (`Alt+Space`) aparece en menos de 200ms tras presionar el atajo
- **SC-005**: El 95% de los favicons se obtienen y cachean correctamente en el primer intento
- **SC-006**: La aplicación puede manejar colecciones de hasta 10,000 favoritos manteniendo scroll a 60fps y búsqueda bajo 50ms
- **SC-007**: La UI de creación de favoritos tiene todos los campos claramente etiquetados con placeholders descriptivos y ayudas visuales
- **SC-008**: La aplicación consume menos de 100MB de memoria RAM en estado idle
- **SC-009**: El tiempo de apertura de un favorito (click → navegador abierto) es menor a 1 segundo
- **SC-010**: La importación de 1,000 favoritos se completa en menos de 30 segundos con notificación de resumen

## Assumptions

- La API REST de Favoritos ya está implementada y funcional
- La API expone endpoints para CRUD de favoritos, validación de health y obtención de metadata
- El usuario tiene credenciales válidas de la API Favoritos
- El sistema operativo es Windows 10/11 (prioridad), con soporte futuro para macOS y Linux
- El navegador predeterminado del sistema está configurado correctamente
- La API Key tiene permisos suficientes para realizar todas las operaciones necesarias
- Los archivos HTML de marcadores para importación siguen el formato Netscape Bookmark estándar

## Open Questions

Ninguna por el momento. La especificación proporcionada es suficientemente detallada.

## Dependencies

- API REST de Favoritos debe estar accesible y documentada
- Icono de la aplicación (logo simplificado para System Tray)
- Soporte del servidor para importación vía `POST /bookmarks/import` con archivos HTML (Netscape Bookmark)
