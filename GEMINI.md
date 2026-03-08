# favoritos-frontend - Normas para Gemini

Este archivo contiene las instrucciones específicas para agentes Gemini que trabajen en este proyecto.

## Stack Tecnológico

| Componente | Versión |
|------------|---------|
| **Tauri** | v2.8.0+ |
| **SvelteKit** | v2.9.0+ |
| **Svelte** | v5.0.0+ |
| **Tailwind CSS** | v4.2.1+ |
| **TypeScript** | v5.6.2+ |
| **Rust** | 1.70+ |
| **Node.js** | v20+ |

## Paleta de Colores (fija, sin modo oscuro)

| Color | Hex | Uso |
|-------|-----|-----|
| Taupe | `#483C32` | Fondos principales |
| Grey Olive | `#A9A9A1` | Textos secundarios |
| Silver | `#C0C0C0` | Detalles |
| Parchment | `#F5F5DC` | Tarjetas |
| Powder Blush | `#E6E6FA` | Destacados |

## Arquitectura

```
src/
├── lib/components/ui/       # Shadcn-Svelte
├── lib/components/favoritos/ # Dominio
├── lib/stores/              # Estado
├── lib/services/            # Lógica de negocio
src-tauri/src/              # Rust backend
tests/                      # Vitest + Playwright
```

---

<!-- MANUAL ADDITIONS START -->
## Normas de Desarrollo para Agentes IA

### Comunicación
- Comunícate con el usuario en español.
- Todas las instrucciones y output escríbelos en español.

### Documentos Normativos (orden de prioridad)
1. **`specs/001-favoritos-desktop/spec.md`** - Especificación funcional completa
2. **`.specify/memory/constitution.md`** - Reglas y limitaciones de diseño (NO negociables)
3. **`Manual-Uso-API.md`** - Manual didáctico sobre el uso, opciones, posibilidades y características de la API (Referencia fundamental para entender el backend).
4. **`historias.md`** - Documentación en formato "Historias de usuario"
5. **`specs/001-favoritos-desktop/plan.md`** - Plan de implementación
6. **`specs/001-favoritos-desktop/tasks.md`** - Tareas de desarrollo (113 tareas)

### Instrucciones para Agentes
- Sigue estrictamente las definiciones del plan de implementación en `historias.md`, `constitution.md` y `spec.md`.
- No modifiques el comportamiento fuera de los criterios descritos en esos documentos.
- Si detectas discrepancias entre el código y esos documentos, solicita autorización antes de aplicar cualquier cambio que altere el comportamiento.
- Cuando una tarea no esté cubierta por esos documentos, propone cambios como diffs o notas de diseño, pero no los apliques sin validación explícita.

### Confirmación Obligatoria
- Antes de realizar refactors o "arreglos" no especificados, pregunta y espera confirmación.
- Evita optimizaciones que cambien efectos observables si no están contempladas en `historias.md`, `constitution.md` o `spec.md`.

### Cumplimiento y Trazabilidad
- Referencia siempre el/los criterio(s) de aceptación correspondiente(s) al realizar cambios.
- Si un criterio es ambiguo, pide aclaración en lugar de asumir comportamiento.

### Documentación Obligatoria al Finalizar Tareas
- **Cualquier cambio en el código — sin excepción — debe reflejarse en `spec.md` antes de dar la tarea por terminada.**
- Esto incluye: correcciones de bugs que cambian comportamiento, ajustes de lógica, nuevas métricas, cambios en reglas de negocio y correcciones de campo.
- Solo se puede omitir la documentación cuando el cambio es puramente interno y no altera ningún comportamiento observable (ej. renombrar una variable local o reformatear código).
- La regla es: documenta siempre, salvo que el cambio sea invisible para el usuario. La excepción es la que requiere justificación, no la documentación.

### Claridad Antes de Implementar
- Siempre que se te mande hacer un cambio o añadir una nueva característica, si la intención no está 100% clara o hay huecos/dudas/ambigüedades, pregunta antes de implementar nada.

### Documentación Técnica
- Para implementar código, **utiliza Context7** para consultar la documentación más reciente y asegúrate de que la implementación cumpla con el Spec y use la sintaxis más moderna.

### GitHub
- Para commits y pushes, utiliza la CLI `gh` instalada en el sistema.

<!-- MANUAL ADDITIONS END -->
