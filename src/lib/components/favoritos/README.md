# Componentes de Favoritos

Este directorio contiene los componentes específicos del dominio para la aplicación Favoritos Desktop.

## Componentes Principales

### `BuscadorGlobal.svelte`
Ventana de búsqueda Spotlight-style.
- **Uso**: Se abre con `Alt+Space`.
- **Características**: Filtrado local instantáneo (<50ms), navegación por teclado, apertura en navegador predeterminado.
- **Dependencias**: Shadcn Command, `favoritos` store.

### `FavoritoCard.svelte`
Tarjeta individual para mostrar un favorito en el grid.
- **Características**: Muestra favicon, título, URL, etiquetas y carpeta. Incrementa el contador de visitas al hacer clic.

### `FavoritoGrid.svelte`
Grid principal de favoritos con lazy loading.
- **Características**: Usa Intersection Observer para manejar grandes colecciones eficientemente.

### `FavoritoForm.svelte` / `FavoritoDialog.svelte`
Formulario para crear y editar favoritos.
- **Características**: Obtención automática de metadata (título/descripción) al pegar una URL.

### `ConfigForm.svelte` / `ConfigDialog.svelte`
Gestión de credenciales de la API.
- **Características**: Validación de conexión en tiempo real.

### `ImportDialog.svelte`
Importación masiva de favoritos desde JSON.
- **Características**: Validación de formato y resumen de resultados.

## Estándares de Diseño

Todos los componentes siguen la paleta de colores fija del proyecto:
- **Fondo**: Parchment (`#F5F5DC`)
- **Texto Primario**: Taupe (`#483C32`)
- **Texto Secundario**: Grey Olive (`#A9A9A1`)
- **Destacados**: Powder Blush (`#E6E6FA`)
