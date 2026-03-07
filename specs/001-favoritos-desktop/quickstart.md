# Quickstart Guide - Favoritos Desktop

**Feature**: 001-favoritos-desktop
**Date**: 2026-03-07

## Overview

Guía rápida para configurar y comenzar el desarrollo de Favoritos Desktop.

---

## Prerrequisitos

| Herramienta | Versión Mínima | Instalación |
|-------------|----------------|-------------|
| **Node.js** | v20 LTS | [nodejs.org](https://nodejs.org/) |
| **Rust** | v1.70+ | [rustup.rs](https://rustup.rs/) |
| **Git** | Última | [git-scm.com](https://git-scm.com/) |
| **pnpm** (recomendado) | Última | `npm install -g pnpm` |

### Verificar Instalación

```bash
node --version   # v20.x.x
rustc --version  # 1.70+
pnpm --version   # Última
git --version    # 2.x.x
```

---

## Configuración Inicial del Proyecto

### 1. Crear Proyecto Tauri + SvelteKit

```bash
# Usar create-tauri-app con template SvelteKit
pnpm create tauri-app@latest

# Durante el setup:
# - Project name: favoritos-frontend
# - Frontend: SvelteKit
# - UI template: Skeleton (luego añadiremos Shadcn)
```

### 2. Configurar SvelteKit para Tauri

Actualizar `svelte.config.js`:

```javascript
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: vitePreprocess(),
    kit: {
        adapter: adapter({
            fallback: 'index.html'
        })
    }
};

export default config;
```

### 3. Instalar Dependencias Adicionales

```bash
cd favoritos-frontend

# Adapter estático para SvelteKit
pnpm add -D @sveltejs/adapter-static

# Shadcn-Svelte
pnpm add -D bits-ui clsx tailwind-merge
pnpm add @tailwindcss/typography

# Utilidades
pnpm add date-fns
pnpm add -D @types/node

# Tauri plugins
pnpm add @tauri-apps/api
pnpm add @tauri-apps/plugin-shell
pnpm add @tauri-apps/plugin-credential
```

---

## Configurar Tailwind con Paleta de Colores

Actualizar `tailwind.config.js`:

```javascript
/** @type {import('tailwindcss').Config} */
export default {
    darkMode: [], // SIN modo oscuro
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            colors: {
                taupe: '#483C32',
                greyOlive: '#A9A9A1',
                silver: '#C0C0C0',
                parchment: '#F5F5DC',
                powderBlush: '#E6E6FA'
            },
            borderRadius: {
                lg: 'var(--radius)',
                md: 'calc(var(--radius) - 2px)',
                sm: 'calc(var(--radius) - 4px)'
            }
        }
    },
    plugins: [require('@tailwindcss/typography')]
};
```

Actualizar `src/app.css`:

```css
@import 'tailwindcss/base';
@import 'tailwindcss/components';
@import 'tailwindcss/utilities';

:root {
    --radius: 0.5rem;
}

body {
    @apply bg-taupe text-silver;
}
```

---

## Instalar Shadcn-Svelte

```bash
# Inicializar Shadcn
npx shadcn-svelte@latest init

# Durante el setup:
# - TypeScript: Yes
# - Tailwind config: tailwind.config.js
# - CSS file: src/app.css
# - Base color: Neutral (luego personalizaremos)
# - CSS variables: Yes

# Instalar componentes necesarios
npx shadcn-svelte@latest add button
npx shadcn-svelte@latest add input
npx shadcn-svelte@latest add card
npx shadcn-svelte@latest add dialog
npx shadcn-svelte@latest add command
npx shadcn-svelte@latest add label
npx shadcn-svelte@latest add toast
npx shadcn-svelte@latest add separator
npx shadcn-svelte@latest add scroll-area
```

---

## Estructura de Directorios

Crear la estructura base:

```bash
# Directorios principales
mkdir -p src/lib/components/ui
mkdir -p src/lib/components/favoritos
mkdir -p src/lib/stores
mkdir -p src/lib/services
mkdir -p src/routes/(dashboard)
mkdir -p src/routes/(dashboard)/configuracion
mkdir -p tests/unit
mkdir -p tests/integration
```

---

## Scripts de Desarrollo

Actualizar `package.json`:

```json
{
    "scripts": {
        "dev": "vite dev",
        "build": "vite build",
        "preview": "vite preview",
        "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
        "check:watch": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --watch",
        "test": "vitest",
        "test:ui": "vitest --ui",
        "lint": "eslint .",
        "tauri": "tauri",
        "tauri:dev": "tauri dev",
        "tauri:build": "tauri build"
    }
}
```

---

## Configurar TypeScript

`tsconfig.json`:

```json
{
    "extends": "./.svelte-kit/tsconfig.json",
    "compilerOptions": {
        "strict": true,
        "moduleResolution": "bundler",
        "allowJs": true,
        "checkJs": true,
        "esModuleInterop": true
    }
}
```

---

## Comandos de Desarrollo

```bash
# Desarrollo frontend (hot reload)
pnpm dev

# Desarrollo completo (Tauri + SvelteKit)
pnpm tauri:dev

# Build para producción
pnpm tauri:build

# Ejecutar tests
pnpm test

# Type checking
pnpm check

# Linting
pnpm lint
```

---

## Configurar Tauri

### tauri.conf.json

Configuración clave en `src-tauri/tauri.conf.json`:

```json
{
    "build": {
        "beforeDevCommand": "pnpm dev",
        "beforeBuildCommand": "pnpm build",
        "devUrl": "http://localhost:5173",
        "frontendDist": "../build"
    },
    "app": {
        "windows": [
            {
                "title": "Favoritos Desktop",
                "width": 1200,
                "height": 800,
                "minWidth": 800,
                "minHeight": 600,
                "resizable": true,
                "fullscreen": false
            },
            {
                "label": "search",
                "title": "Buscar Favoritos",
                "width": 600,
                "height": 400,
                "decorations": false,
                "alwaysOnTop": true,
                "center": true,
                "visible": false,
                "skipTaskbar": true
            }
        ]
    },
    "tauri": {
        "allowlist": {
            "all": false,
            "shell": {
                "all": false,
                "open": true
            },
            "notification": {
                "all": true
            }
        },
        "bundle": {
            "active": true,
            "icon": ["icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png", "icons/icon.icns", "icons/icon.ico"]
        },
        "systemTray": {
            "iconPath": "icons/icon.png",
            "iconAsTemplate": true
        }
    }
}
```

---

## Testing Setup

### Vitest Config

`vitest.config.ts`:

```typescript
import { defineConfig } from 'vitest/config';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
    plugins: [svelte({ hot: !process.env.VITEST })],
    test: {
        globals: true,
        environment: 'jsdom',
        coverage: {
            provider: 'v8',
            reporter: ['text', 'json', 'html'],
            exclude: ['node_modules/', 'src/lib/components/ui/']
        }
    },
    resolve: {
        alias: {
            $lib: './src/lib'
        }
    }
});
```

### Playwright Config (E2E)

```bash
pnpm add -D @playwright/test
npx playwright install
```

---

## Primer Commit Inicial

```bash
git add .
git commit -m "feat: initial Tauri + SvelteKit setup with Shadcn-Svelte"
```

---

## Verificación de Setup

Para verificar que todo está configurado correctamente:

```bash
# 1. Iniciar desarrollo
pnpm tauri:dev

# 2. Deberías ver:
# - Compilación de Rust exitosa
# - Servidor de desarrollo en http://localhost:5173
# - Ventana de la aplicación abierta

# 3. Probar hot reload:
# - Modificar src/routes/+page.svelte
# - La ventana debería actualizarse automáticamente
```

---

## Siguientes Pasos

Una vez configurado el proyecto:

1. **Leer** `data-model.md` para entender las entidades
2. **Revisar** `contracts/api-endpoints.md` para la API
3. **Revisar** `contracts/tauri-commands.md` para comandos IPC
4. **Ejecutar** `/speckit.tasks` para generar las tareas de implementación

---

## Troubleshooting

### Error: "Cannot find module '@tauri-apps/api'"

```bash
pnpm install
```

### Error: "Rust not found"

Instalar Rust desde [rustup.rs](https://rustup.rs/) y reiniciar la terminal.

### Error: "pnpm not found"

```bash
npm install -g pnpm
```

### Hot reload no funciona

Asegúrate de que Vite esté configurado correctamente en `tauri.conf.json`:

```json
"devUrl": "http://localhost:5173"
```

---

## Recursos Útiles

- [Tauri v2 Docs](https://v2.tauri.app/)
- [SvelteKit Docs](https://kit.svelte.dev/)
- [Shadcn-Svelte](https://www.shadcn-svelte.com/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
