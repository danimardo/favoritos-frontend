# favoritos-frontend Development Guidelines

Auto-generated from constitution. Last updated: 2026-03-07

## Stack Tecnológico (segundo constitution.md)

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
├── lib/components/ui/     # Shadcn-Svelte
├── lib/components/favoritos/ # Dominio
├── lib/stores/            # Estado
├── lib/services/          # Lógica de negocio
src-tauri/src/             # Rust backend
tests/                     # Vitest + Playwright
```

## Commands

Frontend: `npm run dev`, `npm run build`, `npm run check`
Tauri: `npm run tauri dev`, `npm run tauri build`
Backend: `cargo test; cargo clippy`

## Testing

- Cobertura mínima: 80% para código de negocio
- Vitest para unit tests
- Playwright para E2E

## Recent Changes

- 001-favoritos-desktop: Constitución creada con stack completo y principios no negociables

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
